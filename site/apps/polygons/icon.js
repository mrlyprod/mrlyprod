import { mix } from "mrlyjs/view";

const R3 = Math.sqrt(3);
const E = 1 / (1 + R3);
const AP = (E * R3) / 2;
const SPACING = 300;
const GAP = 7;
const BAND = [0.42, 0.58];
const HUES = ["red", "orange", "yellow", "green", "teal", "blue", "indigo", "purple"];

const at = (deg, r, s = 0) => {
  const a = (deg * Math.PI) / 180;
  return [r * Math.cos(a) - s * Math.sin(a), r * Math.sin(a) + s * Math.cos(a)];
};

const HEX = Array.from({ length: 6 }, (_, k) => at(30 + 60 * k, E));
const square = (deg) => [at(deg, AP, -E / 2), at(deg, AP + E, -E / 2), at(deg, AP + E, E / 2), at(deg, AP, E / 2)];
const TILES = [HEX, square(0), square(60), square(120), [HEX[0], at(0, AP + E, E / 2), at(60, AP + E, -E / 2)], [HEX[1], at(60, AP + E, E / 2), at(120, AP + E, -E / 2)]];

const area = (pts) => Math.abs(pts.reduce((sum, [x, y], i) => sum + x * pts[(i + 1) % pts.length][1] - pts[(i + 1) % pts.length][0] * y, 0)) / 2;

const centre = (pts) => [pts.reduce((s, p) => s + p[0], 0) / pts.length, pts.reduce((s, p) => s + p[1], 0) / pts.length];

const shrink = (pts, by) => {
  const [cx, cy] = centre(pts);
  const r = Math.min(...pts.map(([x, y], i) => {
    const [nx, ny] = pts[(i + 1) % pts.length];
    return Math.abs((nx - x) * (y - cy) - (x - cx) * (ny - y)) / Math.hypot(nx - x, ny - y);
  }));
  const k = Math.max(r - by, 0) / r;
  return pts.map(([x, y]) => [cx + (x - cx) * k, cy + (y - cy) * k]);
};

const hash = (i, j, n) => {
  let h = Math.imul(i * 73856093 ^ j * 19349663 ^ n * 83492791, 0x5bd1e995);
  h ^= h >>> 15;
  return (Math.imul(h, 0x27d4eb2d) >>> 0) / 4294967296;
};

export default function draw(pen, ink) {
  const kinds = TILES.map((t) => t.length);
  if (kinds.join() !== "6,4,4,4,3,3") throw new Error("polygons: a cell of 3.4.6.4 is one hexagon, three squares and two triangles");
  const whole = TILES.reduce((sum, t) => sum + area(t), 0);
  if (Math.abs(whole - R3 / 2) > 1e-9) throw new Error(`polygons: the tiles cover ${whole}, and the lattice cell is ${R3 / 2}`);
  const { width: w, height: h } = pen;
  const s = SPACING;
  const reach = Math.ceil(Math.max(w, h) / s) + 2;
  for (let j = -reach; j <= reach; j++) {
    for (let i = -reach; i <= reach; i++) {
      const ox = w / 2 + (i + j / 2) * s;
      const oy = h / 2 + ((j * R3) / 2) * s;
      TILES.forEach((tile, n) => {
        const pts = tile.map(([x, y]) => [ox + x * s, oy + y * s]);
        const [cx, cy] = centre(pts);
        if (cx < -s / 2 || cx > w + s / 2 || cy < -s / 2 || cy > h + s / 2) return;
        const along = (cx + cy) / (w + h);
        const k = Math.min(Math.max(along * HUES.length + (hash(i, j, n) - 0.5) * 0.9, 0), HUES.length - 1e-9);
        const base = hash(i, j, n + 9) < 0.06 ? ink.fg : ink[HUES[Math.floor(k)]];
        const lit = along > BAND[0] && along < BAND[1] ? mix(base, [255, 255, 255, 255], 0.3) : base;
        pen.polygon(shrink(pts, GAP / 2), lit);
      });
    }
  }
}
