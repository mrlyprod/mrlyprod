import { expect, test } from "bun:test";
import type { Act, Who, World } from "../../kernel/index.ts";
import { digest, kernel } from "../../kernel/index.ts";
import { geese } from "../../test/gym.ts";
import type { Care } from "./app.ts";
import care, { AHEAD, SLOT, UPCOMING } from "./app.ts";

const ACTORS = Array.from({ length: 8 }, (_, i) => `test-${i + 1}`);

const CONFIG = { name: "health", apps: ["health"], actors: ACTORS };

const K = kernel([care]);

const as = (id: string): Who => ({ id, kind: "human" });

const state = (world: World) => world.apps.health as Care;

type Step = [string, string, Act["args"], number?];

const play = (world: World, acts: Step[]) =>
  acts.reduce(
    (now, [id, verb, args, at]) => K.call(now, { verb: `health.${verb}`, args, who: as(id), at }).world,
    world,
  );

const notes = (world: World, acts: Step[]) =>
  acts.map(([id, verb, args, at]) => {
    const done = K.call(world, { verb: `health.${verb}`, args, who: as(id), at });
    expect(digest(done.world.apps)).toBe(digest(world.apps));
    return done.envelope.last.note;
  });

const CLINIC: Step[] = [
  ["test-1", "open", { name: "Clinic 1" }],
  ["test-1", "hire", { clinic: 1, person: "test-2" }],
];

test("an open, hire, leave, book or cancel out of its schema is a bad act and changes nothing", () => {
  const world = play(K.boot(CONFIG), [...CLINIC, ["test-3", "book", { clinic: 1, with: "test-1", in: 60 }]]);
  const bad: [string, Act["args"]][] = [
    ["open", { name: "" }],
    ["hire", { clinic: 2, person: "test-4" }],
    ["book", { clinic: 1, with: "test-9", in: 60 }],
    ["book", { clinic: 1, with: "test-1", in: 0 }],
    ["book", { clinic: 1, with: "test-1", in: AHEAD + 1 }],
    ["book", { clinic: 1, with: "test-1", in: 60, for: "test-5" }],
    ["cancel", { visit: 2 }],
  ];
  for (const [verb, args] of bad)
    expect(() => K.call(world, { verb: `health.${verb}`, args, who: as("test-4") })).toThrow("bad-act");
});

test("only who runs a clinic hires, only a visit's two people cancel it, nobody books themselves, and a watcher may not book", () => {
  const world = play(K.boot(CONFIG), [...CLINIC, ["test-3", "book", { clinic: 1, with: "test-1", in: 60 }]]);
  expect(
    notes(world, [
      ["test-2", "hire", { clinic: 1, person: "test-4" }],
      ["test-4", "cancel", { visit: 1 }],
      ["test-1", "book", { clinic: 1, with: "test-1", in: 60 }],
      ["test-4", "book", { clinic: 1, with: "test-3", in: 60 }],
    ]),
  ).toEqual([
    "only test-1, who runs Clinic 1, adds practitioners",
    "only test-3 and test-1 cancel visit 1",
    "you cannot book a visit with yourself",
    "test-3 does not practise at Clinic 1",
  ]);
  expect(() =>
    K.call(world, { verb: "health.book", args: { clinic: 1, with: "test-1", in: 60 }, who: as("guest") }),
  ).toThrow("refused");
});

test("a visit starts at the next slot, one person in one slot at a time, a few ahead each, and is cancelled only before it begins", () => {
  const world = play(K.boot(CONFIG), [...CLINIC, ["test-3", "book", { clinic: 1, with: "test-1", in: 50 }, 1000]]);
  const full = play(world, [
    ["test-3", "book", { clinic: 1, with: "test-2", in: 200 }],
    ["test-3", "book", { clinic: 1, with: "test-2", in: 400 }],
  ]);
  expect(state(world).visits["1"]!.start).toBe(4 * SLOT);
  expect(
    notes(full, [
      ["test-4", "book", { clinic: 1, with: "test-1", in: 46 }],
      ["test-3", "book", { clinic: 1, with: "test-2", in: 600 }],
      ["test-2", "book", { clinic: 1, with: "test-1", in: 200 }],
      ["test-3", "cancel", { visit: 1 }, 4 * SLOT],
    ]),
  ).toEqual([
    "test-1 is booked at that time; pick another",
    `you have ${UPCOMING} visits ahead; cancel one first`,
    "you are booked at that time; pick another",
    "visit 1 has begun; it cannot be cancelled",
  ]);
});

test("a practitioner who leaves cancels their visits ahead, and the last to leave closes the clinic", () => {
  const world = play(K.boot(CONFIG), [
    ...CLINIC,
    ["test-3", "book", { clinic: 1, with: "test-1", in: 60 }],
    ["test-4", "book", { clinic: 1, with: "test-2", in: 60 }],
    ["test-1", "leave", { clinic: 1 }],
  ]);
  const shut = play(world, [["test-2", "leave", { clinic: 1 }]]);
  const clinic = state(world).clinics["1"]!;
  expect([
    clinic.by,
    clinic.practitioners,
    state(world).visits["1"]!.cancelled,
    state(world).visits["2"]!.cancelled,
  ]).toEqual(["test-2", ["test-2"], 0, null]);
  expect([state(shut).clinics["1"]!.closed, state(shut).visits["2"]!.cancelled]).toEqual([0, 0]);
  expect(notes(shut, [["test-5", "book", { clinic: 1, with: "test-2", in: 60 }]])).toEqual(["Clinic 1 has closed"]);
  expect(K.read(world, "health/@test-2/practises").state).toEqual([1]);
});

test("10,000 seeded worlds of care played by the goose through the gym replay to their digests", async () => {
  const played = await geese(CONFIG, 10_000, 6);
  expect([played.same, played.acts, played.refused > 0]).toEqual([10_000, 60_000, true]);
}, 30_000);
