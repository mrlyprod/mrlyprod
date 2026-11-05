import { below, pick } from "@mrlyprod/os/contract/index.ts";
import type { Canon } from "../apps/canon/app.ts";
import { dialed } from "./types.ts";

const knob = dialed("canon");

import { kinds, known, slug, things } from "../canon.ts";
import type { Bot, Step } from "./types.ts";

const AFTER: Record<string, string> = {
  book: "After",
  song: "Song for",
  film: "Return of",
  game: "Pocket",
  snack: "Spiced",
  meal: "Village",
  drink: "Iced",
  dessert: "Frozen",
  activity: "Night",
};

const bot: Bot<Canon> = ({ state, me, draw, world }): Step => {
  const { rules } = world;
  if (!knob(rules, "open") || below(draw, 3)) return null;
  if (Object.values(state.added).filter((one) => one.added === me).length >= knob(rules, "each")) return null;
  const here = kinds(rules).filter((kind) => kind in AFTER && things(rules, kind).length);
  if (!here.length) return null;
  const kind = pick(draw, here);
  const source = pick(draw, things(rules, kind));
  const name = `${AFTER[kind]} ${source.name}`.slice(0, knob(rules, "name")).trim();
  if (known(slug(name)) || slug(name) in state.added) return null;
  const about = `Made by ${me}, inspired by ${source.name}.`.slice(0, knob(rules, "about"));
  return { verb: "add", args: { kind, name, about } };
};

export default bot;
