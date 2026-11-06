import { Canvas, save } from "../paint.ts";

const S = 64;
const inks = {
  k: "#3e1a2c",
  q: "#5c2434",
  s: "#893839",
  d: "#a8482c",
  r: "#c8622f",
  m: "#e2883a",
  n: "#f0a04c",
  l: "#f8bc68",
  h: "#fdd898",
  c: "#fde8c8",
  e: "#8cc43c",
  g: "#c4e070",
};
type Ink = keyof typeof inks;
const DARKER: Record<string, Ink> = {
  c: "h",
  h: "l",
  l: "n",
  n: "m",
  m: "r",
  r: "d",
  d: "s",
  s: "s",
  q: "q",
  k: "k",
  e: "e",
  g: "e",
};
const art = new Canvas(S, S, inks);
const key = Object.fromEntries(Object.keys(inks).map((name) => [name, name]));
const LX = 0.6;
const LY = -0.8;

class Part {
  shape = new Canvas(S, S, { x: "#000000" });
  has(x: number, y: number): boolean {
    return this.shape.get(Math.round(x), Math.round(y)) === "x";
  }
  stroke(points: [number, number, number][]): this {
    for (let i = 0; i + 1 < points.length; i++) {
      const [x0, y0, r0] = points[i]!;
      const [x1, y1, r1] = points[i + 1]!;
      const n = Math.ceil(Math.hypot(x1 - x0, y1 - y0) * 3);
      for (let k = 0; k <= n; k++) {
        const t = k / n;
        const r = r0 + (r1 - r0) * t;
        this.shape.ellipse(x0 + (x1 - x0) * t, y0 + (y1 - y0) * t, r, r, "x");
      }
    }
    return this;
  }
  reach(x: number, y: number, dx: number, dy: number): number {
    let d = 0;
    while (d < 48 && this.has(x + dx * (d + 0.5), y + dy * (d + 0.5))) d += 0.5;
    return d;
  }
  paint(ramp: Ink[], rim: Ink, edge: Ink): this {
    this.shape.each((x, y, v) => {
      if (v !== "x") return undefined;
      const lit = this.reach(x, y, LX, LY);
      const dark = this.reach(x, y, -LX, -LY);
      const t = Math.min(0.999, dark / (lit + dark + 0.001));
      let ink: Ink = ramp[Math.floor(t * ramp.length)]!;
      const open = (u: number, v: number) => !this.has(u, v);
      if (lit < 1 && open(x + LX * 1.5, y + LY * 1.5)) ink = rim;
      if (dark < 1) ink = open(x - LX * 1.5, y - LY * 1.5) ? edge : DARKER[ink]!;
      art.px(x, y, ink);
      return undefined;
    });
    return this;
  }
}

const tailPath: [number, number, number][] = [
  [13, 30, 2.1],
  [11, 26, 2],
  [9.5, 23, 1.9],
  [7.5, 20, 1.8],
  [6, 17, 1.8],
  [5.5, 14, 1.7],
  [6, 11, 1.7],
  [7.5, 8.5, 1.6],
  [10, 6.5, 1.6],
  [13, 5.5, 1.5],
  [16, 6.5, 1.5],
  [18, 8.5, 1.4],
  [19, 11, 1.2],
];
function along(x: number, y: number): number {
  let [best, at, run] = [Infinity, 0, 0];
  for (let i = 0; i + 1 < tailPath.length; i++) {
    const [x0, y0] = tailPath[i]!;
    const [x1, y1] = tailPath[i + 1]!;
    const [dx, dy] = [x1 - x0, y1 - y0];
    const len = Math.hypot(dx, dy);
    const t = Math.max(0, Math.min(1, ((x - x0) * dx + (y - y0) * dy) / (len * len)));
    const d = Math.hypot(x0 + dx * t - x, y0 + dy * t - y);
    if (d < best) [best, at] = [d, run + t * len];
    run += len;
  }
  return at;
}

const tail = new Part().stroke(tailPath).paint(["s", "r", "m", "n", "h"], "l", "s");
const rings: [number, number][] = [
  [2.5, 6],
  [12, 15.5],
  [20.5, 24],
];
tail.shape.each((x, y, v) => {
  if (v !== "x") return undefined;
  const a = along(x, y);
  if (rings.some(([from, to]) => a >= from && a < to)) art.px(x, y, DARKER[DARKER[DARKER[art.get(x, y)!]!]!]!);
  return undefined;
});

art.sprite(
  0,
  21,
  [
    "........................................l",
    "......................................ll",
    "....................................llnn",
    "..................................llnnmm",
    "................................llnnmmmr",
    ".............................lllnrrmmmrr",
    "............nlllllllll.....llrrrmrmmmrrr",
    "...........smnnnnhhhnnlllllnnrrnmrmmmmrrr",
    "..........sdrmmnnnnhhnnnnrrrnrnmrmmmmmrrrd",
    ".........sdrrmmnnnnnnhnnnrrmnrmmrmmmmmnmrrd",
    "........sdrrmmmmnnnnnhnnnrmmrmmmmmmmmnnlmmmmnlhhl",
    "........sddrrrmmnnnnnnnnmrmmrmmmmmmmmnnlnmmnlhchl",
    "........sddrrmmmmnnnnnmrrmmmmmmmmmmrmmnnnmmnlhchl",
    "........sdrrrrmmmmnnnnmrmmmmmmmmmmmrmmmnnnmmnlhhl",
    "........sdddrrmmmmmmnnmrmmmmmrmrrrrrrmmmnnmmnnll",
    "........sdddrrrrmmmmmnmrmmrmrrrrrrrdrmmmmnnmmnnl",
    ".........sddrrrrmmmmmmmrrrrrrrrrddddrrmmmmnnmmnl",
    "..........sdddrrrrmmmmmnrrrrrdddddddddrrmmmmnnnl",
    "...........sddrrrrmmmmnndddddssssssddddrmmmmmnnl",
    "............sddrrrrmmmnlsssss.....sddr.srmmmnnnl",
    "..............ssdrrmmnnl.ssddd....sddrsdrmmmnnl",
    "................sdrmmmnl.sdddr....sddr.sdrmmnnl",
    "................srmmmnnl.sddrr....sddr...srmmnnl",
    "...............mnnmmrrs..sddrr....sddr....srmnnl",
    "..............mnnmmrs...rddds....sddr......srmmnl",
    ".............mnmmrs....rddds.....sddr.......srmnl",
    "............mnmrs......rdds.....sddr........srmmnl",
    "...........mnmrs......rddds.....sddr.........srmnl",
    "..........srmns.......sdddr....sddr...........srmnl",
    "...........srmn........sddr....sddr...........srmnl",
    "..........srmn.........sddr....sddr............srmnl",
    "..........srmn..........sddr..sddr.............srmnl",
    "..........srmn..........sddr..sddr.............srml",
    ".........srmn...........sdddr..sddr............srml",
    ".........srmn............sddr..sddr............srmnl",
    ".........srmn............ssss..sdddr...........srmmnnl",
    ".........srmnn.................sdddrr...........sddds",
    "........ssdddrs................ssssss",
  ],
  key,
);

art.sprite(
  40,
  11,
  [
    "......n................",
    ".....mn................",
    "....rmmn...............",
    "....rmmn...n...........",
    ".....dmmn.rrn..........",
    "...rmmmmndrrrn.........",
    "..rmmmmmnndrrrn........",
    "..rrmmmnnnmnnnnnnl.....",
    "..mmnnnlnhhllllllll....",
    ".mmnnlnllllllllllhl....",
    ".mnnnllqqqqqqllllhhl...",
    "mnnnllqeeeckelllllhl...",
    "mnnnnlqggggkqnlllhhl...",
    "mmnnrrrrrrrrnnlllcdr...",
    "mmrmnnnnnnnnnnlccch.hh.",
    "mmmmmmmmnnnnnncccr.....",
    "rmmmmmmmmnnnncccr.hhh..",
    ".rmmmmmmmmnnnchr.......",
    "..rrmmmmmmmnnn.........",
    "...drrrrrrrd...........",
  ],
  key,
);

save(import.meta.dir, "4x", art);
