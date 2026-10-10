import * as math from "mrlyjs/math";
import * as num from "mrlyjs/num";
import { Grid, frame as boxed, plot, type Color, type Frame, type Ink, type Pen, type Point } from "mrlyjs/view";

export const units = { math, num };

const CODE = 7;
const SIDES = [3, 5, 7];
const SPAN = 105;
const CELL = 5;
const REACH = 5;
const STOPS = 400;
const TALL = 300;

// WALK

type Data = { types: ArrayLike<number>; stops: number[]; walk: Float64Array; constant: number };

let memo: Data | undefined;

function data() {
  if (memo) return memo;
  const layers = SIDES.map((side) => math.bang.MagicLayer.new(new math.name.Bang(CODE, 2, 2), side));
  const word = math.bang.magic(layers);
  if (word.shape.length !== 2 || word.shape[0] !== SPAN || word.shape[1] !== SPAN) throw new Error(`demo-staircase: the word is ${word.shape}, want ${SPAN},${SPAN}`);
  const types = word.data;
  let filled = 0;
  for (const kind of types) if (kind !== 0) filled++;
  if (filled !== 6720) throw new Error(`demo-staircase: ${filled} cells, want 6720`);

  const profile = num.sieve.row_profile(CODE, 2);
  const row = num.sieve.row_law(profile);
  if (row.closed !== "3 pi/(4 Gamma(1/3))") throw new Error(`demo-staircase: closed form ${row.closed}, want 3 pi/(4 Gamma(1/3))`);
  if (!(Math.abs(row.constant - 0.8795254014476253) < 1e-14)) throw new Error(`demo-staircase: constant ${row.constant}, want 0.8795254014476253`);
  const stops: number[] = [];
  for (let i = 0; i < STOPS; i++) {
    const stop = Math.round(Math.pow(10, (REACH * i) / (STOPS - 1)));
    if (stops.length === 0 || stops[stops.length - 1] !== stop) stops.push(stop);
  }
  const walk = num.sieve.row_settle(profile, stops, false);
  if (!(Math.abs(walk[walk.length - 1] - row.constant) < 1e-5)) throw new Error(`demo-staircase: the walk ends at ${walk[walk.length - 1]}, want ${row.constant}`);
  memo = { types, stops, walk, constant: row.constant };
  return memo;
}

// DRAW

function mapped(box: Frame, x: number, y: number, low: number, high: number): Point {
  return box.at(x, 1 - (y - low) / (high - low));
}

function rule(pen: Pen, box: Frame, y: number, low: number, high: number, color: Color) {
  pen.segment(mapped(box, 0, y, low, high), mapped(box, 1, y, low, high), 2, color);
}

export default function draw(pen: Pen, ink: Ink) {
  const { types, stops, walk, constant } = data();
  let high = -Number.MAX_VALUE;
  for (const value of walk) high = Math.max(high, value);
  const low = constant - 0.06 * (high - constant);

  const margin = Math.round(pen.width * 0.08);
  const plate = CELL * SPAN;
  const lattice = new Grid(boxed(margin, margin, plate, plate), SPAN, SPAN, 0);
  for (let row = 0; row < SPAN; row++) {
    for (let col = 0; col < SPAN; col++) {
      lattice.fill(pen, col, row, types[row * SPAN + col] !== 0 ? ink.blue : ink.panel);
    }
  }

  const panel = boxed(pen.width - margin - plate, pen.height - margin - TALL, plate, TALL);
  plot.axis(pen, panel, ink.line);
  const inner = panel.inset(24);
  rule(pen, inner, constant, low, high, ink.dim);
  const path = stops.map((level, i): Point => mapped(inner, Math.log10(level) / REACH, walk[i], low, high));
  pen.polyline(path, 4, ink.yellow);
}
