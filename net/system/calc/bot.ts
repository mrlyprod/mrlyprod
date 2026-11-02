import type { Bot, Step } from "../../sdk/system.ts";
import { below, pick } from "../../sdk/system.ts";
import type { Calc } from "./app.ts";

/* BOT */

const bot: Bot<Calc> = ({ state, me, draw }): Step => {
  const history = state.history[me] ?? [];
  if (history.length >= 20) return { verb: "clear", args: {} };
  const a = 1 + below(draw, 99);
  const b = 1 + below(draw, 99);
  const c = 1 + below(draw, 9);
  const forms = [
    `${a} + ${b}`,
    `${a} * ${b}`,
    `${a} / ${b}`,
    `(${a} + ${b}) * ${c}`,
    `${a}.${c}5 * ${b}`,
    `${a} - ${b} / ${c}`,
    `${c} ^ ${c}`,
    ...(history.length ? [`ans * ${c}`, `ans + ${a}`] : []),
  ];
  return below(draw, 3) ? { verb: "eval", args: { expr: pick(draw, forms) } } : null;
};

export default bot;
