/* TYPES */

export type Vec = ArrayLike<number>;

export type Vec3 = [number, number, number];

/* VECTORS */

export const add = (a: Vec, b: Vec): Vec3 => [a[0] + b[0], a[1] + b[1], a[2] + b[2]];

export const sub = (a: Vec, b: Vec): Vec3 => [a[0] - b[0], a[1] - b[1], a[2] - b[2]];

export const mul = (a: Vec, s: number): Vec3 => [a[0] * s, a[1] * s, a[2] * s];

export const dot = (a: Vec, b: Vec) => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];

export const cross = (a: Vec, b: Vec): Vec3 => [
  a[1] * b[2] - a[2] * b[1],
  a[2] * b[0] - a[0] * b[2],
  a[0] * b[1] - a[1] * b[0],
];

export const len = (a: Vec) => Math.hypot(a[0], a[1], a[2]);

export const norm = (a: Vec): Vec3 => {
  const l = len(a) || 1;
  return [a[0] / l, a[1] / l, a[2] / l];
};

export function mix(a: number, b: number, k: number): number;
export function mix(a: readonly number[], b: readonly number[], k: number): number[];
export function mix(a: number | readonly number[], b: number | readonly number[], k: number): number | number[] {
  return typeof a === "number"
    ? a + ((b as number) - a) * k
    : a.map((v, i) => v + ((b as readonly number[])[i] - v) * k);
}

export function basis(fwd: Vec, up: Vec = [0, 1, 0]): [Vec3, Vec3, Vec3] {
  const f = norm(fwd);
  let r = cross(up, f);
  if (len(r) < 1e-6) r = cross(Math.abs(f[2]) < 0.9 ? [0, 0, 1] : [1, 0, 0], f);
  r = norm(r);
  return [r, cross(f, r), f];
}

export const mat3 = (r: Vec, u: Vec, f: Vec) =>
  new Float32Array([r[0], r[1], r[2], u[0], u[1], u[2], f[0], f[1], f[2]]);

export function rot(v: Vec, axis: Vec, ang: number): Vec3 {
  const k = norm(axis);
  const c = Math.cos(ang);
  const s = Math.sin(ang);
  return add(add(mul(v, c), mul(cross(k, v), s)), mul(k, dot(k, v) * (1 - c)));
}

export const clamp01 = (x: number) => Math.min(Math.max(x, 0), 1);

export const smooth = (x: number) => {
  const p = clamp01(x);
  return p * p * (3 - 2 * p);
};
