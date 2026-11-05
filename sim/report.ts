import { readFileSync } from "node:fs";
import { join } from "node:path";
import type { Store } from "@mrlyprod/os/hosts/local/store.ts";
import { boot, configOf, exists, faults, open } from "@mrlyprod/os/hosts/local/store.ts";
import type { App, Config, Obj, World } from "@mrlyprod/os/kernel/index.ts";
import { digest, Fault, kernel, parse } from "@mrlyprod/os/kernel/index.ts";
import { journalOf, REACHES, round } from "./measures.ts";
import type { Table } from "./round.ts";
import type { Plan } from "./run.ts";
import { playersOf, run } from "./run.ts";

export type Tally = { id: string; kind: string; acts: number; refused: number };

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

const DAY = 86_400_000;

const FIRST = 10;

const STEADY = 3;

const NEWLINE = 10;

export const guarded = (store: Store): Table => ({
  world: store.world,
  list: store.list,
  call: (act) => {
    try {
      return store.call(act);
    } catch (thrown) {
      if (!(thrown instanceof Fault)) throw thrown;
      return null;
    }
  },
});

const worst = (dir: string) => {
  const all = faults(dir);
  const counts: Record<string, number> = {};
  for (const one of all) counts[one.note] = (counts[one.note] ?? 0) + 1;
  const top = Object.entries(counts)
    .sort((a, b) => b[1] - a[1] || a[0].localeCompare(b[0]))
    .slice(0, WORST);
  return { faults: all.length, faulty: Object.fromEntries(top) };
};

export function scanner(apps: readonly App[], dir: string) {
  const config: Config = configOf(dir);
  const k = kernel(apps);
  const journal = join(dir, "journal.jsonl");
  const tallies = new Map<string, Tally>();
  const verbs: Record<string, number> = {};
  const refusals: Record<string, number> = {};
  let world = k.boot(config);
  let bytes = 0;
  const scan = () => {
    const text = readFileSync(journal);
    const end = text.lastIndexOf(NEWLINE) + 1;
    for (const line of parse(text.subarray(bytes, end).toString("utf8"))) {
      const done = k.call(world, line);
      world = done.world;
      const ok = done.envelope.last.ok;
      if (!ok) refusals[line.verb] = (refusals[line.verb] ?? 0) + 1;
      if (line.via) continue;
      const one = tallies.get(line.who.id) ?? { id: line.who.id, kind: line.who.kind, acts: 0, refused: 0 };
      tallies.set(line.who.id, { ...one, acts: one.acts + 1, refused: one.refused + (ok ? 0 : 1) });
      verbs[line.verb] = (verbs[line.verb] ?? 0) + 1;
    }
    bytes = Math.max(bytes, end);
    return { world, tallies, verbs, refusals };
  };
  return { config, scan };
}

export function report(apps: readonly App[], dir: string, live?: World): Report {
  const { config, scan } = scanner(apps, dir);
  const { world, tallies, verbs, refusals } = scan();
  const kept = live ?? open(apps, dir).world();
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

export type Spent = { wake: number; cost: number };

export const MINDS: { name: string; line: string }[] = [
  { name: "steady", line: `pairs in contact on ${STEADY} or more distinct days` },
  { name: "return", line: "share of pairs in contact on one day who are in contact again on a later day" },
  { name: "chores", line: `body acts among each bot's first ${FIRST} acts of a day, on average` },
  { name: "cost", line: "cost per wake, by day" },
];

const pairOf = (a: string, b: string) => [a, b].sort().join(" ");

function costs(spent: Spent[]) {
  const days = new Map<number, number[]>();
  for (const one of spent) days.set(one.wake, [...(days.get(one.wake) ?? []), one.cost]);
  return [...days.entries()]
    .sort((a, b) => a[0] - b[0])
    .map(([wake, all]) => `day ${wake} $${(all.reduce((a, b) => a + b, 0) / all.length).toFixed(2)}`)
    .join(", ");
}

export function minds(apps: readonly App[], dir: string, spent: Spent[] = []): Record<string, number | string | null> {
  const config = configOf(dir);
  const people = new Set(config.actors ?? []);
  const k = kernel(apps);
  let world = k.boot(config);
  const contact = new Map<string, Set<number>>();
  const firsts = new Map<string, string[]>();
  const dayOf = (at: number) => Math.floor((at - (config.start ?? 0)) / DAY);
  for (const line of journalOf(dir)) {
    const done = k.call(world, line);
    world = done.world;
    if (line.via) continue;
    const day = dayOf(line.at);
    const key = `${line.who.id} ${day}`;
    firsts.set(key, [...(firsts.get(key) ?? []), line.verb].slice(0, FIRST));
    if (!done.envelope.last.ok) continue;
    for (const to of REACHES.map((one) => line.args[one]))
      if (typeof to === "string" && to !== line.who.id && people.has(to) && people.has(line.who.id)) {
        const pair = pairOf(line.who.id, to);
        contact.set(pair, (contact.get(pair) ?? new Set()).add(day));
      }
  }
  const spans = [...contact.values()];
  const walks = [...firsts.values()];
  const chores = walks.reduce((sum, verbs) => sum + verbs.filter((one) => one.startsWith("body.")).length, 0);
  return {
    steady: spans.filter((one) => one.size >= STEADY).length,
    return: spans.length ? round(spans.filter((one) => one.size > 1).length / spans.length) : null,
    chores: walks.length ? round(chores / walks.length) : null,
    cost: spent.length ? costs(spent) : null,
  };
}

export function simulate(apps: readonly App[], dir: string, config: Config, plan: Plan): Report {
  const store = exists(dir) ? open(apps, dir) : boot(apps, dir, config);
  run(guarded(store), plan, playersOf(store.world(), plan.bots));
  return report(apps, dir, store.world());
}
