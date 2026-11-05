// The plot kit: draw numbers without a chart library.
import { ink, plot, raster } from "@mrlyprod/draw";
import { dark } from "@mrlyprod/draw/theme.ts";
import { save } from "./save.ts";

const paint = ink(dark);
const pen = raster(800, 520, paint.ground);
const [bars, curve, steps, dots] = pen.area(0.04).panels(2, 2, 16);

const wave = Array.from({ length: 48 }, (_, i) => Math.sin(i / 4) ** 2 + i / 48);

plot.bars(pen, bars, wave, 0.3, paint.blue);
plot.baseline(pen, bars, paint.dim);

const xs = wave.map((_, i) => i);
plot.axis(pen, curve, paint.line);
plot.curve(pen, curve.inset(8), xs, wave, 3, paint.orange);
plot.dashed(pen, curve, curve.y + curve.h / 2, 1, paint.dim);

plot.staircase(pen, steps, wave, 2, paint.green);

// dots() takes points, so place them with the frame.
plot.dots(
  pen,
  wave.map((y, i) => dots.at(i / 47, 1 - y / 2)),
  4,
  paint.pink,
);
plot.rings(pen, dots.center(), [30, 60, 90], 1, paint.line);

save(import.meta.file, pen);
