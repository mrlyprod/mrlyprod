import type { Chat, Thread } from "@mrlyprod/apps/messages/app.ts";
import { dialed } from "./types.ts";

const knob = dialed("messages");

import { below, pick } from "@mrlyprod/os/contract/index.ts";
import type { Bot, Seen, Step } from "./types.ts";

export const LINES = [
  "Hi, how are you?",
  "Are you free for lunch today?",
  "Did you see the news this morning?",
  "Running a few minutes late, sorry.",
  "Can you send me that link again?",
  "Coffee at four?",
  "Thanks for yesterday, that was fun.",
  "What are you up to this weekend?",
];

export const REPLIES = [
  "Sounds good.",
  "Sure, see you then.",
  "Ha, yes.",
  "Not today, maybe tomorrow?",
  "On my way.",
  "Thanks!",
  "Let me check and get back to you.",
  "Good, you?",
];

const NAMES = ["Lunch crew", "Weekend plans", "Book club", "Neighbours", "Project team"];

type Orgs = { orgs?: Record<string, { n: number; name: string; dissolved: number | null; members: object }> };

const orgOf = ({ world, me, draw }: Seen<Chat>, state: Chat): Step => {
  const orgs = Object.values((world.apps.people as unknown as Orgs | undefined)?.orgs ?? {}).filter(
    (one) => one.dissolved === null && me in one.members && !state.threads[`org-${one.n}`],
  );
  if (!orgs.length) return null;
  const org = pick(draw, orgs);
  return { verb: "group", args: { name: org.name.slice(0, knob(world.rules, "name")), org: org.n } };
};

const answered = (key: string, thread: Thread, me: string, draw: () => number, size: number): Step => {
  const last = thread.messages.at(-1);
  const text = pick(draw, REPLIES).slice(0, size);
  if (last && last.from !== me && below(draw, 2)) return { verb: "reply", args: { thread: key, n: last.n, text } };
  return { verb: "post", args: { thread: key, text } };
};

const bot: Bot<Chat> = (seen): Step => {
  const { state, me, actors, draw, world } = seen;
  const keys = Object.keys(state.threads)
    .filter((key) => state.threads[key]!.between.includes(me))
    .sort();
  const size = knob(world.rules, "text");
  const unread = keys.filter((key) => state.threads[key]!.n > (state.seen[me]?.[key] ?? 0));
  if (unread.length) {
    const key = unread[0]!;
    if (below(draw, 3)) return answered(key, state.threads[key]!, me, draw, size);
    return { verb: "read", args: { thread: key } };
  }
  if (below(draw, 2)) return null;
  const people = (state.people ?? actors).filter((one) => one !== me);
  if (!people.length) return null;
  const groups = keys.filter((key) => key.startsWith("group-"));
  const roll = below(draw, 12);
  if (roll === 0) return orgOf(seen, state);
  if (roll === 1)
    return {
      verb: "group",
      args: { name: pick(draw, NAMES), people: [...new Set([pick(draw, people), pick(draw, people)])] },
    };
  if (roll === 2 && groups.length)
    return { verb: "add", args: { thread: pick(draw, groups), person: pick(draw, people) } };
  if (roll === 3 && groups.length) return { verb: "leave", args: { thread: pick(draw, groups) } };
  if (roll < 6 && groups.length)
    return { verb: "post", args: { thread: pick(draw, groups), text: pick(draw, LINES).slice(0, size) } };
  const known = keys
    .filter((key) => key.includes("+"))
    .map((key) => state.threads[key]!.between.find((one) => one !== me)!);
  const to = known.length && below(draw, 2) ? pick(draw, known) : pick(draw, people);
  return { verb: "send", args: { to, text: pick(draw, LINES).slice(0, size) } };
};

export default bot;
