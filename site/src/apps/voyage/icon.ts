import * as math from "@mrlyprod/mrly/math";
import type { Color, Frame, Pen, Pixels } from "#view";
import { frame, iso } from "#view";
import { plate } from "../../lib/plate.ts";

/* TYPES */

type Arc = { x0: number; y0: number; w: number; h: number; top: number; image: Pixels };

export const units = { math };

const CODE = 23;
const NUMBER = 3;
const LEVEL = 2;
const BASE = 2;
const WANT = { fills: 400 };
const SHIP = { top: 0.05, side: 0.58 };
const WORLD = { top: 0.71, radius: 0.72, span: 0.68, rim: 0.045, glow: 0.28, haze: 1, taper: 0.75 };

let quads: math.three.Quad[] | null = null;
const worlds = new Map<string, Arc>();

function ship(): math.three.Quad[] {
  if (quads) return quads;
  const cube = math.three.create(CODE, NUMBER, LEVEL, BASE);
  const fills = math.three.fills(cube);
  if (fills !== WANT.fills) throw new Error(`voyage: the ship fills ${fills}, want ${WANT.fills}`);
  quads = math.three.quads(cube);
  return quads;
}

function world(box: Frame, light: Color): Arc {
  const key = `${box.x}:${box.w}:${light.join()}`;
  if (worlds.has(key)) return worlds.get(key)!;
  const r = WORLD.radius * box.w;
  const top = box.y + WORLD.top * box.h;
  const cx = box.x + box.w / 2;
  const cy = top + r;
  const rim = WORLD.rim * box.w;
  const glow = WORLD.glow * box.w;
  if (r * Math.sin(WORLD.span) + rim > box.w / 2) throw new Error("voyage: the planet arc leaves the frame");
  const x0 = Math.floor(box.x);
  const y0 = Math.floor(top - rim);
  const w = Math.ceil(box.w);
  const bottom = box.y + box.h;
  const h = Math.ceil(bottom) - y0;
  const colors = new Uint8ClampedArray(w * h * 4);
  for (let y = 0; y < h; y++) {
    for (let x = 0; x < w; x++) {
      const dx = x0 + x + 0.5 - cx;
      const dy = y0 + y + 0.5 - cy;
      const turn = Math.atan2(dx, -dy) / WORLD.span;
      if (Math.abs(turn) >= 1) continue;
      const fade = (1 - turn * turn) ** WORLD.taper;
      const d = Math.hypot(dx, dy);
      const edge = Math.min(1, Math.max(0, rim / 2 + 0.5 - Math.abs(d - r)));
      const inside = r - d;
      const haze =
        inside > 0 ? Math.exp(-inside / glow) * WORLD.haze * Math.max(0, (bottom - y0 - y) / (bottom - top)) : 0;
      const a = edge * fade;
      const b = haze * fade * (1 - a);
      const out = a + b;
      if (out <= 0) continue;
      const i = (y * w + x) * 4;
      colors[i] = (255 * a + light[0] * b) / out;
      colors[i + 1] = (255 * a + light[1] * b) / out;
      colors[i + 2] = (255 * a + light[2] * b) / out;
      colors[i + 3] = 255 * out;
    }
  }
  const made: Arc = { x0, y0, w, h, top, image: { shape: [h, w], colors } };
  worlds.set(key, made);
  return made;
}

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "indigo", 0.14);
  const globe = world(box, paint.light);
  pen.image(globe.x0, globe.y0, globe.w, globe.h, globe.image);
  const side = SHIP.side * box.w;
  const hull = frame(box.x + (box.w - side) / 2, box.y + SHIP.top * box.h, side, side);
  if (hull.y + hull.h >= globe.top - WORLD.rim * box.w) throw new Error("voyage: the ship touches the planet");
  iso.draw(pen, hull, ship(), [paint.white, paint.light, paint.deep], null);
}
