import { Canvas, save } from "../paint.ts";

const colors = {
  a: "#17111c",
  h: "#2c1d2b",
  i: "#3e2731",
  b: "#1b223c",
  d: "#5e3428",
  m: "#9c5a34",
  n: "#cc8c52",
  e: "#efcb98",
  f: "#ee7424",
  y: "#ffc457",
  g: "#fff3d6",
};

const key = Object.fromEntries(Object.keys(colors).map((name) => [name, name]));
const bands = ["a", "h", "i"];
const lean = [0, 1, 2, 1];
const glow = [1, 0.98, 0.96, 0.99];

const body = [
  ".ffyggyff..",
  ".fyggggyf..",
  ".fyggggyf..",
  ".fyggggyf..",
  ".fyggggyf..",
  ".ffyggyff..",
  "..fyyyyf...",
  "..ffyyff...",
  "...ffff....",
  "....mm.....",
  ".....n.....",
  ".....a.....",
  "....aa.....",
];

const tops = [
  {
    y: 6,
    rows: [
      "......m....",
      "......f....",
      ".....ff....",
      ".....ff....",
      ".....ff....",
      "....fyf....",
      "....fyf....",
      "...fyyf....",
      "...fyyf....",
      "...fygyf...",
      "..ffygyf...",
      "..fyggyf...",
      "..fyggyf...",
    ],
  },
  {
    y: 6,
    rows: [
      ".......m...",
      ".......f...",
      "......ff...",
      "......ff...",
      "......ff...",
      ".....fyf...",
      ".....fyf...",
      "....fyyf...",
      "....fyyf...",
      "....fygyf..",
      "...ffygyf..",
      "..ffyggyf..",
      "..fyyggyf..",
    ],
  },
  {
    y: 4,
    rows: [
      ".........m.",
      ".........f.",
      "........ff.",
      "........ff.",
      "........f..",
      ".......ff..",
      ".......ff..",
      "......fyf..",
      "......fyf..",
      "......fyf..",
      ".....fyyf..",
      ".....fygf..",
      "....fyggf..",
      "...ffyggf..",
      "..ffyggyf..",
    ],
  },
  {
    y: 8,
    rows: [
      ".......m...",
      ".......f...",
      "......ff...",
      "......ff...",
      ".....fyf...",
      "....fyyf...",
      "....fyyf...",
      "....fygyf..",
      "...ffygyf..",
      "..ffyggyf..",
      "..fyyggyf..",
    ],
  },
];

const ghosts = [
  [".i.", ".d.", "idi", "imi", "idi", ".i."],
  ["..i", ".id", "idi", "imi", "idi", ".i."],
  ["...", "..i", ".i.", "idi", ".i.", "..."],
  ["...", ".id", "idi", "imi", "idi", ".i."],
];

const wax = [
  "mneeeeeeeenm",
  "neeeennneeem",
  "eeeeeeeeeenm",
  "eeeeeeeeeenm",
  "eeeeeeeeenmm",
  "eeeeeeeeenmm",
  "eeeeneeeenmm",
  "eeennneennmd",
  "eennnnennmmd",
  "eenmnnenmmdd",
  "enmmmnemmddd",
  "nmmmmmmmdddd",
];

const saucer = [
  "....dnnn............nnnd....",
  "..dnmmmm............mmmmnd..",
  ".dnmmmmmmmmmmmmmmmmmmmmmmnd.",
  "dnnnnnnnnnnnnnnnnnnnnnnnnnnd",
  ".dmmmmmmmmmmmmmmmmmmmmmmmmd.",
  "...dddddddddddddddddddddd...",
];

const handle = ["..dnnm..", ".dn..md.", ".m....d.", ".m....d.", ".d....d.", "..d..d..", "...dd..."];

const reach = { f: [2.6, 3, 4.2, 4.8], y: [4.6, 5.6, 8, 10], g: [7, 8.5, 11, 13] } as Record<string, number[]>;

function flame(step: number): [number, number, string][] {
  const lit: [number, number, string][] = [];
  const add = (x: number, y: number, rows: string[]) => {
    rows.forEach((row, v) => {
      [...row].forEach((ch, u) => {
        if (ch in reach) lit.push([x + u, y + v, ch]);
      });
    });
  };
  add(13, tops[step]!.y, tops[step]!.rows);
  add(13, 19, body);
  return lit;
}

function near(lit: [number, number, string][], x: number, y: number, outer: boolean): boolean {
  const k = outer ? 2 : 0;
  return lit.some(([u, v, ch]) => ((x - u) / reach[ch]![k]!) ** 2 + ((y - v) / reach[ch]![k + 1]!) ** 2 <= 1);
}

function wash(x: number, y: number, step: number): boolean {
  const dx = x - 17.5;
  const dy = (y - 21) / 1.25;
  const t = Math.atan2(dy, dx);
  const wobble = 1 + 0.11 * Math.sin(2 * t + 0.9) + 0.07 * Math.sin(3 * t + 2.3) + 0.045 * Math.sin(5 * t + 4.1);
  return Math.hypot(dx, dy) < 15.5 * wobble * glow[step]!;
}

function ellipse(x: number, y: number, cx: number, cy: number, rx: number, ry: number): boolean {
  return ((x - cx) / rx) ** 2 + ((y - cy) / ry) ** 2 < 1;
}

function sill(x: number, y: number, step: number): string {
  const s = glow[step]!;
  const u = 18 + (x - 18) / s;
  let level = 0;
  if (y <= 51) level = ellipse(u, y, 18, 47.5, 23, 4.6) ? 2 : ellipse(u, y, 18, 47.5, 36, 6) ? 1 : 0;
  else if (y === 52) level = ellipse(u, 0, 17, 0, 16, 1) ? 2 : ellipse(u, 0, 18, 0, 29, 1) ? 1 : 0;
  else level = ellipse(u, y, 16, 52, 17, 2.8) ? 1 : 0;
  if (ellipse(x, y, 17.5, 47, 11.5, 2.7)) level = 0;
  else if (ellipse(x, y, 17.5, 47, 14, 3.4)) level = Math.max(0, level - 1);
  return bands[level]!;
}

function paint(step: number): Canvas {
  const c = new Canvas(64, 64, colors, "a");
  const lit = flame(step);
  const apex = 17.5 + lean[step]! * 0.7;

  const levels = Array.from({ length: 48 }, (_, y) =>
    Array.from({ length: 64 }, (_, x) => {
      let level = near(lit, x, y, false) ? 2 : wash(x, y, step) || near(lit, x, y, true) ? 1 : 0;
      const spread = (y - 22) / 10;
      if (y >= 32 && x > apex + (11.5 - apex) * spread && x < apex + (23.5 - apex) * spread)
        level = Math.max(0, level - 1);
      return level;
    }),
  );
  for (let pass = 0; pass < 2; pass++)
    levels.forEach((row, y) => {
      row.forEach((level, x) => {
        const around = [levels[y - 1]?.[x], levels[y + 1]?.[x], row[x - 1], row[x + 1]].map((n) => n ?? level);
        if (level > 0 && around.filter((n) => n >= level).length < 2) row[x] = level - 1;
        else if (level < 2 && around.filter((n) => n > level).length >= 3) row[x] = level + 1;
      });
    });
  levels.forEach((row, y) => {
    row.forEach((level, x) => {
      c.px(x, y, bands[level]!);
    });
  });

  c.rect(40, 8, 8, 16, "b").rect(52, 8, 8, 16, "b").rect(40, 28, 8, 16, "b").rect(52, 28, 8, 16, "b");
  for (let y = 8; y < 44; y++) {
    const dy = (y - 21) / glow[step]!;
    c.px(36, y, dy > -11 && dy < 12 ? "i" : dy > -16 && dy < 22 ? "h" : "a");
    if (dy > -6 && dy < 7) c.px(37, y, "h");
  }
  c.sprite(41, 32, ghosts[step]!, key);
  for (let y = 48; y < 55; y++) for (let x = 0; x < 64; x++) c.px(x, y, sill(x, y, step));

  c.sprite(12, 32, wax, key);
  c.sprite(4, 42, saucer, key);
  c.sprite(28, 37, handle, key);
  const glint = Math.round(16 + lean[step]! * 0.7);
  c.rect(glint, 45, 4, 1, "e");

  const top = tops[step]!;
  c.sprite(13, top.y, top.rows, key);
  c.sprite(13, 19, body, key);
  return c;
}

save(import.meta.dir, "4x", [0, 1, 2, 3].map(paint));
