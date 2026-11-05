import type { Ctx, Json, Obj, Wish } from "@mrlyprod/os/contract/index.ts";
import { act, below, seedOf, stream } from "@mrlyprod/os/contract/index.ts";

export const KINDS = ["human", "org", "bot", "agent"];

export const NOTICE = 140;

const SHOWN = 12;

export const notice = (to: string, text: string, system = false, low = false): Wish =>
  act("notifications.notice", {
    to,
    text: text.length > NOTICE ? `${text.slice(0, NOTICE - 3)}...` : text,
    ...(system ? { system } : {}),
    ...(low ? { low } : {}),
  });

export const blockedBy = (ctx: Ctx, id: string, by: string) =>
  ((ctx.read(`people/blocks/${id}`) as string[] | null) ?? []).includes(by);

export const mutedBy = (ctx: Ctx, id: string, by: string) =>
  ((ctx.read(`people/mutes/${id}`) as string[] | null) ?? []).includes(by);

export function reach(ctx: Ctx, person: string) {
  const me = ctx.who.id;
  if (blockedBy(ctx, person, me)) ctx.refuse(`${person} is not taking this from you`);
  if (blockedBy(ctx, me, person)) ctx.refuse(`you blocked ${person}; unblock first`);
}

export const among = (list: readonly string[]) =>
  list.length > SHOWN ? `${list.slice(0, SHOWN).join(", ")} and ${list.length - SHOWN} more` : list.join(", ");

export function someone(people: readonly string[] | null, id: string, ctx: Ctx, extra: readonly string[] = []): string {
  if (!people || people.includes(id) || extra.includes(id)) return id;
  return ctx.refuse(`nobody here is called ${id}; try ${among([...people, ...extra]) || "nobody"}`);
}

export function known<T>(table: Record<string, T>, id: string, what: string, ctx: Ctx): T {
  if (Object.hasOwn(table, id)) return table[id]!;
  const known = Object.keys(table);
  return ctx.refuse(`no ${what} ${id}${known.length ? `; try ${among(known)}` : "; there are none yet"}`);
}

const isKind = (value: Json | undefined): value is string => typeof value === "string" && KINDS.includes(value);

export function palette(rule: Json | undefined): string[] {
  const list = rule === undefined || rule === null ? ["human"] : Array.isArray(rule) ? rule : [rule];
  if (!list.length || !list.every(isKind))
    throw new Error(`kinds: a rule is one of ${KINDS.join(", ")} or a list of them, not ${JSON.stringify(rule)}`);
  return list as string[];
}

export const kindFor = (seed: number, id: string, kinds: readonly string[]): string =>
  kinds[below(stream(seedOf(seed, id)).draw, kinds.length)]!;

export function kindOf(world: { apps: Obj }, id: string): string {
  const people = world.apps.people;
  const card = people && typeof people === "object" && !Array.isArray(people) ? people[id] : undefined;
  const kind = card && typeof card === "object" && !Array.isArray(card) ? card.kind : undefined;
  return typeof kind === "string" ? kind : "human";
}
