#!/usr/bin/env bun
import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { install } from "@mrlyprod/os/hosts/local/apps.ts";
import { configOf } from "@mrlyprod/os/hosts/local/store.ts";
import type { Config, Json } from "@mrlyprod/os/kernel/index.ts";
import { canon } from "@mrlyprod/os/kernel/index.ts";
import type { Made } from "./made.ts";
import type { Measures } from "./measures.ts";
import { MEASURES, measuresOf } from "./measures.ts";

const HERE = import.meta.dir;
const TESTERS = join(HERE, "..", "data", "sim", "testers");
const LEDGER = join(TESTERS, "ledger.tsv");
const IMPLIED: Record<string, string> = { "canon.not": "canon.keep" };
const HELP =
  "usage: bun compare.ts [--keys=rule,rule] <arm>=<run>,<run> <arm>=<run>,<run> ...; the first arm is the control";

type Arm = { name: string; runs: string[] };

type Run = { name: string; config: Config; personas: string; measures: Measures; cost: number; made: Made | null };

function parse(argv: string[]) {
  const arms: Arm[] = [];
  let keys: string[] | null = null;
  for (const arg of argv) {
    if (arg.startsWith("--keys=")) keys = arg.slice(7).split(",").filter(Boolean);
    else if (arg.includes("=")) {
      const at = arg.indexOf("=");
      const runs = arg
        .slice(at + 1)
        .split(",")
        .filter(Boolean);
      if (!runs.length) throw new Error(`arm ${arg.slice(0, at)} has no runs`);
      arms.push({ name: arg.slice(0, at), runs });
    } else throw new Error(`${arg} is not <arm>=<run>,<run>\n${HELP}`);
  }
  if (arms.length < 2) throw new Error(`give a control arm and at least one other arm\n${HELP}`);
  const all = arms.flatMap((one) => one.runs);
  const twice = all.find((one, i) => all.indexOf(one) !== i);
  if (twice) throw new Error(`run ${twice} is listed twice`);
  return { arms, keys };
}

function ledger(name: string) {
  const rows = existsSync(LEDGER)
    ? readFileSync(LEDGER, "utf8")
        .split("\n")
        .filter(Boolean)
        .slice(1)
        .map((line) => line.split("\t"))
        .filter((row) => row[0] === name)
    : [];
  const personas = new Map<string, string>();
  for (const row of rows) {
    const bot = row[2]!;
    const persona = row[3]!;
    if (personas.get(bot) && personas.get(bot) !== persona) throw new Error(`${name}: ${bot} has two personas`);
    personas.set(bot, persona);
  }
  return {
    cost: rows.reduce((sum, row) => sum + Number(row[6]), 0),
    personas: JSON.stringify([...personas].sort((a, b) => a[0].localeCompare(b[0], "en", { numeric: true }))),
  };
}

async function load(name: string): Promise<Run> {
  const dir = join(TESTERS, name, "world");
  if (!existsSync(join(dir, "config.json"))) throw new Error(`no world for run ${name} at ${dir}`);
  const config = configOf(dir);
  const ids = config.apps.map((one) => (typeof one === "string" ? one : one.id));
  const apps = await install(ids, HERE);
  const made = join(TESTERS, name, "made.json");
  return {
    name,
    config,
    ...ledger(name),
    measures: measuresOf(apps, dir),
    made: existsSync(made) ? (JSON.parse(readFileSync(made, "utf8")) as Made) : null,
  };
}

const same = (runs: Run[], pick: (run: Run) => unknown) =>
  new Set(runs.map((one) => canon(pick(one) as Json))).size === 1;

function must(runs: Run[]) {
  for (const [label, pick] of [
    ["seed", (one: Run) => one.config.seed],
    ["actors", (one: Run) => one.config.actors],
    ["personas", (one: Run) => one.personas],
  ] as const)
    if (!same(runs, pick))
      throw new Error(
        `runs differ in ${label}: ${runs.map((one) => `${one.name}=${canon(pick(one) as Json)}`).join(" ")}`,
      );
}

const ruleOf = (run: Run, key: string) => run.config.rules?.[key];

function differing(runs: Run[]) {
  const keys = [...new Set(runs.flatMap((one) => Object.keys(one.config.rules ?? {})))].sort();
  return keys.filter((key) => !same(runs, (one) => ruleOf(one, key) ?? null));
}

function rules(arms: { arm: Arm; runs: Run[] }[], allowed: string[] | null) {
  for (const { arm, runs } of arms) {
    const inside = differing(runs);
    if (inside.length) throw new Error(`arm ${arm.name} has runs with different rules: ${inside}`);
  }
  const all = arms.flatMap((one) => one.runs);
  const apart = differing(all);
  const bounds = allowed
    ? new Set([...allowed, ...Object.keys(IMPLIED).filter((key) => allowed.includes(IMPLIED[key]!))])
    : null;
  const extra = bounds ? apart.filter((key) => !bounds.has(key)) : apart.length > 1 ? apart : [];
  if (extra.length)
    throw new Error(
      `rules differ beyond the intended: ${extra}${allowed ? "" : "; name the intended ones with --keys=rule,rule"}`,
    );
  process.stdout.write(`rules that differ: ${apart.length ? "" : "none"}\n`);
  for (const key of apart)
    process.stdout.write(
      `  ${key}: ${arms.map((one) => `${one.arm.name}=${canon((ruleOf(one.runs[0]!, key) ?? null) as Json)}`).join("  ")}\n`,
    );
}

function code(runs: Run[]) {
  const stamp = (one: Run) =>
    one.made ? canon({ os: one.made.os, apps: one.made.apps, sim: one.made.sim } as unknown as Json) : "unknown";
  if (new Set(runs.map(stamp)).size > 1)
    process.stdout.write("note: runs were made on different code (see made.json in each run)\n");
}

const show = (n: number | null | undefined) => (n === null || n === undefined ? "-" : String(Number(n.toFixed(3))));

const mean = (list: (number | null)[]) => {
  const have = list.filter((one): one is number => one !== null);
  return have.length ? have.reduce((a, b) => a + b, 0) / have.length : null;
};

const span = (list: (number | null)[]) => {
  const have = list.filter((one): one is number => one !== null);
  return have.length ? `${show(Math.min(...have))}..${show(Math.max(...have))}` : "-";
};

function side(control: (number | null)[], own: (number | null)[]): "+" | "-" | "" {
  const base = control.filter((one): one is number => one !== null);
  if (!base.length || own.some((one) => one === null)) return "";
  const low = Math.min(...base);
  const high = Math.max(...base);
  if (own.every((one) => one! > high)) return "+";
  if (own.every((one) => one! < low)) return "-";
  return "";
}

function table(arms: { arm: Arm; runs: Run[] }[]) {
  const rows: { name: string; of: (run: Run) => number | null; flag: boolean }[] = [
    ...MEASURES.map((one) => ({ name: one.name, of: (run: Run) => run.measures[one.name] ?? null, flag: true })),
    { name: "cost", of: (run: Run) => run.cost, flag: false },
  ];
  const heads = [
    "",
    ...arms.flatMap((one) => one.runs.map((run) => run.name)),
    ...arms.flatMap((one) => [`${one.arm.name} mean`, `${one.arm.name} range`]),
  ];
  const lines = rows.map((row) => {
    const control = arms[0]!.runs.map(row.of);
    const cells = arms.flatMap((one) => one.runs.map((run) => show(row.of(run))));
    const stats = arms.flatMap((one, i) => {
      const own = one.runs.map(row.of);
      const mark = row.flag && i > 0 ? side(control, own) : "";
      return [`${show(mean(own))}${mark ? ` ${mark}` : ""}`, span(own)];
    });
    return [row.name, ...cells, ...stats];
  });
  const widths = heads.map((head, i) => Math.max(head.length, ...lines.map((line) => line[i]!.length)));
  const print = (line: string[]) =>
    `${line.map((cell, i) => (i === 0 ? cell.padEnd(widths[i]!) : cell.padStart(widths[i]!))).join("  ")}\n`;
  process.stdout.write(print(heads));
  for (const line of lines) process.stdout.write(print(line));
  process.stdout.write(`\n+ / - : every run of the arm is above / below the control (${arms[0]!.arm.name}) range\n`);
}

async function main(argv: string[]) {
  const { arms, keys } = parse(argv);
  const loaded = await Promise.all(arms.map(async (arm) => ({ arm, runs: await Promise.all(arm.runs.map(load)) })));
  const all = loaded.flatMap((one) => one.runs);
  must(all);
  rules(loaded, keys);
  code(all);
  process.stdout.write(
    `${all.length} runs, seed ${all[0]!.config.seed}, ${all[0]!.config.actors?.length ?? 0} people\n\n`,
  );
  table(loaded);
}

await main(process.argv.slice(2)).catch((thrown: unknown) => {
  process.stderr.write(`${String((thrown as Error)?.message ?? thrown)}\n`);
  process.exit(1);
});
