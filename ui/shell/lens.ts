export type Spot = { x: number; y: number };

export type Item = { c: number; dx: number; w: number };

export type Lens = { m: number[]; dx: number[]; l: number; r: number };

export type Shape = { big: number; reach: number };

const snap = (px: number) => Math.round(px * 64) / 64;

export function lens(items: Item[], x: number, { big, reach }: Shape, amount = 1): Lens {
  const rest = items.map(({ c, dx, w }) => ({ a: c - dx - w / 2, w }));
  const m = rest.map(({ a, w }) => {
    const d = w ? Math.abs(x - a - w / 2) / w : reach;
    return d >= reach ? 1 : 1 + (amount * (big - 1) * (1 + Math.cos((Math.PI * d) / reach))) / 2;
  });
  const grown = (u: number) => rest.reduce((sum, { a, w }, i) => sum + (m[i] - 1) * Math.min(Math.max(u - a, 0), w), 0);
  const l = grown(x);
  return { m, dx: rest.map(({ a, w }) => snap(grown(a + w / 2) - l)), l: snap(l), r: snap(grown(Infinity) - l) };
}
