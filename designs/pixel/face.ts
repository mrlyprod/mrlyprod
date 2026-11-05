export const FAMILY = "Pixel";

export const EM = 1000;

export const DOT = 100;

export const ADVANCE = 6 * DOT;

export const ASCENT = 9 * DOT;

export const DESCENT = 3 * DOT;

const G: Record<string, string> = {
  " ": "..... ..... ..... ..... ..... ..... .....",
  "!": "..#.. ..#.. ..#.. ..#.. ..#.. ..... ..#..",
  '"': ".#.#. .#.#. .#.#. ..... ..... ..... .....",
  "#": ".#.#. .#.#. ##### .#.#. ##### .#.#. .#.#.",
  $: "..#.. .#### #.#.. .###. ..#.# ####. ..#..",
  "%": "##... ##..# ...#. ..#.. .#... #..## ...##",
  "&": ".##.. #..#. #.#.. .#... #.#.# #..#. .##.#",
  "'": "..#.. ..#.. .#... ..... ..... ..... .....",
  "(": "...#. ..#.. .#... .#... .#... ..#.. ...#.",
  ")": ".#... ..#.. ...#. ...#. ...#. ..#.. .#...",
  "*": "..... ..#.. #.#.# .###. #.#.# ..#.. .....",
  "+": "..... ..#.. ..#.. ##### ..#.. ..#.. .....",
  ",": "..... ..... ..... ..... ..... .##.. .##.. ..#.. .#...",
  "-": "..... ..... ..... .###. ..... ..... .....",
  ".": "..... ..... ..... ..... ..... .##.. .##..",
  "/": "..... ....# ...#. ..#.. .#... #.... .....",
  "0": ".###. #...# #..## #.#.# ##..# #...# .###.",
  "1": "..#.. .##.. ..#.. ..#.. ..#.. ..#.. .###.",
  "2": ".###. #...# ....# ...#. ..#.. .#... #####",
  "3": "##### ...#. ..#.. ...#. ....# #...# .###.",
  "4": "...#. ..##. .#.#. #..#. ##### ...#. ...#.",
  "5": "##### #.... ####. ....# ....# #...# .###.",
  "6": "..##. .#... #.... ####. #...# #...# .###.",
  "7": "##### ....# ...#. ..#.. .#... .#... .#...",
  "8": ".###. #...# #...# .###. #...# #...# .###.",
  "9": ".###. #...# #...# .#### ....# ...#. .##..",
  ":": "..... .##.. .##.. ..... .##.. .##.. .....",
  ";": "..... .##.. .##.. ..... .##.. .##.. ..#.. .#...",
  "<": "...#. ..#.. .#... #.... .#... ..#.. ...#.",
  "=": "..... ..... ##### ..... ##### ..... .....",
  ">": ".#... ..#.. ...#. ....# ...#. ..#.. .#...",
  "?": ".###. #...# ....# ...#. ..#.. ..... ..#..",
  "@": ".###. #...# ....# .##.# #.#.# #.#.# .###.",
  A: ".###. #...# #...# #...# ##### #...# #...#",
  B: "####. #...# #...# ####. #...# #...# ####.",
  C: ".###. #...# #.... #.... #.... #...# .###.",
  D: "###.. #..#. #...# #...# #...# #..#. ###..",
  E: "##### #.... #.... ####. #.... #.... #####",
  F: "##### #.... #.... ####. #.... #.... #....",
  G: ".###. #...# #.... #.### #...# #...# .####",
  H: "#...# #...# #...# ##### #...# #...# #...#",
  I: ".###. ..#.. ..#.. ..#.. ..#.. ..#.. .###.",
  J: "..### ...#. ...#. ...#. ...#. #..#. .##..",
  K: "#...# #..#. #.#.. ##... #.#.. #..#. #...#",
  L: "#.... #.... #.... #.... #.... #.... #####",
  M: "#...# ##.## #.#.# #.#.# #...# #...# #...#",
  N: "#...# #...# ##..# #.#.# #..## #...# #...#",
  O: ".###. #...# #...# #...# #...# #...# .###.",
  P: "####. #...# #...# ####. #.... #.... #....",
  Q: ".###. #...# #...# #...# #.#.# #..#. .##.#",
  R: "####. #...# #...# ####. #.#.. #..#. #...#",
  S: ".#### #.... #.... .###. ....# ....# ####.",
  T: "##### ..#.. ..#.. ..#.. ..#.. ..#.. ..#..",
  U: "#...# #...# #...# #...# #...# #...# .###.",
  V: "#...# #...# #...# #...# #...# .#.#. ..#..",
  W: "#...# #...# #...# #.#.# #.#.# #.#.# .#.#.",
  X: "#...# #...# .#.#. ..#.. .#.#. #...# #...#",
  Y: "#...# #...# #...# .#.#. ..#.. ..#.. ..#..",
  Z: "##### ....# ...#. ..#.. .#... #.... #####",
  "[": ".###. .#... .#... .#... .#... .#... .###.",
  "\\": "..... #.... .#... ..#.. ...#. ....# .....",
  "]": ".###. ...#. ...#. ...#. ...#. ...#. .###.",
  "^": "..#.. .#.#. #...# ..... ..... ..... .....",
  _: "..... ..... ..... ..... ..... ..... ..... #####",
  "`": ".#... ..#.. ...#. ..... ..... ..... .....",
  a: "..... ..... .###. ....# .#### #...# .####",
  b: "#.... #.... #.##. ##..# #...# #...# ####.",
  c: "..... ..... .###. #.... #.... #...# .###.",
  d: "....# ....# .##.# #..## #...# #...# .####",
  e: "..... ..... .###. #...# ##### #.... .###.",
  f: "..##. .#..# .#... ###.. .#... .#... .#...",
  g: "..... ..... .#### #...# #...# #...# .#### ....# .###.",
  h: "#.... #.... #.##. ##..# #...# #...# #...#",
  i: "..#.. ..... .##.. ..#.. ..#.. ..#.. .###.",
  j: "...#. ..... ..##. ...#. ...#. ...#. ...#. #..#. .##..",
  k: "#.... #.... #..#. #.#.. ##... #.#.. #..#.",
  l: ".##.. ..#.. ..#.. ..#.. ..#.. ..#.. .###.",
  m: "..... ..... ##.#. #.#.# #.#.# #...# #...#",
  n: "..... ..... #.##. ##..# #...# #...# #...#",
  o: "..... ..... .###. #...# #...# #...# .###.",
  p: "..... ..... ####. #...# #...# #...# ####. #.... #....",
  q: "..... ..... .#### #...# #...# #...# .#### ....# ....#",
  r: "..... ..... #.##. ##..# #.... #.... #....",
  s: "..... ..... .#### #.... .###. ....# ####.",
  t: ".#... .#... ###.. .#... .#... .#..# ..##.",
  u: "..... ..... #...# #...# #...# #..## .##.#",
  v: "..... ..... #...# #...# #...# .#.#. ..#..",
  w: "..... ..... #...# #...# #.#.# #.#.# .#.#.",
  x: "..... ..... #...# .#.#. ..#.. .#.#. #...#",
  y: "..... ..... #...# #...# #...# #...# .#### ....# .###.",
  z: "..... ..... ##### ...#. ..#.. .#... #####",
  "{": "...#. ..#.. ..#.. .#... ..#.. ..#.. ...#.",
  "|": "..#.. ..#.. ..#.. ..#.. ..#.. ..#.. ..#..",
  "}": ".#... ..#.. ..#.. ...#. ..#.. ..#.. .#...",
  "~": "..... ..... .#... #.#.# ...#. ..... .....",
  "\u00a0": "..... ..... ..... ..... ..... ..... .....",
  "\u2026": "..... ..... ..... ..... ..... ..... #.#.#",
  "\u2022": "..... ..... .###. .###. .###. ..... .....",
  "\u00b7": "..... ..... ..... ..#.. ..... ..... .....",
  "\u00d7": "..... #...# .#.#. ..#.. .#.#. #...# .....",
  "\u00f7": "..... ..#.. ..... ##### ..... ..#.. .....",
  "\u2212": "..... ..... ..... ##### ..... ..... .....",
  "\u2013": "..... ..... ..... ##### ..... ..... .....",
  "\u2014": "..... ..... ..... ##### ..... ..... .....",
  "\u2018": "..#.. .#... .#... ..... ..... ..... .....",
  "\u2019": "..#.. ..#.. .#... ..... ..... ..... .....",
  "\u201c": ".#.#. #.#.. #.#.. ..... ..... ..... .....",
  "\u201d": ".#.#. .#.#. #.#.. ..... ..... ..... .....",
  "\u00b0": ".##.. #..#. #..#. .##.. ..... ..... .....",
  "\u2190": "..... ..#.. .#... ##### .#... ..#.. .....",
  "\u2192": "..... ..#.. ...#. ##### ...#. ..#.. .....",
  "\u2191": "..#.. .###. #.#.# ..#.. ..#.. ..#.. .....",
  "\u2193": "..... ..#.. ..#.. ..#.. #.#.# .###. ..#..",
  "\u25b2": "..... ..#.. ..#.. .###. .###. ##### .....",
  "\u25bc": "..... ##### .###. .###. ..#.. ..#.. .....",
  "\u25b6": "#.... ##... ###.. ####. ###.. ##... #....",
  "\u25c0": "....# ...## ..### .#### ..### ...## ....#",
  "\u2713": "..... ....# ...#. #.#.. .#... ..... .....",
  "\u2605": "..#.. ..#.. ##### .###. .#.#. #...# .....",
  "\u2665": "..... .#.#. ##### ##### .###. ..#.. .....",
  "\u2588": "##### ##### ##### ##### ##### ##### #####",
  "\u2318": "..... ##.## ##### .#.#. ##### ##.## .....",
  "\u21e7": "..#.. .#.#. #...# ##.## .#.#. .###. .....",
  "\u21b5": "....# ....# ..#.# .#..# ####. .#... ..#..",
  "\u00e9": "...#. ..#.. .###. #...# ##### #.... .###.",
  "\u00e8": ".#... ..#.. .###. #...# ##### #.... .###.",
  "\u00ea": "..#.. .#.#. .###. #...# ##### #.... .###.",
  "\u00e0": ".#... ..#.. .###. ....# .#### #...# .####",
  "\u00e7": "..... ..... .###. #.... #.... #...# .###. ..#.. .#...",
  "\u00fc": ".#.#. ..... #...# #...# #...# #..## .##.#",
  "\u00f6": ".#.#. ..... .###. #...# #...# #...# .###.",
  "\u00e4": ".#.#. ..... .###. ....# .#### #...# .####",
  "\u00f1": ".##.# #.##. ..... #.##. ##..# #...# #...#",
};

export const GLYPHS: [number, string][] = Object.entries(G)
  .map(([char, rows]) => [char.codePointAt(0)!, rows] as [number, string])
  .sort((a, b) => a[0] - b[0]);

const NOTDEF = "##### #...# #...# #...# #...# #...# #####";

export type Box = [number, number, number, number];

export function boxes(rows: string): Box[] {
  return rows.split(" ").flatMap((row, r) => {
    const top = (7 - r) * DOT;
    const out: Box[] = [];
    let from = -1;
    for (let c = 0; c <= row.length; c++) {
      const on = row[c] === "#";
      if (on && from < 0) from = c;
      if (!on && from >= 0) {
        out.push([from * DOT, top - DOT, c * DOT, top]);
        from = -1;
      }
    }
    return out;
  });
}

class Out {
  bytes: number[] = [];
  u8(v: number) {
    this.bytes.push(v & 255);
  }
  u16(v: number) {
    this.u8(v >> 8);
    this.u8(v);
  }
  i16(v: number) {
    this.u16(v < 0 ? v + 65536 : v);
  }
  u32(v: number) {
    this.u16(Math.floor(v / 65536) & 65535);
    this.u16(v & 65535);
  }
  tag(s: string) {
    for (const c of s) this.u8(c.charCodeAt(0));
  }
  zeros(n: number) {
    for (let i = 0; i < n; i++) this.u8(0);
  }
}

const sum = (bytes: number[]) => {
  let total = 0;
  for (let i = 0; i < bytes.length; i += 4)
    total = (total + ((bytes[i]! << 24) | (bytes[i + 1]! << 16) | (bytes[i + 2]! << 8) | bytes[i + 3]!)) >>> 0;
  return total;
};

const pad = (bytes: number[]) => {
  const out = [...bytes];
  while (out.length % 4) out.push(0);
  return out;
};

type Glyph = { boxes: Box[] };

function glyf(glyphs: Glyph[]) {
  const data = new Out();
  const loca: number[] = [];
  for (const one of glyphs) {
    loca.push(data.bytes.length);
    if (!one.boxes.length) continue;
    const xs = one.boxes.flatMap(([x0, , x1]) => [x0, x1]);
    const ys = one.boxes.flatMap(([, y0, , y1]) => [y0, y1]);
    data.i16(one.boxes.length);
    data.i16(Math.min(...xs));
    data.i16(Math.min(...ys));
    data.i16(Math.max(...xs));
    data.i16(Math.max(...ys));
    one.boxes.forEach((_, i) => {
      data.u16(i * 4 + 3);
    });
    data.u16(0);
    const points = one.boxes.flatMap(([x0, y0, x1, y1]) => [
      [x0, y0],
      [x0, y1],
      [x1, y1],
      [x1, y0],
    ]);
    for (const _ of points) data.u8(1);
    let x = 0;
    for (const [px] of points) {
      data.i16(px! - x);
      x = px!;
    }
    let y = 0;
    for (const [, py] of points) {
      data.i16(py! - y);
      y = py!;
    }
    while (data.bytes.length % 4) data.u8(0);
  }
  loca.push(data.bytes.length);
  const offsets = new Out();
  for (const at of loca) offsets.u32(at);
  return { glyf: data.bytes, loca: offsets.bytes };
}

function cmap(codes: number[]) {
  const ends = [...codes, 0xffff];
  const seg = ends.length;
  const power = 2 ** Math.floor(Math.log2(seg));
  const sub = new Out();
  sub.u16(4);
  sub.u16(16 + seg * 8);
  sub.u16(0);
  sub.u16(seg * 2);
  sub.u16(power * 2);
  sub.u16(Math.log2(power));
  sub.u16(seg * 2 - power * 2);
  for (const c of ends) sub.u16(c);
  sub.u16(0);
  for (const c of ends) sub.u16(c);
  ends.forEach((c, i) => {
    sub.u16(c === 0xffff ? 1 : (i + 1 - c + 65536) % 65536);
  });
  for (const _ of ends) sub.u16(0);
  const out = new Out();
  out.u16(0);
  out.u16(1);
  out.u16(3);
  out.u16(1);
  out.u32(12);
  out.bytes.push(...sub.bytes);
  return out.bytes;
}

function name() {
  const records: [number, string][] = [
    [1, FAMILY],
    [2, "Regular"],
    [3, `${FAMILY} 1.0`],
    [4, FAMILY],
    [5, "Version 1.0"],
    [6, `${FAMILY}-Regular`],
  ];
  const out = new Out();
  out.u16(0);
  out.u16(records.length);
  out.u16(6 + records.length * 12);
  const strings = new Out();
  for (const [id, text] of records) {
    out.u16(3);
    out.u16(1);
    out.u16(0x409);
    out.u16(id);
    out.u16(text.length * 2);
    out.u16(strings.bytes.length);
    for (const c of text) strings.u16(c.charCodeAt(0));
  }
  out.bytes.push(...strings.bytes);
  return out.bytes;
}

export function font(): ArrayBuffer {
  const glyphs: Glyph[] = [{ boxes: boxes(NOTDEF) }, ...GLYPHS.map(([, rows]) => ({ boxes: boxes(rows) }))];
  const codes = GLYPHS.map(([code]) => code);
  const all = glyphs.flatMap((one) => one.boxes);
  const most = Math.max(...glyphs.map((one) => one.boxes.length));
  const { glyf: shapes, loca } = glyf(glyphs);

  const head = new Out();
  head.u32(0x00010000);
  head.u32(0x00010000);
  head.u32(0);
  head.u32(0x5f0f3cf5);
  head.u16(0x000b);
  head.u16(EM);
  head.zeros(16);
  head.i16(0);
  head.i16(Math.min(...all.map((b) => b[1])));
  head.i16(Math.max(...all.map((b) => b[2])));
  head.i16(Math.max(...all.map((b) => b[3])));
  head.u16(0);
  head.u16(8);
  head.i16(2);
  head.i16(1);
  head.i16(0);

  const hhea = new Out();
  hhea.u32(0x00010000);
  hhea.i16(ASCENT);
  hhea.i16(-DESCENT);
  hhea.i16(0);
  hhea.u16(ADVANCE);
  hhea.i16(0);
  hhea.i16(DOT);
  hhea.i16(5 * DOT);
  hhea.i16(1);
  hhea.i16(0);
  hhea.i16(0);
  hhea.zeros(8);
  hhea.i16(0);
  hhea.u16(glyphs.length);

  const maxp = new Out();
  maxp.u32(0x00010000);
  maxp.u16(glyphs.length);
  maxp.u16(most * 4);
  maxp.u16(most);
  maxp.u16(0);
  maxp.u16(0);
  maxp.u16(2);
  maxp.zeros(16);

  const os2 = new Out();
  os2.u16(4);
  os2.i16(ADVANCE);
  os2.u16(400);
  os2.u16(5);
  os2.u16(0);
  for (const v of [325, 350, 0, 75, 325, 350, 0, 350, 50, 300]) os2.i16(v);
  os2.i16(0);
  os2.zeros(10);
  os2.u32(1);
  os2.u32(0);
  os2.u32(0);
  os2.u32(0);
  os2.tag("MRLY");
  os2.u16(0x00c0);
  os2.u16(codes[0]!);
  os2.u16(codes.at(-1)!);
  os2.i16(ASCENT);
  os2.i16(-DESCENT);
  os2.i16(0);
  os2.u16(ASCENT);
  os2.u16(DESCENT);
  os2.u32(1);
  os2.u32(0);
  os2.i16(5 * DOT);
  os2.i16(7 * DOT);
  os2.u16(0);
  os2.u16(32);
  os2.u16(0);

  const hmtx = new Out();
  for (const one of glyphs) {
    hmtx.u16(ADVANCE);
    hmtx.i16(one.boxes.length ? Math.min(...one.boxes.map((b) => b[0])) : 0);
  }

  const post = new Out();
  post.u32(0x00030000);
  post.u32(0);
  post.i16(-DOT);
  post.i16(DOT);
  post.u32(1);
  post.zeros(16);

  const tables: [string, number[]][] = (
    [
      ["OS/2", os2.bytes],
      ["cmap", cmap(codes)],
      ["glyf", shapes],
      ["head", head.bytes],
      ["hhea", hhea.bytes],
      ["hmtx", hmtx.bytes],
      ["loca", loca],
      ["maxp", maxp.bytes],
      ["name", name()],
      ["post", post.bytes],
    ] as [string, number[]][]
  ).sort(([a], [b]) => (a < b ? -1 : 1));

  const power = 2 ** Math.floor(Math.log2(tables.length));
  const top = new Out();
  top.u32(0x00010000);
  top.u16(tables.length);
  top.u16(power * 16);
  top.u16(Math.log2(power));
  top.u16(tables.length * 16 - power * 16);
  let at = 12 + tables.length * 16;
  const body: number[] = [];
  let adjust = 0;
  for (const [tag, bytes] of tables) {
    top.tag(tag);
    top.u32(sum(pad(bytes)));
    top.u32(at);
    top.u32(bytes.length);
    if (tag === "head") adjust = at + 8;
    const padded = pad(bytes);
    body.push(...padded);
    at += padded.length;
  }
  const whole = [...top.bytes, ...body];
  const fix = (0xb1b0afba - sum(whole) + 2 ** 32) % 2 ** 32;
  whole[adjust] = fix >>> 24;
  whole[adjust + 1] = (fix >>> 16) & 255;
  whole[adjust + 2] = (fix >>> 8) & 255;
  whole[adjust + 3] = fix & 255;
  return new Uint8Array(whole).buffer;
}

let worn: FontFace | null = null;

export function wear() {
  if (worn || typeof FontFace !== "function" || typeof document === "undefined") return;
  worn = new FontFace(FAMILY, font(), { weight: "100 900", style: "normal", display: "block" });
  document.fonts.add(worn);
  worn.load().catch((error) => console.error(error));
}
