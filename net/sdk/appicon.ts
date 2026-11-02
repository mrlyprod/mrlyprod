/* TYPES */

export type Tag = { name: string; attrs: [string, string][]; kids: Tag[] };

export type Icon = { defs: string; art: string };

type Matrix = number[];

type Pt = [number, number];

type Named = Map<string, Tag>;

type Defs = Map<string | undefined, Tag>;

type Last = { cubic?: Pt; quad?: Pt };

type Kind = "matrix" | "translate" | "scale" | "rotate" | "skewX" | "skewY";

type Stop = [number, string] | [number, string, number];

type Ramp = [number, number][];

export const SIDE = 1024;

export const LOOKS = ["default", "dark", "clear-light", "clear-dark", "tinted-light", "tinted-dark"];

const APPLE =
  "M39.75 0L60.25 0C71.7 0 77.42 0 82.58 1.7L83.58 1.95C90.31 4.4 95.6 9.69 98.05 16.42C100 22.58 100 28.3 100 39.75L100 60.25C100 71.7 100 77.42 98.3 82.58L98.05 83.58C95.6 90.31 90.31 95.6 83.58 98.05C77.42 100 71.7 100 60.25 100L39.75 100C28.3 100 22.58 100 17.42 98.3L16.42 98.05C9.69 95.6 4.4 90.31 1.95 83.58C0 77.42 0 71.7 0 60.25L0 39.75C0 28.3 0 22.58 1.7 17.42L1.95 16.42C4.4 9.69 9.69 4.4 16.42 1.95C22.58 0 28.3 0 39.75 0Z";

const scale = (k: number) => APPLE.replace(/\d+(?:\.\d+)?/g, (x) => String(Math.round(Number(x) * k * 1e4) / 1e4));

export const SHAPE = scale(SIDE / 100);

export const BOX = scale(1 / 100);

const DEFS = new Set([
  "linearGradient",
  "radialGradient",
  "stop",
  "clipPath",
  "mask",
  "filter",
  "feGaussianBlur",
  "feOffset",
  "feFlood",
  "feComposite",
  "feMerge",
  "feMergeNode",
  "feColorMatrix",
  "feBlend",
  "feMorphology",
  "feComponentTransfer",
  "feFuncR",
  "feFuncG",
  "feFuncB",
  "feFuncA",
]);
const SHAPES = new Set(["g", "path", "rect", "circle", "ellipse", "line", "polyline", "polygon", "use"]);
const WRAPS = new Set(["linearGradient", "radialGradient", "clipPath", "mask", "filter"]);
const ATTR = /^(?:[a-z][a-z0-9-]*|[a-zA-Z]+|xlink:href)$/;
const BANNED = /^(?:on|style$|class$|xmlns|xml:)/;
const MONO = new Set(["hide", "full", "dim"]);
const LINKS = /^(?:fill|stroke|stop-color|flood-color|clip-path|mask|filter|data-dark|data-dark-stroke|data-key)$/;
const WHITE = /^(?:#fff|#ffffff|white)$/i;

const fail = (id: string, text: string) => new Error(`appicon: ${id}/icon.svg ${text}`);

/* PARSE */

export function parse(text: string, id = "?"): Tag {
  const src = text.replace(/<\?xml[\s\S]*?\?>/g, "").replace(/<!--[\s\S]*?-->/g, "");
  const root: Tag = { name: "", attrs: [], kids: [] };
  const stack = [root];
  const tag = /<(\/?)([a-zA-Z][\w:-]*)((?:\s+[\w:-]+\s*=\s*(?:"[^"]*"|'[^']*'))*)\s*(\/?)>/g;
  let at = 0;
  for (let m = tag.exec(src); m; m = tag.exec(src)) {
    if (src.slice(at, m.index).trim()) throw fail(id, `holds text before <${m[2]}>; an icon is shapes only`);
    at = tag.lastIndex;
    const [, close, name, rest, self] = m;
    if (close) {
      const open = stack.pop();
      if (open?.name !== name) throw fail(id, `closes <${name}> while <${open?.name}> is open`);
      continue;
    }
    const attrs = [...rest.matchAll(/([\w:-]+)\s*=\s*(?:"([^"]*)"|'([^']*)')/g)].map(
      ([, key, a, b]): [string, string] => [key, a ?? b],
    );
    const node: Tag = { name, attrs, kids: [] };
    stack.at(-1)!.kids.push(node);
    if (!self) stack.push(node);
  }
  if (src.slice(at).trim()) throw fail(id, "ends in text or a broken tag");
  if (stack.length !== 1) throw fail(id, `never closes <${stack.at(-1)!.name}>`);
  if (root.kids.length !== 1 || root.kids[0].name !== "svg") throw fail(id, "is not one <svg>");
  return root.kids[0];
}

const get = (node: Tag | undefined, key: string) => node?.attrs.find(([k]) => k === key)?.[1];

const esc = (value: string) =>
  value
    .replace(/&(?!(?:[a-z]+|#\d+|#x[\da-f]+);)/gi, "&amp;")
    .replace(/"/g, "&quot;")
    .replace(/</g, "&lt;");

function write(node: Tag): string {
  const attrs = node.attrs.map(([k, v]) => ` ${k}="${esc(v)}"`).join("");
  return node.kids.length
    ? `<${node.name}${attrs}>${node.kids.map(write).join("")}</${node.name}>`
    : `<${node.name}${attrs}/>`;
}

/* FACE */

function check(id: string, svg: Tag): Set<string> {
  if (
    get(svg, "viewBox")
      ?.trim()
      .split(/[\s,]+/)
      .join(" ") !== `0 0 ${SIDE} ${SIDE}`
  )
    throw fail(id, `needs viewBox="0 0 ${SIDE} ${SIDE}"`);
  const ids = new Set<string>();
  const walk = (node: Tag, inDefs: boolean) => {
    for (const kid of node.kids) {
      if (!DEFS.has(kid.name) && !SHAPES.has(kid.name) && kid.name !== "defs")
        throw fail(
          id,
          `uses <${kid.name}>; an icon draws with ${[...SHAPES].join(", ")} and keeps paint servers in <defs>`,
        );
      if (WRAPS.has(kid.name) && !inDefs) throw fail(id, `puts <${kid.name}> outside <defs>`);
      if (kid.name === "defs" && node !== svg) throw fail(id, "nests <defs>; keep one at the top");
      for (const [key, value] of kid.attrs) {
        if (BANNED.test(key) && !(key === "class" && value === "back" && node === svg))
          throw fail(id, `sets ${key} on <${kid.name}>; CSS owns the looks, so no style, class or handlers`);
        if (!ATTR.test(key)) throw fail(id, `sets the unknown attribute ${key} on <${kid.name}>`);
        if (key === "id") {
          if (!/^[\w-]+$/.test(value)) throw fail(id, `names an id ${value}; ids are letters, digits, - and _`);
          if (ids.has(value)) throw fail(id, `repeats the id ${value}`);
          ids.add(value);
        }
        if ((key === "href" || key === "xlink:href") && !value.startsWith("#"))
          throw fail(id, `links ${value}; only #ids inside the icon`);
        if (key === "data-mono" && !MONO.has(value))
          throw fail(id, `sets data-mono="${value}"; it is ${[...MONO].join(", ")}`);
        if (key.startsWith("data-") && !["data-mono", "data-dark", "data-dark-stroke", "data-key"].includes(key))
          throw fail(id, `sets ${key}; the data attributes are data-dark, data-dark-stroke, data-mono and data-key`);
        if (/url\((?!#)/.test(value)) throw fail(id, `points ${key} outside the icon`);
      }
      walk(kid, inDefs || kid.name === "defs");
    }
  };
  walk(svg, false);
  return ids;
}

export function appicon(id: string, text: string): Icon {
  const svg = parse(text, id);
  const ids = check(id, svg);
  const lost = [...text.matchAll(/url\(#([\w-]+)\)|href="#([\w-]+)"/g)]
    .map((m) => m[1] ?? m[2])
    .find((name) => !ids.has(name));
  if (lost) throw fail(id, `points at #${lost}, which it never defines`);
  if (svg.kids.slice(1).some((kid) => kid.name === "defs")) throw fail(id, "puts <defs> after a shape; it comes first");
  const body = svg.kids.filter((kid) => kid.name !== "defs");
  if (get(body[0], "class") !== "back") throw fail(id, 'starts with no class="back"; the first shape is the plate');
  if (!body[1]) throw fail(id, "draws nothing on its plate");
  const local = (value: string) => value.replace(/url\(#([\w-]+)\)/g, (_m, name: string) => `url(#${id}-${name})`);
  const defs: Defs = new Map(
    svg.kids
      .filter((kid) => kid.name === "defs")
      .flatMap((kid) => kid.kids)
      .map((kid): [string | undefined, Tag] => [get(kid, "id"), kid]),
  );
  const key = get(body[0], "data-key") ?? keyOf(get(body[0], "fill"), defs);
  if (!key) throw fail(id, "has a back whose fill gives no key colour; set data-key on the back");
  const named: Named = new Map();
  const find = (node: Tag) => {
    if (get(node, "id")) named.set(get(node, "id")!, node);
    node.kids.forEach(find);
  };
  find(svg);
  const box: Pt = [Infinity, -Infinity];
  const turns: Matrix[] = [];
  const slot = (m: Matrix) => {
    if (plain(m)) return `appicon-${id}-w`;
    const at = turns.findIndex((n) => same(n, m));
    if (at < 0) turns.push(m);
    return `appicon-${id}-w${at < 0 ? turns.length : at + 1}`;
  };
  const dress = (node: Tag, ctm: Matrix | null, white: boolean, scope: Matrix): Tag => {
    const attrs: [string, string][] = [];
    const style: string[] = [];
    const classes: string[] = [];
    for (const [k, v] of node.attrs) {
      const value = LINKS.test(k) ? local(v) : v;
      if (k === "href" || k === "xlink:href") attrs.push(["href", `#${id}-${v.slice(1)}`]);
      else if (k === "id") attrs.push(["id", `${id}-${v}`]);
      else if (k === "class") classes.push(v);
      else if (k === "data-key") continue;
      else if (k === "data-dark") {
        style.push(`--d:${value}`);
        classes.push("d");
      } else if (k === "data-dark-stroke") {
        style.push(`--ds:${value}`);
        classes.push("ds");
      } else attrs.push([k, value]);
    }
    const own = get(node, "fill");
    const here = ctm && get(node, "transform") ? times(ctm, transform(get(node, "transform")!)) : ctm;
    const paint = Boolean(ctm) && !classes.length && (own ? WHITE.test(own) : white);
    let space = here;
    if (paint) {
      reach(node, here!, named, box);
      const inside = node.name === "use" ? inner(node, named) : IDENTITY;
      if (!inside)
        throw fail(
          id,
          `turns the shapes of its white <use href="${get(node, "href") ?? get(node, "xlink:href")}"> different ways; the Dark fade needs one space per <use>`,
        );
      space = times(here!, inside);
    }
    const w = paint && (own || !same(space!, scope));
    if (w) {
      classes.push("w");
      if (!same(space!, scope)) style.push(`--w:url(#${slot(space!)})`);
    }
    if (classes.length) attrs.unshift(["class", classes.join(" ")]);
    if (style.length) attrs.push(["style", style.join(";")]);
    return { ...node, attrs, kids: node.kids.map((kid) => dress(kid, here, paint, w ? space! : scope)) };
  };
  const bare = (node: Tag): Tag => ({
    ...node,
    attrs: node.attrs.filter(([k]) => !["id", "class", "style"].includes(k)),
    kids: node.kids.map(bare),
  });
  const [back, ...ink] = body.map((node, at) => dress(node, at ? IDENTITY : null, false, IDENTITY));
  const glyph = ink.map(write).join("");
  const whites =
    box[0] < box[1]
      ? [IDENTITY, ...turns].map(
          (m, at) =>
            `<linearGradient id="appicon-${id}-w${at || ""}" gradientUnits="userSpaceOnUse" x1="0" y1="${round(box[0])}" x2="0" y2="${round(box[1])}"${at ? ` gradientTransform="matrix(${invert(m).map(round).join(" ")})"` : ""}>${stops(
              [
                [0, `color-mix(in oklab, ${key} 35%, #212121)`],
                [1, key],
              ],
            )}</linearGradient>`,
        )
      : [];
  return {
    defs: [...[...defs.values()].map((node) => write(dress(node, null, false, IDENTITY))), ...whites].join(""),
    art: `${write(back)}<g class="lift">${ink.map(bare).map(write).join("")}</g><g class="ink" mask="url(#appicon-fade)"${whites.length ? ` style="--w:url(#appicon-${id}-w)"` : ""}><g style="isolation:isolate">${glyph}</g></g>`,
  };
}

function keyOf(fill: string | undefined, defs: Defs): string | null {
  if (!fill) return null;
  const at = /^url\(#([\w-]+)\)$/.exec(fill);
  if (!at) return fill;
  const stops = defs.get(at[1])?.kids.filter((kid) => kid.name === "stop") ?? [];
  return get(stops.at(-1), "stop-color") ?? null;
}

/* BOX */

const IDENTITY: Matrix = [1, 0, 0, 1, 0, 0];

const round = (x: number) => Math.round(x * 1e4) / 1e4 || 0;

const same = (a: Matrix, b: Matrix) => a.every((x, at) => Math.abs(x - b[at]) < 1e-9);

const plain = (m: Matrix) => same(m, IDENTITY);

const times = (a: Matrix, b: Matrix): Matrix => [
  a[0] * b[0] + a[2] * b[1],
  a[1] * b[0] + a[3] * b[1],
  a[0] * b[2] + a[2] * b[3],
  a[1] * b[2] + a[3] * b[3],
  a[0] * b[4] + a[2] * b[5] + a[4],
  a[1] * b[4] + a[3] * b[5] + a[5],
];

function invert([a, b, c, d, e, f]: Matrix): Matrix {
  const det = a * d - b * c;
  return [d / det, -b / det, -c / det, a / det, (c * f - d * e) / det, (b * e - a * f) / det];
}

const numbers = (text: string | undefined) =>
  (text ?? "").match(/[-+]?(?:\d*\.\d+|\d+\.?)(?:e[-+]?\d+)?/gi)?.map(Number) ?? [];

function transform(text: string): Matrix {
  let m = IDENTITY;
  for (const [, name, args] of text.matchAll(/(matrix|translate|scale|rotate|skewX|skewY)\s*\(([^)]*)\)/g)) {
    const n = numbers(args);
    const rad = ((n[0] ?? 0) * Math.PI) / 180;
    const step = {
      matrix: () => n.slice(0, 6),
      translate: () => [1, 0, 0, 1, n[0], n[1] ?? 0],
      scale: () => [n[0], 0, 0, n[1] ?? n[0], 0, 0],
      rotate: () =>
        times(
          times(
            [1, 0, 0, 1, n[1] ?? 0, n[2] ?? 0],
            [Math.cos(rad), Math.sin(rad), -Math.sin(rad), Math.cos(rad), 0, 0],
          ),
          [1, 0, 0, 1, -(n[1] ?? 0), -(n[2] ?? 0)],
        ),
      skewX: () => [1, 0, Math.tan(rad), 1, 0, 0],
      skewY: () => [1, Math.tan(rad), 0, 1, 0, 0],
    }[name as Kind]();
    m = times(m, step);
  }
  return m;
}

const STEPS = 48;

const ring = (cx: number, cy: number, rx: number, ry: number): Pt[] =>
  Array.from(
    { length: STEPS },
    (_, k): Pt => [cx + rx * Math.cos((2 * Math.PI * k) / STEPS), cy + ry * Math.sin((2 * Math.PI * k) / STEPS)],
  );

function arc([x1, y1]: Pt, rx: number, ry: number, phi: number, large: number, sweep: number, [x2, y2]: Pt): Pt[] {
  rx = Math.abs(rx);
  ry = Math.abs(ry);
  if (!rx || !ry) return [[x2, y2]];
  const cos = Math.cos((phi * Math.PI) / 180);
  const sin = Math.sin((phi * Math.PI) / 180);
  const dx = (x1 - x2) / 2;
  const dy = (y1 - y2) / 2;
  const xp = cos * dx + sin * dy;
  const yp = -sin * dx + cos * dy;
  const grow = Math.sqrt(Math.max(1, (xp * xp) / (rx * rx) + (yp * yp) / (ry * ry)));
  rx *= grow;
  ry *= grow;
  const num = rx * rx * ry * ry - rx * rx * yp * yp - ry * ry * xp * xp;
  const co = (large === sweep ? -1 : 1) * Math.sqrt(Math.max(0, num / (rx * rx * yp * yp + ry * ry * xp * xp)));
  const cxp = (co * rx * yp) / ry;
  const cyp = (-co * ry * xp) / rx;
  const cx = cos * cxp - sin * cyp + (x1 + x2) / 2;
  const cy = sin * cxp + cos * cyp + (y1 + y2) / 2;
  const angle = (ux: number, uy: number, vx: number, vy: number) => Math.atan2(ux * vy - uy * vx, ux * vx + uy * vy);
  const t1 = angle(1, 0, (xp - cxp) / rx, (yp - cyp) / ry);
  let dt = angle((xp - cxp) / rx, (yp - cyp) / ry, (-xp - cxp) / rx, (-yp - cyp) / ry);
  if (!sweep && dt > 0) dt -= 2 * Math.PI;
  if (sweep && dt < 0) dt += 2 * Math.PI;
  return Array.from({ length: STEPS + 1 }, (_, k): Pt => {
    const t = t1 + (dt * k) / STEPS;
    return [cx + rx * Math.cos(t) * cos - ry * Math.sin(t) * sin, cy + rx * Math.cos(t) * sin + ry * Math.sin(t) * cos];
  });
}

const bezier = (points: Pt[]): Pt[] =>
  Array.from({ length: STEPS + 1 }, (_, k) => {
    const t = k / STEPS;
    let row = points;
    while (row.length > 1)
      row = row
        .slice(1)
        .map(([x, y], at): Pt => [row[at][0] + (x - row[at][0]) * t, row[at][1] + (y - row[at][1]) * t]);
    return row[0];
  });

function trace(d: string): Pt[] {
  const tokens = d.match(/[a-df-z]|[-+]?(?:\d*\.\d+|\d+\.?)(?:e[-+]?\d+)?/gi) ?? [];
  const out: Pt[] = [];
  let at = 0;
  let cmd = "";
  let pen: Pt = [0, 0];
  let start: Pt = [0, 0];
  let last: Last | null = null;
  const num = () => Number(tokens[at++]);
  const flag = () => {
    const token = tokens[at];
    if (token.length > 1 && /^[01]/.test(token)) {
      tokens[at] = token.slice(1);
      return Number(token[0]);
    }
    at++;
    return Number(token);
  };
  const point = (rel: boolean): Pt => {
    const x = num();
    const y = num();
    return rel ? [pen[0] + x, pen[1] + y] : [x, y];
  };
  while (at < tokens.length) {
    if (/[a-z]/i.test(tokens[at])) cmd = tokens[at++];
    const rel = cmd === cmd.toLowerCase();
    const up = cmd.toUpperCase();
    let next = pen;
    let control: Pt | Last | null = null;
    if (up === "M") {
      next = point(rel);
      start = next;
      out.push(next);
      cmd = rel ? "l" : "L";
    } else if (up === "L") {
      next = point(rel);
      out.push(next);
    } else if (up === "H") {
      next = [(rel ? pen[0] : 0) + num(), pen[1]];
      out.push(next);
    } else if (up === "V") {
      next = [pen[0], (rel ? pen[1] : 0) + num()];
      out.push(next);
    } else if (up === "Z") {
      next = start;
      if (!/[a-z]/i.test(tokens[at] ?? "z")) break;
    } else if (up === "C" || up === "S") {
      const first: Pt =
        up === "C" ? point(rel) : last?.cubic ? [2 * pen[0] - last.cubic[0], 2 * pen[1] - last.cubic[1]] : pen;
      control = point(rel);
      next = point(rel);
      out.push(...bezier([pen, first, control, next]));
      control = { cubic: control };
    } else if (up === "Q" || up === "T") {
      const mid: Pt =
        up === "Q" ? point(rel) : last?.quad ? [2 * pen[0] - last.quad[0], 2 * pen[1] - last.quad[1]] : pen;
      next = point(rel);
      out.push(...bezier([pen, mid, next]));
      control = { quad: mid };
    } else if (up === "A") {
      const rx = num();
      const ry = num();
      const phi = num();
      const large = flag();
      const sweep = flag();
      next = point(rel);
      out.push(...arc(pen, rx, ry, phi, large, sweep, next));
    } else break;
    last = control;
    pen = next;
  }
  return out;
}

function shape(node: Tag): Pt[] {
  const n = (key: string) => Number(get(node, key) ?? 0);
  if (node.name === "path") return trace(get(node, "d") ?? "");
  if (node.name === "rect")
    return [
      [n("x"), n("y")],
      [n("x") + n("width"), n("y") + n("height")],
      [n("x"), n("y") + n("height")],
      [n("x") + n("width"), n("y")],
    ];
  if (node.name === "circle") return ring(n("cx"), n("cy"), n("r"), n("r"));
  if (node.name === "ellipse") return ring(n("cx"), n("cy"), n("rx"), n("ry"));
  if (node.name === "line")
    return [
      [n("x1"), n("y1")],
      [n("x2"), n("y2")],
    ];
  if (node.name === "polyline" || node.name === "polygon")
    return numbers(get(node, "points")).flatMap((x, at, all): Pt[] => (at % 2 ? [] : [[x, all[at + 1]]]));
  return [];
}

function reach(node: Tag, ctm: Matrix, named: Named, box: Pt, depth = 0) {
  for (const [x, y] of shape(node)) {
    const top = ctm[1] * x + ctm[3] * y + ctm[5];
    if (!Number.isFinite(top)) continue;
    box[0] = Math.min(box[0], top);
    box[1] = Math.max(box[1], top);
  }
  const link = get(node, "href") ?? get(node, "xlink:href");
  if (node.name === "use" && link && depth < 8) {
    const target = named.get(link.slice(1));
    const at = times(ctm, [1, 0, 0, 1, Number(get(node, "x") ?? 0), Number(get(node, "y") ?? 0)]);
    if (target)
      reach(
        target,
        get(target, "transform") ? times(at, transform(get(target, "transform")!)) : at,
        named,
        box,
        depth + 1,
      );
  }
  if (depth)
    for (const kid of node.kids)
      reach(kid, get(kid, "transform") ? times(ctm, transform(get(kid, "transform")!)) : ctm, named, box, depth);
}

function inner(node: Tag, named: Named, depth = 0): Matrix | null {
  if (node.name === "use") {
    const target = named.get((get(node, "href") ?? get(node, "xlink:href") ?? "").slice(1));
    const at = [1, 0, 0, 1, Number(get(node, "x") ?? 0), Number(get(node, "y") ?? 0)];
    if (!target || get(target, "fill") || depth >= 8) return at;
    const m = outer(target, named, depth + 1);
    return m && times(at, m);
  }
  const spaces = node.kids.filter((kid) => !get(kid, "fill")).map((kid) => outer(kid, named, depth));
  if (spaces.some((m) => !m || !same(m, spaces[0]!))) return null;
  return spaces[0] ?? IDENTITY;
}

function outer(node: Tag, named: Named, depth: number): Matrix | null {
  const m = inner(node, named, depth);
  return m && get(node, "transform") ? times(transform(get(node, "transform")!), m) : m;
}

/* MARKUP */

export const markup = (art: string) =>
  `<svg class="appicon" viewBox="0 0 ${SIDE} ${SIDE}" aria-hidden="true" focusable="false"><g clip-path="url(#appicon-clip)">${art}<path class="glow" d="${SHAPE}"/><path class="rim" d="${SHAPE}"/><path class="shade" d="${SHAPE}"/></g></svg>`;

const tone = (color: string) => (color.includes("var(") ? ` style="stop-color:${color}"` : ` stop-color="${color}"`);

const stops = (list: Stop[]) =>
  list
    .map(
      ([at, color, alpha = 1]) =>
        `<stop offset="${at}"${tone(color)}${alpha === 1 ? "" : ` stop-opacity="${round(alpha)}"`}/>`,
    )
    .join("");

const down = (id: string, list: Stop[]) =>
  `<linearGradient id="${id}" x1="0" y1="0" x2="0" y2="1">${stops(list)}</linearGradient>`;

const mirror = (list: Ramp): Ramp => [
  ...list,
  ...list
    .filter(([at]) => at < 0.5)
    .reverse()
    .map(([at, y]): [number, number] => [round(1 - at), y]),
];

const light = (id: string, list: Ramp, over: number, color = "#fff") =>
  down(
    id,
    list.map(([at, y]): Stop => [at, color, y / over]),
  );

const RIM: Ramp = [
  [0, 26],
  [0.03, 40],
  [0.07, 27],
  [0.14, 19],
  [0.25, 5],
  [0.33, 0],
  [0.72, 0],
  [0.9, 16],
  [0.93, 21],
  [0.965, 31],
  [0.98, 31],
  [1, 24],
];

const GLOW: Ramp = [
  [0, 130],
  [0.004, 70],
  [0.008, 30],
  [0.012, 18],
  [0.025, 6],
  [0.1, 7],
  [0.14, 0],
  [0.86, 0],
  [0.9, 4],
  [0.965, 4],
  [0.975, 5],
  [0.988, 12],
  [0.992, 22],
  [0.996, 49],
  [1, 81],
];

const RIM_CLEAR = mirror([
  [0, 28],
  [0.035, 39],
  [0.07, 32],
  [0.19, 19],
  [0.33, 5],
  [0.42, 0],
]);

const GLOW_CLEAR = mirror([
  [0, 46],
  [0.004, 40],
  [0.008, 30],
  [0.012, 23],
  [0.025, 13],
  [0.035, 8],
  [0.07, 1.5],
  [0.1, 0],
]);

const TINT_GLOW = "oklch(from var(--accent) calc(l * 1.49 - .245) calc(c * .85) h)";

const LIFT = `<filter id="appicon-lift" filterUnits="userSpaceOnUse" x="-64" y="-64" width="${SIDE + 128}" height="${SIDE + 128}" color-interpolation-filters="sRGB"><feGaussianBlur in="SourceAlpha" stdDeviation="22"/><feOffset dy="34" result="b"/><feComposite in="b" in2="SourceAlpha" operator="out"/></filter>`;

const mono = (id: string, amplitude: number, flood: string) =>
  `<filter id="${id}" filterUnits="userSpaceOnUse" x="0" y="0" width="${SIDE}" height="${SIDE}" color-interpolation-filters="sRGB"><feColorMatrix in="SourceGraphic" type="matrix" values="0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 ${[0.2126, 0.7152, 0.0722].map((k) => round(k * amplitude)).join(" ")} 0 0" result="l"/><feComposite in="l" in2="SourceAlpha" operator="in" result="a"/><feFlood ${flood}/><feComposite in2="a" operator="in"/></filter>`;

export const SPRITE = [
  `<clipPath id="appicon-clip"><path d="${SHAPE}"/></clipPath>`,
  `<clipPath id="appicon-shape" clipPathUnits="objectBoundingBox"><path d="${BOX}"/></clipPath>`,
  `<mask id="appicon-fade" maskUnits="userSpaceOnUse" x="0" y="0" width="${SIDE}" height="${SIDE}" maskContentUnits="objectBoundingBox"><rect x="-.1" y="0" width="1.2" height="1.25" fill="url(#appicon-fall)"/></mask>`,
  down("appicon-fall", [
    [0, "#fff"],
    [0.2, "#fff", 0.96],
    [0.4, "#fff", 0.85],
    [0.6, "#fff", 0.68],
    [0.8, "#fff", 0.5],
    [1, "#fff", 0.5],
  ]),
  light("appicon-rim", RIM, 255),
  light("appicon-glow", GLOW, 255),
  light(
    "appicon-shade",
    [
      [0.25, 0],
      [0.5, 11],
      [0.75, 0],
    ],
    24,
    "#000",
  ),
  light("appicon-rim-clear", RIM_CLEAR, 127),
  light("appicon-glow-clear", GLOW_CLEAR, 127),
  light(
    "appicon-shade-clear",
    [
      [0.38, 0],
      [0.5, 4.3],
      [0.62, 0],
    ],
    24,
    "#000",
  ),
  light("appicon-rim-tint", RIM_CLEAR, 127, TINT_GLOW),
  light("appicon-glow-tint", GLOW_CLEAR, 127, TINT_GLOW),
  down("appicon-dark", [
    [0, "#212121"],
    [1, "#0d0d0d"],
  ]),
  down("appicon-tint-dark", [
    [0, "color-mix(in srgb, var(--accent) 15%, #19191a)"],
    [1, "#19191a"],
  ]),
  down("appicon-tint-light", [
    [0, "color-mix(in oklab, var(--accent) 63%, #666)"],
    [1, "color-mix(in oklab, var(--accent) 63%, #666)"],
  ]),
  down("appicon-clear-light", [
    [0, "#fff", 0.14],
    [1, "#fff", 0.04],
  ]),
  down("appicon-clear-dark", [
    [0, "#000", 0.5],
    [1, "#000", 0.66],
  ]),
  LIFT,
  mono("appicon-white", 1, 'flood-color="#fff"'),
  mono("appicon-white-tint", 0.96, 'flood-color="#fff"'),
  mono("appicon-white-soft", 0.94, 'flood-color="#fff"'),
  mono("appicon-accent", 1, 'style="flood-color:var(--accent)"'),
].join("");
