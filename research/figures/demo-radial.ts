import * as math from "mrlyjs/math";
import { field, frame } from "mrlyjs/view";
import type { Ink, Pen } from "mrlyjs/view";

export const units = { math };

const LEVEL = 4;
const SIDE = 81;
const COPIES = 6;
const ORDER = 4;
const OUT = 860;
const RINGS = 160;
const ORDERS = 48;
const SAMPLES = 3;

type Stack = { values: Float32Array; high: number };

let memo: Stack | undefined;

function stack() {
  if (memo) return memo;
  const carpet = math.two.create(495, 3, LEVEL, 0, 3);
  if (carpet.shape[1] !== SIDE) throw new Error(`demo-radial: carpet width ${carpet.shape[1]}, want ${SIDE}`);
  let filled = 0;
  for (const kind of carpet.types) filled += kind;
  if (filled !== 8 ** LEVEL) throw new Error(`demo-radial: carpet holds ${filled} cells, want ${8 ** LEVEL}`);
  const source = Float32Array.from(carpet.types);
  if (source.length !== SIDE * SIDE) throw new Error(`demo-radial: source holds ${source.length} cells, want ${SIDE * SIDE}`);
  const power = math.spin.harmonics(source, SIDE, RINGS, ORDERS);
  if (math.spin.turns(power) !== ORDER) throw new Error(`demo-radial: carpet turns ${math.spin.turns(power)}, want ${ORDER}`);
  if (math.spin.petals(COPIES, ORDER) !== 12) throw new Error(`demo-radial: ${math.spin.petals(COPIES, ORDER)} petals, want 12`);
  const values = math.spin.radial(source, SIDE, OUT, COPIES, 1 / COPIES, "Mean", SAMPLES);
  if (values.length !== OUT * OUT) throw new Error(`demo-radial: stack holds ${values.length} samples, want ${OUT * OUT}`);
  const high = values.reduce((a, v) => Math.max(a, v), -Number.MAX_VALUE);
  if (!(high > 0.9)) throw new Error(`demo-radial: stack peaks at ${high}, want over 0.9`);
  memo = { values, high };
  return memo;
}

export default function draw(pen: Pen, ink: Ink) {
  const { values, high } = stack();
  const ramp = new ink.Ramp([ink.ground, ink.blue, ink.yellow]);
  const edge = (pen.width - OUT) / 2;
  field.draw_range(pen, frame(edge, edge, OUT, OUT), OUT, OUT, values, [0, high], ramp);
}
