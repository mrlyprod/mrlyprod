import { afterAll, expect, test } from "bun:test";
import { readFileSync, rmSync } from "node:fs";
import { join } from "node:path";
import { HOME, install } from "../hosts/local/apps.ts";
import { simulate } from "../hosts/local/sim.ts";
import { DATA_DIR } from "../hosts/local/store.ts";
import type { Config } from "../kernel/index.ts";
import { digest } from "../kernel/index.ts";
import { round } from "../sim/bots.ts";
import { BEAT } from "../sim/run.ts";
import type { Options } from "./world.ts";
import { create, paced, WATCH } from "./world.ts";

const CONFIG = JSON.parse(readFileSync(join(HOME, "world.json"), "utf8")) as Config;

const ROOT = join(DATA_DIR, "test-world", String(process.pid));

const ROUNDS = 100;

const tab = (options: Options) =>
  create({ ...options, players: (table, who, at) => round(table, who, at, options.apps) });

afterAll(() => rmSync(join(DATA_DIR, "test-world"), { recursive: true, force: true }));

test("100 rounds of the test users in the tab's store reach the digest mrlynet sim reaches on world.json", async () => {
  const apps = await install(CONFIG.apps.map((one) => (typeof one === "string" ? one : one.id)));
  const config = { ...CONFIG, seed: 7 };
  const headless = simulate(apps, ROOT, config, { bots: 0, rounds: ROUNDS, beat: BEAT });
  const store = tab({ apps, config });
  for (let i = 0; i < ROUNDS; i++) store.round((i + 1) * BEAT);
  expect([store.world().tick, digest(store.world())]).toEqual([headless.tick, headless.digest]);
});

test("the guest watches: its act comes back as the sign-in note; signed in, the bots leave that user to the visitor", async () => {
  const apps = await install(["people", "messages"]);
  const store = tab({ apps, config: { ...CONFIG, apps: ["people", "messages"], seed: 3 } });
  const at = store.world().at + BEAT;
  const act = { verb: "messages.send", args: { to: "test-2", text: "hi" }, at };
  const watched = store.call(act);
  store.sign("test-3");
  const signed = store.call(act);
  store.round(at + BEAT);
  const movers = new Set(
    store
      .lines()
      .slice(1)
      .map((line) => line.who.id),
  );
  expect([watched.note, signed.ok, movers.has("test-3"), movers.size > 0]).toEqual([WATCH, true, false, true]);
});

test("a hidden tab makes no act", async () => {
  const apps = await install(["people", "messages"]);
  const store = tab({ apps, config: { ...CONFIG, apps: ["people", "messages"], seed: 3 } });
  const beat = paced(store, () => true);
  beat();
  expect(store.world().tick).toBe(0);
});

test("a tab its host gave no players holds still on a round: the shell plays nobody by itself", async () => {
  const apps = await install(["people", "messages"]);
  const store = create({ apps, config: { ...CONFIG, apps: ["people", "messages"], seed: 3 } });
  expect([store.round(BEAT), store.world().tick]).toEqual([0, 0]);
});
