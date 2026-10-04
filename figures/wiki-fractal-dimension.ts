import * as math from "mrlyjs/math";
import { Grid, frame as box, type Color, type Frame, type Ink, type Pen } from "mrlyjs/view";

export const units = { math };

const BOXES = 3;
const GUTTER = 52;
const SAMPLES = 1000;

function outline(pen: Pen, frame: Frame, thick: number, color: Color) {
  pen.rect(frame.x, frame.y, frame.w, thick, color);
  pen.rect(frame.x, frame.y + frame.h - thick, frame.w, thick, color);
  pen.rect(frame.x, frame.y, thick, frame.h, color);
  pen.rect(frame.x + frame.w - thick, frame.y, thick, frame.h, color);
}

export default function draw(pen: Pen, ink: Ink) {
  const area = pen.frame(0.08);
  const tile = (area.w - 2 * GUTTER) / 3;
  const top = area.y + (area.h - tile) / 2;
  const carpet = math.two.create(495, 3, 3, 0, 3);
  if (carpet.shape.length !== 2 || carpet.shape[0] !== 27 || carpet.shape[1] !== 27) throw new Error(`wiki-fractal-dimension: carpet shape ${carpet.shape}, want 27 square`);
  const side = carpet.shape[1];

  const lit = [0, 0, 0];
  for (let panel = 0; panel < lit.length; panel++) {
    const x = area.x + panel * (tile + GUTTER);
    const frame = box(x, top, tile, tile);
    const on = Array.from({ length: BOXES }, () => new Array<boolean>(BOXES).fill(false));
    if (panel === 0) {
      pen.rect(frame.x, frame.y, frame.w, frame.h, ink.blue);
      for (const line of on) line.fill(true);
    } else if (panel === 1) {
      new Grid(frame, side, side, 0).paint(pen, carpet, (kind) => (kind !== 0 ? ink.blue : null));
      const block = Math.trunc(side / BOXES);
      for (let row = 0; row < BOXES; row++) {
        for (let col = 0; col < BOXES; col++) {
          let any = false;
          for (let r = row * block; r < (row + 1) * block; r++) {
            for (let c = col * block; c < (col + 1) * block; c++) if (carpet.types[r * side + c] !== 0) any = true;
          }
          on[row][col] = any;
        }
      }
    } else {
      pen.segment([frame.x, frame.y + frame.h], [frame.x + frame.w, frame.y], tile * 0.03, ink.blue);
      for (let i = 0; i < SAMPLES; i++) {
        const u = (i + 0.5) / SAMPLES;
        const col = Math.trunc(u * BOXES);
        const row = Math.trunc((1 - u) * BOXES);
        on[Math.min(row, BOXES - 1)][Math.min(col, BOXES - 1)] = true;
      }
    }
    const step = tile / BOXES;
    for (let row = 0; row < BOXES; row++) {
      for (let col = 0; col < BOXES; col++) {
        const cell = box(frame.x + col * step, frame.y + row * step, step, step);
        outline(pen, cell, 1.5, ink.line);
        if (on[row][col]) {
          outline(pen, cell.inset(step * 0.07), 6, ink.yellow);
          lit[panel]++;
        }
      }
    }
  }
  if (lit[0] !== 9 || lit[1] !== 8 || lit[2] !== 3) throw new Error(`wiki-fractal-dimension: lit ${lit}, want 9,8,3`);
}
