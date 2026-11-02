import type { Bot, Step } from "../../sdk/system.ts";
import { below, pick } from "../../sdk/system.ts";
import type { Sheets } from "./app.ts";
import { COLS } from "./formula.ts";

/* WORDS */

export const TITLES = ["Budget", "Pantry", "Hours", "Seeds"];

const LABELS = ["rent", "food", "travel", "total", "lentils", "onions", "hours"];

const NUMBERS = ["900", "300", "120", "3.50", "12", "0.25", "48"];

const FORMULAS = ["=A1+A2", "=SUM(A1:A5)", "=B2*2", "=(A1-A2)/4", "=SUM(A1:B3)-C1", "=-A3", "=B1/3"];

/* BOT */

const bot: Bot<Sheets> = ({ state, me, actors, draw }): Step => {
  const own = Object.keys(state.sheets).filter((id) => state.sheets[id]!.owner === me);
  const shared = Object.keys(state.sheets).filter((id) => state.sheets[id]!.editors.includes(me));
  const mine = [...own, ...shared];
  if (!own.length || below(draw, 10) === 0) return { verb: "create", args: { title: pick(draw, TITLES) } };
  const id = pick(draw, mine);
  const sheet = state.sheets[id]!;
  const others = actors.filter((one) => one !== me && !sheet.editors.includes(one));
  if (own.includes(id) && others.length && below(draw, 6) === 0)
    return { verb: "share", args: { sheet: id, with: pick(draw, others) } };
  if (own.length > 5 && below(draw, 4) === 0) return { verb: "delete", args: { sheet: pick(draw, own) } };
  const col = COLS[below(draw, 3)]!;
  const row = 1 + below(draw, 6);
  const kind = below(draw, 4);
  const text = kind === 0 ? pick(draw, LABELS) : kind === 3 ? pick(draw, FORMULAS) : pick(draw, NUMBERS);
  return { verb: "set", args: { sheet: id, col, row, text } };
};

export default bot;
