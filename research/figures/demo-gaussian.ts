import * as num from "mrlyjs/num";
import { plot, type Ink, type Pen, type Point } from "mrlyjs/view";

export const units = { num };

const REACH = 40;

export default function draw(pen: Pen, ink: Ink) {
  const frame = pen.frame(0.09);
  const window = new num.gauss.Window("Gaussian", REACH);
  const census = window.census();
  if (census.points !== 6561) throw new Error(`demo-gaussian: ${census.points} points, want 6561`);
  if (census.ramified !== 4) throw new Error(`demo-gaussian: ${census.ramified} ramified, want 4`);
  if (census.split + census.inert + census.ramified !== census.primes) throw new Error("demo-gaussian: split, inert and ramified do not sum to the primes");
  const step = frame.w / (2 * REACH);
  const [cx, cy] = frame.center();
  const place = (a: bigint, b: bigint): Point => [cx + Number(a) * step, cy - Number(b) * step];
  const split: Point[] = [];
  const inert: Point[] = [];
  const ramified: Point[] = [];
  for (const [a, b] of window.points()) {
    const kind = window.class(a, b);
    if (kind === "Split") split.push(place(a, b));
    else if (kind === "Inert") inert.push(place(a, b));
    else if (kind === "Ramified") ramified.push(place(a, b));
  }
  if (split.length !== census.split) throw new Error(`demo-gaussian: ${split.length} split points, want ${census.split}`);
  if (inert.length !== census.inert) throw new Error(`demo-gaussian: ${inert.length} inert points, want ${census.inert}`);
  if (ramified.length !== census.ramified) throw new Error(`demo-gaussian: ${ramified.length} ramified points, want ${census.ramified}`);
  const dot = step * 0.36;
  plot.dots(pen, split, dot, ink.blue);
  plot.dots(pen, inert, dot, ink.orange);
  plot.dots(pen, ramified, dot, ink.yellow);
}
