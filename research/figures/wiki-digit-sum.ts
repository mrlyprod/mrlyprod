import { plot, type Ink, type Pen, type Point } from "mrlyjs/view";

const BASE = 10;
const TOP = 100;
const PEAK = 18;
const LINES = [0, 9, 18];

function digitSum(n: number) {
  let sum = 0;
  while (n > 0) {
    sum += n % BASE;
    n = Math.floor(n / BASE);
  }
  return sum;
}

export default function draw(pen: Pen, ink: Ink) {
  const sums = Array.from({ length: TOP }, (_, n) => digitSum(n));
  if (sums.some((s, n) => (s - n) % (BASE - 1) !== 0)) throw new Error("wiki-digit-sum: a digit sum is off its number by more than a multiple of 9");
  if (Math.max(...sums) !== PEAK) throw new Error(`wiki-digit-sum: peak ${Math.max(...sums)}, want ${PEAK}`);
  const nines = sums.filter((_, n) => n % 9 === 0).length;
  const onLines = sums.filter((s) => LINES.includes(s)).length;
  const heights = LINES.map((line) => sums.filter((s) => s === line).length);
  if (nines !== 12 || onLines !== 12 || heights.join() !== "1,10,1") throw new Error(`wiki-digit-sum: ${nines} multiples of 9, ${onLines} on the lines, heights ${heights}, want 12, 12, 1,10,1`);
  const frame = pen.frame(0.08);
  const slot = frame.w / TOP;
  const at = (n: number): Point => [frame.x + (n + 0.5) * slot, frame.y + frame.h * (1 - sums[n] / PEAK)];
  for (const line of LINES) pen.segment([frame.x, frame.y + frame.h * (1 - line / PEAK)], [frame.x + frame.w, frame.y + frame.h * (1 - line / PEAK)], 1.8, ink.fade(ink.dim, 0.55));
  pen.polyline(sums.map((_, n) => at(n)), 1.4, ink.fade(ink.dim, 0.5));
  plot.dots(pen, sums.map((_, n) => at(n)).filter((_, n) => n % 9 !== 0), slot * 0.44, ink.blue);
  plot.dots(pen, sums.map((_, n) => at(n)).filter((_, n) => n % 9 === 0), slot * 0.66, ink.yellow);
}
