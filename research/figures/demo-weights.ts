import * as core from "mrlyjs/core";
import * as math from "mrlyjs/math";
import { field, frame, type Ink, type Pen } from "mrlyjs/view";

const CODE = "495";
const SIDE = 3;
const LEVEL = 5;
const SPAN = 243;
const MARGIN = 0.08;
const GAMMA = 0.12;
const SHARES = [5, 4, 3, 4, 2, 3, 2, 1];
const WHOLE = 24n;
const CORNERS = [0, 1, 2, 3, 5, 6, 7, 8];
const FILLED = 32768;
const PEAK = 3125;
const LEAST = 1;

export const units = { core, math };

// FIELD

let memo: Float64Array | undefined;

function reads() {
  if (memo) return memo;
  const tile = math.two.create(CODE, SIDE, 1, 0, SIDE);
  if (tile.shape[0] !== SIDE || tile.shape[1] !== SIDE) throw new Error(`demo-weights: tile ${tile.shape}, want ${SIDE} square`);
  const corners = Array.from(tile.types.keys()).filter((at) => tile.types[at] !== 0);
  if (corners.join() !== CORNERS.join()) throw new Error(`demo-weights: corners ${corners}, want ${CORNERS}`);
  const shares = new Uint32Array(SIDE * SIDE);
  corners.forEach((at, i) => (shares[at] = SHARES[i]));
  const weights = core.tensor.u32(shares, tile.shape);
  if (core.tensor.sum(weights) !== WHOLE) throw new Error(`demo-weights: shares sum ${core.tensor.sum(weights)}, want ${WHOLE}`);
  const mass = core.tensor.fractal(weights, LEVEL);
  if (mass.shape[0] !== SPAN || mass.shape[1] !== SPAN || mass.data.length !== SPAN * SPAN) throw new Error(`demo-weights: mass ${mass.shape}, want ${SPAN} square`);
  const filled = SPAN * SPAN - core.tensor.count(mass, 0);
  if (filled !== FILLED) throw new Error(`demo-weights: ${filled} filled cells, want ${FILLED}`);
  if (core.tensor.sum(mass) !== WHOLE ** BigInt(LEVEL)) throw new Error(`demo-weights: mass sums to ${core.tensor.sum(mass)}, want ${WHOLE ** BigInt(LEVEL)}`);
  let peak = 0;
  let least = Infinity;
  for (const m of mass.data) {
    peak = Math.max(peak, m);
    if (m > 0) least = Math.min(least, m);
  }
  if (peak !== PEAK || least !== LEAST) throw new Error(`demo-weights: mass runs ${least} to ${peak}, want ${LEAST} to ${PEAK}`);
  memo = Float64Array.from(mass.data, (m) => (m > 0 ? Math.pow(m / peak, GAMMA) : 0));
  return memo;
}

// DRAW

export default function draw(pen: Pen, ink: Ink) {
  const values = reads();
  const ramp = new ink.Ramp([ink.ground, ink.blue, ink.yellow]);
  const edge = Math.round(pen.width * MARGIN);
  const plate = pen.width - 2 * edge;
  field.draw_range(pen, frame(edge, edge, plate, plate), SPAN, SPAN, values, [0, 1], ramp);
}
