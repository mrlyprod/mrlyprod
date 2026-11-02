import { expect, test } from "bun:test";
import type { Act, Obj, Who, World } from "../../kernel/index.ts";
import { digest, kernel } from "../../kernel/index.ts";
import { noticed } from "../../sdk/system.ts";
import { geese, scripted } from "../../test/gym.ts";
import chat from "../messages/app.ts";
import type { Typed } from "./app.ts";
import term, { KEEP, LINE, USAGE } from "./app.ts";

const ACTORS = Array.from({ length: 8 }, (_, i) => `test-${i + 1}`);

const CONFIG = { name: "term", apps: ["messages", "term"], actors: ACTORS };

const K = kernel([chat, term]);

const as = (id: string): Who => ({ id, kind: "human" });

const typed = (world: World, id: string, line: string) =>
  K.call(world, { verb: "term.type", args: { line }, who: as(id) });

const history = (world: World, id: string) => K.read(world, `term/@${id}`).state as Typed[];

test("a line out of its schema is a bad act, and a line that is none of the calls is refused with the usage", () => {
  const world = K.boot(CONFIG);
  const bad: Obj[] = [{}, { line: "" }, { line: "x".repeat(LINE + 1) }, { line: "list", as: "test-2" }];
  for (const args of bad)
    expect(() => K.call(world, { verb: "term.type", args, who: as("test-1") })).toThrow("bad-act");
  const refused = [
    typed(world, "test-1", "ls"),
    typed(world, "test-1", "boot world.json"),
    typed(world, "test-1", "call messages.send not json"),
    typed(world, "test-1", "call snake.turn {}"),
    typed(world, "test-1", "call hello"),
  ];
  expect(refused.map((one) => one.envelope.last.note)).toEqual([
    `${USAGE}, not ls`,
    `a world boots from outside; ${USAGE}`,
    'args are JSON like {"to":"test-2","text":"hi"}, not not json',
    "no app snake in this world; type list to see them",
    "call takes a verb like messages.send, then its args as JSON",
  ]);
  expect(refused.map((one) => digest(one.world.apps))).toEqual(refused.map(() => digest(world.apps)));
  expect(() => typed(world, "guest", "list")).toThrow("refused");
});

test("list and read answer from the world into the typist's own history, and nobody else's", () => {
  const world = typed(typed(K.boot(CONFIG), "test-1", "list").world, "test-1", "read messages/unread/test-2").world;
  const mine = history(world, "test-1");
  expect(mine.map((one) => [one.call, one.result])).toEqual([
    ["list", { messages: ["messages.send", "messages.read"], term: ["term.type"] }],
    ["read", 0],
  ]);
  expect(history(world, "test-2")).toEqual([]);
});

test("a typed call becomes the typist's own act on the bus, and the history keeps its last lines", () => {
  const sent = typed(K.boot(CONFIG), "test-1", 'call messages.send {"to":"test-2","text":"typed hello"}');
  const [next] = noticed(sent, [chat, term]);
  expect([sent.envelope.last.ok, history(sent.world, "test-1")[0]!.result, next]).toEqual([
    true,
    "sent messages.send as test-1",
    { verb: "messages.send", args: { to: "test-2", text: "typed hello" }, who: as("test-1"), at: sent.line.at },
  ]);
  const landed = K.call(sent.world, next!).world;
  expect(K.read(landed, "messages/unread/test-2").state).toBe(1);
  const many = Array.from(
    { length: KEEP + 3 },
    (_, i): Act => ({
      verb: "term.type",
      args: { line: `read messages/unread/test-${(i % 8) + 1}` },
      who: as("test-1"),
    }),
  );
  const long = many.reduce((at, one) => K.call(at, one).world, landed);
  expect([history(long, "test-1").length, history(long, "test-1")[0]!.line]).toEqual([
    KEEP,
    "read messages/unread/test-4",
  ]);
});

test("the scripted test users type lists, reads and calls with no refusal, and the typed sends land in chat", () => {
  const run = scripted([chat, term], CONFIG, 100);
  expect([run.refused, run.verbs["term.type"]! > 20, run.verbs["messages.send"]! > 5]).toEqual([{}, true, true]);
});

test("10,000 seeded worlds of chat and term played by the goose through the gym replay to their digests", async () => {
  const played = await geese(CONFIG, 10_000, 6);
  expect([played.same, played.acts, played.refused > 0]).toEqual([10_000, 60_000, true]);
}, 30_000);
