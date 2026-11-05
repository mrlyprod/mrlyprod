// A Frame is a box you can slice, so layouts never hold raw numbers.
import { ink, raster } from "@mrlyprod/draw";
import { dark } from "@mrlyprod/draw/theme.ts";
import { save } from "./save.ts";

const paint = ink(dark);
const pen = raster(720, 480, paint.ground);

const [left, right] = pen.area(0.04).cols(2);
pen.rect(left.x, left.y, left.w - 8, left.h, paint.panel);

// square() finds the largest centered square; inset() pulls the edges in.
const inner = left.square().inset(32);
pen.round_rect(inner.x, inner.y, inner.w, inner.h, 16, paint.teal);

// rows() and cols() slice; panels() does both with a gap.
right.rows(2).forEach((row, i) => {
  row
    .inset(8)
    .cols(i + 2)
    .forEach((cell, j) => {
      const box = cell.inset(4);
      pen.rect(box.x, box.y, box.w, box.h, paint.inks[(i + j) % paint.inks.length]);
    });
});

save(import.meta.file, pen);
