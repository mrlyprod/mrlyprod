import { readFileSync } from "node:fs";
import { join } from "node:path";
import type { App, Config, Kind, Line, Obj } from "../../kernel/index.ts";
import { digest, Fault, kernel, parse } from "../../kernel/index.ts";
import type { Table } from "../../sim/bots.ts";
import type { Plan } from "../../sim/run.ts";
import { playersOf, run } from "../../sim/run.ts";
import type { Store } from "./store.ts";
import { boot, configOf, exists, fault, faults, open } from "./store.ts";

/* TYPES */

export type Tally = { id: string; kind: Kind; acts: number; refused: number };

export type Report = {
  world: string;
  seed: number;
  rules: Obj;
  tick: number;
  at: number;
  bots: Tally[];
  verbs: Record<string, number>;
  refusals: Record<string, number>;
  faults: number;
  faulty: Record<string, number>;
  digest: string;
  replayed: boolean;
};

export const WORST = 5;

/* FAULTS */

export const guarded = (store: Store): Table => ({
  world: store.world,
  list: store.list,
  call: (act) => {
    try {
      return store.call(act);
    } catch (thrown) {
      if (!(thrown instanceof Fault)) throw thrown;
      const { verb, args = {}, who = null } = act;
      fault(store.dir, { tick: store.world().tick, who, verb, args, fault: thrown.code, note: thrown.note });
      return null;
    }
  },
});

const worst = (dir: string) => {
  const counts: Record<string, number> = {};
  for (const one of faults(dir)) counts[one.note] = (counts[one.note] ?? 0) + 1;
  const top = Object.entries(counts)
    .sort((a, b) => b[1] - a[1] || a[0].localeCompare(b[0]))
    .slice(0, WORST);
  return { faults: faults(dir).length, faulty: Object.fromEntries(top) };
};

/* REPORT */

export function report(apps: readonly App[], dir: string): Report {
  const config: Config = configOf(dir);
  const lines: Line[] = parse(readFileSync(join(dir, "journal.jsonl"), "utf8"));
  const k = kernel(apps);
  const tallies = new Map<string, Tally>();
  const verbs: Record<string, number> = {};
  const refusals: Record<string, number> = {};
  let world = k.boot(config);
  for (const line of lines) {
    const done = k.call(world, { verb: line.verb, args: line.args, who: line.who, at: line.at });
    world = done.world;
    const one = tallies.get(line.who.id) ?? { id: line.who.id, kind: line.who.kind, acts: 0, refused: 0 };
    one.acts++;
    if (!done.envelope.last.ok) {
      one.refused++;
      refusals[line.verb] = (refusals[line.verb] ?? 0) + 1;
    }
    tallies.set(line.who.id, one);
    verbs[line.verb] = (verbs[line.verb] ?? 0) + 1;
  }
  const kept = open(apps, dir).world();
  return {
    world: config.name,
    seed: world.seed,
    rules: world.rules,
    tick: world.tick,
    at: world.at,
    bots: [...tallies.values()].sort((a, b) => a.id.localeCompare(b.id, "en", { numeric: true })),
    verbs: Object.fromEntries(Object.entries(verbs).sort()),
    refusals: Object.fromEntries(Object.entries(refusals).sort()),
    ...worst(dir),
    digest: digest(world),
    replayed: digest(kept) === digest(world),
  };
}

export function simulate(apps: readonly App[], dir: string, config: Config, plan: Plan): Report {
  const store = exists(dir) ? open(apps, dir) : boot(apps, dir, config);
  run(guarded(store), plan, playersOf(store.world(), plan.bots), apps);
  return report(apps, dir);
}
