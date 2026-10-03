import { expect, test } from "bun:test";
import { dark } from "../../../site/kit/theme/theme.js";
import { ink, raster } from "./index.js";

const BLACK = [0, 0, 0, 255];
const WHITE = [255, 255, 255, 255];
const theme = ink(dark);
const red = (pen, x, y) => pen.pixels().colors[(y * pen.width + x) * 4];
const at = (pen, x, y) => Array.from(pen.pixels().colors.slice((y * pen.width + x) * 4, (y * pen.width + x) * 4 + 4));

function board(draw) {
  const pen = raster(64, 64, BLACK);
  draw(pen);
  return pen;
}

function lit(pen) {
  const { colors } = pen.pixels();
  let sum = 0;
  for (let i = 0; i < colors.length; i += 4) sum += (colors[i] - theme.ground[0]) / (theme.fg[0] - theme.ground[0]);
  return sum;
}

// VERBS

test("rect feathers its edge by the signed distance", () => {
  const pen = board((p) => p.rect(10.25, 8, 20, 10, WHITE));
  expect([red(pen, 20, 12), red(pen, 10, 12), red(pen, 9, 12), red(pen, 30, 12)]).toEqual([255, 191, 0, 64]);
});

test("round_rect cuts its corner on the circle of its radius", () => {
  const pen = board((p) => p.round_rect(8.5, 8.5, 40, 40, 10, WHITE));
  expect([red(pen, 12, 10), red(pen, 8, 8), red(pen, 28, 9)]).toEqual([128, 0, 255]);
});

test("disc covers inside, feathers its rim and leaves outside", () => {
  const pen = board((p) => p.disc(32.25, 32.5, 10, WHITE));
  expect([red(pen, 32, 32), red(pen, 41, 32), red(pen, 42, 32), red(pen, 43, 32)]).toEqual([255, 255, 64, 0]);
});

test("a tie rounds half away from zero as Rust does", () => {
  const pen = board((p) => p.disc(32.5, 32.5, 10, [253, 253, 253, 255]));
  expect(red(pen, 42, 32)).toBe(127);
});

test("ring strokes about its radius and leaves its centre", () => {
  const pen = board((p) => p.ring(32.25, 32.5, 10, 2, WHITE));
  expect([red(pen, 42, 32), red(pen, 43, 32), red(pen, 21, 32), red(pen, 32, 32)]).toEqual([255, 64, 191, 0]);
});

test("segment strokes its width with round caps", () => {
  const pen = board((p) => p.segment([10, 20.5], [50.25, 20.5], 4, WHITE));
  expect([red(pen, 30, 20), red(pen, 30, 22), red(pen, 30, 23), red(pen, 52, 20)]).toEqual([255, 128, 0, 64]);
});

test("polyline blends its joint once where two segments blend twice", () => {
  const glass = [255, 255, 255, 128];
  const pts = [[10, 20.5], [40.5, 20.5], [40.5, 50]];
  const one = board((p) => p.polyline(pts, 4, glass));
  const two = board((p) => {
    p.segment(pts[0], pts[1], 4, glass);
    p.segment(pts[1], pts[2], 4, glass);
  });
  expect([red(one, 40, 20), red(two, 40, 20)]).toEqual([128, 192]);
});

test("triangle feathers its edge by the signed distance", () => {
  const pen = board((p) => p.triangle([10, 10.25], [50, 10.25], [10, 50], WHITE));
  expect([red(pen, 20, 20), red(pen, 12, 10), red(pen, 12, 9)]).toEqual([255, 191, 0]);
});

test("polygon fills by the even-odd rule", () => {
  const star = [0, 1, 2, 3, 4].map((i) => [32 + 28 * Math.cos(-Math.PI / 2 + (i * 4 * Math.PI) / 5), 32 + 28 * Math.sin(-Math.PI / 2 + (i * 4 * Math.PI) / 5)]);
  const pen = board((p) => p.polygon(star, WHITE));
  expect([red(pen, 31, 12), red(pen, 32, 32)]).toEqual([255, 0]);
});

test("arc strokes its sweep alone with round caps", () => {
  const pen = board((p) => p.arc([32, 32.5], 10, [0, Math.PI / 2], 2, WHITE));
  expect([red(pen, 42, 32), red(pen, 38, 39), red(pen, 42, 31), red(pen, 21, 32)]).toEqual([255, 255, 97, 0]);
});

test("image paints whole pixels nearest sampled and blends its alpha", () => {
  const pixels = { shape: [2, 2], colors: Uint8Array.from([255, 0, 0, 255, 0, 255, 0, 255, 0, 0, 255, 255, 255, 255, 255, 128]) };
  const pen = board((p) => p.image(8, 8, 4, 4, pixels));
  expect([at(pen, 8, 8), at(pen, 11, 8), at(pen, 8, 11), at(pen, 10, 10)]).toEqual([[255, 0, 0, 255], [0, 255, 0, 255], [0, 0, 255, 255], [128, 128, 128, 255]]);
  const shifted = board((p) => p.image(8.5, 8, 4, 4, pixels));
  expect([red(shifted, 8, 8), red(shifted, 9, 8), at(shifted, 11, 8), red(shifted, 12, 8)]).toEqual([0, 255, [0, 255, 0, 255], 0]);
});

// BOARD

test("frame cells tile the frame exactly", () => {
  const frame = raster(1024, 1024, theme.ground).frame(0.08);
  expect(Math.abs(frame.cell(81) * 81 - frame.w)).toBeLessThan(1e-9);
});

test("a disc covers its own area", () => {
  const pen = raster(256, 256, theme.ground);
  pen.disc(128, 128, 90, theme.fg);
  const want = Math.PI * 90 * 90;
  expect(Math.abs(lit(pen) - want) / want).toBeLessThan(0.02);
});

test("a polyline covers its stroke area", () => {
  const pen = raster(512, 512, theme.ground);
  const thick = 6;
  const pts = Array.from({ length: 1000 }, (_, i) => {
    const x = 6 + (500 * i) / 999;
    return [x, 256 + 100 * Math.sin((2 * Math.PI * x) / 250)];
  });
  pen.polyline(pts, thick, theme.fg);
  let length = 0;
  for (let i = 1; i < pts.length; i++) length += Math.sqrt((pts[i][0] - pts[i - 1][0]) ** 2 + (pts[i][1] - pts[i - 1][1]) ** 2);
  const want = length * thick + Math.PI * (thick / 2) ** 2;
  expect(Math.abs(lit(pen) - want) / want).toBeLessThan(0.03);
});

test("a two point polyline is a segment", () => {
  const [a, b] = [[17.3, 40.9], [190.7, 123.4]];
  const one = raster(256, 192, theme.ground);
  one.segment(a, b, 7, theme.fg);
  const two = raster(256, 192, theme.ground);
  two.polyline([a, b], 7, theme.fg);
  expect(two.pixels().colors).toEqual(one.pixels().colors);
});

test("the pixels are rows of rgba, shape height then width", () => {
  const { shape, colors } = raster(1200, 630, theme.ground).pixels();
  expect(shape).toEqual([630, 1200]);
  expect(colors).toBeInstanceOf(Uint8ClampedArray);
  expect(colors.length).toBe(630 * 1200 * 4);
});
