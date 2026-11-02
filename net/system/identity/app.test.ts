import { expect, test } from "bun:test";
import type { Obj, Who, World } from "../../kernel/index.ts";
import { digest, kernel } from "../../kernel/index.ts";
import { geese, scripted } from "../../test/gym.ts";
import type { Card } from "./app.ts";
import iden, { HANDLE } from "./app.ts";

const ACTORS = Array.from({ length: 8 }, (_, i) => `test-${i + 1}`);

const CONFIG = { name: "identity", apps: ["identity"], actors: ACTORS };

const K = kernel([iden]);

const as = (id: string): Who => ({ id, kind: "human" });

const card = (world: World, id: string) => K.read(world, `identity/@${id}`).state as Card;

test("a handle out of its schema is a bad act and changes nothing", () => {
  const world = K.boot(CONFIG);
  const bad: Obj[] = [{}, { handle: "ab" }, { handle: "x".repeat(HANDLE + 1) }, { handle: "ada", kind: "bot" }];
  for (const args of bad)
    expect(() => K.call(world, { verb: "identity.handle", args, who: as("test-1") })).toThrow("bad-act");
});

test("a handle changes the caller's card alone, never one another holds or a malformed one, and a watcher may not act", () => {
  const world = K.call(K.boot(CONFIG), { verb: "identity.handle", args: { handle: "ada" }, who: as("test-2") }).world;
  const refused = [
    K.call(world, { verb: "identity.handle", args: { handle: "ada" }, who: as("test-5") }),
    K.call(world, { verb: "identity.handle", args: { handle: "Ada!" }, who: as("test-5") }),
  ];
  expect([card(world, "test-2").handle, card(world, "test-1").handle]).toEqual(["ada", "test-1"]);
  expect(refused.map((one) => one.envelope.last.note)).toEqual([
    "test-2 holds the handle ada; pick another",
    "a handle starts with a letter and holds lowercase letters, digits and hyphens, not Ada!",
  ]);
  expect(refused.map((one) => digest(one.world.apps))).toEqual(refused.map(() => digest(world.apps)));
  expect(() => K.call(world, { verb: "identity.handle", args: { handle: "eve" }, who: as("guest") })).toThrow(
    "refused",
  );
});

test("every test user boots with a card: its id as handle, kind human, created at tick 0", () => {
  const world = K.boot(CONFIG);
  expect(card(world, "test-3")).toEqual({ handle: "test-3", kind: "human", created: 0 });
  expect(Object.keys(K.read(world, "identity/cards").state as object)).toEqual(ACTORS);
});

test("the scripted test users pick handles with no refusal", () => {
  const run = scripted([iden], CONFIG, 40);
  expect([run.refused, run.verbs["identity.handle"]! >= 5]).toEqual([{}, true]);
});

test("10,000 seeded worlds of iden played by the goose through the gym replay to their digests", async () => {
  const played = await geese(CONFIG, 10_000, 4);
  expect([played.same, played.acts]).toEqual([10_000, 40_000]);
}, 30_000);
