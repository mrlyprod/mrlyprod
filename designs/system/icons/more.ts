import type { Color, Pen } from "@mrlyprod/draw";
import { stamp } from "../own/stamp.ts";

const ROWS = [".......", ".......", ".......", ".#.#.#.", ".......", ".......", "......."];

export default function draw(pen: Pen, ink: Color) {
  stamp(pen, ROWS, ink);
}
