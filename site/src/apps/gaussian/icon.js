import * as num from "@mrlyprod/mrly/num";
import { mix } from "#view";
import { plate } from "../../lib/plate.js";

export const units = { num };

const REACH = 7;
const SIDE = 2 * REACH + 1;
const DOT = 0.54;
const LINK = 0.58;
const WANT = { points: 225, split: 64, inert: 8, ramified: 4, units: 4 };
const CLASSES = { Composite: 0, Split: 1, Inert: 2, Ramified: 3, Unit: 4, Zero: 5 };

let fates = null;

function facts() {
  if (fates) return fates;
  const window = new num.gauss.Window("Gaussian", REACH);
  const census = window.census();
  const grid = new Uint8Array(SIDE * SIDE);
  const seen = [0, 0, 0, 0, 0, 0];
  for (let b = -REACH; b <= REACH; b++) {
    for (let a = -REACH; a <= REACH; a++) {
      const fate = CLASSES[window.class(a, b)];
      grid[(REACH - b) * SIDE + (a + REACH)] = fate;
      seen[fate]++;
    }
  }
  window.free();
  const read = { points: census.points, split: seen[1], inert: seen[2], ramified: seen[3], units: seen[4] };
  const got = Object.values(read).join();
  if (got !== Object.values(WANT).join() || census.split !== seen[1] || census.inert !== seen[2]) throw new Error(`gaussian: the window of ${REACH} reads ${got}, want ${Object.values(WANT).join()}`);
  fates = grid;
  return grid;
}

function prime(grid, a, b) {
  if (Math.abs(a) > REACH || Math.abs(b) > REACH || Math.hypot(a, b) > REACH + 0.5) return false;
  const fate = grid[(REACH - b) * SIDE + (a + REACH)];
  return fate >= 1 && fate <= 3;
}

export default function draw(pen) {
  const { box, paint } = plate(pen, "green", 0.15);
  const grid = facts();
  const cell = box.w / SIDE;
  const [cx, cy] = box.center();
  const at = (a, b) => [cx + a * cell, cy - b * cell];
  const tone = (a, b) => mix(paint.white, paint.light, (Math.hypot(a, b) / REACH) ** 3);
  for (let b = -REACH; b <= REACH; b++) {
    for (let a = -REACH; a <= REACH; a++) {
      if (!prime(grid, a, b)) continue;
      for (const s of [1, -1]) if (prime(grid, a + 1, b + s)) pen.segment(at(a, b), at(a + 1, b + s), cell * LINK, tone(a + 0.5, b + s / 2));
    }
  }
  for (let b = -REACH; b <= REACH; b++) {
    for (let a = -REACH; a <= REACH; a++) {
      if (!prime(grid, a, b)) continue;
      const [x, y] = at(a, b);
      pen.disc(x, y, cell * DOT, tone(a, b));
    }
  }
}
