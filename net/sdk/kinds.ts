import type { Json, Kind, World } from "../kernel/index.ts";
import { below, seedOf, stream } from "../kernel/index.ts";

/* KINDS */

export const KINDS: Kind[] = ["human", "org", "bot", "agent"];

const isKind = (value: Json | undefined): value is Kind =>
  typeof value === "string" && (KINDS as string[]).includes(value);

export function palette(rule: Json | undefined): Kind[] {
  const list = rule === undefined ? ["human"] : Array.isArray(rule) ? rule : [rule];
  if (!list.length || !list.every(isKind))
    throw new Error(`kinds: a rule is one of ${KINDS.join(", ")} or a list of them, not ${JSON.stringify(rule)}`);
  return list;
}

export const kindFor = (seed: number, id: string, kinds: readonly Kind[]): Kind =>
  kinds[below(stream(seedOf(seed, id)).draw, kinds.length)]!;

export function kindOf(world: Pick<World, "apps" | "rules">, id: string): Kind {
  const people = world.apps.people;
  const card = people && typeof people === "object" && !Array.isArray(people) ? people[id] : undefined;
  const kind = card && typeof card === "object" && !Array.isArray(card) ? card.kind : world.rules.kinds;
  return isKind(kind) ? kind : "human";
}
