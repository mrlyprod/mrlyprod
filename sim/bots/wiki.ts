import type { Article, Wiki } from "@mrlyprod/apps/wiki/app.ts";
import { slugOf } from "@mrlyprod/apps/wiki/app.ts";
import { dialed } from "./types.ts";

const knob = dialed("wiki");

import { below, pick } from "@mrlyprod/os/contract/index.ts";
import type { Bot, Step } from "./types.ts";

export const TOPICS: [string, string][] = [
  ["Salt", "Salt is mined from the hills and sold by the sack at the market."],
  ["Clocks", "A clock shows the world time; every act carries the time it happened at."],
  ["Rivers", "A river is a line of water cells running downhill to the sea."],
  ["Lamps", "Lamps light the streets after the world day's twentieth hour."],
  ["Rope", "Rope is twisted from fibre and sold by the metre."],
  ["Wheels", "A wheel turns a cart; carts carry goods between pinned places."],
  ["Glass", "Glass is sand melted and cooled; windows and bottles are made of it."],
  ["Honey", "Honey keeps for years and sweetens bread."],
  ["Paper", "Paper is pressed from pulp; books, letters and news are printed on it."],
  ["Wells", "A well reaches water under a grass or sand cell."],
];

const LINES = [
  "It is counted in whole numbers.",
  "People pin it on the map.",
  "The weather changes it a little each day.",
  "It is sold at the market.",
  "Every world has one.",
  "See also the calendar.",
];

const TALKS = [
  "Needs a source.",
  "Merged the two paragraphs.",
  "Is this the same as the market page?",
  "Added a line on prices.",
  "Looks good to me.",
];

const reverted = (draw: () => number, one: Article): Step => {
  const older = one.revisions.filter((rev) => rev.n !== one.n && rev.text !== one.text);
  return older.length ? { verb: "revert", args: { slug: one.slug, to: pick(draw, older).n } } : null;
};

const bot: Bot<Wiki> = ({ state, me, draw, world }): Step => {
  const slugs = Object.keys(state.articles);
  const missing = TOPICS.filter(([title]) => !state.articles[slugOf(title)]);
  if (missing.length && !below(draw, 4)) {
    const [title, text] = pick(draw, missing);
    return { verb: "write", args: { title, text } };
  }
  if (!slugs.length) return null;
  const slug = pick(draw, slugs);
  const one = state.articles[slug]!;
  const turn = below(draw, 11);
  if (turn < 5) {
    const text = `${one.text} ${pick(draw, LINES)}`;
    return text.length <= knob(world.rules, "text") ? { verb: "edit", args: { slug, text } } : null;
  }
  if (turn < 8) return { verb: "talk", args: { slug, text: pick(draw, TALKS) } };
  if (turn === 8) return reverted(draw, one);
  const watching = state.watching[me] ?? [];
  if (!watching.includes(slug) && watching.length < knob(world.rules, "watch"))
    return { verb: "watch", args: { slug } };
  return watching.length > 3 ? { verb: "unwatch", args: { slug } } : null;
};

export default bot;
