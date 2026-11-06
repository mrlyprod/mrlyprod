import { fbm, noise } from "../../pixels/paint.ts";
import { Volume } from "../sculpt.ts";

const colors = {
  d: "#2a566a",
  m: "#33697a",
  s: "#437f87",
  f: "#d3dfdb",
  k: "#4a4552",
  n: "#615a62",
  r: "#7a736d",
  g: "#738450",
  b: "#6b4f3f",
  R: "#a63f37",
  C: "#e3d8c1",
  i: "#465059",
  L: "#f5b748",
};
const v = new Volume(32, colors);
const key: Record<string, string> = Object.fromEntries(Object.keys(colors).map((name) => [name, name]));

const height = [
  "................................",
  "................................",
  "....137631......................",
  "...148a97421............1231....",
  "..158bb89753...........169972...",
  "..379a87865553.........28ab831..",
  ".2588766665555.3.......279a841..",
  ".3676766655555555244...147752...",
  ".2565656555555555555441..1331...",
  ".145555555555555555555552.1.....",
  ".335555555555555555555551.......",
  "..5555555555555555555552441.....",
  ".2665555555555555555555544.3....",
  ".266555555555555555555551..2....",
  "..1554555555555555555554........",
  "..3355555555555555555552........",
  "....55555555555555555554........",
  "....35555555555555555552........",
  ".....155555555555555555.........",
  "......13344444545155554.........",
  "......12333333344.1111..........",
  ".......1333333331...33..........",
  "........33333331................",
  ".........11..1..................",
];

const sea = [
  "....mssfsfsmmddddddddddmmsmm....",
  "..mmsfffffsfsmddddddddmssfssmm..",
  ".msf......ffssmmddddmmssffffsmm.",
  ".mf.........fssmmdddmmsf....fsm.",
  "ff..........ffsmmdmmssf......fsm",
  "ff............fsfssmssf.......fs",
  "f................fssfsf.......sm",
  "f....................ff......ffs",
  "f......................ff....fsm",
  "f.......................fs.fsmmm",
  "f.........................fssmmd",
  "s..........................fsmdd",
  "f...........................fsmm",
  "f..........................smmdd",
  "s.........................fsmddd",
  "mf.......................ssmmddd",
  "msf......................fsmdddd",
  "dmsf....................ssmmdddd",
  "dmmsf...................fsmdddmm",
  "dmmssf................ffsmmdmmdd",
  "ddmmsf..............fsfsmdmmdddd",
  "dmmmmss...........fssmdddddddddd",
  "dmmmmmsf........fssmmddddddddddd",
  "ddmmmmmsf..ff.fssmmdddddmmdddddd",
  "dddmmmmmsfsssfsmmdddddmmddddmmdd",
  "ddddmmmmmssmmsmmddddmmddddmmdddd",
  "dddddmmmmmmmmmddddmmddddmmdddddd",
  "ddddddmmmddddddddddddddmmddddddd",
  ".dddddddddddddddddddmmddddddddd.",
  ".ddddddddddddddddddmmdddddddddd.",
  "..ddddddddddddddmmdddddddddddd..",
  "....dddddddddddddddddddddddd....",
];

const cut = [4, 2, 1, 1, ...new Array(24).fill(0), 1, 1, 2, 4];
v.layer(0).each((x, z) => (x >= cut[z]! && x < 32 - cut[z]! ? "d" : undefined));
v.layer(1).sprite(0, 0, sea, key);
for (let z = 0; z < 32; z++) {
  for (let x = 0; x < 32; x++) {
    const c = height[z]?.[x] ?? ".";
    if (c === ".") continue;
    const h = parseInt(c, 16);
    for (let y = 0; y <= h; y++) v.set(x, y, z, y <= 3 ? "k" : y === 4 ? "n" : "r");
  }
}
const land = (x: number, z: number) => (height[z]?.[x] ?? ".") !== ".";
const gap = (x: number, z: number) => {
  let best = 9;
  for (let dz = -3; dz <= 3; dz++)
    for (let dx = -3; dx <= 3; dx++) if (land(x + dx, z + dz)) best = Math.min(best, Math.hypot(dx, dz));
  return best;
};
const exposed = (x: number, z: number) => {
  let free = 0;
  for (const [ux, uz] of [
    [0, 1],
    [0.5, 1],
    [1, 1],
    [1, 0.5],
  ]) {
    let hit = false;
    for (let t = 1; t <= 8 && !hit; t++) hit = land(Math.round(x + ux! * t), Math.round(z + uz! * t));
    if (!hit) free++;
  }
  return free / 4;
};
const spray: [number, number][] = [];
v.layer(1).each((x, z, ink) => {
  if (x < cut[z]! || x >= 32 - cut[z]! || land(x, z)) return undefined;
  const [d, open] = [gap(x, z), exposed(x, z)];
  if (open >= 0.75 && d < 1.2 && noise(x * 0.8, z * 0.8, 23) > 0.6) spray.push([x, z]);
  const edge = (0.5 + open * 1.4) * (0.35 + 1.2 * fbm(x * 0.7, z * 0.7, 21, 3));
  const lace = d < 1.2 || noise(x * 1.1, z * 1.1, 29) > 0.4;
  return d <= edge && lace ? "f" : ink === null || ink === "f" ? "s" : ink;
});
for (const [x, z] of spray) v.set(x, 2, z, "f");
v.box(18, 4, 20, 4, 1, 1, "n").box(18, 5, 20, 4, 1, 1, "r");

const D11 = [
  "...#####...",
  "..#######..",
  ".#########.",
  "###########",
  "###########",
  "###########",
  "###########",
  "###########",
  ".#########.",
  "..#######..",
  "...#####...",
];
const D9 = [
  "..#####..",
  ".#######.",
  "#########",
  "#########",
  "#########",
  "#########",
  "#########",
  ".#######.",
  "..#####..",
];
const D7 = ["..###..", ".#####.", "#######", "#######", "#######", ".#####.", "..###.."];
const D5 = [".###.", "#####", "#####", "#####", ".###."];
const D3 = [".#.", "###", ".#."];

const disc = (y: number, shape: string[], ink: string) => {
  const half = (shape.length - 1) / 2;
  v.layer(y).sprite(11 - half, 11 - half, shape, { "#": ink });
};

disc(6, D11, "n");
for (let y = 7; y <= 10; y++) disc(y, D9, "R");
for (let y = 11; y <= 14; y++) disc(y, D9, "C");
for (let y = 15; y <= 18; y++) disc(y, D7, "R");

const brackets = [
  "..i.i.i..",
  ".i.CCC.i.",
  "i.CCCCC.i",
  ".CCCCCCC.",
  "iCCCCCCCi",
  ".CCCCCCC.",
  "i.CCCCC.i",
  ".i.CCC.i.",
  "..i.i.i..",
];
v.layer(19).sprite(7, 7, brackets, key);
disc(20, D11, "i");
v.layer(21).sprite(
  6,
  6,
  [
    "...i.i.i...",
    "..i.....i..",
    ".i.......i.",
    "i.........i",
    "...........",
    "i.........i",
    "...........",
    "i.........i",
    ".i.......i.",
    "..i.....i..",
    "...i.i.i...",
  ],
  key,
);
v.layer(22).sprite(
  6,
  6,
  [
    "...iiiii...",
    "..i.....i..",
    ".i.......i.",
    "i.........i",
    "i.........i",
    "i.........i",
    "i.........i",
    "i.........i",
    ".i.......i.",
    "..i.....i..",
    "...iiiii...",
  ],
  key,
);
for (let y = 21; y <= 25; y++)
  v.layer(y).sprite(8, 8, ["..LiL..", ".iLLLi.", "LLLLLLL", "iLLLLLi", "LLLLLLL", ".iLLLi.", "..LiL.."], key);
disc(26, D7, "i");
disc(27, D7, "R");
disc(28, D5, "R");
disc(29, D3, "R");
v.set(11, 30, 11, "i").set(11, 31, 11, "i");

for (let y = 7; y <= 9; y++) {
  for (let x = 10; x <= 12; x++) v.set(x, y, 15, null).set(x, y, 14, "b");
}
v.set(11, 10, 15, null).set(11, 10, 14, "b");

const slit = (x: number, y: number, z: number, dx: number, dz: number) => {
  for (const h of [y, y + 1]) v.set(x, h, z, null).set(x + dx, h, z + dz, "i");
};
slit(7, 8, 11, 1, 0);
slit(11, 11, 7, 0, 1);
slit(15, 13, 11, -1, 0);
slit(11, 16, 14, 0, -1);

const cottage: Record<number, string[]> = {
  6: ["CCCCCC.", "CCCCCC.", "CCCCCC.", "CCCCCC.", "CCCCCC.", "CbCCCC."],
  7: ["CCCCCC.", "CCCCCC.", "CCCCCi.", "CCCCCi.", "CCCCCC.", "CbCiiC."],
  8: ["CCCCCC.", "CCCCCC.", "CCCCCi.", "CCCCCi.", "CCCCCC.", "CbCiiC."],
  9: ["CCCCCC.", "CCCCCC.", "CCCCCC.", "CCCCCC.", "CCCCCC.", "CCCCCC."],
};
for (const [y, rows] of Object.entries(cottage)) v.layer(Number(y)).sprite(16, 10, rows, key);
const roof: Record<number, string[]> = {
  9: ["kkkkkkk", "", "", "", "", "", "", "kkkkkkk"],
  10: ["", "kkkkkkk", "CCCCCC.", "CCCCCC.", "CCCCCC.", "CCCCCC.", "kkkkkkk", ""],
  11: ["", "", "kkkkkkk", "CCCCCC.", "CCCCCC.", "kkkkkkk", "", ""],
  12: ["", "", "", "kkkkkkk", "kkkkkkk", "", "", ""],
};
for (const [y, rows] of Object.entries(roof)) v.layer(Number(y)).sprite(16, 9, rows, key);
v.box(20, 12, 12, 2, 3, 2, "n").set(21, 14, 13, "k");
v.box(19, 6, 9, 2, 2, 1, "b");

for (const [x, z] of [
  [10, 16],
  [11, 16],
  [12, 16],
  [11, 17],
  [10, 17],
  [11, 18],
  [12, 18],
])
  v.set(x!, 5, z!, "n");
v.box(9, 4, 19, 5, 1, 1, "r").box(8, 3, 20, 7, 1, 3, "r");
v.box(17, 5, 17, 5, 1, 2, "b");
for (const [x, z] of [
  [17, 17],
  [19, 17],
  [21, 17],
  [18, 18],
  [20, 18],
])
  v.set(x!, 6, z!, "g");
v.box(9, 4, 21, 1, 2, 1, "i");
const lichen = [
  [6, 7, 2],
  [6, 10, 3],
  [7, 9, 3],
  [5, 9, 4],
  [3, 8, 6],
  [2, 6, 7],
  [3, 6, 4],
  [25, 9, 4],
  [26, 9, 4],
  [26, 10, 6],
  [3, 6, 12],
  [2, 3, 12],
];
for (const [x, y, z] of lichen) v.set(x!, y!, z!, "g");

const boat: Record<number, string[]> = {
  1: ["..R..", ".RbR.", "RbbbR", "RbbbR", "RbbbR", "RbbbR", "RbbbR", ".RbR.", "..R.."],
  2: ["..R..", ".R.R.", "R...R", "RCCCR", "R...R", "R...R", "RCCCR", ".R.R.", "..R.."],
  3: ["..R..", "", "", ".b...", ".b...", ".b...", ".b...", "", "..R.."],
};
for (const [y, rows] of Object.entries(boat)) v.layer(Number(y)).sprite(1, 21, rows, key);

v.save(import.meta.dir, "2x");
