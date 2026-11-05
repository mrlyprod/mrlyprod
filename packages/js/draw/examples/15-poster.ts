// Everything together: a computed field, its histogram, and an inset mark.
import { field, frame, grid, ink, plot, raster } from "@mrlyprod/draw";
import { dark } from "@mrlyprod/draw/theme.ts";
import { save } from "./save.ts";

const paint = ink(dark);
const pen = raster(960, 1200, paint.ground);
const page = pen.area(0.04);

// The picture takes three quarters of the page, the chart the rest.
const picture = frame(page.x, page.y, page.w, page.h * 0.75);
const chart = frame(page.x, page.y + page.h * 0.75, page.w, page.h * 0.25).inset(16);

const WIDE = 640;
const TALL = Math.round((WIDE * picture.h) / picture.w);
const LIMIT = 200;
const SPAN = 3;

// Escape time of z -> z^2 + c, the Mandelbrot set.
function escapes(cx: number, cy: number): number {
  let [x, y] = [0, 0];
  for (let i = 0; i < LIMIT; i++) {
    [x, y] = [x * x - y * y + cx, 2 * x * y + cy];
    if (x * x + y * y > 4) return i + 1 - Math.log2(Math.log2(x * x + y * y));
  }
  return 0;
}

const values = new Float64Array(WIDE * TALL);
const counts = new Float64Array(64);
for (let row = 0; row < TALL; row++) {
  for (let col = 0; col < WIDE; col++) {
    const value = escapes(-2.2 + (SPAN * col) / WIDE, (SPAN * (row - TALL / 2)) / WIDE);
    values[row * WIDE + col] = Math.sqrt(Math.max(value, 0));
    if (value > 0) counts[Math.min(Math.floor(value), counts.length - 1)]++;
  }
}

field.draw(pen, picture, WIDE, TALL, values, paint.Ramp.fire());

// The mark sits on top of the field, in the corner.
const [x, y] = picture.at(0.84, 0.03);
const side = picture.w * 0.13;
grid.carpet(pen, frame(x, y, side, side), grid.mask(grid.LOGO, 2), 0, paint.fg);

// The histogram reads the same numbers the picture was painted from.
plot.bars(pen, chart, counts, 0.2, paint.orange);
plot.baseline(pen, chart, paint.dim);

save(import.meta.file, pen);
