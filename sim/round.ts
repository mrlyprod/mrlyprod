import type { Act, Json, Move, Obj, Schema, Who, World } from "@mrlyprod/os/kernel/index.ts";
import { below, check, seedOf, stream } from "@mrlyprod/os/kernel/index.ts";
import type { Bot } from "./bots/types.ts";
import { goose } from "./goose.ts";

export type Table = {
  world: () => World;
  list: () => { moves: Move[] };
  call: (act: Act) => unknown;
  dropped?: (verb: string, why: string) => void;
};

export const SKIP: string[] = [];

export const drawFor = (seed: number, tick: number, id: string) => stream(seedOf((seed ^ tick) >>> 0, id)).draw;

const owned = (verb: string, id: string) => {
  const head = verb.slice(0, verb.lastIndexOf("."));
  return head === id || head.endsWith(`.${id}`);
};

function clamped(schema: Schema, value: Json): Json {
  if (typeof value === "string") return value.slice(0, schema.maxLength ?? value.length);
  if (typeof value === "number") return Math.max(schema.minimum ?? value, Math.min(schema.maximum ?? value, value));
  if (Array.isArray(value)) {
    const kept = value.slice(0, schema.maxItems ?? value.length);
    return schema.items ? kept.map((one) => clamped(schema.items!, one)) : kept;
  }
  if (value === null || typeof value !== "object") return value;
  const fields = schema.properties ?? {};
  return Object.fromEntries(
    Object.entries(value).map(([key, one]) => [key, fields[key] ? clamped(fields[key]!, one) : one]),
  );
}

function scripted(table: Table, who: Who, at: number, draw: () => number, bots: Record<string, Bot>): Act | null {
  const world = table.world();
  const { moves } = table.list();
  const ids = Object.keys(world.apps);
  const id = ids[below(draw, ids.length)]!;
  const bot = Object.hasOwn(bots, id) ? bots[id] : undefined;
  if (!bot)
    return goose(
      moves.filter((move) => owned(move.verb, id)),
      draw,
      SKIP,
    );
  const act = bot({ state: world.apps[id]!, me: who.id, actors: world.actors ?? [], at, draw, world });
  if (!act) return null;
  const verb = `${id}.${act.verb}`;
  const move = moves.find((one) => one.verb === verb);
  const args = move ? (clamped(move.args, act.args) as Obj) : act.args;
  const why = move ? check(move.args, args) : "not legal now";
  if (!why) return { verb, args };
  table.dropped?.(verb, why);
  return null;
}

export function round(table: Table, players: readonly Who[], at: number, bots: Record<string, Bot>): number {
  let acts = 0;
  for (const who of players) {
    const world = table.world();
    const act = scripted(table, who, at, drawFor(world.seed, world.tick, `${who.id}@${at}`), bots);
    if (!act) continue;
    table.call({ ...act, who, at });
    acts++;
  }
  return acts;
}
