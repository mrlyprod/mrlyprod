import { escape } from "../kit/ssg/text.ts";

/* SHAPES */

export type Shape = { kind: string; cols: number; rows: number; accent: number; wide: [number, number]; tall: [number, number] };

export const WHOLE: Shape = { kind: "hero", cols: 16, rows: 9, accent: 8, wide: [6, 2], tall: [7, 3] };

export const HALF: Shape = { kind: "tile", cols: 8, rows: 9, accent: 4, wide: [5, 1], tall: [5, 1] };

export const HUE: Record<string, string> = { red: "r", orange: "o", yellow: "y", green: "g", mint: "m", teal: "t", cyan: "c", blue: "b", indigo: "i", purple: "p", pink: "k", brown: "n", accent: "a" };

const ART = { spark: 0.04, jitter: 0.9 };

/* TILINGS */

type Poly = [number, number][];

export type Cell = { id: string; poly: Poly; shape: number; x: number; y: number; cx: number; cy: number; whole: boolean };

type Tiling = { shapes: Poly[]; lay: (cols: number, rows: number) => Cell[] };

const ROOT3 = Math.sqrt(3);

const HEXW = 16 / 12;

const HEXS = HEXW / ROOT3;

const TRIA = 16 / 8;

const TRIH = (TRIA * ROOT3) / 2;

const EPS = 1e-6;

const shift = (poly: Poly, x: number, y: number): Poly => poly.map((p) => [p[0] + x, p[1] + y]);

function cell(id: string, shape: Poly, n: number, x: number, y: number, cols: number, rows: number): Cell | null {
  const poly = shift(shape, x, y);
  const xs = poly.map((p) => p[0]);
  const ys = poly.map((p) => p[1]);
  if (Math.max(...xs) <= EPS || Math.min(...xs) >= cols - EPS || Math.max(...ys) <= EPS || Math.min(...ys) >= rows - EPS) return null;
  const whole = Math.min(...xs) >= -EPS && Math.max(...xs) <= cols + EPS && Math.min(...ys) >= -EPS && Math.max(...ys) <= rows + EPS;
  const cx = xs.reduce((a, b) => a + b, 0) / xs.length;
  const cy = ys.reduce((a, b) => a + b, 0) / ys.length;
  return { id, poly, shape: n, x, y, cx, cy, whole };
}

const SQUARE: Poly = [[0, 0], [1, 0], [1, 1], [0, 1]];

const HEXAGON: Poly = [0, 1, 2, 3, 4, 5].map((i) => [HEXS * Math.cos(Math.PI / 6 + (i * Math.PI) / 3), HEXS * Math.sin(Math.PI / 6 + (i * Math.PI) / 3)]);

const UP: Poly = [[TRIA / 2, 0], [TRIA, TRIH], [0, TRIH]];

const DOWN: Poly = [[TRIA / 2, 0], [TRIA * 1.5, 0], [TRIA, TRIH]];

function square(cols: number, rows: number): Cell[] {
  const out: Cell[] = [];
  for (let y = 0; y < rows; y++) for (let x = 0; x < cols; x++) out.push(cell(`${x},${y}`, SQUARE, 0, x, y, cols, rows)!);
  return out;
}

function hex(cols: number, rows: number): Cell[] {
  const out: Cell[] = [];
  for (let k = -1; k * 1.5 * HEXS < rows + HEXS; k++) {
    for (let m = -1; m * HEXW < cols + HEXW; m++) {
      const one = cell(`${k},${m}`, HEXAGON, 0, m * HEXW + (k & 1 ? HEXW : HEXW / 2), HEXS + k * 1.5 * HEXS, cols, rows);
      if (one) out.push(one);
    }
  }
  return out;
}

function tri(cols: number, rows: number): Cell[] {
  const out: Cell[] = [];
  for (let k = 0; k * TRIH < rows; k++) {
    for (let m = -1; m * TRIA < cols + TRIA; m++) {
      const x0 = m * TRIA + (k & 1 ? TRIA / 2 : 0);
      const up = cell(`u${k},${m}`, UP, 0, x0, k * TRIH, cols, rows);
      const down = cell(`d${k},${m}`, DOWN, 1, x0, k * TRIH, cols, rows);
      if (up) out.push(up);
      if (down) out.push(down);
    }
  }
  return out;
}

export const TILINGS: Record<string, Tiling> = {
  square: { shapes: [SQUARE], lay: square },
  hex: { shapes: [HEXAGON], lay: hex },
  tri: { shapes: [UP, DOWN], lay: tri },
};

/* PLATE */

export type Box = [number, number, number, number];

const R = 8;

const vertex = (p: [number, number]) => `${Math.round(p[0] * 1e4)},${Math.round(p[1] * 1e4)}`;

export function inside(poly: Poly, x: number, y: number): boolean {
  let on = false;
  for (let i = 0, j = poly.length - 1; i < poly.length; j = i++) {
    const [xi, yi] = poly[i]!;
    const [xj, yj] = poly[j]!;
    if (yi > y !== yj > y && x < ((xj - xi) * (y - yi)) / (yj - yi) + xi) on = !on;
  }
  return on;
}

export function fit(on: (x: number, y: number) => boolean, w: number, h: number, need: [number, number]): Box | null {
  const tall = new Array<number>(w).fill(0);
  let best: Box | null = null;
  let area = 0;
  for (let y = 0; y < h; y++) {
    for (let x = 0; x < w; x++) tall[x] = on(x, y) ? tall[x]! + 1 : 0;
    const stack: number[] = [];
    for (let x = 0; x <= w; x++) {
      const here = x < w ? tall[x]! : 0;
      while (stack.length && tall[stack[stack.length - 1]!]! >= here) {
        const height = tall[stack.pop()!]!;
        const left = stack.length ? stack[stack.length - 1]! + 1 : 0;
        const width = x - left;
        if (height >= need[1] && width >= need[0] && width * height > area) {
          area = width * height;
          best = [left, y - height + 1, width, height];
        }
      }
      stack.push(x);
    }
  }
  return best;
}

type Plate = { cells: Cell[]; near: Cell[]; box: Box };

function plate(cells: Cell[], cols: number, rows: number, need: [number, number], roll: () => number): Plate {
  const corners = new Map<string, Cell[]>();
  for (const one of cells) for (const p of one.poly) {
    const list = corners.get(vertex(p)) ?? [];
    list.push(one);
    corners.set(vertex(p), list);
  }
  let [w, h] = need;
  const x0 = roll() * (cols - w);
  const y0 = roll() * (rows - h);
  for (let grow = 0; w <= cols + 1 && h <= rows + 1; grow++) {
    const x = Math.min(x0, cols - w);
    const y = Math.min(y0, rows - h);
    const picked = cells.filter((one) => one.whole && one.cx >= x && one.cx < x + w && one.cy >= y && one.cy < y + h);
    const on = (gx: number, gy: number) => picked.some((one) => inside(one.poly, (gx + 0.5) / R, (gy + 0.5) / R));
    const box = fit(on, cols * R, rows * R, [Math.ceil(need[0] * R), Math.ceil(need[1] * R)]);
    if (box) {
      const near = new Set<Cell>();
      for (const one of picked) for (const p of one.poly) for (const other of corners.get(vertex(p)) ?? []) if (!picked.includes(other)) near.add(other);
      return { cells: picked, near: [...near], box: box.map((v) => v / R) as Box };
    }
    if (grow % 2 === 0) w += 0.25;
    else h += 0.25;
  }
  throw new Error(`heroes: no plate of ${need[0]} by ${need[1]} fits the ${cols} by ${rows} art`);
}

/* ART */

export const seeded = (name: string) => [...name].reduce((h, c) => Math.imul(h ^ c.charCodeAt(0), 16777619) >>> 0, 2166136261);

export function rng(seed: number) {
  let s = seed >>> 0;
  return () => {
    s = (s + 0x6d2b79f5) >>> 0;
    let t = Math.imul(s ^ (s >>> 15), s | 1);
    t ^= t + Math.imul(t ^ (t >>> 7), t | 61);
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
}

const num = (v: number) => String(Math.round(v * 1000) / 1000);

const path = (poly: Poly) => `M${poly.map((p) => `${num(p[0])} ${num(p[1])}`).join("L")}Z`;

const use = (id: string, one: Cell, rest = "") => `<use href="#${id}${one.shape}" x="${num(one.x)}" y="${num(one.y)}"${rest}/>`;

const uses = (id: string, cells: Cell[]) => cells.map((one) => use(id, one)).join("");

export function art(name: string, hues: string[], seed: number, shape: Shape, avoid = "", id = name.toLowerCase().replace(/[^a-z0-9]/g, "")) {
  if (!Array.isArray(hues) || hues.length === 0) throw new Error(`site: site.json gives the ${shape.kind} ${name} no hues, and the hues are ${Object.keys(HUE).join(", ")}`);
  for (const hue of hues) if (!Object.hasOwn(HUE, hue)) throw new Error(`site: site.json gives the ${shape.kind} ${name} the hue ${hue}, and the hues are ${Object.keys(HUE).join(", ")}`);
  const { cols, rows, accent } = shape;
  const roll = rng(seeded(`${seed}`));
  const names = Object.keys(TILINGS).filter((one) => one !== avoid);
  const tiling = names[Math.floor(roll() * names.length)]!;
  const cells = TILINGS[tiling]!.lay(cols, rows);
  const turn = roll() * 2 * Math.PI;
  const along = (x: number, y: number) => (x - cols / 2) * Math.cos(turn) + (y - rows / 2) * Math.sin(turn);
  const ends = [along(0, 0), along(cols, 0), along(0, rows), along(cols, rows)];
  const [low, high] = [Math.min(...ends), Math.max(...ends)];
  const span = hues.length;
  const half = Math.ceil(cols / 2);
  const blocks = Array.from({ length: Math.ceil(rows / 2) * half }, roll);
  const clamp = (v: number, top: number) => Math.min(Math.max(v, 0), top);
  const paint = cells.map((one) => {
    const block = blocks[clamp(Math.floor(one.cy) >> 1, Math.ceil(rows / 2) - 1) * half + clamp(Math.floor(one.cx) >> 1, half - 1)]!;
    const noise = 0.65 * block + 0.35 * roll() - 0.5;
    const k = Math.min(Math.max(((along(one.cx, one.cy) - low) / (high - low)) * span + noise * ART.jitter, 0), span - 1e-9);
    return HUE[roll() < ART.spark ? "accent" : hues[Math.floor(k)]!]!;
  });
  const lit = HUE.accent!;
  let have = paint.filter((one) => one === lit).length;
  while (have < accent) {
    const pick = Math.floor(roll() * paint.length);
    if (paint[pick] === lit) continue;
    paint[pick] = lit;
    have++;
  }
  const wide = plate(cells, cols, rows, [shape.wide[0], shape.wide[1]], roll);
  const tall = plate(cells, cols, rows, [shape.tall[1], shape.tall[0]], roll);
  const defs = TILINGS[tiling]!.shapes.map((poly, n) => `<path id="${id}${n}" d="${path(poly)}" vector-effect="non-scaling-stroke"/>`).join("");
  const base = cells.map((one, n) => use(id, one, ` class="${paint[n]}" style="--n:${Math.max(0, Math.floor(one.cx) + Math.floor(one.cy))}"`)).join("");
  const over = (one: Plate, mark: string) => `<g class="near${mark}">${uses(id, one.near)}</g><g class="plate${mark}">${uses(id, one.cells)}</g>`;
  const svg = `<svg class="art ${tiling}" viewBox="0 0 ${cols} ${rows}" aria-hidden="true"><defs>${defs}</defs><g class="base">${base}</g>${over(wide, "")}${over(tall, " tall")}</svg>`;
  const [x, y, cx, cy] = wide.box;
  const [qy, qx, qcy, qcx] = tall.box;
  const style = `--x:${num(x)};--y:${num(y)};--cx:${num(cx)};--cy:${num(cy)};--qx:${num(qx)};--qy:${num(qy)};--qcx:${num(qcx)};--qcy:${num(qcy)}`;
  return { art: svg, style, tiling, cells: cells.length, paint };
}

export function doorway(hero: { name: string; href: string; line: string; hues: string[]; seed: number }, avoid = "") {
  const { art: cells, style, tiling } = art(hero.name, hero.hues, hero.seed, WHOLE, avoid);
  return { html: `<a class="hero" href="${escape(hero.href)}" style="${style}">${cells}<div class="plate"><h2>${escape(hero.name)}</h2><p>${escape(hero.line)}</p></div></a>`, tiling };
}

export function tiled(name: string, href: string, hues: string[], avoid = "") {
  const { art: cells, style, tiling } = art(name, hues, seeded(name), HALF, avoid, `t${name.toLowerCase().replace(/[^a-z0-9]/g, "")}`);
  return { html: `<a class="half" href="${escape(href)}" style="${style}">${cells}<div class="plate"><h2>${escape(name)}</h2></div></a>`, tiling };
}

export function row<T>(list: T[], draw: (one: T, avoid: string) => { html: string; tiling: string }): string {
  let avoid = "";
  return list.map((one) => {
    const drawn = draw(one, avoid);
    avoid = drawn.tiling;
    return drawn.html;
  }).join("");
}
