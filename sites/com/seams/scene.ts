import type { App, Ctx, Json, Obj, Schema, Verb } from "@mrlyprod/os/kernel/index.ts";
import { app, canon, copy } from "@mrlyprod/os/kernel/index.ts";
import type { Key } from "@mrlyprod/ui/kit/keys.tsx";
import { rng } from "@mrlyprod/ui/kit/rand.ts";
import type { Nudge, Raw, Roll, Spec, Value } from "@mrlyprod/ui/kit/spec.ts";
import { rolled, tidy } from "@mrlyprod/ui/kit/spec.ts";
import { list } from "@mrlyprod/ui/kit/verbs.ts";

export type Played = {
  boot: (config: Raw) => unknown;
  moves: (state: unknown) => unknown[];
  step: (state: unknown, act: unknown) => unknown;
  deal: (value: Value, table: Record<string, unknown>) => unknown;
  reducer?: (value: Value) => Pick<Played, "moves" | "step">;
};

export type Scene<U = unknown> = {
  id: string;
  user: string;
  title: string;
  line: string;
  spec?: Spec<U>;
  defaults?: Value;
  units?: U;
  engine?: Played;
  doors?: Record<string, unknown>;
  reroll?: Roll<U>;
  acts?: Record<string, Nudge<U>>;
  keys?: readonly Key<never>[];
};

type Named = { verb: string; args?: Obj };

const STAGE = { w: 1280, h: 800, dpr: 1, lock: 0 };

const RUN = 16;

const named = (act: unknown): act is Named =>
  !!act && typeof act === "object" && typeof (act as { verb?: unknown }).verb === "string";

const word = (verb: string) => verb.toLowerCase().replace(/[^a-z0-9]/g, "");

const tail = (name: string) => name.slice(name.lastIndexOf(".") + 1);

const NONE: Schema = { type: "object", additionalProperties: false };

function choices(values: Json[]): Schema {
  const unique = [...new Map(values.map((one) => [canon(one), one])).values()];
  const ints = unique.every((one) => Number.isInteger(one)) ? (unique as number[]).sort((a, b) => a - b) : null;
  if (ints && ints.length > RUN && ints.at(-1)! - ints[0] === ints.length - 1)
    return { type: "integer", minimum: ints[0], maximum: ints.at(-1)! };
  return { enum: unique };
}

export function argsOf(acts: readonly Named[]): Schema {
  const shapes = acts.map((act) => act.args ?? {});
  const keys = [...new Set(shapes.flatMap((one) => Object.keys(one)))].sort();
  if (!keys.length) return { type: "object", additionalProperties: false };
  if (shapes.some((one) => Object.keys(one).sort().join() !== keys.join()))
    return { type: "object", properties: { args: { enum: shapes } }, required: ["args"], additionalProperties: false };
  const properties = Object.fromEntries(keys.map((key) => [key, choices(shapes.map((one) => one[key]))]));
  return { type: "object", properties, required: keys, additionalProperties: false };
}

export function scene<U>(one: Scene<U>): App {
  const { spec, units, engine, doors, reroll, acts: nudges = {}, keys = [] } = one;
  type State = Record<string, Json>;
  const tidied = (raw: Raw): Value => {
    const { play: _, ...rest } = raw;
    const value = rest as Value;
    return spec ? { ...value, ...tidy(spec, value, units) } : value;
  };
  const rules = (value: Value) => (engine ? (engine.reducer?.(value) ?? engine) : null);
  const dealt = (value: Value): State => {
    if (!engine) return value as State;
    const seed = Number(value.seed);
    return {
      ...value,
      play: copy(engine.deal(value, { ...STAGE, seed, rand: rng(seed), units: { ...doors, ...units } }) as Json),
    };
  };
  const knobs = (state: State): Value => tidied(state);
  const turned = (state: State, ctx: Ctx, patch: (value: Value) => Raw): State => {
    try {
      return dealt(tidied({ ...knobs(state), ...patch(knobs(state)) }));
    } catch (error) {
      return ctx.refuse(`${one.id} cannot take these knobs: ${(error as Error).message}`);
    }
  };
  const legal = (state: State): Named[] => (engine ? rules(knobs(state))!.moves(state.play).filter(named) : []);
  const acts = (state: State, verb: string) => legal(state).filter((act) => word(act.verb) === verb);
  const opening = dealt(tidied({ ...one.defaults, seed: 0 }));
  const played = [...new Set(legal(opening).map((act) => word(act.verb)))];
  for (const verb of [...played, ...Object.keys(nudges)])
    if (verb === "set" || verb === "reroll") throw new Error(`apps: ${one.id} plays ${verb}, a knob verb`);
  for (const verb of Object.keys(nudges))
    if (played.includes(verb)) throw new Error(`apps: ${one.id} both plays and turns ${verb}`);
  const listed = (state: State) => list({ id: one.id, spec, keys, value: knobs(state), units });
  const setArgs = (state: State): Schema => listed(state)[0].args as Schema;
  const pressed = (state: State, verb: string) => listed(state).find((act) => tail(act.name) === verb);
  const first = (state: State, verb: string) =>
    ([] as string[]).concat(keys.find((row) => row.act === verb && (!row.when || row.when(knobs(state))))!.key)[0];
  const verbs: Record<string, Verb<State>> = {
    set: {
      line: engine
        ? "Turn knobs and deal again: any of the keys below, each checked against its row."
        : "Turn knobs: any of the keys below, each checked against its row.",
      args: setArgs(opening),
      run: (state, args, ctx) => turned(state, ctx, () => args as Raw),
    },
    reroll: {
      line: reroll
        ? "A new seed and the knobs it rolls, as the page's Reroll."
        : engine
          ? "A new seed, dealt again."
          : "A new seed: the same knobs, another picture.",
      run: (state, _args, ctx) => {
        const seed = ctx.draw();
        return turned(state, ctx, (value) => rolled(value, seed, units, reroll));
      },
    },
  };
  for (const [verb, nudge] of Object.entries(nudges))
    verbs[verb] = {
      line: `Turn knobs as the page's ${pressed(opening, verb)?.title ?? verb} key does.`,
      args: (pressed(opening, verb)?.args as Schema | undefined) ?? NONE,
      run: (state, args, ctx) =>
        turned(state, ctx, (value) => nudge(value, String(args.key ?? first(state, verb)), units)),
    };
  for (const verb of played)
    verbs[verb] = {
      line: `Play: ${verb}.`,
      args: { type: "object" },
      run: (state, args, ctx) => {
        const want = canon(args.args ?? args);
        const act = acts(state, verb).find((one) => canon(one.args ?? {}) === want);
        if (!act) return ctx.refuse(`${one.id} cannot ${verb} that way now`);
        return { ...state, play: copy(rules(knobs(state))!.step(state.play, act) as Json) };
      },
    };
  return app<State>({
    id: one.id,
    title: one.title,
    line: one.line,
    boot: ({ seed, config }) => dealt(tidied({ ...one.defaults, seed, ...(config as Raw) })),
    verbs,
    moves: (state) => [
      "set",
      "reroll",
      ...Object.keys(nudges).filter((verb) => pressed(state, verb)),
      ...new Set(legal(state).map((act) => word(act.verb))),
    ],
  });
}
