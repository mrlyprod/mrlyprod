import type { Act, App, Move, Who, World } from "../kernel/index.ts";
import { below, check, seedOf, stream } from "../kernel/index.ts";
import type { Bot } from "../sdk/system.ts";
import { hooked } from "../sdk/system.ts";
import { goose } from "./goose.ts";

export type { Bot, Seen, Step } from "../sdk/system.ts";

/* TYPES */

export type Table = { world: () => World; list: () => { moves: Move[] }; call: (act: Act) => unknown };

export const SKIP = ["nav.open"];

/* ROUND */

export const drawFor = (seed: number, tick: number, id: string) => stream(seedOf((seed ^ tick) >>> 0, id)).draw;

export const botOf = (one: App): Bot | undefined => hooked(one).bot;

const owned = (verb: string, id: string) => {
  const head = verb.slice(0, verb.lastIndexOf("."));
  return head === id || head.endsWith(`.${id}`);
};

function scripted(world: World, moves: Move[], who: Who, at: number, draw: () => number, bots: Map<string, Bot>) {
  const ids = Object.keys(world.apps);
  const id = ids[below(draw, ids.length)]!;
  const bot = bots.get(id);
  if (!bot)
    return goose(
      moves.filter((move) => owned(move.verb, id)),
      draw,
      SKIP,
    );
  const act = bot({ state: world.apps[id]!, me: who.id, actors: world.actors ?? [], at, draw, world });
  const move = act && moves.find((one) => one.verb === `${id}.${act.verb}`);
  return act && move && !check(move.args, act.args) ? { verb: move.verb, args: act.args } : null;
}

export function round(table: Table, players: readonly Who[], at: number, apps: readonly App[] = []): number {
  const bots = new Map(apps.flatMap((one): [string, Bot][] => (botOf(one) ? [[one.id, botOf(one)!]] : [])));
  let acts = 0;
  for (const who of players) {
    const world = table.world();
    const draw = drawFor(world.seed, world.tick, bots.size ? `${who.id}@${at}` : who.id);
    const moves = table.list().moves;
    const act: Act | null = bots.size ? scripted(world, moves, who, at, draw, bots) : goose(moves, draw, SKIP);
    if (!act) continue;
    table.call({ ...act, who, at });
    acts++;
  }
  return acts;
}
