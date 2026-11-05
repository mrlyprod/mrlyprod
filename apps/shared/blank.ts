import type { App, Knob, Knobs, Schema, Who } from "@mrlyprod/os/contract/index.ts";
import { app, sliced } from "@mrlyprod/os/contract/index.ts";

export type Kept = { saved: Record<string, string[]> };

export type Blank = {
  id: string;
  title: string;
  line: string;
  source: string;
  noun: string;
  add: string;
  drop: string;
};

const KNOBS = {
  keep: { line: "Things one person keeps.", schema: { type: "integer", minimum: 1, maximum: 200 }, reality: 10 },
  name: { line: "Characters in one kept name.", schema: { type: "integer", minimum: 8, maximum: 200 }, reality: 40 },
} satisfies Knobs;

export const blank = ({ id, title, line, source, noun, add, drop }: Blank): App => {
  const named = (knob: Knob): Schema => ({
    type: "object",
    properties: { name: { type: "string", minLength: 1, maxLength: knob("name") } },
    required: ["name"],
    additionalProperties: false,
  });
  const mine = (state: Kept, who: string) => ({ source, [noun]: state.saved[who] ?? [] });
  const whole = (state: Kept, who: Who | null) => (who ? mine(state, who.id) : { source });
  return app<Kept>({
    id,
    title,
    line,
    knobs: KNOBS,
    boot: () => ({ saved: {} }),
    verbs: {
      [add]: {
        line: (knob) => `Keep a ${noun.slice(0, -1)} by name; you keep at most ${knob("keep")}.`,
        args: named,
        run: (state, args, ctx) => {
          const me = ctx.who.id;
          const name = String(args.name).trim();
          const have = state.saved[me] ?? [];
          const most = ctx.knob("keep");
          if (!name) return ctx.refuse("the name is only spaces; name it");
          if (have.includes(name)) return ctx.refuse(`you keep ${name} already`);
          if (have.length >= most) return ctx.refuse(`you keep ${most} already; let one go first`);
          return { saved: { ...state.saved, [me]: [...have, name] } };
        },
      },
      [drop]: {
        line: `Let go of a ${noun.slice(0, -1)} you keep.`,
        args: named,
        run: (state, args, ctx) => {
          const me = ctx.who.id;
          const name = String(args.name).trim();
          const have = state.saved[me] ?? [];
          if (!have.includes(name)) return ctx.refuse(`you keep no ${name}`);
          return { saved: { ...state.saved, [me]: have.filter((one) => one !== name) } };
        },
      },
    },
    read: sliced(
      (state, who) => mine(state, who),
      (state, who) => whole(state, who),
    ),
  });
};
