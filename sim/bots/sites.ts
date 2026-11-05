import type { Block } from "@mrlyprod/apps/shared/blocks.ts";
import type { Sites } from "@mrlyprod/apps/sites/app.ts";
import { below, pick } from "@mrlyprod/os/contract/index.ts";
import type { Bot, Step } from "./types.ts";

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

const bot: Bot<Sites> = ({ state, me, draw }): Step => {
  const site = state.sites[me];
  const pages = Object.keys(site?.pages ?? {});
  const live = Object.keys(state.sites).filter((id) => id !== me && state.sites[id]!.published);
  if (live.length && below(draw, 2) === 0) {
    const id = pick(draw, live);
    const slugs = Object.keys(state.sites[id]!.pages);
    return below(draw, 2) === 0
      ? { verb: "visit", args: { site: id } }
      : { verb: "visit", args: { site: id, page: pick(draw, slugs) } };
  }
  if (pages.length && !site?.published && below(draw, 3) === 0) return { verb: "publish", args: { on: true } };
  if (site?.published && below(draw, 20) === 0) return { verb: "publish", args: { on: false } };
  if (pages.length > 3 && below(draw, 6) === 0) return { verb: "drop", args: { slug: pick(draw, pages) } };
  const slug = pick(draw, SLUGS);
  return { verb: "page", args: { slug, title: TITLES[slug]!, blocks: pick(draw, TREES) } };
};

export default bot;
