import * as num from "mrlyjs/num";
import { frame as box, plot } from "mrlyjs/view";
import type { Color, Frame, Ink, Pen, Point } from "mrlyjs/view";

export const units = { num };

const TOP = 2000;
const FLOOR = 2;
const SAMPLES = 240;
const DECADES = 4.5;
const LEFT = FLOOR;
const RIGHT = TOP;

// DATA

function ladder() {
  const lo = Math.log(FLOOR);
  const hi = Math.log(TOP);
  const out: number[] = [];
  for (let k = 0; k < SAMPLES; k++) {
    const step = lo + ((hi - lo) * k) / (SAMPLES - 1);
    const at = Math.round(Math.exp(step));
    if (out[out.length - 1] !== at) out.push(at);
  }
  return out;
}

let memo: { rungs: number[]; gauges: number[][] } | undefined;

function data() {
  if (memo) return memo;
  const { series, prime } = num;
  const rungs = ladder();
  const chasers: [(n: number) => number, number][] = [
    [series.wallis_half_pi, Math.PI / 2],
    [series.leibniz, Math.PI / 4],
    [series.basel, series.BASEL()],
    [series.e_partial, Math.E],
    [series.euler_gamma_partial, series.EULER()],
  ];
  const gauges = chasers.map(([partial, limit]) => rungs.map((m) => Math.abs(partial(m) - limit) / limit));
  gauges.push(
    rungs.map((m) => {
      const li = series.li(m);
      return Math.abs(prime.prime_count(m) - li) / li;
    }),
  );
  gauges.push(rungs.map((m) => 1 / prime.goldbach(2 * m)));
  gauges.push(
    rungs.map((m) => {
      const walk = series.mertens(m);
      return Number(walk < 0n ? -walk : walk) / Math.sqrt(m);
    }),
  );
  memo = { rungs, gauges };
  return memo;
}

// DRAW

function place(panel: Frame, at: number, gauge: number): Point | null {
  if (gauge <= 0) return null;
  const across = (Math.log10(at) - Math.log10(LEFT)) / (Math.log10(RIGHT) - Math.log10(LEFT));
  const drop = Math.min(Math.max(-Math.log10(gauge), 0), DECADES) / DECADES;
  return [panel.x + panel.w * across, panel.y + panel.h * drop];
}

function trace(panel: Frame, rungs: number[], gauge: number[]) {
  const path: Point[] = [];
  rungs.forEach((m, i) => {
    const point = place(panel, m, gauge[i]);
    if (point) path.push(point);
  });
  return path;
}

function stage(pen: Pen, panel: Frame, path: Point[], color: Color, line: Color) {
  pen.polyline(path, 2.6, color);
  plot.axis(pen, panel, line);
}

export default function draw(pen: Pen, ink: Ink) {
  const { rungs, gauges } = data();
  const frame = pen.frame(0.08);
  const panels: Frame[] = [];
  for (const row of frame.rows(2)) {
    for (const cell of row.cols(4)) {
      panels.push(box(cell.x + cell.w * 0.04, cell.y + cell.h * 0.09, cell.w * 0.92, cell.h * 0.82));
    }
  }
  if (panels.length !== 8) throw new Error(`wiki-famous-formulas: ${panels.length} panels, want 8`);

  for (let i = 0; i < 5; i++) pen.polyline(trace(panels[i], rungs, gauges[i]), 2.6, ink.blue);
  for (const panel of panels.slice(0, 5)) plot.axis(pen, panel, ink.line);

  const counted = trace(panels[5], rungs, gauges[5]);
  stage(pen, panels[5], counted, ink.yellow, ink.line);

  const comet = trace(panels[6], rungs, gauges[6]);
  stage(pen, panels[6], comet, ink.yellow, ink.line);

  const meter = trace(panels[7], rungs, gauges[7]);
  plot.dots(pen, meter, 2.4, ink.orange);
  plot.axis(pen, panels[7], ink.line);

  if (rungs[0] !== FLOOR) throw new Error(`wiki-famous-formulas: the ladder starts at ${rungs[0]}, want ${FLOOR}`);
  if (rungs[rungs.length - 1] !== TOP) throw new Error(`wiki-famous-formulas: the ladder ends at ${rungs[rungs.length - 1]}, want ${TOP}`);
  if (counted.length !== rungs.length) throw new Error(`wiki-famous-formulas: ${counted.length} counted points on ${rungs.length} rungs`);
  if (comet.length !== rungs.length) throw new Error(`wiki-famous-formulas: ${comet.length} comet points on ${rungs.length} rungs`);
  if (!(meter.length < rungs.length)) throw new Error(`wiki-famous-formulas: ${meter.length} meter points on ${rungs.length} rungs, want fewer`);
}
