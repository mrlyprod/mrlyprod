import { expect, test } from "bun:test";
import type { Obj, Who, World } from "../../kernel/index.ts";
import { digest, kernel } from "../../kernel/index.ts";
import { noticed } from "../../sdk/system.ts";
import { geese, scripted } from "../../test/gym.ts";
import chat from "../messages/app.ts";
import type { Sim } from "./app.ts";
import sim, { KEEP, SNAPS } from "./app.ts";

const ACTORS = Array.from({ length: 8 }, (_, i) => `test-${i + 1}`);

const CONFIG = { name: "sim", apps: ["messages", "sim"], actors: ACTORS };

const K = kernel([chat, sim]);

const as = (id: string): Who => ({ id, kind: "human" });

const state = (world: World) => world.apps.sim as Sim;

const bussed = (world: World, verb: string, args: Obj, id: string) => {
  const done = K.call(world, { verb, args, who: as(id) });
  return noticed(done, [chat, sim]).reduce((at, next) => K.call(at, next).world, done.world);
};

test("a tick or a snap out of its schema is a bad act and changes nothing", () => {
  const world = K.boot(CONFIG);
  for (const [verb, args] of [
    ["tick", { app: "messages", verb: "send" }],
    ["tick", { app: "", verb: "send", ok: true }],
    ["tick", { app: "messages", verb: "send", ok: "yes" }],
    ["snap", { all: true }],
  ] as const)
    expect(() => K.call(world, { verb: `sim.${verb}`, args, who: as("test-1") })).toThrow("bad-act");
});

test("the bus ticks every act of the world's other apps as its actor, counting refusals, and a watcher may not act", () => {
  const sent = bussed(K.boot(CONFIG), "messages.send", { to: "test-2", text: "hi" }, "test-1");
  const refused = bussed(sent, "messages.send", { to: "test-3", text: "me" }, "test-3");
  const own = bussed(refused, "sim.snap", {}, "test-2");
  expect([state(own).acts, state(own).refused, state(own).apps, state(own).now.tick]).toEqual([
    2,
    1,
    { messages: { acts: 2, refused: 1 } },
    5,
  ]);
  expect(K.read(own, "sim/@test-3").state).toEqual({
    acts: 1,
    refused: 1,
    history: [
      {
        tick: 3,
        at: 0,
        app: "messages",
        verb: "send",
        ok: false,
        note: "a thread is between two people; write to someone else",
      },
    ],
  });
  expect((K.read(own, "sim/@test-1").state as { history: unknown[] }).history.length).toBe(1);
  expect(() => K.call(sent, { verb: "sim.snap", args: {}, who: as("guest") })).toThrow("refused");
});

test(`a snap records every app's size in bytes and the tick, keeping the last ${SNAPS}; a history keeps its last ${KEEP}`, () => {
  const world = K.call(K.boot(CONFIG), { verb: "sim.snap", args: {}, who: as("test-1") }).world;
  const [snap] = state(world).snaps;
  expect([snap!.tick, snap!.by, Object.keys(snap!.bytes), snap!.bytes.messages! > 50]).toEqual([
    1,
    "test-1",
    ["messages", "sim"],
    true,
  ]);
  let many = world;
  for (let i = 0; i < SNAPS + 2; i++) many = K.call(many, { verb: "sim.snap", args: {}, who: as("test-1") }).world;
  for (let i = 0; i < KEEP + 2; i++)
    many = K.call(many, { verb: "sim.tick", args: { app: "x", verb: "y", ok: true }, who: as("test-1") }).world;
  expect([
    state(many).snaps.length,
    state(many).history["test-1"]!.length,
    digest(world.apps) !== digest(many.apps),
  ]).toEqual([SNAPS, KEEP, true]);
});

test("the scripted test users chat while the bus ticks every act into sim, with no refusal", () => {
  const run = scripted([chat, sim], CONFIG, 60);
  const ticks = state(run.world);
  const others = run.acts - run.verbs["sim.tick"]! - (run.verbs["sim.snap"] ?? 0);
  expect([run.refused, run.verbs["sim.tick"], ticks.acts === others, ticks.apps.messages!.acts === others]).toEqual([
    {},
    ticks.acts,
    true,
    true,
  ]);
});

test("10,000 seeded worlds of chat and sim played by the goose through the gym replay to their digests", async () => {
  const played = await geese(CONFIG, 10_000, 6);
  expect([played.same, played.acts]).toEqual([10_000, 60_000]);
}, 30_000);
