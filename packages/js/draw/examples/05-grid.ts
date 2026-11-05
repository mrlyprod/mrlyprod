// A Grid hands out cells that land on whole pixels, so no seams show.
import { Grid, ink, raster } from "@mrlyprod/draw";
import { dark } from "@mrlyprod/draw/theme.ts";
import { save } from "./save.ts";

const paint = ink(dark);
const pen = raster(480, 480, paint.ground);
const cells = new Grid(pen.frame(0.05), 8, 8, 0);

for (let row = 0; row < 8; row++) {
  for (let col = 0; col < 8; col++) {
    if ((row + col) % 2) continue;
    pen.rect(...cells.cell(col, row), paint.fg);
  }
}

save(import.meta.file, pen);
