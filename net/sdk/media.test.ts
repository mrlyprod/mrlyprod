import { expect, test } from "bun:test";
import type { Act, Who, World } from "../kernel/index.ts";
import { kernel } from "../kernel/index.ts";
import type { Media } from "./media.ts";
import { media, QUEUE } from "./media.ts";

const ACTORS = Array.from({ length: 4 }, (_, i) => `test-${i + 1}`);

const CONFIG = { name: "media", apps: ["demo"], actors: ACTORS };

const demo = media({ id: "demo", title: "Demo", line: "A shelf.", unit: "item", titles: ["One", "Two"] });

const K = kernel([demo]);

const as = (id: string): Who => ({ id, kind: "human" });

const state = (world: World) => world.apps.demo as Media;

const play = (world: World, acts: [string, string, Act["args"]][]) =>
  acts.reduce((at, [id, verb, args]) => K.call(at, { verb: `demo.${verb}`, args, who: as(id) }).world, world);

test("a published title is metadata only, and a library, a queue and plays are one person's", () => {
  const world = play(K.boot(CONFIG), [
    ["test-1", "publish", { title: "One", minutes: 4 }],
    ["test-1", "publish", { title: "Two", minutes: 30, about: "a line" }],
    ["test-2", "add", { title: 1 }],
    ["test-2", "add", { title: 2 }],
    ["test-2", "remove", { title: 1 }],
    ["test-2", "play", { title: 2 }],
    ["test-2", "play", { title: 2 }],
    ["test-3", "play", { title: 2 }],
  ]);
  const after = state(world);
  expect([after.titles[2]!.plays, after.plays["test-2"]![2], after.library["test-2"], after.now["test-3"]]).toEqual([
    3,
    2,
    [2],
    2,
  ]);
  expect(Object.keys(after.titles[1]!).sort()).toEqual(["about", "at", "by", "id", "minutes", "plays", "title"]);
  expect(K.read(world, "demo/@test-2", ["now", "library"]).state).toEqual({
    now: { id: 2, title: "Two", by: "test-1", minutes: 30, plays: 2 },
    library: [{ id: 2, title: "Two", by: "test-1", minutes: 30, plays: 2 }],
  });
});

test(`a queue plays in order through next, holds ${QUEUE}, and refuses next when empty`, () => {
  let world = play(K.boot(CONFIG), [
    ["test-1", "publish", { title: "One", minutes: 4 }],
    ["test-1", "publish", { title: "Two", minutes: 5 }],
    ["test-1", "queue", { title: 2 }],
    ["test-1", "queue", { title: 1 }],
  ]);
  world = play(world, [["test-1", "next", {}]]);
  expect([state(world).now["test-1"], state(world).queue["test-1"]]).toEqual([2, [1]]);
  world = play(world, [["test-1", "next", {}]]);
  const empty = K.call(world, { verb: "demo.next", args: {}, who: as("test-1") });
  expect([state(world).queue["test-1"], empty.envelope.last.note]).toEqual([
    [],
    "your queue is empty; queue a item first",
  ]);
  const full = play(
    world,
    Array.from({ length: QUEUE }, (): [string, string, Act["args"]] => ["test-2", "queue", { title: 1 }]),
  );
  expect(K.call(full, { verb: "demo.queue", args: { title: 1 }, who: as("test-2") }).envelope.last.note).toBe(
    `your queue holds ${QUEUE} items; play the next first`,
  );
});
