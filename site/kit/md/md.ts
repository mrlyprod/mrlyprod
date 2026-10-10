import type { Element, Root as Hast, RootContent as HastContent } from "hast";
import type { Heading, Nodes, Paragraph, Root } from "mdast";
import rehypeStringify from "rehype-stringify";
import remarkGfm from "remark-gfm";
import remarkMath from "remark-math";
import remarkParse from "remark-parse";
import remarkRehype from "remark-rehype";
import { unified } from "unified";
import { SKIP, visit } from "unist-util-visit";
import { escape } from "./text.ts";

/* TYPES */

export type Link = (url: string, image?: boolean) => string | null;

export type Options = {
  link?: Link;
  math?: (tex: string, display: boolean) => string;
  lazy?: boolean;
};

type Raw = { type: "raw"; value: string };

/* TEXT */

export const slug = (text: string) => text.toLowerCase().replace(/[^\p{L}\p{N} _-]/gu, "").replace(/ /g, "-");

const TEX: Record<string, string> = { lceil: "⌈", rceil: "⌉", lfloor: "⌊", rfloor: "⌋", ne: "≠", neq: "≠", le: "≤", leq: "≤", ge: "≥", geq: "≥", times: "×", infty: "∞", pi: "π", cdot: "·", to: "→", left: "", right: "" };

export function plain(text: string): string {
  return text
    .replace(/\\([a-zA-Z]+)\s?/g, (m, name: string) => (name in TEX ? TEX[name]! : m))
    .replace(/\s+([⌉⌋])/g, "$1")
    .replace(/!\[([^\]]*)\]\([^)]*\)/g, "$1")
    .replace(/\[([^\]]*)\]\([^)]*\)/g, "$1")
    .replace(/`([^`]*)`/g, "$1")
    .replace(/\*\*([^*]*)\*\*/g, "$1")
    .replace(/\*([^*]*)\*/g, "$1")
    .replace(/\$\$?([^$]*)\$\$?/g, "$1")
    .replace(/\s+/g, " ")
    .trim();
}

const STOP = /[.!?]["'”’)]*(?= (?![a-z]))/g;

const SHORT = /(?<!\p{L})(?:\p{Lu}\p{L}{0,2}\. \p{Lu}|\p{L}{1,3}\. \p{N})$/u;

export function summary(md: string, max = 200): string {
  const skip = /^(#{1,6} |```|\$\$|\||!\[)/;
  const line = md.split("\n").find((l) => l.trim() && !skip.test(l)) ?? "";
  const text = plain(line.replace(/^(- |\d+\. |> )/, ""));
  if (text.length <= max) return text;
  const ends = [...text.matchAll(STOP)].filter((hit) => hit[0] !== "." || !SHORT.test(text.slice(0, hit.index + 3)));
  const stop = ends.map((hit) => hit.index + hit[0].length).filter((end) => end <= max).at(-1);
  if (stop !== undefined) return text.slice(0, stop);
  const cut = text.lastIndexOf(" ", max);
  return text.slice(0, cut > 0 ? cut : max);
}

export function front(md: string): { data: Record<string, string>; body: string } {
  const m = md.match(/^---\n([\s\S]*?)\n---\n?/);
  if (!m) return { data: {}, body: md };
  const data: Record<string, string> = {};
  for (const line of m[1]!.split("\n")) {
    const at = line.indexOf(":");
    if (at > 0) data[line.slice(0, at).trim()] = line.slice(at + 1).trim();
  }
  return { data, body: md.slice(m[0].length) };
}

/* SHAPE */

const WORD = /[\p{L}\p{N}]/u;

const word = (code: number | null) => code !== null && code > 0 && WORD.test(String.fromCharCode(code));

type Step = (code: number | null) => Step | undefined;

type Effects = { enter: (type: string) => void; exit: (type: string) => void; consume: (code: number | null) => void };

const STAR = {
  name: "star",
  tokenize(this: { previous: number | null }, effects: Effects, ok: Step, nok: Step): Step {
    const before = this.previous;
    return (code) => {
      if (!word(before)) return nok(code);
      effects.enter("data");
      effects.consume(code);
      effects.exit("data");
      return (next) => (word(next) ? ok(next) : nok(next));
    };
  },
};

function stars(this: { data: () => { micromarkExtensions?: unknown[] } }) {
  const data = this.data();
  (data.micromarkExtensions ??= []).push({ text: { 42: STAR } });
}

const unescape = (tex: string) => tex.replace(/\\\\([!-\/:-@[-`{-~])/g, "\\$1");

const FIGURE = /^!\[([^\]]*)\]\(([^)]*)\)$/;

export const NAME = /^[a-z0-9-]+$/;

const source = (src: string, node: Nodes) => src.slice(node.position!.start.offset, node.position!.end.offset);

const headingText = (src: string, node: Heading) =>
  source(src, node).replace(/^#{1,6}[ \t]+/, "").replace(/[ \t]+#+[ \t]*$/, "").replace(/\n[ \t]*[=-]+[ \t]*$/, "").trim();

const LAZY = ' loading="lazy" decoding="async"';

const lone = (src: string, node: Paragraph) =>
  node.children.length === 1 && node.children[0]!.type === "image" ? (source(src, node).match(FIGURE)?.slice(1) ?? null) : null;

function figure(src: string, node: Paragraph, opts: Options, lazy: boolean): Raw | null {
  const found = lone(src, node);
  if (!found) return null;
  const caption = `<figcaption>${inline(found[0]!, opts)}</figcaption>`;
  const target = found[1]!.trim();
  const alt = escape((node.children[0] as { alt?: string }).alt ?? "");
  if (NAME.test(target)) return { type: "raw", value: `<figure data-figure="${target}"><canvas role="img" aria-label="${alt}"></canvas>${caption}</figure>` };
  const href = opts.link?.(target, true) ?? target;
  return { type: "raw", value: `<figure><img src="${escape(href)}" alt="${alt}"${lazy ? LAZY : ""}>${caption}</figure>` };
}

function shape(src: string, opts: Options) {
  return (tree: Root) => {
    const ids = new Map<string, number>();
    const refs = new Set<string>();
    visit(tree, "imageReference", (node) => void refs.add(node.identifier));
    const gone = new Set<string>();
    if (opts.link)
      visit(tree, "definition", (node) => {
        const to = opts.link!(node.url, refs.has(node.identifier));
        if (to === null) gone.add(node.identifier);
        node.url = to ?? node.url;
      });
    let images = opts.lazy ? 1 : 0;
    visit(tree, (node, index, parent) => {
      if (node.type === "heading") {
        const base = slug(plain(headingText(src, node)));
        const seen = ids.get(base) ?? 0;
        ids.set(base, seen + 1);
        node.data = { ...node.data, hProperties: { id: seen ? `${base}-${seen}` : base } };
      } else if (node.type === "list") node.spread = false;
      else if (node.type === "listItem") node.spread = false;
      else if (node.type === "image" || node.type === "imageReference") {
        if (node.type === "image" && opts.link) node.url = opts.link(node.url, true) ?? node.url;
        if (images++) node.data = { ...node.data, hProperties: { loading: "lazy", decoding: "async" } };
      } else if ((node.type === "link" && opts.link) || (node.type === "linkReference" && gone.has(node.identifier))) {
        const to = node.type === "link" ? opts.link!(node.url, false) : null;
        if (node.type === "link" && to !== null) node.url = to;
        else if (parent && index !== undefined) {
          parent.children.splice(index, 1, ...(node.children as never[]));
          return index;
        }
      } else if (node.type === "paragraph" && parent && index !== undefined) {
        const only = node.children.length === 1 ? node.children[0]! : null;
        if (only && only.type === "inlineMath" && source(src, node).startsWith("$$")) {
          parent.children[index] = { type: "math", value: only.value };
          return;
        }
        const raw = figure(src, node, opts, images > 0);
        if (!raw) return;
        parent.children[index] = raw as never;
        images++;
        return SKIP;
      }
    });
  };
}

/* TABLES */

const blank = (node: HastContent) => node.type === "text" && !node.value.trim();

function tables() {
  return (tree: Hast) => {
    visit(tree, "element", (node: Element, index, parent) => {
      if (node.tagName === "th" || node.tagName === "td") {
        const align = node.properties.align;
        delete node.properties.align;
        if (align === "center" || align === "right") node.properties.className = [align];
      } else if (node.tagName === "tr" || node.tagName === "thead") node.children = node.children.filter((kid) => !blank(kid));
      else if (node.tagName === "table" && parent && index !== undefined) {
        node.children = node.children.filter((kid) => !blank(kid));
        parent.children[index] = { type: "element", tagName: "div", properties: { className: ["table"] }, children: [node] };
      }
    });
  };
}

/* RENDER */

const code = (tex: string) => `<code>${escape(tex)}</code>`;

function parser(opts: Options) {
  const chain = unified().use(remarkParse).use(remarkGfm).use(stars);
  return opts.math ? chain.use(remarkMath) : chain;
}

export function render(md: string, opts: Options = {}): string {
  const src = md.replace(/\r\n?/g, "\n");
  const math = opts.math ?? code;
  const handlers = {
    math: (_: unknown, node: { value: string }) => ({ type: "raw", value: math(unescape(node.value), true) }),
    inlineMath: (_: unknown, node: { value: string }) => ({ type: "raw", value: math(unescape(node.value), false) }),
    html: (_: unknown, node: { value: string }) => ({ type: "text", value: node.value }),
    raw: (_: unknown, node: { value: string }) => ({ type: "raw", value: node.value }),
  };
  const out = parser(opts)
    .use(shape, src, opts)
    .use(remarkRehype, { allowDangerousHtml: true, handlers: handlers as never })
    .use(tables)
    .use(rehypeStringify, { allowDangerousHtml: true, characterReferences: { useNamedReferences: true } })
    .processSync(src);
  return String(out).replace(/\n$/, "");
}

export function inline(text: string, opts: Options = {}): string {
  const html = render(text, opts);
  const m = html.match(/^<p>([\s\S]*)<\/p>$/);
  return m ? m[1]! : html;
}

/* FIGURES */

export function figures(md: string): string[] {
  const src = md.replace(/\r\n?/g, "\n");
  const out: string[] = [];
  visit(parser({}).parse(src), "paragraph", (node: Paragraph) => {
    const target = lone(src, node)?.[1]?.trim() ?? "";
    if (NAME.test(target)) out.push(target);
  });
  return out;
}
