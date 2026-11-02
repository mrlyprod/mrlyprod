import type { Bot, Step } from "../../sdk/system.ts";
import { below, pick } from "../../sdk/system.ts";
import type { Mail } from "./app.ts";

/* WORDS */

const LETTERS: [string, string][] = [
  ["Lunch on Friday", "Are you free for lunch on Friday?"],
  ["Notes from today", "Here are my notes from today's meeting."],
  ["Quick question", "Do you have a minute later today?"],
  ["Weekend plans", "Any plans for the weekend?"],
  ["The report", "I sent the report; let me know what you think."],
  ["Meeting moved", "The meeting moved to Thursday at ten."],
  ["Book recommendation", "I finished the book you lent me. Thank you."],
  ["Dinner", "Dinner at our place on Saturday?"],
];

const REPLIES = [
  "Sounds good.",
  "Thanks, got it.",
  "Yes, that works for me.",
  "Let me check and get back to you.",
  "Not this time, sorry.",
  "Thanks for letting me know.",
];

/* BOT */

const bot: Bot<Mail> = ({ state, me, actors, draw }): Step => {
  const box = state.boxes[me] ?? {};
  const ids = Object.keys(box)
    .map(Number)
    .sort((a, b) => a - b);
  const inbox = ids.filter((id) => box[id]!.folder === "inbox");
  const unread = inbox.filter((id) => !box[id]!.read);
  if (unread.length) return { verb: "mark", args: { id: unread[0]!, read: true } };
  const owed = inbox.filter((id) => {
    const thread = Object.values(state.messages).filter((one) => one.thread === state.messages[id]!.thread);
    return thread.length < 4 && !thread.some((one) => one.from === me && one.id > id);
  });
  if (owed.length && below(draw, 3)) return { verb: "reply", args: { id: owed[0]!, text: pick(draw, REPLIES) } };
  if (inbox.length > 15) return { verb: "archive", args: { id: inbox[0]! } };
  const archived = ids.filter((id) => box[id]!.folder === "archive");
  if (archived.length > 30) return { verb: "delete", args: { id: archived[0]! } };
  const others = actors.filter((one) => one !== me);
  if (!others.length || below(draw, 2)) return null;
  const [subject, text] = pick(draw, LETTERS);
  const to = [...new Set([pick(draw, others), ...(below(draw, 4) ? [] : [pick(draw, others)])])];
  return { verb: "send", args: { to, subject, text } };
};

export default bot;
