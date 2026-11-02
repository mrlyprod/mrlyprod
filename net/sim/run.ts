import type { App, Who, World } from "../kernel/index.ts";
import { kindOf } from "../sdk/kinds.ts";
import type { Table } from "./bots.ts";
import { round } from "./bots.ts";

/* TYPES */

export type Plan = { bots: number; rounds: number; beat: number };

export const BEAT = 1000;

/* PLAYERS */

export const botsOf = (n: number): Who[] =>
  Array.from({ length: n }, (_, i) => ({ id: `bot-${i + 1}`, kind: "bot" as const }));

export const playersOf = (world: World, bots: number): Who[] =>
  world.actors ? world.actors.map((id) => ({ id, kind: kindOf(world, id) })) : botsOf(bots);

/* RUN */

export function run(table: Table, plan: Plan, players: readonly Who[], apps: readonly App[] = []): void {
  const from = table.world().at;
  for (let i = 0; i < plan.rounds; i++) round(table, players, from + (i + 1) * plan.beat, apps);
}
