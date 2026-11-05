import type { Mail } from "@mrlyprod/apps/mail/app.ts";
import { dialed } from "./types.ts";

const knob = dialed("mail");

import { below, pick } from "@mrlyprod/os/contract/index.ts";
import type { Bot, Step } from "./types.ts";

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

const bot: Bot<Mail> = ({ state, me, actors, draw, world }): Step => {
  const size = knob(world.rules, "text");
  const box = state.boxes[me] ?? {};
  const ids = Object.keys(box)
    .map(Number)
    .sort((a, b) => a - b);
  const inbox = ids.filter((id) => box[id]!.folder === "inbox");
  const unread = inbox.filter((id) => !box[id]!.read);
  if (unread.length)
    return below(draw, 2)
      ? { verb: "open", args: { id: unread[0]! } }
      : { verb: "mark", args: { id: unread[0]!, read: true } };
  const others = actors.filter((one) => one !== me);
  const owed = inbox.filter((id) => {
    const thread = Object.values(state.messages).filter((one) => one.thread === state.messages[id]!.thread);
    return thread.length < 4 && !thread.some((one) => one.from === me && one.id > id);
  });
  if (owed.length && below(draw, 3)) {
    const id = owed[0]!;
    const text = pick(draw, REPLIES).slice(0, size);
    const many = state.messages[id]!.to.length > 1;
    if (many && below(draw, 2)) return { verb: "reply", args: { id, text, all: true } };
    if (others.length && !below(draw, 5)) return { verb: "reply", args: { id, text, add: [pick(draw, others)] } };
    return { verb: "reply", args: { id, text } };
  }
  if (inbox.length && others.length && !below(draw, 6))
    return {
      verb: "forward",
      args: { id: pick(draw, inbox), to: [pick(draw, others)], text: "Thought you should see this.".slice(0, size) },
    };
  if (inbox.length > 15) return { verb: "archive", args: { id: inbox[0]! } };
  const archived = ids.filter((id) => box[id]!.folder === "archive");
  if (archived.length > 30) return { verb: "delete", args: { id: archived[0]! } };
  if (!others.length || below(draw, 2)) return null;
  const [subject, text] = pick(draw, LETTERS);
  const cap = knob(world.rules, "to");
  const to = [...new Set([pick(draw, others), ...(below(draw, 4) ? [] : [pick(draw, others)])])];
  return {
    verb: "send",
    args: { to: to.slice(0, cap), subject: subject.slice(0, knob(world.rules, "subject")), text: text.slice(0, size) },
  };
};

export default bot;
