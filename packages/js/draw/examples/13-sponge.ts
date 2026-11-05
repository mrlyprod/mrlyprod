// A Menger sponge: thousands of faces from one rule, hidden faces dropped first.
import { ink, iso, raster } from "@mrlyprod/draw";
import { dark } from "@mrlyprod/draw/theme.ts";
import { save } from "./save.ts";

const paint = ink(dark);
const pen = raster(900, 900, paint.ground);
const LEVEL = 3;
const SIDE = 3 ** LEVEL;

// A cell survives unless two of its coordinates have a middle digit at the same scale.
function filled(x: number, y: number, z: number): boolean {
  if (x < 0 || y < 0 || z < 0 || x >= SIDE || y >= SIDE || z >= SIDE) return false;
  for (let scale = 1; scale < SIDE; scale *= 3) {
    const middle = [x, y, z].filter((one) => Math.floor(one / scale) % 3 === 1).length;
    if (middle >= 2) return false;
  }
  return true;
}

const v = (x: number, y: number, z: number): iso.Vec3 => ({ x, y, z });

const quads: iso.Quad[] = [];
for (let x = 0; x < SIDE; x++) {
  for (let y = 0; y < SIDE; y++) {
    for (let z = 0; z < SIDE; z++) {
      if (!filled(x, y, z)) continue;
      const [X, Y, Z] = [x + 1, y + 1, z + 1];
      // A face behind a filled neighbour can never be seen.
      if (!filled(x, y, Z)) quads.push({ normal: v(0, 0, 1), verts: [v(x, y, Z), v(X, y, Z), v(X, Y, Z), v(x, Y, Z)] });
      if (!filled(x, Y, z)) quads.push({ normal: v(0, 1, 0), verts: [v(x, Y, z), v(X, Y, z), v(X, Y, Z), v(x, Y, Z)] });
      if (!filled(X, y, z)) quads.push({ normal: v(1, 0, 0), verts: [v(X, y, z), v(X, Y, z), v(X, Y, Z), v(X, y, Z)] });
    }
  }
}

iso.draw(pen, pen.frame(0.05), quads, [paint.fg, paint.teal, paint.indigo]);

save(import.meta.file, pen);
