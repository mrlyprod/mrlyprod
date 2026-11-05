// Three dimensions with no camera: isometric faces, shaded by which way they look.
import { ink, iso, raster } from "@mrlyprod/draw";
import { dark } from "@mrlyprod/draw/theme.ts";
import { save } from "./save.ts";

type Vec3 = iso.Vec3;

const paint = ink(dark);
const pen = raster(480, 480, paint.ground);

const v = (x: number, y: number, z: number): Vec3 => ({ x, y, z });

// Only faces whose normal points at the viewer get drawn.
const cube: iso.Quad[] = [
  { normal: v(0, 0, 1), verts: [v(0, 0, 1), v(1, 0, 1), v(1, 1, 1), v(0, 1, 1)] },
  { normal: v(0, 1, 0), verts: [v(0, 1, 0), v(1, 1, 0), v(1, 1, 1), v(0, 1, 1)] },
  { normal: v(1, 0, 0), verts: [v(1, 0, 0), v(1, 1, 0), v(1, 1, 1), v(1, 0, 1)] },
];

// The shades are top, then the y side, then the x side.
iso.draw(pen, pen.frame(0.2), cube, [paint.cyan, paint.blue, paint.indigo], paint.ground);

save(import.meta.file, pen);
