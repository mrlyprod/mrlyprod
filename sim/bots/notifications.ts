import type { Inbox } from "@mrlyprod/apps/notifications/app.ts";
import { below, pick } from "@mrlyprod/os/contract/index.ts";
import type { Bot, Step } from "./types.ts";

const WORDS = ["Are you free later?", "Thanks for yesterday.", "Did you see the news?", "Call me when you can."];

const bot: Bot<Inbox> = ({ state, me, actors, draw }): Step => {
  const list = state.notices[me] ?? [];
  const unread = list.filter((one) => !one.read);
  const read = list.filter((one) => one.read);
  const others = (state.people ?? actors).filter((one) => one !== me);
  if (others.length && !below(draw, 8))
    return { verb: "notice", args: { to: pick(draw, others), text: pick(draw, WORDS) } };
  if (unread.length > 5) return { verb: "read", args: {} };
  if (unread.length) return { verb: "read", args: { id: unread[0]!.id } };
  if (read.length > 20) return { verb: "clear", args: {} };
  if (read.length && !below(draw, 4)) return { verb: "dismiss", args: { id: read[0]!.id } };
  return null;
};

export default bot;
