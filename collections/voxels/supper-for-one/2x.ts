import { Volume } from "../sculpt.ts";

const colors = {
  glaze: "#5f9db3",
  foot: "#44788c",
  broth: "#b8702a",
  deep: "#9a5a20",
  noodle: "#f2d27c",
  strand: "#d9ad52",
  white: "#fbf6ea",
  yolk: "#f79a1e",
  leaf: "#4c8a36",
  stem: "#a8cc74",
  pink: "#ee7fa0",
  wood: "#86452f",
};
const v = new Volume(16, colors);
const disc = (y: number, d: number, ink: string | null) => {
  v.layer(y).ellipse(7.5, 7.5, d / 2 - 0.25, d / 2 - 0.25, ink);
};

const profile = [8, 10, 12, 14, 14, 16, 16];
profile.forEach((d, y) => {
  disc(y, d, y === 0 || y === 5 ? "foot" : "glaze");
});
disc(6, 14, null);

const key: Record<string, string> = {
  o: "broth",
  d: "deep",
  n: "noodle",
  m: "strand",
  w: "white",
  y: "yolk",
  g: "leaf",
  l: "stem",
  p: "pink",
};
const top = [
  "................",
  ".....donnod.....",
  "....donmnood....",
  "...ggonmnnood...",
  "..gglonmnnoood..",
  ".ggollnnmnooood.",
  ".ggllonnmwwoood.",
  ".ggooonmwwwwood.",
  ".dowwwnmwwyywoo.",
  ".owwppwnmwyywwo.",
  ".owpwpwnmnwwwwo.",
  "..wppwwnmnowwd..",
  "...wwwonnmnod...",
  "....donnmnod....",
  ".....ddoodd.....",
  "................",
];
const rise = [
  "................",
  ".....001100.....",
  "....00101000....",
  "...1101011000...",
  "..111010110000..",
  ".22011110100000.",
  ".21110110110000.",
  ".11000101111000.",
  ".00111101111100.",
  ".01111110111110.",
  ".01111110111110.",
  "..111110000110..",
  "...1110000000...",
  "....00000000....",
  ".....000000.....",
  "................",
];
top.forEach((row, z) => {
  [...row].forEach((glyph, x) => {
    if (glyph === ".") return;
    const lift = Number(rise[z]![x]);
    for (let y = 5; y <= 5 + lift; y++) v.set(x, y, z, key[glyph]!);
  });
});

v.box(0, 7, 2, 16, 1, 1, "wood");
v.box(0, 7, 4, 16, 1, 1, "wood");

v.save(import.meta.dir, "2x");
