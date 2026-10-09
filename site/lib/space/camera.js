import { rng } from '../scene.js';
import { add, basis, dot, mat3, mul, rot, sub } from './vec.js';

const TAU = Math.PI * 2;
const FOV = 1;
const NEAR = 1e-6;

const clamp = (p) => Math.min(Math.max(p, 0), 1);

export const ease = {
  smooth: (p) => {
    const x = clamp(p);
    return x * x * (3 - 2 * x);
  },
  in: (p, k = 2) => clamp(p) ** k,
  out: (p, k = 2) => 1 - (1 - clamp(p)) ** k,
  expo: (p) => (2 ** (10 * clamp(p)) - 1) / 1023,
};

/* VIEW */

export function look(pos, at, up = [0, 1, 0], roll = 0) {
  const [r, u, f] = basis(sub(at, pos), up);
  const rr = roll ? rot(r, f, roll) : r;
  const ru = roll ? rot(u, f, roll) : u;
  return { pos: [...pos], rot: mat3(rr, ru, f), fov: FOV };
}

const axes = (cam) => {
  const m = cam.rot;
  return [[m[0], m[1], m[2]], [m[3], m[4], m[5]], [m[6], m[7], m[8]]];
};

export function uniforms(view, cam) {
  return { uPos: cam.pos, uRot: cam.rot, uTan: Math.tan(cam.fov / 2), uRes: [view.w, view.h] };
}

export function project(view, cam, p) {
  const [r, u, f] = axes(cam);
  const d = sub(p, cam.pos);
  const z = dot(d, f);
  if (z <= NEAR) return null;
  const focal = Math.min(view.w, view.h) / (2 * Math.tan(cam.fov / 2));
  return [view.w / 2 + (dot(d, r) / z) * focal, view.h / 2 - (dot(d, u) / z) * focal, z];
}

/* PATHS */

export function spline(points, u) {
  const n = points.length;
  if (n < 2) return [...points[0]];
  const x = clamp(u) * (n - 1);
  const i = Math.min(Math.floor(x), n - 2);
  const s = x - i;
  const P = [points[Math.max(i - 1, 0)], points[i], points[i + 1], points[Math.min(i + 2, n - 1)]];
  const s2 = s * s;
  const s3 = s2 * s;
  return mul(add(add(mul(P[0], -s3 + 2 * s2 - s), mul(P[1], 3 * s3 - 5 * s2 + 2)), add(mul(P[2], -3 * s3 + 4 * s2 + s), mul(P[3], s3 - s2))), 0.5);
}

export function orbit(center, radius, tilt, rate, phase, t) {
  const a = phase + (rate * t) / 1000;
  const off = [radius * Math.cos(a), radius * Math.sin(a) * Math.sin(tilt), radius * Math.sin(a) * Math.cos(tilt)];
  return look(add(center, off), center);
}

const waves = (seed, n, lo, hi) => {
  const rand = rng(seed >>> 0);
  return Array.from({ length: n }, () => [lo + rand() * (hi - lo), rand() * TAU]);
};

const sum = (list, t) => list.reduce((acc, [hz, ph]) => acc + Math.sin((TAU * hz * t) / 1000 + ph), 0) / list.length;

export function drift(seed, t, amp) {
  const w = waves(seed, 6, 0.03, 0.12);
  return [sum(w.slice(0, 2), t) * amp, sum(w.slice(2, 4), t) * amp, sum(w.slice(4), t) * amp];
}

export function shake(seed, t, amp) {
  const w = waves(seed ^ 0x2545f491, 6, 7, 13);
  return [sum(w.slice(0, 3), t) * amp, sum(w.slice(3), t) * amp];
}

