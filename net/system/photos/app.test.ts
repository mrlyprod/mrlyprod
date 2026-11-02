import { expect, test } from "bun:test";
import type { Act, Who, World } from "../../kernel/index.ts";
import { digest, kernel } from "../../kernel/index.ts";
import { geese, scripted } from "../../test/gym.ts";
import type { Library, Photos } from "./app.ts";
import photos, { PHOTOS, TITLE } from "./app.ts";
import { hashOf, image, SIZE } from "./image.ts";

const ACTORS = Array.from({ length: 8 }, (_, i) => `test-${i + 1}`);

const CONFIG = { name: "photos", apps: ["photos"], actors: ACTORS };

const K = kernel([photos]);

const as = (id: string): Who => ({ id, kind: "human" });

const state = (world: World) => world.apps.photos as Photos;

const play = (world: World, acts: [string, string, Act["args"]][]) =>
  acts.reduce((at, [id, verb, args]) => K.call(at, { verb: `photos.${verb}`, args, who: as(id) }).world, world);

const shot = play(K.boot(CONFIG), [
  ["test-1", "take", { title: "Sunset" }],
  ["test-1", "album", { title: "Holiday" }],
  ["test-1", "add", { photo: 1, album: 2 }],
]);

test("a take, album, add or delete out of its schema is a bad act and changes nothing", () => {
  const bad: [string, Act["args"]][] = [
    ["take", { title: "x".repeat(TITLE + 1) }],
    ["take", { title: "a", pixels: [1, 2, 3] }],
    ["album", {}],
    ["add", { photo: 1 }],
    ["delete", { photo: 9 }],
  ];
  for (const [verb, args] of bad)
    expect(() => K.call(shot, { verb: `photos.${verb}`, args, who: as("test-1") })).toThrow("bad-act");
});

test("only a library's owner deletes or files its photos, and a watcher may not take one", () => {
  const notes = [
    K.call(shot, { verb: "photos.delete", args: { photo: 1 }, who: as("test-2") }),
    K.call(shot, { verb: "photos.remove", args: { photo: 1, album: 2 }, who: as("test-2") }),
    K.call(shot, { verb: "photos.add", args: { photo: 1, album: 2 }, who: as("test-1") }),
  ];
  expect(notes.map((one) => one.envelope.last.note)).toEqual([
    "photo 1 is test-1's",
    "photo 1 is test-1's",
    "photo 1 is already in Holiday",
  ]);
  expect(notes.map((one) => digest(one.world.apps))).toEqual(notes.map(() => digest(shot.apps)));
  expect(() => K.call(shot, { verb: "photos.take", args: {}, who: as("guest") })).toThrow("refused");
});

test("a photo names its image by the content hash of pixels drawn from a seed, kept once and swept when unused", () => {
  const { seed } = state(shot).images[state(shot).libraries["test-1"]!.photos[0]!.hash]!;
  const pixels = image(seed);
  const gone = play(shot, [["test-1", "delete", { photo: 1 }]]);
  expect([pixels.rgb.length, pixels.rgb.every((one) => Number.isInteger(one) && one >= 0 && one < 256)]).toEqual([
    SIZE * SIZE * 3,
    true,
  ]);
  expect(state(shot).libraries["test-1"]!.photos[0]!.hash).toBe(hashOf(pixels));
  expect([state(gone).images, (K.read(gone, "photos/@test-1").state as Library).albums[0]!.photos]).toEqual([{}, []]);
});

test(`a library keeps at most ${PHOTOS} photos`, () => {
  const full = play(
    K.boot(CONFIG),
    Array.from({ length: PHOTOS }, (): [string, string, Act["args"]] => ["test-1", "take", {}]),
  );
  expect(K.call(full, { verb: "photos.take", args: {}, who: as("test-1") }).envelope.last.note).toBe(
    `you keep ${PHOTOS} photos; delete one first`,
  );
});

test("the scripted test users take photos, make albums and file them, with no refusal", () => {
  const run = scripted([photos], CONFIG, 150);
  const counts = ["photos.take", "photos.album", "photos.add", "photos.remove"].map((verb) => run.verbs[verb]! > 5);
  expect([run.refused, counts]).toEqual([{}, [true, true, true, true]]);
});

test("10,000 seeded worlds of photos played by the goose through the gym replay to their digests", async () => {
  const played = await geese(CONFIG, 10_000, 6);
  expect([played.same, played.acts, played.refused > 0]).toEqual([10_000, 60_000, true]);
}, 30_000);
