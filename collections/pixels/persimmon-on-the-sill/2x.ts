import { Canvas, save } from "../paint.ts";

const art = new Canvas(64, 64, {
  sky: "#c5d3d6",
  haze: "#dadcd0",
  cream: "#f1e3c2",
  grain: "#e6d6b4",
  peach: "#f3d2a4",
  veil: "#cfc3b6",
  shade: "#8f95b0",
  blush: "#a58694",
  pewter: "#626c88",
  slate: "#56607e",
  ink: "#3f4762",
  glow: "#ffb347",
  orange: "#f07a1a",
  amber: "#d95f18",
  rust: "#b4421f",
  wine: "#742a2c",
  moss: "#55592f",
  sage: "#a69c5a",
});

const key = {
  e: "cream",
  r: "blush",
  d: "slate",
  i: "glow",
  j: "orange",
  o: "amber",
  k: "rust",
  l: "wine",
  m: "moss",
  n: "sage",
};

const clamp = (value: number, low = 0, high = 1) => Math.max(low, Math.min(high, value));

function runs(ink: string, list: [number, number, number][]): void {
  for (const [x, y, len] of list) art.rect(x, y, len, 1, ink);
}

art.rect(0, 0, 64, 7, "sky");
art.rect(0, 7, 64, 3, "haze");
art.rect(0, 10, 64, 2, "cream");
art.rect(3, 6, 20, 1, "haze");
art.rect(34, 6, 9, 1, "haze");
art.rect(0, 9, 27, 1, "cream");

art.rect(48, 0, 1, 12, "shade");
art.rect(49, 0, 2, 12, "slate");
art.rect(51, 0, 1, 12, "ink");

art.rect(0, 12, 64, 2, "shade");
art.rect(0, 14, 64, 11, "slate");
art.rect(0, 23, 64, 2, "pewter");
art.rect(0, 25, 64, 1, "ink");

runs("slate", [
  [52, 12, 5],
  [52, 13, 7],
  [16, 13, 6],
  [42, 23, 4],
  [58, 23, 3],
]);
runs("pewter", [
  [2, 16, 11],
  [5, 17, 15],
  [38, 17, 12],
  [41, 18, 15],
  [57, 15, 7],
  [0, 19, 4],
  [44, 20, 10],
  [47, 21, 14],
  [0, 22, 6],
  [36, 22, 5],
  [50, 22, 8],
]);
runs("ink", [
  [21, 15, 10],
  [36, 19, 6],
  [1, 21, 4],
]);

art.rect(0, 26, 64, 31, "cream");
art.rect(0, 57, 64, 1, "grain");
art.rect(0, 58, 64, 2, "veil");
art.rect(0, 60, 64, 3, "shade");
art.rect(0, 63, 64, 1, "slate");

runs("grain", [
  [0, 33, 4],
  [36, 32, 10],
  [39, 33, 12],
  [50, 35, 7],
  [1, 40, 8],
  [3, 41, 9],
  [12, 42, 12],
  [30, 45, 13],
  [34, 46, 14],
  [50, 48, 10],
  [0, 49, 10],
  [0, 50, 6],
  [10, 52, 13],
  [14, 53, 16],
  [2, 55, 14],
  [28, 55, 12],
  [0, 56, 16],
  [22, 56, 26],
  [54, 56, 10],
]);
runs("cream", [
  [17, 57, 6],
  [35, 57, 4],
]);
runs("shade", [
  [6, 59, 4],
  [24, 59, 5],
  [44, 59, 3],
]);
runs("slate", [
  [3, 61, 9],
  [26, 62, 12],
  [45, 61, 10],
]);

function level(sd: number, soft: number): number {
  const t = sd / soft + 0.5;
  return t < 0.1 ? 3 : t < 0.55 ? 2 : t < 1 ? 1 : 0;
}

function bar(x: number, y: number, cx: number, cy: number, slope: number, half: number, soft: number): number {
  const d = (y + 0.5 - cy - slope * (x + 0.5 - cx)) / Math.hypot(1, slope);
  return level(Math.abs(d) - half, soft);
}

function fruitShadow(x: number, y: number): number {
  const [px, py] = [x + 0.5 - 40, y + 0.5 - 38.75];
  const det = 21.5 * 2.75 + 6.5 * 4;
  const s = (2.75 * px + 6.5 * py) / det;
  const t = (-4 * px + 21.5 * py) / det;
  const f = Math.hypot(s, t);
  if (f < 0.01) return 3;
  const gx = (s * 2.75 - t * 4) / det / f;
  const gy = (s * 6.5 + t * 21.5) / det / f;
  return level((f - 1) / Math.hypot(gx, gy), 0.4 + 1.3 * clamp((s + 0.5) / 1.5));
}

function cast(x: number, y: number): number {
  const rail = y <= 29 ? 3 : y === 30 ? 2 : 0;
  const far = bar(x, y, 32, 52.5, 0.21, 1.5, 1.8);
  const near = bar(x, y, 56, 30, 0.17, 1, 1.1);
  return Math.max(rail, fruitShadow(x, y), far, near);
}

const tones = ["cream", "grain", "veil", "shade"];

art.each((x, y, ink) => {
  if (y < 26 || y > 59 || ink === null) return undefined;
  const depth = cast(x, y);
  if (depth === 0) return undefined;
  return tones[Math.min(3, depth + (ink === "veil" ? 1 : 0))];
});

function warm(ink: string, list: [number, number, number][]): void {
  for (const [x, y, len] of list)
    for (let u = x; u < x + len; u++) if (tones.includes(art.get(u, y) ?? "")) art.px(u, y, ink);
}

warm("blush", [
  [7, 28, 1],
  [7, 29, 1],
  [6, 30, 2],
]);
warm("peach", [
  [7, 31, 2],
  [7, 32, 2],
  [7, 33, 3],
  [8, 34, 3],
  [9, 35, 3],
  [10, 36, 4],
  [12, 37, 5],
  [15, 38, 4],
]);

["............drr", "...........ddrrr", ".........dddrrrr", "......ddddrrrr", "dddddddddrrrr"].forEach((row, v) => {
  [...row].forEach((glyph, u) => {
    const [x, y] = [19 + u, 34 + v];
    if (glyph !== "." && art.get(x, y) === "shade") art.px(x, y, key[glyph as keyof typeof key]);
  });
});

art.sprite(
  8,
  20,
  [
    ".......iiiiiiiiiiii.......",
    "....iiiiiiiiiiiiiiiiii....",
    "..iiiiijjjookkkkkkkkojjj..",
    ".iiiijjjjooooookkkkkkkkko.",
    "iiijjjjjjjjoooookkkkkkkkko",
    "iijjjjjjjjjjjoooooookkkkkk",
    "iijjjjjjjjjjjjoooooookkkkk",
    "ijjjjjjjjjjjjjjoooooookkkk",
    "ijjjjjjjjjjjjjjjooooookkkl",
    "ijjjjjjjjjjjjjjjjoooookkkl",
    "jjjjjjjjjjjjjjjjjoooookkll",
    ".jjjjjjjjjjjjjjjjjooookll.",
    ".jjjjjjjjjjjjjjjjjoookkll.",
    "..jjjjjjjjjjjjjjjjoookll..",
    "...jjjjjjjjjjjjjjjookll...",
    "....jjjjjjjjjjjjjookll....",
    "......jjjjjjjjjjookl......",
    ".........oooookkl.........",
  ],
  key,
);

art.sprite(10, 22, ["..e", "ee"], key);

art.sprite(
  14,
  19,
  ["......nm.......", "....nnnmmmm....", ".nnnnnmnmmmmmm.", "nnnmm.nnm.mmmmm", ".......nm......"],
  key,
);

save(import.meta.dir, "2x", art);
