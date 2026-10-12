import * as math from "mrlyjs/math";
import { plate } from "../../lib/icon.js";

export const units = { math };

const CODE = "7";
const LEVELS = 6;
const PERIMETER = [16, 80, 496, 3536, 26992, 212048];
const STEM = 0.22;
const HEAD = 0.4;
const LINE = 0.14;
const FLOOR = 0.12;

let heights = null;

function terms() {
  if (heights) return heights;
  const read = Array.from({ length: LEVELS }, (_, k) => Number(math.counts.exposure(CODE, 3, 2, k + 1, 2)));
  if (read.join() !== PERIMETER.join()) throw new Error(`plot: the carpet perimeter reads ${read.join(", ")}, want ${PERIMETER.join(", ")}`);
  const top = Math.log10(read[LEVELS - 1]);
  heights = read.map((v) => FLOOR + (1 - FLOOR) * (Math.log10(v) / top));
  return heights;
}

export default function draw(pen) {
  const { box, paint } = plate(pen, "pink", 0.2);
  const tall = terms();
  const slot = box.w / LEVELS;
  const r = slot * HEAD;
  const floor = box.y + box.h;
  const head = (k) => [box.x + (k + 0.5) * slot, floor - r - (box.h - 2 * r) * tall[k]];
  const ahead = [box.x + (LEVELS + 0.1) * slot, floor - r - (box.h - 2 * r) * (2 * tall[LEVELS - 1] - tall[LEVELS - 2])];
  const back = [box.x - 0.1 * slot, floor - r - (box.h - 2 * r) * (2 * tall[0] - tall[1])];
  pen.polyline([back, ...tall.map((_, k) => head(k)), ahead], slot * LINE, paint.light);
  tall.forEach((_, k) => {
    const [x, y] = head(k);
    pen.round_rect(x - (slot * STEM) / 2, y, slot * STEM, floor - y, (slot * STEM) / 2, paint.white);
    pen.disc(x, y, r, paint.white);
  });
}
