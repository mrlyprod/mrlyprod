import { Volume } from "../sculpt.ts";

const colors = {
  d: "#2c5a6e",
  s: "#3d7d86",
  f: "#d3dfdb",
  k: "#4e4956",
  r: "#77706c",
  g: "#738450",
  R: "#a63f37",
  C: "#e3d8c1",
  i: "#465059",
  L: "#f5b748",
};
const v = new Volume(16, colors);
const key: Record<string, string> = Object.fromEntries(Object.keys(colors).map((name) => [name, name]));

const height = [
  "................",
  "..1111..........",
  ".1354211....121.",
  ".2442222210.2420",
  ".1322222221.131.",
  ".1222222222101..",
  ".022222222211...",
  ".122222222210...",
  "..1222222221....",
  "...12221110.....",
  "...011110.......",
  "....1110........",
  "................",
  "................",
  "................",
  "................",
];

const tone = [
  "................",
  "................",
  "..g.............",
  ".gg.....k.......",
  "........kk......",
  "................",
  "................",
  "................",
  ".........gg.....",
  "................",
  "................",
  "................",
  "................",
  "................",
  "................",
  "................",
];

const sea = [
  "..sssddddddddd..",
  ".s....ssdddsdss.",
  "s.......ssds...s",
  "s..........f....",
  "s..........s...f",
  "s.............fd",
  "s............fsd",
  "s............fdd",
  "ds..........fsdd",
  "dsf........ffddd",
  "ds.......ffsdddd",
  "s.......fsdddddd",
  "s...fffsdddddddd",
  "d...sddddddsssdd",
  ".dddddsssdddddd.",
  "..dddddddddddd..",
];

for (let z = 0; z < 16; z++) {
  for (let x = 0; x < 16; x++) {
    const h = Number(height[z]![x]);
    for (let y = 0; y <= h; y++) v.set(x, y, z, y <= 1 ? "k" : "r");
    const top = tone[z]![x]!;
    if (top !== ".") v.set(x, h, z, top);
  }
}
v.layer(0).sprite(0, 0, sea, key);
v.box(4, 1, 10, 3, 1, 2, "r");
v.set(4, 2, 11, "i").set(3, 3, 8, "r").set(2, 3, 7, "r").set(13, 4, 3, "g");

const tower: Record<number, string[]> = {
  3: [".RRR.", "RRRRR", "RRRRR", "RRRRR", ".RiR."],
  4: [".RRR.", "RRRRR", "RRRRR", "RRRRR", ".RiR."],
  5: [".CCC.", "CCCCC", "CCCCC", "CCCCC", ".CCC."],
  6: [".....", ".CiC.", ".iCC.", ".CCC.", "....."],
  7: [".....", ".RRR.", ".RRR.", ".RRR.", "....."],
  8: [".....", ".RRR.", ".RRR.", ".RRR.", "....."],
  9: [".....", ".CCC.", ".CCC.", ".CCC.", "....."],
  10: [".iii.", "iiiii", "iiiii", "iiiii", ".iii."],
  11: [".....", ".LLL.", ".LLL.", ".LLL.", "....."],
  12: [".....", ".LLL.", ".LLL.", ".LLL.", "....."],
  13: [".....", ".RRR.", ".RRR.", ".RRR.", "....."],
  14: [".....", ".....", "..R..", ".....", "....."],
  15: [".....", ".....", "..i..", ".....", "....."],
};
for (const [y, rows] of Object.entries(tower)) v.layer(Number(y)).sprite(3, 3, rows, key);

const cottage: Record<number, string[]> = {
  3: ["CCC.", "CCC.", "CCC."],
  4: ["CCC.", "CCi.", "CiC."],
  5: ["kkkk", "CCC.", "kkkk"],
  6: ["....", "kkkk", "...."],
};
for (const [y, rows] of Object.entries(cottage)) v.layer(Number(y)).sprite(8, 5, rows, key);
v.set(9, 6, 5, "r").set(9, 7, 5, "k");

const boat: Record<number, string[]> = {
  0: [".R.", "RkR", "RkR", "RkR", ".R."],
  1: [".R.", "R.R", "RCR", "R.R", ".R."],
};
for (const [y, rows] of Object.entries(boat)) v.layer(Number(y)).sprite(1, 10, rows, key);

v.save(import.meta.dir, "1x");
