export const BIG = 1.6;
const REACH = 2.5;

const snap = (px) => Math.round(px * 64) / 64;

export function lens(items, x, amount = 1) {
  const rest = items.map(({ c, dx, w }) => ({ a: c - dx - w / 2, w }));
  const m = rest.map(({ a, w }) => {
    const d = w ? Math.abs(x - a - w / 2) / w : REACH;
    return d >= REACH ? 1 : 1 + (amount * (BIG - 1) * (1 + Math.cos((Math.PI * d) / REACH))) / 2;
  });
  const grown = (u) => rest.reduce((sum, { a, w }, i) => sum + (m[i] - 1) * Math.min(Math.max(u - a, 0), w), 0);
  const l = grown(x);
  return { m, dx: rest.map(({ a, w }) => snap(grown(a + w / 2) - l)), l: snap(l), r: snap(grown(Infinity) - l) };
}
