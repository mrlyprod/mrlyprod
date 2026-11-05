import type { Call, Calls } from "@mrlyprod/apps/phone/app.ts";
import { current, live } from "@mrlyprod/apps/phone/app.ts";
import { dialed } from "./types.ts";

const knob = dialed("phone");

import { below, pick } from "@mrlyprod/os/contract/index.ts";
import type { Bot, Step } from "./types.ts";

const TALK = [
  "Can you hear me?",
  "How has your day been?",
  "I wanted to run something by you.",
  "Sounds good, thanks.",
];

const MESSAGES = ["Call me back when you can.", "Sorry I missed you.", "Nothing urgent, talk soon."];

const bot: Bot<Calls> = ({ state, me, actors, at, draw, world }): Step => {
  const lines = state.open.map((id) => current(state.calls[id]!, at)).filter(live);
  const busy = (id: string) => lines.find((call) => call.from === id || call.to === id);
  const size = knob(world.rules, "text");
  const line = busy(me);
  if (line?.status === "ringing" && line.to === me)
    return { verb: below(draw, 4) ? "accept" : "decline", args: { id: line.id } };
  if (line?.status === "ringing") return below(draw, 6) ? null : { verb: "end", args: { id: line.id } };
  if (line) {
    if (line.said.length < 4 && below(draw, 4)) return { verb: "say", args: { text: pick(draw, TALK).slice(0, size) } };
    return below(draw, 3) ? null : { verb: "end", args: { id: line.id } };
  }
  const log = (state.logs[me] ?? []).flatMap((id) => (state.calls[id] ? [current(state.calls[id]!, at)] : []));
  const unheard = log.find((call: Call) => call.to === me && call.voicemail && !call.voicemail.heard);
  if (unheard && below(draw, 2)) return { verb: "listen", args: { id: unheard.id } };
  const silent = log.find(
    (call: Call) => call.from === me && (call.status === "missed" || call.status === "declined") && !call.voicemail,
  );
  if (silent && knob(world.rules, "voicemail") && !below(draw, 3))
    return { verb: "voicemail", args: { id: silent.id, text: pick(draw, MESSAGES).slice(0, size) } };
  const free = actors.filter((one) => one !== me && !busy(one));
  return free.length && !below(draw, 4) ? { verb: "ring", args: { to: pick(draw, free) } } : null;
};

export default bot;
