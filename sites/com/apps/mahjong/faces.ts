import { blend, veil } from "@mrlyprod/ui/kit/color.ts";
import type { Mrly } from "@mrlyprod/ui/kit/designs/engine.ts";
import { HUES } from "@mrlyprod/ui/kit/theme/hues.ts";
import type { Dragon, Face, Grown, Plate, Rose } from "./engine.ts";

export type Pen = Pick<
  CanvasRenderingContext2D,
  | "beginPath"
  | "moveTo"
  | "lineTo"
  | "closePath"
  | "rect"
  | "arc"
  | "fill"
  | "stroke"
  | "fillRect"
  | "fillStyle"
  | "strokeStyle"
  | "lineWidth"
  | "lineCap"
  | "lineJoin"
  | "globalAlpha"
>;

export type Tones = { pale: string; dark: string; hue: (name: string) => string };

export type Art = (pen: Pen, side: number, tones: Tones, hue: string) => void;

const VOID = 0.07;
const PLATE = 160;
const NODE = 0.08;
const FLOOD = 0.14;
const STRAND = 0.44;
const PENCIL = 0.9;
const SAMPLES = 360;
const REACH = 0.48;
const LINE = 0.011;
const FIT = 0.92;
const TOP = 0.55;
const SIDE = 0.42;
const HALF = Math.PI / 2;
const RATIO = Math.sqrt(3) / 2;
const SPOT = 1.5;
const GHOST = 0.09;
const STRIDES = [0, 4, 7, 2, 9, 5];

const held = new Map<string, Art>();

const paletteOf = (tones: Tones, hue: string) => {
  const at = Math.max(0, HUES.indexOf(hue));
  return STRIDES.map((by) => tones.hue(HUES[(at + by) % HUES.length]));
};

const grid = (pen: Pen, types: ArrayLike<number>, n: number, side: number, on: (type: number) => boolean) => {
  const stop = (i: number) => Math.round((i * side) / n);
  pen.beginPath();
  for (let r = 0; r < n; r++) {
    const row = r * n;
    for (let c = 0; c < n; c++) {
      if (!on(types[row + c])) continue;
      const start = c;
      while (c + 1 < n && on(types[row + c + 1])) c++;
      pen.rect(stop(start), stop(r), stop(c + 1) - stop(start), stop(r + 1) - stop(r));
    }
  }
  pen.fill();
};

function carpet(mrly: Mrly, { code, base, number, level }: Grown): Art {
  const cell = mrly.two.create(code, number, level, 0, base);
  const n = cell.shape[0];
  return (pen, side, tones, hue) => {
    const main = tones.hue(hue);
    pen.fillStyle = veil(main, VOID);
    grid(pen, cell.types, n, side, (type) => !type);
    pen.fillStyle = main;
    grid(pen, cell.types, n, side, (type) => Boolean(type));
  };
}

function plate(mrly: Mrly, { n, m, sign, fine = PLATE }: Plate): Art {
  const field = mrly.chladni.plate(n, m, sign, fine);
  const kinds = Uint8Array.from(field, (v) => (Math.abs(v) < NODE ? 1 : v > 0 ? 2 : 0));
  return (pen, side, tones, hue) => {
    const [main, other] = paletteOf(tones, hue);
    pen.fillStyle = veil(other, FLOOD);
    grid(pen, kinds, fine, side, (type) => type === 0);
    pen.fillStyle = veil(main, FLOOD);
    grid(pen, kinds, fine, side, (type) => type === 2);
    pen.fillStyle = main;
    grid(pen, kinds, fine, side, (type) => type === 1);
  };
}

function loop(mrly: Mrly, { code, base, level }: Grown): Art {
  const drawn = mrly.arcs.draw(code, base, level);
  const strands = mrly.arcs.curves(code, base, level).map((one) => Array.from(one.arcs));
  const across = drawn.side;
  return (pen, side, tones, hue) => {
    const inks = paletteOf(tones, hue);
    const unit = side / across;
    pen.lineWidth = unit * STRAND;
    pen.lineCap = "round";
    for (const [k, list] of strands.entries()) {
      pen.strokeStyle = inks[k % inks.length];
      pen.beginPath();
      for (const a of list) {
        const i = a >> 1;
        const x = i % across;
        const y = (i - x) / across;
        const filled = drawn.cells[i] & 1;
        const which = a & 1;
        const cx = which ? (filled ? x + 1 : x) : filled ? x : x + 1;
        const cy = which ? y + 1 : y;
        const right = cx !== x;
        const down = cy !== y;
        const from = down ? -HALF : HALF;
        const to = right !== down ? from + HALF : from - HALF;
        pen.moveTo((cx + Math.cos(from) / 2) * unit, (cy + Math.sin(from) / 2) * unit);
        pen.arc(cx * unit, cy * unit, unit / 2, from, to, to < from);
      }
      pen.stroke();
    }
  };
}

function rose(mrly: Mrly, { ring, wheel, lace, code, number }: Rose): Art {
  const track = mrly.roulette.track("in", ring, wheel, lace, 1);
  const design = mrly.two.create(code, number, 1, 0, 3);
  const pencils = mrly.roulette.pencils(design.types, design.shape[1], design.shape[0], "fill", PENCIL, 0, 1);
  const trace = mrly.roulette.trace(track, pencils, SAMPLES);
  const each = trace.length / 2 / pencils.length;
  let far = 0;
  for (let i = 0; i < trace.length; i += 2) far = Math.max(far, Math.hypot(trace[i], trace[i + 1]));
  return (pen, side, tones, hue) => {
    const inks = paletteOf(tones, hue);
    const k = (side * REACH) / far;
    pen.lineWidth = Math.max(0.5, side * LINE);
    pen.lineJoin = "round";
    for (let one = 0; one < pencils.length; one++) {
      pen.strokeStyle = inks[one % 3];
      pen.beginPath();
      for (let i = 0; i < each; i++) {
        const at = (one * each + i) * 2;
        const x = side / 2 + trace[at] * k;
        const y = side / 2 + trace[at + 1] * k;
        if (i) pen.lineTo(x, y);
        else pen.moveTo(x, y);
      }
      pen.stroke();
    }
  };
}

function mesh(
  mrly: Mrly,
  cell: Parameters<Mrly["hex"]["triangles"]>[0] extends infer _ ? ReturnType<Mrly["hex"]["iso_design"]> : never,
) {
  const hex = mrly.hex;
  const kinds = {
    VOID: hex.VOID(),
    FILL: hex.FILL(),
    GRID: hex.GRID(),
    UP: hex.UP(),
    LEFT: hex.LEFT(),
    RIGHT: hex.RIGHT(),
  };
  const custom = Object.fromEntries(
    Object.entries(kinds).map(([name, type]) => [String(type), [[type, 0, 0, name === "GRID" ? 0 : 255]]]),
  );
  const across = cell.orientation === "Horizontal";
  const out = new Map<number, number[]>();
  let [x0, y0, x1, y1] = [Infinity, Infinity, -Infinity, -Infinity];
  for (const [corners, rgba] of hex.triangles(hex.paint(cell, custom as never))) {
    const list = out.get(rgba[0]) ?? [];
    for (const [x, y] of corners) {
      const u = across ? Number(x) : Number(x) * RATIO;
      const v = across ? Number(y) * RATIO : Number(y);
      list.push(u, v);
      x0 = Math.min(x0, u);
      x1 = Math.max(x1, u);
      y0 = Math.min(y0, v);
      y1 = Math.max(y1, v);
    }
    out.set(rgba[0], list);
  }
  return { kinds, groups: out, box: [x0, y0, x1, y1] };
}

function hexed(
  made: ReturnType<typeof mesh>,
  tone: (kind: number, main: string, tones: Tones, other: string) => string,
): Art {
  const [x0, y0, x1, y1] = made.box;
  return (pen, side, tones, hue) => {
    const [main, other] = paletteOf(tones, hue);
    const k = (side * FIT) / Math.max(x1 - x0, y1 - y0);
    const ox = (side - (x1 - x0) * k) / 2 - x0 * k;
    const oy = (side - (y1 - y0) * k) / 2 - y0 * k;
    for (const [kind, points] of made.groups) {
      const color = tone(kind, main, tones, other);
      if (!color) continue;
      pen.fillStyle = color;
      pen.strokeStyle = color;
      pen.lineWidth = 0.4;
      pen.beginPath();
      for (let i = 0; i < points.length; i += 6) {
        pen.moveTo(ox + points[i] * k, oy + points[i + 1] * k);
        pen.lineTo(ox + points[i + 2] * k, oy + points[i + 3] * k);
        pen.lineTo(ox + points[i + 4] * k, oy + points[i + 5] * k);
        pen.closePath();
      }
      pen.fill();
      pen.stroke();
    }
  };
}

function cube(mrly: Mrly, { code, number, level, base }: Grown): Art {
  const made = mesh(mrly, mrly.hex.iso_design(code, number, level, base));
  const { UP, LEFT, RIGHT, FILL } = made.kinds;
  return hexed(made, (kind, main, tones) => {
    if (kind === UP) return blend(main, tones.pale, TOP);
    if (kind === LEFT || kind === FILL) return main;
    return kind === RIGHT ? blend(main, tones.dark, SIDE) : "";
  });
}

function star(mrly: Mrly, { code, number, level, base }: Grown): Art {
  const made = mesh(mrly, mrly.hex.cut_design(code, number, level, base));
  const { FILL, VOID: EMPTY } = made.kinds;
  return hexed(made, (kind, main, _tones, other) => (kind === FILL ? main : kind === EMPTY ? veil(other, GHOST) : ""));
}

function dragon(mrly: Mrly, { name, level }: Dragon): Art {
  const radix = mrly.radix[name as "twindragon"]();
  const raw = radix.plane(level);
  const size = radix.size();
  radix.free();
  const points = raw.map(([x, y]) => [Number(x), Number(y)]);
  const xs = points.map(([x]) => x);
  const ys = points.map(([, y]) => y);
  const [x0, x1, y0, y1] = [Math.min(...xs), Math.max(...xs), Math.min(...ys), Math.max(...ys)];
  const span = Math.max(x1 - x0, y1 - y0) || 1;
  const stride = size ** Math.max(0, level - 2);
  const dot = (SPOT * span) / Math.sqrt(points.length);
  return (pen, side, tones, hue) => {
    const inks = paletteOf(tones, hue);
    const k = (side * FIT) / (span + dot);
    const ox = (side - (x1 - x0) * k) / 2 - x0 * k;
    const oy = (side - (y1 - y0) * k) / 2 - y0 * k;
    const width = Math.max(1, dot * k);
    for (let digit = 0; digit < size; digit++) {
      pen.fillStyle = inks[digit % inks.length];
      pen.beginPath();
      for (let i = 0; i < points.length; i++) {
        if (Math.floor(i / stride) % size !== digit) continue;
        pen.rect(ox + points[i][0] * k - width / 2, oy + points[i][1] * k - width / 2, width, width);
      }
      pen.fill();
    }
  };
}

const MAKERS = {
  carpet,
  plate,
  loop,
  rose,
  cube,
  star,
  dragon,
} as Record<Face["suit"], (mrly: Mrly, face: never) => Art>;

export function artOf(mrly: Mrly, face: Face): Art {
  const { hue: _hue, ...rest } = face;
  const key = JSON.stringify(rest);
  if (!held.has(key)) held.set(key, MAKERS[face.suit](mrly, rest as never));
  return held.get(key)!;
}

export const LABELS: Record<Face["suit"], string> = {
  carpet: "carpets",
  plate: "plates",
  loop: "loops",
  rose: "roses",
  cube: "cubes",
  star: "stars",
  dragon: "dragons",
};
