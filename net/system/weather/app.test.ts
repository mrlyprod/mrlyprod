import { expect, test } from "bun:test";
import type { Act, Who, World } from "../../kernel/index.ts";
import { digest, kernel } from "../../kernel/index.ts";
import { geese } from "../../test/gym.ts";
import type { Weather } from "./app.ts";
import weather, { AHEAD, DAY, NAME, OWN, RADIUS, report } from "./app.ts";

const ACTORS = Array.from({ length: 8 }, (_, i) => `test-${i + 1}`);

const CONFIG = { name: "weather", apps: ["weather"], actors: ACTORS };

const K = kernel([weather]);

const as = (id: string): Who => ({ id, kind: "human" });

const state = (world: World) => world.apps.weather as Weather;

const play = (world: World, acts: [string, string, Act["args"]][]) =>
  acts.reduce((at, [id, verb, args]) => K.call(at, { verb: `weather.${verb}`, args, who: as(id) }).world, world);

test("a report is a pure function of seed, cell and day, in whole degrees and km/h, colder far from the centre row and in winter", () => {
  const one = report(7, 2, -1, 40);
  expect(one).toEqual(report(7, 2, -1, 40));
  expect([one.q, one.r, one.day, Number.isInteger(one.high), one.low < one.high, one.wind >= 0]).toEqual([
    2,
    -1,
    40,
    true,
    true,
    true,
  ]);
  expect(one).not.toEqual(report(8, 2, -1, 40));
  expect(one).not.toEqual(report(7, 2, -1, 41));
  const mean = (r: number, day: number) =>
    Array.from({ length: 50 }, (_, seed) => report(seed, 0, r, day).high).reduce((a, b) => a + b, 0) / 50;
  expect(mean(0, 200) - mean(RADIUS, 200)).toBeCloseTo(RADIUS, 0);
  expect(mean(0, 200)).toBeGreaterThan(mean(0, 10));
});

test("a save or a forget out of its schema is a bad act and changes nothing", () => {
  const world = play(K.boot(CONFIG), [["test-1", "save", { q: 0, r: 0, name: "Home" }]]);
  const bad: [string, Act["args"]][] = [
    ["save", { q: RADIUS + 1, r: 0, name: "Home" }],
    ["save", { q: 0, r: 0, name: "" }],
    ["save", { q: 0, r: 0, name: "x".repeat(NAME + 1) }],
    ["save", { q: 0, r: 0, name: "Home", day: 3 }],
    ["forget", {}],
    ["forget", { name: 4 }],
  ];
  for (const [verb, args] of bad)
    expect(() => K.call(world, { verb: `weather.${verb}`, args, who: as("test-1") })).toThrow("bad-act");
});

test("saved places are one person's own: a forget of a name you never saved, a name saved twice or a cell off the map is refused, and a watcher may not save", () => {
  const world = play(K.boot(CONFIG), [["test-1", "save", { q: 0, r: 0, name: "Home" }]]);
  const notes = [
    ["test-2", "forget", { name: "Home" }],
    ["test-1", "save", { q: 1, r: 1, name: "Home" }],
    ["test-1", "save", { q: RADIUS, r: RADIUS, name: "Corner" }],
  ].map(([id, verb, args]) => {
    const done = K.call(world, { verb: `weather.${verb}`, args: args as Act["args"], who: as(id as string) });
    expect(digest(done.world.apps)).toBe(digest(world.apps));
    return done.envelope.last.note;
  });
  expect(notes).toEqual([
    "you saved no place called Home",
    "you saved a place called Home already; forget it first",
    `(${RADIUS}, ${RADIUS}) is off the map; q + r stays within ${RADIUS} of the centre`,
  ]);
  expect(() => K.call(world, { verb: "weather.save", args: { q: 0, r: 0, name: "x" }, who: as("guest") })).toThrow(
    "refused",
  );
});

test(`a read by cell answers any day, a slice answers today and ${AHEAD} days ahead for each saved place, and the day follows the acts`, () => {
  const booted = K.boot(CONFIG);
  const seed = state(booted).seed;
  const world = K.call(booted, {
    verb: "weather.save",
    args: { q: 3, r: -2, name: "Work" },
    who: as("test-1"),
    at: 5 * DAY,
  }).world;
  expect(K.read(world, "weather/at/3/-2/9").state).toEqual(report(seed, 3, -2, 9));
  expect(K.read(world, "weather/at/3/-2").state).toEqual(report(seed, 3, -2, 5));
  expect(K.read(world, "weather/at/9/9").state).toBeNull();
  const mine = K.read(world, "weather/@test-1").state as { day: number; places: { name: string; ahead: unknown[] }[] };
  expect([mine.day, mine.places[0]!.name, mine.places[0]!.ahead]).toEqual([
    5,
    "Work",
    Array.from({ length: AHEAD + 1 }, (_, i) => report(seed, 3, -2, 5 + i)),
  ]);
  const full = play(
    K.boot(CONFIG),
    Array.from({ length: OWN }, (_, i): [string, string, Act["args"]] => [
      "test-3",
      "save",
      { q: 0, r: 0, name: `P${i}` },
    ]),
  );
  expect(
    K.call(full, { verb: "weather.save", args: { q: 0, r: 0, name: "More" }, who: as("test-3") }).envelope.last.note,
  ).toBe(`you saved ${OWN} places; forget one first`);
});

test("10,000 seeded worlds of weather played by the goose through the gym replay to their digests", async () => {
  const played = await geese(CONFIG, 10_000, 6);
  expect([played.same, played.acts, played.refused > 0]).toEqual([10_000, 60_000, true]);
}, 30_000);
