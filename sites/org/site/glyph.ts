export type Color = readonly number[];

export type Pen = {
  width: number;
  height: number;
  round_rect(x: number, y: number, w: number, h: number, r: number, c: Color): void;
};

export type Ink = Readonly<Record<string, Color>>;

export const HUES = [
  "red",
  "orange",
  "yellow",
  "green",
  "mint",
  "teal",
  "cyan",
  "blue",
  "indigo",
  "purple",
  "pink",
  "brown",
  "gray",
] as const;

export type Hue = (typeof HUES)[number];

export const hue = (route: string): Hue => {
  let sum = 0;
  for (const c of route) sum = (sum * 31 + c.charCodeAt(0)) >>> 0;
  return HUES[sum % HUES.length]!;
};

const fnv = (text: string) => {
  let h = 0x811c9dc5;
  for (const c of text) h = Math.imul(h ^ c.charCodeAt(0), 0x01000193) >>> 0;
  return h;
};

export function cells(route: string): boolean[][] {
  const bits = fnv(route);
  const half = Array.from({ length: 5 }, (_, row) =>
    Array.from({ length: 3 }, (_, col) => (bits >>> (row * 3 + col)) & 1),
  );
  const count = half.flat().reduce((sum, on, i) => sum + on * (i % 3 === 2 ? 1 : 2), 0);
  const flip = count < 9;
  return half.map((row) => [row[0]!, row[1]!, row[2]!, row[1]!, row[0]!].map((on) => Boolean(on) !== flip));
}

export const glyph =
  (route: string) =>
  (pen: Pen, ink: Ink): void => {
    const size = Math.min(pen.width, pen.height) * 0.5;
    const unit = size / 5;
    const x0 = (pen.width - size) / 2;
    const y0 = (pen.height - size) / 2;
    const color = ink[hue(route)] ?? [0, 0, 0, 255];
    cells(route).forEach((row, r) => {
      row.forEach((on, c) => {
        if (on) pen.round_rect(x0 + c * unit, y0 + r * unit, unit * 0.9, unit * 0.9, unit * 0.18, color);
      });
    });
  };
