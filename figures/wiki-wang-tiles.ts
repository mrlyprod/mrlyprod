import type { Color, Ink, Pen } from "mrlyjs/view";
import census from "./census/wiki-wang-tiles.json" with { type: "json" };

const SIDE = 10;
const KINDS = 11;
const LEFT = 0;
const RIGHT = 1;
const BOTTOM = 2;
const TOP = 3;

export const units = {};

function hue(ink: Ink, colour: number): Color {
  switch (colour) {
    case 0:
      return ink.dim;
    case 1:
      return ink.blue;
    case 2:
      return ink.yellow;
    default:
      return ink.orange;
  }
}

export default function draw(pen: Pen, ink: Ink) {
  const { tiles, patch } = census;
  if (tiles.length !== KINDS || tiles.some((t) => t.length !== 4)) throw new Error(`wiki-wang-tiles: ${tiles.length} tiles, want ${KINDS} of 4 colours`);
  if (patch.length !== SIDE * SIDE || patch.some((k) => k < 0 || k >= KINDS)) throw new Error(`wiki-wang-tiles: patch of ${patch.length}, want ${SIDE * SIDE} tile indices below ${KINDS}`);
  const frame = pen.frame(0.08);
  const cell = frame.w / SIDE;
  const half = cell * 0.27;
  const deep = cell * 0.22;
  const pad = cell * 0.035;
  for (let row = 0; row < SIDE; row++) {
    for (let col = 0; col < SIDE; col++) {
      const t = tiles[patch[row * SIDE + col]];
      const x = frame.x + col * cell;
      const y = frame.y + row * cell;
      const cx = x + cell / 2;
      const cy = y + cell / 2;
      pen.round_rect(x + pad, y + pad, cell - 2 * pad, cell - 2 * pad, pad, ink.line);
      const top = y + pad;
      const bottom = y + cell - pad;
      const left = x + pad;
      const right = x + cell - pad;
      pen.triangle([cx - half, top], [cx + half, top], [cx, top + deep], hue(ink, t[TOP]));
      pen.triangle([cx - half, bottom], [cx + half, bottom], [cx, bottom - deep], hue(ink, t[BOTTOM]));
      pen.triangle([left, cy - half], [left, cy + half], [left + deep, cy], hue(ink, t[LEFT]));
      pen.triangle([right, cy - half], [right, cy + half], [right - deep, cy], hue(ink, t[RIGHT]));
    }
  }
}
