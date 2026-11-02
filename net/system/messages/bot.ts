import type { Bot, Step } from "../../sdk/system.ts";
import { below, pick } from "../../sdk/system.ts";
import type { Chat } from "./app.ts";

/* WORDS */

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

/* BOT */

const bot: Bot<Chat> = ({ state, me, actors, draw }): Step => {
  const keys = Object.keys(state.threads)
    .filter((key) => state.threads[key]!.between.includes(me))
    .sort();
  const other = (key: string) => state.threads[key]!.between.find((one) => one !== me)!;
  const unread = keys.filter((key) => state.threads[key]!.n > (state.seen[me]?.[key] ?? 0));
  if (unread.length) {
    const key = unread[0]!;
    if (below(draw, 3)) return { verb: "send", args: { to: other(key), text: pick(draw, REPLIES) } };
    return { verb: "read", args: { thread: key } };
  }
  if (below(draw, 2)) return null;
  const people = (state.people ?? actors).filter((one) => one !== me);
  if (!people.length) return null;
  const known = keys.map(other).filter((one) => people.includes(one));
  const to = known.length && below(draw, 2) ? pick(draw, known) : pick(draw, people);
  return { verb: "send", args: { to, text: pick(draw, LINES) } };
};

export default bot;
