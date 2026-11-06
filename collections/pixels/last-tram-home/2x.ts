import { Canvas, save } from "../paint.ts";

const colors = {
  a: "#0a0d1a",
  A: "#0f1427",
  b: "#141b35",
  c: "#1c2847",
  d: "#2a3b64",
  e: "#40588a",
  f: "#6384b0",
  F: "#9db8d4",
  n: "#1e4a56",
  N: "#2b6670",
  g: "#3a1f2d",
  G: "#63302b",
  h: "#934728",
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
const c = new Canvas(128, 128, colors, "b");

function art(x: number, y: number, rows: string[]): void {
  const wide = rows[0]!.length;
  rows.forEach((row, at) => {
    if (row.length !== wide) throw new Error(`row ${at} at ${x},${y} is ${row.length} wide, not ${wide}`);
  });
  c.sprite(x, y, rows, key);
}

const crown = (x: number) => 52 - Math.floor((x + 25) / 6);
const gutter = (x: number) => 56 - Math.floor((x + 29) / 8);
const wtop = (x: number) => 58 - Math.floor((x + 33) / 10);
const skirt = (x: number) => 80 + Math.floor((x + 31) / 9);
const ground = (x: number) => 84 + Math.floor((x + 29) / 7);

const blocks: [number, number][] = [
  [0, 39],
  [7, 37],
  [13, 38],
  [17, 35],
  [26, 36],
  [32, 34],
  [36, 33],
  [45, 31],
  [52, 32],
  [56, 28],
  [66, 24],
  [84, 20],
  [104, 14],
];
blocks.forEach(([x, y], at) => {
  c.rect(x, y, (blocks[at + 1]?.[0] ?? 128) - x, 128, "a");
});
c.rect(11, 35, 1, 2, "a")
  .rect(28, 33, 2, 3, "a")
  .rect(48, 28, 2, 3, "a")
  .rect(40, 30, 2, 3, "a")
  .rect(63, 24, 2, 3, "a");
art(18, 32, ["...aa...", "..aaaa..", ".aaaaaa."]);
art(57, 24, ["...aa...", "..aaaa..", ".aaaaaa.", "aaaaaaaa"]);
c.rect(72, 20, 3, 4, "a").rect(71, 19, 5, 1, "a").rect(98, 15, 4, 5, "a").rect(97, 14, 6, 1, "a");
c.rect(108, 8, 4, 6, "a").rect(107, 7, 6, 1, "a");
art(86, 16, ["....aa....", "...aaaa...", "..aaaaaa..", ".aaaaaaaa."]);
c.rect(107, 22, 4, 6, "A").rect(114, 22, 1, 6, "A").rect(121, 22, 4, 6, "A");
c.rect(88, 27, 4, 5, "A").rect(96, 27, 4, 5, "A");

c.rect(116, 0, 1, 91, "d").rect(117, 0, 1, 91, "c");
c.rect(116, 58, 1, 32, "G").rect(116, 70, 1, 14, "h");
c.line(115, 9, 106, 0, "c");
c.rect(90, 90, 38, 1, "A");

for (let x = 0; x < 81; x++) c.px(x, 28 - Math.floor((x + 4) / 3), x < 52 ? "e" : "d");
c.line(38, 41, 19, 22, "d");
c.rect(36, 41, 5, 1, "d");
c.px(18, 21, "F").px(17, 21, "f");

for (let x = 0; x < 62; x++) {
  c.px(x, crown(x), "f");
  c.rect(x, crown(x) + 1, 1, gutter(x) - crown(x) - 1, "c");
  c.px(x, crown(x) + 1, "d");
  c.px(x, gutter(x), "b");
  c.rect(x, gutter(x) + 1, 1, wtop(x) - gutter(x) - 1, "d");
  c.rect(x, wtop(x), 1, 68 - wtop(x), "c");
  c.px(x, 68, "e").px(x, 69, "d");
  c.rect(x, 70, 1, skirt(x) - 70, "n");
  c.rect(x, skirt(x) - 2, 1, 2, "c");
  c.rect(x, skirt(x), 1, ground(x) - skirt(x), "a");
}

for (let x = 0; x < 62; x++) if (x < 34 || x > 59) c.px(x, 70, "N");
for (let x = 27; x < 34; x++) c.rect(x, skirt(x) - 2, 1, 2, "g");
for (let x = 60; x < 62; x++) c.rect(x, skirt(x) - 2, 1, 2, "g");
c.rect(7, 71, 1, 10, "c").rect(19, 71, 1, 11, "c").rect(33, 71, 1, 13, "c");
art(9, 85, [".AAAAAAA.", "AAAAAAAAA", "AAAAAAAAA", "AAAAAAAAA"]);
art(22, 87, [".AAAAAAA.", "AAAAAAAAA", "AAAAAAAAA", "AAAAAAAAA"]);
art(47, 38, ["fffff", "ccccc"]);
art(7, 45, ["eeee", "cccc"]);
c.px(11, 43, "F").px(28, 42, "F").px(75, 37, "F");

const panes: [number, number][] = [
  [20, 31],
  [8, 17],
  [0, 5],
];
for (const [x0, x1] of panes)
  for (let x = x0; x <= x1; x++) {
    c.rect(x, wtop(x), 1, 2, "k");
    c.px(x, wtop(x) + 2, "J");
    c.rect(x, wtop(x) + 3, 1, 61 - wtop(x), "j");
    c.px(x, 64, "I");
    c.rect(x, 65, 1, 3, "h");
  }
art(20, 56, [".IIII..IIII.", ".IIII..IIII.", ".IIII..IIII.", ".IIII..IIII.", ".IIII..IIII."]);
art(20, 65, ["IhhhIIIhhhII", "hhhhhIhhhhhI", "hGGGhIhGGGhI"]);
c.rect(25, 55, 1, 10, "i");
art(8, 57, [".III..III.", ".III..III.", ".III..III.", ".III..III."]);
art(8, 65, ["IhhIIIhhII", "hhhhIhhhhI", "hGGhIhGGhI"]);
c.rect(12, 56, 1, 9, "i");
art(0, 58, ["II..II", "II..II", "II..II"]);
art(0, 65, ["hIIhhI", "hhIhhh", "GhIhGh"]);

for (let x = 36; x <= 57; x++) {
  const top = x < 47 ? 51 : 50;
  c.rect(x, top, 1, 82 - top, "k");
  c.rect(x, 82, 1, 2, "J");
  c.rect(x, 84, 1, 5, "j");
  c.px(x, 89, "J");
  c.rect(x, 90, 1, ground(x) - 90, "I");
  c.px(x, ground(x) - 1, "i");
}
c.rect(36, 53, 3, 29, "J").rect(36, 53, 1, 29, "I").rect(55, 52, 3, 30, "J").rect(57, 52, 1, 30, "I");
c.rect(38, 53, 1, 29, "j").rect(55, 52, 1, 30, "j");
c.rect(41, 56, 1, 26, "j");
c.rect(34, 52, 2, 42, "h").rect(58, 50, 2, 46, "h");
c.rect(34, 52, 1, 42, "G").rect(59, 50, 1, 46, "G");

art(62, 38, [
  "FFFFFFFFFF..................",
  "ddddddddddFFFFFF............",
  "ddddddddddddddddffff........",
  "ccccccccccccccccddddfff.....",
  "cccccccccccccccccccccccee...",
  "ccccccccccccccccccccccccce..",
  "cccccccccccccccccccccccccce.",
  "bbbbbbbbbbbbbbbbbbbbbbbbbbbd",
  "dddddbbbbbbbbbbbbbbbbbdddddc",
  "dddddbhIIjjjjjjjjjIIhbdddddc",
  "dddddbhIjjJJJJJJJjjIhbdddddc",
  "dddddbhIIjjjjjjjjjIIhbdddddc",
  "dddddbbbbbbbbbbbbbbbbbdddddc",
  "dddddddddddddddddddddddddddc",
  "dbbbbbbbbbbbbbbbbbbbbbbbbbbc",
  "dbhhhhhhhhhhhccGhhhhhhhGGGbc",
  "dbhhIIhhhhhhGccGGhhhhGGGggbc",
  "dbGhhhhhhhGGGccGGGGGGGggggbc",
  "dbGGGGGGGGGGgccGGGGGgggggbbc",
  "dbGGGGGGGGgggccGGGggggggbbbc",
  "dbGGGGGGgggggccGgggggggbbbbc",
  "dbGGGGgggggggccgggggbbbbbbbc",
  "dbGGgggggggggccgggbbbbbbbbbc",
  "dbggggggggggbccgbbbbbbbbbbbc",
  "dbggggggggbbbccbbbbbbbbbbbbc",
  "dbggggggbbbbbccbbbbbbbbbbbbc",
  "dbggggbbbbbbbccbbbbbbbbbbAbc",
  "dbggbbbbbbbbbccbbbbbbbbAAAbc",
  "dbaaaaaabbbbbccbbbbbbAAAAAbc",
  "dbaaaaaaaabbbccbbbbAAAAAAAbc",
  "eedddddddddddddddddddddddddc",
  "ddcccccccccccccccccccccccccb",
  "nnnnnnnnnnnnnnnnccccccccccbb",
  "nnnnnnnnnnnnnnnnccccccccccbb",
  "nnnnnnnnnnnnnnnnccccccccccbb",
  "nnnnnnnnnnnnnnnnccccccccccbb",
  "nnnnnnnnnnnnnnnnccccccccccbb",
  "nnnnnnnnnnnnnnnnccccccccccbb",
  "nnnnnnnnnnnnnnnnccccccccccbb",
  "nnnnnnnnnnnnnnnnccccccccccbb",
  "nnnnnnnnnnnnnnnnccccccccccbb",
  "nnnnnnnnnnnnnnnnccccccccccbb",
  "nnnnnnnnnnnnnnnnccccccccccbb",
  "nnnnnnnnnnnnnnnnccccccccccbb",
  "nnnnnnnnnnnnnnnnccccccccccbb",
  "nnnnnnnnnnnnnnnnccccccccccbb",
  "nnnnnnnnnnnnnnnnccccccccccbb",
  "nnnnnnnnnnnnnnnnccccccccccbb",
  "nnnnnnnnnnnnnnncccccccccccbb",
  "nnnnnnnnnnnnnncccccccccccbbb",
  "cccccccccccccccccccccbbbbbbb",
  "cccccccccccccccccccccbbbbbbb",
  "eeeeeeeeeeehIIIIheeeeeeeeddc",
  "dddddddddddddddddddddddddccb",
  "bbbbbbbbbbbbbbbbbbbbbbbbbbba",
  "aaAAAAAAAAAAAAAAAAAAAAAAaaaa",
  "aaaaaaaaaaaaaaaaaaaaaaaaaaaa",
  "aaaaaaaaaaaaaaaaaaaaaaaaaaaa",
]);
art(71, 73, [
  "..GhhhhG..",
  ".GhIIIIhG.",
  "GhIjJJjIhG",
  "hIjJkkJjIh",
  "hIJkkkkJIh",
  "hIJkkkkJIh",
  "hIjJkkJjIh",
  "GhIjJJjIhG",
  ".GhIIIIhG.",
  "..GhhhhG..",
]);
c.line(86, 66, 80, 60, "d");

for (let x = 0; x < 128; x++) c.rect(x, x < 62 ? ground(x) : 96, 1, 128, "a");
for (let x = 0; x < 62; x++) c.px(x, ground(x), x > 35 && x < 58 ? "I" : "c");
for (let x = 62; x < 128; x++)
  c.px(x, 96 + Math.floor((x - 56) / 7), x < 72 ? "d" : x < 88 ? "I" : x < 100 ? "h" : "d");
for (let x = 90; x < 128; x++) c.px(x, 94 + Math.floor((x - 90) / 8), x < 100 ? "G" : "c");

art(0, 91, [
  "AAAAAAAAAAAAAAAAAA..........",
  "............................",
  "....AAAAAAAAAAAAAAAAAAAAAAAA",
  "............................",
  "AAAAAAAAAAAA................",
]);
art(63, 99, [
  "AAAAAAAAAAAAAAAAAAAAAAAA..",
  "..AAAAAAAAAAAAAAAAAAAAAAAA",
  "..........................",
  "AAAAAAAAAA......AAAAAAAAA.",
  "..........................",
  "..AAAAAAAA........AAAAAA..",
  "..........................",
  "..........................",
  "..........................",
  "..........................",
  "..........................",
  ".ggggggggggg..............",
  "...ggggggggggg............",
  "..........................",
  ".gggggggg.................",
  "..........................",
  "..........................",
  "..........................",
  "..........................",
  "......GGGGGGGGGGGG........",
  "........GGGGGGGGGGG.......",
  "..........................",
  ".......gggggggg...........",
]);
art(116, 92, ["GG", "GG", "..", "gg", "gg", "gg", "..", "..", "gg", "..", "..", "AA", "AA"]);

art(0, 100, [
  "hhhhhh..",
  "iiiiiih.",
  ".iIIIIi.",
  "iIIIIii.",
  "iiii....",
  "....jjj.",
  "jjjjjj..",
  ".jJkkJj.",
  "jjJJj...",
  "jj......",
  "...iiii.",
  "iiiii...",
  "........",
  ".hhhh...",
]);
art(6, 102, [
  "..hhhhhhhhhh..",
  "..iiiiiiiiiii.",
  "..iiIIIIIIIii.",
  "...iIIIIIIIi..",
  ".iiIIIIIIii...",
  "..iiiii.......",
  ".......jjjjj..",
  "..jjjjjjjjjjj.",
  "...jJJkkkJJj..",
  ".jjJJJJJjj....",
  ".jjjj.........",
  "......iiiiii..",
  "..iiiiiiii....",
  "..............",
  "...hhhhhh.....",
  ".hhh..........",
  "..............",
  ".....hhh......",
]);
art(18, 104, [
  "..hhhhhhhhhhhh..",
  "..iiiiiiiiiiiii.",
  "..iiIIIIIIIIIii.",
  "...iIIIIIIIIIi..",
  ".iiIIIIIIIIii...",
  "..iiiiii........",
  "........jjjjjj..",
  "..jjjjjjjjjjjjj.",
  "...jJJJkkkkJJj..",
  ".jjJJJJJJJjj....",
  ".jjjjj..........",
  ".......iiiiiii..",
  "..iiiiiiiiii....",
  "................",
  "...hhhhhhhh.....",
  ".hhhh...........",
  "................",
  ".....hhhh.......",
]);

art(30, 94, [
  ".....jkkkkk.....................",
  ".....Jkkkkkkkkkkkk..............",
  "...ijJkkkkkkkkkkkkkkkkkkk.......",
  "......JkkkggggggkkkkkkkkkkkJj...",
  ".....jkkkkkgggggkkkkkkkkkkkkJjI.",
  ".......jkkkkkggggkkkkkkkkkkJj...",
  "......jkkkkkkggggkkkkkkkkkkkj...",
  "...iIjkkkkkkkgggkkkkkkkkkkkJ....",
  ".....jkkkkkkkggggkkkkgggggkkj...",
  "......Ijkkkkkkgggkkkgggggkkj....",
  "....iII...jkkkggggkggggkkkkJj...",
  "......IjkkkkkkgggggggggkkkkJjI..",
  ".....IjkkkkkkggggggJkkkkkkkj....",
  "......IjkkkkgggggggggggJ..iII...",
  ".....iIjJkkggggggggggggkkkJjI...",
  "......iIjJkkgggggggggggkkJjjI...",
  ".....hiii...IjgggggggggJJjjI....",
  ".......iIjJJJJJJJggggggJjjIi....",
  "......iIjJJJJJggggggggj..hii....",
  "......iIjjJJJgggggggggjjjIIi....",
  ".......iIjjjmmgggggggmmjjIi.....",
  "......hii.mmmmmmmmmmmmm...hh....",
  ".......iIImllllllllllmjjjIi.....",
  "......hiIIjmllllllllllmIIiih....",
  ".............mmmmllllllmIih.....",
  ".......hiIIIImmllllllllmiih.....",
  "........hiIIIIIImmmmmIIih.......",
  "...................GhhhhG.......",
  "......GhiiIIIIIIIIIIIiihhG......",
  "......hiiiIIIIIIIIih..hiiih.....",
  "........hiiih...................",
  "..........GhiiiiIIIIIiiiihG.....",
  "..............Ghiiiiiiihh.......",
  ".......GhhiiiiiiiiiiiihhG.......",
]);
art(53, 104, [".jjj.", "j...I", ".III."]);
art(29, 122, [".hhh.", "G...G", ".GGG."]);

art(72, 104, [
  "..II..",
  ".IjjI.",
  "IjJJjI",
  "IjkkjI",
  ".jJJj.",
  ".IjjI.",
  "..IjI.",
  "..iI..",
  "......",
  "..ii..",
  "..hi..",
  "......",
  "...h..",
]);
art(82, 112, ["AAAAAAAAAAAAAA.................."]);
art(100, 116, ["AAAAAAAAAAAAAAAAAA"]);
art(66, 122, ["AAAAAAAAAA"]);
art(104, 123, ["AAAAAAAAAAAAAA"]);

art(34, 52, [
  "................F.........",
  "........oooooooe..........",
  ".....ooolllllllloo........",
  "...oollllllllllllloo......",
  "..ollllllllllllllllloo....",
  ".ollllllllllllllllllllo...",
  "olllllllllllllllllllllLo..",
  "lllllllllllllllllllllLLLo.",
  "lllllllmmmmmmmlllllllLLLLo",
  "LllllmmmmmmmmmmmlllllLLLLL",
  ".LlllmmmmmmmmmmmmllllLLLLL",
  "...LlmmmmmaaaammmlllLLLLLL",
  ".....mmm.aaaaaa...a..LLLLL",
  ".........aaaaaa...a...L.LL",
  ".........aaaaaa..aaa......",
  "..........aaaa...aaa......",
  "........aaaaaaaaaaaa......",
  ".......aaaaaaaaaaaaa......",
  ".......aaaaaaaaaaaa.......",
  "........aaaaaaaaaa........",
  "........aaaaaaaaaa........",
  "........aaaaaaaaaa........",
  "........aaaaaaaaaa........",
  "........aaaaaaaaaa........",
  "........aaaaaaaaaa........",
  "........aaaaaaaaaa........",
  "........aaaaaaaaaa........",
  "....aaaaaaaaaaaaaa........",
  "...aaaaaaaaaaaaaaa........",
  "...aaaaaaaaaaaaaaa........",
  "...aaaaaaaaaaaaaaa........",
  "...aaaaaaaaaaaaaaa........",
  "....aaaaaaaaaaaaaa........",
  "......aaaaaaaaaaaa........",
  "......aaaaaaaaaaaa........",
  ".....aaaaaaaaaaaaa........",
  "........aaaa..aaaa........",
  "........aaaa..aaaa........",
  "........aaaa..aaaa........",
  "........aaaa..aaaa........",
  "........aaa...aaaa........",
  "........aaa...aaaa........",
  "........aaa....aaa........",
  ".......aaaa....aaa........",
  "......aaaaa....aaa........",
  "................aaa.......",
  "................aaa.......",
  "................aaaa......",
  "................aaaaaa....",
  ".................aaaaa....",
]);

art(90, 103, [".bbb.", "b...b", ".bbb."]);
art(106, 119, [".bbbb.", "b....b", ".bbbb."]);
art(74, 124, [".bbb.", "b...b", ".bbb."]);
art(16, 123, [".ccc.", "c...c", ".ccc."]);
c.px(92, 103, "c").px(108, 119, "c").px(18, 122, "e");

c.px(43, 51, "F").px(54, 53, "F").px(56, 67, "F").px(59, 68, "F").px(39, 66, "F");
c.line(51, 69, 42, 78, "A");
c.rect(38, 81, 4, 1, "A").rect(46, 83, 1, 5, "A");
const rim: [number, number, string][] = [
  [42, 68, "i"],
  [41, 69, "I"],
  [41, 70, "i"],
  [42, 71, "h"],
  [42, 72, "G"],
  [43, 68, "h"],
  [42, 69, "h"],
  [42, 70, "G"],
  [48, 68, "G"],
  [49, 68, "h"],
  [51, 66, "h"],
  [53, 66, "i"],
  [53, 67, "I"],
  [53, 68, "I"],
  [53, 69, "i"],
  [52, 70, "h"],
  [52, 67, "h"],
  [52, 68, "h"],
  [52, 69, "G"],
  [41, 79, "G"],
  [40, 79, "h"],
  [39, 79, "i"],
  [38, 79, "I"],
  [37, 80, "i"],
  [37, 81, "h"],
  [37, 82, "h"],
  [37, 83, "G"],
  [38, 80, "h"],
  [38, 81, "G"],
  [39, 87, "G"],
  [39, 64, "L"],
  [40, 64, "h"],
  [41, 64, "i"],
  [41, 63, "L"],
  [42, 63, "i"],
  [43, 63, "h"],
  [48, 63, "L"],
  [49, 63, "i"],
  [50, 63, "h"],
  [55, 64, "l"],
  [57, 64, "l"],
  [56, 65, "l"],
];
for (const [x, y, ink] of rim) c.px(x, y, ink);

const dim: Record<string, string> = { a: "b", A: "c", b: "c", c: "d", d: "e", n: "d", e: "f" };
const lit: Record<string, string> = { a: "c", A: "d", b: "d", c: "e", d: "e", n: "e", e: "f", f: "F" };
const hot: Record<string, string> = { a: "g", A: "g", b: "G", c: "G", d: "h" };
const pane: Record<string, string> = { j: "J", I: "j", J: "k", h: "i", i: "I", c: "d" };
const maps = [dim, lit, hot, pane];
const drops: [number, number, number, number][] = [
  [13, 0, 12, 1],
  [20, 3, 6, 0],
  [24, 7, 4, 0],
  [49, 1, 12, 1],
  [56, 4, 6, 0],
  [60, 8, 4, 0],
  [85, 0, 12, 1],
  [92, 3, 6, 0],
  [96, 7, 4, 0],
  [120, 1, 12, 1],
  [127, 4, 6, 0],
  [31, 17, 12, 1],
  [38, 20, 6, 0],
  [42, 24, 4, 0],
  [67, 18, 12, 1],
  [74, 21, 6, 0],
  [78, 25, 4, 0],
  [103, 17, 12, 1],
  [110, 20, 6, 0],
  [114, 24, 4, 0],
  [4, 19, 10, 1],
  [10, 22, 5, 0],
  [95, 35, 12, 1],
  [102, 38, 6, 0],
  [106, 42, 4, 0],
  [123, 33, 12, 1],
  [8, 35, 7, 0],
  [111, 49, 12, 1],
  [118, 52, 6, 0],
  [122, 56, 4, 0],
  [95, 52, 6, 0],
  [94, 64, 8, 2],
  [104, 70, 12, 2],
  [112, 62, 6, 2],
  [124, 78, 8, 2],
  [126, 66, 12, 1],
  [100, 86, 6, 2],
  [122, 94, 6, 0],
  [54, 42, 6, 0],
  [28, 53, 8, 3],
  [14, 57, 6, 3],
  [18, 72, 8, 0],
  [30, 76, 6, 0],
  [86, 74, 8, 0],
  [82, 108, 8, 0],
  [110, 106, 12, 1],
  [96, 116, 6, 0],
  [124, 114, 8, 0],
  [70, 118, 6, 0],
  [10, 118, 8, 0],
  [26, 122, 6, 0],
];
for (const [x, y, n, kind] of drops)
  for (let t = 0; t < n; t++) {
    const [u, v] = [x - Math.floor(t / 2), y + t];
    const next = maps[kind]![c.get(u, v) ?? ""];
    if (next) c.px(u, v, next);
  }

save(import.meta.dir, "2x", c);
