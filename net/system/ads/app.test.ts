import { expect, test } from "bun:test";
import type { Act, Who, World } from "../../kernel/index.ts";
import { digest, kernel } from "../../kernel/index.ts";
import { geese } from "../../test/gym.ts";
import type { Ads } from "./app.ts";
import ads, { LIVE, TITLE } from "./app.ts";

const ACTORS = Array.from({ length: 8 }, (_, i) => `test-${i + 1}`);

const CONFIG = { name: "ads", apps: ["ads"], actors: ACTORS };

const K = kernel([ads]);

const as = (id: string): Who => ({ id, kind: "human" });

const state = (world: World) => world.apps.ads as Ads;

const AD = { org: "Bakery", title: "Bread", text: "Fresh every morning" };

const play = (world: World, acts: [string, string, Act["args"]][]) =>
  acts.reduce((at, [id, verb, args]) => K.call(at, { verb: `ads.${verb}`, args, who: as(id) }).world, world);

const notes = (world: World, acts: [string, string, Act["args"]][]) =>
  acts.map(([id, verb, args]) => {
    const done = K.call(world, { verb: `ads.${verb}`, args, who: as(id) });
    expect(digest(done.world.apps)).toBe(digest(world.apps));
    return done.envelope.last.note;
  });

test("a post or a pull out of its schema is a bad act and changes nothing", () => {
  const world = play(K.boot(CONFIG), [["test-1", "post", AD]]);
  const bad: [string, Act["args"]][] = [
    ["post", { ...AD, org: "" }],
    ["post", { ...AD, title: "x".repeat(TITLE + 1) }],
    ["post", { org: "Bakery", title: "Bread" }],
    ["post", { ...AD, by: "test-1" }],
    ["pull", { ad: 2 }],
  ];
  for (const [verb, args] of bad)
    expect(() => K.call(world, { verb: `ads.${verb}`, args, who: as("test-2") })).toThrow("bad-act");
});

test(`only its poster pulls an ad, ${LIVE} live each, a pulled ad is gone, and a watcher may not post`, () => {
  const full = play(
    K.boot(CONFIG),
    Array.from({ length: LIVE }, (): [string, string, Act["args"]] => ["test-1", "post", AD]),
  );
  const pulled = play(full, [["test-1", "pull", { ad: 2 }]]);
  expect(
    notes(full, [
      ["test-2", "pull", { ad: 1 }],
      ["test-1", "post", AD],
    ]),
  ).toEqual(["only test-1 pulls ad 1", `you have ${LIVE} ads live; pull one first`]);
  expect([Object.keys(state(pulled).ads), notes(pulled, [["test-1", "pull", { ad: 2 }]])]).toEqual([
    ["1", "3"],
    ["ad 2 was already pulled"],
  ]);
  expect(K.read(pulled, "ads/@test-1").state).toHaveLength(LIVE - 1);
  expect(() => K.call(full, { verb: "ads.post", args: AD, who: as("guest") })).toThrow("refused");
});

test("10,000 seeded worlds of ads played by the goose through the gym replay to their digests", async () => {
  const played = await geese(CONFIG, 10_000, 6);
  expect([played.same, played.acts, played.refused > 0]).toEqual([10_000, 60_000, true]);
}, 30_000);
