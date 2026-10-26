import type { Paints } from "../../lib/color.ts";
import { rgb } from "../../lib/color.ts";
import { ESCAPE } from "../../lib/fractal/consts.ts";
import type { Pair } from "../../lib/fractal/orbit.ts";
import { glow } from "../../lib/fractal/shader.ts";
import { spanOf } from "../../lib/fractal/view.ts";
import { rng } from "../../lib/rand.ts";

/* TYPES */

export type Kind = "mandelbrot" | "julia";

export type Place = { id: string; name: string; x: string; y: string; zoom: number } & (
  | { kind: "mandelbrot"; c?: undefined }
  | { kind: "julia"; c: Pair }
);

export type Look = Pick<Paints, "paper" | "accent">;

export const KINDS: [Kind, string][] = [
  ["mandelbrot", "Mandelbrot"],
  ["julia", "Julia"],
];

const FEW = 64;
const PER = 32;
const MOST = 640;

/* PLACES */

export const PLACES: Place[] = [
  { id: "whole", name: "Whole Set", kind: "mandelbrot", x: "-0.75", y: "0", zoom: 0 },
  { id: "seahorse", name: "Seahorse Valley", kind: "mandelbrot", x: "-0.7463", y: "0.1102", zoom: 8 },
  { id: "elephant", name: "Elephant Valley", kind: "mandelbrot", x: "0.2851", y: "0.0115", zoom: 8 },
  { id: "triple", name: "Triple Spiral", kind: "mandelbrot", x: "-0.0889", y: "0.6546", zoom: 11 },
  { id: "scepter", name: "Scepter Valley", kind: "mandelbrot", x: "-1.2505", y: "0.0217", zoom: 8 },
  { id: "mini", name: "Mini Mandelbrot", kind: "mandelbrot", x: "-1.7548776662467", y: "0", zoom: 6.3 },
  { id: "feigenbaum", name: "Feigenbaum Point", kind: "mandelbrot", x: "-1.401155", y: "0", zoom: 7 },
  { id: "branch", name: "Branch Point", kind: "mandelbrot", x: "-0.1011", y: "0.9563", zoom: 8 },
  {
    id: "spiral",
    name: "Seahorse Spiral",
    kind: "mandelbrot",
    x: "-0.743643887037151",
    y: "0.13182590420533",
    zoom: 17,
  },
  { id: "rabbit", name: "Douady Rabbit", kind: "julia", c: [-0.122561167, 0.744861767], x: "0", y: "0", zoom: 0 },
  { id: "dendrite", name: "Dendrite", kind: "julia", c: [0, 1], x: "0", y: "0", zoom: 0.3 },
  { id: "marco", name: "San Marco", kind: "julia", c: [-0.75, 0], x: "0", y: "0", zoom: 0 },
  { id: "siegel", name: "Siegel Disk", kind: "julia", c: [-0.39054087, -0.586787907], x: "0", y: "0", zoom: 0 },
  { id: "dragon", name: "Dragon", kind: "julia", c: [-0.8, 0.156], x: "0", y: "0", zoom: -0.1 },
  { id: "basilica", name: "Basilica", kind: "julia", c: [-1, 0], x: "0", y: "0", zoom: -0.1 },
  { id: "cauliflower", name: "Cauliflower", kind: "julia", c: [0.25, 0], x: "0", y: "0", zoom: 0 },
  { id: "dust", name: "Dust", kind: "julia", c: [0.45, 0.1428], x: "0", y: "0", zoom: 0 },
  { id: "spirals", name: "Spirals", kind: "julia", c: [-0.7269, 0.1889], x: "0", y: "0", zoom: 0 },
];

export const DIVES: Pair[] = [
  [-0.4, 0.6],
  [-0.8, 0.156],
  [0.285, 0.01],
  [-0.7269, 0.1889],
  [-0.1, 0.651],
  [0.355, 0.355],
];

export const roll = (kind: Kind, seed: number) => {
  const list = PLACES.filter((one) => one.kind === kind);
  return list[Math.floor(rng(seed)() * list.length)];
};

/* THUMBS */

export function dwell(one: Place, x: number, y: number, max: number) {
  const julia = one.kind === "julia";
  const [cr, ci] = julia ? one.c : [x, y];
  let zr = julia ? x : 0;
  let zi = julia ? y : 0;
  for (let n = 1; n <= max; n++) {
    const t = zr * zr - zi * zi + cr;
    zi = 2 * zr * zi + ci;
    zr = t;
    const r = zr * zr + zi * zi;
    if (r > ESCAPE) return n + 1 - Math.log2(Math.max(0.5 * Math.log2(r), 1e-6));
  }
  return -1;
}

export const most = (one: Pick<Place, "zoom">) => Math.min(MOST, FEW + PER * Math.max(0, one.zoom));

export function thumb(ctx: CanvasRenderingContext2D, one: Place, w: number, h: number, look: Look) {
  const image = ctx.createImageData(w, h);
  const paper = rgb(look.paper);
  const accent = rgb(look.accent);
  const span = spanOf(one.zoom);
  const short = Math.min(w, h);
  const max = most(one);
  for (let row = 0; row < h; row++) {
    const y = Number(one.y) + ((h / 2 - row - 0.5) / short) * span;
    for (let col = 0; col < w; col++) {
      const sl = dwell(one, Number(one.x) + ((col + 0.5 - w / 2) / short) * span, y, max);
      const k = sl < 0 ? 0 : glow(sl);
      const at = (row * w + col) * 4;
      for (let i = 0; i < 3; i++) image.data[at + i] = paper[i] + (accent[i] - paper[i]) * k;
      image.data[at + 3] = 255;
    }
  }
  ctx.putImageData(image, 0, 0);
}
