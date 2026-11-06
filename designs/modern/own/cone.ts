import type { Point } from "@mrlyprod/draw";
import { clip } from "./path.ts";

const APEX: Point = [3.4, 28.6];
const AXIS: Point = [Math.SQRT1_2, -Math.SQRT1_2];
const SIDE: Point = [Math.SQRT1_2, Math.SQRT1_2];
const LONG = 15;

export const HALF = 6.6;
export const TILT = Math.PI / 4;

export const at = (t: number, s: number): Point => [
  APEX[0] + AXIS[0] * t + SIDE[0] * s,
  APEX[1] + AXIS[1] * t + SIDE[1] * s,
];

export const CONE: Point[] = [APEX, at(LONG, -HALF), at(LONG, HALF)];
export const MID = at(LONG, 0);

export const slice = (t0: number, t1: number) => clip([at(t0, -20), at(t1, -20), at(t1, 20), at(t0, 20)], CONE);
