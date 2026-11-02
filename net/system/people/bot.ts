import type { Bot, Step } from "../../sdk/system.ts";
import { below, pick } from "../../sdk/system.ts";
import type { People } from "./app.ts";
import { cards } from "./app.ts";

/* NAMES */

export const NAMES = ["Ada", "Alan", "Grace", "Hugo", "Iris", "Leo", "Maya", "Noor", "Omar", "Rosa", "Vera", "Yuki"];

export const KINDS = ["Bakery", "Garden", "Workshop", "Library", "School", "Market", "Studio", "Repair shop"];

/* POLICY */

const renaming = (state: People, draw: () => number): Step => {
  const held = new Set(Object.values(cards(state)).map((card) => card.name.toLowerCase()));
  const free = NAMES.filter((name) => !held.has(name.toLowerCase()));
  return free.length ? { verb: "rename", args: { name: pick(draw, free) } } : null;
};

const organising = (state: People, me: string, draw: () => number): Step => {
  const live = Object.values(state.orgs).filter((one) => one.dissolved === null);
  const leading = live.find((one) => one.asking.length && ["owner", "admin"].includes(one.members[me] ?? ""));
  if (leading) return { verb: "admit", args: { org: leading.n, person: leading.asking[0]! } };
  const mine = live.filter((one) => me in one.members);
  const others = live.filter((one) => !(me in one.members) && !one.asking.includes(me));
  if (others.length && (!mine.length ? below(draw, 3) : !below(draw, 4)))
    return { verb: "join", args: { org: pick(draw, others).n } };
  if (mine.length && below(draw, 6)) return null;
  const crowded = mine.find((one) => Object.keys(one.members).length > 2 && one.members[me] === "member");
  if (crowded && mine.length > 1) return { verb: "leave", args: { org: crowded.n } };
  if (mine.some((one) => one.by === me)) return null;
  const names = new Set(live.map((one) => one.name));
  const kinds = KINDS.map((kind) => `${kind} ${me.split("-").pop()}`).filter((name) => !names.has(name));
  return kinds.length ? { verb: "found", args: { name: pick(draw, kinds), open: below(draw, 3) > 0 } } : null;
};

const bot: Bot<People> = ({ state, me, draw }): Step =>
  below(draw, 8) ? organising(state, me, draw) : renaming(state, draw);

export default bot;
