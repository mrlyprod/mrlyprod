import type { Todo } from "@mrlyprod/apps/reminders/app.ts";
import { below, pick } from "@mrlyprod/os/contract/index.ts";
import type { Bot, Step } from "./types.ts";

const ITEMS: Record<string, string[]> = {
  Groceries: ["Milk", "Eggs", "Bread", "Apples", "Coffee", "Rice", "Olive oil"],
  Errands: ["Post office", "Pick up the dry cleaning", "Return library books", "Pay the electricity bill"],
  Work: ["Reply to emails", "Prepare the slides", "Review the budget", "Book the meeting room"],
  Home: ["Water the plants", "Clean the kitchen", "Fix the shelf", "Take out the recycling"],
};

const MINUTE = 60_000;

const bot: Bot<Todo> = ({ state, me, actors, at, draw }): Step => {
  const lists = Object.values(state.lists).filter((list) => list.owner === me || list.shared.includes(me));
  const own = lists.filter((list) => list.owner === me);
  const fresh = Object.keys(ITEMS).filter((title) => !own.some((list) => list.title === title));
  if (fresh.length && (!own.length || (own.length < 3 && !below(draw, 10))))
    return { verb: "make", args: { title: pick(draw, fresh) } };
  if (!lists.length) return null;
  const list = pick(draw, lists);
  const open = list.items.filter((item) => !item.done);
  const done = list.items.filter((item) => item.done);
  if (done.length > 5) return { verb: "remove", args: { item: done[0]!.id } };
  const others = actors.filter((one) => one !== me && !list.shared.includes(one));
  if (list.owner === me && others.length && !below(draw, 8))
    return { verb: "share", args: { list: list.id, with: pick(draw, others) } };
  if (list.owner !== me && !below(draw, 30)) return { verb: "unshare", args: { list: list.id, with: me } };
  if (list.owner === me && list.items.length >= 12 && !below(draw, 10))
    return { verb: "delete", args: { list: list.id } };
  if (open.length && below(draw, 2)) return { verb: "tick", args: { item: pick(draw, open).id } };
  if (list.items.length < 12) {
    const text = pick(draw, ITEMS[list.title] ?? ITEMS.Home!);
    const plain = { list: list.id, text };
    return { verb: "add", args: below(draw, 3) ? plain : { ...plain, due: at + (1 + below(draw, 120)) * MINUTE } };
  }
  return null;
};

export default bot;
