import * as num from "mrlyjs/num";
import { plot, type Ink, type Pen, type Point } from "mrlyjs/view";

export const units = { num };

const REACH = 20;

export default function draw(pen: Pen, ink: Ink) {
  const frame = pen.frame(0.09);
  const window = new num.gauss.Window("Gaussian", REACH);
  const census = window.census();
  if (census.points !== 1681) throw new Error(`wiki-gaussian-integers: ${census.points} points, want 1681`);
  if (census.ramified !== 4) throw new Error(`wiki-gaussian-integers: ${census.ramified} ramified, want 4`);
  if (census.split + census.inert + census.ramified !== census.primes) throw new Error("wiki-gaussian-integers: split, inert and ramified do not sum to the primes");
  const step = frame.w / (2 * REACH);
  const [cx, cy] = frame.center();
  const place = (a: bigint, b: bigint): Point => [cx + Number(a) * step, cy - Number(b) * step];
  const broken: Point[] = [];
  const whole: Point[] = [];
  for (const [a, b] of window.points()) {
    const kind = window.class(a, b);
    if (kind === "Split" || kind === "Ramified") broken.push(place(a, b));
    else if (kind === "Inert") whole.push(place(a, b));
  }
  window.free();
  if (broken.length !== census.split + census.ramified) throw new Error(`wiki-gaussian-integers: ${broken.length} broken points, want ${census.split + census.ramified}`);
  if (whole.length !== census.inert) throw new Error(`wiki-gaussian-integers: ${whole.length} whole points, want ${census.inert}`);
  const dot = step * 0.34;
  plot.dots(pen, broken, dot, ink.blue);
  plot.dots(pen, whole, dot, ink.orange);
}
