// Every shape a pen can draw, one per panel.
import { ink, raster } from "@mrlyprod/draw";
import { dark } from "@mrlyprod/draw/theme.ts";
import { save } from "./save.ts";

const paint = ink(dark);
const pen = raster(720, 720, paint.ground);
const [a, b, c, d, e, f, g, h, i] = pen.area(0.04).panels(3, 3, 24);

pen.rect(a.x, a.y, a.w, a.h, paint.red);
pen.round_rect(b.x, b.y, b.w, b.h, b.w / 5, paint.orange);
pen.disc(...c.center(), c.radius(), paint.yellow);
pen.ring(...d.center(), d.radius() * 0.8, 16, paint.green);
pen.segment(e.at(0, 1), e.at(1, 0), 12, paint.mint);
pen.polyline([f.at(0, 1), f.at(0.33, 0), f.at(0.66, 1), f.at(1, 0)], 10, paint.cyan);
pen.triangle(g.at(0.5, 0), g.at(1, 1), g.at(0, 1), paint.blue);
pen.polygon([h.at(0.5, 0), h.at(1, 0.4), h.at(0.8, 1), h.at(0.2, 1), h.at(0, 0.4)], paint.indigo);
pen.arc(i.center(), i.radius() * 0.8, [0, 4.5], 16, paint.pink);

save(import.meta.file, pen);
