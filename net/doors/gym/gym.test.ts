import { afterAll, expect, test } from "bun:test";
import { readFileSync, rmSync } from "node:fs";
import { join } from "node:path";
import { install, site, user } from "../../hosts/local/apps.ts";
import { boot, DATA_DIR } from "../../hosts/local/store.ts";
import type { Act, Envelope, Line, Move, Who, World } from "../../kernel/index.ts";
import { digest, kernel, parse, replay, seedOf, stream } from "../../kernel/index.ts";
import { goose } from "../../sim/goose.ts";
import { APPS } from "../../test/apps.ts";
import type { Ask, Failed } from "./gym.ts";
import { BEAT, BOT, gym } from "./gym.ts";

const WORLDS = 10_000;

const ACTS = 3;

const CONFIG = { name: "snake", apps: ["snake"] };

const ME = user(site());

const ROOT = join(DATA_DIR, "test-gym", String(process.pid));

afterAll(() => rmSync(join(DATA_DIR, "test-gym"), { recursive: true, force: true }));

const drawFor = (world: World) => stream(seedOf((world.seed ^ world.tick) >>> 0, "goose")).draw;

test("10,000 seeded worlds stepped through the gym in batches by the goose replay identically from their lines", async () => {
  const one = gym(APPS);
  const ask = async (body: Ask) => JSON.parse(await one.answer(JSON.stringify(body)));
  const seeds = Array.from({ length: WORLDS }, (_, i) => i);
  const booted: Envelope[] = await ask({ op: "boot", config: CONFIG, seeds });
  expect(booted.map((envelope) => envelope.tick)).toEqual(seeds.map(() => 0));
  const lines: Line[][] = seeds.map(() => []);
  for (let round = 0; round < ACTS; round++) {
    const moves: Move[][] = await ask({ op: "list" });
    const before = one.worlds() as World[];
    const acts = moves.map((legal, i) => goose(legal, drawFor(before[i]), ["nav.open"]) as Act);
    const stepped: Envelope[] = await ask({ op: "call", acts });
    stepped.forEach((envelope, i) => {
      const { verb, args = {} } = acts[i];
      lines[i].push({ tick: envelope.tick, at: before[i].at + BEAT, who: BOT, verb, args, n: 1 });
    });
  }
  const after = one.worlds() as World[];
  const k = kernel(await install(["snake"], APPS));
  const same = seeds.filter((seed, i) => digest(replay(k, { ...CONFIG, seed }, lines[i])) === digest(after[i]));
  expect([same.length, after.every((world) => world.tick === ACTS)]).toEqual([WORLDS, true]);
}, 30_000);

test("a fault stays in its own world: the batch around it boots, steps and answers", async () => {
  const one = gym(APPS);
  const ask = async (body: Ask | string) =>
    JSON.parse(await one.answer(typeof body === "string" ? body : JSON.stringify(body)));
  const code = (out: (Envelope | Failed)[]) => out.map((item) => ("error" in item ? item.error.code : item.tick));
  const booted = await ask({ op: "boot", configs: [CONFIG, { name: "x", apps: ["snak"] }, CONFIG] });
  const tick = { verb: `${ME}.snake.tick` };
  const stepped = await ask({ op: "call", acts: [tick, tick, { verb: `${ME}.snake.tock` }] });
  const skipped = await ask({ op: "call", acts: [null, null, tick] });
  expect([code(booted), code(stepped), code(skipped)]).toEqual([
    [0, "bad-config", 0],
    [1, "unknown-world", "unknown-verb"],
    [1, "unknown-world", 1],
  ]);
  expect([(await ask("nope")).error.code, (await ask({ op: "call", acts: [] })).error.code]).toEqual([
    "bad-act",
    "bad-act",
  ]);
});

test("one act sequence is one journal through any door: the gym runs the bus as the local store does, and its lines op hands the journal back", async () => {
  const config = { name: "bus", seed: 1, apps: ["messages", "sim"], actors: ["test-1", "test-2", "test-3"] };
  const as = (id: string): Who => ({ id, kind: "human" });
  const acts: Act[] = [
    { verb: "messages.send", args: { to: "test-2", text: "hi" }, who: as("test-1"), at: 1000 },
    { verb: "messages.send", args: { to: "test-3", text: "me" }, who: as("test-3"), at: 2000 },
    { verb: "sim.snap", args: {}, who: as("test-2"), at: 3000 },
  ];
  const store = boot(await install(config.apps), ROOT, config);
  const one = gym();
  const ask = async (body: Ask) => JSON.parse(await one.answer(JSON.stringify(body)));
  await ask({ op: "boot", configs: [config] });
  for (const act of acts) {
    store.call(act);
    await ask({ op: "call", acts: [act] });
  }
  const [journal] = (await ask({ op: "lines" })) as Line[][];
  expect(journal).toEqual(parse(readFileSync(join(ROOT, "journal.jsonl"), "utf8")));
  expect([journal!.length, journal!.map((line) => line.verb)]).toEqual([
    5,
    ["messages.send", "sim.tick", "messages.send", "sim.tick", "sim.snap"],
  ]);
});
