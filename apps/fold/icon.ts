import type { Pen } from "@mrlyprod/draw";
import { plate } from "../../net/sdk/plate.ts";

const LACE = ["100001", "010010", "001100", "001100", "010010", "100001"];
const HOLE = 0.3;

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "mint", 0.2);
  const n = LACE.length;
  const c = box.w / n;
  pen.rect(box.x, box.y, box.w, box.h, paint.white);
  for (let y = 0; y < n; y++)
    for (let x = 0; x < n; x++)
      if (LACE[y][x] === "1") pen.disc(box.x + (x + 0.5) * c, box.y + (y + 0.5) * c, c * HOLE, paint.tone);
}
