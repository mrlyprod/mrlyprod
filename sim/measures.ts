import { readFileSync } from "node:fs";
import { join } from "node:path";
import { configOf } from "@mrlyprod/os/hosts/local/store.ts";
import type { App, Config, Line, Obj, World } from "@mrlyprod/os/kernel/index.ts";
import { kernel, parse } from "@mrlyprod/os/kernel/index.ts";

export type Measures = Record<string, number | null>;

type Kernel = ReturnType<typeof kernel>;

type Seen = { line: Line; ok: boolean; rose: boolean };

type Wishing = { wishes?: Record<string, { status: string; by: string }> };

export const REACHES = ["to", "person", "with"];

const HOUR = 3_600_000;
const DAY = 24 * HOUR;
const MORNING = 7 * HOUR;
const HUNGRY = 70;
const FIRST = 10;
const REGULAR = 3;
const FOOD = ["market.buy", "body.eat", "body.cook"];
const SOCIAL = ["messages.send", "maps.say", "maps.invite", "people.friend", "market.give", "wallet.give"];
const MAKING = ["market.make", "canon.add", "news.post"];
const MAKERS = ["books.", "docs.", "wiki.", "code.", "sites."];

export const MEASURES: { name: string; line: string }[] = [
  { name: "acts", line: "acts taken (lines with no via)" },
  { name: "food", line: "share of acts: market.buy, body.eat, body.cook" },
  { name: "social", line: "share of acts that reach out or give to people" },
  { name: "making", line: "share of acts that make something" },
  { name: "contacts", line: "share of all pairs with a landed contact" },
  { name: "regulars", line: `share of all pairs in contact on ${REGULAR}+ distinct days` },
  { name: "followup", line: `share of a bot's first ${FIRST} acts per day aimed at a person touched the day before` },
  { name: "gini", line: "gini of balances at the end" },
  { name: "hunger", line: "mean hunger before each dawn and at the end" },
  { name: "hungry", line: `bot-days at hunger ${HUNGRY}+` },
  { name: "claims", line: "wallet.claim acts" },
  { name: "earning", line: "share of acts after which the actor held more coins" },
  { name: "wishes", line: "wishes posted" },
  { name: "decided", line: "share of wishes granted or declined" },
  { name: "granted", line: "bots with a wish granted" },
  { name: "votes", line: "vote.cast acts per bot" },
  { name: "borders", line: "share of maps.go acts refused" },
];

export const round = (n: number) => Math.round(n * 1000) / 1000;

const share = (part: number, whole: number) => (whole ? round(part / whole) : 0);

export function gini(values: number[]) {
  const sorted = [...values].sort((a, b) => a - b);
  const sum = sorted.reduce((a, b) => a + b, 0);
  if (!sum) return 0;
  const weighted = sorted.reduce((total, value, i) => total + (i + 1) * value, 0);
  return (2 * weighted) / (sorted.length * sum) - (sorted.length + 1) / sorted.length;
}

const aimed = (line: Line, known: Set<string>): string[] =>
  REACHES.map((key) => line.args[key]).filter(
    (to): to is string => typeof to === "string" && to !== line.who.id && known.has(to) && known.has(line.who.id),
  );

const pairOf = (a: string, b: string) => [a, b].sort().join(" ");

export function contacts(lines: Line[], people: string[]) {
  const known = new Set(people);
  const pairs = new Set<string>();
  for (const line of lines) for (const to of aimed(line, known)) pairs.add(pairOf(line.who.id, to));
  return pairs.size / Math.max(1, (people.length * (people.length - 1)) / 2);
}

export function wellbeing(k: Kernel, world: World, people: string[]) {
  if (!("body" in world.apps)) return 0;
  const needs = people.map((id) => k.read(world, `body/@${id}`).state as Obj | null);
  const scores = needs.map((one) => {
    const hunger = Number(one?.hunger ?? 100);
    const energy = Number(one?.energy ?? 0);
    const fun = Number(one?.fun ?? 0);
    return (100 - hunger + energy + fun) / 300;
  });
  return scores.reduce((a, b) => a + b, 0) / Math.max(1, scores.length);
}

const held = (world: World, id: string) =>
  Number((world.apps.wallet as { balances?: Record<string, number> } | undefined)?.balances?.[id] ?? 0);

const dayOf = (line: Line) => Math.floor((line.at - MORNING) / DAY);

const verbIn = (verb: string, list: string[], prefixes: string[] = []) =>
  list.includes(verb) || prefixes.some((one) => verb.startsWith(one));

function replayed(k: Kernel, config: Config, lines: Line[], people: string[]) {
  let world = k.boot(config);
  const seen: Seen[] = [];
  const hungers: number[] = [];
  const sample = () => {
    if (!("body" in world.apps)) return;
    for (const id of people) {
      const one = k.read(world, `body/@${id}`).state as Obj | null;
      if (one && one.hunger !== undefined) hungers.push(Number(one.hunger));
    }
  };
  for (const line of lines) {
    if (line.verb === "dawn.rise") sample();
    const before = held(world, line.who.id);
    const done = k.call(world, line);
    world = done.world;
    seen.push({ line, ok: done.envelope.last.ok, rose: held(world, line.who.id) > before });
  }
  sample();
  return { world, seen, hungers };
}

function regulars(landed: Line[], people: string[]) {
  const known = new Set(people);
  const days = new Map<string, Set<number>>();
  for (const line of landed)
    for (const to of aimed(line, known)) {
      const pair = pairOf(line.who.id, to);
      days.set(pair, (days.get(pair) ?? new Set()).add(dayOf(line)));
    }
  const many = [...days.values()].filter((one) => one.size >= REGULAR).length;
  return many / Math.max(1, (people.length * (people.length - 1)) / 2);
}

function followup(acts: Line[], landed: Line[], people: string[]): number | null {
  const known = new Set(people);
  const touched = new Map<number, Set<string>>();
  for (const line of landed)
    for (const to of aimed(line, known)) {
      const day = dayOf(line);
      touched.set(day, (touched.get(day) ?? new Set()).add(pairOf(line.who.id, to)));
    }
  const first = Math.min(...acts.map(dayOf));
  const taken = new Map<string, Line[]>();
  for (const line of acts) {
    const day = dayOf(line);
    if (day === first) continue;
    const key = `${line.who.id} ${day}`;
    taken.set(key, [...(taken.get(key) ?? []), line]);
  }
  if (!taken.size) return null;
  let hits = 0;
  let total = 0;
  for (const lines of taken.values())
    for (const line of lines.slice(0, FIRST)) {
      total++;
      const before = touched.get(dayOf(line) - 1);
      if (aimed(line, known).some((to) => before?.has(pairOf(line.who.id, to)))) hits++;
    }
  return share(hits, total);
}

export function measures(apps: readonly App[], config: Config, lines: Line[]): Measures {
  const people = config.actors ?? [];
  const k = kernel(apps);
  const { world, seen, hungers } = replayed(k, config, lines, people);
  const known = new Set(people);
  const acts = seen.filter((one) => !one.line.via);
  const landed = acts.filter((one) => one.ok).map((one) => one.line);
  const count = (pick: (line: Line) => boolean) => acts.filter((one) => pick(one.line)).length;
  const wishing = world.apps.wish as Wishing | undefined;
  const wishes = Object.values(wishing?.wishes ?? {});
  const going = acts.filter((one) => one.line.verb === "maps.go");
  const balances = people.map((id) => held(world, id));
  const hungry = hungers.filter((one) => one >= HUNGRY).length;
  return {
    acts: acts.length,
    food: share(
      count((line) => FOOD.includes(line.verb)),
      acts.length,
    ),
    social: share(
      count(
        (line) =>
          verbIn(line.verb, SOCIAL) ||
          (line.verb === "body.play" && typeof line.args.with === "string" && known.has(line.args.with)),
      ),
      acts.length,
    ),
    making: share(
      count((line) => verbIn(line.verb, MAKING, MAKERS)),
      acts.length,
    ),
    contacts: round(contacts(landed, people)),
    regulars: round(regulars(landed, people)),
    followup: acts.length
      ? followup(
          acts.map((one) => one.line),
          landed,
          people,
        )
      : null,
    gini: round(gini(balances)),
    hunger: hungers.length ? round(hungers.reduce((a, b) => a + b, 0) / hungers.length) : null,
    hungry,
    claims: count((line) => line.verb === "wallet.claim"),
    earning: share(acts.filter((one) => one.rose).length, acts.length),
    wishes: wishing ? wishes.length : null,
    decided: wishing
      ? share(wishes.filter((one) => one.status === "granted" || one.status === "declined").length, wishes.length)
      : null,
    granted: wishing ? new Set(wishes.filter((one) => one.status === "granted").map((one) => one.by)).size : null,
    votes: round(count((line) => line.verb === "vote.cast") / Math.max(1, people.length)),
    borders: share(going.filter((one) => !one.ok).length, going.length),
  };
}

export const journalOf = (dir: string): Line[] => parse(readFileSync(join(dir, "journal.jsonl"), "utf8"));

export const measuresOf = (apps: readonly App[], dir: string): Measures =>
  measures(apps, configOf(dir), journalOf(dir));
