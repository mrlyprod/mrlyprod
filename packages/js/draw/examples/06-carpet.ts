// A rule grown into itself: the mark, drawn at its third level.
import { grid, ink, raster } from "@mrlyprod/draw";
import { dark } from "@mrlyprod/draw/theme.ts";
import { save } from "./save.ts";

const paint = ink(dark);
const pen = raster(750, 750, paint.ground);

// mask() substitutes the rule into every filled cell, level by level.
const cells = grid.mask(grid.LOGO, 3);
grid.carpet(pen, pen.frame(0.04), cells, 0, paint.fg);

save(import.meta.file, pen);
