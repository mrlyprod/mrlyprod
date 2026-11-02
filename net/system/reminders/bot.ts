import type { Bot, Step } from "../../sdk/system.ts";
import { below, pick } from "../../sdk/system.ts";
import type { Todo } from "./app.ts";

/* WORDS */

const ITEMS: Record<string, string[]> = {
  Groceries: ["Milk", "Eggs", "Bread", "Apples", "Coffee", "Rice", "Olive oil"],
  Errands: ["Post office", "Pick up the dry cleaning", "Return library books", "Pay the electricity bill"],
  Work: ["Reply to emails", "Prepare the slides", "Review the budget", "Book the meeting room"],
  Home: ["Water the plants", "Clean the kitchen", "Fix the shelf", "Take out the recycling"],
};

/* BOT */

const bot: Bot<Todo> = ({ state, me, actors, draw }): Step => {
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
  if (open.length && below(draw, 2)) return { verb: "tick", args: { item: pick(draw, open).id, done: true } };
  if (list.items.length < 12)
    return { verb: "add", args: { list: list.id, text: pick(draw, ITEMS[list.title] ?? ITEMS.Home!) } };
  return null;
};

export default bot;
