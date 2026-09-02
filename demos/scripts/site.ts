import { copyFileSync, cpSync, existsSync, mkdirSync, readdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import katex from "katex";
import { escape, plain, render, summary, title } from "../lib/md.js";

const demos = resolve(import.meta.dir, "..");
const dist = join(demos, "dist");
const shelf = process.env.MRLY_SHELF ?? resolve(demos, "../../carlomitchener/research");
const research = resolve(demos, "../research");
const site = (process.env.MRLY_SITE ?? "https://www.mrly.net").replace(/\/$/, "");
const GITHUB = "https://github.com/mrlyprod/mrlyprod/tree/main";
const LIST = /^- \[([^\]]+)\]\([^)]*\) - (.+)$/gm;

const read = (p: string) => readFileSync(p, "utf8");
const write = (p: string, s: string) => {
  mkdirSync(dirname(p), { recursive: true });
  writeFileSync(p, s);
};
const math = (tex: string, display: boolean) =>
  katex.renderToString(tex, { output: "mathml", throwOnError: false, displayMode: display });

function meta(route: string, name: string, description: string, type: string) {
  const url = site + route;
  return [
    `<link rel="canonical" href="${url}">`,
    `<meta name="description" content="${escape(description)}">`,
    `<meta property="og:title" content="${escape(name)}">`,
    `<meta property="og:description" content="${escape(description)}">`,
    `<meta property="og:url" content="${url}">`,
    `<meta property="og:type" content="${type}">`,
  ].join("\n");
}

function page(route: string, name: string, description: string, body: string, type = "article") {
  return `<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>mrly · ${escape(name)}</title>
${meta(route, name, description, type)}
<link rel="stylesheet" href="/paper.css">
</head>
<body>
<nav><a href="/">Demos</a><a href="/papers/">Papers</a><a href="/research/">Research</a></nav>
<main>
${body}
</main>
</body>
</html>
`;
}

function blurbs() {
  const map = new Map<string, string>();
  const json = join(demos, "pages.json");
  if (existsSync(json)) {
    const data = JSON.parse(read(json));
    const rows = (Array.isArray(data) ? data : Object.values(data).flat()) as { name?: string; blurb?: string }[];
    for (const row of rows) if (row && row.name && row.blurb) map.set(row.name, plain(row.blurb));
  }
  const readme = read(join(demos, "README.md"));
  for (const m of readme.matchAll(LIST)) if (!map.has(m[1])) map.set(m[1], plain(m[2]));
  const root = readme.match(/^- (.+)$/m);
  map.set("", plain(root ? root[1] : "mrly demos"));
  return map;
}

function shells() {
  const blurb = blurbs();
  const names = [""].concat(
    readdirSync(dist).filter((d) => d !== "papers" && d !== "research" && existsSync(join(dist, d, "index.html"))),
  );
  for (const name of names) {
    const file = join(dist, name, "index.html");
    let html = read(file);
    if (html.includes('rel="canonical"')) continue;
    const found = html.match(/<title>([^<]*)<\/title>/);
    const heading = found ? found[1].replace(/&amp;/g, "&").replace(/&lt;/g, "<").replace(/&gt;/g, ">") : name ? `mrly · ${name}` : "mrly";
    const tags = meta(name ? `/${name}/` : "/", heading, blurb.get(name) ?? heading, "website");
    html = found
      ? html.replace(found[0], found[0] + "\n" + tags)
      : html.replace("<head>", `<head>\n<title>${escape(heading)}</title>\n${tags}`);
    writeFileSync(file, html);
  }
  return names.length;
}

function papers() {
  if (!existsSync(join(shelf, "README.md"))) {
    console.warn(`site: no shelf at ${relative(demos, shelf)}, papers skipped`);
    return 0;
  }
  const readme = read(join(shelf, "README.md"));
  const order = [...readme.matchAll(LIST)].map((m) => ({ slug: m[1], blurb: plain(m[2]) }));
  const lanes = readdirSync(shelf).filter(
    (d) => d !== "template" && existsSync(join(shelf, d, "README.md")) && existsSync(join(shelf, d, "paper.tex")),
  );
  const known = new Set(order.map((o) => o.slug));
  const list = order.filter((o) => lanes.includes(o.slug)).concat(lanes.filter((l) => !known.has(l)).map((slug) => ({ slug, blurb: "" })));
  const items = [];
  for (const { slug, blurb } of list) {
    const lane = join(shelf, slug);
    const md = read(join(lane, "README.md"));
    const tex = read(join(lane, "paper.tex"));
    const date = tex.match(/\\date\{First published (\d{4}-\d{2}-\d{2})(?:, revised (\d{4}-\d{2}-\d{2}))?\}/);
    const published = date?.[1] ?? "";
    const revised = date?.[2] ?? "";
    const out = join(dist, "papers", slug);
    mkdirSync(out, { recursive: true });
    const pdf = existsSync(join(lane, "paper.pdf"));
    if (pdf) copyFileSync(join(lane, "paper.pdf"), join(out, "paper.pdf"));
    copyFileSync(join(lane, "paper.tex"), join(out, "paper.tex"));
    if (existsSync(join(lane, "figures"))) cpSync(join(lane, "figures"), join(out, "figures"), { recursive: true });
    const name = title(md) || slug;
    const when = [published && `First published ${published}`, revised && `revised ${revised}`].filter(Boolean).join(", ");
    const files = [pdf && `<a href="paper.pdf">PDF</a>`, `<a href="paper.tex">TeX</a>`].filter(Boolean).join(" · ");
    const body = `<p class="meta">${when ? when + " · " : ""}${files}</p>\n${render(md, { math })}`;
    write(join(out, "index.html"), page(`/papers/${slug}/`, name, blurb || summary(md), body));
    items.push({ slug, name, blurb, published });
  }
  const rows = items.map(
    (p) =>
      `<li><a href="/papers/${p.slug}/">${escape(p.name)}</a>${p.blurb ? " - " + escape(p.blurb) : ""}${p.published ? ` <span class="date">${p.published}</span>` : ""}</li>`,
  );
  const index = `<h1 id="papers">Papers</h1>\n<ul>\n${rows.join("\n")}\n</ul>`;
  write(join(dist, "papers", "index.html"), page("/papers/", "Papers", summary(readme), index, "website"));
  return items.length;
}

function researchLink(url: string) {
  if (/^(https?:|mailto:|#)/.test(url)) return url;
  let m: RegExpMatchArray | null;
  if ((m = url.match(/^([A-Za-z0-9_-]+)\.md(#.*)?$/))) return (m[1] === "README" ? "/research/" : `/research/${m[1]}/`) + (m[2] ?? "");
  if ((m = url.match(/^\.\.\/demos\/([^/#?]+)/))) return `/${m[1]}/`;
  if ((m = url.match(/^lab\/(.*)$/))) return `${GITHUB}/research/lab/${m[1]}`;
  if ((m = url.match(/^\.\.\/crates\/(.*)$/))) return `${GITHUB}/crates/${m[1]}`;
  if ((m = url.match(/^figures\/(.*)$/))) return `/research/figures/${m[1]}`;
  return url;
}

function researchPages() {
  if (!existsSync(research)) {
    console.warn(`site: no research tree at ${relative(demos, research)}, research skipped`);
    return 0;
  }
  const files = readdirSync(research).filter((f) => f.endsWith(".md"));
  for (const file of files) {
    const md = read(join(research, file));
    const name = file.slice(0, -3);
    const home = name === "README";
    write(join(dist, "research", file), md);
    const route = home ? "/research/" : `/research/${name}/`;
    const body = render(md, { math, link: researchLink });
    write(join(dist, "research", home ? "" : name, "index.html"), page(route, title(md) || name, summary(md), body, home ? "website" : "article"));
  }
  if (existsSync(join(research, "figures"))) cpSync(join(research, "figures"), join(dist, "research", "figures"), { recursive: true });
  return files.length;
}

function walk(dir: string, out: string[] = []) {
  for (const e of readdirSync(dir, { withFileTypes: true })) {
    const p = join(dir, e.name);
    if (e.isDirectory()) walk(p, out);
    else if (e.name === "index.html") out.push(p);
  }
  return out;
}

function sitemap() {
  const routes = walk(dist)
    .map((p) => {
      const r = relative(dist, dirname(p));
      return r ? `/${r}/` : "/";
    })
    .sort();
  const urls = routes.map((r) => `<url><loc>${site}${r}</loc></url>`).join("\n");
  write(join(dist, "sitemap.xml"), `<?xml version="1.0" encoding="UTF-8"?>\n<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">\n${urls}\n</urlset>\n`);
  write(join(dist, "robots.txt"), `User-agent: *\nAllow: /\nSitemap: ${site}/sitemap.xml\n`);
  return routes.length;
}

if (!existsSync(join(dist, "index.html"))) throw new Error("dist/index.html missing: run bun build first");
copyFileSync(join(demos, "lib", "paper.css"), join(dist, "paper.css"));
const nShells = shells();
const nPapers = papers();
const nResearch = researchPages();
const nRoutes = sitemap();
console.log(`site: ${nShells} shells, ${nPapers} papers, ${nResearch} research pages, ${nRoutes} routes`);
