import { afterAll, beforeAll, expect, test } from "bun:test";
import type { Color, Pen, Pixels } from "./index.ts";
import { canvas } from "./index.ts";

const ORANGE: Color = [255, 143, 44, 255];
const STROKE = [
  ["strokeStyle", "rgba(255,143,44,1)"],
  ["lineWidth", 2],
  ["lineCap", "round"],
  ["lineJoin", "round"],
  ["stroke"],
];
const FILL = [
  ["fillStyle", "rgba(255,143,44,1)"],
  ["fill", "evenodd"],
];
const globals: { OffscreenCanvas?: typeof OffscreenCanvas; ImageData?: typeof ImageData } = {};

class Surface {
  static made = 0;
  width: number;
  height: number;
  image: Image | undefined;

  constructor(width: number, height: number) {
    this.width = width;
    this.height = height;
    Surface.made++;
  }

  getContext() {
    return { putImageData: (image: Image) => (this.image = image) };
  }
}

class Image {
  data: Uint8ClampedArray;
  w: number;
  h: number;

  constructor(data: Uint8ClampedArray, w: number, h: number) {
    this.data = data;
    this.w = w;
    this.h = h;
  }
}

beforeAll(() => {
  globals.OffscreenCanvas = globalThis.OffscreenCanvas;
  globals.ImageData = globalThis.ImageData;
  Object.assign(globalThis, { OffscreenCanvas: Surface, ImageData: Image });
});

afterAll(() => {
  Object.assign(globalThis, { OffscreenCanvas: globals.OffscreenCanvas, ImageData: globals.ImageData });
});

function record(draw: (pen: Pen) => void): unknown[][] {
  const calls: unknown[][] = [];
  const state: Record<PropertyKey, unknown> = { imageSmoothingEnabled: true };
  const ctx = new Proxy({} as CanvasRenderingContext2D, {
    get: (_, key) => (key in state ? state[key] : (...args: unknown[]) => calls.push([key, ...args])),
    set: (_, key, value) => {
      state[key] = value;
      calls.push([key, value]);
      return true;
    },
  });
  const pen = canvas(ctx, 64, 48, [0, 0, 0, 0]);
  calls.length = 0;
  draw(pen);
  return calls;
}

function drawn(call: unknown[]): { surface: Surface; image: Image } {
  const surface = call[1];
  if (!(surface instanceof Surface) || !surface.image) throw new Error("canvas: drawImage was not handed a surface");
  return { surface, image: surface.image };
}

/* VERBS */

test("canvas clears and floods its ground", () => {
  const calls: unknown[][] = [];
  const ctx = new Proxy({} as CanvasRenderingContext2D, {
    get:
      (_, key) =>
      (...args: unknown[]) =>
        calls.push([key, ...args]),
    set: (_, key, value) => {
      calls.push([key, value]);
      return true;
    },
  });
  canvas(ctx, 64, 48, [17, 17, 18, 255]);
  expect(calls).toEqual([
    ["clearRect", 0, 0, 64, 48],
    ["fillStyle", "rgba(17,17,18,1)"],
    ["fillRect", 0, 0, 64, 48],
  ]);
});

test("rect is fillRect", () => {
  expect(record((p) => p.rect(1, 2, 3, 4, [255, 143, 44, 51]))).toEqual([
    ["fillStyle", "rgba(255,143,44,0.2)"],
    ["fillRect", 1, 2, 3, 4],
  ]);
});

test("round_rect is roundRect with its radius clamped", () => {
  expect(record((p) => p.round_rect(1, 2, 10, 20, 50, ORANGE))).toEqual([
    ["beginPath"],
    ["roundRect", 1, 2, 10, 20, 5],
    ...FILL,
  ]);
});

test("disc is a filled full arc", () => {
  expect(record((p) => p.disc(30, 20, 9, ORANGE))).toEqual([
    ["beginPath"],
    ["arc", 30, 20, 9, 0, 2 * Math.PI],
    ...FILL,
  ]);
});

test("ring is a stroked full arc", () => {
  expect(record((p) => p.ring(30, 20, 9, 2, ORANGE))).toEqual([
    ["beginPath"],
    ["arc", 30, 20, 9, 0, 2 * Math.PI],
    ...STROKE,
  ]);
});

test("segment is a stroked line with round caps", () => {
  expect(record((p) => p.segment([1, 2], [30, 40], 2, ORANGE))).toEqual([
    ["beginPath"],
    ["moveTo", 1, 2],
    ["lineTo", 30, 40],
    ...STROKE,
  ]);
});

test("polyline is one stroked path", () => {
  expect(
    record((p) =>
      p.polyline(
        [
          [1, 2],
          [3, 4],
          [5, 6],
        ],
        2,
        ORANGE,
      ),
    ),
  ).toEqual([["beginPath"], ["moveTo", 1, 2], ["lineTo", 3, 4], ["lineTo", 5, 6], ...STROKE]);
});

test("triangle is a closed filled path", () => {
  expect(record((p) => p.triangle([1, 2], [3, 4], [5, 0], ORANGE))).toEqual([
    ["beginPath"],
    ["moveTo", 1, 2],
    ["lineTo", 3, 4],
    ["lineTo", 5, 0],
    ["closePath"],
    ...FILL,
  ]);
});

test("polygon fills by the even-odd rule", () => {
  const calls = record((p) =>
    p.polygon(
      [
        [0, 0],
        [10, 0],
        [10, 10],
        [0, 10],
      ],
      ORANGE,
    ),
  );
  expect(calls.at(-1)).toEqual(["fill", "evenodd"]);
  expect(calls.filter(([key]) => key === "lineTo").length).toBe(3);
});

test("arc is a clockwise stroked arc from its lower angle", () => {
  expect(record((p) => p.arc([20, 20], 10, [2, 0.5], 2, ORANGE))).toEqual([
    ["beginPath"],
    ["arc", 20, 20, 10, 0.5, 2],
    ...STROKE,
  ]);
});

test("image draws the pixels unsmoothed onto its box", () => {
  const pixels: Pixels = { shape: [1, 2], colors: Uint8Array.from([255, 0, 0, 255, 0, 0, 255, 128]) };
  const calls = record((p) => p.image(4, 8, 32, 16, pixels));
  expect(calls.map(([key]) => key)).toEqual(["imageSmoothingEnabled", "drawImage", "imageSmoothingEnabled"]);
  expect(calls[0][1]).toBe(false);
  const { surface, image } = drawn(calls[1]);
  expect(calls[1].slice(2)).toEqual([4, 8, 32, 16]);
  expect([surface.width, surface.height]).toEqual([2, 1]);
  expect(image.data).toBeInstanceOf(Uint8ClampedArray);
  expect(Array.from(image.data)).toEqual(Array.from(pixels.colors));
  expect(calls[2][1]).toBe(true);
});

test("image folds its cover into the alpha of the surface it draws", () => {
  const pixels: Pixels = { shape: [1, 2], colors: Uint8Array.from([255, 0, 0, 255, 0, 0, 255, 128]), cover: [0.5, 2] };
  const calls = record((p) => p.image(4, 8, 32, 16, pixels));
  const { image } = drawn(calls[1]);
  expect(image.data).toBeInstanceOf(Uint8ClampedArray);
  expect(Array.from(image.data)).toEqual([255, 0, 0, 128, 0, 0, 255, 128]);
  expect(Array.from(pixels.colors)).toEqual([255, 0, 0, 255, 0, 0, 255, 128]);
});

test("image paints every field through one surface, resized when the shape changes", () => {
  const made = Surface.made;
  const draw = (shape: [number, number]) =>
    drawn(record((p) => p.image(0, 0, 8, 8, { shape, colors: new Uint8Array(shape[0] * shape[1] * 4) }))[1]);
  const [a, b, c] = [draw([2, 3]), draw([2, 3]), draw([5, 4])];
  expect(Surface.made - made).toBeLessThanOrEqual(1);
  expect([a.surface === b.surface, b.surface === c.surface]).toEqual([true, true]);
  expect([c.surface.width, c.surface.height, c.image.w, c.image.h]).toEqual([4, 5, 4, 5]);
});
