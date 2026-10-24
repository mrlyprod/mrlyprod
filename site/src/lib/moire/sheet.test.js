import { afterAll, expect, test } from "bun:test";
import { stamp } from "./sheet.js";

const kept = { OffscreenCanvas: globalThis.OffscreenCanvas, ImageData: globalThis.ImageData };

globalThis.ImageData = class {
  constructor(data, width, height) {
    this.data = data;
    this.width = width;
    this.height = height;
  }
};

globalThis.OffscreenCanvas = class {
  constructor(width, height) {
    this.width = width;
    this.height = height;
  }
  getContext() {
    return { putImageData: (image) => (this.image = image) };
  }
};

afterAll(() => Object.assign(globalThis, kept));

const pen = (side) => {
  const log = [];
  return {
    log,
    canvas: { width: side },
    putImageData: (image, x, y) => log.push(["put", image.width, x, y]),
    drawImage: (tile, x, y, w, h) => log.push(["draw", tile.width, tile.image.width, x, y, w, h]),
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
