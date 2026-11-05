import type { Knob, Knobs, Schema, Who } from "@mrlyprod/os/contract/index.ts";
import { app, sliced } from "@mrlyprod/os/contract/index.ts";
import { evaluate, exact, shown } from "./exact.ts";

export type Line = { expr: string; result: string; at: number };

export type Calc = { history: Record<string, Line[]> };

const SHOWN = 5;

const KNOBS = {
  expr: {
    line: "Characters one expression may hold.",
    schema: { type: "integer", minimum: 10, maximum: 5000 },
    reality: 200,
  },
  keep: {
    line: "Lines one person's history keeps.",
    schema: { type: "integer", minimum: 1, maximum: 5000 },
    reality: 50,
  },
} satisfies Knobs;

const typing = (knob: Knob): Schema => ({
  type: "object",
  properties: { expr: { type: "string", minLength: 1, maxLength: knob("expr") } },
  required: ["expr"],
  additionalProperties: false,
});

const mine = (state: Calc, id: string) => {
  const history = [...(state.history[id] ?? [])].reverse();
  return { ans: history[0]?.result ?? null, history };
};

function whole(state: Calc, who: Who | null) {
  if (!who) return { ans: null, history: [] };
  const full = mine(state, who.id);
  return { ...full, history: full.history.slice(0, SHOWN), more: "your latest lines; calc/@me holds them all" };
}

export default app<Calc>({
  id: "calc",
  title: "Calculator",
  line: "Exact arithmetic on integers and decimals: + - * / ^ and parentheses, ans for your last result, a history per user.",
  boot: () => ({ history: {} }),
  verbs: {
    eval: {
      line: (knob) =>
        `Work out an expression of up to ${knob("expr")} characters, exactly; a result is an integer, a decimal or a fraction; you keep the last ${knob("keep")}, privately.`,
      args: typing,
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const expr = String(args.expr);
        const history = state.history[me] ?? [];
        const last = history.at(-1);
        const value = evaluate(expr, last ? exact(last.result) : null);
        if (typeof value === "string") return ctx.refuse(value);
        const line: Line = { expr: expr.trim(), result: shown(value), at: ctx.at };
        ctx.say({ note: `${line.expr} = ${line.result}`, result: line.result });
        return { history: { ...state.history, [me]: [...history, line].slice(-ctx.knob("keep")) } };
      },
    },
    clear: {
      line: "Clear your history.",
      run: (state, _args, ctx) => {
        ctx.say({ note: "Cleared your history" });
        return { history: { ...state.history, [ctx.who.id]: [] } };
      },
    },
  },
  read: sliced(mine, whole),
  knobs: KNOBS,
});
