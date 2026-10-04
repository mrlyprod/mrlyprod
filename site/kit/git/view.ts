import { seti } from "../code/seti/seti.ts";
import { escape } from "../ssg/text.ts";

/* TYPES */

export type Twig = { n: string; k: "d" | "f"; i?: string; s?: number; u?: string; c?: Twig[] };

export type Wood = { base: string; name: string; slug: string; branch: string; c: Twig[] };

export type Paint = (text: string, lang: string) => Promise<string[] | null> | string[] | null;

export type Tools = {
  load: (url: string) => Promise<Uint8Array | null>;
  md?: (text: string, link: (url: string) => string) => Promise<string> | string;
};

export type Code = { text: string; lang: string };

export type Full = { html: string; code?: Code };

export type View = { path: string; name: string; found: boolean; html: string; more?: (tools: Tools) => Promise<Full | null> };

/* TEXT */

export const stem = (path: string) => path.slice(path.lastIndexOf("/") + 1);

export const home = (path: string) => (path.includes("/") ? path.slice(0, path.lastIndexOf("/")) : "");

export const ext = (path: string) => {
  const name = stem(path).toLowerCase();
  const cut = name.lastIndexOf(".");
  return cut > 0 ? name.slice(cut + 1) : "";
};

const anchor = (text: string) => text.replace(/[^A-Za-z0-9]+/g, "-").replace(/^-|-$/g, "").toLowerCase() || "top";

export const size = (n: number) =>
  n < 1024 ? `${n} B` : n < 1024 * 1024 ? `${(n / 1024).toFixed(1)} kB` : `${(n / (1024 * 1024)).toFixed(1)} MB`;

/* ROUTES */

export const fileRoute = (path: string) => `/git/${path}`;

export const dirRoute = (dir: string) => (dir ? `/git/${dir}/` : "/git/");

export const rawPath = (path: string) => `raw/${path}`;

export const href = (url: string) => escape(encodeURI(url).replace(/#/g, "%23").replace(/\?/g, "%3F"));

export function owner(path: string): string | null {
  if (!path.startsWith("/raw/")) return null;
  return `/git/${path.slice(5)}`;
}

/* LANGS */

const LANGS: Record<string, string> = {
  bash: "shellscript",
  c: "c",
  cjs: "javascript",
  css: "css",
  csv: "csv",
  h: "c",
  html: "html",
  js: "javascript",
  json: "json",
  jsx: "jsx",
  lock: "toml",
  md: "markdown",
  mjs: "javascript",
  py: "python",
  pyi: "python",
  rs: "rust",
  sh: "shellscript",
  svg: "xml",
  tex: "latex",
  toml: "toml",
  ts: "typescript",
  tsx: "tsx",
  wgsl: "wgsl",
  xml: "xml",
  yaml: "yaml",
  yml: "yaml",
  zsh: "shellscript",
};

export const lang = (path: string) => (Object.hasOwn(LANGS, ext(path)) ? LANGS[ext(path)]! : "text");

/* SHAPE */

export const IMAGE = new Set(["avif", "gif", "ico", "jpeg", "jpg", "png", "svg", "webp"]);

export const HUGE = 1 << 20;

const WIDE = 200 * 1024;

export function reads(body: Uint8Array): string | null {
  const look = body.subarray(0, 8192);
  for (const one of look) if (one === 0) return null;
  try {
    return new TextDecoder("utf-8", { fatal: true }).decode(body);
  } catch {
    return null;
  }
}

/* PATHS */

export function decode(text: string): string {
  try {
    return decodeURIComponent(text);
  } catch {
    return text;
  }
}

/* TREE */

export function find(wood: Wood, path: string): Twig | null {
  let at: Twig = { n: wood.name, k: "d", c: wood.c };
  for (const part of path ? path.split("/") : []) {
    const next = (at.c ?? []).find((kid) => kid.n === part);
    if (!next) return null;
    at = next;
  }
  return at;
}

/* LINKS */

const DENY = /^(?:javascript|data|vbscript):/i;

export function link(dir: string, url: string, wood?: Wood): string {
  if (DENY.test(url.replace(/[\u0000- ]/g, ""))) return "#";
  if (/^(https?:|mailto:|tel:|#|\/)/.test(url)) return url;
  const cut = url.search(/[#?]/);
  const tail = cut < 0 ? "" : url.slice(cut);
  const head = cut < 0 ? url : url.slice(0, cut);
  const parts = (dir ? `${dir}/${head}` : head).split("/");
  const out: string[] = [];
  for (const part of parts) {
    if (part === "." || part === "") continue;
    if (part === "..") out.pop();
    else out.push(part);
  }
  const path = out.join("/");
  if (head.endsWith("/") || !path) return dirRoute(path) + tail;
  if (IMAGE.has(ext(path)) || ext(path) === "pdf") return `${(wood && find(wood, path)?.u) || `/${rawPath(path)}`}${tail}`;
  return fileRoute(path) + tail;
}

const github = (wood: Wood, path: string, dir: boolean) =>
  wood.slug ? `https://github.com/${wood.slug}/${dir ? "tree" : "blob"}/${wood.branch}${path ? `/${path}` : ""}` : "";

function bar(wood: Wood, path: string, dir: boolean, tools: string[]): string {
  const parts = path ? path.split("/") : [];
  const crumbs = [`<a href="/git/">${escape(wood.name)}</a>`];
  let at = "";
  parts.forEach((part, i) => {
    at = at ? `${at}/${part}` : part;
    const last = i === parts.length - 1;
    crumbs.push(last ? `<b>${escape(part)}</b>` : `<a href="${href(dirRoute(at))}">${escape(part)}</a>`);
  });
  const side = tools.filter(Boolean).join(" · ");
  return `<nav class="bar" aria-label="Path"><span class="crumbs">${crumbs.join('<span class="sep">/</span>')}</span><span class="tools">${side}</span></nav>`;
}

const lede = (name: string, lead: string) => `<div class="lede"><h1 id="${anchor(name)}">${escape(name)}</h1><p class="lead">${escape(lead)}</p></div>`;

const source = (path: string, twig: Twig) => twig.u ?? `/${rawPath(path)}`;

const prose = async (wood: Wood, tools: Tools, path: string, text: string) => `<div class="prose readme">${await tools.md!(text, (url) => link(home(path), url, wood))}</div>`;

/* LISTING */

function rows(dir: string, kids: Twig[]): string {
  const items = kids.map((kid) => {
    const path = dir ? `${dir}/${kid.n}` : kid.n;
    if (kid.k === "d") {
      const count = kid.c?.length ?? 0;
      return `<li class="dir"><span class="ico" aria-hidden="true"></span><a href="${href(dirRoute(path))}">${escape(kid.n)}/</a><span class="n">${count} item${count === 1 ? "" : "s"}</span></li>`;
    }
    return `<li><span class="${seti(kid.n)}" aria-hidden="true"></span><a href="${href(fileRoute(path))}">${escape(kid.n)}</a><span class="n">${size(kid.s ?? 0)}</span></li>`;
  });
  return `<ul class="files">\n${items.join("\n")}\n</ul>`;
}

function listing(wood: Wood, dir: string, twig: Twig): { html: string; more?: View["more"] } {
  const kids = twig.c ?? [];
  const where = github(wood, dir, true);
  const head = bar(wood, dir, true, [where ? `<a href="${href(where)}">GitHub</a>` : ""]);
  const readme = kids.find((kid) => kid.k === "f" && kid.n.toLowerCase() === "readme.md");
  const folders = kids.filter((kid) => kid.k === "d").length;
  const files = kids.length - folders;
  const name = dir || wood.name;
  const lead = `${folders} director${folders === 1 ? "y" : "ies"} and ${files} file${files === 1 ? "" : "s"} in ${name}.`;
  const page = (intro: string) => `${head}\n${lede(name, lead)}\n${intro}\n${rows(dir, kids)}`;
  if (!readme || (readme.s ?? 0) > HUGE) return { html: page("") };
  const path = dir ? `${dir}/${readme.n}` : readme.n;
  const more = async (tools: Tools) => {
    const body = tools.md ? await tools.load(source(path, readme)) : null;
    const text = body ? reads(body) : null;
    return text ? { html: page(await prose(wood, tools, path, text)) } : null;
  };
  return { html: page(""), more };
}

/* CODE */

const NARROW = 2;

const WIDEST = 6;

const gutter = (count: number) => Math.min(WIDEST, Math.max(NARROW, String(count).length));

export function block(text: string, kind: string): string {
  const lines = text.replace(/\r\n?/g, "\n").split("\n");
  if (lines.length > 1 && lines[lines.length - 1] === "") lines.pop();
  if (!paints(text)) return `<div class="code plain" data-lang="${escape(kind)}"><pre><code>${escape(text)}</code></pre></div>`;
  const out = lines.map((line, i) => {
    const n = i + 1;
    return `<span class="line" id="L${n}"><a class="n" href="#L${n}">${n}</a><span class="t">${escape(line)}</span></span>`;
  });
  return `<div class="code d${gutter(lines.length)}" data-lang="${escape(kind)}"><pre><code>${out.join("")}</code></pre></div>`;
}

export const paints = (text: string) => text.length <= WIDE;

/* FILE */

function file(wood: Wood, path: string, twig: Twig): { html: string; more?: View["more"] } {
  const weight = twig.s ?? 0;
  const raw = href(source(path, twig));
  const kind = ext(path);
  const name = stem(path);
  const tongue = lang(path);
  const where = github(wood, path, false);
  const head = bar(wood, path, false, [`<a href="${raw}">Raw</a>`, where ? `<a href="${href(where)}">GitHub</a>` : ""]);
  const page = (note: string, main: string) => `${head}\n${lede(name, note)}\n${main}`;
  const loose = page(`${size(weight)} · ${kind || "binary"}`, `<p class="lead"><a href="${raw}">Download ${escape(name)}</a> · ${size(weight)}</p>`);
  if (weight > HUGE) return { html: loose };
  if (kind === "pdf") return { html: page(`${size(weight)} · pdf`, `<embed class="doc" src="${raw}" type="application/pdf">`) };
  if (IMAGE.has(kind)) return { html: page(`${size(weight)} · ${kind}`, `<figure class="shot"><img src="${raw}" alt="${escape(name)}"></figure>`) };
  const more = async (tools: Tools): Promise<Full | null> => {
    const body = await tools.load(source(path, twig));
    if (!body) return { html: page(`${size(weight)} · ${tongue}`, `<p class="lead">${escape(name)} did not load. <a href="${raw}">Open the raw file</a> or reload.</p>`) };
    const text = reads(body);
    if (text === null) return { html: loose };
    if (kind === "md" && tools.md) return { html: page(`${size(weight)} · ${tongue}`, await prose(wood, tools, path, text)) };
    const count = text ? text.replace(/\n$/, "").split("\n").length : 0;
    const html = page(`${size(weight)} · ${tongue} · ${count} line${count === 1 ? "" : "s"}`, block(text, tongue));
    return paints(text) ? { html, code: { text, lang: tongue } } : { html };
  };
  return { html: page(`${size(weight)} · ${tongue}`, ""), more };
}

/* DRAW */

function lost(wood: Wood, path: string): string {
  return `${bar(wood, "", true, [])}\n${lede("Not found", `No ${path} in ${wood.name}.`)}\n<p><a href="/git/">Back to ${escape(wood.name)}</a></p>`;
}

export function draw(wood: Wood, at: string): View {
  const path = at.split("/").filter(Boolean).join("/");
  const twig = find(wood, path);
  if (!twig) return { path, name: "Not found", found: false, html: lost(wood, path) };
  if (twig.k === "d") return { path: path ? `${path}/` : "", name: path || wood.name, found: true, ...listing(wood, path, twig) };
  return { path, name: stem(path), found: true, ...file(wood, path, twig) };
}
