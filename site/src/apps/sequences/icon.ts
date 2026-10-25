import * as math from "@mrlyprod/mrly/math";
import type { Pen } from "#view";
import { plate } from "../../lib/plate.ts";

export const units = { math };

const CODE = "7";
const PINS = 8;
const GAP = 0.22;
const ROUND = 0.3;
const OCTAGONAL = [8, 21, 40, 65, 96, 133, 176, 225];

let heights: number[] | null = null;

function terms(): number[] {
  if (heights) return heights;
  const read = Array.from({ length: PINS }, (_, k) => Number(math.counts.fill(CODE, 2 * (k + 2) - 1, 2, 1, 2)));
  if (read.join() !== OCTAGONAL.join())
    throw new Error(
      `sequences: the carpet fills by odd side read ${read.join(", ")}, want the octagonal numbers ${OCTAGONAL.join(", ")}`,
    );
  heights = read.map((v) => v / read[PINS - 1]);
  return heights;
}

export default function draw(pen: Pen): void {
  const { box, paint } = plate(pen, "blue", 0.18);
  const tall = terms();
  const w = box.w / (PINS + (PINS - 1) * GAP);
  const floor = box.y + box.h;
  tall.forEach((t, k) => {
    const h = box.h * t;
    pen.round_rect(box.x + k * w * (1 + GAP), floor - h, w, h, w * ROUND, k === PINS - 1 ? paint.light : paint.white);
  });
}
