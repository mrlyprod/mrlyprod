import type { Term } from "@mrlyprod/apps/term/app.ts";
import { below, pick } from "@mrlyprod/os/contract/index.ts";
import type { Bot, Step } from "./types.ts";

export const LINES = ["list", "read", "read help", "read term", "read wallet/@me", 'call term.type {"line":"list"}'];

const bot: Bot<Term> = ({ me, actors, draw, world }): Step => {
  const others = actors.filter((one) => one !== me);
  if ("messages" in world.apps && others.length && below(draw, 2) === 0)
    return { verb: "type", args: { line: `call messages.send {"to":"${pick(draw, others)}","text":"typed hello"}` } };
  const apps = Object.keys(world.apps);
  if (below(draw, 2) === 0) return { verb: "type", args: { line: `read ${pick(draw, apps)}` } };
  return { verb: "type", args: { line: pick(draw, LINES) } };
};

export default bot;
