import { expect, test } from "bun:test";
import { createHash } from "node:crypto";
import { dark, light } from "../../../site/kit/theme/theme.js";
import * as core from "../core.js";
import { Grid, field, frame, grid, hex, ink, iso, plot, raster } from "./index.js";

const T = ink(dark);
const sha = (data) => createHash("sha256").update(data).digest("hex");

// PRIMITIVES

const FG = [255, 255, 255, 255];
const BLUE = [0, 140, 255, 255];
const ORANGE = [255, 143, 44, 255];
const YELLOW = [255, 209, 0, 255];
const GREEN = [50, 204, 88, 255];
const PINK = [255, 50, 90, 255];
const INDIGO = [103, 104, 250, 255];
const PANEL = [17, 17, 18, 255];
const GLASS = [255, 255, 255, 128];

const ZIG = [[10, 128], [30.5, 60.25], [55.1, 190.7], [80, 70], [105.5, 200.5], [130.25, 50], [155, 180.75], [180.4, 90], [205, 210], [245.5, 40.5]];
const BITE = [[30, 30], [220, 40.5], [150.5, 120], [225, 200.25], [40.75, 215], [100, 125]];

const PRIMITIVES = {
  rect: [["rect", 20.3, 30.7, 150.4, 90.2, FG]],
  round_rect: [["round_rect", 20.3, 30.7, 200, 150, 30, FG], ["round_rect", 10.5, 200.25, 100, 20, 50, BLUE]],
  disc: [["disc", 128.3, 127.6, 90, FG]],
  ring: [["ring", 128.3, 127.6, 80, 6, FG]],
  segment: [["segment", [20.5, 30.25], [230.1, 200.7], 7, FG]],
  polyline: [["polyline", ZIG, 5, FG]],
  triangle: [["triangle", [30, 220], [128.4, 25.3], [230.2, 210.1], FG]],
  polygon: [["polygon", BITE, FG]],
  arc: [["arc", [128.3, 127.6], 80, [0.3, 4.1], 9, FG], ["arc", [60, 60], 30, [4.1, 0.3], 5, ORANGE], ["arc", [200, 200], 40, [0, 7], 3, YELLOW], ["arc", [200, 60], 30, [1, 1], 6, PINK]],
  edges: [["rect", 5.5, 5.5, 0.4, 0.4, FG], ["disc", 100.5, 100.5, 0.3, FG], ["disc", -3, 20, 10, FG], ["rect", 250, 250, 20, 20, FG], ["segment", [0, 0], [256, 256], 1, FG], ["polygon", [[1, 1], [9, 9]], FG], ["ring", 128, 128, 0, 6, FG], ["segment", [40, 40], [40, 40], 9, BLUE]],
  glass: [["rect", 20, 20, 150, 150, BLUE], ["disc", 150, 150, 70, GLASS], ["polyline", ZIG, 9, [255, 143, 44, 153]], ["triangle", [20, 240], [120, 140], [240, 236], [50, 204, 88, 128]]],
  mixed: [
    ["rect", 10, 10, 236, 236, PANEL],
    ["disc", 90, 100, 60, BLUE],
    ["ring", 160, 150, 50, 8, [255, 143, 44, 153]],
    ["polyline", ZIG, 3, YELLOW],
    ["triangle", [20, 240], [120, 140], [240, 236], [50, 204, 88, 128]],
    ["arc", [128, 128], 100, [-1, 2], 5, PINK],
    ["round_rect", 150, 20, 80, 60, 12, INDIGO],
    ["segment", [20, 20], [100, 60], 4, FG],
    ["polygon", BITE, [255, 255, 255, 102]],
  ],
};

// SOLIDS

const DIRS = [[1, 0, 0], [-1, 0, 0], [0, 1, 0], [0, -1, 0], [0, 0, 1], [0, 0, -1]];

function corners(i, j, k, [dx, dy, dz]) {
  const c = (a, b, d) => ({ x: i + a, y: j + b, z: k + d });
  const u = (dx || dy || dz) > 0 ? 1 : 0;
  if (dx) return [c(u, 0, 0), c(u, 1, 0), c(u, 1, 1), c(u, 0, 1)];
  if (dy) return [c(0, u, 0), c(1, u, 0), c(1, u, 1), c(0, u, 1)];
  return [c(0, 0, u), c(1, 0, u), c(1, 1, u), c(0, 1, u)];
}

function quadsOf(filled, scale = 1) {
  const has = new Set(filled.map((c) => c.join()));
  const out = [];
  for (const [i, j, k] of filled) {
    for (const d of DIRS) {
      if (has.has([i + d[0], j + d[1], k + d[2]].join())) continue;
      const verts = corners(i, j, k, d).map((v) => ({ x: Math.fround(v.x * scale), y: Math.fround(v.y * scale), z: Math.fround(v.z * scale) }));
      out.push({ normal: { x: d[0], y: d[1], z: d[2] }, verts });
    }
  }
  return out;
}

const SPONGE = [];
for (let i = 0; i < 27; i++) {
  const cell = [Math.floor(i / 9), Math.floor(i / 3) % 3, i % 3];
  if (cell.filter((v) => v === 1).length <= 1) SPONGE.push(cell);
}
const STAIRS = [[0, 0, 0], [1, 0, 0], [1, 1, 0], [1, 1, 1], [2, 1, 1], [0, 0, 1]];

// SCENES

const Q_STAIRS = quadsOf(STAIRS);
const Q_SPONGE = quadsOf(SPONGE);
const Q_FRAC = quadsOf(STAIRS, 0.1);

const vals = Array.from({ length: 48 }, (_, i) => ((i * 37) % 17) / 3);
const cells = { shape: [4, 5], types: Uint8Array.from([1, 0, 2, 1, 0, 0, 1, 1, 2, 0, 2, 0, 1, 0, 1, 1, 1, 0, 2, 2]) };
const hcells = (h, w) => ({ cell: { shape: [h, w], types: Uint8Array.from({ length: h * w }, (_, i) => (i * 5 + (i >> 2)) % 3) }, start: 0 });

const KIT = {
  plot(p) {
    const f = frame(20.5, 30.25, 200.3, 150.7);
    plot.bars(p, f, [1, -4, 2.5, 0, 3.3], 0.2, T.blue);
    plot.axis(p, f, T.dim);
    plot.baseline(p, f, T.fg);
    plot.dots(p, [[10.5, 10.5], [100.25, 50]], 4.5, T.yellow);
    plot.rings(p, [128, 128], [10, 20.5, 33.3], 2, T.orange);
    plot.staircase(p, f, [3, 1, 4, 1.5, 9, 2.6], 2, T.green);
    plot.curve(p, f, [0, 1, 2.5, 4], [5, 9, 7, 1], 3, T.pink);
    plot.axis(p, frame(200, 200, 40, 20), T.fg);
    plot.axis(p, frame(10, 200, 600, 1000), T.fg);
  },
  grid(p) {
    const g = new Grid(frame(3.3, 0.7, 200.12, 180.9), 7, 5, 0);
    for (let row = 0; row < 5; row++) for (let col = 0; col < 7; col++) if ((row * 3 + col * 5) % 4 === 0) g.fill(p, col, row, T.blue);
    grid.carpet(p, frame(10, 10, 120, 120), grid.mask(grid.LOGO, 2), 0, T.fg);
    grid.carpet(p, frame(130.5, 130.5, 120, 100), grid.mask(["11", "10"], 3), 0.15, T.orange);
    new Grid(frame(140, 10, 100, 80), 5, 4, 0.1).paint(p, cells, (t) => (t === 1 ? T.yellow : t === 2 ? T.pink : null));
    new Grid(frame(0.5, 0.5, 90, 60), 0, 0, 0).fill(p, 0, 0, T.green);
  },
  hex(p) {
    const f = frame(8, 8, 240, 240);
    hex.hexagon(p, f, 4, 1.2, (row, col, par) => ((row + col) % 3 === 0 ? null : par === 1 ? T.blue : T.yellow));
    hex.hexagon(p, frame(100.5, 100.5, 60, 60), 1, 0, (r, c, par) => (par === 1 ? T.fg : T.dim));
    hex.hexagon(p, f, 0, 0, () => T.fg);
  },
  hexdraw(p) {
    const color = (t) => (t === 1 ? T.blue : t === 2 ? T.orange : null);
    hex.draw(p, frame(10.5, 20.25, 230, 100), { ...hcells(4, 9), start: 1 }, 0.8, color);
    hex.draw(p, frame(60.5, 120.25, 100, 130), { ...hcells(9, 4), start: 0 }, 0, color);
    hex.draw(p, frame(160.5, 120.25, 90, 100), { ...hcells(5, 8), start: 3 }, 1.5, color);
  },
  iso(p) {
    const shade = [T.fg, T.blue, T.dim];
    iso.draw(p, frame(5, 5, 120, 120), Q_STAIRS, shade, null);
    iso.draw(p, frame(130.5, 5, 120, 120), Q_STAIRS, shade, T.line);
    iso.draw(p, frame(5.5, 130, 110, 120), Q_SPONGE, shade, T.ground);
    iso.draw(p, frame(130, 130, 100.5, 120.5), Q_FRAC, [T.green, T.pink, T.indigo], T.fg);
  },
  field(p) {
    field.draw(p, frame(3.6, 2.2, 150.3, 111.7), 8, 6, vals, T.Ramp.heat());
    field.draw_range(p, frame(100.2, 120.9, 140, 120), 8, 6, vals, [1, 4], T.Ramp.fire());
    field.sample(p, frame(5, 130, 90, 90), 9, (u, v) => u * u - v, T.Ramp.diverge());
    field.draw(p, frame(180, 5, 50, 50), 2, 2, [3, 3, 3, 3], T.Ramp.tone(T.blue, T.yellow));
    field.draw(p, frame(180, 60, 50, 50), 2, 2, [0, 1, 2, 3], new T.Ramp([T.ground, T.fg]));
  },
};

const Q_CUBE2 = quadsOf([[0, 0, 0], [1, 0, 0], [1, 1, 0], [1, 1, 1], [0, 1, 1]]);

KIT.stamp = (p) => {
  const shade = [T.blue, T.mix(T.blue, T.ground, 0.4), T.mix(T.blue, T.ground, 0.65)];
  const wire = T.mix(T.line, T.dim, 0.3);
  iso.cage(p, 60, 60, 25, 1, 25 / 20, wire);
  iso.stamp(p, Q_CUBE2, 60, 60, 25, shade, T.ground, 25 / 14);
  iso.cage(p, 180, 60.5, 22.3, 1.5, 22.3 / 24, wire);
  iso.stamp(p, Q_SPONGE, 180, 60.5, 22.3, shade, T.ground, 22.3 / 18);
  iso.cage(p, 60.5, 180, 30, 1, 1, wire);
  iso.stamp(p, Q_STAIRS, 60.5, 180, 30, shade, T.ground, 2);
};

const washed = (p) => {
  const w = field.patch(30, 20, 80, 50);
  for (let py = 20; py < 70; py++) {
    for (let px = 30; px < 110; px++) {
      const u = (px - 30) / 80;
      const v = (py - 20) / 50;
      w.blend(px, py, [Math.trunc(u * 255), Math.trunc(v * 255), 128, (px + py) % 7 === 0 ? 100 : 255], u * 1.3 - v * 0.2 - 0.1);
    }
  }
  w.paint(p);
  const x = field.patch(60, 40, 120, 90);
  for (let py = 40; py < 130; py++) for (let px = 60; px < 180; px++) x.blend(px, py, T.orange, ((px * 31 + py * 17) % 101) / 100);
  x.paint(p);
};
KIT.cover = washed;
KIT.cover_clear = Object.assign((p) => washed(p), { ground: [0, 0, 0, 0] });

const numbers = () => {
  const out = {};
  const mixes = [];
  for (let k = 0; k <= 510; k++) mixes.push(T.mix([0, 0, 0, 255], [255, 255, 255, 255], k / 510), T.mix([10, 200, 7, 0], [250, 3, 99, 255], k / 510));
  out.mix = mixes.flat();
  out.mixclamp = [T.mix(T.blue, T.orange, -3), T.mix(T.blue, T.orange, 7)].flat();
  out.fade = Array.from({ length: 511 }, (_, k) => T.fade(T.blue, k / 510)).flat();
  out.fadeclamp = [T.fade(T.blue, -1), T.fade(T.blue, 2)].flat();
  const ramps = [T.Ramp.heat(), T.Ramp.fire(), T.Ramp.diverge(), T.Ramp.tone(T.blue, T.yellow), new T.Ramp([]), new T.Ramp([T.pink])];
  out.ramp = ramps.map((r) => Array.from({ length: 301 }, (_, k) => r.at((k - 20) / 260)).flat()).flat();
  const f = frame(10.3, 20.7, 300.1, 200.9);
  out.frame = [f.inset(7.5), f.inset(-4), ...f.cols(3), ...f.rows(7), f.square(), frame(1, 2, 3, 40).square()].map((x) => [x.x, x.y, x.w, x.h]).flat();
  out.frame.push(f.cell(7), f.cell(81), ...f.at(0.3, 0.7), ...f.center(), f.radius(), frame(0, 0, 5, 3).radius());
  const pen = raster(1200, 630, T.ground);
  const ok = [0, 0.08, 0.1, 0.5].flatMap((m) => [pen.frame(m), pen.area(m)]);
  out.board = ok.map((x) => [x.x, x.y, x.w, x.h]).flat();
  const cellsOut = [];
  for (const [fr, cols, rows, gap] of [[frame(3.3, 0.7, 389.12, 389.12), 5, 5, 0], [frame(0.5, 0.5, 1000.3, 700.1), 81, 27, 0], [frame(5.25, 3.75, 100, 50), 9, 4, 0.2], [frame(0, 0, 10, 10), 0, 0, 0.5], [frame(-2.5, -3.5, 7, 9), 3, 3, 0]]) {
    const g = new Grid(fr, cols, rows, gap);
    for (let r = 0; r < g.rows; r++) for (let c = 0; c < g.cols; c++) cellsOut.push(...g.cell(c, r));
  }
  out.gridcell = cellsOut;
  out.mask = [grid.mask(grid.LOGO, 1), grid.mask(grid.LOGO, 2), grid.mask(["101", "010"], 3), grid.mask(["1"], 0), grid.mask([], 3)].map((m) => m.map((row) => row.map(Number).join("")).join("/"));
  out.hexcount = [1, 2, 3, 7].flatMap((n) => [hex.count(n), ...Array.from({ length: 2 * n }, (_, r) => hex.row_len(n, r))]);
  out.project = [[0, 0, 0], [1, 2, 3], [-1.5, 2.25, 0.1], [3, 3, 3]].flatMap(([x, y, z]) => iso.project(x, y, z));
  return out;
};

const PRIMITIVE_HASH = {
  rect: "33637a2fffa5725446aaa59fcd4b55178699b5ee7e443c1a560ad74fab07ca88",
  round_rect: "89c5ed4e96012193dae29db7d6da83db27ef42299ad5b4e82edb7452d8d938fc",
  disc: "193c864aee7b986d8ddd778408509a1a5f769322ee08a1fb24b2090f4bd9075d",
  ring: "24e32c43a8ec1fdb37add48ade3c6eda8bf0b43f0a9a0645e66520f8074ab23e",
  segment: "6eff5804631cc30dc67dd306a9475371f961343649cdb89b49ba1b3c5dcb0375",
  polyline: "05c522f1851a92e4a3aa8d82bd51cb1fa80aba77150c770e83be6a06daea8ba6",
  triangle: "4510bcea5dfc7fa992387926689caecc9db3eb946284fa81e2c393c6e839db61",
  polygon: "8ce392b193ea46ffc2841d9eaa3fafa9b24127dd83689b3730db0ce842466799",
  arc: "f6afbafc9a81a467103d8aae1ec287d98d30809bfffbb7f3ed940a57f64162af",
  edges: "b852bd06558560426fb3e37fe970cd7743d3224c1b17c29bb8b26958631f2f98",
  glass: "92fe0a502443e87f4de95d226f8468acd1d348f388e17c17e17a21d86feb5853",
  mixed: "e30cbc520e1115ac282362f98966f69f10ee899259024f67188157a18fb2375e",
};

for (const [name, ops] of Object.entries(PRIMITIVES)) {
  test(`${name} paints the bytes board.rs paints`, () => {
    const pen = raster(256, 256, T.ground);
    for (const [verb, ...args] of ops) pen[verb](...args);
    expect(sha(pen.pixels().colors)).toBe(PRIMITIVE_HASH[name]);
  });
}

// KIT

const KIT_HASH = {
  plot: "35027d286712d9645b0f32d5ac20e4282c91a1049bdacbc6ea73944dc5fc54b2",
  grid: "95a7e341c87d242ee31f124a7dc11e2c37e3a925aa54cd2d8f8b08b05764108c",
  hex: "d45be623a8bbc008ce10363fdc8cab7fe38acf8e2d3f8e43b7bd8afbc2d9a685",
  hexdraw: "67b0b02a387ca982a178ade2d038608173c1481c7fb1f6215bfd6bad23283516",
  iso: "c6e9beecb96d3ef7ef493e93ec847021a3a1bcc4bf8627c3c7e7b38f4b4b586e",
  field: "77d64330e4964c4adc0e98c2d54a949c24bf14861fa71ca5b7d7914ffe7ba52f",
  stamp: "d64a36cbf75b3ca53605325593de70df7c906edbc63a96eaa2d0aafbd96a5173",
  cover: "d8e6141541d6f5a5aaae2eb9b3969e53ea00d0d0c6db99feb4545969f2bde449",
  cover_clear: "ecc6b75a0d49d31c220e2b1a2d462ea725eb0e1007b81e1ea9ba0d85d8235306",
};

for (const [name, run] of Object.entries(KIT)) {
  test(`${name} paints the bytes the Rust kit paints`, () => {
    const pen = raster(256, 256, run.ground ?? T.ground);
    run(pen);
    expect(sha(pen.pixels().colors)).toBe(KIT_HASH[name]);
  });
}

// NUMBERS

const NUMBER_HASH = {
  mix: "eb017477ac9e5d126a0371f0e4398cff12418f3b3927c6a6d6ecc198aa08f076",
  mixclamp: "7ceb2f817be4678d83a05e207bf33bc3a721daf19cc708c869f6733028376518",
  fade: "fa64a0aaf12d55155d64644e87f824b9459e084d4c6b09d5dedd40f9892475fa",
  fadeclamp: "3be3185695cfc76d0b37dd088ef4938ff911f49f479dd18c9f2f953d2420bb2e",
  ramp: "94ca8e792131dbd210117603cbf1fa9b6625eacdea248710bffacc4c591b5366",
  frame: "e473a0333ad51f75753806915357ec16a435204265b606ad2c86a9e3351583cb",
  board: "47788d5ca5b065bb02cb1ffb837b4a679ed3263b22ccc29094b7642eb04dcb9f",
  gridcell: "3696344b2afcef40e56dc56c15695fb2d15b4d79f94e0b4cc2434ef3d43d9e78",
  mask: "bf4a3d4f774967dda93acdf82543caac62d6a71f07a2907d034011ee9b6acf9f",
  hexcount: "100da5292c1dec79b80932855c5e0133f48373887a37eaee786ba30de9b02a07",
  project: "8a412a91f33f3bd37ac0f35cfa1f9b216e770a6e27ae4c16429cd8d0a013fd42",
};

for (const [name, value] of Object.entries(numbers())) {
  test(`${name} gives the numbers the Rust kit gives`, () => {
    expect(sha(JSON.stringify(value))).toBe(NUMBER_HASH[name]);
  });
}

// THEME

test("ink from the Rust theme equals ink from the kit theme", async () => {
  core.initSync({ module: await Bun.file(new URL("../pkg/core/mrlyjs_core_bg.wasm", import.meta.url)).arrayBuffer() });
  for (const [rust, kit] of [[core.colors.DARK(), dark], [core.colors.LIGHT(), light]]) {
    const ours = ink(rust);
    const theirs = ink(kit);
    for (const key of Object.keys(theirs)) if (Array.isArray(theirs[key])) expect([key, ours[key]]).toEqual([key, theirs[key]]);
  }
});
