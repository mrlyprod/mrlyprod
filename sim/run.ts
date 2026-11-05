import { kindOf } from "@mrlyprod/apps/shared/people.ts";
import type { Who, World } from "@mrlyprod/os/kernel/index.ts";
import { BOTS } from "./bots/index.ts";
import type { Table } from "./round.ts";
import { round } from "./round.ts";

export type Plan = { bots: number; rounds: number; beat: number };

export const BEAT = 1000;

export const botsOf = (n: number): Who[] => Array.from({ length: n }, (_, i) => ({ id: `bot-${i + 1}`, kind: "bot" }));

export const playersOf = (world: World, bots: number): Who[] =>
  world.actors ? world.actors.map((id) => ({ id, kind: kindOf(world, id) })) : botsOf(bots);

export function run(table: Table, plan: Plan, players: readonly Who[]): void {
  const from = table.world().at;
  for (let i = 0; i < plan.rounds; i++) round(table, players, from + (i + 1) * plan.beat, BOTS);
}
