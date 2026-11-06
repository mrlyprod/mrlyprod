import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const KEYS = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
const BAYER = [0, 8, 2, 10, 12, 4, 14, 6, 3, 11, 1, 9, 15, 7, 13, 5];

export type Ink = string | null;
export type Point = [number, number];
export type Colors = Record<string, string>;

export class Canvas {
  w: number;
  h: number;
  colors: Colors;
  cells: Ink[];

  constructor(w: number, h: number, colors: Colors, back: Ink = null) {
    this.w = w;
    this.h = h;
    this.colors = colors;
    this.cells = new Array<Ink>(w * h).fill(back);
  }

  inside(x: number, y: number): boolean {
    return x >= 0 && y >= 0 && x < this.w && y < this.h;
  }

  get(x: number, y: number): Ink {
    return this.inside(x, y) ? this.cells[y * this.w + x]! : null;
  }

  px(x: number, y: number, ink: Ink): this {
    const [u, v] = [Math.round(x), Math.round(y)];
    if (ink !== null && !(ink in this.colors)) throw new Error(`unknown colour ${ink}`);
    if (this.inside(u, v)) this.cells[v * this.w + u] = ink;
    return this;
  }

  rect(x: number, y: number, w: number, h: number, ink: Ink): this {
    for (let v = y; v < y + h; v++) for (let u = x; u < x + w; u++) this.px(u, v, ink);
    return this;
  }

  line(x0: number, y0: number, x1: number, y1: number, ink: Ink): this {
    finite(x0, y0, x1, y1);
    let [x, y] = [Math.round(x0), Math.round(y0)];
    const [tx, ty] = [Math.round(x1), Math.round(y1)];
    const [dx, dy] = [Math.abs(tx - x), -Math.abs(ty - y)];
    const [sx, sy] = [Math.sign(tx - x), Math.sign(ty - y)];
    let err = dx + dy;
    for (;;) {
      this.px(x, y, ink);
      if (x === tx && y === ty) return this;
      const twice = 2 * err;
      if (twice >= dy) {
        err += dy;
        x += sx;
      }
      if (twice <= dx) {
        err += dx;
        y += sy;
      }
    }
  }

  ellipse(cx: number, cy: number, rx: number, ry: number, ink: Ink): this {
    finite(cx, cy, rx, ry);
    for (let y = Math.floor(cy - ry); y <= Math.ceil(cy + ry); y++)
      for (let x = Math.floor(cx - rx); x <= Math.ceil(cx + rx); x++)
        if (((x - cx) / (rx + 0.25)) ** 2 + ((y - cy) / (ry + 0.25)) ** 2 <= 1) this.px(x, y, ink);
    return this;
  }

  poly(points: Point[], ink: Ink): this {
    finite(...points.flat());
    const ys = points.map((point) => point[1]);
    for (let y = Math.floor(Math.min(...ys)); y <= Math.ceil(Math.max(...ys)); y++) {
      const hits: number[] = [];
      points.forEach(([x0, y0], i) => {
        const [x1, y1] = points[(i + 1) % points.length]!;
        if (y0 <= y + 0.5 !== y1 <= y + 0.5) hits.push(x0 + ((y + 0.5 - y0) * (x1 - x0)) / (y1 - y0));
      });
      hits.sort((a, b) => a - b);
      for (let i = 0; i + 1 < hits.length; i += 2)
        for (let x = Math.ceil(hits[i]! - 0.5); x <= Math.floor(hits[i + 1]! - 0.5); x++) this.px(x, y, ink);
    }
    return this;
  }

  sprite(x: number, y: number, rows: string[], key: Record<string, Ink>, flip = false): this {
    rows.forEach((row, v) => {
      [...row].forEach((glyph, u) => {
        if (glyph === " " || glyph === ".") return;
        if (!(glyph in key)) throw new Error(`sprite uses unknown key ${glyph}`);
        this.px(flip ? x + row.length - 1 - u : x + u, y + v, key[glyph]!);
      });
    });
    return this;
  }

  ramp(x: number, y: number, w: number, h: number, stops: string[], across = false, soft = 0.3): this {
    for (let v = 0; v < h; v++)
      for (let u = 0; u < w; u++)
        this.px(x + u, y + v, pick(stops, across ? (u + 0.5) / w : (v + 0.5) / h, x + u, y + v, soft));
    return this;
  }

  each(fn: (x: number, y: number, ink: Ink) => Ink | undefined): this {
    for (let y = 0; y < this.h; y++)
      for (let x = 0; x < this.w; x++) {
        const ink = fn(x, y, this.cells[y * this.w + x]!);
        if (ink !== undefined) this.px(x, y, ink);
      }
    return this;
  }

  paste(other: Canvas, x = 0, y = 0): this {
    other.each((u, v, ink) => {
      if (ink !== null) this.px(x + u, y + v, ink);
      return undefined;
    });
    return this;
  }

  copy(): Canvas {
    const twin = new Canvas(this.w, this.h, this.colors);
    twin.cells = this.cells.slice();
    return twin;
  }
}

function finite(...values: number[]): void {
  if (!values.every(Number.isFinite)) throw new Error(`not a finite number in ${values.join(", ")}`);
}

export function pick(stops: string[], t: number, x: number, y: number, soft = 0.3): string {
  if (stops.length === 0) throw new Error("a ramp needs at least one colour");
  const at = Math.max(0, Math.min(0.999999, t)) * (stops.length - 1);
  const low = Math.floor(at);
  const lean = soft > 0 ? (at - low - 0.5) / soft + 0.5 : at - low;
  const edge = soft > 0 ? (BAYER[(y & 3) * 4 + (x & 3)]! + 0.5) / 16 : 0.5;
  return stops[lean > edge ? low + 1 : low]!;
}

export function rng(seed: number): () => number {
  let state = seed >>> 0 || 1;
  return () => {
    state ^= state << 13;
    state ^= state >>> 17;
    state ^= state << 5;
    state >>>= 0;
    return state / 4294967296;
  };
}

function lattice(x: number, y: number, seed: number): number {
  let hash = (Math.imul(x, 374761393) + Math.imul(y, 668265263) + Math.imul(seed, 2147483647)) | 0;
  hash = Math.imul(hash ^ (hash >>> 13), 1274126177);
  return ((hash ^ (hash >>> 16)) >>> 0) / 4294967296;
}

export function noise(x: number, y: number, seed = 1): number {
  const [ix, iy] = [Math.floor(x), Math.floor(y)];
  const [fx, fy] = [x - ix, y - iy];
  const [u, v] = [fx * fx * (3 - 2 * fx), fy * fy * (3 - 2 * fy)];
  const top = lattice(ix, iy, seed) * (1 - u) + lattice(ix + 1, iy, seed) * u;
  const low = lattice(ix, iy + 1, seed) * (1 - u) + lattice(ix + 1, iy + 1, seed) * u;
  return top * (1 - v) + low * v;
}

export function fbm(x: number, y: number, seed = 1, octaves = 4): number {
  let [sum, amp, total] = [0, 1, 0];
  for (let i = 0; i < octaves; i++) {
    sum += amp * noise(x * 2 ** i, y * 2 ** i, seed + i);
    total += amp;
    amp /= 2;
  }
  return sum / total;
}

export function save(dir: string, size: string, art: Canvas | Canvas[]): void {
  const frames = Array.isArray(art) ? art : [art];
  const first = frames[0];
  if (!first) throw new Error("nothing to save");
  const used = new Set(frames.flatMap((frame) => frame.cells).filter((ink) => ink !== null));
  const names = Object.keys(first.colors).filter((name) => used.has(name));
  const stray = [...used].filter((name) => !(name in first.colors));
  if (stray.length > 0) throw new Error(`colours missing from the first frame: ${stray.join(", ")}`);
  if (names.length === 0) throw new Error("nothing painted");
  if (names.length > KEYS.length) throw new Error(`${names.length} colours, the limit is ${KEYS.length}`);
  const keys = new Map(names.map((name, i) => [name, KEYS[i]!]));
  const palette = names.map((name) => {
    const hex = first.colors[name]!.replace("#", "").toLowerCase();
    if (!/^[0-9a-f]{6}$/.test(hex)) throw new Error(`colour ${name} is not six hex digits`);
    const twin = names.find((other) => other !== name && first.colors[other]!.replace("#", "").toLowerCase() === hex);
    if (twin) throw new Error(`colours ${name} and ${twin} are both ${hex}`);
    return `${keys.get(name)} ${hex}\n`;
  });
  const blocks = frames.map((frame) => {
    if (frame.w !== first.w || frame.h !== first.h) throw new Error("frames differ in size");
    let block = "\n";
    for (let y = 0; y < frame.h; y++) {
      for (let x = 0; x < frame.w; x++) block += keys.get(frame.cells[y * frame.w + x] ?? "") ?? ".";
      block += "\n";
    }
    return block;
  });
  const file = join(dir, `${size}.txt`);
  writeFileSync(file, palette.join("") + blocks.join(""));
  console.log(`${file} ${first.w}x${first.h} ${frames.length} frames ${names.length} colours`);
}

export function load(dir: string, size: string): Canvas[] {
  const [head, ...blocks] = readFileSync(join(dir, `${size}.txt`), "utf8")
    .trim()
    .split(/\n\n+/);
  const colors: Colors = {};
  for (const line of (head ?? "").split("\n")) colors[line[0]!] = line.slice(2);
  return blocks.map((block) => {
    const rows = block.split("\n");
    const canvas = new Canvas(rows[0]!.length, rows.length, colors);
    rows.forEach((row, y) => {
      [...row].forEach((glyph, x) => {
        canvas.px(x, y, glyph === "." ? null : glyph);
      });
    });
    return canvas;
  });
}
