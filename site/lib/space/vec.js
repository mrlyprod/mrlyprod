export const add = (a, b) => [a[0] + b[0], a[1] + b[1], a[2] + b[2]];

export const sub = (a, b) => [a[0] - b[0], a[1] - b[1], a[2] - b[2]];

export const mul = (a, s) => [a[0] * s, a[1] * s, a[2] * s];

export const dot = (a, b) => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];

export const cross = (a, b) => [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];

export const len = (a) => Math.hypot(a[0], a[1], a[2]);

export const norm = (a) => {
  const l = len(a) || 1;
  return [a[0] / l, a[1] / l, a[2] / l];
};

export const mix = (a, b, k) => (typeof a === 'number' ? a + (b - a) * k : a.map((v, i) => v + (b[i] - v) * k));

export function basis(fwd, up = [0, 1, 0]) {
  const f = norm(fwd);
  let r = cross(up, f);
  if (len(r) < 1e-6) r = cross(Math.abs(f[2]) < 0.9 ? [0, 0, 1] : [1, 0, 0], f);
  r = norm(r);
  return [r, cross(f, r), f];
}

export const mat3 = (r, u, f) => new Float32Array([r[0], r[1], r[2], u[0], u[1], u[2], f[0], f[1], f[2]]);

export function rot(v, axis, ang) {
  const k = norm(axis);
  const c = Math.cos(ang);
  const s = Math.sin(ang);
  return add(add(mul(v, c), mul(cross(k, v), s)), mul(k, dot(k, v) * (1 - c)));
}
