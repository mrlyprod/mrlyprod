import { fbm, noise } from "../../pixels/paint.ts";
import { Volume } from "../sculpt.ts";

const colors = {
  d: "#27526a",
  m: "#30647a",
  s: "#3f7a85",
  a: "#5a949a",
  f: "#d3dfdb",
  e: "#a9c4c4",
  k: "#463f4e",
  n: "#5b5460",
  r: "#756e6b",
  l: "#8a827a",
  g: "#738450",
  b: "#6b4f3f",
  R: "#a63f37",
  C: "#e3d8c1",
  i: "#465059",
  L: "#f5b748",
};
const N = 64;
const v = new Volume(N, colors);
const key: Record<string, string> = Object.fromEntries(Object.keys(colors).map((name) => [name, name]));

const rock = [
  "................................................................",
  "................................................................",
  "................................................................",
  "..........3455433...............................................",
  ".......359b87ac954..............................................",
  "......38ehfbafhgb64.............................................",
  ".....4ajlkgcdhjkicc66...........................3345543.........",
  "....38hnmkhffijkkhdd76999996........454........35gjkkjh73.......",
  "....4bjmljhgghiihfdcc9aaaaa9..3..3..565.......36hlmnmlki63......",
  "....5cikjhgffggfedccc9aaaaa9533333..454.......49jmnnmlkj943.....",
  "....6dhihgfeeeddccbbb9aaaaa955888857aaaa7.....38ilmmlkjf8533....",
  "...37ehhgfeddcccbbbbb69999965899998abbbba.474.37hkllkjiea643....",
  "..359dffedccbbbbbbbbbbbbbbbb5899998abbbba5787.36cckkjihhb743....",
  "..47acdccbbbbbbbbbbbbbbbbbbbb588885abbbba5787.35bcijihgga633....",
  ".3.cdffeeffebbbbbbbbbbbbbbbbbbbbbbb7aaaa75787.3479fggfec843.....",
  ".4.cdeeddeedbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb469996469bcb975.......",
  "..4cddccddcbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb9aaa9..49985.........",
  "..3a6999996bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb9aaa9...344.55.......",
  "...49aaaaa9bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb69996.......4........",
  "...49aaaaa9bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbabba4............",
  "...39aaaaa9bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb3............",
  "...49aaaaa9bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb3............",
  "...49aaaaa9bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb3..464.......",
  "....6999996bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb3..676.......",
  "....455555555bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbba...676.......",
  "..addda8bb8bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb44444..464.......",
  "..3eeedbccbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbaaaa9............",
  "..3eeedbccbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbaaaaa443.........",
  "..deeedbccbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbaaa4a4...........",
  "..adddabccbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbaaaaa............",
  "....58888885bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbaaaaa............",
  ".56389999998bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbaaaa8............",
  ".67389999998bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb7774..............",
  "..5.89999998bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb7773..............",
  "....58888885bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb777...............",
  ".........7aaa7bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb9777...............",
  ".........abbbabbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb9998...............",
  ".........abbbaaaaaaaaaaaaaaabbbbbbbbbbbbbbbbb9999...............",
  ".........7aaa7aaaa9999999999bbbbbbbbbbbbbbbbb99994..............",
  "..........466664aa8888888888aabbbbb4bbbbbbaaa99993..............",
  "..........67747677777777777777bbbbb4333333aaa9999...............",
  "..........67747677777777777777ab4bb43333336669997...............",
  "..........4664647777777777777744344.3..3..6664.3................",
  "..............4377777777777777.3.3........343...................",
  "..............3377777777777777........4664......................",
  "................77777777777777........6776......................",
  "................43..33.....33.........4664......................",
  ".........................................3......................",
];
const height = Array.from({ length: N }, (_, z) =>
  Array.from({ length: N }, (_, x) => {
    const c = rock[z]?.[x] ?? ".";
    return c === "." ? -1 : parseInt(c, 36);
  }),
);
const stone = (x: number, y: number, z: number, h: number) => {
  const tide = 5 + Math.round(fbm(x * 0.15, z * 0.15, 7, 2) * 2);
  if (y <= tide) return "k";
  if (y === h) return h >= 13 ? "l" : "r";
  return noise(x * 0.31 + z * 0.27, y * 0.45, 9) > 0.64 ? "r" : "n";
};
for (let z = 0; z < N; z++) {
  for (let x = 0; x < N; x++) {
    const h = height[z]![x]!;
    for (let y = 0; y <= h; y++) v.set(x, y, z, stone(x, y, z, h));
  }
}
for (const [x, z, w, d, y0, y1] of [
  [36, 40, 6, 2, 7, 8],
  [51, 20, 1, 4, 8, 10],
  [29, 9, 5, 1, 6, 8],
  [3, 19, 1, 4, 7, 9],
]) {
  for (let t = z!; t < z! + d!; t++)
    for (let u = x!; u < x! + w!; u++) for (let y = y0!; y <= y1!; y++) v.set(u, y, t, stone(u, y, t, y1!));
}
const tall = (x: number, z: number) => height[z]?.[x] ?? -1;
const slot = (x: number, z: number) => {
  const h = tall(x, z) + 3;
  return (tall(x - 1, z) >= h && tall(x + 1, z) >= h) || (tall(x, z - 1) >= h && tall(x, z + 1) >= h);
};
const covered = (x: number, y: number, z: number) => {
  for (let up = y + 1; up < 16; up++) if (v.get(x, up, z) !== null) return true;
  return false;
};
const recess = (x: number, y: number, z: number) => v.get(x, y, z) === null && (slot(x, z) || covered(x, y, z));
v.each((x, y, z, ink) => {
  if ((ink !== "n" && ink !== "r") || v.get(x, y + 1, z) === null) return undefined;
  const shade = [
    [1, 0],
    [-1, 0],
    [0, 1],
    [0, -1],
  ].some(([dx, dz]) => recess(x + dx!, y, z + dz!));
  return shade ? "k" : undefined;
});

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
const cut = [8, 5, 3, 2, 2, 1, 1, 1];
const rim = (z: number) => cut[Math.min(z, N - 1 - z)] ?? 0;
const land = (x: number, z: number) => (height[z]?.[x] ?? -1) >= 3;
const gap = (x: number, z: number) => {
  let best = 9;
  for (let dz = -5; dz <= 5; dz++)
    for (let dx = -5; dx <= 5; dx++) if (land(x + dx, z + dz)) best = Math.min(best, Math.hypot(dx, dz));
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
    for (let t = 1; t <= 14 && !hit; t++) hit = land(Math.round(x + ux! * t), Math.round(z + uz! * t));
    if (!hit) free++;
  }
  return free / 4;
};
const foam: [number, number, number][] = [];
for (let z = 0; z < N; z++) {
  for (let x = 0; x < N; x++) {
    if (x < rim(z) || x >= N - rim(z) || land(x, z)) continue;
    for (let y = 0; y <= 2; y++) v.set(x, y, z, "d");
    let tone = sea[z >> 1]![x >> 1]!;
    if (tone === "." || tone === "f") tone = "s";
    const [d, open] = [gap(x, z), exposed(x, z)];
    const edge = (0.8 + open * 3.4) * (0.35 + 1.3 * fbm(x * 0.3, z * 0.3, 21, 3));
    if (d <= edge) {
      const [t, lace] = [d / edge, noise(x * 0.7, z * 0.7, 29)];
      if (t < 0.5 || d < 1.5) tone = lace > 0.2 ? "f" : "e";
      else tone = lace > 0.25 + 0.45 * t ? "e" : lace > 0.15 ? "a" : tone;
    }
    v.set(x, 3, z, tone);
    const clump = noise(x * 0.45, z * 0.45, 23);
    if (open >= 0.5 && d < 1.5 && clump > 0.5) foam.push([x, 4, z]);
    if (open >= 0.75 && d < 1.2 && clump > 0.68) foam.push([x, 5, z]);
  }
}
for (const [x, y, z] of foam) v.set(x, y, z, "f");

const CX = 23;
const inDisc = (x: number, z: number, r: number) => (x + 0.5 - CX) ** 2 + (z + 0.5 - CX) ** 2 <= r * r;
const disc = (y: number, r: number, ink: string) => {
  for (let z = CX - 13; z < CX + 13; z++)
    for (let x = CX - 13; x < CX + 13; x++) if (inDisc(x, z, r)) v.set(x, y, z, ink);
};
const radius = (y: number) => (y <= 21 ? 9 : y <= 29 ? 8.3 : 7.75);

disc(12, 10.2, "n");
disc(13, 10.2, "n");
for (let y = 14; y <= 37; y++) disc(y, radius(y), y <= 21 ? "R" : y <= 29 ? "C" : "R");
disc(38, radius(37), "C");
disc(39, radius(37), "C");
for (let k = 0; k < 16; k++) {
  const angle = (k * Math.PI) / 8 + Math.PI / 16;
  const [ux, uz] = [Math.cos(angle), Math.sin(angle)];
  for (let t = 6.4; t <= 10.6; t += 0.25) {
    const [x, z] = [Math.floor(CX + ux * t), Math.floor(CX + uz * t)];
    if (!inDisc(x, z, radius(37))) {
      if (t <= 9.6) v.set(x, 39, z, "i");
      if (t <= 8.6) v.set(x, 38, z, "i");
    }
  }
}
disc(40, 11, "i");
disc(41, 11, "i");

const posts = [
  ".........#..#.........",
  "......#........#......",
  "......................",
  "...#..............#...",
  "......................",
  "......................",
  ".#..................#.",
  "......................",
  "......................",
  "#....................#",
  "......................",
  "......................",
  "#....................#",
  "......................",
  "......................",
  ".#..................#.",
  "......................",
  "......................",
  "...#..............#...",
  "......................",
  "......#........#......",
  ".........#..#.........",
];
const topRail = [
  "........######........",
  ".....####....####.....",
  "....##..........##....",
  "...##............##...",
  "..##..............##..",
  ".##................##.",
  ".#..................#.",
  ".#..................#.",
  "##..................##",
  "#....................#",
  "#....................#",
  "#....................#",
  "#....................#",
  "##..................##",
  ".#..................#.",
  ".#..................#.",
  ".##................##.",
  "..##..............##..",
  "...##............##...",
  "....##..........##....",
  ".....####....####.....",
  "........######........",
];
for (let y = 42; y <= 44; y++) v.layer(y).sprite(12, 12, posts, { "#": "i" });
v.layer(45).sprite(12, 12, topRail, { "#": "i" });

disc(42, 7, "i");
disc(43, 7, "i");
const glass = [
  "....iLLLLi....",
  "...LLLLLLLL...",
  "..LLLLLLLLLL..",
  ".LLLLLLLLLLLL.",
  "iLLLLLLLLLLLLi",
  "LLLLLLLLLLLLLL",
  "LLLLLLLLLLLLLL",
  "LLLLLLLLLLLLLL",
  "LLLLLLLLLLLLLL",
  "iLLLLLLLLLLLLi",
  ".LLLLLLLLLLLL.",
  "..LLLLLLLLLL..",
  "...LLLLLLLL...",
  "....iLLLLi....",
];
const bar = [
  "....iiiiii....",
  "...iLLLLLLi...",
  "..iLLLLLLLLi..",
  ".iLLLLLLLLLLi.",
  "iLLLLLLLLLLLLi",
  "iLLLLLLLLLLLLi",
  "iLLLLLLLLLLLLi",
  "iLLLLLLLLLLLLi",
  "iLLLLLLLLLLLLi",
  "iLLLLLLLLLLLLi",
  ".iLLLLLLLLLLi.",
  "..iLLLLLLLLi..",
  "...iLLLLLLi...",
  "....iiiiii....",
];
for (let y = 44; y <= 51; y++) v.layer(y).sprite(16, 16, y === 48 ? bar : glass, key);
disc(52, 7.6, "i");
disc(53, 8.2, "i");
[7.4, 6.8, 6.0, 5.0, 3.6, 2.1].forEach((r, at) => {
  disc(54 + at, r, "R");
});
disc(60, 1.6, "i");
disc(61, 1.6, "i");
v.set(22, 62, 22, "i").set(22, 63, 22, "i");

const surface = (u: number, y: number, side: string): [number, number, number, number] => {
  const [dx, dz] = side === "front" ? [0, 1] : side === "back" ? [0, -1] : side === "left" ? [-1, 0] : [1, 0];
  let [x, z] = dz !== 0 ? [u, CX] : [CX, u];
  while (["R", "C"].includes(v.get(x + dx, y, z + dz) ?? "")) [x, z] = [x + dx, z + dz];
  return [x, z, -dx, -dz];
};
const carve = (u: number, y: number, side: string, ink: string) => {
  const [x, z, ix, iz] = surface(u, y, side);
  v.set(x, y, z, null).set(x + ix, y, z + iz, ink);
};
for (let y = 14; y <= 19; y++) for (let u = 21; u <= 24; u++) carve(u, y, "front", "b");
carve(22, 20, "front", "b");
carve(23, 20, "front", "b");
for (const y of [15, 18])
  for (const u of [21, 22]) {
    const [x, z, ix, iz] = surface(u, y, "front");
    v.set(x + ix, y, z + iz, "i");
  }
const slit = (u: number, y: number, side: string) => {
  for (let h = y; h < y + 4; h++) for (const w of [u, u + 1]) carve(w, h, side, "i");
};
slit(22, 17, "left");
slit(22, 23, "back");
slit(22, 27, "right");
slit(22, 32, "front");

v.box(33, 12, 20, 12, 8, 12, "C");
for (let y = 19; y <= 26; y++) {
  const [back, front] = [18 + (y - 19), 33 - (y - 19)];
  const slate = y === 26 ? "i" : y % 2 === 0 ? "n" : "k";
  for (let x = 32; x <= 45; x++) {
    for (let z = back; z <= front; z++) {
      const skin = z <= back + 1 || z >= front - 1;
      if (skin) v.set(x, y, z, slate);
      else if (x >= 33 && x <= 44) v.set(x, y, z, "C");
    }
  }
}
v.box(40, 20, 24, 3, 10, 4, "r").box(39, 29, 23, 5, 1, 6, "n");
v.set(40, 30, 25, "R").set(42, 30, 26, "R");
for (let y = 12; y <= 17; y++) for (let x = 35; x <= 37; x++) v.set(x, y, 31, null).set(x, y, 30, "b");
for (let y = 14; y <= 16; y++) {
  for (let x = 39; x <= 42; x++) v.set(x, y, 31, null).set(x, y, 30, "i");
  for (let z = 24; z <= 27; z++) v.set(44, y, z, null).set(43, y, z, "i");
  v.set(38, y, 32, "R").set(43, y, 32, "R").set(45, y, 23, "R").set(45, y, 28, "R");
}
v.box(39, 13, 32, 4, 1, 1, "r").box(45, 13, 24, 1, 1, 4, "r");
for (let y = 12; y <= 15; y++) {
  const ink = y === 13 ? "i" : "b";
  v.set(40, y, 16, ink).set(40, y, 17, ink).set(40, y, 18, ink).set(39, y, 17, ink).set(41, y, 17, ink);
}

for (let z = 33; z <= 36; z++) for (let x = 20; x <= 25; x++) v.set(x, 11, z, "n");
v.box(18, 10, 37, 10, 1, 1, "l").box(18, 9, 38, 10, 1, 1, "l").box(18, 8, 39, 10, 1, 1, "l");
for (let z = 40; z <= 45; z++) {
  for (let x = 16; x <= 29; x++) {
    for (let y = 4; y <= 7; y++) {
      const joint = y === 5 || (y < 5 ? (x + z) % 4 === 0 : (x + z) % 4 === 2);
      if (y === 7) v.set(x, y, z, (x - 16) % 5 === 4 || (z - 40) % 3 === 2 ? "r" : "l");
      else if (v.bare(x, y, z)) v.set(x, y, z, joint ? "n" : "r");
    }
  }
}
v.box(18, 8, 41, 2, 3, 2, "i");

for (let z = 34; z <= 37; z++) for (let x = 34; x <= 43; x++) v.set(x, 11, z, "b");
for (const [x, z] of [
  [34, 34],
  [37, 34],
  [40, 34],
  [35, 36],
  [38, 36],
  [41, 36],
])
  v.box(x!, 12, z!, 2, 1, 2, "g");
v.box(33, 12, 38, 12, 1, 1, "n");

const hull = [
  "...RR...",
  "..RRRR..",
  ".RRRRRR.",
  ".RRRRRR.",
  "RRRRRRRR",
  "RRRRRRRR",
  "RRRRRRRR",
  "RRRRRRRR",
  "RRRRRRRR",
  "RRRRRRRR",
  "RRRRRRRR",
  "RRRRRRRR",
  ".RRRRRR.",
  ".RRRRRR.",
  "..RRRR..",
  "...RR...",
];
const BX = 6;
v.layer(2).sprite(BX, 43, hull, key);
for (const y of [3, 4, 5]) {
  v.layer(y).sprite(BX, 43, hull, key);
  v.layer(y).each((x, z, ink) => {
    if (ink !== "R" || x < BX || x > BX + 7 || z < 43 || z > 58) return undefined;
    const rim = [
      [1, 0],
      [-1, 0],
      [0, 1],
      [0, -1],
    ].some(([dx, dz]) => hull[z - 43 + dz!]?.[x - BX + dx!] !== "R");
    return rim ? (y === 5 ? "C" : "R") : y === 3 ? "b" : null;
  });
}
v.box(BX + 1, 4, 47, 6, 1, 1, "b").box(BX + 1, 4, 53, 6, 1, 1, "b");
v.box(BX + 2, 5, 45, 1, 1, 11, "b").box(BX + 2, 5, 55, 2, 1, 2, "b");
v.set(BX + 3, 6, 43, "R")
  .set(BX + 4, 6, 43, "R")
  .set(BX + 3, 6, 58, "R")
  .set(BX + 4, 6, 58, "R");

const swell = (x0: number, x1: number, z0: number, caps: number[]) => {
  for (let x = x0; x <= x1; x++) {
    const z = z0 - Math.floor((x - x0) / 2);
    if (["d", "m", "s"].includes(v.get(x, 3, z) ?? "")) v.set(x, 3, z, caps.includes(x) ? "e" : "a");
  }
};
swell(36, 51, 53, [44, 45]);
swell(32, 59, 61, [41, 50, 51]);
swell(52, 63, 41, []);

const topAt = (x: number, z: number) => {
  let y = N - 1;
  while (y > 0 && v.get(x, y, z) === null) y--;
  return y;
};
const lichen = [
  [7, 8, 1.6],
  [14, 6, 1.2],
  [5, 12, 1.5],
  [19, 9, 1],
  [50, 10, 1.5],
  [53, 13, 1.1],
  [3, 20, 1.1],
  [9, 31, 1.4],
  [44, 15, 1],
  [12, 15, 1.2],
  [2, 29, 1],
];
for (const [cx, cz, r] of lichen) {
  for (let z = Math.floor(cz! - r!); z <= Math.ceil(cz! + r!); z++)
    for (let x = Math.floor(cx! - r!); x <= Math.ceil(cx! + r!); x++) {
      const y = topAt(x, z);
      const stone = ["r", "l", "n"].includes(v.get(x, y, z) ?? "");
      if ((x - cx!) ** 2 + (z - cz!) ** 2 <= r! * r! && stone) v.set(x, y, z, "g");
    }
}
v.set(49, 24, 9, "n").set(50, 24, 9, "f").set(51, 24, 9, "f").set(51, 25, 9, "n").set(52, 25, 9, "f");

v.save(import.meta.dir, "4x");
