import type { Block } from "../../sdk/blocks.ts";
import { BLOCKS, checked, TREE } from "../../sdk/blocks.ts";
import type { Json, Schema, Who } from "../../sdk/system.ts";
import { app, SLICE, walk } from "../../sdk/system.ts";
import bot from "./bot.ts";

/* TYPES */

export type Page = { title: string; blocks: Block[]; at: number };

export type Site = { published: boolean; pages: Record<string, Page> };

export type Sites = { sites: Record<string, Site> };

export const TITLE = 100;

export const SLUG = 40;

export const PAGES = 20;

export const ROOT = "sites";

const SLUGS = /^[a-z0-9]+(-[a-z0-9]+)*$/;

const EMPTY: Site = { published: false, pages: {} };

/* SITES */

export const pathOf = (user: string, slug?: string) => `${ROOT}/${user}${slug ? `/${slug}` : ""}`;

const shown = (user: string, site: Site): Json => ({
  path: pathOf(user),
  published: site.published,
  pages: Object.fromEntries(
    Object.entries(site.pages).map(([slug, page]) => [slug, { ...page, path: pathOf(user, slug) }]),
  ),
});

function read(state: Sites, path: string[], _who: Who | null): Json | undefined {
  const [head = "", ...rest] = path;
  if (!head) return { published: Object.keys(state.sites).filter((user) => state.sites[user]!.published) };
  if (head.startsWith(SLICE)) {
    const me = head.slice(SLICE.length);
    return walk(shown(me, state.sites[me] ?? EMPTY), rest);
  }
  const site = state.sites[head];
  if (!site?.published) return undefined;
  return walk(shown(head, site), rest);
}

/* SCHEMAS */

const shape = (properties: Record<string, Schema>): Schema => ({
  type: "object",
  properties,
  required: Object.keys(properties),
  additionalProperties: false,
});

const slug: Schema = { type: "string", minLength: 1, maxLength: SLUG };

/* APP */

export default app<Sites>({
  id: "sites",
  title: "Sites",
  line: `Your own tiny site: pages as block trees, published inside this world at ${ROOT}/<you>/<page>.`,
  boot: () => ({ sites: {} }),
  verbs: {
    page: {
      line: `Write a page of your site, by its slug (lowercase words joined by hyphens): a title and at most ${BLOCKS} blocks.`,
      args: shape({ slug, title: { type: "string", minLength: 1, maxLength: TITLE }, blocks: TREE }),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const site = state.sites[me] ?? EMPTY;
        const name = String(args.slug);
        if (!SLUGS.test(name))
          return ctx.refuse(`a slug is lowercase letters and digits joined by hyphens, like about-me, not ${name}`);
        if (!site.pages[name] && Object.keys(site.pages).length >= PAGES)
          return ctx.refuse(`your site holds ${PAGES} pages already; drop one first`);
        const page = { title: String(args.title), blocks: checked(args.blocks as Block[], ctx.refuse), at: ctx.at };
        return { sites: { ...state.sites, [me]: { ...site, pages: { ...site.pages, [name]: page } } } };
      },
    },
    drop: {
      line: "Drop a page of your site by its slug.",
      args: shape({ slug }),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const site = state.sites[me] ?? EMPTY;
        const name = String(args.slug);
        if (!site.pages[name]) return ctx.refuse(`your site has no page ${name}`);
        const { [name]: _, ...pages } = site.pages;
        return { sites: { ...state.sites, [me]: { ...site, pages } } };
      },
    },
    publish: {
      line: `Publish your site for every reader at ${ROOT}/<you>, or take it down with on: false.`,
      args: shape({ on: { type: "boolean" } }),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const site = state.sites[me] ?? EMPTY;
        const on = Boolean(args.on);
        if (on && !Object.keys(site.pages).length) return ctx.refuse("your site has no page yet; write one first");
        if (on === site.published)
          return ctx.refuse(on ? "your site is published already" : "your site is not published");
        return { sites: { ...state.sites, [me]: { ...site, published: on } } };
      },
    },
  },
  read,
  bot,
});
