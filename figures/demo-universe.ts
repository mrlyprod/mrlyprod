import * as core from "mrlyjs/core";
import * as math from "mrlyjs/math";
import { iso } from "mrlyjs/view";
import type { Ink, Pen } from "mrlyjs/view";

export const units = { core, math };

const NUMBER = 3;
const ROWS = [4, 5, 4, 5, 4];
const HALF = 1.5;
const RATIO = 0.8660254037844386;

export default function draw(pen: Pen, ink: Ink) {
  const frame = pen.frame(0.08);
  const gallery = math.bang.bang(3).canonical();
  if (gallery.length !== 22) throw new Error(`demo-universe: ${gallery.length} designs, want 22`);
  const seats = ROWS.reduce((a, b) => a + b, 0);
  if (seats !== gallery.length) throw new Error(`demo-universe: rows hold ${seats}, want ${gallery.length}`);
  const shade = core.colors.shades(core.colors.INDIGO());
  const wire = ink.mix(ink.line, ink.dim, 0.3);
  const [mx] = frame.center();
  const pitch = [frame.w / 5, frame.h / 5];
  const s = Math.min((pitch[0] * 0.46) / (3 * RATIO), (pitch[1] * 0.46) / 3);
  let taken = 0;
  ROWS.forEach((count, r) => {
    const cy = frame.y + (r + 0.5) * pitch[1];
    for (let j = 0; j < count; j++) {
      const design = gallery[taken];
      const cx = mx + (j - (count - 1) / 2) * pitch[0];
      const solid = math.three.create(design.i, NUMBER, 1, 2);
      if (solid.shape.length !== 3 || solid.shape.some((n) => n !== 3)) throw new Error(`demo-universe: design ${design.i} is ${solid.shape}, want 3 cubed`);
      const fills = math.three.fills(solid);
      const want = math.bang.baseq.fill_from_corners(design.rule(), NUMBER, design.dimension);
      if (String(fills) !== want) throw new Error(`demo-universe: design ${design.i} fills ${fills}, want ${want}`);
      iso.cage(pen, cx, cy, s, HALF, s / 24, wire);
      iso.stamp(pen, math.three.quads(solid), cx, cy, s, shade, ink.ground, s / 18);
      taken++;
    }
  });
  if (taken !== 22) throw new Error(`demo-universe: drew ${taken}, want 22`);
}
