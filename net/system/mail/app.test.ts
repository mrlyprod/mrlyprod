import { expect, test } from "bun:test";
import type { Act, Who, World } from "../../kernel/index.ts";
import { digest, kernel } from "../../kernel/index.ts";
import { geese, scripted } from "../../test/gym.ts";
import inbox from "../notifications/app.ts";
import type { Mail } from "./app.ts";
import mail, { KEEP, SUBJECT } from "./app.ts";

const ACTORS = Array.from({ length: 8 }, (_, i) => `test-${i + 1}`);

const CONFIG = { name: "mail", apps: ["mail"], actors: ACTORS };

const K = kernel([mail]);

const as = (id: string): Who => ({ id, kind: "human" });

const state = (world: World) => world.apps.mail as Mail;

const play = (world: World, acts: [string, string, Act["args"]][]) =>
  acts.reduce((at, [id, verb, args]) => K.call(at, { verb: `mail.${verb}`, args, who: as(id) }).world, world);

const sent = play(K.boot(CONFIG), [
  ["test-1", "send", { to: ["test-2", "test-3"], subject: "Lunch", text: "Friday?" }],
]);

test("a send, reply, mark or move out of its schema is a bad act and changes nothing", () => {
  const bad: [string, Act["args"]][] = [
    ["send", { to: [], subject: "a", text: "b" }],
    ["send", { to: ["test-9"], subject: "a", text: "b" }],
    ["send", { to: ["test-2"], subject: "x".repeat(SUBJECT + 1), text: "b" }],
    ["send", { to: "test-2", subject: "a", text: "b" }],
    ["reply", { id: 2, text: "yes" }],
    ["mark", { id: 1 }],
    ["archive", { id: 1, folder: "sent" }],
  ];
  for (const [verb, args] of bad)
    expect(() => K.call(sent, { verb: `mail.${verb}`, args, who: as("test-2") })).toThrow("bad-act");
});

test("only a mailbox's owner touches its messages, nobody mails themself, and a watcher may not act", () => {
  const nosy = K.call(sent, { verb: "mail.archive", args: { id: 1 }, who: as("test-4") });
  const self = K.call(sent, {
    verb: "mail.send",
    args: { to: ["test-4"], subject: "a", text: "b" },
    who: as("test-4"),
  });
  expect([nosy.envelope.last.note, self.envelope.last.note]).toEqual([
    "message 1 is not in your mailbox",
    "you are among the recipients; mail goes to others",
  ]);
  expect(digest(nosy.world.apps)).toBe(digest(sent.apps));
  expect(() => K.call(sent, { verb: "mail.mark", args: { id: 1, read: true }, who: as("guest") })).toThrow("refused");
});

test("a message lives once, filed per mailbox; a reply joins its thread; archive, restore and delete move one copy", () => {
  const replied = play(sent, [["test-2", "reply", { id: 1, text: "Yes" }]]);
  const moved = play(replied, [
    ["test-1", "archive", { id: 2 }],
    ["test-3", "delete", { id: 1 }],
    ["test-2", "archive", { id: 1 }],
    ["test-2", "restore", { id: 1 }],
  ]);
  const box = (id: string) => K.read(moved, `mail/@${id}`).state as Record<string, { id: number; read: boolean }[]>;
  expect(state(replied).messages["2"]).toMatchObject({
    thread: 1,
    from: "test-2",
    to: ["test-1"],
    subject: "Re: Lunch",
  });
  expect([box("test-1").archive!.map((one) => one.id), box("test-1").sent!.map((one) => one.id)]).toEqual([[2], [1]]);
  expect([
    box("test-2").inbox!.map((one) => one.read),
    box("test-3").inbox,
    K.read(moved, "mail/@test-3/unread").state,
  ]).toEqual([[true], [], 0]);
  const gone = play(moved, [
    ["test-1", "delete", { id: 1 }],
    ["test-2", "delete", { id: 1 }],
  ]);
  expect([Object.keys(state(moved).messages), Object.keys(state(gone).messages)]).toEqual([["1", "2"], ["2"]]);
});

test(`a mailbox keeps its last ${KEEP} messages and a message nobody holds is dropped`, () => {
  const many = Array.from({ length: KEEP + 4 }, (_, i): [string, string, Act["args"]] => [
    "test-1",
    "send",
    { to: ["test-2"], subject: `m${i}`, text: "x" },
  ]);
  const full = state(play(K.boot(CONFIG), many));
  expect([
    Object.keys(full.boxes["test-2"]!).length,
    Object.keys(full.messages).length,
    full.messages["5"]!.subject,
  ]).toEqual([KEEP, KEEP, "m4"]);
});

test("the scripted test users mail each other, read and reply, with no refusal, and each mail raises a notice", () => {
  const run = scripted([mail, inbox], { ...CONFIG, apps: ["mail", "notifications"] }, 150);
  const counts = ["mail.send", "mail.reply", "mail.mark", "notifications.notice"].map((verb) => run.verbs[verb]! > 20);
  expect([run.refused, counts]).toEqual([{}, [true, true, true, true]]);
});

test("10,000 seeded worlds of mail played by the goose through the gym replay to their digests", async () => {
  const played = await geese(CONFIG, 10_000, 6);
  expect([played.same, played.acts, played.refused > 0]).toEqual([10_000, 60_000, true]);
}, 30_000);
