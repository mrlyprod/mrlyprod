import * as math from "mrlyjs/math";
import { plot, type Color, type Frame, type Ink, type Pen, type Point } from "mrlyjs/view";

const VERTICES = 32;
const CEILING = 4;

export const units = { math };

type Spectra = { path: Float64Array; cycle: Float64Array };

let memo: Spectra | undefined;

function spectrum(closed: boolean) {
  const network = new math.graph.Network(1);
  for (let k = 0; k < VERTICES; k++) network.add_node([k]);
  for (let k = 1; k < VERTICES; k++) network.add_branch(k - 1, k, 1);
  if (closed) network.add_branch(VERTICES - 1, 0, 1);
  const values = math.spectrum.laplacian_spectrum(network, false);
  network.free();
  return values;
}

function spectra() {
  if (memo) return memo;
  const path = spectrum(false);
  const cycle = spectrum(true);
  if (path.length !== VERTICES || cycle.length !== VERTICES) throw new Error(`wiki-graph-laplacian: ${path.length} and ${cycle.length} eigenvalues, want ${VERTICES}`);
  const distinct = [math.spectrum.clusters(path, 1e-9).length, math.spectrum.clusters(cycle, 1e-9).length];
  if (distinct[0] !== 32 || distinct[1] !== 17) throw new Error(`wiki-graph-laplacian: ${distinct} distinct values, want 32,17`);
  if (!(Math.abs(path[0]) < 1e-12 && Math.abs(cycle[0]) < 1e-12)) throw new Error("wiki-graph-laplacian: a spectrum does not start at zero");
  if (!(Math.abs(cycle[VERTICES - 1] - CEILING) < 1e-12)) throw new Error(`wiki-graph-laplacian: the cycle tops at ${cycle[VERTICES - 1]}, want ${CEILING}`);
  memo = { path, cycle };
  return memo;
}

function steps(pen: Pen, frame: Frame, values: Float64Array, thick: number, paint: Color) {
  const slot = frame.w / values.length;
  const pts: Point[] = [];
  values.forEach((value, index) => {
    const y = frame.y + frame.h * (1 - value / CEILING);
    pts.push([frame.x + index * slot, y]);
    pts.push([frame.x + (index + 1) * slot, y]);
  });
  pen.polyline(pts, thick, paint);
  values.forEach((value, index) => {
    const y = frame.y + frame.h * (1 - value / CEILING);
    pen.disc(frame.x + (index + 0.5) * slot, y, thick * 0.9, paint);
  });
}

export default function draw(pen: Pen, ink: Ink) {
  const { path, cycle } = spectra();
  const frame = pen.frame(0.08);
  const stage = frame.inset(frame.w * 0.04);
  plot.baseline(pen, stage, ink.line);
  for (let k = 1; k < 4; k++) {
    const y = stage.y + stage.h * (1 - k / CEILING);
    pen.segment([stage.x, y], [stage.x + stage.w, y], 1.5, ink.fade(ink.dim, 0.7));
  }
  steps(pen, stage, cycle, 7, ink.orange);
  steps(pen, stage, path, 7, ink.blue);
}
