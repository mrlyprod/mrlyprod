import { existsSync, readFileSync, readdirSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import { createElement as h } from "react";
import { renderToStaticMarkup } from "react-dom/server";
import katex from "katex";
import { build, bytes, walk, type Output, type Route, type Site, type Spec } from "../../../pkgs/js/mrlyjs/ssg/build.ts";
import { isGit, link as gitLink } from "../../../pkgs/js/mrlyjs/git/git.ts";
import { escape, front, plain, render as md, summary, title } from "../lib/md.js";
import { tree } from "../lib/tree.js";
import { logoSvg } from "../lib/logo.js";
import { glyphSvg } from "../../../pkgs/js/mrlyjs/ui/font.js";
import { Shell } from "../../../pkgs/js/mrlyjs/ui/chrome.jsx";
import kit from "../lib/site.js";
import { shelf } from "./shelf.ts";

const org = resolve(import.meta.dir, "..");
const dist = join(org, "dist");
const BLOG = join(org, "blog");
const postFile = (slug: string) => join(BLOG, `${slug}.md`);
const root = (process.env.MRLY_SITE ?? kit.root).replace(/\/$/, "");
const AUTHOR = "Carlo Mitchener";
const GITHUB = "https://github.com/mrlyprod/mrlyprod/tree/main";
const LIST = /^- \[([^\]]+)\]\([^)]*\) - (.+)$/gm;
const HEADING = /<h([23]) id="([^"]+)">(.*?)<\/h\1>/g;
const AVATAR = /^!\[avatar\]\(figures\/avatar\.png\)\n?/m;
const WORD = glyphSvg(kit.title.toUpperCase());
const ICONS = [
  `<link rel="icon" href="/favicon.svg" type="image/svg+xml">`,
  `<link rel="apple-touch-icon" href="/apple-touch-icon.png">`,
  `<link rel="manifest" href="/manifest.webmanifest">`,
].join("\n");

const read = (p: string) => readFileSync(p, "utf8");
const math = (tex: string, display: boolean) =>
  katex.renderToString(tex, { output: "mathml", throwOnError: false, displayMode: display });
const untag = (html: string) =>
  html.replace(/<[^>]+>/g, "").replace(/&amp;/g, "&").replace(/&lt;/g, "<").replace(/&gt;/g, ">").replace(/&quot;/g, '"');
const brand = (name: string) => (name === kit.title ? name : `${name} · ${kit.title}`);

/* FIGURES */

function press(site: Site, out: Output[]) {
  const home = site.input("figures").path;
  return (name: string, route: string) => {
    const file = join(home, `${name}.png`);
    if (!existsSync(file)) throw new Error(`site: ${name}.png missing from ${relative(org, home)} for ${route}; draw it with bun run figures`);
    const path = `figures/${name}.png`;
    if (!out.some((item) => item.path === path)) out.push({ path, bytes: bytes(file) });
    return `/${path}`;
  };
}

type Fig = ReturnType<typeof press>;

const hero = (fig: Fig, name: string, route: string, alt: string) =>
  `<figure class="opener"><img src="${fig(name, route)}" alt="${escape(alt)}" width="1024" height="1024"></figure>`;

const card = (fig: Fig, href: string, name: string, alt: string, text: string, dates: string[] = []) =>
  `<a class="tile" href="${href}"><img src="${fig(name, href)}" alt="${escape(alt)}" width="1024" height="1024" loading="lazy"><h2>${escape(alt)}</h2><p>${escape(text)}</p>${dates.length ? `<p class="dates">${dates.map((d) => `<span>${escape(d)}</span>`).join("")}</p>` : ""}</a>`;

/* HEAD */

function meta(route: string, name: string, description: string, type: string) {
  const url = root + route;
  return [
    `<link rel="canonical" href="${url}">`,
    `<meta name="description" content="${escape(description)}">`,
    `<meta property="og:title" content="${escape(name)}">`,
    `<meta property="og:description" content="${escape(description)}">`,
    `<meta property="og:url" content="${url}">`,
    `<meta property="og:type" content="${type}">`,
    `<meta property="og:site_name" content="${escape(kit.title)}">`,
    `<meta property="og:image" content="${root}/og.png">`,
    `<meta property="og:image:width" content="1200">`,
    `<meta property="og:image:height" content="630">`,
    `<meta name="twitter:card" content="summary_large_image">`,
    `<meta name="twitter:image" content="${root}/og.png">`,
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
};

function shell(site: Site, leaf: Leaf) {
  const { route, name, description, body, type = "article", wide = false, bare = false, code = false, data } = leaf;
  const article = h(bare ? "div" : "article", { className: bare ? undefined : "prose", dangerouslySetInnerHTML: { __html: body } });
  const main = renderToStaticMarkup(h(Shell, { route, tree: site.nav, contents: headings(body), wide }, article));
  const ld = data ? `<script type="application/ld+json">${JSON.stringify(data)}</script>\n` : "";
  return `<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>${escape(brand(name))}</title>
${meta(route, name, description, type)}
<link rel="stylesheet" href="${site.asset("tokens.css")}">
<link rel="stylesheet" href="${site.asset("base.css")}">
<link rel="stylesheet" href="${site.asset("chrome.css")}">
<link rel="stylesheet" href="/pages.css">
${code ? `<link rel="stylesheet" href="${site.asset("code.css")}">\n<link rel="stylesheet" href="${site.asset("seti/seti.css")}">\n` : ""}${ld}<script type="module" src="${site.asset("chrome.js")}"></script>
</head>
<body>
${main}
</body>
</html>
`;
}

/* DEMOS */

type Card = { name: string; title: string; blurb: string };

function shelves(site: Site) {
  const data = (site.pages ?? {}) as { pages?: { name?: string; title?: string; blurb?: string }[] };
  const titles = new Map<string, string>();
  const blurbs = new Map<string, string>();
  for (const row of data.pages ?? []) {
    if (!row?.name) continue;
    if (row.title) titles.set(row.name, row.title);
    if (row.blurb) blurbs.set(row.name, plain(row.blurb));
  }
  const readme = read(join(org, "README.md"));
  for (const m of readme.matchAll(LIST)) if (!blurbs.has(m[1])) blurbs.set(m[1], plain(m[2]));
  const lead = readme.match(/^- (.+)$/m);
  blurbs.set("", plain(lead ? lead[1] : kit.title));
  titles.set("", "Demos");
  return { titles, blurbs };
}

function demoNames(site: Site) {
  const home = site.input("demos").path;
  return site
    .input("demos")
    .files.filter((f) => f.endsWith("/index.html"))
    .map((f) => dirname(f).slice(home.length + 1))
    .sort((a, b) => (a === "" ? -1 : b === "" ? 1 : a.localeCompare(b)));
}

const demoRoute = (name: string) => (name ? `/demos/${name}/` : "/demos/");

function demoGroup(site: Site): Route {
  const { titles, blurbs } = shelves(site);
  const home = site.input("demos").path;
  const list: Card[] = demoNames(site).map((name) => ({ name, title: titles.get(name) ?? name, blurb: blurbs.get(name) ?? "" }));
  const inputs = ["demos", "lib", "pkg", "ui"].flatMap((one) => site.input(one).files);
  return {
    route: "/demos/",
    kind: "demos",
    name: "Demos",
    data: list,
    inputs,
    urls: list.map((d) => ({ route: demoRoute(d.name), name: d.title })),
  };
}

function seo(html: string, card: Card) {
  const route = demoRoute(card.name);
  const found = html.match(/<title>([^<]*)<\/title>/);
  const name = found ? untag(found[1]) : card.title;
  const tags = meta(route, name, card.blurb || name, "website");
  const block = `<title>${escape(brand(name))}</title>\n${tags}`;
  return found ? html.replace(found[0], block) : html.replace("<head>", `<head>\n${block}`);
}

async function demos(site: Site, route: Route): Promise<Output[]> {
  const list = route.data as Card[];
  const home = site.input("demos").path;
  const built = await Bun.build({
    entrypoints: list.map((d) => join(home, d.name, "index.html")),
    root: org,
    splitting: true,
    minify: true,
    define: { "process.env.NODE_ENV": '"production"' },
    naming: { chunk: "lib-[hash].[ext]", asset: "[name]-[hash].[ext]" },
  });
  if (!built.success) throw new Error(`site: the demos failed to bundle\n${built.logs.join("\n")}`);
  const shells = new Map(list.map((d) => [`${demoRoute(d.name).slice(1)}index.html`, d]));
  const out: Output[] = [];
  for (const item of built.outputs) {
    const path = item.path.replace(/^\.\//, "");
    const card = shells.get(path);
    out.push({ path, bytes: card ? seo(await item.text(), card) : new Uint8Array(await item.arrayBuffer()) });
  }
  return out;
}

/* PAPERS */

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
  const avatar = `<img class="avatar" src="${fig(`paper-${p.slug}`, route.route)}" alt="${escape(p.name)}" width="1024" height="1024">`;
  const plate = `<div class="plate paper">${avatar}<h1 id="${escape(p.slug)}">${escape(p.name)}</h1><p class="by">${escape(AUTHOR)}</p><p class="by">${escape(when)}</p></div>\n<p class="meta">${files}</p>`;
  const body = `${plate}\n${md(p.md.replace(/^# .+\n/, "").replace(AVATAR, ""), { math })}`;
  const data = {
    "@context": "https://schema.org",
    "@type": "ScholarlyArticle",
    headline: p.name,
    description: p.blurb || summary(p.md),
    url: root + route.route,
    image: `${root}/figures/paper-${p.slug}.png`,
    author: { "@type": "Person", name: AUTHOR },
    datePublished: p.published || undefined,
    dateModified: dated(p) || undefined,
    license: "https://creativecommons.org/licenses/by/4.0/",
  };
  out.push({ path: `${at}/index.html`, bytes: shell(site, { route: route.route, name: p.name, description: p.blurb || summary(p.md), body, data }) });
  return out;
}

function paperIndex(site: Site, route: Route): Output[] {
  const list = route.data as Lane[];
  const out: Output[] = [];
  const fig = press(site, out);
  const cards = list.map((p) => card(fig, `/papers/${p.slug}/`, `paper-${p.slug}`, p.name, p.blurb, stamps(p)));
  const lead = summary(read(join(SHELF, "README.md")));
  const body = `<div class="lede"><h1 id="papers">Papers</h1><p class="lead">${escape(lead)}</p></div>\n<div class="gallery wrap">\n${cards.join("\n")}\n</div>`;
  out.push({ path: "papers/index.html", bytes: shell(site, { route: route.route, name: "Papers", description: lead, body, type: "website", wide: true, bare: true }) });
  return out;
}

/* RESEARCH */

function researchLink(url: string) {
  if (/^(https?:|mailto:|#)/.test(url)) return url;
  let m: RegExpMatchArray | null;
  if ((m = url.match(/^([A-Za-z0-9_-]+)\.md(#.*)?$/))) return (m[1] === "README" ? "/research/" : `/research/${m[1]}/`) + (m[2] ?? "");
  if ((m = url.match(/^\.\.\/demos\/([^/#?]+)/))) return `/demos/${m[1]}/`;
  if ((m = url.match(/^lab\/(.*)$/))) return `${GITHUB}/research/lab/${m[1]}`;
  if ((m = url.match(/^\.\.\/crates\/(.*)$/))) return `${GITHUB}/crates/${m[1]}`;
  if ((m = url.match(/^figures\/(.*)$/))) return `/research/figures/${m[1]}`;
  return url;
}

type Note = { file: string; name: string; md: string; home: boolean };

const SHARED = new Set(["DISCOVERIES", "REFS"]);

function notes(site: Site): Note[] {
  const dir = site.input("research");
  if (dir.missing) {
    console.warn(`site: no research tree at ${relative(org, dir.path)}, research skipped`);
    return [];
  }
  return dir.files
    .map((source) => {
      const file = source.slice(dir.path.length + 1);
      return { file, name: file.slice(0, -3), md: read(source), home: file === "README.md" };
    })
    .sort((a, b) => (a.home ? -1 : b.home ? 1 : a.name.localeCompare(b.name)));
}

function note(site: Site, route: Route): Output[] {
  const n = route.data as Note;
  const out: Output[] = [];
  const fig = press(site, out);
  out.push({ path: `research/${n.file}`, bytes: n.md });
  const name = title(n.md) || n.name;
  const lead = summary(n.md);
  const which = n.home || SHARED.has(n.name) ? "research-index" : `research-${n.name}`;
  const body = `${hero(fig, which, route.route, name)}\n${md(n.md, { math, link: researchLink })}`;
  const data = {
    "@context": "https://schema.org",
    "@type": "Article",
    headline: name,
    description: lead,
    url: root + route.route,
    image: `${root}/figures/${which}.png`,
    author: { "@type": "Person", name: AUTHOR },
  };
  const at = n.home ? "research/index.html" : `research/${n.name}/index.html`;
  out.push({ path: at, bytes: shell(site, { route: route.route, name, description: lead, body, type: n.home ? "website" : "article", data }) });
  return out;
}

/* BLOG */

type Post = { slug: string; name: string; date: string; lead: string; figure: string; body: string };

function posts(): Post[] {
  if (!existsSync(BLOG)) return [];
  return readdirSync(BLOG)
    .filter((f) => f.endsWith(".md"))
    .map((file) => {
      const { data, body } = front(read(join(BLOG, file)));
      const slug = file.slice(0, -3);
      return { slug, name: data.title ?? slug, date: data.date ?? "", lead: data.lead ?? summary(body), figure: data.figure || `blog-${slug}`, body };
    })
    .sort((a, b) => (a.date < b.date ? 1 : a.date > b.date ? -1 : a.slug < b.slug ? 1 : -1));
}

function post(site: Site, route: Route): Output[] {
  const p = route.data as Post;
  const out: Output[] = [];
  const fig = press(site, out);
  const head = `${hero(fig, p.figure, route.route, p.name)}\n<div class="plate"><h1 id="${escape(p.slug)}">${escape(p.name)}</h1><p class="by">${escape(p.date)} · ${escape(AUTHOR)}</p></div>`;
  const data = {
    "@context": "https://schema.org",
    "@type": "BlogPosting",
    headline: p.name,
    description: p.lead,
    url: root + route.route,
    image: `${root}/figures/${p.figure}.png`,
    author: { "@type": "Person", name: AUTHOR },
    datePublished: p.date || undefined,
  };
  out.push({ path: `blog/${p.slug}/index.html`, bytes: shell(site, { route: route.route, name: p.name, description: p.lead, body: `${head}\n${md(p.body, { math })}`, data }) });
  return out;
}

function blogIndex(site: Site, route: Route): Output[] {
  const list = route.data as Post[];
  const out: Output[] = [];
  const fig = press(site, out);
  const cards = list.map((p) => card(fig, `/blog/${p.slug}/`, p.figure, p.name, p.lead, [p.date]));
  const lead = "Notes on what lands on this site and in the crates behind it.";
  const body = `<div class="lede"><h1 id="blog">Blog</h1><p class="lead">${escape(lead)}</p></div>\n<div class="gallery wrap">\n${cards.join("\n")}\n</div>`;
  out.push({ path: "blog/index.html", bytes: shell(site, { route: route.route, name: "Blog", description: lead, body, type: "website", wide: true, bare: true }) });
  return out;
}

/* PAGES */

function about(site: Site, route: Route): Output[] {
  const source = route.source as string;
  const { data, body } = front(read(source));
  const name = data.title ?? "About";
  const lead = data.lead ?? summary(body);
  const head = `<div class="lede"><h1 id="about">${escape(name)}</h1><p class="lead">${escape(lead)}</p></div>`;
  return [{ path: "about/index.html", bytes: shell(site, { route: route.route, name, description: lead, body: `${head}\n${md(body, { math })}`, type: "website" }) }];
}

const MISSION = "The mathematics of designs on the corners of a cube, and the instruments that measure them.";

const DOORS = [
  { name: "Demos", href: "/demos/", figure: "site-demos", text: "Browser pages that draw a design and the numbers around it, live." },
  { name: "Papers", href: "/papers/", figure: "site-papers", text: "Write-ups with their LaTeX source and the scripts that check them." },
  { name: "Research", href: "/research/", figure: "site-research", text: "The working notes behind the demos and the papers, one page per idea." },
];

function home(site: Site, route: Route): Output[] {
  const { lanes: list, posts: written } = route.data as { lanes: Lane[]; posts: Post[] };
  const out: Output[] = [];
  const fig = press(site, out);
  const latest = list
    .map((p, i) => ({ p, i }))
    .sort((a, b) => (dated(a.p) < dated(b.p) ? 1 : dated(a.p) > dated(b.p) ? -1 : b.i - a.i))
    .slice(0, 3)
    .map(({ p }) => card(fig, `/papers/${p.slug}/`, `paper-${p.slug}`, p.name, p.blurb, [p.published]));
  const first = written[0];
  const news = first
    ? `<section><h2 id="latest">From the blog</h2><p class="lead"><a href="/blog/${first.slug}/">${escape(first.name)}</a> · ${escape(first.date)}</p><p class="lead">${escape(first.lead)}</p></section>`
    : "";
  const what = `<section class="what"><h2 id="mrlymath">What is MrlyMath</h2><p>A design is a rule on the corners of a cube: a code says which of the eight corners are filled. The Kronecker product grows that rule into itself, level by level, and the object it converges to is a fractal - the Sierpinski carpet and the Menger sponge are two of them.</p><p>Everything else is measurement. Count the fills, the voids and the exposed faces; cut the solid with a plane; join the filled cells into a graph and read its spectrum; collect the integer sequences the counts write down. The Rust crates do the arithmetic, the browser only paints, and a claim is either proved, checked over a stated finite domain, or labelled a conjecture.</p></section>`;
  const body = `<div class="home">
${hero(fig, "site-home", "/", kit.title)}
<div class="hero"><h1><span role="img" aria-label="${escape(kit.title)}">${WORD}</span></h1><p>${escape(MISSION)}</p></div>
<section><h2 id="doors">Three doors</h2><div class="gallery doors">\n${DOORS.map((d) => card(fig, d.href, d.figure, d.name, d.text)).join("\n")}\n</div></section>
<section><h2 id="shelf">Latest papers</h2><div class="gallery wrap">\n${latest.join("\n")}\n</div></section>
${news}
${what}
</div>`;
  out.push({ path: "index.html", bytes: shell(site, { route: "/", name: kit.title, description: MISSION, body, type: "website", wide: true, bare: true }) });
  return out;
}

function missing(site: Site, route: Route): Output[] {
  const body = `<div class="lede"><h1 id="lost">Nothing here</h1><p class="lead">That page does not exist. The four doors are <a href="/demos/">Demos</a>, <a href="/papers/">Papers</a>, <a href="/research/">Research</a> and <a href="/blog/">Blog</a>.</p></div>`;
  return [{ path: "404.html", bytes: shell(site, { route: route.route, name: "Nothing here", description: "That page does not exist.", body, type: "website", bare: true }) }];
}

/* COLLECT */

const counts = { papers: 0, research: 0, blog: 0, demos: 0 };

async function collect(site: Site) {
  SHELF = await shelf();
  const laneList = lanes();
  const noteList = notes(site);
  const postList = posts();
  const group = demoGroup(site);
  counts.papers = laneList.length;
  counts.research = noteList.length;
  counts.blog = postList.length;
  counts.demos = (group.data as Card[]).length;
  const nav = tree({
    papers: laneList.map((p) => ({ name: p.name, href: `/papers/${p.slug}/` })),
    research: noteList.filter((n) => !n.home).map((n) => ({ name: title(n.md) || n.name, href: `/research/${n.name}/` })),
    blog: postList.map((p) => ({ name: p.name, href: `/blog/${p.slug}/` })),
  });
  const routes: Route[] = [];
  const readme = join(org, "README.md");
  routes.push({
    route: "/",
    kind: "home",
    name: kit.title,
    data: { lanes: laneList, posts: postList },
    source: readme,
    inputs: [readme],
  });
  routes.push(group);
  if (laneList.length) {
    const index = join(SHELF, "README.md");
    routes.push({ route: "/papers/", kind: "papers", name: "Papers", data: laneList, source: index, inputs: [index] });
    for (const p of laneList) {
      const lane = join(SHELF, p.slug);
      routes.push({ route: `/papers/${p.slug}/`, kind: "paper", name: p.name, data: p, source: lane, inputs: [lane] });
    }
  }
  const notesHome = site.input("research").path;
  for (const n of noteList) {
    const source = join(notesHome, n.file);
    routes.push({
      route: n.home ? "/research/" : `/research/${n.name}/`,
      kind: "note",
      name: title(n.md) || n.name,
      data: n,
      source,
      inputs: [source],
    });
  }
  if (postList.length) {
    const files = postList.map((p) => postFile(p.slug));
    routes.push({ route: "/blog/", kind: "blog", name: "Blog", data: postList, source: BLOG, inputs: files });
    for (const p of postList) {
      const source = postFile(p.slug);
      routes.push({ route: `/blog/${p.slug}/`, kind: "post", name: p.name, data: p, source, inputs: [source] });
    }
  }
  const page = join(org, "pages", "about.md");
  if (existsSync(page)) routes.push({ route: "/about/", kind: "about", name: "About", source: page, inputs: [page] });
  routes.push({ route: "/404.html", kind: "missing", name: "Nothing here", hidden: true });
  return { routes, nav };
}

/* RENDER */

const KINDS: Record<string, (site: Site, route: Route) => Output[] | Promise<Output[]>> = {
  home,
  demos,
  papers: paperIndex,
  paper,
  note,
  blog: blogIndex,
  post,
  about,
  missing,
};

function draw(site: Site, route: Route) {
  const fn = KINDS[route.kind ?? ""];
  if (!fn) throw new Error(`site: no template for ${route.route}`);
  return fn(site, route);
}

/* EXTRAS */

function extras(site: Site): Output[] {
  const out: Output[] = [];
  out.push({ path: "favicon.svg", bytes: logoSvg(1, "#5a4bd1") });
  const home = site.input("figures").path;
  for (const [name, target] of [["site-og", "og.png"], ["site-icon", "icon-512.png"], ["site-icon", "apple-touch-icon.png"]]) {
    const file = join(home, `${name}.png`);
    if (existsSync(file)) out.push({ path: target, bytes: bytes(file) });
  }
  const shared = join(site.input("research").path, "figures");
  for (const file of walk(shared)) out.push({ path: `research/figures/${file.slice(shared.length + 1)}`, bytes: bytes(file) });
  return out;
}

/* SPEC */

export const MANIFEST = ".cache/manifest.json";

export const counted = () => ({ ...counts });

export const spec: Spec = {
  root: org,
  out: dist,
  templates: ["lib", "scripts"],
  collect,
  render: draw,
  globals: extras,
  git: {
    page: shell,
    md: (text, dir) => md(text, { math, link: (url: string) => gitLink(dir, url) }),
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
