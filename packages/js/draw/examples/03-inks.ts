// Figures name their inks instead of writing hex: one palette, many themes.
import { fade, ink, mix, NAMES, raster } from "@mrlyprod/draw";
import { dark } from "@mrlyprod/draw/theme.ts";
import { save } from "./save.ts";

const paint = ink(dark);
const pen = raster(720, 400, paint.ground);
const [top, middle, bottom] = pen.area(0.04).rows(3);

// One swatch per named ink.
top.cols(NAMES.length).forEach((cell, i) => {
  pen.rect(cell.x, cell.y, cell.w - 2, cell.h - 8, paint[NAMES[i]]);
});

// mix walks between two inks.
middle.cols(16).forEach((cell, i) => {
  pen.rect(cell.x, cell.y, cell.w, cell.h - 8, mix(paint.blue, paint.orange, i / 15));
});

// fade keeps the hue and lowers the alpha.
bottom.cols(16).forEach((cell, i) => {
  pen.rect(cell.x, cell.y, cell.w, cell.h - 8, fade(paint.pink, i / 15));
});

save(import.meta.file, pen);
