import { plate } from "../../lib/icon.js";

export const units = {};

const BODY = 0.56;
const RINGS = [[0.72, 0.9], [0.93, 1]];
const SQUASH = 0.3;
const TILT = -0.32;
const SUN = [-0.72, -0.42, 0.55];
const SHADOW = [0.04, 0.08];
const STEPS = 48;
const SAMPLE = 33;

const unit = (v) => {
  const l = Math.hypot(...v);
  return v.map((c) => c / l);
};

const sun = unit(SUN);

const ring = (r, t) => {
  const u = r * Math.cos(t);
  const v = r * Math.sin(t) * SQUASH;
  return [u * Math.cos(TILT) - v * Math.sin(TILT), u * Math.sin(TILT) + v * Math.cos(TILT)];
};

const lit = (x, y) => {
  const z = Math.sqrt(Math.max(BODY * BODY - x * x - y * y, 0));
  return x * sun[0] + y * sun[1] + z * sun[2];
};

const onRing = (x, y) => {
  const u = x * Math.cos(TILT) + y * Math.sin(TILT);
  const v = (-x * Math.sin(TILT) + y * Math.cos(TILT)) / SQUASH;
  const r = Math.hypot(u, v);
  return { on: RINGS.some(([a, b]) => r >= a && r <= b), front: v > 0 };
};

let checked = false;

function facts() {
  if (checked) return;
  const seen = { lit: 0, dark: 0, front: 0, back: 0, shade: 0 };
  const step = 2 / (SAMPLE - 1);
  for (let row = 0; row < SAMPLE; row++) {
    for (let col = 0; col < SAMPLE; col++) {
      const x = -1 + col * step;
      const y = -1 + row * step;
      const disc = Math.hypot(x, y) <= BODY;
      const { on, front } = onRing(x, y);
      if (on && front && disc) seen.front++;
      if (on && !front && !disc) seen.back++;
      if (disc && !(on && front)) {
        if (lit(x, y) > 0) seen.lit++;
        else seen.dark++;
        const { on: above, front: near } = onRing(x - SHADOW[0], y - SHADOW[1]);
        if (above && near) seen.shade++;
      }
    }
  }
  if (!seen.lit || !seen.dark) throw new Error("planets: the terminator does not cross the disc");
  if (!seen.front || !seen.back) throw new Error("planets: the rings do not pass both in front of and behind the disc");
  if (!seen.shade) throw new Error("planets: the rings cast no shadow on the disc");
  checked = true;
}

function band(pen, at, [a, b], from, to, color) {
  const span = to - from;
  for (let i = 0; i < STEPS; i++) {
    const t0 = from + (span * (i - 0.05)) / STEPS;
    const t1 = from + (span * (i + 1.05)) / STEPS;
    pen.polygon([ring(a, t0), ring(b, t0), ring(b, t1), ring(a, t1)].map(at), color);
  }
}

function outline([a, b], from, to) {
  const pts = [];
  for (let k = 0; k <= STEPS; k++) pts.push(ring(b, from + ((to - from) * k) / STEPS));
  for (let k = STEPS; k >= 0; k--) pts.push(ring(a, from + ((to - from) * k) / STEPS));
  return pts;
}

function cap(pen, at, color) {
  const d = unit([sun[0], sun[1]]);
  const p = [-d[1], d[0]];
  const start = Math.atan2(d[1], d[0]) + Math.PI / 2;
  const pts = [];
  for (let k = 0; k <= STEPS; k++) {
    const phi = start + (Math.PI * k) / STEPS;
    pts.push(at([BODY * Math.cos(phi), BODY * Math.sin(phi)]));
  }
  for (let k = STEPS - 1; k > 0; k--) {
    const t = (Math.PI * k) / STEPS;
    const c = Math.cos(t);
    const s = Math.sin(t) * sun[2];
    pts.push(at([BODY * (c * p[0] - s * d[0]), BODY * (c * p[1] - s * d[1])]));
  }
  pen.polygon(pts, color);
}

export default function draw(pen) {
  const { box, paint } = plate(pen, "blue", 0.16);
  facts();
  const [cx, cy] = box.center();
  const R = box.radius();
  const at = ([x, y]) => [cx + x * R, cy + y * R];
  const tones = [paint.deep, paint.light];
  const pull = ([x, y]) => {
    const d = Math.hypot(x, y);
    const k = d > BODY ? BODY / d : 1;
    return at([x * k, y * k]);
  };
  RINGS.forEach((r, i) => band(pen, at, r, Math.PI, 2 * Math.PI, tones[i]));
  pen.disc(cx, cy, BODY * R, paint.white);
  cap(pen, at, paint.light);
  const shadow = [...paint.ink.slice(0, 3), 90];
  for (const r of RINGS) pen.polygon(outline(r, 0, Math.PI).map(([x, y]) => pull([x + SHADOW[0], y + SHADOW[1]])), shadow);
  RINGS.forEach((r, i) => band(pen, at, r, 0, Math.PI, tones[i]));
}
