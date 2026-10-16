import * as math from "@mrlyprod/mrly/math";
import { frame, hex } from "#view";
import { plate } from "../../lib/icon.js";

export const units = { math };

const CODE = "23";
const SIDE = 7;
const WANT = { fills: 204, pieces: 1, holes: 7 };
const INSET = 0.15;
const YAW = 0.3;
const PITCH = 0.62;
const NORMAL = [1, -1, 1];
const FLAT = 1000;
const GAP = -1;
const VOID = 45;
const GLASS = [48, 32, 20];
const BACK = 110;
const RIM = 220;
const HALO = 110;
const EDGE = 0.014;

let slice = null;

function facts() {
  if (slice) return slice;
  const cut = math.six.cut(math.three.create(CODE, SIDE, 1, 2));
  const tally = math.six.census(cut, false);
  if (tally.triangles !== hex.count(SIDE)) throw new Error(`slices: the cut holds ${tally.triangles} triangles, want ${hex.count(SIDE)}`);
  const read = { fills: tally.fills, pieces: math.six.components(cut), holes: math.six.holes(cut) };
  if (read.fills !== WANT.fills || read.pieces !== WANT.pieces || read.holes !== WANT.holes) throw new Error(`slices: the carpet's middle hexagon of side ${SIDE} reads ${JSON.stringify(read)}, want ${JSON.stringify(WANT)}`);
  const tones = { [math.six.FILL()]: "fill", [math.six.VOID()]: "void" };
  const tris = [];
  hex.draw({ triangle: (a, b, c, tone) => tris.push([[a, b, c], tone]) }, frame(-FLAT, -FLAT, 2 * FLAT, 2 * FLAT), cut, GAP, (type) => tones[type] ?? null);
  slice = tris;
  return slice;
}

const dot = (a, b) => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
const cross = (a, b) => [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
const unit = (a) => a.map((v) => v / Math.hypot(...a));
const LOOK = [Math.cos(PITCH) * Math.cos(YAW), Math.cos(PITCH) * Math.sin(YAW), Math.sin(PITCH)];
const RIGHT = [-Math.sin(YAW), Math.cos(YAW), 0];
const UP = cross(LOOK, RIGHT);
const CORNERS = Array.from({ length: 8 }, (_, i) => [(i & 1) * 2 - 1, ((i >> 1) & 1) * 2 - 1, ((i >> 2) & 1) * 2 - 1]);
const EDGES = CORNERS.flatMap((a, i) => CORNERS.slice(i + 1).filter((b) => dot(a, b) === 1).map((b) => [a, b]));
const NEAR = CORNERS.reduce((best, p) => (dot(p, LOOK) > dot(best, LOOK) ? p : best));
const FAR = CORNERS.reduce((best, p) => (dot(p, LOOK) < dot(best, LOOK) ? p : best));

function faces() {
  const out = [];
  for (let axis = 0; axis < 3; axis++) {
    for (const sign of [-1, 1]) {
      const normal = [0, 0, 0];
      normal[axis] = sign;
      const quad = [[-1, -1], [1, -1], [1, 1], [-1, 1]].map(([u, v]) => {
        const p = [0, 0, 0];
        p[axis] = sign;
        p[(axis + 1) % 3] = u;
        p[(axis + 2) % 3] = v;
        return p;
      });
      out.push({ quad, facing: dot(normal, LOOK) });
    }
  }
  return out.filter((f) => f.facing > 0).sort((a, b) => b.facing - a.facing);
}

export default function draw(pen) {
  const { box, paint, size } = plate(pen, "orange", INSET);
  const view = (p) => [dot(p, RIGHT), -dot(p, UP)];
  const flat = CORNERS.map(view);
  const xs = flat.map(([x]) => x);
  const ys = flat.map(([, y]) => y);
  const [lx, hx, ly, hy] = [Math.min(...xs), Math.max(...xs), Math.min(...ys), Math.max(...ys)];
  const scale = Math.min(box.w / (hx - lx), box.h / (hy - ly));
  const [cx, cy] = box.center();
  const place = (p) => {
    const [x, y] = view(p);
    return [cx + (x - (lx + hx) / 2) * scale, cy + (y - (ly + hy) / 2) * scale];
  };
  const meets = (corner) => ([a, b]) => a === corner || b === corner;
  const line = ([a, b], thick, color) => pen.segment(place(a), place(b), size * thick, color);
  faces().forEach((f, i) => pen.polygon(f.quad.map(place), [255, 255, 255, GLASS[i]]));
  for (const edge of EDGES.filter(meets(FAR))) line(edge, EDGE * 0.6, [255, 255, 255, BACK]);
  const n = unit(NORMAL);
  const e1 = unit([Math.sign(NORMAL[0]), -Math.sign(NORMAL[1]), 0]);
  const e2 = cross(n, e1);
  const lift = ([fx, fy]) => [0, 1, 2].map((k) => (Math.SQRT2 * (fx * e1[k] - fy * e2[k])) / FLAT);
  for (const [pts, tone] of facts()) {
    const [a, b, c] = pts.map((p) => place(lift(p)));
    pen.triangle(a, b, c, tone === "fill" ? paint.white : [255, 255, 255, VOID]);
  }
  for (const edge of EDGES.filter((e) => !meets(FAR)(e) && !meets(NEAR)(e))) line(edge, EDGE, [255, 255, 255, RIM]);
  const spine = EDGES.filter(meets(NEAR));
  for (const edge of spine) line(edge, EDGE * 2.2, [...paint.deep.slice(0, 3), HALO]);
  for (const edge of spine) line(edge, EDGE, paint.white);
}
