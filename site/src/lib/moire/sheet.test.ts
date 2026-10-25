import { afterAll, expect, test } from "bun:test";
import { stamp } from "./sheet.ts";

class FakeImage {
  data: Uint8ClampedArray;
  width: number;
  height: number;
  constructor(data: Uint8ClampedArray, width: number, height: number) {
    this.data = data;
    this.width = width;
    this.height = height;
  }
}

class FakeCanvas {
  width: number;
  height: number;
  image?: FakeImage;
  constructor(width: number, height: number) {
    this.width = width;
    this.height = height;
  }
  getContext() {
    return { putImageData: (image: FakeImage) => (this.image = image) };
  }
}

const kept = { OffscreenCanvas: globalThis.OffscreenCanvas, ImageData: globalThis.ImageData };

Object.assign(globalThis, { ImageData: FakeImage, OffscreenCanvas: FakeCanvas });

afterAll(() => Object.assign(globalThis, kept));

const pen = (side: number) => {
  const log: (string | number)[][] = [];
  return {
    log,
    canvas: { width: side },
    imageSmoothingEnabled: false,
    putImageData: (image: ImageData, x: number, y: number) => log.push(["put", image.width, x, y]),
    drawImage: (tile: OffscreenCanvas & { image?: FakeImage }, x: number, y: number, w: number, h: number) =>
      log.push(["draw", tile.width, tile.image!.width, x, y, w, h]),
  };
};

test("a stamp puts a field of colours on a canvas of its own side, and scales it smooth onto any other", () => {
  const colors = new Uint8Array(4 * 4 * 4);
  const same = pen(4);
  stamp(same, { colors }, 4);
  const wide = pen(8);
  stamp(wide, { colors }, 4);
  expect([same.log, wide.log, wide.imageSmoothingEnabled]).toEqual([
    [["put", 4, 0, 0]],
    [["draw", 4, 4, 0, 0, 8, 8]],
    true,
  ]);
});
