import { expect, test } from "bun:test";
import { dark, light } from "../../../site/kit/theme/theme.js";
import { Grid, fade, field, frame, grid, hex, ink, iso, mix, plot, raster } from "./index.js";

const HUES = {
  red: [255, 61, 64],
  orange: [255, 143, 44],
  yellow: [255, 209, 0],
  green: [50, 204, 88],
  mint: [0, 209, 187],
  teal: [0, 202, 216],
  cyan: [30, 201, 243],
  blue: [0, 140, 255],
  indigo: [103, 104, 250],
  purple: [211, 50, 233],
  pink: [255, 50, 90],
  brown: [177, 132, 98],
  gray: [142, 142, 147],
};
const RUST = {
  dark: { ground: [0, 0, 0], panel: [17, 17, 18], line: [31, 31, 32], fg: [255, 255, 255], dim: [142, 142, 147], ...HUES },
  light: { ground: [255, 255, 255], panel: [241, 241, 242], line: [221, 221, 223], fg: [0, 0, 0], dim: [85, 85, 88], ...HUES },
};
const theme = ink(dark);

function recorder() {
  const calls = [];
  const pen = new Proxy({}, { get: (_, key) => (...args) => calls.push([key, ...args]) });
  return { pen, calls };
}

// INK

test("the dark and light inks are ink.rs's", () => {
  for (const [name, palette] of [["dark", dark], ["light", light]]) {
    const read = ink(palette);
    for (const [key, rgb] of Object.entries(RUST[name])) expect([name, key, read[key]]).toEqual([name, key, [...rgb, 255]]);
  }
});

test("ink reads hex strings, byte arrays and serde colors alike", () => {
  const bytes = Object.fromEntries(Object.entries(theme).filter(([, v]) => v.length === 4 && typeof v[0] === "number"));
  const serde = Object.fromEntries(Object.entries(bytes).map(([k, [r, g, b, a]]) => [k, { r, g, b, a }]));
  expect(ink(bytes).blue).toEqual(theme.blue);
  expect(ink(serde).dim).toEqual(theme.dim);
  expect(() => ink({ ...bytes, fg: undefined })).toThrow("no fg");
});

test("ramp ends are its end stops", () => {
  const ramp = theme.Ramp.heat();
  expect(ramp.at(0)).toEqual(theme.ground);
  expect(ramp.at(1)).toEqual(theme.fg);
});

test("an empty ramp reads the ground", () => {
  expect(new theme.Ramp([]).at(0.5)).toEqual(theme.ground);
});

test("mix halfway sits between the two", () => {
  expect(mix([0, 0, 0, 255], [255, 255, 255, 255], 0.5)[0]).toBe(128);
});

test("fade sets the alpha to its share of opaque", () => {
  expect(fade(theme.blue, 0.5)).toEqual([0, 140, 255, 128]);
});

test("the inks are the theme's six", () => {
  expect(theme.inks).toEqual([theme.blue, theme.orange, theme.yellow, theme.green, theme.pink, theme.indigo]);
});

// FRAME

test("a frame splits into columns and rows that tile it", () => {
  const box = frame(10, 20, 300, 200);
  expect(box.cols(3).map((f) => f.x)).toEqual([10, 110, 210]);
  expect(box.rows(4).at(-1)).toMatchObject({ y: 170, h: 50, w: 300 });
  expect(box.square()).toMatchObject({ x: 60, y: 20, w: 200, h: 200 });
  expect(box.center()).toEqual([160, 120]);
});

test("a frame lays panels row by row, each inset by the gap", () => {
  const tiles = frame(10, 20, 300, 200).panels(2, 3, 5);
  expect(tiles.length).toBe(6);
  expect(tiles[0]).toMatchObject({ x: 15, y: 25, w: 90, h: 90 });
  expect(tiles[2]).toMatchObject({ x: 215, y: 25 });
  expect(tiles[5]).toMatchObject({ x: 215, y: 125, w: 90, h: 90 });
});

// GRID

test("the logo at level two is twenty five wide and squares its ones", () => {
  const rows = grid.mask(grid.LOGO, 2);
  expect([rows.length, rows[0].length, rows.flat().filter(Boolean).length]).toEqual([25, 25, 21 * 21]);
});

test("at gap zero neighbouring cells meet on a whole pixel", () => {
  const lattice = new Grid(frame(3.3, 0, 389.12, 389.12), 5, 5, 0);
  const [x0, , w0] = lattice.cell(0, 0);
  const [x1] = lattice.cell(1, 0);
  expect(x0 + w0).toBe(x1);
  expect(x1).toBe(Math.round(x1));
});

test("at gap zero a carpet leaves no seam on the raster", () => {
  const pen = raster(400, 400, [0, 0, 0, 255]);
  grid.carpet(pen, frame(3.3, 0.7, 389.12, 389.12), grid.mask(["11", "11"], 3), 0, [255, 255, 255, 255]);
  const { colors } = pen.pixels();
  const seams = [];
  for (let x = 3; x < 392; x++) if (colors[(200 * 400 + x) * 4] !== 255) seams.push(["x", x]);
  for (let y = 1; y < 390; y++) if (colors[(y * 400 + 200) * 4] !== 255) seams.push(["y", y]);
  expect(seams).toEqual([]);
});

test("paint fills the cells whose type the ink maps", () => {
  const { pen, calls } = recorder();
  const cells = { shape: [2, 3], types: Uint8Array.from([1, 0, 1, 0, 2, 0]) };
  new Grid(frame(0, 0, 30, 20), 3, 2, 0).paint(pen, cells, (t) => (t === 1 ? theme.blue : t === 2 ? theme.orange : null));
  expect(calls).toEqual([["rect", 0, 0, 10, 10, theme.blue], ["rect", 20, 0, 10, 10, theme.blue], ["rect", 10, 10, 10, 10, theme.orange]]);
});

test("paint reads a tensor's data as it reads a cell's types", () => {
  const { pen, calls } = recorder();
  const ink = (t) => (t === 1 ? theme.blue : null);
  const lattice = new Grid(frame(0, 0, 30, 20), 3, 2, 0);
  lattice.paint(pen, { shape: [2, 3], data: Uint8Array.from([1, 0, 1, 0, 1, 0]) }, ink);
  expect(calls).toEqual([["rect", 0, 0, 10, 10, theme.blue], ["rect", 20, 0, 10, 10, theme.blue], ["rect", 10, 10, 10, 10, theme.blue]]);
});

// HEX

test("a hex slice reads its rows centred in its width", () => {
  const types = Uint8Array.from({ length: 28 }, (_, i) => i);
  const slice = { cell: { shape: [4, 7], types } };
  expect([hex.at(slice, 2, 0, 0), hex.at(slice, 2, 1, 0), hex.at(slice, 2, 3, 4)]).toEqual([1, 7, 26]);
});

test("a hexagon of side s has six s squared triangles", () => {
  for (let n = 1; n < 8; n++) {
    let rows = 0;
    for (let row = 0; row < 2 * n; row++) rows += hex.row_len(n, row);
    expect(rows).toBe(hex.count(n));
  }
});

test("every triangle of a hexagon is offered to the ink", () => {
  const pen = raster(128, 128, theme.ground);
  let seen = 0;
  hex.hexagon(pen, pen.frame(0.1), 3, 0, () => void seen++);
  expect(seen).toBe(hex.count(3));
});

test("a hex slice draws one triangle per inked site and wants a hexagon", () => {
  const { pen, calls } = recorder();
  const cell = { cell: { shape: [2, 3], types: Uint8Array.from([1, 1, 0, 1, 0, 1]) }, start: 0 };
  hex.draw(pen, frame(0, 0, 100, 100), cell, 1, (t) => (t ? theme.fg : null));
  expect(calls.map(([key]) => key)).toEqual(["triangle", "triangle", "triangle", "triangle"]);
  expect(() => hex.draw(pen, frame(0, 0, 100, 100), { cell: { shape: [2, 2], types: [1, 1, 1, 1] }, start: 0 }, 0, () => theme.fg)).toThrow("hexagon");
});

// ISO

test("the projection lifts z straight up the screen", () => {
  const ground = iso.project(0, 0, 0);
  const above = iso.project(0, 0, 1);
  expect(Math.abs(above[0] - ground[0])).toBeLessThan(1e-12);
  expect(above[1]).toBeLessThan(ground[1]);
});

test("iso drops faces turned away and paints back to front in three tones", () => {
  const { pen, calls } = recorder();
  const face = (normal, z) => ({ normal, verts: [{ x: 0, y: 0, z }, { x: 1, y: 0, z }, { x: 1, y: 1, z }, { x: 0, y: 1, z }] });
  const quads = [face({ x: 0, y: 0, z: 1 }, 1), face({ x: 0, y: 0, z: -1 }, 0), face({ x: 0, y: 1, z: 0 }, 0.5), face({ x: 1, y: 0, z: 0 }, -1)];
  const shade = [theme.fg, theme.blue, theme.dim];
  iso.draw(pen, frame(0, 0, 400, 400), quads, shade, theme.line);
  expect(calls.filter(([key]) => key === "polygon").map((call) => call[2])).toEqual([theme.dim, theme.blue, theme.fg]);
  expect(calls.filter(([key]) => key === "polyline").map((call) => [call[1].length, call[2]])).toEqual([[5, 1], [5, 1], [5, 1]]);
});

test("the cube's twelve edges join corners one bit apart, low to high", () => {
  const bits = (n) => n.toString(2).replace(/0/g, "").length;
  expect(iso.EDGES.length).toBe(12);
  for (const [a, b] of iso.EDGES) expect([a < b, bits(a ^ b)]).toEqual([true, 1]);
  expect(iso.EDGES.map(([a, b]) => a * 8 + b)).toEqual(iso.EDGES.map(([a, b]) => a * 8 + b).sort((x, y) => x - y));
});

test("faces drops the faces turned away and sorts the rest back to front", () => {
  const face = (normal, z) => ({ normal, verts: [{ x: 0, y: 0, z }, { x: 1, y: 0, z }, { x: 1, y: 1, z }, { x: 0, y: 1, z }] });
  const list = iso.faces([face({ x: 0, y: 0, z: 1 }, 1), face({ x: 0, y: 0, z: -1 }, 0), face({ x: 0, y: 1, z: 0 }, 0.5), face({ x: 1, y: 0, z: 0 }, -1)]);
  expect(list.map((f) => f.tone)).toEqual([2, 1, 0]);
  expect(list.map((f) => f.depth)).toEqual([0, 1.5, 2]);
  expect(list[2].quad[0]).toEqual(iso.project(0, 0, 1));
});

test("fit centres the faces in the frame and keeps their proportions", () => {
  const list = iso.faces([{ normal: { x: 0, y: 0, z: 1 }, verts: [{ x: 0, y: 0, z: 0 }, { x: 2, y: 0, z: 0 }, { x: 2, y: 1, z: 0 }, { x: 0, y: 1, z: 0 }] }]);
  const place = iso.fit(list, frame(10, 20, 200, 100));
  const pts = list[0].quad.map(place);
  const xs = pts.map((p) => p[0]);
  const ys = pts.map((p) => p[1]);
  expect([Math.min(...xs) + Math.max(...xs), Math.min(...ys) + Math.max(...ys)].map(Math.round)).toEqual([220, 140]);
  expect([Math.max(...xs) - Math.min(...xs), Math.max(...ys) - Math.min(...ys)]).toEqual([expect.closeTo(100 * Math.sqrt(3), 6), expect.closeTo(100, 6)]);
});

test("stamp paints faces back to front about a point and rings each in the edge", () => {
  const { pen, calls } = recorder();
  const face = (normal, z) => ({ normal, verts: [{ x: 0, y: 0, z }, { x: 1, y: 0, z }, { x: 1, y: 1, z }, { x: 0, y: 1, z }] });
  const quads = [face({ x: 0, y: 0, z: 1 }, 1), face({ x: 0, y: 1, z: 0 }, 0.5), face({ x: 0, y: 0, z: -1 }, 0)];
  const shade = [theme.fg, theme.blue, theme.dim];
  iso.stamp(pen, quads, 100, 50, 10, shade, theme.line, 3);
  const polygons = calls.filter(([key]) => key === "polygon");
  expect(polygons.map((call) => call[2])).toEqual([theme.blue, theme.fg]);
  expect(polygons[1][1][2]).toEqual([100 + iso.project(1, 1, 1)[0] * 10, 50 + iso.project(1, 1, 1)[1] * 10]);
  expect(calls.filter(([key]) => key === "polyline").map((call) => [call[1].length, call[2], call[3]])).toEqual([[5, 3, theme.line], [5, 3, theme.line]]);
});

test("stamp without an edge only fills", () => {
  const { pen, calls } = recorder();
  iso.stamp(pen, [{ normal: { x: 0, y: 0, z: 1 }, verts: [{ x: 0, y: 0, z: 1 }, { x: 1, y: 0, z: 1 }, { x: 1, y: 1, z: 1 }, { x: 0, y: 1, z: 1 }] }], 0, 0, 5, [theme.fg], null, 0);
  expect(calls.map(([key]) => key)).toEqual(["polygon"]);
});

test("cage strokes the twelve edges of a box about its centre in EDGES order", () => {
  const { pen, calls } = recorder();
  iso.cage(pen, 100, 50, 10, 2, 1.5, theme.dim);
  const corner = (i) => {
    const p = iso.project(((i & 1) * 2 - 1) * 2, (((i >> 1) & 1) * 2 - 1) * 2, (((i >> 2) & 1) * 2 - 1) * 2);
    return [100 + p[0] * 10, 50 + p[1] * 10];
  };
  expect(calls).toEqual(iso.EDGES.map(([a, b]) => ["segment", corner(a), corner(b), 1.5, theme.dim]));
  expect(calls[0][1]).toEqual([100, 50]);
});

// PATCH

test("a patch holds a color and a cover a pixel and paints them as one image", () => {
  const { pen, calls } = recorder();
  const patch = field.patch(4, 6, 3, 2);
  patch.blend(5, 7, theme.blue, 0.25);
  patch.blend(6, 6, theme.fg);
  patch.blend(3, 6, theme.fg, 1);
  patch.blend(7, 6, theme.fg, 1);
  patch.blend(5, 8, theme.fg, 1);
  patch.paint(pen);
  const [verb, x, y, w, h, pixels] = calls[0];
  expect([verb, x, y, w, h, pixels.shape]).toEqual(["image", 4, 6, 3, 2, [2, 3]]);
  expect(Array.from(pixels.cover)).toEqual([0, 0, 1, 0, 0.25, 0]);
  expect(Array.from(pixels.colors.slice(16, 20))).toEqual(theme.blue);
  expect(Array.from(pixels.colors.slice(8, 12))).toEqual(theme.fg);
  expect(Array.from(pixels.colors.slice(0, 4))).toEqual([0, 0, 0, 0]);
});

test("a patch wants a box of whole pixels", () => {
  for (const box of [[1.5, 0, 2, 2], [0, 0, 2, NaN], [0, 0, -1, 2], [0, 0, Infinity, 2]]) expect(() => field.patch(...box)).toThrow("whole pixels");
  expect(() => field.patch(-3, -2, 0, 0)).not.toThrow();
});

test("a patch reaches the raster as Board.blend reaches the board", () => {
  const pen = raster(8, 4, [0, 0, 0, 255]);
  const patch = field.patch(2, 1, 4, 2);
  patch.blend(2, 1, [255, 255, 255, 255], 0.5);
  patch.blend(3, 1, [255, 255, 255, 255], 2);
  patch.blend(4, 1, [255, 255, 255, 255], -1);
  patch.blend(2, 2, [255, 255, 255, 128], 0.5);
  patch.paint(pen);
  const { colors } = pen.pixels();
  const at = (x, y) => Array.from(colors.slice((y * 8 + x) * 4, (y * 8 + x) * 4 + 4));
  expect([at(2, 1), at(3, 1), at(4, 1), at(5, 1), at(2, 2)]).toEqual([[128, 128, 128, 255], [255, 255, 255, 255], [0, 0, 0, 255], [0, 0, 0, 255], [64, 64, 64, 255]]);
});

// PLOT

test("bars stand on the foot of the frame, the tallest filling it", () => {
  const { pen, calls } = recorder();
  plot.bars(pen, frame(0, 0, 40, 100), [1, -4, 2, 0], 0.5, theme.blue);
  expect(calls.map((call) => call.slice(1, 5))).toEqual([[2.5, 75, 5, 25], [12.5, 0, 5, 100], [22.5, 50, 5, 50], [32.5, 100, 5, 0]]);
});

test("a curve and a staircase span their frame", () => {
  const { pen, calls } = recorder();
  const box = frame(10, 10, 100, 50);
  plot.curve(pen, box, [0, 1, 2], [5, 9, 7], 2, theme.fg);
  plot.staircase(pen, box, [3, 1], 2, theme.fg);
  expect(calls[0][1]).toEqual([[10, 60], [60, 10], [110, 35]]);
  expect(calls[1][1]).toEqual([[10, 10], [60, 10], [60, 60], [110, 60]]);
});

test("a dashed rule lays forty eight dashes along one height", () => {
  const { pen, calls } = recorder();
  plot.dashed(pen, frame(10, 0, 96, 20), 7, 2.5, theme.fg);
  expect(calls.length).toBe(48);
  expect(calls.map((call) => call[1][0])).toEqual(Array.from({ length: 48 }, (_, i) => 10 + 2 * i));
  expect(calls[0][2][0]).toBeCloseTo(11.1, 9);
  expect(calls[47]).toMatchObject(["segment", [104, 7], [expect.any(Number), 7], 2.5, theme.fg]);
});

// FIELD

test("a flat field paints the ramp's low end everywhere", () => {
  const pen = raster(64, 64, theme.ground);
  field.sample(pen, frame(0, 0, 64, 64), 8, () => 1, theme.Ramp.tone(theme.blue, theme.yellow));
  expect(Array.from(pen.pixels().colors.slice(0, 4))).toEqual([...theme.blue.slice(0, 3), 255]);
});

test("a field paints the pixels field.rs paints", () => {
  const [w, h] = [7, 5];
  const values = Array.from({ length: w * h }, (_, i) => Math.sin(i) * 3);
  const box = frame(3.6, 2.2, 50.3, 41.7);
  const ramp = theme.Ramp.heat();
  const pen = raster(64, 64, theme.panel);
  field.draw(pen, box, w, h, values, ramp);
  const want = raster(64, 64, theme.panel);
  const lo = Math.min(...values);
  const reach = Math.max(...values) - lo;
  for (let py = Math.ceil(box.y); py < Math.floor(box.y + box.h); py++) {
    for (let px = Math.ceil(box.x); px < Math.floor(box.x + box.w); px++) {
      const col = Math.min(Math.floor(((px + 0.5 - box.x) / box.w) * w), w - 1);
      const row = Math.min(Math.floor(((py + 0.5 - box.y) / box.h) * h), h - 1);
      want.rect(px, py, 1, 1, ramp.at((values[row * w + col] - lo) / reach));
    }
  }
  expect(pen.pixels().colors).toEqual(want.pixels().colors);
});
