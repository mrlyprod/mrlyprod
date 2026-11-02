import type { Color } from "./ink.ts";
import type { Pixels } from "./pixels.ts";

/* TYPES */

export type Point = [number, number];

export type Box = [number, number, number, number];

export type Board = {
  readonly width: number;
  readonly height: number;
  frame(margin: number): Frame;
  area(margin: number): Frame;
};

export type Pen = Board & {
  rect(x: number, y: number, w: number, h: number, c: Color): void;
  round_rect(x: number, y: number, w: number, h: number, r: number, c: Color): void;
  disc(cx: number, cy: number, r: number, c: Color): void;
  ring(cx: number, cy: number, r: number, thick: number, c: Color): void;
  segment(a: Point, b: Point, thick: number, c: Color): void;
  polyline(pts: Point[], thick: number, c: Color): void;
  triangle(a: Point, b: Point, c: Point, color: Color): void;
  polygon(pts: Point[], c: Color): void;
  arc(center: Point, r: number, angles: [number, number], thick: number, c: Color): void;
  image(x: number, y: number, w: number, h: number, pixels: Pixels): void;
};

/* FRAME */

class Frame {
  x: number;
  y: number;
  w: number;
  h: number;

  constructor(x: number, y: number, w: number, h: number) {
    this.x = x;
    this.y = y;
    this.w = w;
    this.h = h;
  }

  inset(px: number): Frame {
    return new Frame(this.x + px, this.y + px, this.w - 2 * px, this.h - 2 * px);
  }

  cell(n: number): number {
    return this.w / n;
  }

  at(u: number, v: number): Point {
    return [this.x + u * this.w, this.y + v * this.h];
  }

  center(): Point {
    return this.at(0.5, 0.5);
  }

  square(): Frame {
    const side = Math.min(this.w, this.h);
    return new Frame(this.x + (this.w - side) / 2, this.y + (this.h - side) / 2, side, side);
  }

  radius(): number {
    return Math.min(this.w, this.h) / 2;
  }

  cols(n: number): Frame[] {
    const w = this.w / n;
    return Array.from({ length: n }, (_, i) => new Frame(this.x + i * w, this.y, w, this.h));
  }

  rows(n: number): Frame[] {
    const h = this.h / n;
    return Array.from({ length: n }, (_, i) => new Frame(this.x, this.y + i * h, this.w, h));
  }

  panels(rows: number, cols: number, gap: number): Frame[] {
    return this.rows(rows)
      .flatMap((row) => row.cols(cols))
      .map((cell) => cell.inset(gap));
  }
}

export type { Frame };

export function frame(x: number, y: number, w: number, h: number): Frame {
  return new Frame(x, y, w, h);
}

/* BOARD */

export const CLEAR = Object.freeze<Color>([0, 0, 0, 0]) as Color;

export function board(width: number, height: number): Board {
  return {
    width,
    height,
    frame(margin) {
      const side = Math.min(width, height) * (1 - 2 * margin);
      return new Frame((width - side) / 2, (height - side) / 2, side, side);
    },
    area(margin) {
      return new Frame(0, 0, width, height).inset(Math.min(width, height) * margin);
    },
  };
}
