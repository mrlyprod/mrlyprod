import type { Pen } from "@mrlyprod/draw";
import { BROWN, FACE, GLOW, MOUTH, RIM, TEETH, TONGUE } from "./palette.ts";
import { oval } from "./path.ts";
import { band } from "./shape.ts";

export const LINE = 2;

export function face(pen: Pen) {
  pen.disc(16, 16, 14.5, RIM);
  pen.disc(16, 15.4, 13.9, FACE);
  pen.disc(16, 13.8, 12, GLOW);
}

export function eye(pen: Pen, x: number, y = 12.4) {
  pen.polygon(oval(x, y, 2, 2.9), BROWN);
}

export function eyes(pen: Pen, y = 12.4) {
  eye(pen, 10.8, y);
  eye(pen, 21.2, y);
}

export function shut(pen: Pen, x: number, y = 13.6) {
  pen.arc([x, y], 3, [Math.PI + 0.55, 2 * Math.PI - 0.55], LINE, BROWN);
}

export function smile(pen: Pen, cy: number, r: number, spread = 0.2) {
  pen.arc([16, cy], r, [Math.PI * (0.5 - spread), Math.PI * (0.5 + spread)], LINE, BROWN);
}

type Mouth = { top: number; half: number; depth: number; sag?: number; teeth?: number; tongue?: boolean };

export function mouth(pen: Pen, { top, half, depth, sag = 1.4, teeth = 2.8, tongue = true }: Mouth) {
  const u = (x: number) => Math.min(1, Math.abs(x - 16) / half);
  const upper = (x: number) => top + sag * (1 - u(x) ** 2);
  const lower = (x: number) => top + depth * Math.sqrt(1 - u(x) ** 2);
  const [x0, x1] = [16 - half, 16 + half];
  pen.polygon(band(upper, lower, x0, x1), MOUTH);
  pen.polygon(
    band(upper, (x) => Math.min(lower(x), upper(x) + teeth), x0, x1),
    TEETH,
  );
  if (tongue) {
    const lick = (x: number) => Math.max(upper(x) + teeth + 0.6, top + depth - 3.4 + 5 * ((x - 16) / half) ** 2);
    pen.polygon(
      band(lick, (x) => lower(x) - 0.01, x0, x1),
      TONGUE,
    );
  }
}
