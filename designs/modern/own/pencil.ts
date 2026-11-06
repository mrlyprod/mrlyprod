import type { Point } from "@mrlyprod/draw";
import { shift, turn } from "./path.ts";

export const pencil = (pts: Point[]) => shift(turn(shift(pts, 12, 12), -Math.PI / 4), -0.8, 0.8);
