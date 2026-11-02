import { expect, test } from "bun:test";
import type { Act, Who, World } from "../../kernel/index.ts";
import { digest, kernel } from "../../kernel/index.ts";
import { geese } from "../../test/gym.ts";
import type { Maps } from "./app.ts";
import maps, { inside, NAME, OWN, RADIUS } from "./app.ts";

const ACTORS = Array.from({ length: 8 }, (_, i) => `test-${i + 1}`);

const CONFIG = { name: "maps", apps: ["maps"], actors: ACTORS };

const K = kernel([maps]);

const as = (id: string): Who => ({ id, kind: "human" });

const state = (world: World) => world.apps.maps as Maps;

const play = (world: World, acts: [string, string, Act["args"]][]) =>
  acts.reduce((at, [id, verb, args]) => K.call(at, { verb: `maps.${verb}`, args, who: as(id) }).world, world);

test(`the map boots as every hex cell within ${RADIUS} of the centre, each with a terrain drawn from the seed`, () => {
  const one = state(K.boot({ ...CONFIG, seed: 1 }));
  const count = 3 * RADIUS * (RADIUS + 1) + 1;
  const keys = Object.keys(one.cells);
  expect([
    keys.length,
    keys.every((key) => inside(...(key.split(",").map(Number) as [number, number]), RADIUS)),
  ]).toEqual([count, true]);
  expect(new Set(Object.values(one.cells)).size).toBeGreaterThan(3);
  expect(digest(K.boot({ ...CONFIG, seed: 1 }))).toBe(digest(K.boot({ ...CONFIG, seed: 1 })));
  expect(digest(K.boot({ ...CONFIG, seed: 2 }))).not.toBe(digest(K.boot({ ...CONFIG, seed: 1 })));
});

test("a pin, a rename or an unpin out of its schema is a bad act and changes nothing", () => {
  const world = play(K.boot(CONFIG), [["test-1", "pin", { q: 0, r: 0, name: "Home" }]]);
  const bad: [string, Act["args"]][] = [
    ["pin", { q: RADIUS + 1, r: 0, name: "Home" }],
    ["pin", { q: 0.5, r: 0, name: "Home" }],
    ["pin", { q: 0, r: 0, name: "x".repeat(NAME + 1) }],
    ["pin", { q: 0, r: 0 }],
    ["pin", { q: 0, r: 0, name: "Home", by: "test-2" }],
    ["rename", { place: 2, name: "Mill" }],
    ["unpin", {}],
  ];
  for (const [verb, args] of bad)
    expect(() => K.call(world, { verb: `maps.${verb}`, args, who: as("test-1") })).toThrow("bad-act");
});

test("only the pinner renames or unpins a place, a corner off the hex is refused with a plain note, and a watcher may not pin", () => {
  const world = play(K.boot(CONFIG), [["test-1", "pin", { q: 0, r: 0, name: "Home" }]]);
  const notes = [
    ["test-2", "rename", { place: 1, name: "Mine" }],
    ["test-2", "unpin", { place: 1 }],
    ["test-1", "pin", { q: RADIUS, r: RADIUS, name: "Corner" }],
  ].map(([id, verb, args]) => {
    const done = K.call(world, { verb: `maps.${verb}`, args: args as Act["args"], who: as(id as string) });
    expect(digest(done.world.apps)).toBe(digest(world.apps));
    return done.envelope.last.note;
  });
  expect(notes).toEqual([
    "place 1 is test-1's; only they change it",
    "place 1 is test-1's; only they change it",
    `(${RADIUS}, ${RADIUS}) is off the map; q + r stays within ${RADIUS} of the centre`,
  ]);
  expect(() => K.call(world, { verb: "maps.pin", args: { q: 0, r: 0, name: "x" }, who: as("guest") })).toThrow(
    "refused",
  );
});

test(`a place sits on its cell with the cell's terrain, is read by cell or by owner, and one person pins ${OWN}`, () => {
  const world = play(K.boot(CONFIG), [
    ["test-1", "pin", { q: 2, r: -1, name: "Mill" }],
    ["test-2", "pin", { q: 2, r: -1, name: "Bridge" }],
    ["test-1", "rename", { place: 1, name: "Old Mill" }],
  ]);
  const terrain = state(world).cells["2,-1"];
  expect(K.read(world, "maps/cell/2/-1", ["terrain", "places"]).state).toEqual({
    terrain,
    places: [
      { id: 1, q: 2, r: -1, name: "Old Mill", by: "test-1", at: 0 },
      { id: 2, q: 2, r: -1, name: "Bridge", by: "test-2", at: 0 },
    ],
  });
  expect(K.read(world, "maps/@test-1/places", ["name", "terrain"]).state).toEqual([{ name: "Old Mill", terrain }]);
  expect(K.read(world, "maps/cell/9/9").state).toBeNull();
  const gone = play(world, [["test-1", "unpin", { place: 1 }]]);
  expect(Object.keys(state(gone).places)).toEqual(["2"]);
  const full = play(
    K.boot(CONFIG),
    Array.from({ length: OWN }, (_, i): [string, string, Act["args"]] => [
      "test-3",
      "pin",
      { q: 0, r: i % 3, name: "Spot" },
    ]),
  );
  expect(
    K.call(full, { verb: "maps.pin", args: { q: 0, r: 0, name: "One more" }, who: as("test-3") }).envelope.last.note,
  ).toBe(`you pinned ${OWN} places; unpin one first`);
});

test("10,000 seeded worlds of maps played by the goose through the gym replay to their digests", async () => {
  const played = await geese(CONFIG, 10_000, 6);
  expect([played.same, played.acts, played.refused > 0]).toEqual([10_000, 60_000, true]);
}, 30_000);
