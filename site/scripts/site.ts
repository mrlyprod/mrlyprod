import { existsSync, readFileSync, readdirSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import { createElement as h } from "react";
import { renderToStaticMarkup } from "react-dom/server";
import katex from "katex";
import { build, bytes, jsonScript, jsonText, walk, type Node, type Output, type Route, type Site, type Spec } from "../kit/ssg/build.ts";
import { config as gitConfig, isGit } from "../kit/git/git.ts";
import { resolve as resolveLink } from "../kit/ssg/links.ts";
import { themed } from "../kit/ssg/pic.ts";
import { escape, front, inline, plain, render as md, summary, title } from "../kit/ssg/md.ts";
import { posts as parsed, type Leaf as Posted } from "../kit/ssg/blog.ts";
import { sidebar, tree } from "../lib/tree.js";
import { Glyph, Grid, Menu, Shell } from "../ui/chrome.jsx";
import { claimsScript, headScript, inlineScripts, tintCss } from "../ui/config.js";
import { grid as glyphs, logoSvg } from "../ui/logo.js";
import SITE from "../lib/site.js";
import { shelf } from "./shelf.ts";

const org = resolve(import.meta.dir, "..");
const dist = process.env.MRLY_DIST ? resolve(process.env.MRLY_DIST) : join(org, "dist");
const root = (process.env.MRLY_SITE ?? SITE.root).replace(/\/$/, "");
const AUTHOR = "MrlyProd";
const LIST = /^- \[([^\]]+)\]\([^)]*\) - (.+)$/gm;
const HEADING = /<h([23]) id="([^"]+)">(.*?)<\/h\1>/g;
const AVATAR = /^(!\[avatar\]\(figures\/avatar\.png\)|<picture>.*?figures\/avatar-light\.png.*?<\/picture>)\n?/m;
const WORD = renderToStaticMarkup(h(Glyph, { text: SITE.title.toUpperCase() }));
const BOOT = headScript(SITE.prefix);
const TINT = `<style>${tintCss(SITE.tint)}</style>`;
const ICONS = [
  `<link rel="icon" href="/favicon.svg" type="image/svg+xml">`,
  `<link rel="icon" href="/favicon.png" type="image/png" sizes="40x40">`,
  `<link rel="apple-touch-icon" href="/apple-touch-icon.png">`,
  `<link rel="manifest" href="/manifest.webmanifest">`,
].join("\n");

const read = (p: string) => readFileSync(p, "utf8");
const math = (tex: string, display: boolean) =>
  katex.renderToString(tex, { output: "mathml", throwOnError: false, displayMode: display });
const untag = (html: string) =>
  html.replace(/<[^>]+>/g, "").replace(/&amp;/g, "&").replace(/&lt;/g, "<").replace(/&gt;/g, ">").replace(/&quot;/g, '"');
const brand = (name: string) => (name === SITE.title ? name : `${name} · ${SITE.title}`);

/* LINKS */

const NAME = /^[a-z0-9-]+$/;

function links(site: Site, from: string, out?: Output[]) {
  const home = site.input("figures").path;
  return (url: string) => {
    if (out && NAME.test(url) && existsSync(join(home, `${url}.png`))) {
      const ext = existsSync(join(home, `${url}.webp`)) ? "webp" : "png";
      const path = `figures/${url}.${ext}`;
      if (!out.some((item) => item.path === path)) out.push({ path, bytes: bytes(join(home, `${url}.${ext}`)) });
      return `/${path}`;
    }
    return resolveLink(site, from, url);
  };
}

/* FIGURES */

const SIDES = ["dark", "light"] as const;

function figure(home: string, name: string, route: string, ext = "png") {
  const file = join(home, `${name}.${ext}`);
  if (!existsSync(file)) throw new Error(`site: ${name}.${ext} missing from ${relative(org, home)} for ${route}; draw it with bun run figures`);
  return file;
}

function press(site: Site, out: Output[]) {
  const home = site.input("figures").path;
  const keep = (path: string, file: string) => {
    if (!out.some((item) => item.path === path)) out.push({ path, bytes: bytes(file) });
  };
  return (name: string, route: string) => {
    const pair = { dark: "", light: "" };
    for (const side of SIDES) {
      const path = `figures/${name}-${side}.webp`;
      keep(path, figure(home, `${name}-${side}`, route, "webp"));
      pair[side] = `/${path}`;
    }
    keep(`figures/${name}-dark.png`, figure(home, `${name}-dark`, route));
    return pair;
  };
}

type Fig = ReturnType<typeof press>;

const pic = (fig: Fig, name: string, route: string, alt: string, extra = "", cls = "") => themed(fig(name, route), alt, cls, extra);

const hero = (fig: Fig, name: string, route: string, alt: string) =>
  `<figure class="opener">${pic(fig, name, route, alt)}</figure>`;

const grid = (nodes: Node[]) => renderToStaticMarkup(h(Grid, { nodes }));

/* HEAD */

type Picture = { url: string; width: number; height: number };

const OG: Picture = { url: `${root}/og.png`, width: 1200, height: 630 };

function picture(site: Site, name: string, route: string, fig?: Fig): Picture {
  const file = figure(site.input("figures").path, `${name}-dark`, route);
  if (fig) fig(name, route);
  const png = bytes(file);
  const view = new DataView(png.buffer, png.byteOffset, png.byteLength);
  return { url: `${root}/figures/${name}-dark.png`, width: view.getUint32(16), height: view.getUint32(20) };
}

function meta(route: string, name: string, description: string, type: string, image = OG) {
  const url = root + route;
  return [
    `<link rel="canonical" href="${url}">`,
    `<meta name="description" content="${escape(description)}">`,
    `<meta property="og:title" content="${escape(name)}">`,
    `<meta property="og:description" content="${escape(description)}">`,
    `<meta property="og:url" content="${url}">`,
    `<meta property="og:type" content="${type}">`,
    `<meta property="og:site_name" content="${escape(SITE.title)}">`,
    `<meta property="og:image" content="${image.url}">`,
    `<meta property="og:image:width" content="${image.width}">`,
    `<meta property="og:image:height" content="${image.height}">`,
    `<meta name="twitter:card" content="summary_large_image">`,
    `<meta name="twitter:image" content="${image.url}">`,
    ICONS,
  ].join("\n");
}

function headings(body: string) {
  return [...body.matchAll(HEADING)].map((m) => ({ level: Number(m[1]), id: m[2], text: untag(m[3]) }));
}

type Leaf = {
  route: string;
  name: string;
  description: string;
  body: string;
  type?: string;
  wide?: boolean;
  bare?: boolean;
  code?: boolean;
  data?: object;
  tree?: Node[];
  image?: Picture;
  scripts?: string[];
};

function shell(site: Site, leaf: Leaf) {
  const { route, name, description, body, type = "article", wide = false, bare = false, code = false, data } = leaf;
  const article = h(bare ? "div" : "article", { className: bare ? undefined : "prose", dangerouslySetInnerHTML: { __html: body } });
  const main = renderToStaticMarkup(h(Shell, { route, tree: leaf.tree ?? site.nav, contents: headings(body), wide }, article));
  const ld = data ? `${jsonScript(data)}\n` : "";
  const more = (leaf.scripts ?? []).map((src) => `\n<script type="module" src="${src}"></script>`).join("");
  const image = leaf.image ?? (code ? picture(site, "site-code", route) : OG);
  return `<!doctype html>
<html lang="en" data-prefix="${SITE.prefix}">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
${BOOT}
<title>${escape(brand(name))}</title>
${meta(route, name, description, type, image)}
<link rel="stylesheet" href="${site.asset("palette.css")}">
<link rel="stylesheet" href="${site.asset("tokens.css")}">
<link rel="stylesheet" href="${site.asset("base.css")}">
<link rel="stylesheet" href="${site.asset("chrome.css")}">
<link rel="stylesheet" href="${site.asset("fonts/fonts.css")}">
<link rel="stylesheet" href="/pages.css">
${TINT}
${code ? `<link rel="stylesheet" href="${site.asset("contract.css")}">\n<link rel="stylesheet" href="${site.asset("code.css")}">\n<link rel="stylesheet" href="${site.asset("seti/seti.css")}">\n` : ""}${ld}<script type="module" src="${site.asset("chrome.js")}"></script>${more}
</head>
<body>
${main}
</body>
</html>
`;
}

/* DEMOS */

type Card = { name: string; title: string; blurb: string; shelf: string; order: number; reads: Read[] };

type Read = { name: string; href: string };

type Shelf = { key: string; group: string; title: string; blurb: string };

type Bay = Node & { key: string };

const SHELVES = (SITE.shelves ?? []) as Shelf[];

const TITLE = /<title>([^<]*)<\/title>/;

const OWN = /[ \t]*<meta name="?(?:description|shelf|order)"?[^>]*>\n?/g;

const tag = (html: string, name: string) => {
  const found = html.match(new RegExp(`<meta name="${name}" content="([^"]*)">`));
  return found ? untag(found[1]) : "";
};

const demoHome = (site: Site, name: string) =>
  name ? join(site.input("demos").path, name) : dirname(site.input("demos").path);

export const demoShell = (site: Site, name: string) => join(demoHome(site, name), "index.html");

function demoNames(site: Site) {
  const home = site.input("demos").path;
  return [
    "",
    ...site
      .input("demos")
      .files.filter((f) => f.endsWith("/index.html"))
      .map((f) => dirname(f).slice(home.length + 1))
      .sort((a, b) => a.localeCompare(b)),
  ];
}

const demoRoute = (name: string) => (name ? `/demos/${name}/` : "/demos/");

function cards(site: Site): Card[] {
  return demoNames(site).map((name) => {
    const html = read(demoShell(site, name));
    const found = html.match(TITLE);
    return {
      name,
      title: found ? untag(found[1]) : name,
      blurb: tag(html, "description"),
      shelf: tag(html, "shelf"),
      order: Number(tag(html, "order")) || 0,
      reads: [],
    };
  });
}

const MENTION = (name: string) => new RegExp(`demos/${name}/`);

function readers(list: Card[], pages: { name: string; href: string; md: string }[]) {
  for (const card of list) {
    if (!card.name) continue;
    const seen = MENTION(card.name);
    card.reads = pages.filter((p) => seen.test(p.md)).map((p) => ({ name: p.name, href: p.href }));
  }
}

function shelved(list: Card[]): Bay[] {
  return SHELVES.map((one) => ({
    key: one.key,
    name: one.title,
    nodes: list
      .filter((d) => d.shelf === one.key)
      .sort((a, b) => a.order - b.order || a.title.localeCompare(b.title))
      .map((d) => ({ name: d.title, href: demoRoute(d.name), text: d.blurb })),
  })).filter((one) => one.nodes.length);
}

export const demoTree = (site: Site) => shelved(cards(site));

function demoGroup(site: Site): Route {
  const list = cards(site);
  const gallery = [demoShell(site, ""), join(demoHome(site, ""), "index.jsx")];
  const inputs = [...gallery, ...["demos", "lib", "pkg", "ui"].flatMap((one) => site.input(one).files)];
  return {
    route: "/demos/",
    kind: "demos",
    name: "Demos",
    data: list,
    source: site.input("demos").path,
    inputs,
    urls: list.map((d) => ({ route: demoRoute(d.name), name: d.title, source: demoHome(site, d.name) })),
  };
}

const IMPORTS = /\bimport\s*["']([^"']+)["']|\bfrom\s*["']([^"']+)["']/g;

const MODULE = /<script[^>]*type="module"[^>]*>/;

const SRC = /src="([^"]+)"/;

async function imports(outputs: { path: string; text: () => Promise<string> }[]): Promise<Map<string, string[]>> {
  const trim = (path: string) => path.replace(/^\.\//, "");
  const known = new Set(outputs.map((item) => trim(item.path)));
  const edges = new Map<string, string[]>();
  for (const item of outputs) {
    const path = trim(item.path);
    if (!path.endsWith(".js")) continue;
    const deps: string[] = [];
    for (const [, bare, named] of (await item.text()).matchAll(IMPORTS)) {
      const dep = join(dirname(path), bare ?? named);
      if (known.has(dep) && !deps.includes(dep)) deps.push(dep);
    }
    edges.set(path, deps);
  }
  return edges;
}

function closure(edges: Map<string, string[]>, entry: string): string[] {
  const seen = new Set<string>();
  const queue = [...(edges.get(entry) ?? [])];
  while (queue.length) {
    const next = queue.shift()!;
    if (seen.has(next)) continue;
    seen.add(next);
    queue.push(...(edges.get(next) ?? []));
  }
  return [...seen];
}

function chunks(html: string, path: string, edges: Map<string, string[]>): string[] {
  const tag = html.match(MODULE);
  const src = tag?.[0].match(SRC)?.[1];
  if (!src) return [];
  return closure(edges, join(dirname(path), src)).map((dep) => `/${dep}`);
}

function seo(source: string, card: Card, nav: string, image: Picture, fonts: string, preload: string[]) {
  const html = source
    .replace(/<html([^>]*)>/, (_, attrs: string) => `<html${attrs.replace(/ data-prefix="[^"]*"/, "")} data-prefix="${SITE.prefix}">`)
    .replace(/<script data-boot>[\s\S]*?<\/script>\n?/, "")
    .replace(OWN, "");
  const route = demoRoute(card.name);
  const found = html.match(TITLE);
  const name = found ? untag(found[1]) : card.title;
  const tags = meta(route, name, card.blurb || name, "website", image);
  const reads = `<script type="application/json" id="${SITE.prefix}reads">${jsonText(card.reads)}</script>`;
  const block = `${BOOT}\n<title>${escape(brand(name))}</title>\n${tags}\n${nav}\n${reads}\n<link rel="stylesheet" href="${fonts}">`;
  const page = found ? html.replace(found[0], block) : html.replace("<head>", `<head>\n${block}`);
  const ahead = preload.map((href) => `<link rel="modulepreload" href="${href}">`).join("\n");
  const ready = ahead ? page.replace(MODULE, (whole) => `${ahead}\n${whole}`) : page;
  return ready.replace("</head>", `${TINT}\n</head>`);
}

const widgetFiles = (site: Site) => site.input("demos").files.filter((f) => f.endsWith("/widget.jsx"));

const BESIDE = /"\.\/lib-/g;

async function demos(site: Site, route: Route): Promise<Output[]> {
  const list = route.data as Card[];
  const home = demoHome(site, "");
  const views = relative(home, site.input("demos").path);
  const served = (path: string) => (path.startsWith(`${views}/`) ? `demos/${path.slice(views.length + 1)}` : path);
  const built = await Bun.build({
    entrypoints: [...list.map((d) => demoShell(site, d.name)), ...widgetFiles(site)],
    root: home,
    splitting: true,
    minify: true,
    define: { "process.env.NODE_ENV": '"production"' },
    naming: { chunk: "lib-[hash].[ext]", asset: "[name]-[hash].[ext]" },
  });
  if (!built.success) throw new Error(`site: the demos failed to bundle\n${built.logs.join("\n")}`);
  const shells = new Map(list.map((d) => [relative(home, demoShell(site, d.name)), d]));
  const json = jsonText(shelved(list));
  const nav = `<script type="application/json" id="${SITE.prefix}tree">${json}</script>`;
  const out: Output[] = [{ path: "demos/tree.json", bytes: json, type: "application/json" }];
  const fig = press(site, out);
  for (const d of list) if (d.name) fig(`demo-${d.name}`, "/demos/");
  const edges = await imports(built.outputs);
  const fonts = site.asset("fonts/fonts.css");
  for (const item of built.outputs) {
    const from = item.path.replace(/^\.\//, "");
    const card = shells.get(from);
    if (!card) {
      out.push({ path: served(from), bytes: new Uint8Array(await item.arrayBuffer()) });
      continue;
    }
    const image = picture(site, card.name ? `demo-${card.name}` : "site-demos", demoRoute(card.name), fig);
    const raw = await item.text();
    const ahead = chunks(raw, from, edges);
    const html = card.name ? raw : raw.replace(BESIDE, '"../lib-');
    out.push({ path: `${demoRoute(card.name).slice(1)}index.html`, bytes: seo(html, card, nav, image, fonts, ahead) });
  }
  return out;
}

/* PAPERS */

type Paper = { slug: string; name: string; lead: string; date: string; revised: string; figure: string; shelf: string; body: string; file: string };

function papers(site: Site): Paper[] {
  const home = site.input("papers");
  return home.files
    .map((file) => {
      const slug = file.slice(home.path.length + 1, -3);
      const { data, body } = front(read(file));
      return { slug, name: data.title ?? slug, lead: data.lead ?? summary(body), date: data.date ?? "", revised: data.revised ?? "", figure: data.figure || `paper-${slug}`, shelf: data.shelf ?? "", body, file };
    })
    .sort((a, b) => (a.date < b.date ? 1 : a.date > b.date ? -1 : a.slug.localeCompare(b.slug)));
}

const marked = (p: Paper) => [p.date, p.revised && `revised ${p.revised}`].filter(Boolean) as string[];

function written(site: Site, route: Route): Output[] {
  const p = route.data as Paper;
  const out: Output[] = [];
  const fig = press(site, out);
  const when = [p.date && `First published ${p.date}`, p.revised && `revised ${p.revised}`].filter(Boolean).join(", ");
  const avatar = pic(fig, p.figure, route.route, p.name, "", "avatar");
  const shelf = p.shelf ? `\n<p class="meta"><a href="https://github.com/carlomitchener/carlomitchener/tree/main/research/${escape(p.shelf)}">The LaTeX and PDF of the first edition, on the shelf</a></p>` : "";
  const plate = `<div class="plate paper">${avatar}<h1 id="${escape(p.slug)}">${escape(p.name)}</h1><p class="by">${escape(AUTHOR)}</p><p class="by">${escape(when)}</p></div>${shelf}`;
  const body = `${plate}\n${md(p.body, { math, link: links(site, p.file, out) })}`;
  const data = {
    "@context": "https://schema.org",
    "@type": "ScholarlyArticle",
    headline: p.name,
    description: p.lead,
    url: root + route.route,
    image: `${root}/figures/${p.figure}-dark.png`,
    author: { "@type": "Organization", name: AUTHOR },
    datePublished: p.date || undefined,
    dateModified: p.revised || p.date || undefined,
    license: "https://creativecommons.org/licenses/by/4.0/",
  };
  out.push({ path: `papers/${p.slug}/index.html`, bytes: shell(site, { route: route.route, name: p.name, description: p.lead, body, data, image: picture(site, p.figure, route.route, fig) }) });
  return out;
}

type Lane = { slug: string; blurb: string; name: string; md: string; published: string; revised: string; pdf: boolean };

let SHELF = "";

function lanes(): Lane[] {
  const readme = read(join(SHELF, "README.md"));
  const order = [...readme.matchAll(LIST)].map((m) => ({ slug: m[1], blurb: plain(m[2]) }));
  const found = readdirSync(SHELF).filter(
    (d) => d !== "template" && existsSync(join(SHELF, d, "README.md")) && existsSync(join(SHELF, d, "paper.tex")),
  );
  const known = new Set(order.map((o) => o.slug));
  const list = order.filter((o) => found.includes(o.slug)).concat(found.filter((l) => !known.has(l)).map((slug) => ({ slug, blurb: "" })));
  return list.map(({ slug, blurb }) => {
    const lane = join(SHELF, slug);
    const doc = read(join(lane, "README.md"));
    const tex = read(join(lane, "paper.tex"));
    const date = tex.match(/\\date\{First published (\d{4}-\d{2}-\d{2})(?:, revised (\d{4}-\d{2}-\d{2}))?\}/);
    return {
      slug,
      blurb,
      name: title(doc) || slug,
      md: doc,
      published: date?.[1] ?? "",
      revised: date?.[2] ?? "",
      pdf: existsSync(join(lane, "paper.pdf")),
    };
  });
}

const dated = (p: Lane) => p.revised || p.published;
const stamps = (p: Lane) => [p.published, p.revised && `revised ${p.revised}`].filter(Boolean) as string[];

function paper(site: Site, route: Route): Output[] {
  const p = route.data as Lane;
  const out: Output[] = [];
  const fig = press(site, out);
  const lane = join(SHELF, p.slug);
  const at = `papers/${p.slug}`;
  if (p.pdf) out.push({ path: `${at}/paper.pdf`, bytes: bytes(join(lane, "paper.pdf")) });
  out.push({ path: `${at}/paper.tex`, bytes: bytes(join(lane, "paper.tex")) });
  for (const file of walk(join(lane, "figures"))) out.push({ path: `${at}/figures/${file.slice(join(lane, "figures").length + 1)}`, bytes: bytes(file) });
  const when = [p.published && `First published ${p.published}`, p.revised && `revised ${p.revised}`].filter(Boolean).join(", ");
  const files = [p.pdf && `<a href="paper.pdf">PDF</a>`, `<a href="paper.tex">TeX</a>`].filter(Boolean).join(" · ");
  const avatar = pic(fig, `paper-${p.slug}`, route.route, p.name, "", "avatar");
  const plate = `<div class="plate paper">${avatar}<h1 id="${escape(p.slug)}">${escape(p.name)}</h1><p class="by">${escape(AUTHOR)}</p><p class="by">${escape(when)}</p></div>\n<p class="meta">${files}</p>`;
  const body = `${plate}\n${md(p.md.replace(/^# .+\n/, "").replace(AVATAR, ""), { math, link: links(site, join(lane, "README.md"), out) })}`;
  const data = {
    "@context": "https://schema.org",
    "@type": "ScholarlyArticle",
    headline: p.name,
    description: p.blurb || summary(p.md),
    url: root + route.route,
    image: `${root}/figures/paper-${p.slug}-dark.png`,
    author: { "@type": "Organization", name: AUTHOR },
    datePublished: p.published || undefined,
    dateModified: dated(p) || undefined,
    license: "https://creativecommons.org/licenses/by/4.0/",
  };
  out.push({ path: `${at}/index.html`, bytes: shell(site, { route: route.route, name: p.name, description: p.blurb || summary(p.md), body, data, image: picture(site, `paper-${p.slug}`, route.route, fig) }) });
  return out;
}

function paperIndex(site: Site, route: Route): Output[] {
  const out: Output[] = [];
  const fig = press(site, out);
  const lead = "Write-ups of MrlyMath, each claim a theorem with a proof, a computational fact with its exact finite domain, or a conjecture labelled as one; markdown that prints as a paper.";
  const fresh = new Set(DRESS.papers.map((p) => `/papers/${p.slug}/`));
  const nodes = wear(site, "/papers/", fig, route.route);
  const shelfNote = `<section><h2 id="shelf">The shelf</h2><p class="lead">The first editions, in LaTeX with a PDF each, deprecated: every paper is rewritten here in turn and the shelf is never edited.</p>${grid(nodes.filter((n) => !fresh.has(n.href ?? "")))}</section>`;
  const body = `<div class="lede"><h1 id="papers">Papers</h1><p class="lead">${escape(lead)}</p></div>\n${grid(nodes.filter((n) => fresh.has(n.href ?? "")))}\n${nodes.some((n) => !fresh.has(n.href ?? "")) ? shelfNote : ""}`;
  out.push({ path: "papers/index.html", bytes: shell(site, { route: route.route, name: "Papers", description: lead, body, type: "website", wide: true, bare: true, image: picture(site, "site-papers", route.route, fig) }) });
  return out;
}

/* RESEARCH */

type Note = { file: string; name: string; md: string; home: boolean; topic: boolean; title: string; lead: string; figure: string };

const SHARED = new Set(["REFS"]);

const ROW = /<tr><td>(?:<code>)?([0-9a-f]{8})(?:<\/code>)?<\/td>/g;

function anchored(html: string) {
  const seen = new Set<string>();
  return html.replace(ROW, (whole, id: string) => {
    if (seen.has(id)) return whole;
    seen.add(id);
    return `<tr id="${id}"><td><code>${id}</code></td>`;
  });
}

function notes(site: Site): Note[] {
  const dir = site.input("research");
  if (dir.missing) {
    console.warn(`site: no research tree at ${relative(org, dir.path)}, research skipped`);
    return [];
  }
  const shared = dir.files.map((source) => {
    const file = source.slice(dir.path.length + 1);
    const name = file.slice(0, -3);
    const md = read(source);
    const figure = SHARED.has(name) ? "research-index" : `research-${name}`;
    return { file, name, md, home: file === "README.md", topic: false, title: title(md) || name, lead: summary(md), figure };
  });
  const home = site.input("notes");
  const topics = home.files.map((source) => {
    const name = source.slice(home.path.length + 1, -3);
    const { data, body } = front(read(source));
    return { file: `notes/${name}.md`, name, md: body, home: false, topic: true, title: data.title ?? name, lead: data.lead ?? summary(body), figure: data.figure || `research-${name}` };
  });
  return [...shared, ...topics].sort((a, b) => (a.home ? -1 : b.home ? 1 : a.name.localeCompare(b.name)));
}

function researchIndex(site: Site, route: Route): Output[] {
  const { note: n } = route.data as { note: Note; cards: string[][] };
  const out: Output[] = [];
  const fig = press(site, out);
  out.push({ path: `research/${n.file}`, bytes: n.md });
  const lead = summary(n.md);
  const prose = md(n.md.replace(/^# .+\n/, ""), { math, link: links(site, join(site.input("research").path, n.file), out) });
  const body = `<div class="lede"><h1 id="research">Research</h1><p class="lead">${escape(lead)}</p></div>\n${grid(wear(site, "/research/", fig, route.route))}\n<article class="prose readme">${prose}</article>`;
  out.push({ path: "research/index.html", bytes: shell(site, { route: route.route, name: "Research", description: lead, body, type: "website", wide: true, bare: true, image: picture(site, "site-research", route.route, fig) }) });
  return out;
}

function note(site: Site, route: Route): Output[] {
  const n = route.data as Note;
  const out: Output[] = [];
  const fig = press(site, out);
  out.push({ path: `research/${n.file}`, bytes: n.md });
  const name = n.title;
  const lead = n.lead;
  const head = n.topic ? `<h1 id="${escape(n.name)}">${escape(name)}</h1>\n` : "";
  const used = new Set<string>();
  const prose = md(n.md, { math, link: links(site, join(site.input("research").path, n.file), out), widget: widgets(site, used) });
  const body = `${hero(fig, n.figure, route.route, name)}\n${head}${n.name === "sequences" ? anchored(prose) : prose}`;
  const data = {
    "@context": "https://schema.org",
    "@type": "Article",
    headline: name,
    description: lead,
    url: root + route.route,
    image: `${root}/figures/${n.figure}-dark.png`,
    author: { "@type": "Organization", name: AUTHOR },
  };
  const at = n.home ? "research/index.html" : `research/${n.name}/index.html`;
  const scripts = [...used].sort().map((name) => `/demos/${name}/widget.js`);
  out.push({ path: at, bytes: shell(site, { route: route.route, name, description: lead, body, type: n.home ? "website" : "article", data, image: picture(site, n.figure, route.route, fig), scripts }) });
  return out;
}

/* CLAIMS */

type Claim = { slug: string; title: string; md: string; file: string };

const TAGS = ["Proved", "Verified", "Conjecture", "Refuted"];
const LINE = /<li>(\d{4}-\d{2}-\d{2}) \[(Proved|Verified|Conjecture|Refuted)\] /g;

function claims(site: Site): Claim[] {
  const home = site.input("claims");
  return home.files
    .map((file) => {
      const slug = file.slice(home.path.length + 1, -3);
      const md = read(file);
      return { slug, title: title(md) || slug, md, file };
    })
    .sort((a, b) => a.title.localeCompare(b.title));
}

function discoveries(site: Site, route: Route): Output[] {
  const list = route.data as Claim[];
  const out: Output[] = [];
  const fig = press(site, out);
  const lead = "Every claim of the tree on one dated, tagged line with its witness, one section per topic, filtered by tag, topic and date.";
  const counts = new Map<string, number>(TAGS.map((tag) => [tag, 0]));
  const sections = list.map((c) => {
    const html = md(c.md.replace(/^# .+\n/, ""), { math, link: links(site, c.file, out) }).replace(LINE, (_, date: string, tag: string) => {
      counts.set(tag, (counts.get(tag) ?? 0) + 1);
      return `<li data-date="${date}" data-tag="${tag.toLowerCase()}"><time>${date}</time> <b class="tag ${tag.toLowerCase()}">${tag}</b> `;
    });
    return `<section class="claims" id="${escape(c.slug)}" data-slug="${escape(c.slug)}"><h2 id="${escape(c.slug)}-claims">${escape(c.title)}</h2>${html}</section>`;
  });
  const total = [...counts.values()].reduce((a, b) => a + b, 0);
  const chips = ["", ...TAGS].map((tag) => `<button type="button" data-tag="${tag.toLowerCase()}"${tag ? "" : ' class="on"'}>${tag || "All"} <span>${tag ? counts.get(tag) : total}</span></button>`).join("");
  const topics = `<select aria-label="Topic"><option value="">Every topic</option>${list.map((c) => `<option value="${escape(c.slug)}">${escape(c.title)}</option>`).join("")}</select>`;
  const since = `<label>Since <input type="date" aria-label="Since"></label>`;
  const bar = `<form class="filter" onsubmit="return false">${chips}${topics}${since}<output>${total} claims</output></form>`;
  const script = claimsScript();
  const body = `${hero(fig, "research-index", route.route, "Discoveries")}\n<h1 id="discoveries">Discoveries</h1><p class="lead">${escape(lead)}</p>\n${bar}\n${sections.join("\n")}\n${script}`;
  const data = {
    "@context": "https://schema.org",
    "@type": "Article",
    headline: "Discoveries",
    description: lead,
    url: root + route.route,
    image: `${root}/figures/research-index-dark.png`,
    author: { "@type": "Organization", name: AUTHOR },
  };
  out.push({ path: "research/discoveries/index.html", bytes: shell(site, { route: route.route, name: "Discoveries", description: lead, body, data, image: picture(site, "research-index", route.route, fig) }) });
  return out;
}

/* BLOG */

type Post = { slug: string; name: string; date: string; lead: string; figure: string; body: string };

const BLOG_LEAD = "Notes on what lands on this site and in the crates behind it.";

const posts = (site: Site): Post[] =>
  parsed(site).map((one) => ({ slug: one.slug, name: one.title, date: one.date, lead: one.lead, figure: one.front.figure || `blog-${one.slug}`, body: one.body }));

function blogPage(site: Site, leaf: Posted) {
  const fig = press(site, leaf.out);
  if (leaf.kind === "blog") {
    const body = `<div class="lede"><h1 id="blog">Blog</h1><p class="lead">${escape(BLOG_LEAD)}</p></div>\n${grid(wear(site, "/blog/", fig, leaf.route))}`;
    return shell(site, { route: leaf.route, name: "Blog", description: BLOG_LEAD, body, type: "website", wide: true, bare: true });
  }
  const figure = leaf.front.figure || `blog-${leaf.slug}`;
  const head = `${hero(fig, figure, leaf.route, leaf.name)}\n<div class="plate"><h1 id="${escape(leaf.slug)}">${escape(leaf.name)}</h1><p class="by">${escape(leaf.date)} · ${escape(AUTHOR)}</p></div>`;
  const data = {
    "@context": "https://schema.org",
    "@type": "BlogPosting",
    headline: leaf.name,
    description: leaf.lead,
    url: root + leaf.route,
    image: `${root}/figures/${figure}-dark.png`,
    author: { "@type": "Organization", name: AUTHOR },
    datePublished: leaf.date || undefined,
  };
  return shell(site, { route: leaf.route, name: leaf.name, description: leaf.lead, body: `${head}\n${leaf.body}`, data, image: picture(site, figure, leaf.route, fig) });
}

/* PAGES */

function page(site: Site, route: Route): Output[] {
  const source = route.source as string;
  const slug = route.route.slice(1, -1);
  const { data, body } = front(read(source));
  const name = data.title ?? slug;
  const lead = data.lead ?? summary(body);
  const out: Output[] = [];
  const fig = press(site, out);
  const open = data.figure ? `${hero(fig, data.figure, route.route, name)}\n` : "";
  const head = `<div class="lede"><h1 id="${escape(slug)}">${escape(name)}</h1><p class="lead">${escape(lead)}</p></div>`;
  const act = data.button && data.link ? `\n<p><a class="button primary" href="${escape(data.link)}">${escape(data.button)}</a></p>` : "";
  const own = data.figure || FIXED[route.route];
  const image = own ? picture(site, own, route.route, fig) : OG;
  const html = shell(site, { route: route.route, name, description: lead, body: `${open}${head}\n${md(body, { math, link: links(site, source, out) })}${act}`, type: "website", image });
  out.push({ path: `${slug}/index.html`, bytes: html });
  return out;
}

type Dress = { lanes: Lane[]; papers: Paper[]; notes: Note[]; posts: Post[]; demos: Card[]; wiki: Entry[] };

type Mark = { figure: string; text: string; dates?: string[] };

let DRESS: Dress = { lanes: [], papers: [], notes: [], posts: [], demos: [], wiki: [] };

const FIXED: Record<string, string> = {
  "/": "site-home",
  "/wiki/": "site-wiki",
  "/tools/": "site-tools",
  "/math/": "site-math",
  "/git/": "site-code",
  "/about/": "site-icon",
  "/contact/": "site-contact",
  "/donate/": "site-donate",
};

function marks(data: Dress): Map<string, Mark> {
  const map = new Map<string, Mark>();
  for (const d of data.demos) if (d.name) map.set(demoRoute(d.name), { figure: `demo-${d.name}`, text: d.blurb });
  for (const p of data.papers) map.set(`/papers/${p.slug}/`, { figure: p.figure, text: p.lead, dates: marked(p) });
  for (const p of data.lanes) map.set(`/papers/${p.slug}/`, { figure: `paper-${p.slug}`, text: p.blurb, dates: stamps(p) });
  for (const n of data.notes) {
    if (n.home) continue;
    map.set(`/research/${n.name}/`, { figure: n.figure, text: n.lead });
  }
  for (const p of data.posts) map.set(`/blog/${p.slug}/`, { figure: p.figure, text: p.lead, dates: [p.date] });
  for (const e of data.wiki) map.set(wikiRoute(e.slug), { figure: e.figure, text: e.lead });
  for (const [href, figure] of Object.entries(FIXED)) map.set(href, { figure, text: "" });
  return map;
}

function dress(nodes: Node[], map: Map<string, Mark>, fig: Fig, route: string): Node[] {
  return nodes.map((node) => {
    if (node.nodes?.length) return { ...node, nodes: dress(node.nodes, map, fig, route) };
    const mark = node.href ? map.get(node.href) : undefined;
    if (!mark) return node;
    return { ...node, figure: fig(mark.figure, route), text: mark.text || undefined, dates: mark.dates };
  });
}

let NAV: Node[] = [];

const wear = (site: Site, href: string, fig: Fig, route: string) => dress(NAV.find((node) => node.href === href)?.nodes ?? [], marks(DRESS), fig, route);

function elsewhere() {
  const links = SITE.socials.map((s) => `<li><a href="${escape(s.href)}">${escape(s.name)}</a></li>`).join("");
  const mail = `<li><a href="mailto:${escape(SITE.contact)}">${escape(SITE.contact)}</a></li>`;
  return `<section class="elsewhere"><h2>Elsewhere</h2><ul>${links}${mail}</ul></section>`;
}

function menu(site: Site, route: Route): Output[] {
  const lead = "Every page on mrly.net.";
  const out: Output[] = [];
  const fig = press(site, out);
  const nav = dress(NAV, marks(route.data as Dress), fig, route.route);
  const list = renderToStaticMarkup(h(Menu, { tree: nav }));
  const body = `<div class="hero"><h1><span role="img" aria-label="${escape(SITE.title)}">${WORD}</span></h1><p>${escape(lead)}</p></div>\n${list}\n${elsewhere()}`;
  out.push({ path: "menu/index.html", bytes: shell(site, { route: route.route, name: "Menu", description: lead, body, type: "website", wide: true, bare: true }) });
  return out;
}

function cart(site: Site, route: Route): Output[] {
  const lead = "Coming soon.";
  const body = `<div class="lede"><h1 id="cart">Cart</h1><p class="lead">${escape(lead)}</p></div>\n<p>mrly.net has no shop yet.</p>\n<p><a href="/">Back to the home page</a>.</p>`;
  return [{ path: "cart/index.html", bytes: shell(site, { route: route.route, name: "Cart", description: lead, body, type: "website" }) }];
}

/* STATS */

const WATCH = "The CDN, the Lambdas and the bucket, read from the bucket every minute.";

function stats(site: Site, route: Route): Output[] {
  const body = `<div class="lede"><h1 id="stats">Stats</h1><p class="lead">${escape(WATCH)} Raw: <a href="/stats/stats.json">stats.json</a>.</p></div>
<section><h2 id="cloud">Cloud</h2><div data-stats="cloud"><p class="fine">Loading</p></div></section>
<section><h2 id="errors">Errors</h2><div data-stats="errors"><p class="fine">Loading</p></div></section>`;
  return [{ path: "stats/index.html", bytes: shell(site, { route: route.route, name: "Stats", description: WATCH, body, type: "website", scripts: [site.asset("stats.js")] }) }];
}

const MISSION = SITE.tagline;

const DOORS = [
  { name: "Wiki", href: "/wiki/", figure: "site-wiki", text: "One concept per page, in the order you need them, for a reader with school mathematics." },
  { name: "Demos", href: "/demos/", figure: "site-demos", text: "Browser pages that draw a design and the numbers around it, live." },
  { name: "Discoveries", href: "/research/discoveries/", figure: "research-index", text: "Every claim of the tree on one dated, tagged line with its witness." },
  { name: "Papers", href: "/papers/", figure: "site-papers", text: "Write-ups that print as papers, every claim tagged and every number generated." },
  { name: "Research", href: "/research/", figure: "site-research", text: "The working notes behind the demos and the papers, one page per idea." },
];

type Newest = { date: string; tag: string; text: string; slug: string; file: string };

function newest(list: Claim[]): Newest | null {
  let best: Newest | null = null;
  for (const c of list) {
    for (const line of c.md.split("\n")) {
      const hit = line.match(/^- (\d{4}-\d{2}-\d{2}) \[(Proved|Verified|Conjecture|Refuted)\] (.+)$/);
      if (hit && (!best || hit[1]! > best.date)) best = { date: hit[1]!, tag: hit[2]!, text: hit[3]!, slug: c.slug, file: c.file };
    }
  }
  return best;
}

function home(site: Site, route: Route): Output[] {
  const { lanes: list, papers: fresh, posts: posted, claim } = route.data as { lanes: Lane[]; papers: Paper[]; posts: Post[]; claim: Newest | null };
  const out: Output[] = [];
  const fig = press(site, out);
  const latestClaim = claim
    ? `<section><h2 id="newest">Newest claim</h2><p class="lead"><time>${claim.date}</time> <b class="chip ${claim.tag.toLowerCase()}">${claim.tag}</b> ${inline(claim.text, { math, link: links(site, claim.file, out) })}</p><p class="lead"><a href="/research/discoveries/#${escape(claim.slug)}">Every claim, dated and tagged</a></p></section>`
    : "";
  const latest = [...fresh.map((p) => ({ name: p.name, slug: p.slug, at: p.revised || p.date })), ...list.map((p) => ({ name: p.name, slug: p.slug, at: dated(p) }))]
    .map((p, i) => ({ p, i }))
    .sort((a, b) => (a.p.at < b.p.at ? 1 : a.p.at > b.p.at ? -1 : a.i - b.i))
    .slice(0, 3)
    .map(({ p }) => ({ name: p.name, href: `/papers/${p.slug}/` }));
  const doors = DOORS.map((d) => ({ name: d.name, href: d.href, figure: fig(d.figure, "/"), text: d.text }));
  const first = posted[0];
  const news = first
    ? `<section><h2 id="latest">From the blog</h2><p class="lead"><a href="/blog/${first.slug}/">${escape(first.name)}</a> · ${escape(first.date)}</p><p class="lead">${escape(first.lead)}</p></section>`
    : "";
  const what = `<section class="what"><h2 id="mrlymath">What is MrlyMath</h2><p>A design is a rule on the corners of a cube: a code says which of the eight corners are filled. The Kronecker product grows that rule into itself, level by level, and the object it converges to is a fractal - the Sierpinski carpet and the Menger sponge are two of them.</p><p>Everything else is measurement. Count the fills, the voids and the exposed faces; cut the solid with a plane; join the filled cells into a graph and read its spectrum; collect the integer sequences the counts write down. The Rust crates do the arithmetic, the browser only paints, and a claim is either proved, checked over a stated finite domain, or labelled a conjecture.</p></section>`;
  const body = `<div class="home">
${hero(fig, "site-home", "/", SITE.title)}
<div class="hero"><h1><span role="img" aria-label="${escape(SITE.title)}">${WORD}</span></h1><p>${escape(MISSION)}</p></div>
<section><h2 id="doors">Five doors</h2>${grid(doors)}</section>
${latestClaim}
<section><h2 id="shelf">Latest papers</h2>${grid(dress(latest, marks(DRESS), fig, "/"))}</section>
${news}
${what}
</div>`;
  out.push({ path: "index.html", bytes: shell(site, { route: "/", name: SITE.title, description: MISSION, body, type: "website", wide: true, bare: true, image: picture(site, "site-home", "/", fig) }) });
  return out;
}

function missing(site: Site, route: Route): Output[] {
  const doors = DOORS.map((d) => `<a href="${d.href}">${d.name}</a>`);
  const body = `<div class="lede"><h1 id="lost">Nothing here</h1><p class="lead">That page does not exist. The <a href="/menu/">Menu</a> lists every page on this site, and the doors are ${doors.slice(0, -1).join(", ")} and ${doors[doors.length - 1]}.</p></div>`;
  return [{ path: "404.html", bytes: shell(site, { route: route.route, name: "Nothing here", description: "That page does not exist.", body, type: "website", bare: true }) }];
}

/* THIN */

const THIN: Record<string, { name: string; figure: string; lead: string; note: string }> = {
  "/tools/": {
    name: "Tools",
    figure: "site-tools",
    lead: "A canvas for mrly objects: tiles, slices, rings and roulettes on one sheet, the perforator first.",
    note: `The first tool is on its way. The <a href="/demos/">demos</a> already draw every object it will place, and <a href="/math/">the standard</a> names them.`,
  },
};

function thin(site: Site, route: Route): Output[] {
  const t = THIN[route.route]!;
  const out: Output[] = [];
  const fig = press(site, out);
  const slug = route.route.slice(1, -1);
  const body = `${hero(fig, t.figure, route.route, t.name)}\n<div class="lede"><h1 id="${slug}">${t.name}</h1><p class="lead">${escape(t.lead)}</p></div>\n<p>${t.note}</p>`;
  out.push({ path: `${slug}/index.html`, bytes: shell(site, { route: route.route, name: t.name, description: t.lead, body, type: "website", image: picture(site, t.figure, route.route, fig) }) });
  return out;
}

/* WIKI */

type Entry = { slug: string; name: string; lead: string; figure: string; needs: string[]; body: string; file: string };

type Concept = Omit<Entry, "body" | "file"> & { before: { slug: string; name: string }[]; after: { slug: string; name: string }[] };

const WIKI = "One concept per page, in the order you need them, for a reader with school mathematics.";

function ordered(list: Entry[]): Entry[] {
  const byslug = new Map(list.map((e) => [e.slug, e]));
  const done = new Set<string>();
  const out: Entry[] = [];
  const pending = [...list].sort((a, b) => a.slug.localeCompare(b.slug));
  while (pending.length) {
    const at = pending.findIndex((e) => e.needs.every((need) => done.has(need)));
    if (at < 0) throw new Error(`site: the wiki prerequisites run in a circle through ${pending.map((e) => e.slug).join(", ")}`);
    const [next] = pending.splice(at, 1);
    done.add(next!.slug);
    out.push(byslug.get(next!.slug)!);
  }
  const deep = depths(out);
  return out.sort((a, b) => deep.get(a.slug)! - deep.get(b.slug)! || a.slug.localeCompare(b.slug));
}

function wiki(site: Site): Entry[] {
  const home = site.input("wiki");
  if (home.missing) return [];
  const list = home.files.map((file) => {
    const slug = file.slice(home.path.length + 1, -3);
    const { data, body } = front(read(file));
    const needs = (data.prerequisites ?? "").split(",").map((s: string) => s.trim()).filter(Boolean);
    return { slug, name: data.title ?? slug, lead: data.lead ?? summary(body), figure: data.figure || `wiki-${slug}`, needs, body, file };
  });
  const known = new Set(list.map((e) => e.slug));
  for (const e of list) for (const need of e.needs) if (!known.has(need)) throw new Error(`site: wiki/${e.slug}.md needs ${need}, and wiki/${need}.md does not exist`);
  return ordered(list);
}

const wikiRoute = (slug: string) => `/wiki/${slug}/`;

function concepts(list: Entry[]): [Concept, string][] {
  const name = (slug: string) => ({ slug, name: list.find((e) => e.slug === slug)?.name ?? slug });
  return list.map(({ body: _, file, ...e }) => [{ ...e, before: e.needs.map(name), after: list.filter((o) => o.needs.includes(e.slug)).map((o) => name(o.slug)) }, file]);
}

const cite = (rows: { slug: string; name: string }[]) => rows.map((r) => `<a href="${wikiRoute(r.slug)}">${escape(r.name)}</a>`).join(", ");

/* WIDGETS */

const EMBEDS = /^!\[[^\]]*\]\(demos\/([a-z0-9-]+)\/[a-z0-9-]+\)$/gm;

const VIEW = (view: string) => new RegExp(`^export (?:function|const) ${view}\\b`, "m");

const widgetFile = (site: Site, name: string) => join(site.input("demos").path, name, "widget.jsx");

const embeds = (site: Site, body: string) => [...body.matchAll(EMBEDS)].map((m) => widgetFile(site, m[1]!));

function widgets(site: Site, used: Set<string>) {
  return (name: string, view: string, caption: string) => {
    const file = widgetFile(site, name);
    if (!existsSync(file)) throw new Error(`site: demos/${name}/ has no widget.jsx to embed`);
    if (!VIEW(view).test(read(file))) throw new Error(`site: demos/${name}/widget.jsx exports no view named ${view}`);
    used.add(name);
    return `<figure class="widget" data-demo="${name}" data-view="${view}"><div class="mount"></div><figcaption>${caption}</figcaption></figure>`;
  };
}

function concept(site: Site, route: Route): Output[] {
  const c = route.data as Concept;
  const file = route.source as string;
  const out: Output[] = [];
  const fig = press(site, out);
  const used = new Set<string>();
  const before = c.before.length ? `\n<p class="meta">Before this: ${cite(c.before)}.</p>` : "";
  const after = c.after.length ? `\n<section><h2 id="next">Read next</h2><p>${cite(c.after)}.</p></section>` : "";
  const head = `<div class="lede"><h1 id="${escape(c.slug)}">${escape(c.name)}</h1><p class="lead">${escape(c.lead)}</p></div>`;
  const prose = md(front(read(file)).body, { math, link: links(site, file, out), widget: widgets(site, used) });
  const body = `${hero(fig, c.figure, route.route, c.name)}\n${head}${before}\n${prose}${after}`;
  const data = {
    "@context": "https://schema.org",
    "@type": "Article",
    headline: c.name,
    description: c.lead,
    url: root + route.route,
    image: `${root}/figures/${c.figure}-dark.png`,
    author: { "@type": "Organization", name: AUTHOR },
  };
  const scripts = [...used].sort().map((name) => `/demos/${name}/widget.js`);
  out.push({ path: `wiki/${c.slug}/index.html`, bytes: shell(site, { route: route.route, name: c.name, description: c.lead, body, data, image: picture(site, c.figure, route.route, fig), scripts }) });
  return out;
}

type Leaf_ = { slug: string; name: string; lead: string; figure: string; needs: string[] };

const STEPS = ["Start here", "One step in", "Two steps in", "Three steps in", "Four steps in", "Five steps in", "Six steps in"];

const step = (n: number) => STEPS[n] ?? `${n} steps in`;

function depths(list: Leaf_[]): Map<string, number> {
  const deep = new Map<string, number>();
  for (const e of list) deep.set(e.slug, e.needs.length ? 1 + Math.max(...e.needs.map((need) => deep.get(need) ?? 0)) : 0);
  return deep;
}

function tile(fig: Fig, route: string, e: Leaf_, names: Map<string, string>) {
  const needs = e.needs.length ? `<p class="needs">After ${e.needs.map((need) => `<a href="${wikiRoute(need)}">${escape(names.get(need) ?? need)}</a>`).join(", ")}</p>` : "";
  const img = pic(fig, e.figure, route, "", ` loading="lazy" decoding="async"`);
  return `<div class="tile"><a href="${wikiRoute(e.slug)}">${img}<h2>${escape(e.name)}</h2><p>${escape(e.lead)}</p></a>${needs}</div>`;
}

function wikiIndex(site: Site, route: Route): Output[] {
  const list = (route.data as [string, string, string, string, string[]][]).map(([slug, name, lead, figure, needs]) => ({ slug, name, lead, figure, needs }));
  const out: Output[] = [];
  const fig = press(site, out);
  const names = new Map(list.map((e) => [e.slug, e.name]));
  const deep = depths(list);
  const rows = new Map<number, Leaf_[]>();
  for (const e of list) rows.set(deep.get(e.slug)!, [...(rows.get(deep.get(e.slug)!) ?? []), e]);
  const sections = [...rows.keys()].sort((a, b) => a - b).map((n) => `<section><h2 id="step-${n}">${step(n)}</h2><div class="gallery grid graph">${rows.get(n)!.map((e) => tile(fig, route.route, e, names)).join("")}</div></section>`);
  const order = list.length ? `<p class="lead">Every page needs only the pages above it.</p>` : "";
  const body = `${hero(fig, "site-wiki", route.route, "Wiki")}\n<div class="lede"><h1 id="wiki">Wiki</h1><p class="lead">${escape(WIKI)}</p>${order}</div>\n${sections.join("\n")}`;
  out.push({ path: "wiki/index.html", bytes: shell(site, { route: route.route, name: "Wiki", description: WIKI, body, type: "website", wide: true, bare: true, image: picture(site, "site-wiki", route.route, fig) }) });
  return out;
}

/* MATH */

const STANDARD = "A design is one string: one JSON object per named thing, and every other name a view cut from it.";

function standard(site: Site, route: Route): Output[] {
  const out: Output[] = [];
  const fig = press(site, out);
  const file = route.source as string;
  const body = `${hero(fig, "site-math", route.route, "MrlyMath")}\n<h1 id="math">MrlyMath</h1><p class="lead">${escape(STANDARD)}</p>\n${md(read(file).replace(/^# .+\n/, ""), { math, link: links(site, file, out) })}`;
  out.push({ path: "math/index.html", bytes: shell(site, { route: route.route, name: "Math", description: STANDARD, body, type: "website", image: picture(site, "site-math", route.route, fig) }) });
  return out;
}

/* COLLECT */

const counts = { papers: 0, research: 0, blog: 0, demos: 0, wiki: 0 };

async function collect(site: Site) {
  SHELF = await shelf();
  const paperList = papers(site);
  const fresh = new Set(paperList.map((p) => p.slug));
  const laneList = lanes().filter((p) => !fresh.has(p.slug));
  const noteList = notes(site);
  const postList = posts(site);
  const group = demoGroup(site);
  const demoList = group.data as Card[];
  const claimList = claims(site);
  const wikiList = wiki(site);
  readers(demoList, [
    ...noteList.filter((n) => !n.home).map((n) => ({ name: n.title, href: `/research/${n.name}/`, md: n.md })),
    ...paperList.map((p) => ({ name: p.name, href: `/papers/${p.slug}/`, md: p.body })),
    ...wikiList.map((e) => ({ name: e.name, href: wikiRoute(e.slug), md: e.body })),
  ]);
  counts.papers = laneList.length + paperList.length;
  counts.research = noteList.length;
  counts.blog = postList.length;
  counts.demos = demoList.length;
  counts.wiki = wikiList.length;
  const lists = {
    wiki: wikiList.map((e) => ({ name: e.name, href: wikiRoute(e.slug) })),
    demos: shelved(demoList),
    papers: [...paperList, ...laneList].map((p) => ({ name: p.name, href: `/papers/${p.slug}/` })),
    research: [...(claimList.length ? [{ name: "Discoveries", href: "/research/discoveries/" }] : []), ...noteList.filter((n) => !n.home).map((n) => ({ name: n.title, href: `/research/${n.name}/` }))],
    blog: postList.map((p) => ({ name: p.name, href: `/blog/${p.slug}/` })),
  };
  NAV = tree(lists);
  const nav = sidebar(lists);
  const routes: Route[] = [];
  const readme = join(org, "README.md");
  routes.push({
    route: "/",
    kind: "home",
    name: SITE.title,
    data: { lanes: laneList, papers: paperList, posts: postList, claim: newest(claimList) },
    source: readme,
    inputs: [readme],
  });
  for (const route of Object.keys(THIN)) routes.push({ route, kind: "thin", name: THIN[route]!.name });
  const shelfOfWiki = site.input("wiki");
  routes.push({ route: "/wiki/", kind: "wiki", name: "Wiki", data: wikiList.map((e) => [e.slug, e.name, e.lead, e.figure, e.needs]), source: shelfOfWiki.path, inputs: shelfOfWiki.missing ? [] : wikiList.map((e) => e.file) });
  for (const [c, file] of concepts(wikiList)) {
    routes.push({ route: wikiRoute(c.slug), kind: "concept", name: c.name, data: c, source: file, inputs: [file, ...embeds(site, read(file))] });
  }
  const names = site.input("names");
  if (!names.missing) routes.push({ route: "/math/", kind: "math", name: "Math", source: names.files[0]!, inputs: names.files });
  routes.push(group);
  if (laneList.length || paperList.length) {
    const index = join(SHELF, "README.md");
    routes.push({ route: "/papers/", kind: "papers", name: "Papers", data: { lanes: laneList, papers: paperList.map((p) => [p.slug, p.name, p.lead, p.date, p.revised, p.figure]) }, source: index, inputs: [index, ...paperList.map((p) => p.file)] });
    for (const p of paperList) {
      routes.push({ route: `/papers/${p.slug}/`, kind: "written", name: p.name, data: p, source: p.file, inputs: [p.file] });
    }
    for (const p of laneList) {
      const lane = join(SHELF, p.slug);
      routes.push({ route: `/papers/${p.slug}/`, kind: "paper", name: p.name, data: p, source: lane, inputs: [lane] });
    }
  }
  const notesHome = site.input("research").path;
  if (claimList.length) {
    routes.push({
      route: "/research/discoveries/",
      kind: "discoveries",
      name: "Discoveries",
      data: claimList.map((c) => c.slug),
      source: site.input("claims").path,
      inputs: claimList.map((c) => c.file),
    });
  }
  for (const n of noteList) {
    const source = join(notesHome, n.file);
    routes.push({
      route: n.home ? "/research/" : `/research/${n.name}/`,
      kind: n.home ? "research" : "note",
      name: n.home ? "Research" : n.title,
      data: n.home ? { note: n, cards: noteList.map((one) => [one.name, one.title, one.lead, one.figure]) } : n,
      source,
      inputs: [source, ...embeds(site, n.md)],
    });
  }
  const written = site.input("pages");
  for (const source of written.files) {
    const slug = source.slice(written.path.length + 1, -3);
    const { data } = front(read(source));
    routes.push({ route: `/${slug}/`, kind: "page", name: data.title ?? slug, source, inputs: [source] });
  }
  DRESS = { lanes: laneList, papers: paperList, notes: noteList, posts: postList, demos: demoList, wiki: wikiList };
  routes.push({
    route: "/menu/",
    kind: "menu",
    name: "Menu",
    data: DRESS,
  });
  routes.push({ route: "/cart/", kind: "cart", name: "Cart", hidden: true });
  routes.push({ route: "/stats/", kind: "stats", name: "Stats", hidden: true });
  routes.push({ route: "/404.html", kind: "missing", name: "Nothing here", hidden: true });
  return { routes, nav };
}

/* RENDER */

const KINDS: Record<string, (site: Site, route: Route) => Output[] | Promise<Output[]>> = {
  home,
  demos,
  papers: paperIndex,
  paper,
  written,
  note,
  research: researchIndex,
  discoveries,
  page,
  menu,
  cart,
  stats,
  missing,
  thin,
  math: standard,
  wiki: wikiIndex,
  concept,
};

function draw(site: Site, route: Route) {
  const fn = KINDS[route.kind ?? ""];
  if (!fn) throw new Error(`site: no template for ${route.route}`);
  if (route.kind === "discoveries") return fn(site, { ...route, data: claims(site) });
  return fn(site, route);
}

/* EXTRAS */

function extras(site: Site): Output[] {
  const out: Output[] = [];
  const home = site.input("figures").path;
  out.push({ path: "og.png", bytes: bytes(figure(home, "site-og-dark", "/og.png")) });
  return out;
}

/* SERVED */

function served(site: Site, path: string): string | null {
  const git = gitConfig(site);
  if (!git) return null;
  const file = join(git.root, path);
  const hit = site.serves.get(file);
  if (hit) return hit;
  const home = site.input("figures").path;
  if (!file.startsWith(`${home}/`)) return null;
  const at = `figures/${file.slice(home.length + 1)}`;
  return site.made.has(at) ? `/${at}` : null;
}

/* SPEC */

const GIT = process.env.MRLY_GIT !== "0";

const KEEP = GIT ? ".manifest.json" : ".manifest-nogit.json";

const MANIFEST = process.env.MRLY_DIST ? join(dist, KEEP) : `.cache/${KEEP.slice(1)}`;

export const counted = () => ({ ...counts });

export const spec: Spec = {
  root: org,
  out: dist,
  templates: ["lib", "scripts", "ui"],
  inline: inlineScripts(SITE.prefix),
  icons: { rows: glyphs(1), svg: logoSvg(1, "#000000", "#ffffff") },
  collect,
  render: draw,
  globals: extras,
  git: {
    page: GIT ? shell : undefined,
    md: (site, text, from) => md(front(text).body, { math, link: links(site, from) }),
    served,
  },
  blog: {
    page: blogPage,
    md: (site, text, from, out) => md(text, { math, link: links(site, from, out) }),
  },
};

if (import.meta.main) {
  const done = await build(spec, { manifest: MANIFEST });
  const site = done.site;
  const code = site.routes.filter(isGit).length;
  console.log(
    `site: ${site.routes.length} routes, ${counts.demos} demo shells, ${counts.papers} papers, ${counts.research} research pages, ${counts.blog} posts, ${code} code pages, ${done.rendered} rendered, ${done.written} files written, ${done.removed} removed`,
  );
}
