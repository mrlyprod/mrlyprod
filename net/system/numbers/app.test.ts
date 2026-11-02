import { expect, test } from "bun:test";
import type { Act, Who, World } from "../../kernel/index.ts";
import { digest, kernel } from "../../kernel/index.ts";
import { geese, scripted } from "../../test/gym.ts";
import type { Sheets } from "./app.ts";
import sheets, { CELL, ROWS, SHEETS, USAGE } from "./app.ts";
import { values } from "./formula.ts";

const ACTORS = Array.from({ length: 8 }, (_, i) => `test-${i + 1}`);

const CONFIG = { name: "numbers", apps: ["numbers"], actors: ACTORS };

const K = kernel([sheets]);

const as = (id: string): Who => ({ id, kind: "human" });

const play = (world: World, acts: [string, string, Act["args"]][]) =>
  acts.reduce((at, [id, verb, args]) => K.call(at, { verb: `numbers.${verb}`, args, who: as(id) }).world, world);

const made = play(K.boot(CONFIG), [
  ["test-1", "create", { title: "Budget" }],
  ["test-1", "set", { sheet: "sheet-1", col: "A", row: 1, text: "900" }],
  ["test-1", "set", { sheet: "sheet-1", col: "A", row: 2, text: "0.1" }],
  ["test-1", "set", { sheet: "sheet-1", col: "B", row: 1, text: "=A1+A2*3" }],
  ["test-1", "share", { sheet: "sheet-1", with: "test-2" }],
]);

test("a create, set, share or delete out of its schema is a bad act and changes nothing", () => {
  const bad: [string, Act["args"]][] = [
    ["create", { title: "" }],
    ["set", { sheet: "sheet-9", col: "A", row: 1, text: "1" }],
    ["set", { sheet: "sheet-1", col: "Z", row: 1, text: "1" }],
    ["set", { sheet: "sheet-1", col: "A", row: ROWS + 1, text: "1" }],
    ["set", { sheet: "sheet-1", col: "A", row: 1, text: "x".repeat(CELL + 1) }],
    ["share", { sheet: "sheet-1", with: "test-9" }],
    ["delete", {}],
  ];
  for (const [verb, args] of bad)
    expect(() => K.call(made, { verb: `numbers.${verb}`, args, who: as("test-1") })).toThrow("bad-act");
});

test("an editor sets cells but never shares or deletes, a stranger does nothing, and a watcher may not act", () => {
  const set = K.call(made, {
    verb: "numbers.set",
    args: { sheet: "sheet-1", col: "C", row: 1, text: "ok" },
    who: as("test-2"),
  });
  const refused = [
    K.call(made, { verb: "numbers.share", args: { sheet: "sheet-1", with: "test-3" }, who: as("test-2") }),
    K.call(made, { verb: "numbers.delete", args: { sheet: "sheet-1" }, who: as("test-2") }),
    K.call(made, { verb: "numbers.set", args: { sheet: "sheet-1", col: "A", row: 1, text: "0" }, who: as("test-3") }),
    K.call(made, {
      verb: "numbers.set",
      args: { sheet: "sheet-1", col: "A", row: 1, text: "=A1+" },
      who: as("test-1"),
    }),
    K.call(made, { verb: "numbers.set", args: { sheet: "sheet-1", col: "A", row: 1, text: "=Z9" }, who: as("test-1") }),
  ];
  expect((set.world.apps.numbers as Sheets).sheets["sheet-1"]!.cells.C1).toBe("ok");
  expect(refused.map((one) => one.envelope.last.note)).toEqual([
    "sheet-1 is test-1's; only its owner may",
    "sheet-1 is test-1's; only its owner may",
    "sheet-1 is test-1's; ask to be an editor",
    USAGE,
    `cells run A1 to H${ROWS}, not Z9`,
  ]);
  expect(refused.map((one) => digest(one.world.apps))).toEqual(refused.map(() => digest(made.apps)));
  expect(() => K.call(made, { verb: "numbers.create", args: { title: "x" }, who: as("guest") })).toThrow("refused");
});

test("values are exact: decimals stay decimals, thirds show as fractions, and a cycle, a label or a zero divisor name their error", () => {
  expect(
    values({
      A1: "900",
      A2: "0.1",
      B1: "=A1+A2*3",
      B2: "=A1/7",
      B3: "=(A1-A2)/4",
      C1: "=SUM(A1:A2, B3)",
      C2: "=C3",
      C3: "=C2",
      D1: "rent",
      D2: "=D1+1",
      D3: "=1/0",
      D4: "=-A2",
      D5: "=0.1+0.2",
    }),
  ).toEqual({
    A1: "900",
    A2: "0.1",
    B1: "900.3",
    B2: "900/7",
    B3: "224.975",
    C1: "1125.075",
    C2: "#CYCLE",
    C3: "#CYCLE",
    D1: "rent",
    D2: "#TEXT",
    D3: "#DIV/0",
    D4: "-0.1",
    D5: "0.3",
  });
  expect(K.read(made, "numbers/@test-2/sheets/sheet-1/values").state).toEqual({ A1: "900", A2: "0.1", B1: "900.3" });
  expect(K.read(made, "numbers/@test-3/sheets").state).toEqual({});
});

test(`a user holds at most ${SHEETS} sheets, and an empty text clears a cell`, () => {
  const many = Array.from({ length: SHEETS }, (): [string, string, Act["args"]] => [
    "test-1",
    "create",
    { title: "s" },
  ]);
  const full = K.call(play(K.boot(CONFIG), many), {
    verb: "numbers.create",
    args: { title: "more" },
    who: as("test-1"),
  });
  const cleared = play(made, [["test-1", "set", { sheet: "sheet-1", col: "A", row: 1, text: " " }]]);
  expect([full.envelope.last.note, Object.keys((cleared.apps.numbers as Sheets).sheets["sheet-1"]!.cells)]).toEqual([
    `you hold ${SHEETS} sheets already; delete one first`,
    ["A2", "B1"],
  ]);
});

test("the scripted test users create, fill, share and delete sheets with no refusal", () => {
  const run = scripted([sheets], CONFIG, 150);
  const counts = ["numbers.create", "numbers.set", "numbers.share"].map((verb) => run.verbs[verb]! > 5);
  expect([run.refused, counts]).toEqual([{}, [true, true, true]]);
});

test("10,000 seeded worlds of sheets played by the goose through the gym replay to their digests", async () => {
  const played = await geese(CONFIG, 10_000, 6);
  expect([played.same, played.acts, played.refused > 0]).toEqual([10_000, 60_000, true]);
}, 30_000);
