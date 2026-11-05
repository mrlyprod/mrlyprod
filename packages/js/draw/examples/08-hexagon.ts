// A hexagon of triangles: the ink callback decides each one.
import { hex, ink, raster } from "@mrlyprod/draw";
import { dark } from "@mrlyprod/draw/theme.ts";
import { save } from "./save.ts";

const paint = ink(dark);
const pen = raster(720, 720, paint.ground);
const side = 12;

hex.hexagon(pen, pen.frame(0.04), side, 1.5, (row, col, up) => {
  // Returning null leaves a hole.
  if ((row + col) % 7 === 0) return null;
  const t = row / (2 * side - 1);
  return up ? paint.mix(paint.indigo, paint.teal, t) : paint.mix(paint.pink, paint.yellow, t);
});

save(import.meta.file, pen);
