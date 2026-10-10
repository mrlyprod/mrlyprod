import * as math from "mrlyjs/math";
import { plot, type Ink, type Pen, type Point } from "mrlyjs/view";

export const units = { math };

const WINDOW = 0.1;

type Spectrum = { points: [number, number][]; intercept: number; slope: number; fitted: number };

let memo: Spectrum | undefined;

function spectrum() {
  if (memo) return memo;
  const cell = math.six.cut(math.three.create(23, 3, 2, 2));
  const whole = math.six.slice_core_graph(cell);
  const pieces = math.graph.census(whole).components;
  const network = math.graph.largest_component(whole);
  const values = math.spectrum.laplacian_spectrum(network, true);
  const points = math.spectrum.spectral_points(values);
  const fit = math.spectrum.spectral_fit(values, WINDOW);
  if (!fit) throw new Error("demo-spectra: the low window does not fit");
  const [intercept, slope, fitted] = fit;
  const nodes = network.nodes.length;
  const branches = network.branches.length;
  if (nodes !== 306 || branches !== 378 || pieces !== 1) throw new Error(`demo-spectra: network of ${nodes} nodes, ${branches} branches, ${pieces} pieces, want 306, 378, 1`);
  const levels = math.spectrum.clusters(values, 1e-9).length;
  if (levels !== 179 || fitted !== 30) throw new Error(`demo-spectra: ${levels} levels, ${fitted} fitted, want 179, 30`);
  if (Math.abs(2 * slope - 1.253284) >= 1e-5) throw new Error(`demo-spectra: exponent ${2 * slope}, want 1.253284`);
  memo = { points, intercept, slope, fitted };
  return memo;
}

export default function draw(pen: Pen, ink: Ink) {
  const { points, intercept, slope, fitted } = spectrum();
  const frame = pen.frame(0.08);
  const xs = points.map((p) => Math.log(p[0]));
  const ys = points.map((p) => Math.log(p[1]));
  const left = xs[0];
  const right = xs[xs.length - 1];
  const foot = ys[0];
  const roof = 0;
  const at = (x: number, y: number): Point => [frame.x + (frame.w * (x - left)) / (right - left), frame.y + frame.h * (1 - (y - foot) / (roof - foot))];
  const ray = (x: number) => intercept + slope * x;
  const reach = (from: number, to: number): [Point, Point] => {
    const low = Math.max((foot - intercept) / slope, from);
    const high = Math.min((roof - intercept) / slope, to);
    return [at(low, ray(low)), at(high, ray(high))];
  };

  const [edge] = at(xs[fitted - 1], roof);
  pen.rect(frame.x, frame.y, edge - frame.x, frame.h, ink.panel);
  plot.axis(pen, frame, ink.line);
  const stair = [at(xs[0], ys[0])];
  for (let index = 1; index < points.length; index++) {
    stair.push(at(xs[index], ys[index - 1]));
    stair.push(at(xs[index], ys[index]));
  }
  pen.polyline(stair, 4, ink.blue);
  const [a, b] = reach(left, right);
  pen.segment(a, b, 2.5, ink.fade(ink.yellow, 0.6));
  const [c, d] = reach(left, xs[fitted - 1]);
  pen.segment(c, d, 6, ink.yellow);
}
