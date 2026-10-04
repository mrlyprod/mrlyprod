import type { BunPlugin } from "bun";
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { basename, dirname, join, relative, resolve } from "node:path";
import { createElement as h } from "react";
import { renderToStaticMarkup } from "react-dom/server";
import katex from "katex";
import { build, bytes, jsonScript, probe, walk, type Node, type Output, type Route, type Site, type Spec } from "../kit/ssg/build.ts";
import { config as gitConfig, shell as gitShell } from "../kit/git/git.ts";
import { resolve as resolveLink } from "../kit/ssg/links.ts";
import type { Shell as Entry } from "../kit/ssg/modes.ts";
import { themed } from "../kit/ssg/pic.ts";
import { escape, front, inline, plain, render as md, summary, title } from "../kit/ssg/md.ts";
import { posts as parsed, type Leaf as Posted } from "../kit/ssg/blog.ts";
import { Glyph, Grid, Menu, Settings, Shell } from "../ui/chrome.jsx";
import { claimsScript, headScript, inlineScripts, tintCss } from "../ui/config.js";
import { grid as glyphs, logoSvg } from "../ui/logo.js";
import "../lib/site.js";
import SITE from "../site.json";
import { ensureFigures, said } from "./figs.ts";
import { sections, type Lists } from "./map.ts";
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
const INDEX = "README.md";
const sheets = (files: string[]) => files.filter((file) => !file.endsWith(`/${INDEX}`));
const pageOf = (route: string) => `${route.slice(1)}index.html`;
const math = (tex: string, display: boolean) =>
  katex.renderToString(tex, { output: "mathml", throwOnError: false, displayMode: display });
const untag = (html: string) =>
  html.replace(/<[^>]+>/g, "").replace(/&amp;/g, "&").replace(/&lt;/g, "<").replace(/&gt;/g, ">").replace(/&quot;/g, '"');
const brand = (name: string) => (name === SITE.title ? name : `${name} · ${SITE.title}`);

function lede(text: string, max = 200) {
  const all = summary(text, Infinity);
  if (all.length <= max) return all;
  const cut = all.slice(0, max + 1).lastIndexOf(". ");
  const first = all.indexOf(". ");
  return cut > 0 ? all.slice(0, cut + 1) : first > 0 ? all.slice(0, first + 1) : all;
}

/* FOLDERS */

const HUB = "/research/";
const WIKI = { name: "Wiki", href: `${HUB}wiki/`, figure: "site-wiki", text: "One concept per page, in the order you need them, for a reader with school mathematics." };
const NOTES = { name: "Notes", href: `${HUB}notes/`, figure: "site-research", text: "The working notes behind the demos and the papers, one page per idea." };
const CLAIMS = { name: "Claims", href: `${HUB}claims/`, figure: "research-index", text: "Every claim of the tree on one dated, tagged line with its witness." };
const PAPERS = { name: "Papers", href: `${HUB}papers/`, figure: "site-papers", text: "Write-ups that print as papers, every claim tagged and every number generated." };
const FOLDERS = [WIKI, NOTES, CLAIMS, PAPERS];

/* LINKS */

const NAME = /^[a-z0-9-]+$/;

function links(site: Site, from: string, out: Output[]) {
  const home = site.input("figures").path;
  const fig = press(site, out);
  return (url: string, image = false) => {
    const named = NAME.test(url);
    if (named && (probe(join(home, `${url}.png`)) || probe(join(home, `${url}.webp`)))) {
      const ext = probe(join(home, `${url}.webp`)) ? "webp" : "png";
      const path = `figures/${url}.${ext}`;
      if (!out.some((item) => item.path === path)) out.push({ path, bytes: bytes(join(home, `${url}.${ext}`)) });
      return `/${path}`;
    }
    if (named && probe(join(home, `${url}-light.webp`))) return fig(url, from);
    if (named && image) throw new Error(`site: ![](${url}) in ${relative(resolve(org, ".."), from)} names no figure; figures.lock has no row for it`);
    return resolveLink(site, from, url);
  };
}

/* FIGURES */

const SIDES = ["dark", "light"] as const;

function figure(home: string, name: string, route: string, ext = "png") {
  const file = join(home, `${name}.${ext}`);
  if (!probe(file)) throw new Error(`site: ${name}.${ext} missing from ${relative(org, home)} for ${route}; figures.lock has no row for it and the press made none`);
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

const hero = (fig: Fig, name: string, route: string, alt: string, live = false) =>
  `<figure class="opener"${live ? ` data-live="${name}"` : ""}>${pic(fig, name, route, alt)}</figure>`;

const grid = (nodes: Node[]) => renderToStaticMarkup(h(Grid, { nodes }));

/* LIVE */

type Live = { name: string; file: string; units: string[]; inputs: string[] };

const DRAWS = resolve(org, "../figures");
const MRLYJS = resolve(org, "../pkgs/mrlyjs");
const EXPORTS = JSON.parse(read(join(MRLYJS, "package.json"))).exports as Record<string, { default?: string }>;
const GLUE = join(org, "demos", "live.js");
const GLUE_SRC = "/live.js";
const UNIT = /^mrlyjs\/([a-z]+)$/;
const scanner = new Bun.Transpiler({ loader: "ts" });

const unitWasm = (unit: string) => join(org, "pkg", unit, `mrlyjs_${unit}_bg.wasm`);

let LIVE: Live[] = [];

function sources(file: string, seen = new Set<string>()) {
  if (seen.has(file)) return seen;
  seen.add(file);
  if (!/\.[cm]?[jt]s$/.test(file)) return seen;
  for (const { path } of scanner.scanImports(read(file))) if (path.startsWith(".")) sources(Bun.resolveSync(path, dirname(file)), seen);
  return seen;
}

function live(): Live[] {
  const kit = walk(join(MRLYJS, "view"), false).filter((file) => !file.endsWith(".test.js"));
  return walk(DRAWS, false).flatMap((file) => {
    const name = file.slice(DRAWS.length + 1, -3);
    if (!file.endsWith(".ts") || !NAME.test(name)) return [];
    const { exports, imports } = scanner.scan(read(file));
    if (!exports.includes("loop") || !exports.includes("default")) return [];
    const units = imports.flatMap((one) => one.path.match(UNIT)?.slice(1) ?? []).filter((unit) => unit !== "view");
    for (const unit of units) {
      if (!existsSync(unitWasm(unit))) throw new Error(`site: figures/${name}.ts animates with mrlyjs/${unit}, and site/pkg/${unit}/ does not hold it; scripts/wasm.sh copies the live units there`);
    }
    return [{ name, file, units, inputs: [...sources(file), ...kit, ...units.map((unit) => join(MRLYJS, `${unit}.js`))] }];
  });
}

function roster(list: Live[]) {
  const units = [...new Set(list.flatMap((one) => one.units))].sort();
  return [
    ...units.map((unit) => `import ${unit} from ${JSON.stringify(unitWasm(unit))};`),
    `export const units = { ${units.join(", ")} };`,
    `export const figures = { ${list.map((one) => `${JSON.stringify(one.name)}: () => import(${JSON.stringify(one.file)})`).join(", ")} };`,
  ].join("\n");
}

const mrlyjs = (list: Live[]): BunPlugin => ({
  name: "mrlyjs",
  setup(build) {
    build.onResolve({ filter: /^mrlyjs\// }, ({ path }) => {
      const door = EXPORTS[`./${path.slice(7)}`]?.default;
      return door ? { path: join(MRLYJS, door) } : undefined;
    });
    build.onResolve({ filter: /^\.\/pkg\// }, ({ path, importer }) => (dirname(importer) === MRLYJS ? { path: join(org, path) } : undefined));
    build.onResolve({ filter: /^live:figures$/ }, ({ path }) => ({ path, namespace: "live" }));
    build.onLoad({ filter: /.*/, namespace: "live" }, () => ({ contents: roster(list), loader: "js" }));
  },
});

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

type Heading = { level: number; id: string; text: string };

function headings(body: string): Heading[] {
  return [...body.matchAll(HEADING)].map((m) => ({ level: Number(m[1]), id: m[2]!, text: untag(m[3]!) }));
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
  contents?: Heading[];
  controls?: boolean;
  sheets?: string[];
  head?: string;
};

function shell(site: Site, leaf: Leaf) {
  const { route, name, description, body, type = "article", wide = false, bare = false, code = false, data, controls = false } = leaf;
  const article = h(bare ? "div" : "article", { className: bare ? undefined : "prose", dangerouslySetInnerHTML: { __html: body } });
  const main = renderToStaticMarkup(h(Shell, { route, tree: leaf.tree ?? [], contents: leaf.contents ?? headings(body), controls, late: code, wide }, article));
  const ld = data ? `${jsonScript(data)}\n` : "";
  const more = (leaf.scripts ?? []).map((src) => `\n<script type="module" src="${src}"></script>`).join("");
  const sheets = (leaf.sheets ?? [site.asset(code ? "git.css" : "page.css")]).map((href) => `<link rel="stylesheet" href="${href}">`).join("\n");
  const image = leaf.image ?? (code ? picture(site, "site-code", route) : OG);
  return `<!doctype html>
<html lang="en" data-prefix="${SITE.prefix}">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
${BOOT}
<title>${escape(brand(name))}</title>
${meta(route, name, description, type, image)}
${sheets}
${TINT}
${ld}<script type="module" src="${site.asset("chrome.js")}"></script>${more}${leaf.head ?? ""}
</head>
<body>
${main}
</body>
</html>
`;
}

/* DEMOS */

type Card = { name: string; title: string; blurb: string; shelf: string; order: number; bar: boolean; reads: Read[] };

type Read = { name: string; href: string };

type Shelf = { key: string; group: string; title: string; blurb: string };

type Bay = Node & { key: string };

const SHELVES = (SITE.shelves ?? []) as Shelf[];

const DEMOS = "/demos/";

const EYES = { name: "The eyes of MrlyMath", lead: "Every number and pixel on these pages comes out of the Rust crates through wasm. The browser only draws." };

const NEEDS = { demo: "This demo draws in the browser and needs JavaScript.", widget: "This figure draws in the browser and needs JavaScript.", stats: "These numbers load in the browser and need JavaScript." };

const TITLE = /<title>([^<]*)<\/title>/;

const SHEET = /<link rel="stylesheet"[^>]*?href="([^"]+)"/g;

const CARRIED = /<link rel="modulepreload"[^>]*>|<script\b[^>]*>[\s\S]*?<\/script>/g;

const tag = (html: string, name: string) => {
  const found = html.match(new RegExp(`<meta name="${name}" content="([^"]*)">`));
  return found ? untag(found[1]) : "";
};

const demoHome = (site: Site, name: string) => join(site.input("demos").path, name);

const demoShell = (site: Site, name: string) => join(demoHome(site, name), "index.html");

function demoNames(site: Site) {
  const home = site.input("demos").path;
  return site
    .input("demos")
    .files.filter((f) => f.endsWith("/index.html"))
    .map((f) => dirname(f).slice(home.length + 1))
    .sort((a, b) => a.localeCompare(b));
}

const demoRoute = (name: string) => `${DEMOS}${name}/`;

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
      bar: tag(html, "bar") !== "none",
      reads: [],
    };
  });
}

const MENTION = (name: string) => new RegExp(`demos/${name}/`);

function readers(list: Card[], pages: { name: string; href: string; md: string }[]) {
  for (const card of list) {
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

function demoRoutes(site: Site, list: Card[]): Route[] {
  const home = site.input("demos").path;
  const shells = list.map((d) => demoShell(site, d.name));
  return [
    { route: DEMOS, kind: "gallery", name: "Demos", mode: "ssg", data: shelved(list), source: home, inputs: shells, urls: [{ route: DEMOS, name: EYES.name, source: dirname(home) }] },
    ...list.map((d, n) => ({ route: demoRoute(d.name), kind: "demo", name: d.title, data: d, source: demoHome(site, d.name), inputs: [shells[n]!], entry: shells[n]! })),
  ];
}

function gallery(site: Site, route: Route): Output[] {
  const bays = route.data as Bay[];
  const out: Output[] = [];
  const fig = press(site, out);
  const groups: { name: string; shelves: Shelf[] }[] = [];
  for (const one of SHELVES) {
    const last = groups[groups.length - 1];
    if (last && last.name === one.group) last.shelves.push(one);
    else groups.push({ name: one.group, shelves: [one] });
  }
  const tiles = (key: string) => (bays.find((one) => one.key === key)?.nodes ?? []).map((node) => ({ ...node, figure: fig(`demo-${node.href!.slice(DEMOS.length, -1)}`, route.route) }));
  const bay = (one: Shelf) => {
    const nodes = tiles(one.key);
    return nodes.length ? `<div class="shelf" id="${one.key}"><h2>${escape(one.title)}</h2><p>${escape(one.blurb)}</p></div>${grid(nodes)}` : "";
  };
  const contents = groups.flatMap((group) => [{ id: group.name.toLowerCase(), text: group.name, level: 2 }, ...group.shelves.map((one) => ({ id: one.key, text: one.title, level: 3 }))]);
  const sections = groups.map((group) => `<section id="${group.name.toLowerCase()}"><h2 class="group">${escape(group.name)}</h2>${group.shelves.map(bay).join("")}</section>`);
  const body = `<div class="lede"><h1 id="demos">${escape(EYES.name)}</h1><p class="lead">${escape(EYES.lead)}</p></div>${sections.join("")}`;
  out.push({ path: pageOf(route.route), bytes: shell(site, { route: route.route, name: EYES.name, description: EYES.lead, body, type: "website", wide: true, bare: true, contents, image: picture(site, "site-demos", route.route, fig) }) });
  return out;
}

const widgetFiles = (site: Site) => site.input("demos").files.filter((f) => f.endsWith("/widget.jsx"));

const demoEntries = (site: Site) => [...demoNames(site).map((name) => demoShell(site, name)), ...widgetFiles(site), ...(LIVE.length ? [GLUE] : [])];

const demoPlace = (path: string) => (path.startsWith("demos/views/") ? `demos/${path.slice(12)}` : path === "demos/live.js" ? "live.js" : path);

function demoPage(site: Site, route: Route, entry: Entry, out: Output[]) {
  const card = route.data as Card;
  const fig = press(site, out);
  const figure = `demo-${card.name}`;
  const reads = card.reads.length ? `<p class="reads">Read: ${card.reads.map((p) => `<a href="${p.href}">${escape(p.name)}</a>`).join(", ")}.</p>` : "";
  const still = `<div class="lede"><h1>${escape(card.title)}</h1><p class="lead">${escape(card.blurb)}</p></div>\n${hero(fig, figure, route.route, card.title)}\n<noscript><p>${NEEDS.demo}</p></noscript>${reads}`;
  return shell(site, {
    route: route.route,
    name: card.title,
    description: card.blurb || card.title,
    body: `<div id="root"></div><div>${still}</div>`,
    type: "website",
    wide: true,
    bare: true,
    contents: [],
    controls: card.bar,
    image: picture(site, figure, route.route, fig),
    sheets: [site.asset("fonts.css"), ...[...entry.html.matchAll(SHEET)].map((found) => found[1]!)],
    head: `\n${(entry.html.match(CARRIED) ?? []).join("")}`,
  });
}

/* README */

function intro(site: Site, file: string, out: Output[]) {
  const text = read(file);
  const name = title(text);
  const html = `<h1 id="${escape(name.toLowerCase())}">${escape(name)}</h1>\n${md(text.replace(/^# .+\n/, ""), { math, link: links(site, file, out) })}`;
  return { name, lead: lede(text), html };
}

/* PAPERS */

type Paper = { slug: string; name: string; lead: string; date: string; revised: string; figure: string; shelf: string; body: string; file: string; href: string };

function papers(site: Site): Paper[] {
  const home = site.input("papers");
  return sheets(home.files)
    .map((file) => {
      const slug = file.slice(home.path.length + 1, -3);
      const { data, body } = front(read(file));
      return { slug, name: data.title ?? slug, lead: data.lead ?? lede(body), date: data.date ?? "", revised: data.revised ?? "", figure: data.figure || `paper-${slug}`, shelf: data.shelf ?? "", body, file, href: `${PAPERS.href}${slug}/` };
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
  const body = `${plate}\n${md(p.body, { math, lazy: true, link: links(site, p.file, out) })}`;
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
  out.push({ path: pageOf(route.route), bytes: shell(site, { route: route.route, name: p.name, description: p.lead, body, data, image: picture(site, p.figure, route.route, fig) }) });
  return out;
}

type Lane = { slug: string; blurb: string; name: string; md: string; published: string; revised: string; pdf: boolean; href: string };

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
      href: `${PAPERS.href}${slug}/`,
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
  const at = route.route.slice(1, -1);
  if (p.pdf) out.push({ path: `${at}/paper.pdf`, bytes: bytes(join(lane, "paper.pdf")) });
  out.push({ path: `${at}/paper.tex`, bytes: bytes(join(lane, "paper.tex")) });
  for (const file of walk(join(lane, "figures"))) out.push({ path: `${at}/figures/${file.slice(join(lane, "figures").length + 1)}`, bytes: bytes(file) });
  const when = [p.published && `First published ${p.published}`, p.revised && `revised ${p.revised}`].filter(Boolean).join(", ");
  const files = [p.pdf && `<a href="paper.pdf">PDF</a>`, `<a href="paper.tex">TeX</a>`].filter(Boolean).join(" · ");
  const avatar = pic(fig, `paper-${p.slug}`, route.route, p.name, "", "avatar");
  const plate = `<div class="plate paper">${avatar}<h1 id="${escape(p.slug)}">${escape(p.name)}</h1><p class="by">${escape(AUTHOR)}</p><p class="by">${escape(when)}</p></div>\n<p class="meta">${files}</p>`;
  const body = `${plate}\n${md(p.md.replace(/^# .+\n/, "").replace(AVATAR, ""), { math, lazy: true, link: links(site, join(lane, "README.md"), out) })}`;
  const data = {
    "@context": "https://schema.org",
    "@type": "ScholarlyArticle",
    headline: p.name,
    description: p.blurb || lede(p.md),
    url: root + route.route,
    image: `${root}/figures/paper-${p.slug}-dark.png`,
    author: { "@type": "Organization", name: AUTHOR },
    datePublished: p.published || undefined,
    dateModified: dated(p) || undefined,
    license: "https://creativecommons.org/licenses/by/4.0/",
  };
  out.push({ path: `${at}/index.html`, bytes: shell(site, { route: route.route, name: p.name, description: p.blurb || lede(p.md), body, data, image: picture(site, `paper-${p.slug}`, route.route, fig) }) });
  return out;
}

function paperIndex(site: Site, route: Route): Output[] {
  const out: Output[] = [];
  const fig = press(site, out);
  const top = intro(site, route.source as string, out);
  const cards = (list: { name: string; href: string }[]) => grid(dress(list.map(({ name, href }) => ({ name, href })), marks(DRESS), fig, route.route));
  const shelfNote = `<section><h2 id="shelf">The shelf</h2><p class="lead">The first editions, in LaTeX with a PDF each, deprecated: every paper is rewritten here in turn and the shelf is never edited.</p>${cards(DRESS.lanes)}</section>`;
  const body = `<article class="prose readme">${top.html}</article>\n${cards(DRESS.papers)}\n${DRESS.lanes.length ? shelfNote : ""}`;
  out.push({ path: pageOf(route.route), bytes: shell(site, { route: route.route, name: top.name, description: top.lead, body, type: "website", wide: true, bare: true, image: picture(site, PAPERS.figure, route.route, fig) }) });
  return out;
}

/* RESEARCH */

type Note = { file: string; name: string; md: string; home: boolean; topic: boolean; title: string; lead: string; figure: string; href: string };

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
    const home = file === INDEX;
    return { file, name, md, home, topic: false, title: title(md) || name, lead: lede(md), figure, href: home ? HUB : `${HUB}${name}/` };
  });
  const home = site.input("notes");
  const topics = sheets(home.files).map((source) => {
    const name = source.slice(home.path.length + 1, -3);
    const { data, body } = front(read(source));
    return { file: `notes/${name}.md`, name, md: body, home: false, topic: true, title: data.title ?? name, lead: data.lead ?? lede(body), figure: data.figure || `research-${name}`, href: `${NOTES.href}${name}/` };
  });
  return [...shared, ...topics].sort((a, b) => (a.home ? -1 : b.home ? 1 : a.name.localeCompare(b.name)));
}

const noteCards = (list: Note[], fig: Fig, route: string) => dress(list.map((n) => ({ name: n.title, href: n.href })), marks(DRESS), fig, route);

function researchIndex(site: Site, route: Route): Output[] {
  const n = route.data as Note;
  const out: Output[] = [];
  const fig = press(site, out);
  const lead = lede(n.md);
  const text = n.md.replace(/^# .+\n/, "");
  const cut = text.search(/^## /m);
  const link = links(site, route.source as string, out);
  const top = md(cut < 0 ? text : text.slice(0, cut), { math, link });
  const rest = cut < 0 ? "" : `\n<article class="prose readme">${md(text.slice(cut), { math, link })}</article>`;
  const doors = FOLDERS.map((one) => ({ name: one.name, href: one.href, figure: fig(one.figure, route.route), text: one.text }));
  const ledgers = noteCards(DRESS.notes.filter((one) => !one.home && !one.topic), fig, route.route);
  const body = `<article class="prose readme"><h1 id="research">Research</h1>\n${top}</article>\n${grid([...doors, ...ledgers])}${rest}`;
  out.push({ path: pageOf(route.route), bytes: shell(site, { route: route.route, name: "Research", description: lead, body, type: "website", wide: true, bare: true, image: picture(site, "site-research", route.route, fig) }) });
  return out;
}

function noteIndex(site: Site, route: Route): Output[] {
  const out: Output[] = [];
  const fig = press(site, out);
  const top = intro(site, route.source as string, out);
  const body = `<article class="prose readme">${top.html}</article>\n${grid(noteCards(DRESS.notes.filter((one) => one.topic), fig, route.route))}`;
  out.push({ path: pageOf(route.route), bytes: shell(site, { route: route.route, name: top.name, description: top.lead, body, type: "website", wide: true, bare: true, image: picture(site, NOTES.figure, route.route, fig) }) });
  return out;
}

function note(site: Site, route: Route): Output[] {
  const n = route.data as Note;
  const out: Output[] = [];
  const fig = press(site, out);
  const name = n.title;
  const lead = n.lead;
  const head = n.topic ? `<h1 id="${escape(n.name)}">${escape(name)}</h1>\n` : "";
  const used = new Set<string>();
  const prose = md(n.md, { math, lazy: true, link: links(site, route.source as string, out), widget: widgets(site, used) });
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
  const scripts = [...used].sort().map((name) => `/demos/${name}/widget.js`);
  out.push({ path: pageOf(route.route), bytes: shell(site, { route: route.route, name, description: lead, body, data, image: picture(site, n.figure, route.route, fig), scripts }) });
  return out;
}

/* CLAIMS */

type Claim = { slug: string; title: string; md: string; file: string; href: string };

const TAGS = ["Proved", "Verified", "Conjecture", "Refuted"];
const CLAIM = /^- (\d{4}-\d{2}-\d{2}) \[(Proved|Verified|Conjecture|Refuted)\] (.+)$/gm;
const LINE = /<li>(\d{4}-\d{2}-\d{2}) \[(Proved|Verified|Conjecture|Refuted)\] /g;

function claims(site: Site): Claim[] {
  const home = site.input("claims");
  return sheets(home.files)
    .map((file) => {
      const slug = file.slice(home.path.length + 1, -3);
      const md = read(file);
      return { slug, title: title(md) || slug, md, file, href: `${CLAIMS.href}${slug}/` };
    })
    .sort((a, b) => a.title.localeCompare(b.title));
}

function tally(md: string) {
  const counts = TAGS.map(() => 0);
  let last = "";
  for (const [, date, tag] of md.matchAll(CLAIM)) {
    counts[TAGS.indexOf(tag!)]! += 1;
    if (date! > last) last = date!;
  }
  return { counts, last, total: counts.reduce((a, b) => a + b, 0) };
}

function claimIndex(site: Site, route: Route): Output[] {
  const list = route.data as [string, string, number[], string][];
  const out: Output[] = [];
  const fig = press(site, out);
  const top = intro(site, route.source as string, out);
  const sums = TAGS.map((_, i) => list.reduce((a, row) => a + row[2][i]!, 0));
  const head = `<tr><th>File</th>${TAGS.map((tag) => `<th class="right">${tag}</th>`).join("")}<th>Newest</th></tr>`;
  const rows = list.map(([href, name, counts, last]) => `<tr><td><a href="${href}">${escape(name)}</a></td>${counts.map((n) => `<td class="right num">${n}</td>`).join("")}<td class="num"><time>${last}</time></td></tr>`);
  const foot = `<tr><th>${list.length} files, ${sums.reduce((a, b) => a + b, 0)} claims</th>${sums.map((n) => `<th class="right num">${n}</th>`).join("")}<th></th></tr>`;
  const table = `<h2 id="files">Every claims file</h2>\n<div class="table"><table><thead>${head}</thead><tbody>${rows.join("")}</tbody><tfoot>${foot}</tfoot></table></div>`;
  const body = `${hero(fig, CLAIMS.figure, route.route, top.name)}\n${top.html}\n${table}`;
  out.push({ path: pageOf(route.route), bytes: shell(site, { route: route.route, name: top.name, description: top.lead, body, type: "website", image: picture(site, CLAIMS.figure, route.route, fig) }) });
  return out;
}

function claim(site: Site, route: Route): Output[] {
  const c = route.data as { slug: string; title: string };
  const file = route.source as string;
  const text = read(file);
  const out: Output[] = [];
  const fig = press(site, out);
  const { counts, last, total } = tally(text);
  const lead = `${total} dated claim${total === 1 ? "" : "s"} on ${c.title}, each with its tag and its witness${last ? `; newest ${last}` : ""}.`;
  const html = md(text.replace(/^# .+\n/, ""), { math, link: links(site, file, out) }).replace(
    LINE,
    (_, date: string, tag: string) => `<li data-date="${date}" data-tag="${tag.toLowerCase()}"><time>${date}</time> <b class="tag ${tag.toLowerCase()}">${tag}</b> `,
  );
  const chips = ["", ...TAGS].map((tag, i) => `<button type="button" data-tag="${tag.toLowerCase()}"${tag ? "" : ' class="on"'}>${tag || "All"} <span>${tag ? counts[i - 1] : total}</span></button>`).join("");
  const bar = `<form class="filter" onsubmit="return false">${chips}<select hidden aria-label="Topic"><option value=""></option></select><label>Since <input type="date" aria-label="Since"></label><output>${total} claims</output></form>`;
  const body = `<h1 id="${escape(c.slug)}">${escape(c.title)}</h1>\n<p class="meta"><a href="${CLAIMS.href}">${CLAIMS.name}</a> · ${escape(lead)}</p>\n${bar}\n<section class="claims" data-slug="${escape(c.slug)}">${html}</section>\n${claimsScript()}`;
  const data = {
    "@context": "https://schema.org",
    "@type": "Article",
    headline: c.title,
    description: lead,
    url: root + route.route,
    image: `${root}/figures/${CLAIMS.figure}-dark.png`,
    author: { "@type": "Organization", name: AUTHOR },
  };
  out.push({ path: pageOf(route.route), bytes: shell(site, { route: route.route, name: `${c.title} claims`, description: lead, body, data, image: picture(site, CLAIMS.figure, route.route, fig) }) });
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
    const body = `<div class="lede"><h1 id="blog">Blog</h1><p class="lead">${escape(BLOG_LEAD)}</p></div>\n${grid(dress(leaf.posts.map((one) => ({ name: one.title, href: one.route })), marks(DRESS), fig, leaf.route))}`;
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

const pageRoute = (slug: string) => `/${slug}/`;

function page(site: Site, route: Route): Output[] {
  const source = route.source as string;
  const slug = basename(source, ".md");
  const { data, body } = front(read(source));
  const name = data.title ?? slug;
  const lead = data.lead ?? lede(body);
  const out: Output[] = [];
  const fig = press(site, out);
  const open = data.figure ? `${hero(fig, data.figure, route.route, name)}\n` : "";
  const head = `<div class="lede"><h1 id="${escape(slug)}">${escape(name)}</h1><p class="lead">${escape(lead)}</p></div>`;
  const act = data.button && data.link ? `\n<p><a class="button primary" href="${escape(data.link)}">${escape(data.button)}</a></p>` : "";
  const own = data.figure || FIXED[route.route];
  const image = own ? picture(site, own, route.route, fig) : OG;
  const html = shell(site, { route: route.route, name, description: lead, body: `${open}${head}\n${md(body, { math, lazy: Boolean(open), link: links(site, source, out) })}${act}`, type: "website", image });
  out.push({ path: pageOf(route.route), bytes: html });
  return out;
}

/* APPS */

type App = { id: string; title: string; kind: string };

const SETTINGS = { lead: "Theme, font, tint and saver, kept in this browser.", needs: "Settings need JavaScript. Without it the site follows the system theme." };

const APP_KIND: Record<string, string> = { settings: "settings" };

const appRoute = (id: string) => `/${id}/`;

function settings(site: Site, route: Route): Output[] {
  const app = route.data as App;
  const body = `<div class="lede"><h1 id="settings">${escape(app.title)}</h1><p class="lead">${escape(SETTINGS.lead)}</p></div>\n<noscript><p>${SETTINGS.needs}</p></noscript>\n${renderToStaticMarkup(h(Settings))}`;
  return [{ path: pageOf(route.route), bytes: shell(site, { route: route.route, name: app.title, description: SETTINGS.lead, body, type: "website" }) }];
}

type Dress = { lanes: Lane[]; papers: Paper[]; notes: Note[]; posts: Post[] };

type Mark = { figure: string; text: string; dates?: string[] };

let DRESS: Dress = { lanes: [], papers: [], notes: [], posts: [] };

const FIXED: Record<string, string> = {
  "/": "site-home",
  "/math/": "site-math",
  "/git/": "site-code",
  "/about/": "site-icon",
  "/contact/": "site-contact",
  "/donate/": "site-donate",
};

function marks(data: Dress): Map<string, Mark> {
  const map = new Map<string, Mark>();
  for (const p of data.papers) map.set(p.href, { figure: p.figure, text: p.lead, dates: marked(p) });
  for (const p of data.lanes) map.set(p.href, { figure: `paper-${p.slug}`, text: p.blurb, dates: stamps(p) });
  for (const n of data.notes) if (!n.home) map.set(n.href, { figure: n.figure, text: n.lead });
  for (const p of data.posts) map.set(`/blog/${p.slug}/`, { figure: p.figure, text: p.lead, dates: [p.date] });
  return map;
}

function dress(nodes: Node[], map: Map<string, Mark>, fig: Fig, route: string): Node[] {
  return nodes.map((node) => {
    const mark = node.href ? map.get(node.href) : undefined;
    if (!mark) return node;
    return { ...node, figure: fig(mark.figure, route), text: mark.text || undefined, dates: mark.dates };
  });
}

/* MENU */

type Door = { name: string; href?: string; figure?: string; nodes?: Door[] };

const PLAIN = "site-page";

const shipped = (href: string) => FIXED[href] ?? [...FOLDERS, ...DOORS].find((one) => one.href === href)?.figure;

const folded = (name: string) => ({ Research: shipped(HUB), Apps: "site-apps" })[name];

function filled(fills: Record<string, Door[]>): Door[] {
  return (SITE.tree as Door[]).map((node) => (node.href ? node : { ...node, nodes: node.nodes ?? fills[node.name.toLowerCase()] ?? [] })).filter((node) => node.href || node.nodes!.length);
}

function menu(site: Site, route: Route): Output[] {
  const lead = "Every door of mrly.net.";
  const out: Output[] = [];
  const fig = press(site, out);
  const icon = (door: Door) => door.figure ?? (door.href ? shipped(door.href) : folded(door.name)) ?? PLAIN;
  const wear = (door: Door): Node => ({ name: door.name, href: door.href, figure: fig(icon(door), route.route), nodes: door.nodes?.map(wear) });
  const list = renderToStaticMarkup(h(Menu, { tree: (route.data as Door[]).map(wear) }));
  const body = `<div class="hero"><h1><span role="img" aria-label="${escape(SITE.title)}">${WORD}</span></h1><p>${escape(lead)}</p></div>\n${list}`;
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
<noscript><p>${NEEDS.stats}</p></noscript>
<section><h2 id="cloud">Cloud</h2><div data-stats="cloud"><p class="fine">Loading</p></div></section>
<section><h2 id="errors">Errors</h2><div data-stats="errors"><p class="fine">Loading</p></div></section>`;
  return [{ path: "stats/index.html", bytes: shell(site, { route: route.route, name: "Stats", description: WATCH, body, type: "website", scripts: [site.asset("stats.js")] }) }];
}

const MISSION = SITE.tagline;

const DOORS = [
  WIKI,
  { name: "Demos", href: "/demos/", figure: "site-demos", text: "Browser pages that draw a design and the numbers around it, live." },
  CLAIMS,
  PAPERS,
  { name: "Research", href: HUB, figure: "site-research", text: "The whole tree in one place: the wiki, the notes, the claims, the papers and the law they follow." },
];

type Newest = { date: string; tag: string; text: string; title: string; file: string; href: string };

function newest(list: Claim[]): Newest | null {
  let best: Newest | null = null;
  for (const c of list)
    for (const [, date, tag, text] of c.md.matchAll(CLAIM))
      if (!best || date! > best.date) best = { date: date!, tag: tag!, text: text!, title: c.title, file: c.file, href: c.href };
  return best;
}

function home(site: Site, route: Route): Output[] {
  const { lanes: list, papers: fresh, posts: posted, claim } = route.data as { lanes: Lane[]; papers: Paper[]; posts: Post[]; claim: Newest | null };
  const out: Output[] = [];
  const fig = press(site, out);
  const latestClaim = claim
    ? `<section><h2 id="newest">Newest claim</h2><p class="lead"><time>${claim.date}</time> <b class="chip ${claim.tag.toLowerCase()}">${claim.tag}</b> ${inline(claim.text, { math, link: links(site, claim.file, out) })}</p><p class="lead">From <a href="${claim.href}">${escape(claim.title)}</a>. <a href="${CLAIMS.href}">Every claim, dated and tagged</a>.</p></section>`
    : "";
  const latest = [...fresh.map((p) => ({ name: p.name, href: p.href, at: p.revised || p.date })), ...list.map((p) => ({ name: p.name, href: p.href, at: dated(p) }))]
    .map((p, i) => ({ p, i }))
    .sort((a, b) => (a.p.at < b.p.at ? 1 : a.p.at > b.p.at ? -1 : a.i - b.i))
    .slice(0, 3)
    .map(({ p }) => ({ name: p.name, href: p.href }));
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
  const body = `<div class="lede"><h1 id="lost">Nothing here</h1><p class="lead">That page does not exist. The <a href="/menu/">Menu</a> holds every door of this site, starting with ${doors.slice(0, -1).join(", ")} and ${doors[doors.length - 1]}.</p></div>`;
  return [{ path: "404.html", bytes: shell(site, { route: route.route, name: "Nothing here", description: "That page does not exist.", body, type: "website", bare: true }) }];
}

/* WIKI */

type Entry = { slug: string; name: string; lead: string; figure: string; live: boolean; needs: string[]; body: string; file: string; href: string };

type Concept = Omit<Entry, "body" | "file"> & { before: { slug: string; name: string }[]; after: { slug: string; name: string }[] };

const wikiRoute = (slug: string) => `${WIKI.href}${slug}/`;

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
  const list = sheets(home.files).map((file) => {
    const slug = file.slice(home.path.length + 1, -3);
    const { data, body } = front(read(file));
    const needs = (data.prerequisites ?? "").split(",").map((s: string) => s.trim()).filter(Boolean);
    const figure = data.figure || `wiki-${slug}`;
    return { slug, name: data.title ?? slug, lead: data.lead ?? lede(body), figure, live: LIVE.some((one) => one.name === figure), needs, body, file, href: wikiRoute(slug) };
  });
  const known = new Set(list.map((e) => e.slug));
  for (const e of list) for (const need of e.needs) if (!known.has(need)) throw new Error(`site: research/wiki/${e.slug}.md needs ${need}, and research/wiki/${need}.md does not exist`);
  return ordered(list);
}

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
    return `<figure class="widget" data-demo="${name}" data-view="${view}"><div class="mount"></div><noscript><p>${NEEDS.widget}</p></noscript><figcaption>${caption}</figcaption></figure>`;
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
  const prose = md(front(read(file)).body, { math, lazy: true, link: links(site, file, out), widget: widgets(site, used) });
  const body = `${hero(fig, c.figure, route.route, c.name, c.live)}\n${head}${before}\n${prose}${after}`;
  const data = {
    "@context": "https://schema.org",
    "@type": "Article",
    headline: c.name,
    description: c.lead,
    url: root + route.route,
    image: `${root}/figures/${c.figure}-dark.png`,
    author: { "@type": "Organization", name: AUTHOR },
  };
  const scripts = [...[...used].sort().map((name) => `/demos/${name}/widget.js`), ...(c.live ? [GLUE_SRC] : [])];
  out.push({ path: pageOf(route.route), bytes: shell(site, { route: route.route, name: c.name, description: c.lead, body, data, image: picture(site, c.figure, route.route, fig), scripts }) });
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
  const top = intro(site, route.source as string, out);
  const names = new Map(list.map((e) => [e.slug, e.name]));
  const deep = depths(list);
  const rows = new Map<number, Leaf_[]>();
  for (const e of list) rows.set(deep.get(e.slug)!, [...(rows.get(deep.get(e.slug)!) ?? []), e]);
  const sections = [...rows.keys()].sort((a, b) => a - b).map((n) => `<section><h2 id="step-${n}">${step(n)}</h2><div class="gallery grid graph">${rows.get(n)!.map((e) => tile(fig, route.route, e, names)).join("")}</div></section>`);
  const order = list.length ? `\n<p>Every page needs only the pages above it.</p>` : "";
  const body = `${hero(fig, WIKI.figure, route.route, top.name)}\n<article class="prose readme">${top.html}${order}</article>\n${sections.join("\n")}`;
  out.push({ path: pageOf(route.route), bytes: shell(site, { route: route.route, name: top.name, description: top.lead, body, type: "website", wide: true, bare: true, image: picture(site, WIKI.figure, route.route, fig) }) });
  return out;
}

/* MATH */

const STANDARD = "A design is one string: one JSON object per named thing, and every other name a view cut from it.";

function standard(site: Site, route: Route): Output[] {
  const out: Output[] = [];
  const fig = press(site, out);
  const file = route.source as string;
  const body = `${hero(fig, "site-math", route.route, "MrlyMath")}\n<h1 id="math">MrlyMath</h1><p class="lead">${escape(STANDARD)}</p>\n${md(read(file).replace(/^# .+\n/, ""), { math, lazy: true, link: links(site, file, out) })}`;
  out.push({ path: "math/index.html", bytes: shell(site, { route: route.route, name: "Math", description: STANDARD, body, type: "website", image: picture(site, "site-math", route.route, fig) }) });
  return out;
}

/* COLLECT */

const counts = { papers: 0, research: 0, blog: 0, demos: 0, wiki: 0 };

let MAP: Lists = { wiki: [], notes: [], claims: [], papers: [], lanes: [], posts: [], math: [], pages: [] };

const folder = (site: Site, input: string, one: { name: string; href: string }, kind: string, files: string[], data?: unknown): Route => {
  const readme = join(site.input(input).path, INDEX);
  return { route: one.href, kind, name: one.name, data, source: readme, inputs: [readme, ...files] };
};

async function collect(site: Site) {
  SHELF = await shelf();
  const made = await ensureFigures();
  if (made.placed || made.removed) console.log(said(made));
  LIVE = live();
  const paperList = papers(site);
  const fresh = new Set(paperList.map((p) => p.slug));
  const laneList = lanes().filter((p) => !fresh.has(p.slug));
  const noteList = notes(site);
  const postList = posts(site);
  const demoList = cards(site);
  const claimList = claims(site);
  const wikiList = wiki(site);
  readers(demoList, [
    ...noteList.filter((n) => !n.home).map((n) => ({ name: n.title, href: n.href, md: n.md })),
    ...paperList.map((p) => ({ name: p.name, href: p.href, md: p.body })),
    ...wikiList.map((e) => ({ name: e.name, href: e.href, md: e.body })),
  ]);
  counts.papers = laneList.length + paperList.length;
  counts.research = noteList.length;
  counts.blog = postList.length;
  counts.demos = demoList.length;
  counts.wiki = wikiList.length;
  const listed = site.input("apps").files[0];
  const appList = listed ? (JSON.parse(read(listed)) as App[]) : [];
  const pages = site.input("pages");
  const pageList = pages.files.map((source) => {
    const slug = basename(source, ".md");
    const { data } = front(read(source));
    return { slug, source, name: data.title ?? slug, figure: data.figure || undefined };
  });
  const fills = {
    pages: pageList.map((p) => ({ name: p.name, href: pageRoute(p.slug), figure: p.figure })),
    apps: appList.map((one) => ({ name: one.title, href: appRoute(one.id) })),
    elsewhere: [...SITE.socials, { name: SITE.contact, href: `mailto:${SITE.contact}` }],
  };
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
  for (const one of appList) {
    if (!Object.hasOwn(APP_KIND, one.id)) throw new Error(`site: apps/apps.json names ${one.id}, and no page draws it`);
    routes.push({ route: appRoute(one.id), kind: APP_KIND[one.id]!, name: one.title, data: one, inputs: [listed!] });
  }
  if (wikiList.length) routes.push(folder(site, "wiki", WIKI, "wiki", wikiList.map((e) => e.file), wikiList.map((e) => [e.slug, e.name, e.lead, e.figure, e.needs])));
  for (const [c, file] of concepts(wikiList)) {
    routes.push({ route: c.href, kind: "concept", name: c.name, data: c, source: file, inputs: [file, ...embeds(site, read(file))] });
  }
  const names = site.input("names");
  if (!names.missing) routes.push({ route: "/math/", kind: "math", name: "Math", source: names.files[0]!, inputs: names.files });
  routes.push(...demoRoutes(site, demoList));
  if (laneList.length || paperList.length) {
    const index = join(SHELF, INDEX);
    const data = { lanes: laneList, papers: paperList.map((p) => [p.slug, p.name, p.lead, p.date, p.revised, p.figure]) };
    routes.push({ ...folder(site, "papers", PAPERS, "papers", [index, ...paperList.map((p) => p.file)], data), urls: [{ route: PAPERS.href, name: PAPERS.name, source: index }] });
    for (const p of paperList) {
      routes.push({ route: p.href, kind: "written", name: p.name, data: p, source: p.file, inputs: [p.file] });
    }
    for (const p of laneList) {
      const lane = join(SHELF, p.slug);
      routes.push({ route: p.href, kind: "paper", name: p.name, data: p, source: lane, inputs: [lane] });
    }
  }
  const notesHome = site.input("research").path;
  if (claimList.length) {
    const rows = claimList.map((c) => {
      const { counts, last } = tally(c.md);
      return [c.href, c.title, counts, last];
    });
    routes.push(folder(site, "claims", CLAIMS, "claims", claimList.map((c) => c.file), rows));
    for (const c of claimList) routes.push({ route: c.href, kind: "claim", name: c.title, data: { slug: c.slug, title: c.title }, source: c.file, inputs: [c.file] });
  }
  const topics = noteList.filter((n) => n.topic);
  if (topics.length) routes.push(folder(site, "notes", NOTES, "notes", topics.map((n) => join(notesHome, n.file))));
  for (const n of noteList) {
    const source = join(notesHome, n.file);
    routes.push({
      route: n.href,
      kind: n.home ? "research" : "note",
      name: n.home ? "Research" : n.title,
      data: n,
      source,
      inputs: n.home ? noteList.filter((one) => !one.topic).map((one) => join(notesHome, one.file)) : [source, ...embeds(site, n.md)],
    });
  }
  for (const p of pageList) routes.push({ route: pageRoute(p.slug), kind: "page", name: p.name, source: p.source, inputs: [p.source] });
  DRESS = { lanes: laneList, papers: paperList, notes: noteList, posts: postList };
  MAP = { wiki: wikiList, notes: noteList, claims: claimList, papers: paperList, lanes: laneList, posts: postList, math: names.files, pages: pages.files };
  routes.push({ route: "/menu/", kind: "menu", name: "Menu", data: filled(fills) });
  routes.push({ route: "/cart/", kind: "cart", name: "Cart", hidden: true });
  routes.push({ route: "/stats/", kind: "stats", name: "Stats", hidden: true });
  routes.push({ route: "/404.html", kind: "missing", name: "Nothing here", hidden: true });
  return { routes };
}

/* RENDER */

const KINDS: Record<string, (site: Site, route: Route) => Output[] | Promise<Output[]>> = {
  home,
  gallery,
  papers: paperIndex,
  paper,
  written,
  notes: noteIndex,
  note,
  research: researchIndex,
  claims: claimIndex,
  claim,
  page,
  settings,
  menu,
  cart,
  stats,
  missing,
  math: standard,
  wiki: wikiIndex,
  concept,
};

function draw(site: Site, route: Route) {
  const fn = KINDS[route.kind ?? ""];
  if (!fn) throw new Error(`site: no template for ${route.route}`);
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
  return git ? (site.serves.get(join(git.root, path)) ?? null) : null;
}

/* SPEC */

const VIEWER = join(org, "lib", "git.js");

const MANIFEST = process.env.MRLY_DIST ? join(dist, ".manifest.json") : ".cache/manifest.json";

export const counted = () => ({ ...counts });

export const spec: Spec = {
  root: org,
  out: dist,
  templates: ["scripts/site.ts"],
  inline: inlineScripts(SITE.prefix),
  icons: { rows: glyphs(1), svg: logoSvg(1, "#000000", "#ffffff") },
  collect,
  render: draw,
  globals: extras,
  llms: (site) => sections(site, MAP),
  git: {
    page: shell,
    entry: VIEWER,
    served,
  },
  spa: {
    entries: (site) => [...demoEntries(site), VIEWER],
    plugins: () => [mrlyjs(LIVE)],
    place: demoPlace,
    page: (site, route, shell, out) => (route.kind === "demo" ? demoPage(site, route, shell, out) : gitShell(site, route, shell, spec)),
  },
  blog: {
    page: blogPage,
    md: (site, text, from, out) => md(text, { math, lazy: true, link: links(site, from, out) }),
  },
};

if (import.meta.main) {
  process.chdir(org);
  const done = await build(spec, { manifest: MANIFEST, force: process.argv.includes("--force") });
  const site = done.site;
  const code = site.routes.filter((one) => one.kind === "raw").length;
  console.log(
    `site: ${site.routes.length} routes, ${counts.demos} demo shells, ${counts.papers} papers, ${counts.research} research pages, ${counts.blog} posts, ${code} code files, ${done.rendered} rendered, ${done.written} files written, ${done.removed} removed`,
  );
}
