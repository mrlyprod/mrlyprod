import type { Bot, Step } from "../../sdk/system.ts";
import { below, pick } from "../../sdk/system.ts";
import type { Calendar } from "./app.ts";

/* WORDS */

const TITLES = [
  "Team sync",
  "Lunch",
  "Project review",
  "Coffee",
  "Planning",
  "Dentist",
  "Gym",
  "Book club",
  "Call home",
];

const HOUR = 3_600_000;

const DAY = 24 * HOUR;

/* BOT */

const bot: Bot<Calendar> = ({ state, me, actors, at, draw }): Step => {
  const events = Object.values(state.events);
  const invited = events.filter((event) => event.guests[me] === "invited");
  if (invited.length) return { verb: below(draw, 5) ? "accept" : "decline", args: { id: invited[0]!.id } };
  const ahead = events.filter((event) => event.owner === me && event.start > at);
  if (ahead.length && !below(draw, 10)) return { verb: "cancel", args: { id: pick(draw, ahead).id } };
  if (ahead.length && !below(draw, 10)) {
    const one = pick(draw, ahead);
    return { verb: "move", args: { id: one.id, start: one.start + DAY } };
  }
  if (ahead.length >= 5 || below(draw, 2)) return null;
  const others = actors.filter((one) => one !== me);
  const day = Math.floor(at / DAY) + 1 + below(draw, 14);
  const guests = others.length ? [...new Set(Array.from({ length: below(draw, 4) }, () => pick(draw, others)))] : [];
  return {
    verb: "add",
    args: {
      title: pick(draw, TITLES),
      start: day * DAY + (9 + below(draw, 9)) * HOUR,
      minutes: pick(draw, [30, 45, 60, 90]),
      guests,
    },
  };
};

export default bot;
