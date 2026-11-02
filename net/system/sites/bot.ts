import type { Block } from "../../sdk/blocks.ts";
import type { Bot, Step } from "../../sdk/system.ts";
import { below, pick } from "../../sdk/system.ts";
import type { Sites } from "./app.ts";

/* WORDS */

export const SLUGS = ["home", "about", "recipes", "garden", "notes"];

const TITLES: Record<string, string> = {
  home: "Welcome",
  about: "About me",
  recipes: "Recipes",
  garden: "The garden",
  notes: "Notes",
};

const TREES: Block[][] = [
  [
    { kind: "h1", text: "Welcome" },
    { kind: "p", text: "A small site inside a small world." },
  ],
  [
    { kind: "h1", text: "Lentil soup" },
    { kind: "li", text: "lentils" },
    { kind: "li", text: "onion" },
  ],
  [
    { kind: "h2", text: "Tuesdays" },
    { kind: "p", text: "Water the tomatoes." },
  ],
];

/* BOT */

const bot: Bot<Sites> = ({ state, me, draw }): Step => {
  const site = state.sites[me];
  const pages = Object.keys(site?.pages ?? {});
  if (pages.length && !site?.published && below(draw, 3) === 0) return { verb: "publish", args: { on: true } };
  if (pages.length > 3 && below(draw, 6) === 0) return { verb: "drop", args: { slug: pick(draw, pages) } };
  const slug = pick(draw, SLUGS);
  return { verb: "page", args: { slug, title: TITLES[slug]!, blocks: pick(draw, TREES) } };
};

export default bot;
