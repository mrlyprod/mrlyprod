import type { Calendar } from "@mrlyprod/apps/calendar/app.ts";
import type { Obj } from "@mrlyprod/os/contract/index.ts";
import { below, pick, walk } from "@mrlyprod/os/contract/index.ts";
import type { Bot, Step } from "./types.ts";

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

const placed = (apps: Obj, draw: () => number): Obj => {
  const places = Object.keys((walk(apps, ["maps", "places"]) ?? {}) as object);
  if (places.length && below(draw, 2)) return { place: Number(pick(draw, places)) };
  const radius = walk(apps, ["maps", "radius"]);
  return typeof radius === "number" && !below(draw, 3)
    ? { q: below(draw, radius + 1), r: -below(draw, radius + 1) }
    : {};
};

const bot: Bot<Calendar> = ({ state, me, actors, at, draw, world }): Step => {
  const events = Object.values(state.events);
  const invited = events.filter(
    (event) => event.guests[me] === "invited" && (event.repeat || event.start + event.minutes * 60_000 >= at),
  );
  if (invited.length) return { verb: below(draw, 5) ? "accept" : "decline", args: { id: invited[0]!.id } };
  const ahead = events.filter((event) => event.owner === me && event.start > at);
  if (ahead.length && !below(draw, 10)) return { verb: "cancel", args: { id: pick(draw, ahead).id } };
  if (ahead.length && !below(draw, 10)) {
    const one = pick(draw, ahead);
    return { verb: "move", args: { id: one.id, start: one.start + DAY, ...placed(world.apps, draw) } };
  }
  const others = actors.filter((one) => one !== me);
  if (ahead.length && others.length && !below(draw, 10)) {
    const one = pick(draw, ahead);
    const guest = pick(draw, others);
    return one.guests[guest] ? null : { verb: "invite", args: { id: one.id, guest } };
  }
  if (ahead.length >= 5 || below(draw, 2)) return null;
  const day = Math.floor(at / DAY) + 1 + below(draw, 14);
  const guests = others.length ? [...new Set(Array.from({ length: below(draw, 4) }, () => pick(draw, others)))] : [];
  const begins = day * DAY + (9 + below(draw, 9)) * HOUR;
  const when: Obj = pick<Obj>(draw, [
    { start: begins },
    { start: new Date(begins).toISOString() },
    { in: 30 + below(draw, 600) },
  ]);
  const made: Obj = {
    title: pick(draw, TITLES),
    ...when,
    minutes: pick(draw, [30, 45, 60, 90]),
    guests,
    ...placed(world.apps, draw),
  };
  return { verb: "add", args: below(draw, 4) ? made : { ...made, repeat: pick(draw, ["daily", "weekly"]) } };
};

export default bot;
