import type { Color, Pen } from "@mrlyprod/draw";

export type Inks = Record<string, Color | undefined>;

export const rgb = (hex: string): Color => [
  Number.parseInt(hex.slice(1, 3), 16),
  Number.parseInt(hex.slice(3, 5), 16),
  Number.parseInt(hex.slice(5, 7), 16),
  255,
];

export function bits(pen: Pen, rows: readonly string[], inks: Inks) {
  rows.forEach((row, y) => {
    let x = 0;
    while (x < row.length) {
      const mark = row[x]!;
      let w = 1;
      while (row[x + w] === mark) w++;
      const color = inks[mark];
      if (color) pen.rect(x, y, w, 1, color);
      x += w;
    }
  });
}
