import { Grid, plot, type Color, type Ink, type Pen, type Point } from "mrlyjs/view";
import census from "./census/wiki-transfer-matrix.json" with { type: "json" };

const STATES = 3;
const LENGTH = 8;
const TAU = 2 * Math.PI;

export const units = {};

function dart(pen: Pen, tip: Point, way: Point, head: number, color: Color) {
  const side: Point = [-way[1], way[0]];
  const base: Point = [tip[0] - way[0] * head, tip[1] - way[1] * head];
  pen.triangle(tip, [base[0] + side[0] * head * 0.5, base[1] + side[1] * head * 0.5], [base[0] - side[0] * head * 0.5, base[1] - side[1] * head * 0.5], color);
}

export default function draw(pen: Pen, ink: Ink) {
  const { rule, walks } = census;
  if (rule.length !== STATES || rule.some((row) => row.length !== STATES)) throw new Error(`wiki-transfer-matrix: rule is not ${STATES} by ${STATES}`);
  if (walks.length !== LENGTH) throw new Error(`wiki-transfer-matrix: ${walks.length} walk counts, want ${LENGTH}`);

  const frame = pen.frame(0.08);
  const halves = frame.rows(2);
  const panels = halves[0].cols(2);

  const stage = panels[0].square().inset(panels[0].w * 0.06);
  const [cx, cy] = stage.center();
  const reach = stage.w * 0.34;
  const knob = stage.w * 0.1;
  const seats: Point[] = Array.from({ length: STATES }, (_, state) => {
    const turn = TAU * (state / STATES - 0.25);
    return [cx + reach * Math.cos(turn), cy + reach * Math.sin(turn)];
  });
  const head = knob * 0.62;
  for (const [from, to] of [
    [0, 1],
    [1, 2],
    [2, 0],
  ]) {
    const [a, b] = [seats[from], seats[to]];
    const span = Math.sqrt((b[0] - a[0]) ** 2 + (b[1] - a[1]) ** 2);
    const way: Point = [(b[0] - a[0]) / span, (b[1] - a[1]) / span];
    const start: Point = [a[0] + way[0] * knob * 1.12, a[1] + way[1] * knob * 1.12];
    const tip: Point = [b[0] - way[0] * knob * 1.12, b[1] - way[1] * knob * 1.12];
    pen.segment(start, tip, knob * 0.22, ink.yellow);
    dart(pen, tip, way, head, ink.yellow);
  }
  const loopAt: Point = [seats[0][0], seats[0][1] - knob * 1.35];
  pen.ring(loopAt[0], loopAt[1], knob * 0.92, knob * 0.22, ink.yellow);
  for (const seat of seats) pen.disc(seat[0], seat[1], knob, ink.blue);
  const turn = TAU / 6;
  const tip: Point = [loopAt[0] + knob * 0.92 * Math.cos(turn), loopAt[1] + knob * 0.92 * Math.sin(turn)];
  dart(pen, tip, [-Math.sin(turn), Math.cos(turn)], head, ink.yellow);

  const table = panels[1].square().inset(panels[1].w * 0.12);
  const grid = new Grid(table, STATES, STATES, 0.1);
  rule.forEach((line, row) => {
    line.forEach((cell, col) => {
      grid.fill(pen, col, row, cell === 1 ? ink.yellow : ink.fade(ink.line, 0.45));
    });
  });

  const chart = halves[1].inset(halves[1].h * 0.14);
  plot.bars(pen, chart, walks, 0.34, ink.blue);
  plot.baseline(pen, chart, ink.line);
}
