// An app icon: the plate gives a tinted squircle and the box to draw inside.
import { Grid, grid, raster } from "@mrlyprod/draw";
import { plate } from "@mrlyprod/draw/plate.ts";
import { save } from "./save.ts";

const pen = raster(512, 512);
const { box, paint } = plate(pen, "indigo", 0.24);

const cells = new Grid(box, 5, 5, 0.12);
grid.LOGO.forEach((row, y) => {
  [...row].forEach((on, x) => {
    if (on === "1") pen.round_rect(...cells.cell(x, y), 8, paint.white);
  });
});

save(import.meta.file, pen);
