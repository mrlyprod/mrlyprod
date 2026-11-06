import type { Paints } from "@mrlyprod/ui/kit/color.ts";
import { blend } from "@mrlyprod/ui/kit/color.ts";
import type { Mrly } from "@mrlyprod/ui/kit/designs/engine.ts";
import type { Shape } from "@mrlyprod/ui/kit/designs/solid.ts";
import { camera, extent, PHI, solid } from "@mrlyprod/ui/kit/designs/solid.ts";
import { cube } from "@mrlyprod/ui/kit/designs/thumb.ts";
import type { Crop } from "./engine.ts";
import { flat, memo, region, tensor } from "./engine.ts";

export type Pose = "corner" | "edge" | "face";

export type Angles = [number, number];

const FULL = "255";
const NUMBER = 2;
const LEVEL = 4;
const RADIUS = 50;
const FIT = 0.9;
const LOW = 0.1;
const QUARTER = Math.PI / 4;

export const POSES: Record<Pose, Angles> = { corner: [QUARTER, PHI], edge: [QUARTER, LOW], face: [0, LOW] };

export const BLOCK: Shape = { n: 1, cells: Int32Array.of(0), masks: Uint8Array.of(63) };

const whole = memo(1);
const held = memo(8);

export function block(mrly: Mrly, crop: Crop | "" = ""): Shape {
  return held(crop || "none", () => {
    const stock = whole("block", () => mrly.three.create(FULL, NUMBER, LEVEL, 2));
    const cell = crop ? flat(mrly.crop.crop(tensor(stock), region(mrly, crop, RADIUS, false), true)) : stock;
    return solid(mrly.three.quads(cell), cell.shape[0]);
  });
}

export function glance(
  ctx: CanvasRenderingContext2D,
  shape: Shape,
  [theta, phi]: Angles,
  side: number,
  { paper, accent }: Pick<Paints, "paper" | "accent">,
) {
  const [w, h] = extent(shape.n, phi);
  const cam = camera(theta, phi, (FIT * side) / Math.max(w, h));
  ctx.setTransform(1, 0, 0, 1, 0, 0);
  ctx.clearRect(0, 0, side, side);
  cube(
    ctx,
    shape,
    cam,
    side / 2,
    side / 2,
    cam.tone.map((k) => blend(paper, accent, k)),
  );
}
