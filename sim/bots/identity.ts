import type { Card, Iden } from "@mrlyprod/apps/identity/app.ts";
import { FIELDS } from "@mrlyprod/apps/identity/app.ts";
import type { Photos } from "@mrlyprod/apps/photos/app.ts";
import type { Json, Obj } from "@mrlyprod/os/contract/index.ts";
import { dialed } from "./types.ts";

const knob = <T extends Json = string>(rules: Obj, name: string) => dialed("identity")<T>(rules, name);

import { below, pick } from "@mrlyprod/os/contract/index.ts";
import { things } from "../canon.ts";
import type { Bot, Seen, Step } from "./types.ts";

export const HANDLES = ["ada", "bay-leaf", "cook42", "dune", "eve", "fig", "gardener", "hours"];

const MODELS = ["claude-sonnet-5-5", "deepseek", "gpt", "gemini", "llama"];

const PLACES = ["Lisbon", "Osaka", "a small town", "the coast", "somewhere cold"];

const LIKES: Record<string, string[]> = {
  show: ["The Long Road", "Night Shift"],
  film: ["Blue Harbor", "The Last Train"],
  book: ["Salt and Stone", "A Year of Rain"],
  song: ["Morning Light", "Slow River"],
  activity: ["Walking", "Baking"],
  food: ["Toast", "Soup"],
};

const handled = ({ state, me, draw, world }: Seen<Iden>): Step => {
  const held = state.cards[me]?.handles ?? [];
  const taken = new Set(Object.values(state.cards).flatMap((card) => card.handles));
  const free = HANDLES.filter((handle) => !taken.has(handle));
  if (held.length && !below(draw, 6)) return { verb: "drop", args: { handle: pick(draw, held) } };
  if (!free.length || held.length >= knob<number>(world.rules, "handles") || (held.length && below(draw, 4)))
    return null;
  return { verb: "handle", args: { handle: pick(draw, free) } };
};

const names = ({ world }: Seen<Iden>, word: string): string[] => {
  const { rules } = world;
  if (knob(rules, "source") !== "canon" || !("canon" in world.apps)) return [];
  const groups = knob(rules, "groups") as Record<string, string[]>;
  const kinds = Object.hasOwn(groups, word) ? groups[word]! : [word];
  return kinds.flatMap((kind) => things(rules, kind).map((one) => one.name));
};

const birthday = ({ draw, world }: Seen<Iden>): Step => {
  if (knob<string>(world.rules, "birthday") === "off") return null;
  const two = (n: number) => String(n).padStart(2, "0");
  return {
    verb: "birthday",
    args: { date: `19${50 + below(draw, 50)}-${two(1 + below(draw, 12))}-${two(1 + below(draw, 28))}` },
  };
};

const birthplace = (seen: Seen<Iden>): Step => {
  const mode = knob(seen.world.rules, "birthplace");
  if (mode === "off") return null;
  const real = mode === "select" ? names(seen, "place") : [];
  return { verb: "birthplace", args: { place: pick(seen.draw, real.length ? real : PLACES) } };
};

const picture = ({ state, me, draw, world }: Seen<Iden>): Step => {
  if (knob<string>(world.rules, "picture") === "off") return null;
  const photos = (world.apps.photos as Photos | undefined)?.libraries[me]?.photos ?? [];
  if (state.cards[me]?.picture && !below(draw, 3)) return { verb: "picture", args: {} };
  return photos.length ? { verb: "picture", args: { photo: pick(draw, photos).id } } : null;
};

const model = ({ me, draw, world }: Seen<Iden>): Step => {
  const mode = knob(world.rules, "model");
  if (mode === "off") return null;
  const real = (world.rules["identity.models"] ?? {}) as Record<string, string>;
  if (mode === "claimed" || !Object.keys(real).length) return { verb: "model", args: { name: pick(draw, MODELS) } };
  return Object.hasOwn(real, me) ? { verb: "model", args: { name: real[me]! } } : null;
};

const favored = (seen: Seen<Iden>, card: Card | undefined): Step => {
  const { draw, world } = seen;
  const mode = knob(world.rules, "favorites");
  if (mode === "off") return null;
  const kind = pick(draw, Object.keys(LIKES));
  const held = card?.favorites?.[kind] ?? [];
  if (held.length && (!below(draw, 3) || held.length >= knob<number>(world.rules, "each")))
    return { verb: "unfavorite", args: { kind, thing: pick(draw, held) } };
  const real = names(seen, kind);
  if (mode === "select" && knob(world.rules, "source") && "canon" in world.apps && !real.length) return null;
  const fresh = (real.length && mode === "select" ? real : LIKES[kind]!).filter((one) => !held.includes(one));
  return fresh.length ? { verb: "favorite", args: { kind, thing: pick(draw, fresh) } } : null;
};

const kin = ({ state, me, actors, draw, world }: Seen<Iden>, card: Card | undefined): Step => {
  const parents = card?.parents ?? [];
  const asked = card?.asked ?? [];
  if (asked.length && !below(draw, 4)) return { verb: "unparent", args: { person: pick(draw, asked) } };
  if (parents.length && !below(draw, 8)) return { verb: "unparent", args: { person: pick(draw, parents) } };
  if (parents.length + asked.length >= knob<number>(world.rules, "parents")) return null;
  const open = actors.filter(
    (one) =>
      one !== me &&
      !parents.includes(one) &&
      !asked.includes(one) &&
      !(state.cards[one]?.parents ?? []).includes(me) &&
      !(state.cards[one]?.asked ?? []).includes(me),
  );
  return open.length ? { verb: "parent", args: { person: pick(draw, open) } } : null;
};

const privacy = ({ draw }: Seen<Iden>, card: Card | undefined): Step => {
  const hidden = card?.hidden ?? [];
  if (hidden.length && below(draw, 2)) return { verb: "show", args: { field: pick(draw, hidden) } };
  const open = FIELDS.filter((one) => !hidden.includes(one));
  return open.length ? { verb: "hide", args: { field: pick(draw, open) } } : null;
};

const bot: Bot<Iden> = (seen): Step => {
  const { state, me, draw } = seen;
  const card = state.cards[me];
  const asks = Object.keys(state.cards).filter((one) => state.cards[one]?.asked?.includes(me));
  if (asks.length && below(draw, 2)) return { verb: "parent", args: { person: pick(draw, asks) } };
  const steps = [
    () => handled(seen),
    () => birthday(seen),
    () => birthplace(seen),
    () => picture(seen),
    () => model(seen),
    () => favored(seen, card),
    () => favored(seen, card),
    () => kin(seen, card),
    () => privacy(seen, card),
  ];
  return pick(draw, steps)();
};

export default bot;
