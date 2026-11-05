import type { Element, Root as Hast, RootContent as HastContent } from "hast";
import type { Heading, ListItem, Nodes, Paragraph, Root } from "mdast";
import rehypeStringify from "rehype-stringify";
import remarkGfm from "remark-gfm";
import remarkMath from "remark-math";
import remarkParse from "remark-parse";
import remarkRehype from "remark-rehype";
import { unified } from "unified";
import { SKIP, visit } from "unist-util-visit";
import type { Link } from "../../../kit/md/md.ts";
import { NAME, plain, slug } from "../../../kit/md/md.ts";
import { esc } from "../../../kit/md/text.ts";

export type Options = {
  link?: Link;
  math?: (tex: string, display: boolean) => string;
  lazy?: boolean;
};

type Raw = { type: "raw"; value: string };

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
  data.micromarkExtensions ??= [];
  data.micromarkExtensions.push({ text: { 42: STAR } });
}

const unesc = (tex: string) => tex.replace(/\\\\([!-/:-@[-`{-~])/g, "\\$1");

const FIGURE = /^!\[((?:\\.|[^[\]\\]|\[(?:\\.|[^[\]\\])*\])*)\]\(([^)]*)\)$/;

const source = (src: string, node: Nodes) => src.slice(node.position!.start.offset, node.position!.end.offset);

const headingText = (src: string, node: Heading) =>
  source(src, node)
    .replace(/^#{1,6}[ \t]+/, "")
    .replace(/[ \t]+#+[ \t]*$/, "")
    .replace(/\n[ \t]*[=-]+[ \t]*$/, "")
    .trim();

const LAZY = ' loading="lazy" decoding="async"';

const lone = (src: string, node: Paragraph) =>
  node.children.length === 1 && node.children[0]!.type === "image"
    ? (source(src, node).match(FIGURE)?.slice(1) ?? null)
    : null;

function figure(src: string, node: Paragraph, opts: Options, lazy: boolean): Raw | null {
  const found = lone(src, node);
  if (!found) return null;
  const caption = `<figcaption>${inline(found[0]!, opts)}</figcaption>`;
  const target = found[1]!.trim();
  const alt = esc((node.children[0] as { alt?: string }).alt ?? "");
  if (NAME.test(target))
    return {
      type: "raw",
      value: `<figure data-figure="${target}"><canvas role="img" aria-label="${alt}"></canvas>${caption}</figure>`,
    };
  const href = opts.link?.(target, true) ?? target;
  return {
    type: "raw",
    value: `<figure><img src="${esc(href)}" alt="${alt}"${lazy ? LAZY : ""}>${caption}</figure>`,
  };
}

const led = (item: ListItem) => {
  const first = item.children[0];
  return first?.type === "paragraph" && first.children[0]?.type === "link";
};

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
      } else if (node.type === "list") {
        node.spread = false;
        if (node.children.length > 1 && node.children.every(led))
          node.data = { ...node.data, hProperties: { className: ["links"] } };
      } else if (node.type === "listItem") node.spread = false;
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

const blank = (node: HastContent) => node.type === "text" && !node.value.trim();

function tables() {
  return (tree: Hast) => {
    visit(tree, "element", (node: Element, index, parent) => {
      if (node.tagName === "th" || node.tagName === "td") {
        const align = node.properties.align;
        delete node.properties.align;
        if (align === "center" || align === "right") node.properties.className = [align];
      } else if (node.tagName === "tr" || node.tagName === "thead")
        node.children = node.children.filter((kid) => !blank(kid));
      else if (node.tagName === "table" && parent && index !== undefined) {
        node.children = node.children.filter((kid) => !blank(kid));
        parent.children[index] = {
          type: "element",
          tagName: "div",
          properties: { className: ["table"] },
          children: [node],
        };
      }
    });
  };
}

const code = (tex: string) => `<code>${esc(tex)}</code>`;

function parser(opts: Options) {
  const chain = unified().use(remarkParse).use(remarkGfm).use(stars);
  return opts.math ? chain.use(remarkMath) : chain;
}

export function render(md: string, opts: Options = {}): string {
  const src = md.replace(/\r\n?/g, "\n");
  const math = opts.math ?? code;
  const handlers = {
    math: (_: unknown, node: { value: string }) => ({ type: "raw", value: math(unesc(node.value), true) }),
    inlineMath: (_: unknown, node: { value: string }) => ({ type: "raw", value: math(unesc(node.value), false) }),
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

export async function mathml(text: string): Promise<Options["math"]> {
  if (!text.includes("$")) return undefined;
  const { default: katex } = await import("katex");
  return (tex, display) => katex.renderToString(tex, { output: "mathml", throwOnError: false, displayMode: display });
}
