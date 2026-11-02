import { expect, test } from "bun:test";
import type { Act, Who, World } from "../../kernel/index.ts";
import { digest, kernel } from "../../kernel/index.ts";
import { geese } from "../../test/gym.ts";
import people from "../people/app.ts";
import type { News } from "./app.ts";
import news, { FEED, HEADLINE, orgSource, TEXT } from "./app.ts";

const ACTORS = Array.from({ length: 8 }, (_, i) => `test-${i + 1}`);

const CONFIG = { name: "news", apps: ["people", "news"], actors: ACTORS };

const K = kernel([people, news]);

const as = (id: string): Who => ({ id, kind: "human" });

const state = (world: World) => world.apps.news as News;

const play = (world: World, acts: [string, string, Act["args"]][]) =>
  acts.reduce((at, [id, verb, args]) => K.call(at, { verb, args, who: as(id) }).world, world);

test("a post, a follow or an unfollow out of its schema is a bad act and changes nothing", () => {
  const world = K.boot(CONFIG);
  const bad: [string, Act["args"]][] = [
    ["news.post", { headline: "" }],
    ["news.post", { headline: "x".repeat(HEADLINE + 1) }],
    ["news.post", { headline: "Rain", text: "x".repeat(TEXT + 1) }],
    ["news.post", { headline: "Rain", org: 0 }],
    ["news.post", { headline: "Rain", by: "test-2" }],
    ["news.follow", { source: "test-9" }],
    ["news.follow", { source: orgSource(1) }],
    ["news.unfollow", {}],
  ];
  for (const [verb, args] of bad) expect(() => K.call(world, { verb, args, who: as("test-1") })).toThrow("bad-act");
});

test("a post as an org needs a live org of people the poster belongs to; following yourself or a source twice is refused; a watcher may not post", () => {
  const world = play(K.boot(CONFIG), [
    ["test-1", "people.found", { name: "The Gazette", open: false }],
    ["test-1", "news.follow", { source: "test-2" }],
  ]);
  const notes = [
    ["test-2", "news.post", { headline: "Rain", org: 1 }],
    ["test-1", "news.post", { headline: "Rain", org: 2 }],
    ["test-1", "news.follow", { source: "test-1" }],
    ["test-1", "news.follow", { source: "test-2" }],
    ["test-1", "news.unfollow", { source: "test-3" }],
  ].map(([id, verb, args]) => {
    const done = K.call(world, { verb: verb as string, args: args as Act["args"], who: as(id as string) });
    expect(digest(done.world.apps)).toBe(digest(world.apps));
    return done.envelope.last.note;
  });
  expect(notes).toEqual([
    "only a member of The Gazette posts as it",
    "no org 2 in people",
    "your own posts are in your feed already",
    "you follow test-2 already",
    "you do not follow test-3",
  ]);
  const alone = kernel([news]).boot({ ...CONFIG, apps: ["news"] });
  expect(
    kernel([news]).call(alone, { verb: "news.post", args: { headline: "Rain", org: 1 }, who: as("test-1") }).envelope
      .last.note,
  ).toBe("this world has no people, so no orgs; post as yourself");
  expect(() => K.call(world, { verb: "news.post", args: { headline: "Rain" }, who: as("guest") })).toThrow("refused");
});

test(`a feed is your own posts and your sources', newest first, at most ${FEED}; an org that posted becomes a source to follow`, () => {
  const world = play(K.boot(CONFIG), [
    ["test-1", "people.found", { name: "The Gazette", open: true }],
    ["test-1", "news.post", { headline: "Rain tomorrow", org: 1 }],
    ["test-2", "news.post", { headline: "Bread up by one", text: "The bakery says so." }],
    ["test-3", "news.post", { headline: "Nothing happened" }],
    ["test-4", "news.follow", { source: orgSource(1) }],
    ["test-4", "news.follow", { source: "test-2" }],
    ["test-4", "news.post", { headline: "Hello" }],
  ]);
  expect(state(world).sources).toEqual({ [orgSource(1)]: "The Gazette" });
  expect(K.read(world, "news/@test-4", { following: true, feed: ["n", "source", "headline"] }).state).toEqual({
    following: [
      { source: orgSource(1), name: "The Gazette" },
      { source: "test-2", name: "test-2" },
    ],
    feed: [
      { n: 4, source: "test-4", headline: "Hello" },
      { n: 2, source: "test-2", headline: "Bread up by one" },
      { n: 1, source: orgSource(1), headline: "Rain tomorrow" },
    ],
  });
  expect(state(world).posts[0]!.by).toBe("test-1");
  const many = play(
    world,
    Array.from({ length: FEED + 5 }, (): [string, string, Act["args"]] => [
      "test-2",
      "news.post",
      { headline: "Again" },
    ]),
  );
  expect((K.read(many, "news/@test-4/feed").state as unknown[]).length).toBe(FEED);
});

test("10,000 seeded worlds of people and news played by the goose through the gym replay to their digests", async () => {
  const played = await geese(CONFIG, 10_000, 6);
  expect([played.same, played.acts, played.refused > 0]).toEqual([10_000, 60_000, true]);
}, 30_000);
