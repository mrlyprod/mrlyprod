import { copyFileSync, cpSync, existsSync, mkdirSync, readdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import { createElement as h } from "react";
import { renderToStaticMarkup } from "react-dom/server";
import katex from "katex";
import { escape, front, plain, render, summary, title } from "../lib/md.js";
import { tree } from "../lib/tree.js";
import { logoSvg } from "../lib/logo.js";
import { glyphSvg } from "../../ui/font.js";
import { Shell } from "../../ui/chrome.jsx";
import kit from "../../ui/site.json";
import { shelf } from "./shelf.ts";

const org = resolve(import.meta.dir, "..");
const dist = join(org, "dist");
const ui = resolve(org, "../ui");
const figures = resolve(org, "../../files/figures");
const research = resolve(org, "../../research");
const root = (process.env.MRLY_SITE ?? kit.root).replace(/\/$/, "");
const AUTHOR = "Carlo Mitchener";
const GITHUB = "https://github.com/mrlyprod/mrlyprod/tree/main";
const LIST = /^- \[([^\]]+)\]\([^)]*\) - (.+)$/gm;
const HEADING = /<h([23]) id="([^"]+)">(.*?)<\/h\1>/g;
const AVATAR = /^!\[avatar\]\(figures\/avatar\.png\)\n?/m;
const KIT = ["tokens.css", "base.css", "chrome.css", "chrome.js", "font.js", "font.json", "mark.json", "site.json"];
const WORD = glyphSvg(kit.title.toUpperCase());
const ICONS = [
  `<link rel="icon" href="/favicon.svg" type="image/svg+xml">`,
  `<link rel="apple-touch-icon" href="/apple-touch-icon.png">`,
  `<link rel="manifest" href="/manifest.webmanifest">`,
].join("\n");

type Entry = { route: string; source: string };

let nodes = tree();
const entries: Entry[] = [];
const wanted = new Map<string, string>();
const served = new Set<string>();

const read = (p: string) => readFileSync(p, "utf8");
const write = (p: string, s: string) => {
  mkdirSync(dirname(p), { recursive: true });
  writeFileSync(p, s);
};
const math = (tex: string, display: boolean) =>
  katex.renderToString(tex, { output: "mathml", throwOnError: false, displayMode: display });
const untag = (html: string) =>
  html.replace(/<[^>]+>/g, "").replace(/&amp;/g, "&").replace(/&lt;/g, "<").replace(/&gt;/g, ">").replace(/&quot;/g, '"');
const brand = (name: string) => (name === kit.title ? name : `${name} · ${kit.title}`);

/* FIGURES */

function figure(name: string, route: string) {
  if (!wanted.has(name)) wanted.set(name, route);
  served.add(name);
  return `/figures/${name}.png`;
}

const hero = (name: string, route: string, alt: string) =>
  `<figure class="opener"><img src="${figure(name, route)}" alt="${escape(alt)}" width="1024" height="1024"></figure>`;

const card = (href: string, name: string, alt: string, text: string, dates: string[] = []) =>
  `<a class="tile" href="${href}"><img src="${figure(name, href)}" alt="${escape(alt)}" width="1024" height="1024" loading="lazy"><h2>${escape(alt)}</h2><p>${escape(text)}</p>${dates.length ? `<p class="dates">${dates.map((d) => `<span>${escape(d)}</span>`).join("")}</p>` : ""}</a>`;

function press() {
  const missing = [...wanted].filter(([name]) => !existsSync(join(figures, `${name}.png`)));
  if (missing.length) {
    const list = missing.map(([name, route]) => `  ${name}.png  for ${route}`).join("\n");
    throw new Error(`site: ${missing.length} figure(s) missing from ${relative(org, figures)}; draw them with bun run figures\n${list}`);
  }
  mkdirSync(join(dist, "figures"), { recursive: true });
  for (const name of served) copyFileSync(join(figures, `${name}.png`), join(dist, "figures", `${name}.png`));
  return wanted.size;
}

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
  source?: string;
  wide?: boolean;
  bare?: boolean;
  data?: object;
};

function page(leaf: Leaf) {
  const { route, name, description, body, type = "article", source, wide = false, bare = false, data } = leaf;
  const article = h(bare ? "div" : "article", { className: bare ? undefined : "prose", dangerouslySetInnerHTML: { __html: body } });
  const main = renderToStaticMarkup(h(Shell, { route, tree: nodes, contents: headings(body), wide }, article));
  const ld = data ? `<script type="application/ld+json">${JSON.stringify(data)}</script>\n` : "";
  if (source) entries.push({ route, source });
  return `<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>${escape(brand(name))}</title>
${meta(route, name, description, type)}
<link rel="stylesheet" href="/ui/tokens.css">
<link rel="stylesheet" href="/ui/base.css">
<link rel="stylesheet" href="/ui/chrome.css">
<link rel="stylesheet" href="/pages.css">
${ld}<script type="module" src="/ui/chrome.js"></script>
</head>
<body>
${main}
</body>
</html>
`;
}

/* DEMOS */

function blurbs() {
  const map = new Map<string, string>();
  const json = join(org, "pages.json");
  if (existsSync(json)) {
    const data = JSON.parse(read(json));
    const rows = (Array.isArray(data) ? data : Object.values(data).flat()) as { name?: string; blurb?: string }[];
    for (const row of rows) if (row && row.name && row.blurb) map.set(row.name, plain(row.blurb));
  }
  const readme = read(join(org, "README.md"));
  for (const m of readme.matchAll(LIST)) if (!map.has(m[1])) map.set(m[1], plain(m[2]));
  const home = readme.match(/^- (.+)$/m);
  map.set("", plain(home ? home[1] : kit.title));
  return map;
}

function shells() {
  const blurb = blurbs();
  const names = readdirSync(join(dist, "demos")).filter((d) => existsSync(join(dist, "demos", d, "index.html"))).sort();
  for (const name of ["", ...names]) {
    const at = name ? `demos/${name}` : "demos";
    const route = name ? `/demos/${name}/` : "/demos/";
    const file = join(dist, at, "index.html");
    const html = read(file);
    if (!html.includes('rel="canonical"')) {
      const found = html.match(/<title>([^<]*)<\/title>/);
      const heading = found ? untag(found[1]) : name || kit.title;
      const tags = meta(route, heading, blurb.get(name) ?? heading, "website");
      writeFileSync(
        file,
        found
          ? html.replace(found[0], `<title>${escape(brand(heading))}</title>\n${tags}`)
          : html.replace("<head>", `<head>\n<title>${escape(brand(heading))}</title>\n${tags}`),
      );
    }
    entries.push({ route, source: join(org, at, "index.jsx") });
  }
  return names.length + 1;
}

/* PAPERS */

type Lane = { slug: string; blurb: string; name: string; md: string; published: string; revised: string; pdf: boolean; home: string };

function lanes(home: string): Lane[] {
  const readme = read(join(home, "README.md"));
  const order = [...readme.matchAll(LIST)].map((m) => ({ slug: m[1], blurb: plain(m[2]) }));
  const found = readdirSync(home).filter(
    (d) => d !== "template" && existsSync(join(home, d, "README.md")) && existsSync(join(home, d, "paper.tex")),
  );
  const known = new Set(order.map((o) => o.slug));
  const list = order.filter((o) => found.includes(o.slug)).concat(found.filter((l) => !known.has(l)).map((slug) => ({ slug, blurb: "" })));
  return list.map(({ slug, blurb }) => {
    const lane = join(home, slug);
    const md = read(join(lane, "README.md"));
    const tex = read(join(lane, "paper.tex"));
    const date = tex.match(/\\date\{First published (\d{4}-\d{2}-\d{2})(?:, revised (\d{4}-\d{2}-\d{2}))?\}/);
    return {
      slug,
      blurb,
      name: title(md) || slug,
      md,
      published: date?.[1] ?? "",
      revised: date?.[2] ?? "",
      pdf: existsSync(join(lane, "paper.pdf")),
      home,
    };
  });
}

const dated = (p: Lane) => p.revised || p.published;
const stamps = (p: Lane) => [p.published, p.revised && `revised ${p.revised}`].filter(Boolean) as string[];

function papers(list: Lane[]) {
  if (!list.length) return 0;
  for (const p of list) {
    const lane = join(p.home, p.slug);
    const route = `/papers/${p.slug}/`;
    const out = join(dist, "papers", p.slug);
    mkdirSync(out, { recursive: true });
    if (p.pdf) copyFileSync(join(lane, "paper.pdf"), join(out, "paper.pdf"));
    copyFileSync(join(lane, "paper.tex"), join(out, "paper.tex"));
    if (existsSync(join(lane, "figures"))) cpSync(join(lane, "figures"), join(out, "figures"), { recursive: true });
    const when = [p.published && `First published ${p.published}`, p.revised && `revised ${p.revised}`].filter(Boolean).join(", ");
    const files = [p.pdf && `<a href="paper.pdf">PDF</a>`, `<a href="paper.tex">TeX</a>`].filter(Boolean).join(" · ");
    const avatar = `<img class="avatar" src="${figure(`paper-${p.slug}`, route)}" alt="${escape(p.name)}" width="1024" height="1024">`;
    const plate = `<div class="plate paper">${avatar}<h1 id="${escape(p.slug)}">${escape(p.name)}</h1><p class="by">${escape(AUTHOR)}</p><p class="by">${escape(when)}</p></div>\n<p class="meta">${files}</p>`;
    const body = `${plate}\n${render(p.md.replace(/^# .+\n/, "").replace(AVATAR, ""), { math })}`;
    const data = {
      "@context": "https://schema.org",
      "@type": "ScholarlyArticle",
      headline: p.name,
      description: p.blurb || summary(p.md),
      url: root + route,
      image: `${root}/figures/paper-${p.slug}.png`,
      author: { "@type": "Person", name: AUTHOR },
      datePublished: p.published || undefined,
      dateModified: dated(p) || undefined,
      license: "https://creativecommons.org/licenses/by/4.0/",
    };
    write(join(out, "index.html"), page({ route, name: p.name, description: p.blurb || summary(p.md), body, source: join(lane, "README.md"), data }));
  }
  const cards = list.map((p) => card(`/papers/${p.slug}/`, `paper-${p.slug}`, p.name, p.blurb, stamps(p)));
  const readme = read(join(list[0].home, "README.md"));
  const lead = summary(readme);
  const index = `<div class="lede"><h1 id="papers">Papers</h1><p class="lead">${escape(lead)}</p></div>\n<div class="gallery wrap">\n${cards.join("\n")}\n</div>`;
  write(join(dist, "papers", "index.html"), page({ route: "/papers/", name: "Papers", description: lead, body: index, type: "website", source: join(list[0].home, "README.md"), wide: true, bare: true }));
  return list.length;
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

function notes(): Note[] {
  if (!existsSync(research)) {
    console.warn(`site: no research tree at ${relative(org, research)}, research skipped`);
    return [];
  }
  return readdirSync(research)
    .filter((f) => f.endsWith(".md"))
    .sort((a, b) => (a === "README.md" ? -1 : b === "README.md" ? 1 : a.localeCompare(b)))
    .map((file) => ({ file, name: file.slice(0, -3), md: read(join(research, file)), home: file === "README.md" }));
}

function researchPages(list: Note[]) {
  for (const n of list) {
    write(join(dist, "research", n.file), n.md);
    const route = n.home ? "/research/" : `/research/${n.name}/`;
    const name = title(n.md) || n.name;
    const lead = summary(n.md);
    const fig = n.home || SHARED.has(n.name) ? "research-index" : `research-${n.name}`;
    const body = `${hero(fig, route, name)}\n${render(n.md, { math, link: researchLink })}`;
    const source = join(research, n.file);
    const data = {
      "@context": "https://schema.org",
      "@type": "Article",
      headline: name,
      description: lead,
      url: root + route,
      image: `${root}/figures/${fig}.png`,
      author: { "@type": "Person", name: AUTHOR },
    };
    write(join(dist, "research", n.home ? "" : n.name, "index.html"), page({ route, name, description: lead, body, type: n.home ? "website" : "article", source, data }));
  }
  if (list.length && existsSync(join(research, "figures"))) cpSync(join(research, "figures"), join(dist, "research", "figures"), { recursive: true });
  return list.length;
}

/* BLOG */

type Post = { slug: string; name: string; date: string; lead: string; figure: string; body: string; source: string };

function posts(): Post[] {
  const dir = join(org, "blog");
  if (!existsSync(dir)) return [];
  return readdirSync(dir)
    .filter((f) => f.endsWith(".md"))
    .map((file) => {
      const source = join(dir, file);
      const { data, body } = front(read(source));
      const slug = file.slice(0, -3);
      return { slug, name: data.title ?? slug, date: data.date ?? "", lead: data.lead ?? summary(body), figure: data.figure || `blog-${slug}`, body, source };
    })
    .sort((a, b) => (a.date < b.date ? 1 : a.date > b.date ? -1 : a.slug < b.slug ? 1 : -1));
}

function blog(list: Post[]) {
  for (const p of list) {
    const route = `/blog/${p.slug}/`;
    const head = `${hero(p.figure, route, p.name)}\n<div class="plate"><h1 id="${escape(p.slug)}">${escape(p.name)}</h1><p class="by">${escape(p.date)} · ${escape(AUTHOR)}</p></div>`;
    const data = {
      "@context": "https://schema.org",
      "@type": "BlogPosting",
      headline: p.name,
      description: p.lead,
      url: root + route,
      image: `${root}/figures/${p.figure}.png`,
      author: { "@type": "Person", name: AUTHOR },
      datePublished: p.date || undefined,
    };
    write(join(dist, "blog", p.slug, "index.html"), page({ route, name: p.name, description: p.lead, body: `${head}\n${render(p.body, { math })}`, source: p.source, data }));
  }
  const cards = list.map((p) => card(`/blog/${p.slug}/`, p.figure, p.name, p.lead, [p.date]));
  const lead = "Notes on what lands on this site and in the crates behind it.";
  const index = `<div class="lede"><h1 id="blog">Blog</h1><p class="lead">${escape(lead)}</p></div>\n<div class="gallery wrap">\n${cards.join("\n")}\n</div>`;
  write(join(dist, "blog", "index.html"), page({ route: "/blog/", name: "Blog", description: lead, body: index, type: "website", source: join(org, "blog"), wide: true, bare: true }));
  return list.length;
}

/* PAGES */

function about() {
  const source = join(org, "pages", "about.md");
  if (!existsSync(source)) return 0;
  const { data, body } = front(read(source));
  const name = data.title ?? "About";
  const lead = data.lead ?? summary(body);
  const head = `<div class="lede"><h1 id="about">${escape(name)}</h1><p class="lead">${escape(lead)}</p></div>`;
  write(join(dist, "about", "index.html"), page({ route: "/about/", name, description: lead, body: `${head}\n${render(body, { math })}`, type: "website", source }));
  return 1;
}

function home(list: Lane[], written: Post[]) {
  const mission = "The mathematics of designs on the corners of a cube, and the instruments that measure them.";
  const doors = [
    { name: "Demos", href: "/demos/", figure: "site-demos", text: "Browser pages that draw a design and the numbers around it, live." },
    { name: "Papers", href: "/papers/", figure: "site-papers", text: "Write-ups with their LaTeX source and the scripts that check them." },
    { name: "Research", href: "/research/", figure: "site-research", text: "The working notes behind the demos and the papers, one page per idea." },
  ];
  const latest = list
    .map((p, i) => ({ p, i }))
    .sort((a, b) => (dated(a.p) < dated(b.p) ? 1 : dated(a.p) > dated(b.p) ? -1 : b.i - a.i))
    .slice(0, 3)
    .map(({ p }) => card(`/papers/${p.slug}/`, `paper-${p.slug}`, p.name, p.blurb, [p.published]));
  const post = written[0];
  const news = post
    ? `<section><h2 id="latest">From the blog</h2><p class="lead"><a href="/blog/${post.slug}/">${escape(post.name)}</a> · ${escape(post.date)}</p><p class="lead">${escape(post.lead)}</p></section>`
    : "";
  const what = `<section class="what"><h2 id="mrlymath">What is MrlyMath</h2><p>A design is a rule on the corners of a cube: a code says which of the eight corners are filled. The Kronecker product grows that rule into itself, level by level, and the object it converges to is a fractal - the Sierpinski carpet and the Menger sponge are two of them.</p><p>Everything else is measurement. Count the fills, the voids and the exposed faces; cut the solid with a plane; join the filled cells into a graph and read its spectrum; collect the integer sequences the counts write down. The Rust crates do the arithmetic, the browser only paints, and a claim is either proved, checked over a stated finite domain, or labelled a conjecture.</p></section>`;
  const body = `<div class="home">
${hero("site-home", "/", kit.title)}
<div class="hero"><h1><span role="img" aria-label="${escape(kit.title)}">${WORD}</span></h1><p>${escape(mission)}</p></div>
<section><h2 id="doors">Three doors</h2><div class="gallery doors">\n${doors.map((d) => card(d.href, d.figure, d.name, d.text)).join("\n")}\n</div></section>
<section><h2 id="shelf">Latest papers</h2><div class="gallery wrap">\n${latest.join("\n")}\n</div></section>
${news}
${what}
</div>`;
  write(join(dist, "index.html"), page({ route: "/", name: kit.title, description: mission, body, type: "website", source: join(org, "README.md"), wide: true, bare: true }));
  return 1;
}

function missing() {
  const body = `<div class="lede"><h1 id="lost">Nothing here</h1><p class="lead">That page does not exist. The four doors are <a href="/demos/">Demos</a>, <a href="/papers/">Papers</a>, <a href="/research/">Research</a> and <a href="/blog/">Blog</a>.</p></div>`;
  write(join(dist, "404.html"), page({ route: "/404.html", name: "Nothing here", description: "That page does not exist.", body, type: "website", bare: true }));
}

/* ASSETS */

function assets() {
  mkdirSync(join(dist, "ui"), { recursive: true });
  for (const file of KIT) copyFileSync(join(ui, file), join(dist, "ui", file));
  const pub = join(org, "public");
  if (existsSync(pub)) cpSync(pub, dist, { recursive: true });
  write(join(dist, "favicon.svg"), logoSvg(1, "#5a4bd1"));
  for (const [name, out] of [["site-og", "og.png"], ["site-icon", "icon-512.png"], ["site-icon", "apple-touch-icon.png"]]) {
    if (!wanted.has(name)) wanted.set(name, `/${out}`);
    if (existsSync(join(figures, `${name}.png`))) copyFileSync(join(figures, `${name}.png`), join(dist, out));
  }
  write(
    join(dist, "manifest.webmanifest"),
    JSON.stringify(
      {
        name: kit.title,
        short_name: kit.title,
        start_url: "/",
        display: "standalone",
        background_color: "#0b0d10",
        theme_color: "#0b0d10",
        icons: [
          { src: "/favicon.svg", sizes: "any", type: "image/svg+xml" },
          { src: "/icon-512.png", sizes: "512x512", type: "image/png" },
        ],
      },
      null,
      2,
    ) + "\n",
  );
}

/* SITEMAP */

const TODAY = new Date().toISOString().slice(0, 10);
const seen = new Map<string, string>();

function lastmod(source: string) {
  if (!source) return TODAY;
  const hit = seen.get(source);
  if (hit) return hit;
  let out = "";
  try {
    const run = Bun.spawnSync(["git", "log", "-1", "--format=%cI", "--", source], { cwd: dirname(source), stderr: "ignore" });
    out = run.stdout.toString().trim();
  } catch {
    out = "";
  }
  const when = out || TODAY;
  seen.set(source, when);
  return when;
}

function sitemap() {
  const routes = new Map<string, string>();
  for (const e of entries) if (!routes.has(e.route)) routes.set(e.route, e.source);
  const urls = [...routes.keys()].sort().map((route) => `<url><loc>${root}${route}</loc><lastmod>${lastmod(routes.get(route)!)}</lastmod></url>`);
  write(
    join(dist, "sitemap.xml"),
    `<?xml version="1.0" encoding="UTF-8"?>\n<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">\n${urls.join("\n")}\n</urlset>\n`,
  );
  write(join(dist, "robots.txt"), `User-agent: *\nAllow: /\nSitemap: ${root}/sitemap.xml\n`);
  return urls.length;
}

/* BUILD */

export async function statics() {
  assets();
  const shelfLanes = lanes(await shelf());
  const researchNotes = notes();
  const blogPosts = posts();
  nodes = tree({
    papers: shelfLanes.map((p) => ({ name: p.name, href: `/papers/${p.slug}/` })),
    research: researchNotes.filter((n) => !n.home).map((n) => ({ name: n.name, href: `/research/${n.name}/` })),
    blog: blogPosts.map((p) => ({ name: p.name, href: `/blog/${p.slug}/` })),
  });
  const counts = { papers: papers(shelfLanes), research: researchPages(researchNotes), blog: blog(blogPosts), figures: 0 };
  about();
  home(shelfLanes, blogPosts);
  missing();
  counts.figures = press();
  return counts;
}

if (import.meta.main) {
  if (!existsSync(join(dist, "demos", "index.html"))) throw new Error("dist/demos/index.html missing: run bun build first");
  const { papers: nPapers, research: nResearch, blog: nBlog, figures: nFigures } = await statics();
  const nShells = shells();
  const nRoutes = sitemap();
  console.log(`site: ${nShells} shells, ${nPapers} papers, ${nResearch} research pages, ${nBlog} posts, ${nRoutes} routes, ${nFigures} figures`);
}
