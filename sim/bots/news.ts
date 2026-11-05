import type { News, Post } from "@mrlyprod/apps/news/app.ts";
import { dialed } from "./types.ts";

const knob = dialed("news");

import type { Json } from "@mrlyprod/os/contract/index.ts";
import { below, pick } from "@mrlyprod/os/contract/index.ts";
import type { Bot, Step } from "./types.ts";

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

const REPLIES = [
  "Is this confirmed?",
  "Good to know, thanks.",
  "I saw this too.",
  "Who told you?",
  "Not what I heard.",
];

const SHARES = ["Worth reading.", "Pass it on.", ""];

const WANT = 4;

const CROWD = 5;

type Org = { n: number; dissolved: number | null; members: Record<string, string> };

const orgsOf = (apps: Record<string, Json>, me: string): Org[] => {
  const people = apps.people;
  const orgs = people && typeof people === "object" && !Array.isArray(people) ? people.orgs : null;
  const list = orgs && typeof orgs === "object" && !Array.isArray(orgs) ? (Object.values(orgs) as Org[]) : [];
  return list.filter((one) => one.dissolved === null && me in one.members);
};

const replied = (draw: () => number, others: string[]) => {
  const text = pick(draw, REPLIES);
  return others.length && !below(draw, 4) ? `@${pick(draw, others)} ${text}` : text;
};

const ownMove = (draw: () => number, own: Post[]): Step => {
  if (!own.length) return null;
  const one = pick(draw, own);
  if (below(draw, 2)) return { verb: "delete", args: { post: one.n } };
  return { verb: "edit", args: { post: one.n, text: `${one.text} Updated.`.trim() } };
};

const reacted = (draw: () => number, state: News, me: string, others: string[]): Step => {
  const live = state.posts.filter((one) => one.deleted === null);
  if (!live.length) return null;
  const one = pick(draw, live);
  const turn = below(draw, 12);
  if (turn < 3) return { verb: one.likes.includes(me) ? "unlike" : "like", args: { post: one.n } };
  if (turn < 6) return { verb: "reply", args: { post: one.n, text: replied(draw, others) } };
  if (turn === 6 && one.by !== me && one.shared === null)
    return { verb: "share", args: { post: one.n, line: pick(draw, SHARES) } };
  return turn === 7
    ? ownMove(
        draw,
        live.filter((post) => post.by === me),
      )
    : null;
};

const bot: Bot<News> = ({ state, me, actors, draw, world }): Step => {
  const following = state.following[me] ?? [];
  const others = actors.filter((id) => id !== me);
  const sources = [...others, ...Object.keys(state.sources)].filter((source) => !following.includes(source));
  const room = following.length < Math.min(WANT, knob(world.rules, "follow"));
  if (sources.length && room && below(draw, 2)) return { verb: "follow", args: { source: pick(draw, sources) } };
  if (following.length > CROWD && !below(draw, 4)) return { verb: "unfollow", args: { source: pick(draw, following) } };
  if (below(draw, 2)) return reacted(draw, state, me, others);
  if (below(draw, 3)) return null;
  const orgs = orgsOf(world.apps, me);
  const args = { headline: pick(draw, HEADLINES), text: pick(draw, LINES) };
  return { verb: "post", args: orgs.length && below(draw, 2) ? { ...args, org: pick(draw, orgs).n } : args };
};

export default bot;
