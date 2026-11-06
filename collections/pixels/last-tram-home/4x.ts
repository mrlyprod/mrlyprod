import { Canvas, save } from "../paint.ts";

const colors = {
  a: "#090c18",
  A: "#0d1224",
  B: "#111830",
  b: "#151c38",
  c: "#1c2746",
  D: "#233257",
  d: "#2b3d66",
  e: "#3e5687",
  f: "#6485b1",
  F: "#a2bcd8",
  M: "#173b47",
  n: "#1e4a56",
  N: "#2a6570",
  g: "#331c2b",
  Q: "#4c2530",
  G: "#66312b",
  h: "#934728",
  H: "#b35b29",
  i: "#cf6f2b",
  I: "#ec9336",
  j: "#f9bd52",
  J: "#fddc8e",
  k: "#fff2c8",
  m: "#6a1b38",
  L: "#a32544",
  l: "#cf2f4c",
  o: "#f47d84",
};

const key = Object.fromEntries(Object.keys(colors).map((name) => [name, name]));
const c = new Canvas(256, 256, colors, "b");

function art(x: number, y: number, rows: string[]): void {
  const wide = rows[0]!.length;
  rows.forEach((row, at) => {
    if (row.length !== wide) throw new Error(`row ${at} at ${x},${y} is ${row.length} wide, not ${wide}`);
  });
  c.sprite(x, y, rows, key);
}

const crown = (x: number) => 104 - Math.floor((x + 50) / 6);
const gutter = (x: number) => 112 - Math.floor((x + 58) / 8);
const wtop = (x: number) => 116 - Math.floor((x + 66) / 10);
const skirt = (x: number) => 160 + Math.floor((x + 62) / 9);
const ground = (x: number) => 168 + Math.floor((x + 51) / 7);

const cloud: [number, number][] = [
  [0, 21],
  [8, 22],
  [20, 23],
  [34, 22],
  [44, 20],
  [52, 19],
  [66, 20],
  [78, 18],
  [88, 15],
  [100, 14],
  [114, 15],
  [124, 13],
  [136, 10],
  [150, 9],
  [166, 10],
  [178, 8],
  [192, 5],
  [210, 4],
  [228, 5],
  [240, 3],
];
cloud.forEach(([x, y], at) => {
  c.rect(x, 0, (cloud[at + 1]?.[0] ?? 256) - x, y, "B");
});
c.rect(26, 26, 30, 1, "B")
  .rect(62, 23, 22, 1, "B")
  .rect(104, 18, 16, 1, "B")
  .rect(140, 13, 24, 1, "B")
  .rect(196, 8, 26, 1, "B");

const houses: [number, number, number][] = [
  [0, 13, 78],
  [14, 25, 75],
  [26, 33, 77],
  [34, 51, 71],
  [52, 63, 72],
  [64, 71, 68],
  [72, 89, 66],
  [90, 103, 62],
  [104, 111, 64],
  [112, 131, 57],
  [132, 167, 49],
  [168, 207, 41],
  [208, 255, 29],
];
for (const [x0, x1, y] of houses)
  c.poly(
    [
      [x0, y],
      [x1 + 1, y - 1],
      [x1 + 1, 256],
      [x0, 256],
    ],
    "a",
  );
c.poly(
  [
    [36, 71],
    [42, 65],
    [44, 65],
    [50, 71],
  ],
  "a",
);
c.poly(
  [
    [112, 57],
    [120, 49],
    [122, 49],
    [130, 57],
  ],
  "a",
);
c.rect(22, 71, 3, 4, "a").rect(57, 65, 4, 7, "a").rect(56, 64, 6, 1, "a").rect(97, 55, 4, 7, "a");
c.rect(66, 65, 4, 3, "a")
  .rect(80, 60, 4, 6, "a")
  .rect(79, 59, 6, 1, "a")
  .rect(126, 48, 4, 6, "a")
  .rect(125, 47, 6, 1, "a");
c.rect(144, 39, 7, 10, "a").rect(142, 38, 11, 2, "a").rect(145, 35, 2, 3, "a");
c.rect(196, 29, 8, 12, "a").rect(194, 28, 12, 2, "a").rect(197, 25, 2, 3, "a").rect(201, 25, 2, 3, "a");
c.rect(216, 14, 9, 15, "a").rect(214, 13, 13, 2, "a").rect(217, 10, 2, 3, "a").rect(222, 10, 2, 3, "a");
c.poly(
  [
    [171, 41],
    [181, 31],
    [183, 31],
    [193, 41],
  ],
  "a",
);
c.rect(236, 22, 11, 7, "a").rect(235, 21, 13, 1, "a");
c.rect(158, 34, 1, 15, "a").rect(155, 36, 7, 1, "a").rect(156, 40, 5, 1, "a");
for (const x of [214, 228, 242])
  c.rect(x, 44, 8, 12, "A")
    .rect(x + 4, 44, 1, 12, "a")
    .rect(x, 49, 8, 1, "a");
for (const x of [176, 192])
  c.rect(x, 54, 8, 10, "A")
    .rect(x + 4, 54, 1, 10, "a")
    .rect(x, 58, 8, 1, "a");
for (const x of [140, 154]) c.rect(x, 60, 7, 9, "A").rect(x + 3, 60, 1, 9, "a");
c.rect(208, 32, 48, 1, "A").rect(168, 44, 40, 1, "A").rect(132, 52, 36, 1, "A");
c.rect(180, 174, 76, 8, "A");

c.rect(232, 0, 1, 182, "d").rect(233, 0, 1, 182, "D").rect(234, 0, 2, 182, "c");
c.rect(232, 108, 2, 72, "Q").rect(232, 120, 2, 56, "G").rect(232, 140, 2, 28, "h");
c.rect(230, 176, 8, 6, "c").rect(230, 176, 3, 6, "Q");
c.line(231, 18, 213, 0, "c").line(231, 30, 222, 21, "c");
c.rect(229, 16, 3, 4, "D");

for (let x = 0; x < 161; x++) c.px(x, 56 - Math.floor((x + 8) / 3), x < 104 ? "e" : "d");
c.line(77, 83, 37, 43, "d").line(78, 83, 58, 63, "D");
art(71, 81, ["..eeeeee..", ".dDDDDDDd.", "dDccccccDd"]);
c.px(36, 42, "F").px(35, 42, "f").px(36, 41, "f").px(34, 43, "e");

const roof = ["F", "f", "e", "d", "d", "D", "D", "D", "c", "c", "c", "c", "c", "c", "c"];
for (let x = 0; x < 124; x++) {
  for (let y = crown(x); y < gutter(x); y++) c.px(x, y, roof[y - crown(x)]!);
  c.px(x, gutter(x) - 1, "c");
  c.px(x, gutter(x), "b").px(x, gutter(x) + 1, "e");
  c.rect(x, gutter(x) + 2, 1, wtop(x) - gutter(x) - 2, "d");
  c.px(x, wtop(x) - 1, "D");
  c.rect(x, wtop(x), 1, 136 - wtop(x), "c");
  c.px(x, 136, "f").px(x, 137, "e").px(x, 138, "d").px(x, 139, "D");
  c.rect(x, 140, 1, skirt(x) - 140, "n");
  c.rect(x, 140, 1, 2, "N");
  c.rect(x, skirt(x) - 7, 1, 3, "M");
  c.rect(x, skirt(x) - 4, 1, 4, "c");
  c.rect(x, skirt(x), 1, ground(x) - skirt(x), "a");
}
const darker: Record<string, string> = { F: "f", f: "e", e: "d", d: "D", D: "c", c: "b" };
for (const x of [6, 22, 38, 54, 70, 86, 102, 118])
  for (let y = crown(x) + 2; y < gutter(x); y++) c.px(x, y, darker[c.get(x, y) ?? ""] ?? "c");
art(94, 76, [".FFFFFFFF.", "feddddddDc", "feDcDcDcDc", "fecccccccc"]);
art(14, 90, [".ffffff.", "edddddDc", "edcDcDcc"]);

const panes: [number, number][] = [
  [40, 63],
  [16, 35],
  [0, 11],
];
for (const [x0, x1] of panes)
  for (let x = x0; x <= x1; x++) {
    const top = wtop(x) + (x === x0 || x === x1 ? 1 : 0);
    c.rect(x, top, 1, 136 - top, "j");
    c.rect(x, top, 1, 3, "k");
    c.px(x, top + 3, "J");
    c.px(x, wtop(x) + 7, "i");
    c.rect(x, 124, 1, 6, "I");
    c.rect(x, 130, 1, 6, "i");
  }
for (const x of [12, 36, 64])
  c.rect(x, wtop(x), 1, 136 - wtop(x), "D").rect(x + 3, wtop(x + 3), 1, 136 - wtop(x + 3), "b");

c.rect(43, 114, 8, 9, "I").rect(43, 114, 8, 1, "i").rect(50, 114, 1, 9, "i");
c.rect(54, 113, 8, 10, "I").rect(54, 113, 8, 1, "i").rect(61, 113, 1, 10, "i");
c.rect(51, 109, 1, 21, "h").rect(52, 109, 1, 21, "i");
c.rect(46, 108, 4, 2, "k").rect(57, 107, 4, 2, "k");
for (const [x, y] of [
  [45, 113],
  [58, 112],
] as [number, number][])
  art(x - 1, y, [".h.", ".h.", "hhh", "h.h", "hhh"]);
const seat = [
  "..iiiiii..",
  ".ihhhhhhi.",
  "ihhhhhhhhG",
  "hhhhhhhhhG",
  "hhhhhhhhGG",
  "hhGGGGGGGG",
  "hGGGGGGGGQ",
  "GGGGGGGGQQ",
];
art(41, 128, seat);
art(53, 128, seat);

c.rect(18, 115, 7, 8, "I").rect(18, 115, 7, 1, "i").rect(24, 115, 1, 8, "i");
c.rect(27, 114, 7, 9, "I").rect(27, 114, 7, 1, "i").rect(33, 114, 1, 9, "i");
c.rect(25, 111, 1, 19, "h");
c.rect(20, 111, 3, 2, "k").rect(29, 110, 3, 2, "k");
art(20, 115, [".h.", "hhh", "h.h", "hhh"]);
art(29, 114, [".h.", "hhh", "h.h", "hhh"]);
const bench = ["..iiii..", ".ihhhhi.", "ihhhhhhG", "hhhhhhhG", "hhhhhhGG", "hGGGGGGG", "hGGGGGGQ", "GGGGGGQQ"];
art(17, 128, bench);
art(27, 128, bench);

c.rect(1, 116, 4, 7, "I").rect(1, 116, 4, 1, "i").rect(7, 115, 4, 8, "I").rect(7, 115, 4, 1, "i");
c.rect(5, 112, 1, 18, "h");
c.rect(2, 112, 2, 2, "k").rect(8, 112, 2, 2, "k");
art(0, 128, ["iiii.", "hhhhi", "hhhhG", "hhhhG", "hhhGG", "GGGGG", "GGGGQ", "GGGQQ"]);
art(6, 128, [".iiii.", "ihhhhi", "hhhhhG", "hhhhhG", "hhhhGG", "hGGGGG", "hGGGGQ", "GGGGQQ"]);

for (let x = 72; x <= 115; x++) {
  c.rect(x, wtop(x), 1, 164 - wtop(x), "k");
  c.rect(x, 164, 1, 4, "J");
  c.rect(x, 168, 1, 10, "j");
  c.rect(x, 178, 1, 2, "J");
  c.rect(x, 180, 1, ground(x) - 180, "I");
  c.rect(x, ground(x) - 2, 1, 2, "i");
}
for (let x = 72; x <= 77; x++) c.rect(x, wtop(x) + 4, 1, 160 - wtop(x), x === 72 ? "I" : x === 77 ? "j" : "J");
for (let x = 110; x <= 115; x++) c.rect(x, wtop(x) + 4, 1, 160 - wtop(x), x === 115 ? "I" : x === 110 ? "j" : "J");
c.rect(73, 130, 4, 1, "j").rect(73, 150, 4, 1, "j").rect(111, 128, 4, 1, "j").rect(111, 150, 4, 1, "j");
c.rect(80, 108, 2, 56, "j").rect(80, 108, 1, 56, "J");
c.rect(78, 164, 32, 1, "k").rect(72, 168, 44, 1, "I");
for (let y = 104; y < 186; y++) c.px(68, y, "Q").px(69, y, "G").px(70, y, "h").px(71, y, "h");
for (let y = 99; y < 192; y++) c.px(116, y, "h").px(117, y, "h").px(118, y, "G").px(119, y, "Q");

const nose = [
  76, 76, 76, 76, 76, 76, 76, 76, 76, 76, 76, 76, 76, 76, 76, 76, 77, 77, 77, 77, 77, 77, 77, 77, 78, 78, 78, 78, 78,
  78, 79, 79, 79, 79, 79, 80, 80, 80, 80, 81, 81, 81, 82, 82, 82, 83, 83, 84, 84, 85, 85, 86, 87, 88, 89, 91,
];
nose.forEach((top, at) => {
  const x = 124 + at;
  for (let y = top; y < 90; y++) c.px(x, y, roof[y - top]!);
  c.px(x, 90, "b").px(x, 91, x > 177 ? "d" : "e");
  c.rect(x, 92, 1, 94, x > 177 ? "b" : x > 175 ? "c" : "d");
  c.rect(x, 186, 1, 6, "a");
});
c.px(179, 90, "b");
art(134, 93, [
  "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
  "bIJJJJJJJJJIJJJJJJJJJJJJJJJJJJJJIb",
  "bIJJJJJJJJJIJJJJJJJJJJJJJJJJJJJJIb",
  "bIjjjjjjjjjIjjjjjjjjjjjjjjjjjjjjIb",
  "bIjjjjjjjjjIjjjjjjjjjjjjjjjjjjjjIb",
  "bIjjjjjjjjjIjjjjjjjjjjjjjjjjjjjjIb",
  "bIIIIIIIIIIiIIIIIIIIIIIIIIIIIIIIIb",
  "biiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiib",
  "bhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhb",
  "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
]);
c.rect(133, 92, 36, 1, "D").rect(133, 103, 36, 1, "c");

c.rect(126, 104, 52, 32, "b");
c.rect(127, 105, 50, 1, "D");
for (let row = 0; row < 28; row++)
  for (let col = 0; col < 22; col++) {
    const [u, v] = [col + 3 * row, col + 2 * row];
    c.px(128 + col, 106 + row, u < 26 ? "h" : v < 32 ? "G" : v < 42 ? "Q" : v < 54 ? "g" : "b");
    const warm = (row < 2 && col >= 2 && col < 17) || (row < 4 && col >= 5 && col < 13);
    c.px(154 + col, 106 + row, warm ? "h" : v < 22 ? "G" : v < 30 ? "Q" : v < 44 ? "g" : v > 62 ? "A" : "b");
  }
c.rect(150, 104, 4, 32, "c").rect(150, 104, 1, 32, "D");
c.rect(132, 108, 5, 2, "I").rect(131, 108, 1, 2, "i").rect(137, 108, 1, 2, "i");
c.line(172, 133, 158, 119, "d").line(173, 132, 161, 120, "D");
c.line(146, 133, 134, 121, "b");
art(128, 126, [
  "..aaaaaa........",
  ".aaaaaaaa.......",
  "aaaaaaaaaa..aa..",
  "aaaaaaaaaa.aaaa.",
  "aaaaaaaaaaaaaaaa",
  "aaaaaaaaaaaaaaaa",
  "aaaaaaaaaaaaaaaa",
  "aaaaaaaaaaaaaaaa",
]);
c.rect(131, 124, 1, 3, "a").rect(130, 123, 3, 1, "a");

for (let x = 124; x < 180; x++) {
  const turn = x > 175;
  c.px(x, 136, x < 128 ? "f" : turn ? "c" : "d").px(x, 137, x < 128 ? "e" : turn ? "c" : "d");
  c.px(x, 138, x < 128 ? "d" : turn ? "b" : "D").px(x, 139, x < 128 ? "D" : turn ? "b" : "c");
}
for (let y = 140; y < 180; y++) {
  const edge = y < 172 ? 156 : y < 174 ? 155 : y < 176 ? 154 : 124;
  for (let x = 124; x < 176; x++)
    c.px(x, y, x < edge - 1 ? (y < 142 && x < 150 ? "N" : y > 171 ? "M" : "n") : x < edge + 1 ? "M" : "c");
}
c.rect(162, 150, 10, 1, "b").rect(162, 153, 10, 1, "b").rect(162, 156, 10, 1, "b");
art(142, 146, [
  "......GGhhhhGG......",
  "....GGhhiiiihhGG....",
  "...GhhiiIIIIiihhG...",
  "..GhiiIIjjjjIIiihG..",
  ".GhiIIjjJJJJjjIIihG.",
  ".GhiIjjJJJJJJjjIihG.",
  "GhiIjjJJkkkkJJjjIihG",
  "GhiIjJJkkkkkkJJjIihG",
  "hiIjjJkkkkkkkkJjjIih",
  "hiIjJJkkkkkkkkJJjIih",
  "hiIjJJkkkkkkkkJJjIih",
  "hiIjjJkkkkkkkkJjjIih",
  "GhiIjJJkkkkkkJJjIihG",
  "GhiIjjJJkkkkJJjjIihG",
  ".GhiIjjJJJJJJjjIihG.",
  ".GhiIIjjJJJJjjIIihG.",
  "..GhiiIIjjjjIIiihG..",
  "...GhhiiIIIIiihhG...",
  "....GGhhiiiihhGG....",
  "......GGhhhhGG......",
]);
c.rect(124, 180, 54, 1, "e").rect(124, 181, 54, 3, "d").rect(124, 184, 54, 1, "D").rect(124, 185, 54, 1, "b");
c.rect(174, 180, 4, 5, "c")
  .rect(145, 180, 14, 1, "I")
  .rect(143, 180, 2, 1, "h")
  .rect(159, 180, 2, 1, "h")
  .rect(147, 181, 10, 1, "i");
c.rect(128, 187, 46, 1, "B").rect(130, 189, 42, 1, "A");

const un = (row: string) => row.replace(/(\d+)(.)/g, (_, n: string, g: string) => g.repeat(Number(n)));
const span = (y: number, ink: string, ...runs: [number, number][]) => {
  for (const [x0, x1] of runs) c.rect(x0, y, x1 - x0 + 1, 1, ink);
};

for (let x = 0; x < 256; x++) c.rect(x, x < 124 ? ground(x) : 192, 1, 256, "a");
for (let x = 0; x < 124; x++) {
  const lit = x > 71 && x < 116;
  c.px(x, ground(x), lit ? "I" : "c").px(x, ground(x) + 1, lit ? "i" : "A");
}
for (let x = 124; x < 256; x++) {
  const y = 192 + Math.floor((x - 112) / 7);
  c.px(x, y, x < 144 ? "d" : x < 176 ? "I" : x < 200 ? "h" : x < 226 ? "d" : "D");
  c.px(x, y + 1, x < 144 ? "A" : x < 176 ? "h" : x < 200 ? "Q" : "A");
}
for (let x = 180; x < 256; x++) c.px(x, 188 + Math.floor((x - 180) / 8), x < 200 ? "G" : "c");

art(0, 182, ["36A20.", "56.", "8.40A8.", "56.", "30A26.", "56.", "56.", "12.28A16."].map(un));
art(126, 198, ["48A4.", "4.48A", "52.", "20A12.18A2.", "52.", "4.16A16.12A4.", "52.", "52.", "10.14A28."].map(un));
art(127, 220, ["2.22g28.", "6.22Q24.", "52.", "2.18g32.", "8.14g30.", "52.", "4.10g38."].map(un));
art(138, 238, ["24G4.", "4.22G2.", "28.", "2.18Q8.", "28.", "6.12g10."].map(un));
art(232, 184, [
  "GG",
  "GG",
  "GG",
  "..",
  "QQ",
  "QQ",
  "QQ",
  "..",
  "..",
  "gg",
  "gg",
  "..",
  "..",
  "..",
  "AA",
  "AA",
  "..",
  "AA",
]);

art(
  0,
  200,
  [
    "12h4.",
    "11i1h4.",
    "13i3.",
    "1.2i8I3i2.",
    "2.2i9I2i1.",
    "2i10I2i2.",
    "1i10I2i3.",
    "12i4.",
    "8.5i3.",
    "4j5.5j2.",
    "1.12j3.",
    "2.2j8J2j2.",
    "2j9J2j3.",
    "1j3J4k3J1j4.",
    "7j9.",
    "3i4.6i3.",
    "1.11i4.",
    "6i10.",
    "6.6h4.",
    "9h7.",
    "16.",
    "5.6h5.",
    "16.",
    "3.5G8.",
  ].map(un),
);
art(
  12,
  204,
  [
    "4.20h4.",
    "4.1h19i1h3.",
    "5.20i3.",
    "5.3i14I4i2.",
    "6.2i15I3i2.",
    "4.2i16I3i3.",
    "2.3i15I3i5.",
    "3.3i14I3i5.",
    "4.19i5.",
    "16.8i4.",
    "4.5j7.9j3.",
    "5.20j3.",
    "6.3j12J4j3.",
    "5.2j14J3j4.",
    "3.2j4J7k4J2j6.",
    "2.4j12J3j7.",
    "3.12j13.",
    "4.5i7.8i4.",
    "5.18i5.",
    "4.10i14.",
    "12.9h7.",
    "5.15h8.",
    "28.",
    "10.10h8.",
    "7.6h15.",
    "28.",
    "9.7G12.",
  ].map(un),
);
art(
  36,
  208,
  [
    "4.24h4.",
    "4.1h23i1h3.",
    "5.24i3.",
    "5.3i18I4i2.",
    "6.2i19I3i2.",
    "5.2i20I2i3.",
    "3.3i19I3i4.",
    "3.2i19I3i5.",
    "4.3i17I3i5.",
    "5.23i4.",
    "20.9i3.",
    "4.6j10.9j3.",
    "5.24j3.",
    "6.3j16J4j3.",
    "5.2j18J3j4.",
    "3.2j5J9k5J2j6.",
    "2.3j17J3j7.",
    "3.4j13J5j7.",
    "4.14j14.",
    "5.6i8.9i4.",
    "5.22i5.",
    "6.21i5.",
    "4.12i16.",
    "14.11h7.",
    "5.19h8.",
    "7.9h16.",
    "32.",
    "12.12h8.",
    "8.8h16.",
    "32.",
    "32.",
    "10.9G13.",
  ].map(un),
);
art(
  146,
  208,
  [
    "4.4I4.",
    "3.1I4j1I3.",
    "2.1I2j2J2j1I2.",
    "1.1I2j4J2j1I1.",
    "1I2j2J2k2J2j1I",
    "1I1j2J4k2J1j1I",
    "1.1j2J4k2J1j1I",
    "1.1I1j2J2k2J1j1I1.",
    "2.1I2j2J2j1I2.",
    "3.1I1j2J1j1I3.",
    "2.1i1I3j1I1i3.",
    "12.",
    "3.1i1I2j1I1i3.",
    "4.1i2I1i4.",
    "5.2i5.",
    "12.",
    "4.1h2i1h4.",
    "5.2h5.",
    "12.",
    "12.",
    "4.3h5.",
    "12.",
    "5.2G5.",
  ].map(un),
);

art(64, 187, [
  "........Jkk.................................................",
  "......jJkkkkkkkkkk..........................................",
  "......jJkkkkkkkkkkkkkkkkk...................................",
  ".......Jkkkkkkkkkkkkkkkkkkkkkkkk............................",
  "..IjjJJkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkk.....................",
  "....jJJkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkk..............",
  "........JkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkJj........",
  ".......jJkkkkkkkkggggggggggkkkkkkkkkkkkkkkkkkkkkkkkJJj......",
  ".......jJkkkkkkkkkggggggggggkkkkkkkkkkkkkkkkkkkkkkkkJjjII...",
  ".....jJkkkkkkkkkkkkkggggggggkkkkkkkkkkkkkkkkkkkkkkkkJJjI....",
  ".....jJkkkkkkkkkkkkkkgggggggkkkkkkkkkkkkkkkkkkkkkkJj........",
  ".........jJkkkkkkkkkkggggggggkkkkkkkkkkkkkkkkkkkkkJj........",
  ".........jJkkkkkkkkkkkgggggggkkkkkkkkkkkkkkkkkkkkkkkJj......",
  ".......jJkkkkkkkkkkkkkgggggggkkkkkkkkkkkkkkkkkkkkkkkJj......",
  "......jJkkkkkkkkkkkkkgggggggkkkkkkkkkkkkkkkkkkkkkJj.........",
  ".iIIjjJJkkkkkkkkkkkkkggggggkkkkkkkkkkkkkkkkkkkkkkkJj........",
  "...IjjJkkkkkkkkkkkkkkkggggggkkkkkkkkkkkkkkkkkkkkkkkkJj......",
  ".......jJkkkkkkkkkkkkkgggggggkkkkkkkkkkgggggggggkkkJjI......",
  "......IjJkkkkkkkkkkkkkkgggggggkkkkkkkggggggggggkkkJj........",
  "............IjJkkkkkkkkgggggggkkkkkkggggggggkkkkkJj.........",
  ".....iIIIi.......jJkkkkggggggggkkkkgggggggkkkkkkkJjj........",
  "..............IjJkkkkkkkgggggggggggggggggkkkkkkkkJJjI.......",
  ".......IjJkkkkkkkkkkkkkggggggggggggggggggkkkkkkkkkJjjI......",
  "......IjJJkkkkkkkkkkkkggggggggggggggggggggkkkkkkkkJjI.......",
  "......IjjJJkkkkkkkkkkkggggggggggggJJkkkkkkkkkkkkkJj.........",
  ".......IjJJkkkkkkkkkkgggggggggggggggggggggkkkkJj............",
  ".......iIjjJJkkkkkkkggggggggggggggggggggggJ.....iIIIi.......",
  "......iIjjJJJkkkkkggggggggggggggggggggggggkkkkkJJjjI........",
  ".....iIIjjjJJJkkkggggggggggggggggggggggggkkkkkkkJJjjIi......",
  ".....iIIjjjJJJkkkkkkkJJggggggggggggggggggkkkkkkJJJjjII......",
  ".......iIIjjJJJkkkgggggggggggggggggggggggkkkkkJJJjjIi.......",
  ".......iIIjjjJJJkkkgggggggggggggggggggggggkkkJJJjjjII.......",
  "................IjJJkkggggggggggggggggggggkkJJJjjjIIi.......",
  ".......HiiiiiH.......IjjggggggggggggggggggJJJJjjjIIi........",
  "............iIIjjjJJJJJJJJJJJJJgggggggggggJJJjjjIIi.........",
  ".........iIIjjjJJJJJJJJJJggggggggggggggggJJJjjjIIi..........",
  "........iIIjjjJJJJJJJJJJggggggggggggggggJJJjjjIIii..........",
  "........iIIjjjJJJJJJJJJgggggggggggggggggjI...HiiiiH.........",
  ".......HiiIIjjjJJJJJJJggggggggggggggggggjjIIIii.............",
  "........HiiIIjjjjjjjjjjggggggggggIIjjjjjjjjjjjIIIiiH........",
  ".........iIIjjjjjjjjjjjjggggggggggggggjjjjjjjIIIiiH.........",
  "..........iIIjjjjjjjmmmmmggggggggggggmmmmmjjjjIIii..........",
  "................IjjjmmmmmmggggggggggmmmmmmjjI...............",
  "........hHHHHh...mmmmmmmmmmmmmmmmmmmmmmmm.....hHHHHh........",
  "............HiiIImmllllllllllllllllllllmmjjjjIIiH...........",
  ".........HiiIIIjjjmmllllllllllllllllllllmmjjIIIiiHH.........",
  "........hHiiiIIIjjjmmllllllllllllllllllllmmIIIiiiHh.........",
  ".........hHHHHh......mmmmmmmmmmmllllllllllmmIIiiHHh.........",
  "...........hHHiiiIIIIImmmllllllllllllllllmmmiiiHHh..........",
  "..........hHHiiiIIIIIIIImmmlllllllllllllmmmiiiHHhh..........",
  "............hHHiiiIIIIIIIImmmmlllllllmmmmIIiiHHh............",
  "..............hHHiiiIIIIIIIImmmmmmmmmmIIIiHHh...............",
  ".................................GhhhHHHHHHhhG..............",
  "..........GhhHHHiiiiIIIIIIIIIIIIIIIIiiiiHHHh................",
  "........GhhHHHiiiiIIIIIIIIIIIIIIIIIIIIiiiiHHHhhG............",
  ".......GhhHHHiiiiIIIIIIIIIIIIIIIiiH.....hHHHHHhh............",
  ".........GhhHHHiiiiiIIIIIIIIIIIIIIIiiiiHHHhhG...............",
  "...........GhhHHHiiiiHG.....................................",
  "................GhhHHHiiiiiiiiiiiiiiiiiiiiiiiHHHhhG.........",
  "..............GhhHHHiiiiiiIIIIIIIIIIIiiiiiiiiHHhhG..........",
  "........................GhhHHHiiiiiiiiiiiiiHHHh.............",
  "......GhhhhG................................................",
  "............GhhhHHHHHiiiiiiiiiiiiiiiiHHHHhhhG...............",
  "..........GhhhHHHHHHiiiiiiiiiiiiiiiiiiiiHHHHhhhG............",
  "...........GhhhHHHHHHHHHHHHHHHHHHHHHHHHHhhhG................",
  "...............GhhhHHHHHHHHHHHHHHHhhhG......................",
  "............................................................",
  ".............QGGGhhhhHHHHHHHHHHHHHHHHHhhhhGGGQ..............",
  "...........QGGGGhhhhHHHHHHHHHHHHHHHHHHHHhhhhGGGQ............",
]);
art(106, 214, ["...IIIIi...", ".II.....ii.", "I.........H", ".II.....iH.", "...iiiii..."]);
art(75, 236, [".jjjjj.", "j.....I", ".IIIII."]);
art(86, 249, ["....hhhhhhh....", ".hhh.......hhh.", "G.iiiiIIIiiii.G", ".III.......II..", "....IIIIIIi...."]);
art(63, 244, [".hhhhh.", "G.....G", ".GGGGG."]);

art(
  68,
  104,
  [
    "33.1F18.",
    "32.1e1f18.",
    "19.13o20.",
    "15.4o12l4o17.",
    "12.3o19l3o15.",
    "10.2o25l2o13.",
    "8.2o29l2o11.",
    "6.2o33l2o9.",
    "5.1o36l2o8.",
    "4.1o38l2o7.",
    "3.1o37l3L2o6.",
    "2.1o37l5L2o5.",
    "1.1o38l7L1o4.",
    "1.1o38l8L1o3.",
    "1o39l9L1o2.",
    "40l9L1o2.",
    "14l14m12l10L1o1.",
    "11l20m9l10L1o1.",
    "1L9l23m7l11L1o",
    "1.1L8l24m6l12L",
    "2.2L6l25m5l12L",
    "4.2L5l10m8a6m5l12L",
    "7.2L2l9m10a4m5l13L",
    "10.6m3.12a7.2l12L",
    "11.3m5.12a5.2a2.12L",
    "19.12a5.2a5.9L",
    "19.12a5.2a6.3L1.4L",
    "19.12a5.2a7.1L4.2L",
  ].map(un),
);
const body: [number, number, number][] = [
  [28, 19, 30],
  [28, 35, 38],
  [29, 20, 29],
  [29, 34, 39],
  [30, 21, 28],
  [30, 34, 39],
  [31, 19, 30],
  [31, 34, 40],
  [32, 16, 40],
  [33, 15, 41],
  [34, 14, 41],
  [35, 14, 41],
  [36, 14, 40],
  [37, 15, 38],
  [38, 16, 36],
  [39, 16, 35],
  [40, 15, 35],
  [41, 15, 35],
  [42, 15, 35],
  [43, 15, 35],
  [44, 15, 35],
  [45, 15, 35],
  [46, 15, 35],
  [47, 15, 35],
  [48, 16, 35],
  [49, 16, 35],
  [50, 16, 35],
  [51, 16, 35],
  [52, 16, 35],
  [53, 16, 35],
  [54, 9, 35],
  [55, 7, 35],
  [56, 6, 35],
  [57, 6, 35],
  [58, 6, 35],
  [59, 6, 35],
  [60, 6, 35],
  [61, 6, 35],
  [62, 6, 35],
  [63, 6, 35],
  [64, 7, 35],
  [65, 9, 35],
  [66, 13, 35],
  [67, 12, 35],
  [68, 12, 36],
  [69, 11, 36],
  [70, 10, 36],
  [71, 9, 36],
  [72, 9, 36],
  [73, 16, 23],
  [73, 28, 35],
  [74, 16, 23],
  [74, 28, 35],
  [75, 16, 23],
  [75, 28, 35],
  [76, 16, 23],
  [76, 28, 35],
  [77, 16, 23],
  [77, 28, 35],
  [78, 16, 23],
  [78, 28, 35],
  [79, 16, 23],
  [79, 28, 35],
  [80, 16, 22],
  [80, 28, 35],
  [81, 16, 22],
  [81, 28, 35],
  [82, 16, 22],
  [82, 29, 35],
  [83, 16, 22],
  [83, 29, 35],
  [84, 16, 21],
  [84, 29, 35],
  [85, 16, 21],
  [85, 29, 35],
  [86, 14, 21],
  [86, 30, 36],
  [87, 13, 21],
  [87, 30, 36],
  [88, 12, 22],
  [88, 30, 36],
  [89, 12, 22],
  [89, 30, 36],
  [90, 31, 36],
  [91, 32, 37],
  [92, 32, 37],
  [93, 32, 38],
  [94, 32, 39],
  [95, 32, 41],
  [96, 33, 43],
  [97, 33, 44],
  [98, 35, 44],
  [99, 37, 43],
];
for (const [r, x0, x1] of body) span(104 + r, "a", [68 + x0, 68 + x1]);
c.line(104, 137, 83, 158, "A").line(103, 137, 82, 158, "A");
c.rect(75, 162, 8, 1, "A").px(78, 165, "B").rect(94, 164, 1, 12, "A").rect(87, 154, 15, 1, "A");
const rim: [number, number, string][] = [
  [87, 130, "G"],
  [87, 131, "h"],
  [87, 132, "h"],
  [88, 133, "G"],
  [87, 135, "H"],
  [84, 136, "I"],
  [85, 136, "I"],
  [86, 136, "i"],
  [87, 136, "H"],
  [83, 137, "I"],
  [84, 137, "i"],
  [85, 137, "H"],
  [86, 137, "h"],
  [87, 137, "G"],
  [82, 138, "I"],
  [83, 138, "i"],
  [84, 138, "h"],
  [85, 138, "G"],
  [82, 139, "i"],
  [83, 139, "H"],
  [84, 139, "G"],
  [82, 140, "i"],
  [83, 140, "h"],
  [83, 141, "H"],
  [84, 141, "G"],
  [84, 142, "h"],
  [84, 143, "G"],
  [98, 135, "h"],
  [99, 136, "h"],
  [100, 136, "H"],
  [101, 136, "h"],
  [103, 132, "G"],
  [102, 133, "h"],
  [102, 134, "G"],
  [106, 132, "i"],
  [107, 133, "I"],
  [107, 134, "I"],
  [108, 135, "I"],
  [108, 136, "I"],
  [109, 137, "i"],
  [109, 138, "i"],
  [109, 139, "H"],
  [108, 140, "h"],
  [107, 141, "G"],
  [105, 132, "h"],
  [106, 133, "H"],
  [106, 134, "h"],
  [107, 135, "H"],
  [107, 136, "H"],
  [106, 135, "G"],
  [106, 136, "G"],
  [108, 137, "H"],
  [108, 138, "h"],
  [107, 137, "G"],
  [108, 139, "G"],
  [82, 158, "G"],
  [81, 158, "h"],
  [80, 158, "H"],
  [79, 158, "H"],
  [78, 158, "i"],
  [77, 158, "i"],
  [76, 159, "I"],
  [75, 159, "i"],
  [74, 160, "i"],
  [74, 161, "H"],
  [74, 162, "H"],
  [74, 163, "h"],
  [74, 164, "h"],
  [74, 165, "G"],
  [78, 159, "G"],
  [77, 159, "h"],
  [76, 160, "h"],
  [75, 160, "H"],
  [75, 161, "h"],
  [75, 162, "G"],
  [81, 170, "h"],
  [80, 171, "H"],
  [80, 172, "h"],
  [79, 173, "h"],
  [78, 174, "G"],
  [79, 128, "L"],
  [80, 128, "H"],
  [81, 128, "H"],
  [82, 127, "L"],
  [83, 127, "H"],
  [82, 126, "L"],
  [83, 126, "L"],
  [84, 126, "L"],
  [85, 126, "H"],
  [86, 126, "i"],
  [87, 126, "H"],
  [99, 125, "L"],
  [100, 125, "L"],
  [99, 126, "H"],
  [100, 126, "i"],
  [101, 126, "H"],
  [108, 128, "l"],
  [109, 128, "l"],
  [110, 128, "l"],
  [111, 129, "l"],
  [112, 130, "l"],
  [113, 131, "o"],
];
for (const [x, y, ink] of rim) c.px(x, y, ink);

c.line(97, 108, 88, 110, "L").line(88, 110, 78, 114, "L").line(78, 114, 70, 120, "L");
c.line(98, 108, 95, 112, "L").line(95, 112, 90, 119, "L");
c.line(100, 107, 101, 122, "L");
c.rect(88, 120, 3, 1, "l")
  .px(89, 121, "l")
  .px(78, 122, "l")
  .px(78, 123, "l")
  .px(79, 123, "l")
  .px(102, 123, "l")
  .px(102, 124, "l");
c.px(113, 132, "F")
  .px(113, 134, "f")
  .px(118, 133, "F")
  .px(118, 136, "f")
  .px(79, 129, "F")
  .px(79, 131, "f")
  .px(70, 124, "F");
art(84, 101, ["F.F", ".f."]);
art(96, 99, ["F..F", ".ff."]);
art(109, 106, ["F.F", ".f."]);
for (const [x, y] of [
  [9, 92],
  [27, 89],
  [41, 86],
  [66, 82],
  [88, 78],
  [112, 75],
  [131, 73],
  [150, 75],
  [164, 79],
] as [number, number][])
  c.px(x, y, "F").px(x + 1, y - 2, "f");

for (const x of [14, 38, 66]) c.rect(x, 143, 1, skirt(x) - 151, "M");
for (let x = 50; x < 68; x++) c.rect(x, skirt(x) - 3, 1, 3, x < 59 ? "g" : "Q");
for (let x = 120; x < 124; x++) c.rect(x, skirt(x) - 3, 1, 3, "Q");
c.rect(124, 177, 10, 3, "Q").rect(134, 177, 10, 3, "g");
for (let x = 0; x < 68; x++) c.px(x, 154 + Math.floor(x / 15), "N").px(x, 155 + Math.floor(x / 15), "M");
for (let x = 120; x < 124; x++) c.px(x, 162, "N").px(x, 163, "M");
c.line(42, 121, 46, 113, "J").line(44, 122, 47, 116, "J").line(18, 122, 21, 116, "J").line(55, 111, 57, 107, "k");
art(18, 171, ["3.12A3.", "1.2A12B2A1.", "2A14B2A", "18A", "18A", "18A"].map(un));
art(42, 174, ["3.14A3.", "1.2A14B2A1.", "2A16B2A", "20A", "20A", "20A"].map(un));
c.rect(0, 170, 18, 2, "A").rect(36, 173, 6, 2, "A");

const ring = (x: number, y: number, rows: string[], ink: string) => c.sprite(x, y, rows, { r: ink });
const small = [".rrrrr.", "r.....r", ".rrrrr."];
const mid = ["...rrrrr...", ".rr.....rr.", "r.........r", ".rr.....rr.", "...rrrrr..."];
const wide = ["....rrrrrrr....", ".rrr.......rrr.", "r.............r", ".rrr.......rr..", "....rrrrrr....."];
ring(182, 206, mid, "B");
ring(184, 207, small, "b");
ring(212, 238, wide, "B");
ring(148, 248, mid, "B");
ring(236, 222, small, "b");
ring(30, 246, wide, "c");
ring(34, 247, small, "d");
ring(6, 236, small, "B");
ring(168, 226, small, "B");
ring(116, 236, mid, "B");
c.px(187, 204, "d").px(219, 236, "c").px(37, 244, "f").px(239, 220, "d");
c.px(116, 238, "G").px(117, 237, "G").px(118, 237, "Q").px(117, 239, "Q");

art(160, 244, ["30A"].map(un));
art(196, 250, ["40A"].map(un));
art(200, 226, ["24A"].map(un));
art(130, 252, ["22A"].map(un));
art(0, 250, ["18A"].map(un));

const far: Record<string, string> = { a: "A", A: "B", B: "b", b: "c", c: "D", D: "d", d: "e", M: "n", n: "N" };
const dim: Record<string, string> = {
  a: "B",
  A: "b",
  B: "c",
  b: "D",
  c: "d",
  D: "d",
  d: "e",
  M: "N",
  n: "N",
  N: "e",
  e: "f",
};
const lit: Record<string, string> = {
  a: "c",
  A: "D",
  B: "D",
  b: "d",
  c: "e",
  D: "e",
  d: "e",
  M: "e",
  n: "e",
  N: "f",
  e: "f",
  f: "F",
};
const hot: Record<string, string> = { a: "g", A: "Q", B: "Q", b: "G", c: "G", D: "h", d: "h" };
const pane: Record<string, string> = { j: "J", I: "j", J: "k", h: "i", i: "I", c: "d", D: "d" };
const maps = [dim, lit, hot, pane, far];
const drops: [number, number, number, number][] = [
  [26, 0, 24, 1],
  [40, 6, 12, 0],
  [48, 14, 8, 0],
  [98, 2, 24, 1],
  [112, 8, 12, 0],
  [120, 16, 8, 0],
  [170, 0, 24, 1],
  [184, 6, 12, 0],
  [192, 14, 8, 0],
  [240, 2, 24, 1],
  [254, 8, 12, 0],
  [62, 34, 24, 1],
  [76, 40, 12, 0],
  [84, 48, 8, 0],
  [134, 36, 24, 1],
  [148, 42, 12, 0],
  [156, 50, 8, 0],
  [206, 34, 24, 1],
  [220, 40, 12, 0],
  [228, 48, 8, 0],
  [8, 38, 20, 1],
  [20, 44, 10, 0],
  [190, 70, 24, 1],
  [204, 76, 12, 0],
  [212, 84, 8, 0],
  [246, 66, 24, 1],
  [16, 70, 14, 0],
  [222, 98, 24, 1],
  [236, 104, 12, 0],
  [244, 112, 8, 0],
  [190, 104, 12, 0],
  [188, 128, 16, 2],
  [208, 140, 24, 2],
  [224, 124, 12, 2],
  [248, 156, 16, 2],
  [200, 172, 12, 2],
  [252, 132, 24, 1],
  [244, 188, 12, 0],
  [108, 84, 12, 0],
  [36, 144, 16, 0],
  [60, 152, 12, 0],
  [172, 148, 16, 0],
  [56, 106, 16, 3],
  [28, 114, 12, 3],
  [164, 216, 16, 0],
  [220, 212, 24, 1],
  [192, 232, 12, 0],
  [248, 228, 16, 0],
  [140, 236, 12, 0],
  [20, 232, 16, 0],
  [52, 244, 12, 0],
  [14, 8, 8, 4],
  [58, 10, 6, 4],
  [84, 6, 8, 4],
  [122, 9, 6, 4],
  [166, 4, 8, 4],
  [190, 12, 6, 4],
  [232, 8, 8, 4],
  [4, 26, 6, 4],
  [30, 28, 8, 4],
  [74, 24, 6, 4],
  [100, 30, 8, 4],
  [136, 26, 6, 4],
  [184, 24, 8, 4],
  [210, 28, 6, 4],
  [240, 22, 8, 4],
  [20, 46, 8, 4],
  [62, 50, 6, 4],
  [102, 44, 8, 4],
  [146, 46, 6, 4],
  [198, 44, 8, 4],
  [236, 48, 6, 4],
  [8, 62, 6, 4],
  [40, 64, 8, 4],
  [164, 58, 8, 4],
  [186, 62, 6, 4],
  [216, 60, 8, 4],
  [240, 66, 6, 4],
  [196, 82, 8, 4],
  [214, 86, 6, 4],
  [242, 80, 8, 4],
  [186, 104, 6, 4],
  [206, 108, 8, 4],
  [250, 102, 6, 4],
  [196, 126, 6, 4],
  [216, 122, 8, 4],
  [240, 130, 6, 4],
  [188, 150, 8, 4],
  [222, 154, 6, 4],
  [244, 148, 8, 4],
  [196, 170, 6, 4],
  [212, 166, 8, 4],
  [150, 200, 6, 4],
  [180, 204, 8, 4],
  [206, 198, 6, 4],
  [236, 202, 8, 4],
  [130, 222, 6, 4],
  [200, 220, 8, 4],
  [230, 224, 6, 4],
  [110, 246, 6, 4],
  [186, 244, 8, 4],
  [214, 240, 6, 4],
  [244, 246, 8, 4],
  [12, 220, 6, 4],
];
for (const [x, y, n, kind] of drops)
  for (let t = 0; t < n; t++) {
    const [u, v] = [x - Math.floor(t / 2), y + t];
    const head = kind < 2 && n > 10 && t >= n - 4;
    const next = (head ? lit : maps[kind]!)[c.get(u, v) ?? ""];
    if (next) c.px(u, v, next);
  }

save(import.meta.dir, "4x", c);
