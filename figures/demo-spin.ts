import * as math from "mrlyjs/math";
import { field, frame, type Ink, type Pen } from "mrlyjs/view";

const PANEL = 430;
const TRIANGLES = 486;
const STEPS = 2048;

export const units = { math };

type Panels = { source: Float32Array; wheel: Float32Array; peak: number };

let memo: Panels | undefined;

function panels(): Panels {
  if (memo) return memo;
  const cut = math.six.cut_design(23, 3, 2, 2);
  const tally = math.six.census(cut, false);
  if (tally.triangles !== TRIANGLES) throw new Error(`demo-spin: ${tally.triangles} triangles, want ${TRIANGLES}`);
  if (!(tally.fills > 0 && tally.fills < TRIANGLES)) throw new Error(`demo-spin: ${tally.fills} fills of ${TRIANGLES}`);

  const source = math.six.raster(cut, PANEL);
  if (source.length !== PANEL * PANEL) throw new Error(`demo-spin: raster of ${source.length}, want ${PANEL * PANEL}`);
  let lit = 0;
  for (const v of source) lit += v;
  if (!(lit > 0)) throw new Error("demo-spin: the raster is dark");

  const profile = math.spin.profile(source, PANEL, STEPS);
  if (profile.length !== STEPS) throw new Error(`demo-spin: profile of ${profile.length}, want ${STEPS}`);
  const mass = math.spin.mass(profile, PANEL);
  if (!(Math.abs(mass - lit) < 0.03 * lit)) throw new Error(`demo-spin: mass ${mass} against ${lit} lit`);

  const floor = Math.fround(1e-6);
  let reach = STEPS - 1;
  for (let i = STEPS - 1; i >= 0; i--) {
    if (profile[i] > floor) {
      reach = i;
      break;
    }
  }
  if (!(reach > STEPS / 2)) throw new Error(`demo-spin: reach ${reach}, want over ${STEPS / 2}`);
  const wheel = math.spin.wheel(profile.subarray(0, reach + 1), PANEL);
  if (wheel.length !== PANEL * PANEL) throw new Error(`demo-spin: wheel of ${wheel.length}, want ${PANEL * PANEL}`);
  let peak = -Number.MAX_VALUE;
  for (const v of wheel) peak = Math.max(peak, v);
  if (!(peak > 0)) throw new Error("demo-spin: the wheel is dark");

  memo = { source, wheel, peak };
  return memo;
}

export default function draw(pen: Pen, ink: Ink) {
  const { source, wheel, peak } = panels();
  const flat = ink.Ramp.tone(ink.ground, ink.blue);
  const heat = new ink.Ramp([ink.ground, ink.blue, ink.yellow]);
  const edge = (pen.width - 2 * PANEL) / 2;
  const first = frame(edge, edge, PANEL, PANEL);
  const second = frame(edge + PANEL, edge + PANEL, PANEL, PANEL);
  field.draw_range(pen, first, PANEL, PANEL, source, [0, 1], flat);
  field.draw_range(pen, second, PANEL, PANEL, wheel, [0, peak], heat);
}
