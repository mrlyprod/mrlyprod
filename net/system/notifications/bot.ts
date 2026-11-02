import type { Bot, Step } from "../../sdk/system.ts";
import { below } from "../../sdk/system.ts";
import type { Inbox } from "./app.ts";

/* BOT */

const bot: Bot<Inbox> = ({ state, me, draw }): Step => {
  const list = state.notices[me] ?? [];
  const unread = list.filter((one) => !one.read);
  const read = list.filter((one) => one.read);
  if (unread.length > 5) return { verb: "read", args: {} };
  if (unread.length) return { verb: "read", args: { id: unread[0]!.id } };
  if (read.length > 20) return { verb: "clear", args: {} };
  if (read.length && !below(draw, 4)) return { verb: "dismiss", args: { id: read[0]!.id } };
  return null;
};

export default bot;
