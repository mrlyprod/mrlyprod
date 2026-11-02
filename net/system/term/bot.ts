import type { Bot, Step } from "../../sdk/system.ts";
import { below, pick } from "../../sdk/system.ts";
import type { Term } from "./app.ts";

/* LINES */

export const LINES = ["list", "read", "read iden", "read term", 'call term.type {"line":"list"}'];

/* BOT */

const bot: Bot<Term> = ({ me, actors, draw, world }): Step => {
  const others = actors.filter((one) => one !== me);
  if ("messages" in world.apps && others.length && below(draw, 2) === 0)
    return { verb: "type", args: { line: `call messages.send {"to":"${pick(draw, others)}","text":"typed hello"}` } };
  const apps = Object.keys(world.apps);
  if (below(draw, 2) === 0) return { verb: "type", args: { line: `read ${pick(draw, apps)}` } };
  return { verb: "type", args: { line: pick(draw, LINES) } };
};

export default bot;
