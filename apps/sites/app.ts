import type { Ctx, Json, Knob, Knobs, Obj, Schema, Who } from "@mrlyprod/os/contract/index.ts";
import { app, sliced } from "@mrlyprod/os/contract/index.ts";
import type { Block, Limits } from "../shared/blocks.ts";
import { checked, TREE_KNOBS, tree } from "../shared/blocks.ts";
import { blockedBy, notice } from "../shared/people.ts";

export type Page = { title: string; blocks: Block[]; at: number; views: number };

export type Visit = { who: string; page: string; at: number };

export type Site = { published: boolean; views: number; recent: Visit[]; pages: Record<string, Page> };

export type Sites = {
  listed: number;
  visiting: Record<string, { site: string; page: string }>;
  sites: Record<string, Site>;
};

export const ROOT = "sites";

const ID = 40;

const KNOBS = {
  ...TREE_KNOBS,
  title: {
    line: "Characters one page title holds.",
    schema: { type: "integer", minimum: 10, maximum: 1000 },
    reality: 100,
  },
  slug: { line: "Characters one page slug holds.", schema: { type: "integer", minimum: 8, maximum: 200 }, reality: 40 },
  pages: { line: "Pages one site holds.", schema: { type: "integer", minimum: 1, maximum: 500 }, reality: 20 },
  recent: {
    line: "Recent visits one site remembers.",
    schema: { type: "integer", minimum: 1, maximum: 500 },
    reality: 20,
  },
  listed: {
    line: "Sites the overview lists to discover.",
    schema: { type: "integer", minimum: 1, maximum: 500 },
    reality: 20,
  },
} satisfies Knobs;

const limits = (knob: Knob): Limits => ({
  blocks: knob("blocks"),
  text: knob("text"),
  depth: knob("depth"),
});

const SLUGS = /^[a-z0-9]+(-[a-z0-9]+)*$/;

const EMPTY: Site = { published: false, views: 0, recent: [], pages: {} };

export const pathOf = (user: string, slug?: string) => `${ROOT}/${user}${slug ? `/pages/${slug}` : ""}`;

const has = (record: object, key: string) => Object.hasOwn(record, key);

const siteOf = (state: Sites, id: string): Site => (has(state.sites, id) ? state.sites[id]! : EMPTY);

const latest = (site: Site) => Math.max(0, ...Object.values(site.pages).map((page) => page.at));

const front = (site: Site) => (site.pages.home ? "home" : Object.keys(site.pages)[0]!);

const shown = (user: string, site: Site): Json => ({
  path: pathOf(user),
  published: site.published,
  views: site.views,
  pages: Object.fromEntries(
    Object.entries(site.pages).map(([slug, page]) => [slug, { ...page, path: pathOf(user, slug) }]),
  ),
});

const card = (user: string, site: Site): Json => ({
  path: pathOf(user),
  visit: { site: user },
  title: site.pages[front(site)]!.title,
  views: site.views,
  at: latest(site),
  pages: Object.fromEntries(
    Object.entries(site.pages).map(([slug, page]) => [slug, { title: page.title, views: page.views }]),
  ),
});

const live = (state: Sites) =>
  Object.entries(state.sites)
    .filter(([, site]) => site.published && Object.keys(site.pages).length)
    .sort(([, a], [, b]) => latest(b) - latest(a));

const discover = (state: Sites): Json =>
  Object.fromEntries(
    live(state)
      .slice(0, state.listed)
      .map(([user, site]) => [user, card(user, site)]),
  );

const visiting = (state: Sites, me: string): Json => {
  const last = state.visiting[me];
  const page = last && siteOf(state, last.site).published ? siteOf(state, last.site).pages[last.page] : undefined;
  return last && page
    ? { site: last.site, page: last.page, path: pathOf(last.site, last.page), title: page.title, blocks: page.blocks }
    : null;
};

const mine = (state: Sites, me: string): Json => {
  const site = siteOf(state, me);
  return {
    path: pathOf(me),
    published: site.published,
    views: site.views,
    recent: [...site.recent].reverse(),
    pages: Object.fromEntries(
      Object.entries(site.pages).map(([slug, page]) => [
        slug,
        { title: page.title, views: page.views, at: page.at, path: pathOf(me, slug) },
      ]),
    ),
  };
};

const slice = (state: Sites, me: string): Json => ({
  ...(shown(me, siteOf(state, me)) as Obj),
  recent: [...siteOf(state, me).recent].reverse(),
  visiting: visiting(state, me),
  discover: discover(state),
});

const whole = (state: Sites, who: Who | null): Json => ({
  ...Object.fromEntries(live(state).map(([user, site]) => [user, shown(user, site)])),
  ...(who ? { me: slice(state, who.id), visiting: visiting(state, who.id) } : {}),
  discover: discover(state),
});

const deep = sliced<Sites>(slice, whole);

const overview = (state: Sites, who: Who | null): Json => ({
  ...(who ? { me: mine(state, who.id), visiting: visiting(state, who.id) } : {}),
  discover: discover(state),
});

const read = (state: Sites, path: string[], who: Who | null, at = 0): Json | undefined =>
  path.length ? deep(state, path, who, at) : overview(state, who);

const shape = (properties: Record<string, Schema>, optional: string[] = []): Schema => ({
  type: "object",
  properties,
  required: Object.keys(properties).filter((key) => !optional.includes(key)),
  additionalProperties: false,
});

const slug = (knob: Knob): Schema => ({ type: "string", minLength: 1, maxLength: knob("slug") });

const put = (state: Sites, me: string, site: Site): Sites => ({ ...state, sites: { ...state.sites, [me]: site } });

const counted = (state: Sites, id: string, site: Site, page: string, ctx: Ctx): Sites => {
  const me = ctx.who.id;
  const one = site.pages[page]!;
  if (!site.recent.some((visit) => visit.who === me)) ctx.wish(notice(id, `${me} visited your site: ${one.title}`));
  return put(state, id, {
    ...site,
    views: site.views + 1,
    recent: [...site.recent, { who: me, page, at: ctx.at }].slice(-ctx.knob("recent")),
    pages: { ...site.pages, [page]: { ...one, views: one.views + 1 } },
  });
};

export default app<Sites>({
  id: "sites",
  title: "Sites",
  line: `Your own tiny site: pages as block trees, private until you publish it at ${ROOT}/<you>. The overview's discover lists sites to visit, and shows you who visited yours.`,
  boot: ({ knob }) => ({ listed: knob("listed"), visiting: {}, sites: {} }),
  verbs: {
    page: {
      line: (knob) =>
        `Write a page of your site, by its slug (lowercase words joined by hyphens): a title and at most ${knob("blocks")} blocks.`,
      args: (knob) =>
        shape({
          slug: slug(knob),
          title: { type: "string", minLength: 1, maxLength: knob("title") },
          blocks: tree(limits(knob)),
        }),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const site = siteOf(state, me);
        const name = String(args.slug);
        const most = ctx.knob("pages");
        if (!SLUGS.test(name))
          return ctx.refuse(`a slug is lowercase letters and digits joined by hyphens, like about-me, not ${name}`);
        const had = has(site.pages, name);
        if (!had && Object.keys(site.pages).length >= most)
          return ctx.refuse(`your site holds ${most} pages already; drop one first`);
        const page = {
          title: String(args.title),
          blocks: checked(args.blocks as Block[], ctx.knob("blocks"), ctx.refuse),
          at: ctx.at,
          views: had ? site.pages[name]!.views : 0,
        };
        ctx.say({
          note: `${had ? "Changed" : "Wrote"} page ${name}${site.published ? "" : "; your site is not published yet"}`,
          slug: name,
          path: pathOf(me, name),
        });
        return put(state, me, { ...site, pages: { ...site.pages, [name]: page } });
      },
    },
    drop: {
      line: "Drop a page of your site by its slug.",
      args: (knob) => shape({ slug: slug(knob) }),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const site = siteOf(state, me);
        const name = String(args.slug);
        if (!has(site.pages, name)) return ctx.refuse(`your site has no page ${name}`);
        const { [name]: _, ...pages } = site.pages;
        const empty = !Object.keys(pages).length;
        ctx.say({ note: `Dropped page ${name}`, slug: name });
        return put(state, me, { ...site, pages, published: site.published && !empty });
      },
    },
    publish: {
      line: `Publish your site for every reader at ${ROOT}/<you>, or take it down with on: false.`,
      args: shape({ on: { type: "boolean" } }),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const site = siteOf(state, me);
        const on = Boolean(args.on);
        if (on && !Object.keys(site.pages).length) return ctx.refuse("your site has no page yet; write one first");
        if (on === site.published)
          return ctx.refuse(on ? "your site is published already" : "your site is not published");
        ctx.say({ note: on ? `Your site is live at ${pathOf(me)}` : "Your site is down", path: pathOf(me) });
        return put(state, me, { ...site, published: on });
      },
    },
    visit: {
      line: `Visit someone's published site, at a page or its front: the page lands in your ${ROOT}/visiting, it counts a view, and the owner sees you among their recent visitors and hears of your first visit.`,
      args: (knob) => shape({ site: { type: "string", minLength: 1, maxLength: ID }, page: slug(knob) }, ["page"]),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const id = String(args.site);
        const site = siteOf(state, id);
        if (id === me) return ctx.refuse(`that is your own site; read ${ROOT}/me to see it`);
        if (!site.published) return ctx.refuse(`${id} has no published site; ${ROOT} lists those that are`);
        if (blockedBy(ctx, id, me)) return ctx.refuse(`${id} is not taking this from you`);
        const page = args.page === undefined ? front(site) : String(args.page);
        if (!has(site.pages, page))
          return ctx.refuse(`${id}'s site has no page ${page}; it has ${Object.keys(site.pages).join(", ")}`);
        const kept = { ...state.visiting, [me]: { site: id, page } };
        ctx.say({
          note: `Visited ${id}: ${site.pages[page]!.title}; read ${ROOT}/visiting for the page`,
          site: id,
          page,
        });
        return counted({ ...state, visiting: kept }, id, site, page, ctx);
      },
    },
  },
  read,
  knobs: KNOBS,
});
