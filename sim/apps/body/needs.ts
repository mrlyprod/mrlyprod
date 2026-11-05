import type { Knob, Knobs } from "@mrlyprod/os/contract/index.ts";
import { below, digest, stream } from "@mrlyprod/os/contract/index.ts";

export const KNOBS = {
  hunger: {
    line: "Hunger gained per hour.",
    schema: { type: "integer", minimum: 0, maximum: 1000 },
    reality: 5,
  },
  energy: {
    line: "Energy lost per hour.",
    schema: { type: "integer", minimum: 0, maximum: 1000 },
    reality: 4,
  },
  fun: {
    line: "Fun lost per hour.",
    schema: { type: "integer", minimum: 0, maximum: 1000 },
    reality: 3,
  },
  spread: {
    line: "How far people differ when the world starts, in points of each need.",
    schema: { type: "integer", minimum: 0, maximum: 100 },
    reality: 60,
  },
  baseline: {
    line: "Hunger no one rises above, because the world feeds everyone; 100 promises nothing.",
    schema: { type: "integer", minimum: 0, maximum: 100 },
    reality: 100,
  },
  rest: {
    line: "Energy one hour of sleep restores.",
    schema: { type: "integer", minimum: 1, maximum: 100 },
    reality: 12,
  },
  night: {
    line: "Most hours in one sleep.",
    schema: { type: "integer", minimum: 1, maximum: 72 },
    reality: 12,
  },
  asleep: {
    line: "Percent of the hunger rate that runs during sleep.",
    schema: { type: "integer", minimum: 0, maximum: 100 },
    reality: 50,
  },
  meal: {
    line: "A cooked meal's food, as a percent of its raw ingredients' food; what is already cooked counts once.",
    schema: { type: "integer", minimum: 100, maximum: 1000 },
    reality: 150,
  },
  relish: {
    line: "Fun a cooked meal carries on top of its ingredients'.",
    schema: { type: "integer", minimum: 0, maximum: 100 },
    reality: 5,
  },
  recipe: {
    line: "Most ingredients in one meal.",
    schema: { type: "integer", minimum: 2, maximum: 20 },
    reality: 6,
  },
  servings: {
    line: "Most portions one pot splits into.",
    schema: { type: "integer", minimum: 1, maximum: 100 },
    reality: 8,
  },
  famished: {
    line: "Hunger at which you are told, once per spell.",
    schema: { type: "integer", minimum: 1, maximum: 100 },
    reality: 70,
  },
  drained: {
    line: "Energy at which you are told, once per spell.",
    schema: { type: "integer", minimum: 0, maximum: 99 },
    reality: 20,
  },
  cooking: {
    line: "Energy cooking costs.",
    schema: { type: "integer", minimum: 0, maximum: 100 },
    reality: 3,
  },
  play: {
    line: "Fun going out gives, alone.",
    schema: { type: "integer", minimum: 0, maximum: 100 },
    reality: 15,
  },
  together: {
    line: "Extra fun for each person with you; someone you play with gains this too.",
    schema: { type: "integer", minimum: 0, maximum: 100 },
    reality: 8,
  },
  crowd: {
    line: "Most people who add to the fun.",
    schema: { type: "integer", minimum: 0, maximum: 256 },
    reality: 5,
  },
  feast: {
    line: "Extra fun each diner gets from a shared meal, on top of their share.",
    schema: { type: "integer", minimum: 0, maximum: 100 },
    reality: 10,
  },
  table: {
    line: "Most people one shared meal feeds, the host included.",
    schema: { type: "integer", minimum: 2, maximum: 256 },
    reality: 8,
  },
  effort: {
    line: "Energy going out costs.",
    schema: { type: "integer", minimum: 0, maximum: 100 },
    reality: 6,
  },
  empty: {
    line: "What happens when you go out starving or exhausted: nothing, a refusal, or you faint.",
    schema: { enum: ["nothing", "refuse", "faint"] },
    reality: "nothing",
  },
  revive: {
    line: "Energy you wake with after fainting.",
    schema: { type: "integer", minimum: 1, maximum: 100 },
    reality: 20,
  },
} satisfies Knobs;

export type Dials = { [K in keyof typeof KNOBS]: (typeof KNOBS)[K]["reality"] };

export type Need = { level: number; at: number };

export type Person = { hunger: Need; energy: Need; fun: Need; last: string | null; faints: number };

export type Body = {
  seed: number;
  clock: number | null;
  people: string[] | null;
  dials: Dials;
  persons: Record<string, Person>;
  warned: Record<string, string[]>;
};

export type Levels = { hunger: number; energy: number; fun: number };

export const HOUR = 3_600_000;

export const FULL = 100;

export const LOW = 40;

export const dialed = (knob: Knob) => Object.fromEntries(Object.keys(KNOBS).map((name) => [name, knob(name)])) as Dials;

function woken(state: Body, id: string, at: number): Person {
  const { draw } = stream(Number.parseInt(digest(`${state.seed}/${id}`).slice(0, 8), 16));
  const since = state.clock ?? at;
  const vary = () => below(draw, state.dials.spread + 1);
  return {
    hunger: { level: vary(), at: since },
    energy: { level: FULL - vary(), at: since },
    fun: { level: FULL - vary(), at: since },
    last: null,
    faints: 0,
  };
}

export const personOf = (state: Body, id: string, at: number) =>
  Object.hasOwn(state.persons, id) ? state.persons[id]! : woken(state, id, at);

const within = (low: number, n: number, high: number) => Math.min(high, Math.max(low, n));

const passed = (need: Need, rate: number, at: number) => (rate * Math.max(0, at - need.at)) / HOUR;

export function levelsOf(state: Body, id: string, at: number): Levels {
  const { hunger, energy, fun } = personOf(state, id, at);
  const { dials } = state;
  return {
    hunger: within(0, hunger.level + passed(hunger, dials.hunger, at), dials.baseline),
    energy: within(0, energy.level - passed(energy, dials.energy, at), FULL),
    fun: within(0, fun.level - passed(fun, dials.fun, at), FULL),
  };
}

export const badOf = ({ dials }: Body, { hunger, energy }: Levels) => [
  ...(hunger >= dials.famished ? ["hunger"] : []),
  ...(energy <= dials.drained ? ["energy"] : []),
];

export const warnedOf = (state: Body, id: string) => (Object.hasOwn(state.warned, id) ? state.warned[id]! : []);

export const watched = (state: Body) => state.people ?? Object.keys(state.persons);

export const spent = ({ hunger, energy }: Levels) => hunger >= FULL || energy <= 0;
