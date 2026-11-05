// One drawing, two renderers. A figure only knows the Pen, never where it lands.
import type { Ink, Pen } from "@mrlyprod/draw";
import { ink, raster, svg } from "@mrlyprod/draw";
import { dark } from "@mrlyprod/draw/theme.ts";
import { save, saveSvg } from "./save.ts";

function draw(pen: Pen, paint: Ink) {
  const box = pen.frame(0.1);
  const center = box.center();
  for (let i = 0; i < 12; i++) {
    const turn = (i / 12) * 2 * Math.PI;
    const tip: [number, number] = [
      center[0] + Math.cos(turn) * box.radius(),
      center[1] + Math.sin(turn) * box.radius(),
    ];
    pen.segment(center, tip, 3, paint.inks[i % paint.inks.length]);
    pen.disc(tip[0], tip[1], 10, paint.fg);
  }
}

const paint = ink(dark);

const pixels = raster(480, 480, paint.ground);
draw(pixels, paint);
save(import.meta.file, pixels);

const vector = svg(480, 480, paint.ground);
draw(vector, paint);
await saveSvg(import.meta.file, vector);
