import { expect, test } from "bun:test";
import type { Act, Who, World } from "../../kernel/index.ts";
import { digest, kernel } from "../../kernel/index.ts";
import { geese, scripted } from "../../test/gym.ts";
import sites, { PAGES, SLUG } from "./app.ts";

const ACTORS = Array.from({ length: 8 }, (_, i) => `test-${i + 1}`);

const CONFIG = { name: "sites", apps: ["sites"], actors: ACTORS };

const K = kernel([sites]);

const as = (id: string): Who => ({ id, kind: "human" });

const play = (world: World, acts: [string, string, Act["args"]][]) =>
  acts.reduce((at, [id, verb, args]) => K.call(at, { verb: `sites.${verb}`, args, who: as(id) }).world, world);

const HOME = [{ kind: "h1", text: "Welcome" }];

const drafted = play(K.boot(CONFIG), [["test-1", "page", { slug: "home", title: "Welcome", blocks: HOME }]]);

test("a page, drop or publish out of its schema is a bad act and changes nothing", () => {
  const bad: [string, Act["args"]][] = [
    ["page", { slug: "", title: "x", blocks: [] }],
    ["page", { slug: "a".repeat(SLUG + 1), title: "x", blocks: [] }],
    ["page", { slug: "home", title: "x" }],
    ["drop", { slug: "home", user: "test-1" }],
    ["publish", { on: "yes" }],
  ];
  for (const [verb, args] of bad)
    expect(() => K.call(drafted, { verb: `sites.${verb}`, args, who: as("test-1") })).toThrow("bad-act");
});

test("a verb touches the caller's site alone, a bad slug or an empty publish is refused, and a watcher may not act", () => {
  const refused = [
    K.call(drafted, { verb: "sites.drop", args: { slug: "home" }, who: as("test-2") }),
    K.call(drafted, { verb: "sites.page", args: { slug: "About Me", title: "x", blocks: [] }, who: as("test-1") }),
    K.call(drafted, { verb: "sites.publish", args: { on: true }, who: as("test-2") }),
    K.call(drafted, { verb: "sites.publish", args: { on: false }, who: as("test-1") }),
  ];
  expect(refused.map((one) => one.envelope.last.note)).toEqual([
    "your site has no page home",
    "a slug is lowercase letters and digits joined by hyphens, like about-me, not About Me",
    "your site has no page yet; write one first",
    "your site is not published",
  ]);
  expect(refused.map((one) => digest(one.world.apps))).toEqual(refused.map(() => digest(drafted.apps)));
  expect(() => K.call(drafted, { verb: "sites.publish", args: { on: true }, who: as("guest") })).toThrow("refused");
});

test("a site reads at sites/<user>/<page> once published, its owner reads it always, and the root lists the published", () => {
  const published = play(drafted, [["test-1", "publish", { on: true }]]);
  expect([K.read(drafted, "sites/test-1").state, K.read(drafted, "sites").state]).toEqual([null, { published: [] }]);
  expect(K.read(drafted, "sites/@test-1/pages/home/path").state).toBe("sites/test-1/home");
  expect(K.read(published, "sites/test-1/pages/home/blocks").state).toEqual(HOME);
  expect(K.read(published, "sites").state).toEqual({ published: ["test-1"] });
  expect(K.read(published, "sites/@test-2").state).toEqual({ path: "sites/test-2", published: false, pages: {} });
});

test(`a site holds at most ${PAGES} pages, and a drop removes one`, () => {
  const many = Array.from({ length: PAGES - 1 }, (_, i): [string, string, Act["args"]] => [
    "test-1",
    "page",
    { slug: `p${i}`, title: "p", blocks: [] },
  ]);
  const full = play(drafted, many);
  const over = K.call(full, { verb: "sites.page", args: { slug: "more", title: "m", blocks: [] }, who: as("test-1") });
  const again = K.call(full, {
    verb: "sites.page",
    args: { slug: "home", title: "Home again", blocks: [] },
    who: as("test-1"),
  });
  const dropped = play(full, [["test-1", "drop", { slug: "home" }]]);
  expect([
    over.envelope.last.note,
    again.envelope.last.ok,
    Object.keys((K.read(dropped, "sites/@test-1/pages").state ?? {}) as object).length,
  ]).toEqual([`your site holds ${PAGES} pages already; drop one first`, true, PAGES - 1]);
});

test("the scripted test users write, publish and drop pages with no refusal", () => {
  const run = scripted([sites], CONFIG, 150);
  const counts = ["sites.page", "sites.publish", "sites.drop"].map((verb) => run.verbs[verb]! > 5);
  expect([run.refused, counts]).toEqual([{}, [true, true, true]]);
});

test("10,000 seeded worlds of sites played by the goose through the gym replay to their digests", async () => {
  const played = await geese(CONFIG, 10_000, 6);
  expect([played.same, played.acts, played.refused > 0]).toEqual([10_000, 60_000, true]);
}, 30_000);
