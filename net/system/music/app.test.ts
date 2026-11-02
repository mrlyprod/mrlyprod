import { expect, test } from "bun:test";
import type { Act, Who, World } from "../../kernel/index.ts";
import { digest, kernel } from "../../kernel/index.ts";
import type { Media } from "../../sdk/media.ts";
import { MINUTES, TITLE } from "../../sdk/media.ts";
import { geese } from "../../test/gym.ts";
import music from "./app.ts";

const ACTORS = Array.from({ length: 8 }, (_, i) => `test-${i + 1}`);

const CONFIG = { name: "music", apps: ["music"], actors: ACTORS };

const K = kernel([music]);

const as = (id: string): Who => ({ id, kind: "human" });

const play = (world: World, acts: [string, string, Act["args"]][]) =>
  acts.reduce((at, [id, verb, args]) => K.call(at, { verb: `music.${verb}`, args, who: as(id) }).world, world);

test("a publish, an add or a play out of its schema is a bad act and changes nothing", () => {
  const world = play(K.boot(CONFIG), [["test-1", "publish", { title: "One", minutes: 4 }]]);
  const bad: [string, Act["args"]][] = [
    ["publish", { title: "x".repeat(TITLE + 1), minutes: 4 }],
    ["publish", { title: "One", minutes: MINUTES + 1 }],
    ["publish", { title: "One" }],
    ["publish", { title: "One", minutes: 4, file: "one.mp3" }],
    ["add", { title: 2 }],
    ["play", { title: 0 }],
    ["next", { title: 1 }],
  ];
  for (const [verb, args] of bad)
    expect(() => K.call(world, { verb: `music.${verb}`, args, who: as("test-1") })).toThrow("bad-act");
});

test("a library is its owner's: a remove of what is not there is refused with a plain note, and a watcher may not act", () => {
  const world = play(K.boot(CONFIG), [
    ["test-1", "publish", { title: "One", minutes: 4 }],
    ["test-2", "add", { title: 1 }],
  ]);
  const missing = K.call(world, { verb: "music.remove", args: { title: 1 }, who: as("test-3") });
  const twice = K.call(world, { verb: "music.add", args: { title: 1 }, who: as("test-2") });
  expect([missing.envelope.last.note, twice.envelope.last.note]).toEqual([
    "track 1 is not in your library",
    "track 1 is in your library already",
  ]);
  expect([digest(missing.world.apps), digest(twice.world.apps)]).toEqual([digest(world.apps), digest(world.apps)]);
  expect((world.apps.music as Media).library["test-2"]).toEqual([1]);
  expect(() => K.call(world, { verb: "music.play", args: { title: 1 }, who: as("guest") })).toThrow("refused");
});

test("10,000 seeded worlds of music played by the goose through the gym replay to their digests", async () => {
  const played = await geese(CONFIG, 10_000, 6);
  expect([played.same, played.acts, played.refused > 0]).toEqual([10_000, 60_000, true]);
}, 30_000);
