import type { App, Json, Obj, Peek, Schema, Setting, Who, Wish } from "../kernel/index.ts";
import { below, walk } from "../kernel/index.ts";

export type {
  Act,
  App,
  Boot,
  Called,
  Ctx,
  Json,
  Knob,
  Knobs,
  Obj,
  Peek,
  Schema,
  Sent,
  Setting,
  Verb,
  Who,
  Wish,
} from "../kernel/index.ts";
export { app, below, canon, copy, digest, nearest, seedOf, stream, walk } from "../kernel/index.ts";

export type Named = Setting & { name: string };

export const knobsOf = (apps: readonly App[]): Named[] =>
  apps
    .flatMap((one) => Object.entries(one.knobs ?? {}).map(([name, knob]) => ({ ...knob, name: `${one.id}.${name}` })))
    .sort((a, b) => a.name.localeCompare(b.name));

export const pick = <T>(draw: () => number, list: readonly T[]): T => list[below(draw, list.length)]!;

export const SLICE = "@";

export const ME = "me";

export const named = (id: string) => {
  const words = id.split("-").join(" ");
  return words.charAt(0).toUpperCase() + words.slice(1);
};

export const sliced =
  <S>(
    slice: (state: S, id: string, who: Who | null, at: number, peek: Peek) => Json | undefined,
    whole?: (state: S, who: Who | null, at: number, peek: Peek) => Json,
    shown = false,
  ) =>
  (state: S, path: string[], who: Who | null, at = 0, peek: Peek = () => null): Json | undefined => {
    const [head = "", ...rest] = path;
    if (head.startsWith(SLICE)) {
      const named = head.slice(SLICE.length);
      const id = named === ME && who ? who.id : named;
      return who && id !== who.id && !shown ? undefined : walk(slice(state, id, who, at, peek), rest);
    }
    if (whole) return walk(whole(state, who, at, peek), path);
    return walk(who ? slice(state, who.id, who, at, peek) : (state as Json), path);
  };

export const REPLY: Schema = {
  type: "object",
  properties: { wish: { type: "string" }, ok: { type: "boolean" }, data: {} },
  required: ["wish", "ok"],
  additionalProperties: false,
};

export const ACT = "act";

export const act = (verb: string, args: Obj = {}): Wish => ({ kind: ACT, data: { verb, args } });

export const asMe = (verb: string, args: Obj = {}): Wish => ({ kind: ACT, data: { verb, args, plain: true } });

export const appOf = (verb: string) => verb.slice(0, Math.max(0, verb.indexOf(".")));

export const verbOf = (verb: string) => verb.slice(verb.indexOf(".") + 1);
