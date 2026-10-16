import * as math from "@mrlyprod/mrly/math";
import { plate } from "../../lib/icon.js";

export const units = { math };

const CODE = "126";
const LEVEL = 4;
const EACH = 81;
const INSET = 0.17;
const DOT = 0.44;

let cloud = null;

function facts() {
  if (cloud) return cloud;
  const counts = math.three.profile(CODE, 2, LEVEL, 2).map(Number);
  const [low, high] = math.three.support(counts);
  const mid = Math.floor((low + high) / 2);
  cloud = [mid, mid + 1].map((height) => {
    const cells = math.three.diagonal_slice(CODE, 2, LEVEL, 2, height);
    if (cells.length !== EACH) throw new Error(`cuts: height ${height} meets ${cells.length} cells, want ${EACH}`);
    return cells.map((cell) => math.three.project(cell));
  });
  return cloud;
}

export default function draw(pen) {
  const { box, paint } = plate(pen, "pink", INSET);
  const all = facts().flat();
  const xs = all.map(([x]) => x);
  const ys = all.map(([, y]) => y);
  const [lx, hx, ly, hy] = [Math.min(...xs), Math.max(...xs), Math.min(...ys), Math.max(...ys)];
  const scale = Math.min(box.w / (hx - lx + 2 * DOT), box.h / (hy - ly + 2 * DOT));
  const ox = box.x + (box.w - (hx - lx) * scale) / 2;
  const oy = box.y + (box.h - (hy - ly) * scale) / 2;
  const place = ([x, y]) => [ox + (x - lx) * scale, oy + (hy - y) * scale];
  const [one, two] = facts();
  for (const at of two) pen.disc(...place(at), DOT * scale, paint.light);
  for (const at of one) pen.disc(...place(at), DOT * scale, paint.white);
}
