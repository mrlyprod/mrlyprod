import type { Pen } from "@mrlyprod/draw";
import { face, smile } from "../own/face.ts";
import { GLINT, SHADE } from "../own/palette.ts";
import { flip, round } from "../own/path.ts";

const LENS = round(
  [
    [3.6, 10],
    [14.6, 10],
    [13.8, 15.4],
    [11.4, 17.2],
    [6.6, 17.2],
    [4.4, 15.4],
  ],
  [1, 1.6, 2.4, 2, 2, 2.4],
  true,
);

export default function draw(pen: Pen) {
  face(pen);
  pen.segment([3.4, 10.6], [28.6, 10.6], 1.8, SHADE);
  for (const lens of [LENS, flip(LENS, 16)]) pen.polygon(lens, SHADE);
  for (const x of [6.2, 17.2]) pen.segment([x, 14.4], [x + 2, 12.4], 1.3, GLINT);
  smile(pen, 13.6, 8.2, 0.2);
}
