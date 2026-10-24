import * as math from "@mrlyprod/mrly/math";
import { plate } from "../../lib/plate.js";

export const units = { math };

const CODE = 495;
const NUMBER = 3;
const LEVEL = 1;
const BASE = 3;
const RING = 7;
const WHEEL = 3;
const REACH = 0.9;
const SAMPLES = 420;
const THICK = 0.01;
const BANDS = 3;
const ALPHA = [130, 190, 255];
const HUB = 0.8;
const WANT = { pencils: 8, curves: 8, nodes: 1288n };

let plan = null;

function study() {
  if (plan) return plan;
  const s = math.spirograph;
  const design = math.two.create(CODE, NUMBER, LEVEL, 0, BASE);
  const [height, width] = design.shape;
  const track = s.track("in", RING, WHEEL, 4, 1);
  const pencils = s.pencils(design.types, width, height, "fill", REACH, 0, 1);
  const curves = s.distinct(track, pencils, true);
  const nodes = s.nodes(track, pencils, true);
  if (pencils.length !== WANT.pencils || curves !== WANT.curves || nodes !== WANT.nodes)
    throw new Error(
      `spirograph: ${pencils.length} pencils, ${curves} curves, ${nodes} nodes, want ${WANT.pencils}, ${WANT.curves}, ${WANT.nodes}`,
    );
  const radii = pencils.map((p) => Math.hypot(p.x, p.y));
  const lo = Math.min(...radii);
  const hi = Math.max(...radii);
  const bands = radii.map((r) =>
    hi > lo ? Math.min(BANDS - 1, Math.floor(((r - lo) / (hi - lo)) * BANDS)) : BANDS - 1,
  );
  const trace = s.trace(track, pencils, SAMPLES);
  let outer = 0;
  let inner = Infinity;
  for (let i = 0; i < trace.length; i += 2) {
    const d = Math.hypot(trace[i], trace[i + 1]);
    outer = Math.max(outer, d);
    inner = Math.min(inner, d);
  }
  plan = { pencils, trace, bands, outer, inner };
  return plan;
}

export default function draw(pen) {
  const { box, paint, size } = plate(pen, "pink", 0.16);
  const { pencils, trace, bands, outer, inner } = study();
  const [cx, cy] = box.center();
  const k = box.radius() / outer;
  const thick = size * THICK;
  for (let band = 0; band < BANDS; band++) {
    const color = [255, 255, 255, ALPHA[band]];
    pencils.forEach((_, p) => {
      if (bands[p] !== band) return;
      const pts = [];
      for (let i = 0; i < SAMPLES; i++)
        pts.push([cx + trace[(p * SAMPLES + i) * 2] * k, cy - trace[(p * SAMPLES + i) * 2 + 1] * k]);
      pen.polyline(pts, thick, color);
    });
  }
  pen.disc(cx, cy, inner * k * HUB, paint.light);
}
