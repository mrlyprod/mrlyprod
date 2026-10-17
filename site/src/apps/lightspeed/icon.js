import { plate } from "../../lib/plate.js";

export const units = {};

const STARS = 24;
const GOLDEN = Math.PI * (3 - Math.sqrt(5));
const A = 0.7548776662;
const B = 0.569840291;
const POINT = 0.07;
const HALO = 0.2;
const TAIL = 0.01;
const HEAD = 0.065;
const CAP = 8;

const frac = (v) => v - Math.floor(v);

function streaks(r) {
  const out = [];
  for (let i = 0; i < STARS; i++) {
    const angle = i * GOLDEN;
    const head = (0.5 + 0.5 * frac(i * B)) * r;
    const tail = (0.3 + 0.3 * frac(i * A)) * head;
    out.push({ angle, head, tail });
  }
  return out;
}

function streak(cx, cy, angle, tail, head, w0, w1) {
  const dx = Math.cos(angle);
  const dy = Math.sin(angle);
  const nx = -dy;
  const ny = dx;
  const pts = [[cx + dx * tail + nx * w0, cy + dy * tail + ny * w0]];
  const hx = cx + dx * head;
  const hy = cy + dy * head;
  for (let k = 0; k <= CAP; k++) {
    const a = angle - Math.PI / 2 + (Math.PI * k) / CAP;
    pts.push([hx + Math.cos(a) * w1, hy + Math.sin(a) * w1]);
  }
  pts.push([cx + dx * tail - nx * w0, cy + dy * tail - ny * w0]);
  return pts;
}

export default function draw(pen) {
  const { box, paint } = plate(pen, "indigo", 0.14);
  const [cx, cy] = box.center();
  const r = box.radius();
  const spokes = streaks(r);
  const longest = Math.max(...spokes.map((s) => s.head - s.tail));
  for (const { tail } of spokes) if (tail <= r * POINT) throw new Error("lightspeed: a streak crosses the vanishing point");
  pen.disc(cx, cy, r * HALO, [255, 255, 255, 45]);
  for (const { angle, head, tail } of spokes) {
    const t = (head - tail) / longest;
    const w1 = r * (TAIL + (HEAD - TAIL) * t);
    pen.polygon(streak(cx, cy, angle, tail, head, r * TAIL, w1), [255, 255, 255, Math.round(120 + 135 * t)]);
  }
  pen.disc(cx, cy, r * POINT * 1.6, paint.light);
  pen.disc(cx, cy, r * POINT, paint.white);
}
