import type { Bot, Step } from "../../sdk/system.ts";
import { below, pick } from "../../sdk/system.ts";
import type { Call, Calls } from "./app.ts";
import { RING } from "./app.ts";

/* BOT */

const bot: Bot<Calls> = ({ state, me, actors, at, draw }): Step => {
  const live = (call: Call) => call.status === "active" || (call.status === "ringing" && call.at + RING > at);
  const lines = Object.values(state.calls).filter(live);
  const busy = (id: string) => lines.find((call) => call.from === id || call.to === id);
  const line = busy(me);
  if (line?.status === "ringing" && line.to === me)
    return { verb: below(draw, 4) ? "accept" : "decline", args: { id: line.id } };
  if (line?.status === "ringing") return below(draw, 6) ? null : { verb: "end", args: { id: line.id } };
  if (line) return below(draw, 3) ? null : { verb: "end", args: { id: line.id } };
  const free = actors.filter((one) => one !== me && !busy(one));
  return free.length && !below(draw, 4) ? { verb: "ring", args: { to: pick(draw, free) } } : null;
};

export default bot;
