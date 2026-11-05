import type { Org, People } from "@mrlyprod/apps/people/app.ts";
import { cards, glyph } from "@mrlyprod/apps/people/app.ts";
import { dialed } from "./types.ts";

const knob = dialed("people");

import { HUES } from "@mrlyprod/apps/shared/hues.ts";
import type { Obj } from "@mrlyprod/os/contract/index.ts";
import { below, pick } from "@mrlyprod/os/contract/index.ts";
import type { Bot, Step } from "./types.ts";

export const NAMES = ["Ada", "Alan", "Grace", "Hugo", "Iris", "Leo", "Maya", "Noor", "Omar", "Rosa", "Vera", "Yuki"];

export const KINDS = ["Bakery", "Garden", "Workshop", "Library", "School", "Market", "Studio", "Repair shop"];

const BIOS = ["I bake on weekends.", "New here, say hello.", "Looking for work.", "I fix things.", ""];

const STATUSES = ["At the market", "Busy today", "Free this evening", "Back soon", ""];

type Move = { state: People; me: string; draw: () => number; rules: Obj };

const renaming = (state: People, draw: () => number): Step => {
  const held = new Set(Object.values(cards(state)).map((card) => card.name.toLowerCase()));
  const free = NAMES.filter((name) => !held.has(name.toLowerCase()));
  return free.length ? { verb: "rename", args: { name: pick(draw, free) } } : null;
};

const presenting = (draw: () => number): Step =>
  below(draw, 2)
    ? { verb: "about", args: { bio: pick(draw, BIOS), status: pick(draw, STATUSES) } }
    : { verb: "look", args: { hue: pick(draw, HUES), glyph: glyph(draw()) } };

const answering = (person: string, full: boolean, draw: () => number): Step => {
  const roll = below(draw, 10);
  const verb = roll === 0 ? "block" : roll < 3 || full ? "unfriend" : "friend";
  return { verb, args: { person } };
};

const socialising = ({ state, me, draw, rules }: Move): Step => {
  const friends = state.friends[me] ?? [];
  const blocks = state.blocks[me] ?? [];
  const full = friends.length >= knob(rules, "friends");
  const others = Object.keys(cards(state)).filter((id) => id !== me);
  const asking = others.filter((id) => state.asks[id]?.includes(me));
  if (asking.length) return answering(pick(draw, asking), full, draw);
  if (blocks.length && !below(draw, 3)) return { verb: "unblock", args: { person: pick(draw, blocks) } };
  const mutes = state.mutes[me] ?? [];
  if (mutes.length && !below(draw, 3)) return { verb: "unmute", args: { person: pick(draw, mutes) } };
  const loud = others.filter((id) => !mutes.includes(id));
  if (loud.length && !below(draw, 12)) return { verb: "mute", args: { person: pick(draw, loud) } };
  if (friends.length && (full || !below(draw, 8))) return { verb: "unfriend", args: { person: pick(draw, friends) } };
  const strangers = others.filter(
    (id) =>
      !friends.includes(id) && !blocks.includes(id) && !state.blocks[id]?.includes(me) && !state.asks[me]?.includes(id),
  );
  return strangers.length ? { verb: "friend", args: { person: pick(draw, strangers) } } : null;
};

const roomy = (one: Org, rules: Obj) => Object.keys(one.members).length < knob(rules, "members");

const steering = (one: Org, { me, draw, rules }: Move, others: string[]): Step => {
  const org = one.n;
  const room = roomy(one, rules);
  if (one.asking.length) {
    const person = one.asking[0]!;
    return { verb: room && below(draw, 5) ? "admit" : "remove", args: { org, person } };
  }
  const roll = below(draw, 12);
  const outside = others.filter((id) => !(id in one.members) && !one.invited.includes(id));
  if (roll === 0 && room && outside.length) return { verb: "invite", args: { org, person: pick(draw, outside) } };
  const plain = Object.keys(one.members).filter((id) => one.members[id] === "member");
  if (roll === 1 && plain.length) return { verb: "remove", args: { org, person: pick(draw, plain) } };
  if (roll === 2 && plain.length && one.members[me] === "owner")
    return { verb: "role", args: { org, person: pick(draw, plain), role: "admin" } };
  return null;
};

const organising = (move: Move): Step => {
  const { state, me, draw, rules } = move;
  const live = Object.values(state.orgs).filter((one) => one.dissolved === null);
  const others = Object.keys(cards(state)).filter((id) => id !== me);
  const invited = live.filter((one) => one.invited.includes(me));
  if (invited.length) {
    const one = pick(draw, invited);
    return { verb: roomy(one, rules) && below(draw, 4) ? "join" : "leave", args: { org: one.n } };
  }
  const leads = live.filter((one) => ["owner", "admin"].includes(one.members[me] ?? ""));
  const steps = leads.map((one) => steering(one, move, others)).filter((one) => one !== null);
  if (steps.length) return pick(draw, steps);
  const mine = live.filter((one) => me in one.members);
  const outside = live.filter((one) => !(me in one.members) && !one.asking.includes(me) && roomy(one, rules));
  if (outside.length && (!mine.length ? below(draw, 3) : !below(draw, 4)))
    return { verb: "join", args: { org: pick(draw, outside).n } };
  if (mine.length && below(draw, 6)) return null;
  const crowded = mine.find((one) => Object.keys(one.members).length > 2 && one.members[me] === "member");
  if (crowded && mine.length > 1) return { verb: "leave", args: { org: crowded.n } };
  const human = cards(state)[me]?.kind === "human";
  if (mine.some((one) => one.by === me) || (!human && !knob(rules, "botsfound"))) return null;
  const names = new Set(live.map((one) => one.name));
  const free = KINDS.map((kind) => `${kind} ${me.split("-").pop()}`).filter((name) => !names.has(name));
  if (!free.length) return null;
  return {
    verb: "found",
    args: { name: pick(draw, free), kind: pick(draw, knob<string[]>(rules, "kinds")), open: below(draw, 3) > 0 },
  };
};

const bot: Bot<People> = ({ state, me, draw, world }): Step => {
  const move: Move = { state, me, draw, rules: world.rules };
  const roll = below(draw, 16);
  if (roll < 5) return socialising(move);
  if (roll < 14) return organising(move);
  return roll === 14 ? presenting(draw) : renaming(state, draw);
};

export default bot;
