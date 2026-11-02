import type { Bot, Step } from "../../sdk/system.ts";
import { below, pick } from "../../sdk/system.ts";
import type { Maps } from "./app.ts";
import { inside, key } from "./app.ts";

/* WORDS */

export const NAMES = ["Home", "Market", "Well", "Mill", "Bakery", "School", "Harbour", "Bridge", "Orchard", "Workshop"];

/* POLICY */

export const bot: Bot<Maps> = ({ state, me, draw }): Step => {
  const mine = Object.values(state.places).filter((one) => one.by === me);
  if (mine.length >= 6 || (mine.length && !below(draw, 3))) {
    const one = pick(draw, mine);
    return below(draw, 2) || mine.length >= 6
      ? { verb: "unpin", args: { place: one.id } }
      : { verb: "rename", args: { place: one.id, name: `${pick(draw, NAMES)} of ${me}` } };
  }
  const span = 2 * state.radius + 1;
  for (let tries = 0; tries < 8; tries++) {
    const q = below(draw, span) - state.radius;
    const r = below(draw, span) - state.radius;
    if (inside(q, r, state.radius) && state.cells[key(q, r)] !== "water")
      return { verb: "pin", args: { q, r, name: pick(draw, NAMES) } };
  }
  return null;
};
