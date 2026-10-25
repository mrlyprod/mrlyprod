import type { Box } from "../app/run.ts";
import type { Painted } from "./engine.ts";

/* TYPES */

export type Square = { x: number; y: number; side: number };

export type Pad = {
  ctx: CanvasRenderingContext2D;
  put: (image: Pick<Painted, "colors">, size: number) => void;
  clear: () => void;
  blit: (size: number, box: Square, smooth: boolean) => void;
};

export type Surface = {
  canvas: Pick<HTMLCanvasElement, "width">;
  imageSmoothingEnabled: boolean;
  putImageData(image: ImageData, dx: number, dy: number): void;
  drawImage(image: OffscreenCanvas, dx: number, dy: number, dw: number, dh: number): void;
};

const FIT = 0.94;

export function fit(w: number, h: number, fill = FIT): Square {
  const side = Math.max(1, Math.floor(Math.min(w, h) * fill));
  return { x: Math.floor((w - side) / 2), y: Math.floor((h - side) / 2), side };
}

export function shell(canvas: HTMLCanvasElement, view: Box): Pad {
  const ctx = canvas.getContext("2d")!;
  const sheets = new Map<number, OffscreenCanvas>();
  const sheet = (size: number) => {
    if (!sheets.has(size)) sheets.set(size, new OffscreenCanvas(size, size));
    return sheets.get(size)!;
  };
  const put = ({ colors }: Pick<Painted, "colors">, size: number) => {
    sheet(size)
      .getContext("2d")!
      .putImageData(
        new ImageData(new Uint8ClampedArray(colors.buffer, colors.byteOffset, colors.length), size, size),
        0,
        0,
      );
  };
  const clear = () => {
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, view.w, view.h);
  };
  const blit = (size: number, box: Square, smooth: boolean) => {
    ctx.imageSmoothingEnabled = smooth;
    ctx.drawImage(sheet(size), box.x, box.y, box.side, box.side);
  };
  return { ctx, put, clear, blit };
}

export function stamp(ctx: Surface, { colors }: Pick<Painted, "colors">, size: number) {
  const image = new ImageData(new Uint8ClampedArray(colors.buffer, colors.byteOffset, colors.length), size, size);
  const side = ctx.canvas.width;
  if (side === size) {
    ctx.putImageData(image, 0, 0);
    return;
  }
  const tile = new OffscreenCanvas(size, size);
  tile.getContext("2d")!.putImageData(image, 0, 0);
  ctx.imageSmoothingEnabled = true;
  ctx.drawImage(tile, 0, 0, side, side);
}
