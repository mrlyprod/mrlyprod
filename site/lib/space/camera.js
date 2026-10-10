import { rng } from '../scene.js';
import { basis, dot, mat3, rot, sub } from './vec.js';

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
