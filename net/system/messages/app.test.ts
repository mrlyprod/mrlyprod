import { expect, test } from "bun:test";
import type { Act, Who, World } from "../../kernel/index.ts";
import { digest, kernel } from "../../kernel/index.ts";
import { geese, scripted } from "../../test/gym.ts";
import type { Chat } from "./app.ts";
import chat, { KEEP, TEXT } from "./app.ts";
import { LINES, REPLIES } from "./bot.ts";

const ACTORS = Array.from({ length: 8 }, (_, i) => `test-${i + 1}`);

const CONFIG = { name: "messages", apps: ["messages"], actors: ACTORS };

const K = kernel([chat]);

const as = (id: string): Who => ({ id, kind: "human" });

const state = (world: World) => world.apps.messages as Chat;

const play = (world: World, acts: [string, string, Act["args"]][]) =>
  acts.reduce((at, [id, verb, args]) => K.call(at, { verb: `messages.${verb}`, args, who: as(id) }).world, world);

test("a send or a read out of its schema is a bad act and changes nothing", () => {
  const world = play(K.boot(CONFIG), [["test-1", "send", { to: "test-2", text: "hi" }]]);
  const bad: [string, Act["args"]][] = [
    ["send", { to: "test-2", text: "x".repeat(TEXT + 1) }],
    ["send", { to: "test-2", text: "" }],
    ["send", { to: "test-9", text: "hi" }],
    ["send", { to: "test-2", text: "hi", cc: "test-3" }],
    ["read", { thread: "test-2+test-3" }],
    ["read", {}],
  ];
  for (const [verb, args] of bad)
    expect(() => K.call(world, { verb: `messages.${verb}`, args, who: as("test-1") })).toThrow("bad-act");
});

test("only a thread's two people read it, nobody writes to themself, and a watcher may not send", () => {
  const world = play(K.boot(CONFIG), [["test-1", "send", { to: "test-2", text: "hi" }]]);
  const nosy = K.call(world, { verb: "messages.read", args: { thread: "test-1+test-2" }, who: as("test-3") });
  const self = K.call(world, { verb: "messages.send", args: { to: "test-4", text: "me" }, who: as("test-4") });
  expect([nosy.envelope.last.note, self.envelope.last.note]).toEqual([
    "only test-1 and test-2 read test-1+test-2",
    "a thread is between two people; write to someone else",
  ]);
  expect([digest(nosy.world.apps), digest(self.world.apps)]).toEqual([digest(world.apps), digest(world.apps)]);
  expect(() => K.call(world, { verb: "messages.send", args: { to: "test-1", text: "x" }, who: as("guest") })).toThrow(
    "refused",
  );
});

test("a message lives once in its thread; unread counts rise on send and fall on a read or a reply", () => {
  const sent = play(K.boot(CONFIG), [
    ["test-1", "send", { to: "test-2", text: "one" }],
    ["test-1", "send", { to: "test-2", text: "two" }],
    ["test-3", "send", { to: "test-2", text: "three" }],
  ]);
  const read = play(sent, [["test-2", "read", { thread: "test-1+test-2" }]]);
  const replied = play(sent, [["test-2", "send", { to: "test-3", text: "back" }]]);
  const unread = (world: World) => [
    state(world).unread["test-1"],
    state(world).unread["test-2"],
    state(world).unread["test-3"],
  ];
  expect([unread(sent), unread(read), unread(replied)]).toEqual([
    [0, 3, 0],
    [0, 1, 0],
    [0, 2, 1],
  ]);
  expect(state(sent).threads["test-1+test-2"]!.messages.map((one) => one.text)).toEqual(["one", "two"]);
  expect(K.read(read, "messages/@test-2", ["unread"]).state).toEqual({ unread: 1 });
  expect(Object.keys((K.read(sent, "messages/@test-1/threads").state ?? {}) as object)).toEqual(["test-1+test-2"]);
});

test(`a thread keeps its last ${KEEP} messages and counts every one`, () => {
  const many = Array.from({ length: KEEP + 5 }, (_, i): [string, string, Act["args"]] => [
    "test-1",
    "send",
    { to: "test-2", text: `m${i}` },
  ]);
  const thread = state(play(K.boot(CONFIG), many)).threads["test-1+test-2"]!;
  expect([thread.n, thread.messages.length, thread.messages[0]!.text]).toEqual([KEEP + 5, KEEP, "m5"]);
});

test("the scripted test users write each other plain lines, reply to and read their unread threads, with no refusal", () => {
  const run = scripted([chat], CONFIG, 150);
  const texts = Object.values(state(run.world).threads).flatMap((thread) => thread.messages.map((one) => one.text));
  const plausible = texts.every((text) => LINES.includes(text) || REPLIES.includes(text));
  const counts = ["messages.send", "messages.read"].map((verb) => run.verbs[verb]! > 20);
  expect([run.refused, counts, plausible]).toEqual([{}, [true, true], true]);
});

test("10,000 seeded worlds of chat played by the goose through the gym replay to their digests", async () => {
  const played = await geese(CONFIG, 10_000, 6);
  expect([played.same, played.acts, played.refused > 0]).toEqual([10_000, 60_000, true]);
}, 30_000);
