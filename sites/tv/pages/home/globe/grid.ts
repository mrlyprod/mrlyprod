export type Vec = [number, number, number];

export type Grid = {
  level: number;
  centers: Vec[];
  corners: Vec[][];
  neighbors: number[][];
};

export function unit([x, y, z]: Vec): Vec {
  const n = Math.hypot(x, y, z);
  return [x / n, y / n, z / n];
}

export const dot = (a: Vec, b: Vec) => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];

export const cross = (a: Vec, b: Vec): Vec => [
  a[1] * b[2] - a[2] * b[1],
  a[2] * b[0] - a[0] * b[2],
  a[0] * b[1] - a[1] * b[0],
];

export function around(c: Vec, points: Vec[]): number[] {
  const up: Vec = Math.abs(c[1]) < 0.9 ? [0, 1, 0] : [1, 0, 0];
  const u = unit(cross(up, c));
  const v = cross(c, u);
  const angle = (p: Vec) => Math.atan2(dot(p, v), dot(p, u));
  return points
    .map((p, i) => [angle(p), i] as const)
    .sort((a, b) => a[0] - b[0])
    .map(([, i]) => i);
}

const PHI = (1 + Math.sqrt(5)) / 2;
const LEAN = Math.atan2(1, PHI);
const SEED: Vec[] = [
  [-1, PHI, 0],
  [1, PHI, 0],
  [-1, -PHI, 0],
  [1, -PHI, 0],
  [0, -1, PHI],
  [0, 1, PHI],
  [0, -1, -PHI],
  [0, 1, -PHI],
  [PHI, 0, -1],
  [PHI, 0, 1],
  [-PHI, 0, -1],
  [-PHI, 0, 1],
];
const FACES = [
  [0, 11, 5],
  [0, 5, 1],
  [0, 1, 7],
  [0, 7, 10],
  [0, 10, 11],
  [1, 5, 9],
  [5, 11, 4],
  [11, 10, 2],
  [10, 7, 6],
  [7, 1, 8],
  [3, 9, 4],
  [3, 4, 2],
  [3, 2, 6],
  [3, 6, 8],
  [3, 8, 9],
  [4, 9, 5],
  [2, 4, 11],
  [6, 2, 10],
  [8, 6, 7],
  [9, 8, 1],
];

function polar([x, y, z]: Vec): Vec {
  const c = Math.cos(LEAN);
  const s = Math.sin(LEAN);
  return unit([x * c - y * s, x * s + y * c, z]);
}

function centroid(verts: Vec[], [a, b, c]: number[]): Vec {
  const p = verts[a];
  const q = verts[b];
  const r = verts[c];
  return unit([p[0] + q[0] + r[0], p[1] + q[1] + r[1], p[2] + q[2] + r[2]]);
}

function refine(verts: Vec[], faces: number[][]): [Vec[], number[][]] {
  const base = verts.length;
  const out = verts.concat(faces.map((f) => centroid(verts, f)));
  const owner = new Map<number, number>();
  const key = (a: number, b: number) => a * out.length + b;
  faces.forEach((f, i) => {
    for (let k = 0; k < 3; k++) owner.set(key(f[k], f[(k + 1) % 3]), i);
  });
  const next: number[][] = [];
  faces.forEach((f, i) => {
    for (let k = 0; k < 3; k++) {
      const a = f[k];
      const b = f[(k + 1) % 3];
      if (a > b) continue;
      const twin = owner.get(key(b, a))!;
      next.push([a, base + twin, base + i], [b, base + i, base + twin]);
    }
  });
  return [out, next];
}

export function grid(level: number): Grid {
  let verts = SEED.map(polar);
  let faces = FACES;
  for (let i = 0; i < level; i++) [verts, faces] = refine(verts, faces);
  const touching: number[][] = verts.map(() => []);
  const linked: Set<number>[] = verts.map(() => new Set());
  faces.forEach((f, i) => {
    for (let k = 0; k < 3; k++) {
      touching[f[k]].push(i);
      linked[f[k]].add(f[(k + 1) % 3]);
      linked[f[k]].add(f[(k + 2) % 3]);
    }
  });
  const mids = faces.map((f) => centroid(verts, f));
  const corners = verts.map((c, i) => {
    const list = touching[i].map((f) => mids[f]);
    return around(c, list).map((j) => list[j]);
  });
  const neighbors = verts.map((c, i) => {
    const list = [...linked[i]];
    return around(
      c,
      list.map((j) => verts[j]),
    ).map((j) => list[j]);
  });
  return { level, centers: verts, corners, neighbors };
}

export const CELLS = (level: number) => 10 * 3 ** level + 2;

export function latLon([x, y, z]: Vec): [number, number] {
  return [(Math.asin(Math.max(-1, Math.min(1, y))) * 180) / Math.PI, (Math.atan2(x, z) * 180) / Math.PI];
}
