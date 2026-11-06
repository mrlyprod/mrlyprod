import { Volume } from "../sculpt.ts";

const colors = {
  glaze: "#5f9db3",
  band: "#3f7187",
  clay: "#a08873",
  broth: "#b8702a",
  deep: "#9a5a20",
  sheen: "#d18f3e",
  noodle: "#f2d27c",
  strand: "#d9ad52",
  white: "#fbf6ea",
  yolk: "#f9ac2c",
  jam: "#ee8212",
  leaf: "#5a9a3c",
  shade: "#3d7632",
  stem: "#c0db98",
  pink: "#ee7fa0",
  wood: "#86452f",
};
const v = new Volume(32, colors);
const disc = (y: number, d: number, ink: string | null) => {
  v.layer(y).ellipse(15.5, 15.5, d / 2 - 0.25, d / 2 - 0.25, ink);
};

const widths = [16, 16, 20, 22, 24, 26, 28, 28, 30, 30, 30, 32, 32, 32];
const coats = ["clay", "clay", "glaze", "glaze", "glaze", "glaze", "glaze", "glaze", "band", "glaze", "glaze", "band"];
widths.forEach((d, y) => {
  disc(y, d, coats[y] ?? (y === 12 ? "band" : "glaze"));
});
disc(12, 28, null);
disc(13, 28, null);

const key: Record<string, string> = {
  o: "broth",
  d: "deep",
  s: "sheen",
  m: "strand",
  n: "noodle",
  w: "white",
  p: "pink",
  g: "leaf",
  G: "shade",
  l: "stem",
  y: "yolk",
  Y: "jam",
};
const top = [
  "................................",
  "................................",
  "............dddddddd............",
  "..........dddnnnmmmddd..........",
  "........dsdomnmnmnnnoddd........",
  ".......ddoonmnmnmnmnmoodd.......",
  "......dooomnmnmnmnmnmnossd......",
  ".....doggomnmnmnmnmnmnoosdd.....",
  "....dogggommnmnmnmnmnmnooodd....",
  "....doggggmmnmnmnmnmnmnooood....",
  "...gggGGGllmnmnmnmnmnmnoossdd...",
  "...gggggglllnmnmnmnmnmnoooood...",
  "..ddoggggllllmnmnmnmnooooooodd..",
  "..ddooGggllllmnmnmnonooooooodd..",
  "..ddogggglllonmnmnmnonoooooodd..",
  "..dggggggllomnmnmnmnonoooooosd..",
  "..ddgggwwoowwnmnmnmnonoooooosd..",
  "..ddoowwwwwwwwmnmnmnonoooooodd..",
  "..ddowwwppppwwwnmnmnonoooooodd..",
  "..dsowwpwwwwpwwmnmnmnooooooodd..",
  "...soowpwppwpwnmnmnmnoooooood...",
  "...ddowpwpwwpwnmnmnmnmnoooodd...",
  "....dwwpwppppwwmnmnmnmnmoood....",
  "....dwwwpppwwwwmnmnmnmnmoodd....",
  ".....dwwwwwwwwmnmnmnmnmoosd.....",
  "......dwwoowwnmnmnmnmnmoss......",
  ".......ddooomnmnmnmnnnodd.......",
  "........dssooomnnnoooddd........",
  "..........dddddddddddd..........",
  "............dddddddd............",
  "................................",
  "................................",
];
const lift = [
  "................................",
  "................................",
  "............00000000............",
  "..........000111000000..........",
  "........0000010101110000........",
  ".......000010101010100000.......",
  "......00000101010101010000......",
  ".....0022001010101010100000.....",
  "....002210001010101010100000....",
  "....002221001010101010100000....",
  "...43311111010101010101000000...",
  "...43332211110101010101000000...",
  "..0002222221101010101000000000..",
  "..0000122221101010101000000000..",
  "..0002222211010101010100000000..",
  "..0433221110010101010100000000..",
  "..0032111001110101010100000000..",
  "..0000111111110101010100000000..",
  "..0001111111111101010100000000..",
  "..0001111111111010101000000000..",
  "...00011111111101010100000000...",
  "...00011111111101010101000000...",
  "....011111111110101010100000....",
  "....011111111110101010100000....",
  ".....0111111110101010100000.....",
  "......01100111010101010000......",
  ".......000000101010111000.......",
  "........0000000111000000........",
  "..........000000000000..........",
  "............00000000............",
  "................................",
  "................................",
];
top.forEach((row, z) => {
  [...row].forEach((glyph, x) => {
    if (glyph === ".") return;
    const crown = 11 + Number(lift[z]![x]);
    for (let y = 11; y <= crown; y++) v.set(x, y, z, key[glyph === "g" && y < crown ? "G" : glyph]!);
  });
});

v.layer(12).sprite(
  17,
  12,
  [
    "..........",
    ".wwww.....",
    ".wwwwww...",
    ".wwwwwww..",
    ".wwwwwwww.",
    "..wwwwwww.",
    "..wwwwwww.",
    "...wwwwww.",
    "....wwwww.",
    "..........",
  ],
  key,
);
v.layer(13).sprite(
  17,
  12,
  [
    ".wwww.....",
    "wwwwwww...",
    "wwwyyywww.",
    "wwyYYYyww.",
    "wwyYYYYyww",
    ".wyYYYYyww",
    ".wwyYYYyww",
    "..wwyyyyww",
    "..wwwwwwww",
    "....wwwww.",
  ],
  key,
);

v.layer(13).rect(12, 8, 5, 1, "noodle").rect(17, 7, 4, 1, "noodle");
v.layer(13).rect(14, 24, 5, 1, "noodle").rect(19, 25, 3, 1, "noodle");

v.box(0, 14, 6, 31, 1, 1, "wood").box(1, 14, 9, 31, 1, 1, "wood");
v.set(25, 14, 6, "noodle").set(27, 14, 6, "noodle").set(26, 14, 9, "noodle").set(28, 14, 9, "noodle");

v.save(import.meta.dir, "4x");
