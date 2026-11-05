import { below } from "@mrlyprod/os/contract/index.ts";
import type { Mind } from "../apps/mind/app.ts";
import type { Bot, Step } from "./types.ts";

const DAY = 86_400_000;

const bot: Bot<Mind> = ({ state, me, at, draw }): Step => {
  if (!state.dials.on || below(draw, 4)) return null;
  const today = String(Math.floor((at - state.start) / DAY) + 1);
  const day = state.persons[me]?.days[today];
  if (!day || day.diary) return null;
  const met = day.met.length ? `I saw ${day.met.join(", ")}.` : "I kept to myself.";
  return { verb: "sleep", args: { diary: `Day ${today}: ${day.acts} things done. ${met}` } };
};

export default bot;
