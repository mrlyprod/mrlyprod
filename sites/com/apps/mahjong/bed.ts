import type { Box } from "@mrlyprod/ui/kit/run.ts";
import type { Spot } from "./engine.ts";
import { TALL } from "./engine.ts";

export type Bed = {
  w: number;
  h: number;
  wall: number;
  rise: number;
  left: number;
  top: number;
  line: number;
  low: [number, number];
};

const LINE = 0.075;
const FOOT = 0.2;
const FIT = 0.98;
const WALL = 0.11;
const RISE = 0.1;

export function roomOf(view: Box): Box {
  const line = Math.round(Math.min(view.w, view.h) * LINE);
  return { w: view.w * FIT, h: (view.h - line * (1 + FOOT)) * FIT };
}

export function bedOf(view: Box, tiles: readonly Spot[]): Bed {
  const xs = tiles.map((one) => one.x);
  const ys = tiles.map((one) => one.y);
  const top = Math.max(...tiles.map((one) => one.z));
  const low: [number, number] = [Math.min(...xs), Math.min(...ys)];
  const cols = (Math.max(...xs) - low[0]) / 2 + 1;
  const rows = (Math.max(...ys) - low[1]) / 2 + 1;
  const line = Math.round(Math.min(view.w, view.h) * LINE);
  const room = roomOf(view);
  const across = cols + WALL + RISE * top;
  const down = rows * TALL + WALL + RISE * top;
  const w = Math.max(4, Math.floor(Math.min(room.w / across, room.h / down)));
  const h = Math.round(w * TALL);
  const rise = Math.max(1, Math.round(w * RISE));
  const wall = Math.max(1, Math.round(w * WALL));
  const spanX = cols * w + wall + rise * top;
  const spanY = rows * h + wall + rise * top;
  return {
    w,
    h,
    wall,
    rise,
    left: Math.floor((view.w - spanX) / 2) + rise * top,
    top: Math.floor(line + (view.h - line * (1 + FOOT) - spanY) / 2) + rise * top,
    line,
    low,
  };
}

export const spotOf = (bed: Bed, one: Spot): [number, number] => [
  bed.left + ((one.x - bed.low[0]) / 2) * bed.w - one.z * bed.rise,
  bed.top + ((one.y - bed.low[1]) / 2) * bed.h - one.z * bed.rise,
];
