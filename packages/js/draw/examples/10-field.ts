// A field turns one number per pixel into color through a ramp.
import { field, ink, raster } from "@mrlyprod/draw";
import { dark } from "@mrlyprod/draw/theme.ts";
import { save } from "./save.ts";

const paint = ink(dark);
const pen = raster(640, 640, paint.ground);
const size = 320;

// Two circular waves interfering.
const values = new Float64Array(size * size);
for (let y = 0; y < size; y++) {
  for (let x = 0; x < size; x++) {
    const a = Math.hypot(x - size * 0.35, y - size / 2);
    const b = Math.hypot(x - size * 0.65, y - size / 2);
    values[y * size + x] = Math.sin(a / 5) + Math.sin(b / 5);
  }
}

field.draw(pen, pen.frame(0), size, size, values, paint.Ramp.diverge());

save(import.meta.file, pen);
