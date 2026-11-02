import { expect, test } from "bun:test";
import type { Act, Obj, Who, World } from "../../kernel/index.ts";
import { digest, kernel } from "../../kernel/index.ts";
import { named } from "../../sdk/system.ts";
import { geese, scripted } from "../../test/gym.ts";
import type { People } from "./app.ts";
import people, { cards as cardsOf, NAME, ORG } from "./app.ts";
import { NAMES } from "./bot.ts";

const ACTORS = Array.from({ length: 8 }, (_, i) => `test-${i + 1}`);

const CONFIG = { name: "people", apps: ["people"], actors: ACTORS };

const K = kernel([people]);

const as = (id: string): Who => ({ id, kind: "human" });

const state = (world: World) => world.apps.people as People;

const cards = (world: World) => cardsOf(state(world));

const play = (world: World, acts: [string, string, Act["args"]][]) =>
  acts.reduce((at, [id, verb, args]) => K.call(at, { verb: `people.${verb}`, args, who: as(id) }).world, world);

const notes = (world: World, acts: [string, string, Act["args"]][]) =>
  acts.map(([id, verb, args]) => {
    const done = K.call(world, { verb: `people.${verb}`, args, who: as(id) });
    expect(digest(done.world.apps)).toBe(digest(world.apps));
    return done.envelope.last.note;
  });

const BAKERY: [string, string, Act["args"]] = ["test-1", "found", { name: "Bakery", open: false }];

test("each test user boots with its name, a distinct hue and a mirrored 5x5 glyph, the same for a seed and another for the next", () => {
  const one = cards(K.boot({ ...CONFIG, seed: 1 }));
  const hues = Object.values(one).map((card) => card.hue);
  const mirrored = Object.values(one).every((card) =>
    card.glyph.split(" ").every((row) => row.length === 5 && row === [...row].reverse().join("")),
  );
  expect([Object.keys(one), one["test-3"]!.name, new Set(hues).size, mirrored]).toEqual([ACTORS, "Test 3", 8, true]);
  expect(digest(K.boot({ ...CONFIG, seed: 1 }))).toBe(digest(K.boot({ ...CONFIG, seed: 1 })));
  expect(digest(K.boot({ ...CONFIG, seed: 2 }))).not.toBe(digest(K.boot({ ...CONFIG, seed: 1 })));
});

test("the kinds rule sets each test user's kind: one kind for all when pinned, human with no rule, drawn per user from the seed and its id from a list, and a bad rule stops the boot", () => {
  const mixed = { ...CONFIG, seed: 1, rules: { kinds: ["human", "agent", "bot"] } };
  const kinds = (world: World) => Object.values(cards(world)).map((card) => card.kind);
  const drawn = kinds(K.boot(mixed));
  expect(new Set(kinds(K.boot({ ...CONFIG, rules: { kinds: "agent" } })))).toEqual(new Set(["agent"]));
  expect(new Set(kinds(K.boot(CONFIG)))).toEqual(new Set(["human"]));
  expect([new Set(drawn).size > 1, kinds(K.boot(mixed))]).toEqual([true, drawn]);
  expect(kinds(K.boot({ ...mixed, actors: ["test-3"] }))).toEqual([drawn[2]!]);
  expect(() => K.boot({ ...CONFIG, rules: { kinds: "alien" } })).toThrow("kinds");
});

test("a rename out of its schema is a bad act and changes nothing", () => {
  const world = K.boot(CONFIG);
  const bad: Obj[] = [{ name: "" }, { name: "x".repeat(NAME + 1) }, { name: 7 }, { name: "Ada", id: "test-2" }, {}];
  for (const args of bad)
    expect(() => K.call(world, { verb: "people.rename", args, who: as("test-1") })).toThrow("bad-act");
});

test("a rename touches only its caller's card, refuses a name another holds, and a watcher may not rename at all", () => {
  let world = K.boot(CONFIG);
  world = K.call(world, { verb: "people.rename", args: { name: "Ada" }, who: as("test-2") }).world;
  const taken = K.call(world, { verb: "people.rename", args: { name: "ada" }, who: as("test-5") });
  expect([cards(world)["test-2"]!.name, cards(world)["test-1"]!.name]).toEqual(["Ada", "Test 1"]);
  expect([taken.envelope.last.ok, taken.envelope.last.note]).toEqual([
    false,
    "test-2 is already called Ada; pick another name",
  ]);
  expect(digest(taken.world.apps)).toBe(digest(world.apps));
  expect(() => K.call(world, { verb: "people.rename", args: { name: "Eve" }, who: as("guest") })).toThrow("refused");
});

test("a found, join, admit, leave or role out of its schema is a bad act and changes nothing", () => {
  const world = play(K.boot(CONFIG), [BAKERY]);
  const bad: [string, Act["args"]][] = [
    ["found", { name: "", open: true }],
    ["found", { name: "x".repeat(ORG + 1), open: true }],
    ["found", { name: "Mill" }],
    ["join", { org: 2 }],
    ["admit", { org: 1, person: "test-9" }],
    ["role", { org: 1, person: "test-2", role: "king" }],
    ["leave", { org: 1, as: "test-1" }],
  ];
  for (const [verb, args] of bad)
    expect(() => K.call(world, { verb: `people.${verb}`, args, who: as("test-2") })).toThrow("bad-act");
});

test("only an owner or admin admits, only an owner sets roles, nobody joins twice, and a watcher may not found", () => {
  const world = play(K.boot(CONFIG), [BAKERY, ["test-2", "join", { org: 1 }], ["test-3", "join", { org: 1 }]]);
  const member = play(world, [["test-1", "admit", { org: 1, person: "test-2" }]]);
  expect(
    notes(member, [
      ["test-2", "admit", { org: 1, person: "test-3" }],
      ["test-2", "role", { org: 1, person: "test-2", role: "owner" }],
      ["test-2", "join", { org: 1 }],
      ["test-3", "join", { org: 1 }],
      ["test-1", "admit", { org: 1, person: "test-4" }],
    ]),
  ).toEqual([
    "only an owner or an admin of Bakery admits members",
    "only an owner of Bakery sets roles",
    "you are already a member of Bakery",
    "you already asked to join Bakery; an owner or admin admits you",
    "test-4 has not asked to join Bakery",
  ]);
  expect(() => K.call(world, { verb: "people.found", args: { name: "Mill", open: true }, who: as("guest") })).toThrow(
    "refused",
  );
});

test("an org keeps an owner while it has members, an open one takes joiners at once, and the last to leave dissolves it", () => {
  const world = play(K.boot(CONFIG), [
    BAKERY,
    ["test-2", "found", { name: "Garden", open: true }],
    ["test-3", "join", { org: 2 }],
  ]);
  expect(
    notes(world, [
      ["test-2", "leave", { org: 2 }],
      ["test-2", "role", { org: 2, person: "test-2", role: "member" }],
      ["test-4", "found", { name: "garden", open: true }],
    ]),
  ).toEqual([
    "you are the only owner of Garden; make another member owner first",
    "Garden needs an owner; make another member owner first",
    "org 2 is already called Garden; pick another name",
  ]);
  const handed = play(world, [
    ["test-2", "role", { org: 2, person: "test-3", role: "owner" }],
    ["test-2", "leave", { org: 2 }],
  ]);
  const gone = play(handed, [["test-3", "leave", { org: 2 }]]);
  expect([state(handed).orgs["2"]!.members, state(gone).orgs["2"]!.dissolved]).toEqual([{ "test-3": "owner" }, 0]);
  expect(notes(gone, [["test-5", "join", { org: 2 }]])).toEqual(["Garden was dissolved"]);
  expect(Object.keys(state(play(gone, [["test-5", "found", { name: "Garden", open: true }]])).orgs)).toEqual([
    "1",
    "2",
    "3",
  ]);
});

test("people reads the cards, people/<id> one card, people/orgs the organisations, people/orgs/<n> one, and people/@<id> a user's orgs and asks", () => {
  const world = play(K.boot(CONFIG), [BAKERY, ["test-2", "join", { org: 1 }]]);
  expect(K.read(world, "people").state).toEqual(cards(world));
  expect(K.read(world, "people/test-3/name").state).toBe("Test 3");
  expect(Object.keys(K.read(world, "people/orgs").state as Obj)).toEqual(["1"]);
  expect(K.read(world, "people/orgs/1", ["name", "open", "members", "asking"]).state).toEqual({
    name: "Bakery",
    open: false,
    members: { "test-1": "owner" },
    asking: ["test-2"],
  });
  expect([K.read(world, "people/@test-1").state, K.read(world, "people/@test-2").state]).toEqual([
    { orgs: { 1: "owner" }, asking: [] },
    { orgs: {}, asking: [1] },
  ]);
  expect(K.read(world, "people/orgs/2").state).toBe(null);
});

test("the scripted test users rename themselves rarely, each to a distinct first name of the list, found and join orgs, with no refusal", () => {
  const run = scripted([people], CONFIG, 150);
  const renames = run.verbs["people.rename"] ?? 0;
  const names = Object.entries(cards(run.world)).map(([id, card]) => [id, card.name] as const);
  const plausible = names.every(([id, name]) => NAMES.includes(name) || name === named(id));
  expect([run.refused, renames > 50 && renames < 400, plausible, new Set(names.map(([, name]) => name)).size]).toEqual([
    {},
    true,
    true,
    8,
  ]);
  expect([run.verbs["people.found"]! > 0, run.verbs["people.join"]! > 0, Object.keys(cards(run.world)).length]).toEqual(
    [true, true, 8],
  );
});

test("10,000 seeded worlds of people played by the goose through the gym replay to their digests", async () => {
  const played = await geese(CONFIG, 10_000, 6);
  expect([played.same, played.acts, played.refused > 0]).toEqual([10_000, 60_000, true]);
}, 30_000);
