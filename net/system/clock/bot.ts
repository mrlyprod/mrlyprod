import type { Bot, Step } from "../../sdk/system.ts";
import { below, pick } from "../../sdk/system.ts";
import type { Clock } from "./app.ts";

/* WORDS */

const TIMERS: [number, string][] = [
  [10, "Tea"],
  [30, "Eggs"],
  [60, "Break"],
  [300, "Pasta"],
  [1500, "Focus"],
];

const ALARMS = ["Wake up", "Meeting", "Medicine", "Pick up the kids"];

const HOUR = 3_600_000;

const DAY = 24 * HOUR;

/* BOT */

const bot: Bot<Clock> = ({ state, me, at, draw }): Step => {
  const dial = state.dials[me] ?? { alarms: [], timers: [], rung: [] };
  const alarms = dial.alarms.filter((one) => one.at > at);
  const timers = dial.timers.filter((one) => one.at > at);
  if (dial.rung.length > 3) return { verb: "clear", args: {} };
  if (dial.timers.length < 2 && below(draw, 3)) {
    const [seconds, label] = pick(draw, TIMERS);
    return { verb: "timer", args: { seconds, label } };
  }
  if (dial.alarms.length < 2 && !below(draw, 3)) {
    const when = (Math.floor(at / DAY) + 1) * DAY + (6 + below(draw, 4)) * HOUR;
    return { verb: "alarm", args: { at: when, label: pick(draw, ALARMS), daily: !below(draw, 2) } };
  }
  if (timers.length && !below(draw, 6)) return { verb: "cancel", args: { id: pick(draw, timers).id } };
  if (alarms.length && !below(draw, 10)) return { verb: "cancel", args: { id: pick(draw, alarms).id } };
  return below(draw, 2) ? { verb: "tick", args: {} } : null;
};

export default bot;
