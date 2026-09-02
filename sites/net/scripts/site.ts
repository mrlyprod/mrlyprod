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

const org = resolve(import.meta.dir, "..");
const dist = join(org, "dist");
const ui = resolve(org, "../ui");
const shelf = process.env.MRLY_SHELF ?? resolve(org, "../../../carlomitchener/research");
const research = resolve(org, "../../research");
const root = (process.env.MRLY_SITE ?? kit.root).replace(/\/$/, "");
const AUTHOR = "Carlo Mitchener";
const GITHUB = "https://github.com/mrlyprod/mrlyprod/tree/main";
const LIST = /^- \[([^\]]+)\]\([^)]*\) - (.+)$/gm;
const HEADING = /<h([23]) id="([^"]+)">(.*?)<\/h\1>/g;
const KIT = ["tokens.css", "base.css", "chrome.css", "chrome.js", "font.js", "font.json", "mark.json", "site.json"];
const WORD = glyphSvg(kit.title.toUpperCase());
const SKIN = join(org, "public", "pages.css");
const PAINT = join(org, "lib", "cover.jsx");
const ICONS = [
  `<link rel="icon" href="/favicon.svg" type="image/svg+xml">`,
  `<link rel="apple-touch-icon" href="/apple-touch-icon.png">`,
  `<link rel="manifest" href="/manifest.webmanifest">`,
].join("\n");

type Shot = { key: string; route: string; url: string; sources: string[] };
type Entry = { route: string; source: string };

let nodes = tree();
const shots: Shot[] = [];
const entries: Entry[] = [];

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
const fit = (name: string) => (name.length > 76 ? ' class="epic"' : name.length > 44 ? ' class="long"' : "");
const keyOf = (route: string) => route.replace(/^\/|\/$/g, "").replace(/\//g, "-") || "home";

/* HEAD */

function meta(route: string, name: string, description: string, type: string) {
  const url = root + route;
  const cover = `${url}cover.png`;
  return [
    `<link rel="canonical" href="${url}">`,
    `<meta name="description" content="${escape(description)}">`,
    `<meta property="og:title" content="${escape(name)}">`,
    `<meta property="og:description" content="${escape(description)}">`,
    `<meta property="og:url" content="${url}">`,
    `<meta property="og:type" content="${type}">`,
    `<meta property="og:site_name" content="${escape(kit.title)}">`,
    `<meta property="og:image" content="${cover}">`,
    `<meta property="og:image:width" content="1200">`,
    `<meta property="og:image:height" content="630">`,
    `<meta name="twitter:card" content="summary_large_image">`,
    `<meta name="twitter:image" content="${cover}">`,
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

/* COVERS */

function shot(route: string, kind: string, name: string, by: string, art: string, sources: string[]) {
  const key = keyOf(route);
  write(
    join(dist, "_covers", key, "index.html"),
    `<!doctype html>
<html lang="en" data-theme="light">
<head>
<meta charset="utf-8">
<title>${escape(name)}</title>
<link rel="stylesheet" href="/ui/tokens.css">
<link rel="stylesheet" href="/ui/base.css">
<link rel="stylesheet" href="/pages.css">
</head>
<body class="shot">
<div class="cover">
<header>${WORD}<span class="kind">${escape(kind)}</span></header>
${art}
<footer><h1${fit(name)}>${escape(name)}</h1><p class="by">${escape(by)}</p></footer>
</div>
</body>
</html>
`,
  );
  shots.push({ key, route, url: `/_covers/${key}/`, sources: [...sources, SKIN] });
}

const figureArt = (src: string) => `<div class="art still"><img src="${escape(src)}" alt=""></div>`;
const logoArt = () => `<div class="art logo">${logoSvg(2)}</div>`;

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
  const wasm = join(org, "pkg", "mrlyweb_bg.wasm");
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
    if (name) shots.push({ key: keyOf(route), route, url: `/lib/cover.html?k=${name}`, sources: [join(org, at, "index.jsx"), join(org, "lib", "thumbs.jsx"), join(org, "pages.json"), wasm, SKIN, PAINT] });
  }
  shots.push({ key: "demos", route: "/demos/", url: `/lib/cover.html?k=radial&kind=Demos&t=${encodeURIComponent("The eyes of MrlyMath")}&by=${encodeURIComponent(`${names.length} pages drawn by the crates through wasm`)}`, sources: [join(org, "pages.json"), join(org, "demos", "index.jsx"), join(org, "lib", "thumbs.jsx"), wasm, SKIN, PAINT] });
  return names.length + 1;
}

/* PAPERS */

type Lane = { slug: string; blurb: string; name: string; md: string; published: string; revised: string; pdf: boolean; figure: string };

function lanes(): Lane[] {
  if (!existsSync(join(shelf, "README.md"))) {
    console.warn(`site: no shelf at ${relative(org, shelf)}, papers skipped`);
    return [];
  }
  const readme = read(join(shelf, "README.md"));
  const order = [...readme.matchAll(LIST)].map((m) => ({ slug: m[1], blurb: plain(m[2]) }));
  const found = readdirSync(shelf).filter(
    (d) => d !== "template" && existsSync(join(shelf, d, "README.md")) && existsSync(join(shelf, d, "paper.tex")),
  );
  const known = new Set(order.map((o) => o.slug));
  const list = order.filter((o) => found.includes(o.slug)).concat(found.filter((l) => !known.has(l)).map((slug) => ({ slug, blurb: "" })));
  return list.map(({ slug, blurb }) => {
    const lane = join(shelf, slug);
    const md = read(join(lane, "README.md"));
    const tex = read(join(lane, "paper.tex"));
    const date = tex.match(/\\date\{First published (\d{4}-\d{2}-\d{2})(?:, revised (\d{4}-\d{2}-\d{2}))?\}/);
    const figures = existsSync(join(lane, "figures")) ? readdirSync(join(lane, "figures")).filter((f) => f.endsWith(".svg")).sort() : [];
    return {
      slug,
      blurb,
      name: title(md) || slug,
      md,
      published: date?.[1] ?? "",
      revised: date?.[2] ?? "",
      pdf: existsSync(join(lane, "paper.pdf")),
      figure: figures[0] ?? "",
    };
  });
}

const dated = (p: Lane) => p.revised || p.published;

function papers(list: Lane[]) {
  if (!list.length) return 0;
  for (const p of list) {
    const lane = join(shelf, p.slug);
    const route = `/papers/${p.slug}/`;
    const out = join(dist, "papers", p.slug);
    mkdirSync(out, { recursive: true });
    if (p.pdf) copyFileSync(join(lane, "paper.pdf"), join(out, "paper.pdf"));
    copyFileSync(join(lane, "paper.tex"), join(out, "paper.tex"));
    if (existsSync(join(lane, "figures"))) cpSync(join(lane, "figures"), join(out, "figures"), { recursive: true });
    const when = [p.published && `First published ${p.published}`, p.revised && `revised ${p.revised}`].filter(Boolean).join(", ");
    const by = [AUTHOR, when].filter(Boolean).join(" · ");
    const files = [p.pdf && `<a href="paper.pdf">PDF</a>`, `<a href="paper.tex">TeX</a>`].filter(Boolean).join(" · ");
    const plate = `<div class="plate"><img src="cover.png" alt="${escape(p.name)}" width="1200" height="630"><h1 id="${escape(p.slug)}">${escape(p.name)}</h1><p class="by">${escape(by)}</p></div>\n<p class="meta">${files}</p>`;
    const body = `${plate}\n${render(p.md.replace(/^# .+\n/, ""), { math })}`;
    const data = {
      "@context": "https://schema.org",
      "@type": "ScholarlyArticle",
      headline: p.name,
      description: p.blurb || summary(p.md),
      url: root + route,
      image: `${root}${route}cover.png`,
      author: { "@type": "Person", name: AUTHOR },
      datePublished: p.published || undefined,
      dateModified: dated(p) || undefined,
      license: "https://creativecommons.org/licenses/by/4.0/",
    };
    write(join(out, "index.html"), page({ route, name: p.name, description: p.blurb || summary(p.md), body, source: join(lane, "README.md"), data }));
    shot(route, "Paper", p.name, by, p.figure ? figureArt(`${route}figures/${p.figure}`) : logoArt(), [join(lane, "README.md"), join(lane, "paper.tex"), p.figure ? join(lane, "figures", p.figure) : join(lane, "README.md")]);
  }
  const cards = list.map(
    (p) =>
      `<a class="tile" href="/papers/${p.slug}/"><img src="/papers/${p.slug}/cover.png" alt="${escape(p.name)}" width="1200" height="630" loading="lazy"><h2>${escape(p.name)}</h2><p>${escape(p.blurb)}</p><p class="dates"><span>${escape(p.published)}</span>${p.revised ? `<span>revised ${escape(p.revised)}</span>` : ""}</p></a>`,
  );
  const readme = read(join(shelf, "README.md"));
  const lead = summary(readme);
  const index = `<div class="lede"><h1 id="papers">Papers</h1><p class="lead">${escape(lead)}</p></div>\n<div class="gallery wrap">\n${cards.join("\n")}\n</div>`;
  write(join(dist, "papers", "index.html"), page({ route: "/papers/", name: "Papers", description: lead, body: index, type: "website", source: join(shelf, "README.md"), wide: true, bare: true }));
  shot("/papers/", "Papers", "Papers", `${list.length} write-ups by ${AUTHOR}`, logoArt(), [join(shelf, "README.md")]);
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
    const body = render(n.md, { math, link: researchLink });
    const source = join(research, n.file);
    const data = {
      "@context": "https://schema.org",
      "@type": "Article",
      headline: name,
      description: lead,
      url: root + route,
      image: `${root}${route}cover.png`,
      author: { "@type": "Person", name: AUTHOR },
    };
    write(join(dist, "research", n.home ? "" : n.name, "index.html"), page({ route, name, description: lead, body, type: n.home ? "website" : "article", source, data }));
    const fig = n.md.match(/\((figures\/[^)]+)\)/);
    shot(route, "Research", name, AUTHOR, fig ? figureArt(`/research/${fig[1]}`) : logoArt(), [source]);
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
      return { slug, name: data.title ?? slug, date: data.date ?? "", lead: data.lead ?? summary(body), figure: data.figure ?? "", body, source };
    })
    .sort((a, b) => (a.date < b.date ? 1 : a.date > b.date ? -1 : a.slug < b.slug ? 1 : -1));
}

function blog(list: Post[]) {
  for (const p of list) {
    const route = `/blog/${p.slug}/`;
    const head = `<div class="plate"><h1 id="${escape(p.slug)}">${escape(p.name)}</h1><p class="by">${escape(p.date)} · ${escape(AUTHOR)}</p></div>`;
    const data = {
      "@context": "https://schema.org",
      "@type": "BlogPosting",
      headline: p.name,
      description: p.lead,
      url: root + route,
      image: `${root}${route}cover.png`,
      author: { "@type": "Person", name: AUTHOR },
      datePublished: p.date || undefined,
    };
    write(join(dist, "blog", p.slug, "index.html"), page({ route, name: p.name, description: p.lead, body: `${head}\n${render(p.body, { math })}`, source: p.source, data }));
    shot(route, "Blog", p.name, [AUTHOR, p.date].filter(Boolean).join(" · "), p.figure ? figureArt(p.figure) : logoArt(), [p.source]);
  }
  const cards = list.map(
    (p) =>
      `<a class="tile" href="/blog/${p.slug}/"><img src="/blog/${p.slug}/cover.png" alt="${escape(p.name)}" width="1200" height="630" loading="lazy"><h2>${escape(p.name)}</h2><p>${escape(p.lead)}</p><p class="dates"><span>${escape(p.date)}</span></p></a>`,
  );
  const lead = "Notes on what lands on this site and in the crates behind it.";
  const index = `<div class="lede"><h1 id="blog">Blog</h1><p class="lead">${escape(lead)}</p></div>\n<div class="gallery wrap">\n${cards.join("\n")}\n</div>`;
  write(join(dist, "blog", "index.html"), page({ route: "/blog/", name: "Blog", description: lead, body: index, type: "website", source: join(org, "blog"), wide: true, bare: true }));
  shot("/blog/", "Blog", "Blog", `${list.length} post${list.length === 1 ? "" : "s"} by ${AUTHOR}`, logoArt(), list.map((p) => p.source));
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
  shot("/about/", "About", `${kit.title}, Inc.`, `${AUTHOR} · Brussels · since ${kit.since}`, logoArt(), [source]);
  return 1;
}

function home(list: Lane[], written: Post[]) {
  const mission = "The mathematics of designs on the corners of a cube, and the instruments that measure them.";
  const doors = [
    { name: "Demos", href: "/demos/", text: "Browser pages that draw a design and the numbers around it, live." },
    { name: "Papers", href: "/papers/", text: "Write-ups with their LaTeX source and the scripts that check them." },
    { name: "Research", href: "/research/", text: "The working notes behind the demos and the papers, one page per idea." },
  ];
  const door = (d: { name: string; href: string; text: string }) =>
    `<a class="tile" href="${d.href}"><img src="${d.href}cover.png" alt="${escape(d.name)}" width="1200" height="630"><h2>${escape(d.name)}</h2><p>${escape(d.text)}</p></a>`;
  const latest = list
    .map((p, i) => ({ p, i }))
    .sort((a, b) => (dated(a.p) < dated(b.p) ? 1 : dated(a.p) > dated(b.p) ? -1 : b.i - a.i))
    .slice(0, 3)
    .map(
      ({ p }) =>
        `<a class="tile" href="/papers/${p.slug}/"><img src="/papers/${p.slug}/cover.png" alt="${escape(p.name)}" width="1200" height="630" loading="lazy"><h2>${escape(p.name)}</h2><p>${escape(p.blurb)}</p><p class="dates"><span>${escape(p.published)}</span></p></a>`,
    );
  const post = written[0];
  const news = post
    ? `<section><h2 id="latest">From the blog</h2><p class="lead"><a href="/blog/${post.slug}/">${escape(post.name)}</a> · ${escape(post.date)}</p><p class="lead">${escape(post.lead)}</p></section>`
    : "";
  const what = `<section class="what"><h2 id="mrlymath">What is MrlyMath</h2><p>A design is a rule on the corners of a cube: a code says which of the eight corners are filled. The Kronecker product grows that rule into itself, level by level, and the object it converges to is a fractal - the Sierpinski carpet and the Menger sponge are two of them.</p><p>Everything else is measurement. Count the fills, the voids and the exposed faces; cut the solid with a plane; join the filled cells into a graph and read its spectrum; collect the integer sequences the counts write down. The Rust crates do the arithmetic, the browser only paints, and a claim is either proved, checked over a stated finite domain, or labelled a conjecture.</p></section>`;
  const body = `<div class="home">
<div class="hero"><h1><span role="img" aria-label="${escape(kit.title)}">${WORD}</span></h1><p>${escape(mission)}</p></div>
<section><h2 id="doors">Three doors</h2><div class="gallery doors">\n${doors.map(door).join("\n")}\n</div></section>
<section><h2 id="shelf">Latest papers</h2><div class="gallery wrap">\n${latest.join("\n")}\n</div></section>
${news}
${what}
</div>`;
  write(join(dist, "index.html"), page({ route: "/", name: kit.title, description: mission, body, type: "website", source: join(org, "README.md"), wide: true, bare: true }));
  shot("/", "Home", kit.title, mission, logoArt(), [join(org, "README.md"), join(org, "pages.json")]);
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
          { src: "/apple-touch-icon.png", sizes: "180x180", type: "image/png" },
          { src: "/icon-512.png", sizes: "512x512", type: "image/png" },
        ],
      },
      null,
      2,
    ) + "\n",
  );
  write(
    join(dist, "_covers", "icon", "index.html"),
    `<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<title>icon</title>
<link rel="stylesheet" href="/pages.css">
</head>
<body class="shot">
<div class="icon">${logoSvg(1, "#e8ecf1")}</div>
</body>
</html>
`,
  );
}

/* SITEMAP */

const TODAY = new Date().toISOString().slice(0, 10);
const stamps = new Map<string, string>();

function lastmod(source: string) {
  if (!source) return TODAY;
  const hit = stamps.get(source);
  if (hit) return hit;
  let out = "";
  try {
    const run = Bun.spawnSync(["git", "log", "-1", "--format=%cI", "--", source], { cwd: dirname(source), stderr: "ignore" });
    out = run.stdout.toString().trim();
  } catch {
    out = "";
  }
  const when = out || TODAY;
  stamps.set(source, when);
  return when;
}

function sitemap() {
  const seen = new Map<string, string>();
  for (const e of entries) if (!seen.has(e.route)) seen.set(e.route, e.source);
  const covered = new Set(shots.map((s) => s.route));
  const urls = [...seen.keys()].sort().map((route) => {
    const image = covered.has(route) ? `<image:image><image:loc>${root}${route}cover.png</image:loc></image:image>` : "";
    return `<url><loc>${root}${route}</loc><lastmod>${lastmod(seen.get(route)!)}</lastmod>${image}</url>`;
  });
  write(
    join(dist, "sitemap.xml"),
    `<?xml version="1.0" encoding="UTF-8"?>\n<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9" xmlns:image="http://www.google.com/schemas/sitemap-image/1.1">\n${urls.join("\n")}\n</urlset>\n`,
  );
  write(join(dist, "robots.txt"), `User-agent: *\nAllow: /\nSitemap: ${root}/sitemap.xml\n`);
  return urls.length;
}

/* BUILD */

export function statics() {
  assets();
  const shelfLanes = lanes();
  const researchNotes = notes();
  const blogPosts = posts();
  nodes = tree({
    papers: shelfLanes.map((p) => ({ name: p.name, href: `/papers/${p.slug}/` })),
    research: researchNotes.filter((n) => !n.home).map((n) => ({ name: n.name, href: `/research/${n.name}/` })),
    blog: blogPosts.map((p) => ({ name: p.name, href: `/blog/${p.slug}/` })),
  });
  const counts = { papers: papers(shelfLanes), research: researchPages(researchNotes), blog: blog(blogPosts) };
  about();
  home(shelfLanes, blogPosts);
  missing();
  return counts;
}

if (import.meta.main) {
  if (!existsSync(join(dist, "demos", "index.html"))) throw new Error("dist/demos/index.html missing: run bun build first");
  const { papers: nPapers, research: nResearch, blog: nBlog } = statics();
  const nShells = shells();
  const nRoutes = sitemap();
  write(join(dist, "_covers", "list.json"), JSON.stringify(shots, null, 2) + "\n");
  console.log(`site: ${nShells} shells, ${nPapers} papers, ${nResearch} research pages, ${nBlog} posts, ${nRoutes} routes, ${shots.length} covers`);
}
