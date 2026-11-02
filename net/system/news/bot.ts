import type { Bot, Json, Step } from "../../sdk/system.ts";
import { below, pick } from "../../sdk/system.ts";
import type { News } from "./app.ts";

/* WORDS */

const HEADLINES = [
  "Market opens early on the first warm day",
  "New well dug on the east hill",
  "Bakery raises the price of bread by one",
  "Bridge closed for repairs until next week",
  "Library adds a shelf of new chapters",
  "Storm expected over the coast tomorrow",
  "Council meets on the harbour plan",
  "School term starts on day 30",
];

const LINES = ["Details to follow.", "Residents are asked to plan ahead.", "A second report comes tomorrow.", ""];

type Org = { n: number; dissolved: number | null; members: Record<string, string> };

const orgsOf = (apps: Record<string, Json>, me: string): Org[] => {
  const people = apps.people;
  const orgs = people && typeof people === "object" && !Array.isArray(people) ? people.orgs : null;
  const list = orgs && typeof orgs === "object" && !Array.isArray(orgs) ? (Object.values(orgs) as Org[]) : [];
  return list.filter((one) => one.dissolved === null && me in one.members);
};

/* POLICY */

export const bot: Bot<News> = ({ state, me, actors, draw, world }): Step => {
  const following = state.following[me] ?? [];
  const sources = [...actors.filter((id) => id !== me), ...Object.keys(state.sources)].filter(
    (source) => !following.includes(source),
  );
  if (sources.length && following.length < 4 && below(draw, 2))
    return { verb: "follow", args: { source: pick(draw, sources) } };
  if (following.length > 5 && !below(draw, 4)) return { verb: "unfollow", args: { source: pick(draw, following) } };
  if (below(draw, 3)) return null;
  const orgs = orgsOf(world.apps, me);
  const args = { headline: pick(draw, HEADLINES), text: pick(draw, LINES) };
  return { verb: "post", args: orgs.length && below(draw, 2) ? { ...args, org: pick(draw, orgs).n } : args };
};
