import { Canvas, save } from "../paint.ts";

const colors = {
  a: "#0b0e1c",
  b: "#131a33",
  c: "#1d2949",
  d: "#2d3f69",
  e: "#4a638f",
  f: "#8aa7c8",
  n: "#1f4b56",
  g: "#40222c",
  h: "#8b4427",
  i: "#d8792c",
  j: "#f8b94f",
  k: "#fff0c0",
  l: "#d1304d",
  m: "#6c1c39",
  o: "#f47d84",
};

const key = Object.fromEntries(Object.keys(colors).map((name) => [name, name]));
const c = new Canvas(64, 64, colors, "b");

function art(x: number, y: number, rows: string[]): void {
  const wide = rows[0]!.length;
  rows.forEach((row, at) => {
    if (row.length !== wide) throw new Error(`row ${at} at ${x},${y} is ${row.length} wide, not ${wide}`);
  });
  c.sprite(x, y, rows, key);
}

const crown = (x: number) => 26 - Math.floor((x + 12) / 6);
const gutter = (x: number) => 28 - Math.floor((x + 14) / 8);
const wtop = (x: number) => 29 - Math.floor((x + 16) / 10);
const skirt = (x: number) => 40 + Math.floor((x + 15) / 9);
const ground = (x: number) => 42 + Math.floor((x + 14) / 7);

const roofs = [
  19, 19, 19, 19, 19, 19, 19, 19, 18, 18, 18, 18, 18, 18, 18, 18, 17, 17, 16, 16, 16, 16, 16, 15, 15, 15, 16, 16, 14,
  14, 13, 14, 14, 12, 12, 12, 12, 12, 12, 12, 12, 12, 10, 10, 9, 8, 9, 10, 10, 10, 10, 10, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7,
  7, 7,
];
for (let x = 0; x < 64; x++) c.rect(x, roofs[x]!, 1, 64, "a");
c.rect(54, 4, 2, 3, "a").rect(36, 10, 2, 2, "a").rect(21, 14, 1, 2, "a").rect(10, 17, 1, 1, "a");

c.rect(58, 0, 1, 45, "c").rect(58, 30, 1, 15, "g").rect(58, 36, 1, 6, "h").line(57, 4, 53, 0, "c");

for (let x = 0; x < 41; x++) c.px(x, 14 - Math.floor((x + 2) / 3), "e");
c.line(19, 20, 10, 11, "d");
c.px(10, 10, "f");

for (let x = 0; x < 31; x++) {
  c.px(x, crown(x), "e");
  c.rect(x, crown(x) + 1, 1, gutter(x) - crown(x) - 1, "c");
  c.rect(x, gutter(x), 1, wtop(x) - gutter(x), "d");
  c.rect(x, wtop(x), 1, 34 - wtop(x), "c");
  c.px(x, 34, "e");
  c.rect(x, 35, 1, skirt(x) - 35, "n");
  c.px(x, skirt(x) - 1, "c");
  c.rect(x, skirt(x), 1, ground(x) - skirt(x), "a");
}

const panes: [number, number][] = [
  [10, 15],
  [4, 8],
  [0, 2],
];
for (const [x0, x1] of panes)
  for (let x = x0; x <= x1; x++) {
    c.px(x, wtop(x), "k");
    c.rect(x, wtop(x) + 1, 1, 31 - wtop(x), "j");
    c.px(x, 32, "i");
    c.px(x, 33, "h");
  }
c.sprite(10, 32, ["hhihhi", "hhihhi"], key);
c.sprite(4, 32, ["hhihh", "hhihh"], key);
c.rect(13, 28, 1, 4, "i").rect(6, 29, 1, 3, "i");

for (let x = 18; x <= 28; x++) {
  c.rect(x, wtop(x), 1, 45 - wtop(x), "k");
  c.px(x, 45, "j");
  c.rect(x, 46, 1, ground(x) - 46, "i");
}
c.rect(18, 27, 1, 18, "j").rect(28, 26, 1, 19, "j").rect(29, 26, 1, 20, "h").rect(17, 27, 1, 19, "h");

c.sprite(
  31,
  19,
  [
    "deeee.........",
    "ccccceeeee....",
    "cccccccccceee.",
    "ccccccccccccce",
    "ddhiiiiiiihddc",
    "dddddddddddddc",
    "dhhhhhchhhhgdc",
    "dghhhgcghhggdc",
    "dgggggcggggbdc",
    "dgggggcgggbbdc",
    "dgggggcggbbbdc",
    "dggggbcgbbbbdc",
    "dgggbbcbbbbbdc",
    "dggbbbcbbbbbdc",
    "dbbbbbcbbbbbdc",
    "eddddddddddddc",
    "nnnnnnnncccccb",
    "nnnnnhhhcccccb",
    "nnnnhjjjhccccb",
    "nnnnhjkjhccccb",
    "nnnnhjjjhccccb",
    "nnnnnhhhcccccb",
    "nnnnnnnncccccb",
    "nnnnnnncccccbb",
    "cccccccccccbbb",
    "edddddiiiddddc",
    "aaaaaaaaaaaaaa",
    "aaaaaaaaaaaaaa",
    "aaaaaaaaaaaaaa",
  ],
  key,
);

for (let x = 0; x < 64; x++) c.rect(x, x < 31 ? ground(x) : 48, 1, 64, "a");
for (let x = 0; x < 31; x++) c.px(x, ground(x), x > 17 && x < 29 ? "i" : "c");
for (let x = 31; x < 64; x++) c.px(x, 48 + Math.floor((x - 28) / 7), x < 36 ? "d" : x < 44 ? "i" : x < 50 ? "h" : "d");
for (let x = 45; x < 64; x++) c.px(x, 47 + Math.floor((x - 45) / 8), "c");

art(0, 47, [
  "..................kkk...........",
  ".................kkkkkkkkkkk....",
  ".................kkkhhhkkkkkk...",
  "hh................kkkhhkkkkkkj..",
  "ii..hhhh..........kkkhhkkhhhkj..",
  "iii..iiii.hhhhh.jjkkgggggggkk...",
  "ii..iiii...iiiii.jjkkggggggkkj..",
  ".....iiii.iiiii....jgggggjjjjjj.",
  "jjj.jjjj...iiiii..ijjgggggjjji..",
  "jj..jjkjj.jjjjj..iijjjgggjjj....",
  ".....jjj..jjjkjj..iijmmmmmjjii..",
  "ii..........jjjj...iijllllllmi..",
  "i...iiii..jjj.....hiimlllmiii...",
  ".....ii.....iiii....hiiimmiih...",
  "h.........iii.....hhh..hiiih....",
  ".....hh..................hhh....",
  "............hh......hhh.........",
]);

art(35, 51, ["..i..", ".jkj.", ".ijj.", "..j..", ".i...", "..i..", ".....", "..h..", ".....", ".h..."]);
art(32, 54, ["gg........", ".......ggg", "..........", "ggg.......", "......gg..", "..........", "....hhh..."]);
art(44, 56, ["bbbb.........", ".............", ".......bbbbbb", "............."]);
art(53, 61, ["bbbbb"]);
art(58, 46, ["g", "g", ".", "g"]);

art(17, 26, [
  "......e......",
  "....ooooo....",
  "..oollllloo..",
  ".olllllllllo.",
  "olllllllllllo",
  "llmmmmmmmllll",
  ".....aaa.a.ll",
  ".....aaa.a...",
  "......a..a...",
  "....aaaaaa...",
  "....aaaaa....",
  "....aaaaa....",
  "....aaaaa....",
  "....aaaaa....",
  "..aaaaaaa....",
  "..aaaaaaa....",
  "...aaaaaa....",
  "...aaaaaa....",
  "....aa.aa....",
  "....aa.aa....",
  "....aa.aa....",
  "....aa.aa....",
  "...aaa.aa....",
  "........aa...",
  "........aaa..",
]);
c.px(7, 22, "f").px(38, 19, "f");
c.rect(23, 19, 3, 1, "e").rect(3, 23, 2, 1, "e");

const dim: Record<string, string> = { a: "c", b: "c", c: "d", d: "e", n: "d", e: "f" };
const lit: Record<string, string> = { a: "d", b: "d", c: "e", d: "e", n: "e", e: "f" };
const hot: Record<string, string> = { a: "g", b: "g", c: "h", d: "h" };
const maps = [dim, lit, hot];
const drops: [number, number, number, number][] = [
  [6, 1, 5, 0],
  [23, 0, 4, 0],
  [35, 5, 5, 1],
  [48, 1, 6, 1],
  [62, 2, 4, 0],
  [14, 6, 3, 0],
  [53, 14, 5, 0],
  [49, 25, 4, 0],
  [55, 33, 5, 2],
  [62, 39, 4, 2],
  [10, 36, 3, 0],
  [46, 58, 3, 0],
  [61, 55, 4, 0],
];
for (const [x, y, n, kind] of drops)
  for (let t = 0; t < n; t++) {
    const [u, v] = [x - Math.floor(t / 2), y + t];
    const next = maps[kind]![c.get(u, v) ?? ""];
    if (next) c.px(u, v, next);
  }

save(import.meta.dir, "1x", c);
