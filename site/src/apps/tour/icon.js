import * as math from "@mrlyprod/mrly/math";
import { Grid } from "#view";
import { plate } from "../../lib/plate.js";

export const units = { math };

const CODE = "7";
const LEVEL = 2;
const SIDE = 9;
const FILLS = 64;
const TILT = -0.2;
const CARD = [0.74, 0.92];
const ROUND = 0.1;
const INSET = 0.12;
const ART = 0.7;
const PINS = [0.25, 0.45, 0.7, 1];

let carpet = null;

function cells() {
  if (carpet) return carpet;
  const cell = math.two.create(CODE, 3, LEVEL, 0, 2);
  const fills = [...cell.types].filter(Boolean).length;
  if (cell.shape[0] !== SIDE || fills !== FILLS)
    throw new Error(
      `tour: the carpet at level ${LEVEL} is ${cell.shape[0]} wide and fills ${fills}, want ${SIDE} and ${FILLS}`,
    );
  carpet = { shape: cell.shape, types: cell.types };
  return carpet;
}

function turned(cx, cy, w, h, angle) {
  const [c, s] = [Math.cos(angle), Math.sin(angle)];
  return [
    [-w / 2, -h / 2],
    [w / 2, -h / 2],
    [w / 2, h / 2],
    [-w / 2, h / 2],
  ].map(([x, y]) => [cx + x * c - y * s, cy + x * s + y * c]);
}

export default function draw(pen) {
  const { box, paint } = plate(pen, "yellow", 0.16);
  const w = box.w * CARD[0];
  const h = box.h * CARD[1];
  const [cx, cy] = [box.x + box.w / 2, box.y + box.h / 2];
  pen.polygon(turned(cx - w * 0.08, cy, w, h, TILT), paint.light);
  pen.round_rect(cx - w / 2 + w * 0.06, cy - h / 2, w, h, w * ROUND, paint.white);
  const left = cx - w / 2 + w * 0.06;
  const side = w * ART;
  const art = { x: left + (w - side) / 2, y: cy - h / 2 + w * INSET, w: side, h: side };
  new Grid({ ...art, inset: () => art }, SIDE, SIDE, 0.08).paint(pen, cells(), (type) => (type ? paint.deep : null));
  const floor = cy + h / 2 - w * INSET;
  const room = floor - (art.y + side) - w * INSET * 0.7;
  const bar = side / (PINS.length * 2 - 1);
  for (const [k, t] of PINS.entries())
    pen.round_rect(art.x + k * 2 * bar, floor - room * t, bar, room * t, bar / 2, paint.tone);
}
