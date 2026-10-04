import { expect, test } from "bun:test";
import { svg } from "./index.js";

const ORANGE = [255, 143, 44, 255];
const pixels = { shape: [1, 2], colors: Uint8Array.from([255, 0, 0, 255, 0, 0, 255, 128]) };

async function body(draw, options) {
  const pen = svg(64, 48, [0, 0, 0, 0]);
  draw(pen);
  const text = await pen.text(options);
  return text.split("\n").slice(1, -2).join("\n");
}

// VERBS

test("svg opens on its size and floods its ground", async () => {
  const text = await svg(64, 48, [17, 17, 18, 255]).text();
  expect(text).toStartWith('<svg xmlns="http://www.w3.org/2000/svg" width="64" height="48" viewBox="0 0 64 48">\n');
  expect(text).toContain('<rect width="64" height="48" fill="#111112"/>');
});

test("rect is a rect and alpha is its opacity", async () => {
  expect(await body((p) => p.rect(1, 2.5, 3, 4, ORANGE))).toBe('<rect x="1" y="2.5" width="3" height="4" fill="#ff8f2c"/>');
  expect(await body((p) => p.rect(1, 2, 3, 4, [255, 143, 44, 51]))).toBe('<rect x="1" y="2" width="3" height="4" fill="#ff8f2c" fill-opacity="0.2"/>');
});

test("round_rect clamps its radius to half the short side", async () => {
  expect(await body((p) => p.round_rect(1, 2, 10, 20, 50, ORANGE))).toBe('<rect x="1" y="2" width="10" height="20" rx="5" ry="5" fill="#ff8f2c"/>');
});

test("disc is a filled circle", async () => {
  expect(await body((p) => p.disc(30, 20, 9.5, ORANGE))).toBe('<circle cx="30" cy="20" r="9.5" fill="#ff8f2c"/>');
});

test("ring is a stroked circle", async () => {
  expect(await body((p) => p.ring(30, 20, 9, 2, [255, 143, 44, 128]))).toBe('<circle cx="30" cy="20" r="9" fill="none" stroke="#ff8f2c" stroke-opacity="0.5019607843137255" stroke-width="2"/>');
});

test("segment is a line with round caps", async () => {
  expect(await body((p) => p.segment([1, 2], [30, 40], 3, ORANGE))).toBe('<line x1="1" y1="2" x2="30" y2="40" fill="none" stroke="#ff8f2c" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"/>');
  expect(await body((p) => p.segment([5, 6], [5, 6], 4, ORANGE))).toBe('<circle cx="5" cy="6" r="2" fill="#ff8f2c"/>');
});

test("polyline is one stroke with round caps and joints", async () => {
  expect(await body((p) => p.polyline([[1, 2], [3, 4], [5, 6]], 2, ORANGE))).toBe('<polyline points="1,2 3,4 5,6" fill="none" stroke="#ff8f2c" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>');
});

test("triangle is a three point polygon", async () => {
  expect(await body((p) => p.triangle([1, 2], [3, 4], [5, 0], ORANGE))).toBe('<polygon points="1,2 3,4 5,0" fill="#ff8f2c" fill-rule="evenodd"/>');
});

test("polygon fills by the even-odd rule", async () => {
  expect(await body((p) => p.polygon([[0, 0], [10, 0], [10, 10], [0, 10]], ORANGE))).toBe('<polygon points="0,0 10,0 10,10 0,10" fill="#ff8f2c" fill-rule="evenodd"/>');
});

test("arc sweeps clockwise in two arms, a full turn is a ring and none is a dot", async () => {
  expect(await body((p) => p.arc([20, 20], 10, [Math.PI / 2, 0], 2, ORANGE))).toBe(`<path d="M30,20 A10,10 0 0 1 ${20 + 10 * Math.cos(Math.PI / 4)},${20 + 10 * Math.sin(Math.PI / 4)} A10,10 0 0 1 ${20 + 10 * Math.cos(Math.PI / 2)},30" fill="none" stroke="#ff8f2c" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>`);
  expect(await body((p) => p.arc([20, 20], 10, [0, 7], 2, ORANGE))).toBe('<circle cx="20" cy="20" r="10" fill="none" stroke="#ff8f2c" stroke-width="2"/>');
  expect(await body((p) => p.arc([20, 20], 10, [0, 0], 2, ORANGE))).toBe('<circle cx="30" cy="20" r="1" fill="#ff8f2c"/>');
});

test("image embeds the png its encoder returns, nearest sampled", async () => {
  const draw = (p) => p.image(4, 8, 32, 16, pixels);
  await expect(body(draw)).rejects.toThrow("png encoder");
  const bytes = await body(draw, { png: async () => Uint8Array.from([137, 80, 78, 71]) });
  expect(bytes).toBe('<image x="4" y="8" width="32" height="16" preserveAspectRatio="none" image-rendering="pixelated" href="data:image/png;base64,iVBORw=="/>');
  const url = await body(draw, { png: (seen) => `data:${seen.shape.join("x")}` });
  expect(url).toContain('href="data:1x2"');
});

test("image keeps the pixels it was handed at the time it was drawn", async () => {
  const mine = { shape: [1, 1], colors: Uint8Array.from([1, 2, 3, 4]) };
  const pen = svg(8, 8);
  pen.image(0, 0, 8, 8, mine);
  mine.colors[0] = 99;
  let seen;
  await pen.text({ png: (p) => ((seen = Array.from(p.colors)), "data:") });
  expect(seen).toEqual([1, 2, 3, 4]);
});

test("image folds its cover into the alpha it hands the encoder", async () => {
  const pen = svg(8, 8);
  pen.image(0, 0, 2, 1, { shape: [1, 2], colors: Uint8Array.from([255, 0, 0, 255, 0, 0, 255, 128]), cover: [0.5, 2] });
  let seen;
  await pen.text({ png: (p) => ((seen = p.colors), "data:") });
  expect(seen).toBeInstanceOf(Uint8Array);
  expect(Array.from(seen)).toEqual([255, 0, 0, 128, 0, 0, 255, 128]);
});
