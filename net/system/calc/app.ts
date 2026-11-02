import { app, sliced } from "../../sdk/system.ts";
import bot from "./bot.ts";
import { evaluate, exact, shown } from "./exact.ts";

/* TYPES */

export type Line = { expr: string; result: string; at: number };

export type Calc = { history: Record<string, Line[]> };

export const EXPR = 200;

export const KEEP = 50;

/* APP */

export default app<Calc>({
  id: "calc",
  title: "Calculator",
  line: "Exact arithmetic on integers and decimals: + - * / ^ and parentheses, ans for your last result, a history per user.",
  boot: () => ({ history: {} }),
  verbs: {
    eval: {
      line: `Work out an expression of up to ${EXPR} characters, exactly; a result is an integer, a decimal or a fraction; you keep the last ${KEEP}.`,
      args: {
        type: "object",
        properties: { expr: { type: "string", minLength: 1, maxLength: EXPR } },
        required: ["expr"],
        additionalProperties: false,
      },
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const expr = String(args.expr);
        const history = state.history[me] ?? [];
        const last = history.at(-1);
        const value = evaluate(expr, last ? exact(last.result) : null);
        if (typeof value === "string") return ctx.refuse(value);
        const line: Line = { expr: expr.trim(), result: shown(value), at: ctx.at };
        return { history: { ...state.history, [me]: [...history, line].slice(-KEEP) } };
      },
    },
    clear: {
      line: "Clear your history.",
      run: (state, _args, ctx) => ({ history: { ...state.history, [ctx.who.id]: [] } }),
    },
  },
  read: sliced((state: Calc, id: string) => ({ history: [...(state.history[id] ?? [])].reverse() })),
  bot,
});
