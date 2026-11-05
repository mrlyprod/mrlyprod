import type { Obj } from "@mrlyprod/os/contract/index.ts";
import { canon } from "@mrlyprod/os/contract/index.ts";

export type Traits = Obj;

export type Good = { name: string; traits: Traits };

export type Held = Good & { n: number };

export type Stocked = Good & { shelf: string; price: number };

export const NAME = 40;

export const PEOPLE = "people";

export const slugOf = (name: string) =>
  name
    .trim()
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, "-")
    .replace(/^-|-$/g, "");

export const alike = (a: Traits, b: Traits) => canon(a) === canon(b);

export function placed(mine: Record<string, Held>, good: Good): { key: string; name: string } {
  let name = good.name;
  for (let i = 2; ; i++) {
    const key = slugOf(name);
    const had = Object.hasOwn(mine, key) ? mine[key]! : null;
    if (!had) return { key, name };
    if (alike(had.traits, good.traits)) return { key, name: had.name };
    name = `${good.name} ${i}`;
  }
}
