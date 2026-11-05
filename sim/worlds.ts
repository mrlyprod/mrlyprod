import type { Stocked } from "@mrlyprod/apps/market/goods.ts";
import type { Named, Setting } from "@mrlyprod/os/contract/index.ts";
import { knobsOf as engineKnobs } from "@mrlyprod/os/contract/index.ts";
import type { App, Config, Json, Obj, Schema } from "@mrlyprod/os/kernel/index.ts";
import { below, SPEC, seedOf, stream } from "@mrlyprod/os/kernel/index.ts";
import type { Thing } from "./canon.ts";
import { pruned, things, UNIVERSE } from "./canon.ts";

export type Shape = { people?: number; turn?: number; rules?: Obj; pace?: number; start?: number };

export const PEOPLE: Setting = {
  line: "How many people live in this world.",
  schema: { type: "integer", minimum: 2, maximum: 256 },
  reality: 8,
};

const GOODS: Setting = {
  line: "Goods the market stocks at boot, half of them food, drawn from what exists in this world.",
  schema: { type: "integer", minimum: 0, maximum: 400 },
  reality: 40,
};

const START = Date.UTC(2030, 0, 1, 7);

const PACE = 60_000;

const RINGS = 3;

const RING = 3_600_000;

const FRONT = ["notifications", "mind", "body"];

const PLAIN = ["notifications", "body", "notes"];

const TURN = 0.5;

const RANGE = 1_000_000;

const WHOLE = 100;

const FOOD = 5;

const FUN = 2;

const BANK = { causes: { mrlynet: "MrlyNet itself" }, reserves: { exchange: 100_000_000 } };

const GROUPS: Obj = {
  food: [...new Set(UNIVERSE.filter((one) => one.food !== undefined).map((one) => one.kind))],
  place: ["city", "country"],
};

const SHELVES: Record<string, string> = {
  household: "home",
  tool: "home",
  clothing: "clothes",
  electronics: "fun",
  vehicle: "fun",
  instrument: "fun",
  game: "fun",
};

export const knobsOf = (apps: readonly App[]): Named[] =>
  [{ ...PEOPLE, name: "world.people" }, { ...GOODS, name: "world.goods" }, ...engineKnobs(apps)].sort((a, b) =>
    a.name.localeCompare(b.name),
  );

function drawn(schema: Schema, draw: () => number, reality: Json): Json {
  const raw = schema as { enum?: Json[]; type?: string; minimum?: number; maximum?: number };
  if (raw.enum?.length) return raw.enum[below(draw, raw.enum.length)]!;
  if (raw.type === "boolean") return below(draw, 2) === 1;
  if (raw.type === "integer" && raw.minimum !== undefined && raw.maximum !== undefined) {
    const low = Math.max(raw.minimum, 1);
    const roll = below(draw, RANGE) / RANGE;
    const value = Math.round(low * (raw.maximum / low) ** roll);
    return raw.minimum === 0 && below(draw, 8) === 0 ? 0 : Math.min(raw.maximum, value);
  }
  return reality;
}

export function turned(seed: number, apps: readonly App[], turn: number): Obj {
  const { draw } = stream(seedOf(seed, "world"));
  const chance = Math.round(turn * RANGE);
  const rules: Obj = {};
  for (const knob of knobsOf(apps))
    if (below(draw, RANGE) < chance) rules[knob.name] = drawn(knob.schema, draw, knob.reality);
  const keep = rules["canon.keep"];
  if (typeof keep === "number" && keep < WHOLE) rules["canon.not"] = pruned(draw, keep);
  return rules;
}

function some<T>(list: readonly T[], n: number, draw: () => number): T[] {
  const out = [...list];
  for (let i = out.length - 1; i > 0; i--) {
    const j = below(draw, i + 1);
    [out[i], out[j]] = [out[j]!, out[i]!];
  }
  return out.slice(0, n);
}

const stocked = (one: Thing): Stocked => {
  const food = (one.food ?? 0) * FOOD;
  return {
    name: one.name,
    shelf: one.food ? "groceries" : (SHELVES[one.kind] ?? "home"),
    price: one.price ?? 1,
    traits: { food, fun: (one.fun ?? 0) * FUN, lasts: !one.food },
  };
};

export function catalog(seed: number, rules: Obj): Stocked[] {
  const { draw } = stream(seedOf(seed, "goods"));
  const count = Number(rules["world.goods"] ?? GOODS.reality);
  const priced = things(rules).filter((one) => one.price !== undefined && !one.added);
  const foods = priced.filter((one) => one.food);
  const others = priced.filter((one) => one.food === undefined);
  const half = Math.ceil(count / 2);
  return [...some(foods, half, draw), ...some(others, count - half, draw)].map(stocked);
}

const entry = (one: App, seed: number, rules: Obj) => {
  if (one.id === "wallet") return { id: one.id, config: BANK };
  if (one.id === "market") return { id: one.id, config: { catalog: catalog(seed, rules) } };
  return one.id;
};

export function generate(seed: number, apps: readonly App[], shape: Shape = {}): Config {
  const made = turned(seed, apps, shape.turn ?? TURN);
  const { "world.people": people, "world.goods": _goods, ...kept } = made;
  const ids = apps.map((one) => one.id);
  const count = shape.people ?? Number(people ?? PEOPLE.reality);
  const pace = shape.pace ?? PACE;
  const wanted: Obj = {
    ...kept,
    "wallet.payments": "on",
    "wallet.gifts": "on",
    "wallet.grant": 1000,
    "people.palette": "bot",
    "phone.ring": Math.max(Number(kept["phone.ring"] ?? 0), Math.min(RING, pace * RINGS * count)),
    "identity.source": "canon",
    "identity.groups": GROUPS,
    ...shape.rules,
  };
  const rules = Object.fromEntries(Object.entries(wanted).filter(([key]) => ids.includes(key.split(".")[0]!)));
  if (shape.rules && "canon.keep" in shape.rules && !("canon.not" in shape.rules)) {
    const keep = Number(rules["canon.keep"]);
    if (keep < WHOLE) rules["canon.not"] = pruned(stream(seedOf(seed, "canon")).draw, keep);
    else delete rules["canon.not"];
  }
  return {
    spec: SPEC,
    name: `gen-${seed}`,
    seed,
    start: shape.start ?? START,
    pace,
    front: (ids.includes("mind") && rules["mind.on"] !== false ? FRONT : PLAIN).filter((id) => ids.includes(id)),
    apps: apps.map((one) => entry(one, seed, made)) as Config["apps"],
    actors: Array.from({ length: count }, (_, i) => `bot-${i + 1}`),
    rules,
  };
}
