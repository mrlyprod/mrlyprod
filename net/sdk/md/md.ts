/* TYPES */

export type Link = (url: string, image?: boolean) => string | null;

export type Lazy = { link?: Link; lazy?: boolean };

export type Reader = { markdown: (text: string, opts?: Lazy) => Promise<string> };

/* TEXT */

export const slug = (text: string) =>
  text
    .toLowerCase()
    .replace(/[^\p{L}\p{N} _-]/gu, "")
    .replace(/ /g, "-");

const TEX: Record<string, string> = {
  lceil: "⌈",
  rceil: "⌉",
  lfloor: "⌊",
  rfloor: "⌋",
  ne: "≠",
  neq: "≠",
  le: "≤",
  leq: "≤",
  ge: "≥",
  geq: "≥",
  times: "×",
  infty: "∞",
  pi: "π",
  cdot: "·",
  to: "→",
  left: "",
  right: "",
};

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
  const stop = ends
    .map((hit) => hit.index + hit[0].length)
    .filter((end) => end <= max)
    .at(-1);
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

/* FIGURES */

export const NAME = /^[a-z0-9-]+$/;

const FIGURE = /^!\[(?:\\.|[^[\]\\]|\[(?:\\.|[^[\]\\])*\])*\]\(([^)]*)\)$/;

const FENCE = /^ {0,3}(`{3,}|~{3,})/;

const HEAD = /^ {0,3}#{1,6}(?: |$)/;

const LEAD = /^ {0,3}(?:(?:> ?|[-*+] |\d+\. ) {0,3})*/;

export function figures(md: string): string[] {
  const out: string[] = [];
  const para: string[] = [];
  let fence: string | null = null;
  const flush = () => {
    if (para.length === 1) {
      const target = para[0]!.replace(LEAD, "").trim().match(FIGURE)?.[1]?.trim() ?? "";
      if (NAME.test(target)) out.push(target);
    }
    para.length = 0;
  };
  for (const line of md.split(/\r\n?|\n/)) {
    const tick = line.match(FENCE)?.[1];
    if (fence) {
      if (tick && tick[0] === fence[0] && tick.length >= fence.length) fence = null;
      continue;
    }
    if (tick) {
      flush();
      fence = tick;
    } else if (!line.trim() || HEAD.test(line)) flush();
    else para.push(line);
  }
  flush();
  return out;
}

/* READER */

export function reader<R = Reader>(): Promise<R> {
  return import("site:apps").then(({ apps }) => {
    const open = (apps as Partial<typeof apps>).preview;
    if (!open) throw new Error("md: this site has no preview route; a root holding net/system/preview draws markdown");
    return open() as Promise<R>;
  });
}
