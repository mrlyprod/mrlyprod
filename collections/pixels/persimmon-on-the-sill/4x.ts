import { Canvas, save } from "../paint.ts";

const art = new Canvas(128, 128, {
  sky: "#c5d3d6",
  mist: "#cfd9d5",
  haze: "#dadcd0",
  cream: "#f1e3c2",
  grain: "#e8d8b6",
  peach: "#f3d2a4",
  gauze: "#e0d2bc",
  veil: "#cfc3b6",
  fade: "#b0acb5",
  shade: "#8f95b0",
  dusk: "#767d9c",
  blush: "#a58694",
  pewter: "#626c88",
  slate: "#56607e",
  ink: "#3f4762",
  shine: "#ffcb6b",
  glow: "#ffb347",
  tangerine: "#f99530",
  orange: "#f07a1a",
  amber: "#d95f18",
  rust: "#b4421f",
  wine: "#742a2c",
  sage: "#a69c5a",
  olive: "#7b7a43",
  moss: "#55592f",
});

const key = {
  e: "cream",
  s: "dusk",
  r: "blush",
  d: "slate",
  h: "shine",
  i: "glow",
  t: "tangerine",
  j: "orange",
  o: "amber",
  k: "rust",
  l: "wine",
  n: "sage",
  u: "olive",
  m: "moss",
};

const clamp = (value: number, low = 0, high = 1) => Math.max(low, Math.min(high, value));

function runs(ink: string, list: [number, number, number][]): void {
  for (const [x, y, len] of list) art.rect(x, y, len, 1, ink);
}

art.rect(0, 0, 128, 8, "sky");
art.rect(0, 8, 128, 5, "mist");
art.rect(0, 13, 128, 5, "haze");
art.rect(0, 18, 128, 6, "cream");
art.rect(10, 7, 44, 1, "mist");
art.rect(18, 6, 23, 1, "mist");
art.rect(70, 8, 26, 1, "sky");
art.rect(106, 7, 22, 1, "mist");
art.rect(6, 12, 42, 1, "haze");
art.rect(14, 11, 24, 1, "haze");
art.rect(64, 12, 30, 1, "haze");
art.rect(108, 13, 20, 1, "mist");
art.rect(0, 17, 60, 1, "cream");
art.rect(0, 16, 32, 1, "cream");
art.rect(40, 15, 17, 1, "cream");
art.rect(74, 18, 54, 1, "haze");
art.rect(104, 19, 24, 1, "haze");

art.rect(96, 0, 1, 24, "shade");
art.rect(97, 0, 1, 24, "dusk");
art.rect(98, 0, 4, 24, "slate");
art.rect(102, 0, 2, 24, "ink");

art.rect(0, 24, 128, 2, "shade");
art.rect(0, 26, 128, 2, "dusk");
art.rect(0, 28, 128, 23, "slate");
art.rect(0, 46, 128, 5, "pewter");
art.rect(0, 51, 128, 1, "ink");

runs("dusk", [
  [6, 25, 7],
  [46, 25, 10],
  [104, 24, 12],
  [104, 25, 16],
  [122, 25, 6],
]);
runs("slate", [
  [104, 26, 9],
  [104, 27, 13],
  [30, 27, 12],
  [72, 27, 8],
]);
runs("pewter", [
  [4, 31, 22],
  [10, 32, 34],
  [30, 33, 8],
  [72, 34, 26],
  [78, 35, 34],
  [96, 36, 10],
  [110, 30, 18],
  [116, 31, 8],
  [0, 38, 11],
  [0, 39, 6],
  [86, 40, 22],
  [92, 41, 30],
  [118, 42, 10],
  [0, 45, 13],
  [70, 45, 12],
  [96, 45, 20],
  [122, 45, 6],
  [74, 44, 5],
  [102, 44, 9],
]);
runs("ink", [
  [42, 29, 20],
  [48, 30, 8],
  [70, 38, 14],
  [76, 39, 6],
  [2, 42, 9],
  [112, 37, 12],
]);
runs("slate", [
  [84, 46, 6],
  [116, 46, 5],
  [72, 48, 16],
  [78, 49, 6],
  [100, 49, 20],
  [3, 48, 9],
]);
runs("dusk", [
  [0, 50, 14],
  [68, 50, 20],
  [94, 50, 9],
  [110, 50, 18],
]);

art.rect(0, 52, 128, 61, "cream");
art.rect(0, 113, 128, 1, "grain");
art.rect(0, 114, 128, 2, "gauze");
art.rect(0, 116, 128, 2, "veil");
art.rect(0, 118, 128, 7, "shade");
art.rect(0, 125, 128, 1, "dusk");
art.rect(0, 126, 128, 2, "slate");

runs("grain", [
  [0, 66, 9],
  [0, 67, 5],
  [0, 72, 7],
  [72, 64, 20],
  [78, 65, 24],
  [98, 70, 16],
  [104, 71, 8],
  [2, 79, 12],
  [6, 80, 18],
  [22, 84, 20],
  [28, 85, 24],
  [60, 90, 26],
  [68, 91, 30],
  [76, 92, 14],
  [96, 96, 20],
  [104, 97, 22],
  [118, 100, 10],
  [0, 97, 16],
  [0, 98, 24],
  [6, 99, 10],
  [20, 104, 26],
  [28, 105, 34],
  [40, 106, 16],
  [2, 109, 18],
  [10, 110, 30],
  [56, 110, 22],
  [64, 111, 26],
  [0, 112, 30],
  [38, 112, 14],
  [60, 112, 40],
  [108, 112, 20],
]);
runs("gauze", [
  [4, 56, 8],
  [72, 55, 24],
  [80, 56, 10],
  [108, 57, 14],
  [10, 80, 8],
  [34, 85, 10],
  [72, 91, 12],
  [4, 98, 10],
  [34, 105, 14],
  [18, 110, 12],
  [70, 76, 16],
  [88, 80, 22],
  [96, 81, 10],
]);

runs("cream", [
  [34, 113, 12],
  [70, 113, 8],
  [98, 113, 5],
]);
runs("veil", [
  [58, 115, 6],
  [104, 115, 10],
  [106, 114, 5],
  [20, 115, 7],
]);
runs("fade", [
  [12, 116, 7],
  [13, 117, 4],
  [50, 116, 5],
  [48, 117, 9],
  [88, 116, 5],
  [87, 117, 8],
  [120, 116, 6],
  [30, 118, 10],
  [72, 118, 6],
  [110, 118, 12],
]);
runs("blush", [
  [14, 117, 3],
  [51, 117, 4],
]);
runs("dusk", [
  [6, 120, 18],
  [10, 121, 10],
  [52, 122, 24],
  [90, 120, 14],
  [96, 121, 20],
  [28, 124, 30],
  [80, 124, 22],
]);

function level(sd: number, soft: number): number {
  const t = sd / soft + 0.5;
  return t < 0 ? 4 : t < 0.34 ? 3 : t < 0.67 ? 2 : t < 1 ? 1 : 0;
}

function bar(x: number, y: number, cx: number, cy: number, slope: number, half: number, soft: number): number {
  const d = (y + 0.5 - cy - slope * (x + 0.5 - cx)) / Math.hypot(1, slope);
  return level(Math.abs(d) - half, soft);
}

function fruitShadow(x: number, y: number): number {
  const [px, py] = [x + 0.5 - 80, y + 0.5 - 77.5];
  const det = 43 * 5.5 + 13 * 8;
  const s = (5.5 * px + 13 * py) / det;
  const t = (-8 * px + 43 * py) / det;
  const f = Math.hypot(s, t);
  if (f < 0.01) return 4;
  const gx = (s * 5.5 - t * 8) / det / f;
  const gy = (s * 13 + t * 43) / det / f;
  return level((f - 1) / Math.hypot(gx, gy), 0.6 + 2.6 * clamp((s + 0.5) / 1.5));
}

function cast(x: number, y: number): number {
  const rail = clamp(63 - y, 0, 4);
  const far = bar(x, y, 64, 105, 0.21, 2.6, 3.6);
  const near = bar(x, y, 112, 60, 0.17, 2, 2.2);
  return Math.max(rail, fruitShadow(x, y), far, near);
}

const tones = ["cream", "grain", "gauze", "veil", "fade", "shade", "dusk"];

art.each((x, y, ink) => {
  if (y < 52 || y > 117 || ink === null) return undefined;
  const depth = cast(x, y);
  if (depth === 0) return undefined;
  const base = Math.max(1, tones.indexOf(ink));
  return tones[Math.min(y > 112 ? 5 : 6, base + depth)];
});

function warm(ink: string, list: [number, number, number][]): void {
  for (const [x, y, len] of list)
    for (let u = x; u < x + len; u++) if (tones.includes(art.get(u, y) ?? "")) art.px(u, y, ink);
}

warm("blush", [
  [15, 55, 1],
  [15, 56, 1],
  [14, 57, 2],
  [14, 58, 2],
  [13, 59, 3],
  [13, 60, 3],
]);
warm("peach", [
  [14, 61, 2],
  [14, 62, 3],
  [13, 63, 4],
  [13, 64, 4],
  [13, 65, 5],
  [13, 66, 5],
  [14, 67, 5],
  [14, 68, 6],
  [15, 69, 6],
  [16, 70, 6],
  [17, 71, 7],
  [19, 72, 7],
  [21, 73, 7],
  [23, 74, 8],
  [26, 75, 9],
  [29, 76, 10],
  [33, 77, 8],
]);

[
  "............................drrrr",
  "...........................ddrrrrrr",
  "..........................ddrrrrrrr",
  "........................dddrrrrrrr",
  "......................dddrrrrrrrr",
  "....................dddrrrrrrrr",
  ".................ddddrrrrrrrr",
  ".............ddddddrrrrrrrr",
  "...ddddddddddddddsssss",
  ".......ddddddssssssssss",
  "...........ssssssssssss",
  "...............ssssss",
].forEach((row, v) => {
  [...row].forEach((glyph, u) => {
    const [x, y] = [36 + u, 68 + v];
    if (glyph !== "." && art.get(x, y) === "shade") art.px(x, y, key[glyph as keyof typeof key]);
  });
});

art.sprite(
  16,
  40,
  [
    "...............iiiiiiiiiiiiiiiiiiiiii...............",
    "............iiiiiiiiiiiiiiiiiiiiiiiitiii............",
    ".........iiiiiiiiiiiiiiiiiiiiiitttttttttttt.........",
    ".......iiiiiiiiiiiiiittttttttttttttttttttjjtt.......",
    ".....iiiiiiiiiitttjjjjjjjjjjjjjkkkkkkooooojjjjj.....",
    "....iiiiiiiiitttjjooookkkkkkkkkkkkkkkkkkkoooojjj....",
    "...iiiiiiiittttjjjooooooooooookkkkkkkkkkkkkkkkkoo...",
    "..iiiiiiittttjjjjjjjoooooooookkkkkkkkkkkkkkkkkkkoo..",
    ".iiiiiittttjjjjjjjjjjjooooooooookkkkkkkkkkkkkkkkkoo.",
    ".iiiiittttjjjjjjjjjjjjjjooooooooooookkkkkkkkkkkkkko.",
    "iiiittttjjjjjjjjjjjjjjjjjjoooooooooooookkkkkkkkkkkkk",
    "iiiittttjjjjjjjjjjjjjjjjjjjooooooooooooookkkkkkkkkkk",
    "iiittttjjjjjjjjjjjjjjjjjjjjjooooooooooooookkkkkkkkkk",
    "iiitttjjjjjjjjjjjjjjjjjjjjjjjooooooooooooookkkkkkkkk",
    "iitttjjjjjjjjjjjjjjjjjjjjjjjjjoooooooooooookkkkkkkkk",
    "iitttjjjjjjjjjjjjjjjjjjjjjjjjjjoooooooooooookkkkkkkk",
    "iittjjjjjjjjjjjjjjjjjjjjjjjjjjjjooooooooooookkkkkkkl",
    "iittjjjjjjjjjjjjjjjjjjjjjjjjjjjjooooooooooookkkkkkkl",
    "ittjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjoooooooooookkkkkkll",
    "ittjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjoooooooooookkkkkkll",
    "itjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjooooooooookkkkklll",
    "tjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjooooooooookkkkklll",
    ".tjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjoooooooookkkklll.",
    ".tjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjooooooookkkkklll.",
    ".ttjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjooooooookkkkllll.",
    "..ttjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjoooooookkkllll..",
    "..tttjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjooooookkkkllll..",
    "...ttttjjjjjjjjjjjjjjjjjjjjjjjjjjjjjooooookkkllll...",
    "....ttttttjjjjjjjjjjjjjjjjjjjjjjjjjjoooookkkllll....",
    ".....ttttttttjjjjjjjjjjjjjjjjjjjjjjjoooookkllll.....",
    "......tttttttttjjjjjjjjjjjjjjjjjjjjoooookkllll......",
    "........tttttttttjjjjjjjjjjjjjjjjjjooookklll........",
    "..........ttttttttjjjjjjjjjjjjjjjjooooklll..........",
    "............tttttttjjjjjjjjjjjjjjoooklll............",
    "...............jjjjjjjjjjooooooookkll...............",
    "...................ooooooooookkll...................",
  ],
  key,
);

art.sprite(19, 44, ["......hhhh", "....hhehh", "..hheehh", ".hhehh", "hhhh", "hh"], key);

art.sprite(
  27,
  38,
  [
    ".............nnum.............",
    "............nnuumm............",
    ".........nnnnnmummuuu.........",
    ".....nnnnnnnnmmmmmmuuuuuu.....",
    "..nnnnnnnnuummnnnummuuuuuumm..",
    "nnnnuuuuummnnnnnuuummuuuummmmm",
    "....uummm.nnnnmuuumm.mmmmm....",
    "...........nnnmuuumm..........",
    "............nnmuumm...........",
    ".............nmumm............",
    "..............mm..............",
  ],
  key,
);

save(import.meta.dir, "4x", art);
