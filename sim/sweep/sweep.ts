#!/usr/bin/env bun
import { appendFileSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join, relative } from "node:path";
import { bus } from "@mrlyprod/os/hosts/bus.ts";
import { install } from "@mrlyprod/os/hosts/local/apps.ts";
import type { Act, App, Called, Config, Json, Line, Obj } from "@mrlyprod/os/kernel/index.ts";
import { digest, Fault, kernel, replay } from "@mrlyprod/os/kernel/index.ts";
import { contacts, gini, wellbeing } from "../measures.ts";
import { run } from "../run.ts";
import { generate, knobsOf, turned } from "../worlds.ts";

const HERE = import.meta.dir;
const SIM = join(HERE, "..");
const REPO = join(SIM, "..");
const DATA_DIR = join(REPO, "data", relative(REPO, HERE));
const THOUSAND = 1000;
const QUIET = ["notifications.notice"];
const DEFAULTS = { worlds: "100", gens: "1", rounds: "60", most: "32", turn: "0.5", jobs: "8", name: "", from: "0" };
const FIT = 0.2;
const MUTATE = 0.05;

type Result = {
  seed: number;
  rules: Obj;
  people: number;
  ticks: number;
  faults: number;
  refused: number;
  dropped: number;
  crash: string | null;
  replayed: boolean;
  connection: number;
  equality: number;
  wellbeing: number;
  agency: number;
  score: number;
};

const ask: Record<string, string> = { ...DEFAULTS };
let part: string | null = null;
for (const arg of process.argv.slice(2)) {
  const at = arg.indexOf("=");
  if (arg.startsWith("--part=")) part = arg.slice(7);
  else if (at > 0 && arg.slice(0, at) in ask) ask[arg.slice(0, at)] = arg.slice(at + 1);
  else throw new Error(`unknown setting ${arg}; settings: ${Object.keys(DEFAULTS)}`);
}

const thousandths = (n: number) => Math.round(Math.max(0, Math.min(1, n)) * THOUSAND);

function measure(apps: readonly App[], k: ReturnType<typeof kernel>, config: Config, rounds: number): Result {
  const blank = (crash: string | null): Result => ({
    seed: config.seed ?? 0,
    rules: config.rules ?? {},
    people: config.actors?.length ?? 0,
    ticks: 0,
    faults: 0,
    refused: 0,
    dropped: 0,
    crash,
    replayed: false,
    connection: 0,
    equality: 0,
    wellbeing: 0,
    agency: 0,
    score: 0,
  });
  try {
    let world = k.boot(config);
    const lines: Line[] = [];
    const landed: Line[] = [];
    let faults = 0;
    let refused = 0;
    let dropped = 0;
    const step = (act: Act): Called => {
      const done = k.call(world, act);
      world = done.world;
      lines.push(done.line);
      if (done.envelope.last.ok) landed.push(done.line);
      else if (!QUIET.includes(done.line.verb)) refused++;
      return done;
    };
    const fault = (thrown: unknown) => {
      if (!(thrown instanceof Fault)) throw thrown;
      faults++;
    };
    const drive = bus({ apps, world: () => world, step, heard: fault });
    const table = {
      world: () => world,
      list: () => k.list(world),
      call: (act: Act) => {
        try {
          return drive(act);
        } catch (thrown) {
          fault(thrown);
          return null;
        }
      },
      dropped: () => {
        dropped++;
      },
    };
    const people = config.actors ?? [];
    const who = people.map((id) => ({ id, kind: "bot" as const }));
    const beat = config.pace ?? THOUSAND;
    run(table, { bots: who.length, rounds, beat }, who);
    const acts = lines.filter((line) => !QUIET.includes(line.verb)).length;
    const failed = refused + faults + dropped;
    const balances = people.map((id) => Number((k.read(world, `wallet/@${id}`).state as Obj | null)?.balance ?? 0));
    const connection = thousandths(contacts(landed, people));
    const equality = thousandths(1 - gini(balances));
    const well = thousandths(wellbeing(k, world, people));
    const agency = thousandths(1 - failed / Math.max(1, acts + faults + dropped));
    return {
      ...blank(null),
      ticks: world.tick,
      faults,
      refused,
      dropped,
      replayed: digest(replay(k, config, lines)) === digest(world),
      connection,
      equality,
      wellbeing: well,
      agency,
      score: Math.round((connection + equality + well + agency) / 4),
    };
  } catch (thrown) {
    return blank(String((thrown as Error)?.stack ?? thrown).slice(0, 600));
  }
}

function breed(seed: number, apps: readonly App[], parents: Result[], turn: number, most: number): Config {
  if (!parents.length) {
    const made = generate(seed, apps, { turn });
    return (made.actors?.length ?? 0) > most ? generate(seed, apps, { turn, people: most }) : made;
  }
  const a = parents[seed % parents.length]!;
  const b = parents[(seed * 7 + 3) % parents.length]!;
  const keys = [...new Set([...Object.keys(a.rules), ...Object.keys(b.rules)])];
  const sideOf = (key: string) => (seed + keys.indexOf(key === "canon.not" ? "canon.keep" : key) * 31) % 2;
  const rules: Obj = {};
  for (const key of keys) {
    const from = sideOf(key) ? a : b;
    if (from.rules[key] !== undefined) rules[key] = from.rules[key] as Json;
  }
  const { "world.people": people, ...mutant } = turned(seed, apps, MUTATE);
  if ("canon.keep" in mutant) delete rules["canon.not"];
  return generate(seed, apps, {
    turn: 0,
    people: Math.min(most, Number(people ?? a.people)),
    rules: { ...rules, ...mutant },
  });
}

async function worker(spec: string) {
  const [from, to, file, parentsFile] = spec.split("|") as [string, string, string, string];
  const apps = await install(undefined, SIM);
  const k = kernel(apps);
  const parents: Result[] = parentsFile ? JSON.parse(readFileSync(parentsFile, "utf8")) : [];
  for (let seed = Number(from); seed < Number(to); seed++) {
    const config = breed(seed, apps, parents, Number(ask.turn), Number(ask.most));
    appendFileSync(file, `${JSON.stringify(measure(apps, k, config, Number(ask.rounds)))}\n`);
  }
}

const results = (file: string): Result[] =>
  readFileSync(file, "utf8")
    .split("\n")
    .filter(Boolean)
    .map((line) => JSON.parse(line) as Result);

async function generation(dir: string, gen: number, from: number, parents: Result[]) {
  const count = Number(ask.worlds);
  const jobs = Math.max(1, Number(ask.jobs));
  const file = join(dir, `gen-${gen}.jsonl`);
  writeFileSync(file, "");
  const parentsFile = parents.length ? join(dir, `parents-${gen}.json`) : "";
  if (parentsFile) writeFileSync(parentsFile, JSON.stringify(parents));
  const size = Math.ceil(count / jobs);
  const children = Array.from({ length: jobs }, (_, i) => {
    const start = from + i * size;
    const end = Math.min(from + count, start + size);
    const settings = ["rounds", "most", "turn"].map((key) => `${key}=${ask[key]}`);
    return Bun.spawn(
      [process.execPath, import.meta.path, ...settings, `--part=${start}|${end}|${file}|${parentsFile}`],
      {
        stdout: "inherit",
        stderr: "inherit",
      },
    );
  });
  const codes = await Promise.all(children.map((one) => one.exited));
  const dead = codes.filter(Boolean).length;
  if (dead) throw new Error(`${dead} of ${codes.length} workers died in gen ${gen}: this sample is short`);
  return results(file);
}

const SCORES = ["connection", "equality", "wellbeing", "agency", "score"] as const;

const sum = (list: Result[], key: "faults" | "refused" | "dropped") => list.reduce((all, one) => all + one[key], 0);

const mean = (list: number[]) => list.reduce((a, b) => a + b, 0) / Math.max(1, list.length);

function influence(all: Result[], apps: readonly App[]) {
  const out: { knob: string; spread: number; best: string }[] = [];
  for (const knob of knobsOf(apps)) {
    const groups = new Map<string, number[]>();
    for (const one of all) {
      const value = knob.name === "world.people" ? one.people : (one.rules[knob.name] ?? knob.reality);
      const raw = typeof value === "number" ? String(2 ** Math.round(Math.log2(Math.max(1, value)))) : String(value);
      groups.set(raw, [...(groups.get(raw) ?? []), one.score]);
    }
    const big = [...groups.entries()].filter(([, list]) => list.length >= Math.max(3, all.length / 50));
    if (big.length < 2) continue;
    const ranked = big.map(([key, list]) => ({ key, score: mean(list) })).sort((a, b) => b.score - a.score);
    out.push({ knob: knob.name, spread: ranked[0]!.score - ranked.at(-1)!.score, best: ranked[0]!.key });
  }
  return out.sort((a, b) => b.spread - a.spread).slice(0, 25);
}

function writeUp(name: string, all: Result[], gens: number[][], apps: readonly App[]) {
  const live = all.filter((one) => !one.crash);
  const crashes = new Map<string, Result>();
  for (const one of all) if (one.crash) crashes.set(one.crash.split("\n")[0]!, one);
  const top = [...live].sort((a, b) => b.score - a.score).slice(0, 10);
  const line = (one: Result) =>
    `- ${one.seed}: score ${one.score}, connection ${one.connection}, equality ${one.equality}, wellbeing ${one.wellbeing}, agency ${one.agency}, people ${one.people}`;
  return [
    `# ${name}`,
    "",
    `- ${all.length} worlds, ${all.length - live.length} crashed, ${live.filter((one) => !one.replayed).length} failed replay`,
    "- Scores are thousandths: facts about scripted bots, not about people.",
    "",
    "## SCORES",
    "",
    ...SCORES.map((key) => {
      const list = live.map((one) => one[key]);
      return `- ${key}: lowest ${list.length ? Math.min(...list) : 0}, mean ${Math.round(mean(list))}, highest ${list.length ? Math.max(...list) : 0}`;
    }),
    `- faults ${sum(live, "faults")}, refused ${sum(live, "refused")}, dropped ${sum(live, "dropped")}`,
    "",
    "## GENERATIONS",
    "",
    ...gens.map(
      (one, i) =>
        `- gen ${i}: mean ${Math.round(mean(one))}, best ${one.length ? Math.max(...one) : 0}, ${one.length} worlds`,
    ),
    "",
    "## CRASHES",
    "",
    ...[...crashes.entries()].map(([head, one]) => `- seed ${one.seed}: ${head}`),
    "",
    "## TOP",
    "",
    ...top.map(line),
    "",
    "## KNOBS THAT MATTER",
    "",
    ...influence(live, apps).map((one) => `- ${one.knob}: spread ${Math.round(one.spread)}, best ${one.best}`),
    "",
  ].join("\n");
}

if (part) await worker(part);
else {
  const name = ask.name || new Date().toISOString().replace(/[-:T]/g, "").slice(0, 14);
  const dir = join(DATA_DIR, name);
  mkdirSync(dir, { recursive: true });
  const apps = await install(undefined, SIM);
  const all: Result[] = [];
  const scores: number[][] = [];
  let parents: Result[] = [];
  for (let gen = 0; gen < Number(ask.gens); gen++) {
    const from = Number(ask.from) + gen * Number(ask.worlds);
    const done = await generation(dir, gen, from, parents);
    all.push(...done);
    const live = done.filter((one) => !one.crash).sort((a, b) => b.score - a.score);
    scores.push(live.map((one) => one.score));
    parents = live.slice(0, Math.max(2, Math.ceil(live.length * FIT)));
    process.stdout.write(
      `gen ${gen}: ${done.length} worlds, ${done.length - live.length} crashed, mean ${Math.round(mean(scores[gen]!))}\n`,
    );
  }
  writeFileSync(join(dir, "best.json"), JSON.stringify(parents.slice(0, 10), null, 2));
  writeFileSync(join(dir, "summary.md"), writeUp(name, all, scores, apps));
  process.stdout.write(`summary: ${join(dir, "summary.md")}\n`);
}
