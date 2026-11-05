// A figure can take a time t, so one function is a still, a loop and a film.
import type { Frame, Ink, Pen, Point } from "@mrlyprod/draw";
import { ink, raster } from "@mrlyprod/draw";
import { dark } from "@mrlyprod/draw/theme.ts";
import { save } from "./save.ts";

function draw(pen: Pen, box: Frame, paint: Ink, t: number) {
  const center = box.center();
  const radius = box.radius() * 0.9;
  const at = (s: number): Point => [center[0] + Math.sin(3 * s + t) * radius, center[1] + Math.sin(2 * s) * radius];
  const curve = Array.from({ length: 240 }, (_, i) => at((i / 239) * 2 * Math.PI));
  pen.polyline(curve, 1.5, paint.fade(paint.blue, 0.5));
  // The bead rides the curve, so the loop closes at t = 2 pi.
  for (let i = 0; i < 12; i++) {
    const [x, y] = at(t - i * 0.08);
    pen.disc(x, y, 7 - i * 0.5, paint.fade(paint.orange, 1 - i / 12));
  }
}

const paint = ink(dark);
const pen = raster(960, 720, paint.ground);

// A contact sheet: twelve moments of the same figure.
pen
  .area(0.02)
  .panels(3, 4, 8)
  .forEach((box, i) => {
    pen.rect(box.x, box.y, box.w, box.h, paint.panel);
    draw(pen, box.square(), paint, (i / 12) * 2 * Math.PI);
  });

save(import.meta.file, pen);
