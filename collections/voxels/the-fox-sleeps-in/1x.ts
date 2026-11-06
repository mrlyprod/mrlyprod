import { Volume } from "../sculpt.ts";

const v = new Volume(16, {
  rust: "#a8421e",
  brush: "#c55a22",
  coat: "#d9692b",
  fur: "#e87a31",
  cream: "#f6ead6",
  dark: "#45302a",
  moss: "#6c963f",
  shade: "#4a7034",
  leaf: "#e8c547",
});

const top = 2;
const turf = [
  ".....mmmmmm.....",
  "...mmmmmmmmmm...",
  "..mmmdmmmmmmmm..",
  ".mmmddmmmmmmmmm.",
  ".mmmmmmmmmmmmdm.",
  "mmmmmmmmmmmmmddm",
  "mmmmmmmmmmmmmmmm",
  "mdmmmmmmmmmmmmmm",
  "mddmmmmmmmmmmmmm",
  "mmmmmmmmmmmmmmmm",
  "mmmmmmmmmmmmmmmm",
  ".mmmmmmmmmmmmmm.",
  ".mmmmmmmmmmmmdm.",
  "..mmmmmmmmmmmm..",
  "...mmmmddmmmm...",
  ".....mmmmmm.....",
];
v.layer(top).sprite(0, 0, turf, { m: "moss", d: "shade" });
for (let y = top - 1; y >= 0; y--) {
  v.each((x, at, z) => {
    if (at !== y) return undefined;
    const held = [
      [0, 0],
      [1, 0],
      [-1, 0],
      [0, 1],
      [0, -1],
    ].every(([dx, dz]) => v.get(x + dx!, y + 1, z + dz!) !== null);
    return held ? "shade" : undefined;
  });
}

const high = [
  "................",
  "................",
  "......2222......",
  "....23344332....",
  "...2344444432...",
  "...3444334433...",
  "..234433333332..",
  "..244333363336..",
  "..244333334443..",
  "..234433334443..",
  "...34443333333..",
  "...2344432222...",
  "...1222222111...",
  "....12222111....",
  "................",
  "................",
];
const tint = [
  "................",
  "................",
  "......oooo......",
  "....oobbbboo....",
  "...oobbbbbboo...",
  "...obbbrrbboo...",
  "..oobbrbbroooo..",
  "..obborbbooooo..",
  "..obborbbrofoo..",
  "..obbbrbbrofoo..",
  "...obbborkkokk..",
  "...ooooooowww...",
  "...bbbbbbrwww...",
  "....bbbbbrww....",
  "................",
  "................",
];
const coat = { o: "coat", b: "brush", r: "rust", f: "fur", w: "cream", k: "dark" };
high.forEach((row, z) => {
  [...row].forEach((glyph, x) => {
    if (glyph === ".") return;
    const ink = coat[tint[z]![x] as keyof typeof coat];
    for (let y = 1; y <= Number(glyph); y++) v.set(x, top + y, z, ink);
  });
});

const up = (x: number, y: number, z: number, ink: string) => v.set(x, top + y, z, ink);
up(9, 5, 7, "fur");
up(9, 6, 7, "dark");
up(13, 5, 7, "fur");
up(13, 6, 7, "dark");
up(9, 1, 10, "coat");
up(10, 1, 10, "coat");
up(12, 1, 10, "coat");
up(13, 1, 10, "coat");
up(9, 2, 10, "coat");
up(10, 2, 10, "coat");
up(12, 2, 10, "coat");
up(13, 2, 10, "coat");
up(10, 2, 11, "fur");
up(11, 2, 11, "dark");
up(12, 2, 11, "fur");
up(5, 5, 4, "leaf");
up(5, 5, 5, "leaf");
up(0, 0, 7, "leaf");
up(13, 0, 13, "leaf");
up(7, 0, 15, "leaf");
up(12, 0, 1, "leaf");
up(1, 1, 9, "moss");
up(1, 1, 10, "moss");
up(14, 1, 5, "moss");
up(4, 1, 14, "shade");

v.save(import.meta.dir, "1x");
