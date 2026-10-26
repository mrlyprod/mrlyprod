import { rng } from "../rand.ts";
import type { View } from "./gl2.ts";
import type { Vec, Vec3 } from "./vec.ts";
import { basis, clamp01, dot, mat3, rot, sub } from "./vec.ts";

/* TYPES */

export type Cam = { pos: number[]; rot: Float32Array; fov: number };

/* CONSTANTS */

const TAU = Math.PI * 2;
const FOV = 1;
const NEAR = 1e-6;

export const ease = {
  smooth: (p: number) => {
    const x = clamp01(p);
    return x * x * (3 - 2 * x);
  },
  in: (p: number, k = 2) => clamp01(p) ** k,
  out: (p: number, k = 2) => 1 - (1 - clamp01(p)) ** k,
  expo: (p: number) => (2 ** (10 * clamp01(p)) - 1) / 1023,
};

/* VIEW */

export function look(pos: readonly number[], at: Vec, up: Vec = [0, 1, 0], roll = 0): Cam {
  const [r, u, f] = basis(sub(at, pos), up);
  const rr = roll ? rot(r, f, roll) : r;
  const ru = roll ? rot(u, f, roll) : u;
  return { pos: [...pos], rot: mat3(rr, ru, f), fov: FOV };
}

const axes = (cam: Cam) => {
  const m = cam.rot;
  return [
    [m[0], m[1], m[2]],
    [m[3], m[4], m[5]],
    [m[6], m[7], m[8]],
  ];
};

export function uniforms(view: Pick<View, "w" | "h">, cam: Cam) {
  return { uPos: cam.pos, uRot: cam.rot, uTan: Math.tan(cam.fov / 2), uRes: [view.w, view.h] };
}

export function project(view: Pick<View, "w" | "h">, cam: Cam, p: Vec): Vec3 | null {
  const [r, u, f] = axes(cam);
  const d = sub(p, cam.pos);
  const z = dot(d, f);
  if (z <= NEAR) return null;
  const focal = Math.min(view.w, view.h) / (2 * Math.tan(cam.fov / 2));
  return [view.w / 2 + (dot(d, r) / z) * focal, view.h / 2 - (dot(d, u) / z) * focal, z];
}

/* PATHS */

const waves = (seed: number, n: number, lo: number, hi: number) => {
  const rand = rng(seed >>> 0);
  return Array.from({ length: n }, (): [number, number] => [lo + rand() * (hi - lo), rand() * TAU]);
};

const sum = (list: readonly [number, number][], t: number) =>
  list.reduce((acc, [hz, ph]) => acc + Math.sin((TAU * hz * t) / 1000 + ph), 0) / list.length;

export function drift(seed: number, t: number, amp: number): Vec3 {
  const w = waves(seed, 6, 0.03, 0.12);
  return [sum(w.slice(0, 2), t) * amp, sum(w.slice(2, 4), t) * amp, sum(w.slice(4), t) * amp];
}

export function shake(seed: number, t: number, amp: number): [number, number] {
  const w = waves(seed ^ 0x2545f491, 6, 7, 13);
  return [sum(w.slice(0, 3), t) * amp, sum(w.slice(3), t) * amp];
}
