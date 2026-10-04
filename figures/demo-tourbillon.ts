import * as math from "mrlyjs/math";
import { field, type Ink, type Pen } from "mrlyjs/view";

export const units = { math };

const TOP = 55;
const RASTER = 1024;
const LAYERS = 28;
const CENTRE = 14 / 28;
const SEED = 1;
const MARGIN = 0.08;
const CURVE = 1.45;

// FIELD

function shrink(source: Float32Array, size: number, side: number) {
  const step = size / side;
  const span = (k: number): [number, number] => [k * step, (k + 1) * step];
  const out = new Float64Array(side * side);
  let n = 0;
  for (let row = 0; row < side; row++) {
    const [top, bottom] = span(row);
    for (let column = 0; column < side; column++) {
      const [left, right] = span(column);
      let total = 0;
      for (let y = Math.floor(top); y < Math.min(Math.ceil(bottom), size); y++) {
        const height = Math.min(bottom, y + 1) - Math.max(top, y);
        for (let x = Math.floor(left); x < Math.min(Math.ceil(right), size); x++) {
          const width = Math.min(right, x + 1) - Math.max(left, x);
          const value = source[y * size + x];
          if (!Number.isNaN(value)) total += height * width * value;
        }
      }
      out[n++] = total / (step * step);
    }
  }
  return out;
}

let memo: { spun: Float32Array; centre: number } | undefined;

function spin() {
  if (memo) return memo;
  const list = math.tourbillon.layers(TOP, "golden", 0, "odd", "plain", SEED);
  if (list.length !== LAYERS) throw new Error(`demo-tourbillon: ${list.length} layers, want ${LAYERS}`);
  if (list[0].scale !== 1) throw new Error(`demo-tourbillon: the first scale is ${list[0].scale}, want 1`);
  if (list[LAYERS - 1].scale !== TOP) throw new Error(`demo-tourbillon: the last scale is ${list[LAYERS - 1].scale}, want ${TOP}`);
  const spun = math.tourbillon.field(TOP, RASTER, "golden", 0, "odd", "plain", "cells", "mean", SEED);
  if (spun.length !== RASTER * RASTER) throw new Error(`demo-tourbillon: the field holds ${spun.length} sites, want ${RASTER * RASTER}`);
  const read = math.tourbillon.stats(spun, RASTER, TOP, "golden", 0, "odd", "plain", "mean", SEED);
  if (read.layers !== LAYERS) throw new Error(`demo-tourbillon: the stats read ${read.layers} layers, want ${LAYERS}`);
  if (read.centre !== CENTRE) throw new Error(`demo-tourbillon: the centre is ${read.centre}, want ${CENTRE}`);
  memo = { spun, centre: read.centre };
  return memo;
}

// DRAW

export default function draw(pen: Pen, ink: Ink) {
  const { spun, centre } = spin();
  const frame = pen.frame(MARGIN);
  const side = Math.round(frame.w);
  const disc = shrink(spun, RASTER, side).map((value) => Math.pow(Math.min(Math.max(value / centre, 0), 1), CURVE));
  if (disc.length !== side * side) throw new Error(`demo-tourbillon: the disc holds ${disc.length} sites, want ${side * side}`);
  const ramp = new ink.Ramp([ink.ground, ink.indigo, ink.pink]);
  field.draw_range(pen, frame, side, side, disc, [0, 1], ramp);
}
