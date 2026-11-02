import { expect, test } from "bun:test";
import type { Act, Who, World } from "../../kernel/index.ts";
import { digest, kernel } from "../../kernel/index.ts";
import { noticed } from "../../sdk/system.ts";
import { geese, scripted } from "../../test/gym.ts";
import files from "../files/app.ts";
import type { Session } from "./app.ts";
import code, { LINE, LINES, PATH, TEXT } from "./app.ts";

const ACTORS = Array.from({ length: 8 }, (_, i) => `test-${i + 1}`);

const CONFIG = { name: "code", apps: ["files", "code"], actors: ACTORS };

const K = kernel([files, code]);

const as = (id: string): Who => ({ id, kind: "human" });

const play = (world: World, acts: [string, string, Act["args"]][]) =>
  acts.reduce((at, [id, verb, args]) => K.call(at, { verb, args, who: as(id) }).world, world);

const stored = play(K.boot(CONFIG), [
  ["test-1", "files.folder", { path: "/src" }],
  ["test-1", "files.write", { path: "/src/main.ts", text: "const a = 1;\nexport { a };" }],
  ["test-1", "code.open", { path: "/src/main.ts" }],
]);

const session = (world: World, id: string) => K.read(world, `code/@${id}`).state as Session;

test("an open, new, edit, cursor, save or close out of its schema is a bad act and changes nothing", () => {
  const bad: [string, Act["args"]][] = [
    ["open", {}],
    ["open", { path: `/${"a".repeat(PATH)}` }],
    ["new", { path: "/x.ts", text: "" }],
    ["edit", { line: LINES, text: "x" }],
    ["edit", { line: 0, text: "x".repeat(LINE + 1) }],
    ["cursor", { line: 0 }],
    ["save", { path: "/src/main.ts" }],
  ];
  for (const [verb, args] of bad)
    expect(() => K.call(stored, { verb: `code.${verb}`, args, who: as("test-1") })).toThrow("bad-act");
});

test("a buffer is the caller's own, a path names the caller's store, a missing file or folder is refused, and a watcher may not act", () => {
  const refused = [
    K.call(stored, { verb: "code.edit", args: { line: 0, text: "x" }, who: as("test-2") }),
    K.call(stored, { verb: "code.open", args: { path: "/src/main.ts" }, who: as("test-2") }),
    K.call(stored, { verb: "code.new", args: { path: "/lib/a.ts" }, who: as("test-1") }),
    K.call(stored, { verb: "code.new", args: { path: "src/a.ts" }, who: as("test-1") }),
    K.call(stored, { verb: "code.edit", args: { line: 5, text: "x" }, who: as("test-1") }),
  ];
  expect(refused.map((one) => one.envelope.last.note)).toEqual([
    "open a file first",
    "no file /src/main.ts in your store; code.new starts one",
    "no folder /lib; make it in files first",
    "a path starts with / and names a file in your store, like /src/main.ts, not src/a.ts",
    "the buffer has 2 lines; edit one of them or append at 2",
  ]);
  expect(refused.map((one) => digest(one.world.apps))).toEqual(refused.map(() => digest(stored.apps)));
  expect(() => K.call(stored, { verb: "code.new", args: { path: "/a.ts" }, who: as("guest") })).toThrow("refused");
});

test("open loads the file's lines, edit replaces or appends one and moves the cursor, and save wishes the write to files", () => {
  expect(session(stored, "test-1")).toEqual({
    open: "/src/main.ts",
    buffer: ["const a = 1;", "export { a };"],
    cursor: { line: 0, col: 0 },
    dirty: false,
  });
  const edited = play(stored, [
    ["test-1", "code.edit", { line: 0, text: "const a = 2;" }],
    ["test-1", "code.edit", { line: 2, text: "export default a;" }],
  ]);
  expect([session(edited, "test-1").buffer, session(edited, "test-1").cursor, session(edited, "test-1").dirty]).toEqual(
    [["const a = 2;", "export { a };", "export default a;"], { line: 2, col: 17 }, true],
  );
  const saved = K.call(edited, { verb: "code.save", args: {}, who: as("test-1") });
  const [write] = noticed(saved, [files, code]);
  expect([session(saved.world, "test-1").dirty, write?.verb, write?.args]).toEqual([
    false,
    "files.write",
    { path: "/src/main.ts", text: "const a = 2;\nexport { a };\nexport default a;" },
  ]);
  const landed = K.call(saved.world, write!).world;
  expect(K.read(landed, "files/@test-1/src/main.ts").state).toBe("const a = 2;\nexport { a };\nexport default a;");
});

test(`a world without files refuses to open, and a buffer holds at most ${TEXT} characters`, () => {
  const alone = kernel([code]).boot({ name: "alone", apps: ["code"], actors: ACTORS });
  const nowhere = kernel([code]).call(alone, { verb: "code.new", args: { path: "/a.ts" }, who: as("test-1") });
  const big = play(
    stored,
    Array.from({ length: 19 }, (_, i): [string, string, Act["args"]] => [
      "test-1",
      "code.edit",
      { line: i, text: "x".repeat(LINE) },
    ]),
  );
  const over = K.call(big, { verb: "code.edit", args: { line: 19, text: "x".repeat(LINE) }, who: as("test-1") });
  expect([nowhere.envelope.last.note, over.envelope.last.note]).toEqual([
    "this world holds no files app; code edits the files app's store",
    `a file holds at most ${TEXT} characters`,
  ]);
});

test("the scripted test users start, open, edit, save and close files with no refusal, and the saves land in files", () => {
  const run = scripted([files, code], CONFIG, 150);
  const counts = ["code.new", "code.edit", "code.save", "files.write"].map((verb) => run.verbs[verb]! > 5);
  expect([run.refused, counts]).toEqual([{}, [true, true, true, true]]);
});

test("10,000 seeded worlds of files and code played by the goose through the gym replay to their digests", async () => {
  const played = await geese(CONFIG, 10_000, 6);
  expect([played.same, played.acts, played.refused > 0]).toEqual([10_000, 60_000, true]);
}, 30_000);
