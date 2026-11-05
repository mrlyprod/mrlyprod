import type { Json, Obj, World } from "@mrlyprod/os/kernel/index.ts";
import { reader } from "@mrlyprod/os/kernel/index.ts";

export const dialed =
  (id: string) =>
  <T extends Json = number>(rules: Obj, name: string): T =>
    reader(rules, id)<T>(name);

export type Seen<S = Json> = { state: S; me: string; actors: string[]; at: number; draw: () => number; world: World };

export type Step = { verb: string; args: Obj } | null;

export type Bot<S = Json> = (seen: Seen<S>) => Step;
