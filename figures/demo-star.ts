import { field, type Ink, type Pen } from "mrlyjs/view";
import census from "./census/demo-star.json" with { type: "json" };

type Sampled = { t: Float64Array; arm: Float64Array; cover: Float64Array };

const STEPS = 65535;

export const units = {};

// FIELD

let memo: Sampled | undefined;

function sampled(): Sampled {
  if (memo) return memo;
  const { size, seats, tone, mask } = census;
  const cells = size * size;
  const bytes = Uint8Array.from(atob(tone), (c) => c.charCodeAt(0));
  const t = new Float64Array(cells);
  const arm = new Float64Array(cells);
  const cover = new Float64Array(cells);
  if (mask.length % 2 !== 0) throw new Error(`demo-star: ${mask.length} mask numbers, want pairs`);
  let at = 0;
  let drawn = 0;
  for (let i = 0; i < mask.length; i += 2) {
    const hits = Math.floor(mask[i] / (seats + 1));
    const arms = mask[i] % (seats + 1);
    if (hits > seats || arms > hits) throw new Error(`demo-star: code ${mask[i]} holds ${arms} arms in ${hits} hits of ${seats}`);
    if (at + mask[i + 1] > cells) throw new Error(`demo-star: the mask runs past ${cells} cells`);
    for (let k = 0; k < mask[i + 1]; k++, at++) {
      if (hits === 0) continue;
      cover[at] = hits / seats;
      arm[at] = arms / hits;
      t[at] = (bytes[2 * drawn] | (bytes[2 * drawn + 1] << 8)) / STEPS;
      drawn++;
    }
  }
  if (at !== cells) throw new Error(`demo-star: the mask holds ${at} cells, want ${cells}`);
  if (bytes.length !== 2 * drawn) throw new Error(`demo-star: ${bytes.length} tone bytes for ${drawn} drawn cells`);
  memo = { t, arm, cover };
  return memo;
}

// DRAW

export default function draw(pen: Pen, ink: Ink) {
  const { t, arm, cover } = sampled();
  const size = census.size;
  const bright = new ink.Ramp([ink.ground, ink.blue, ink.yellow]);
  const quiet = ink.Ramp.tone(ink.ground, ink.mix(ink.line, ink.dim, 0.5));
  const frame = pen.frame(0.05);
  const x0 = Math.ceil(frame.x);
  const y0 = Math.ceil(frame.y);
  const x1 = Math.min(Math.floor(frame.x + frame.w), pen.width);
  const y1 = Math.min(Math.floor(frame.y + frame.h), pen.height);
  const star = field.patch(x0, y0, x1 - x0, y1 - y0);
  for (let py = y0; py < y1; py++) {
    for (let px = x0; px < x1; px++) {
      const u = (px + 0.5 - frame.x) / frame.w;
      const v = (py + 0.5 - frame.y) / frame.h;
      const slot = Math.min(Math.floor(u * size), size - 1);
      const line = Math.min(Math.floor(v * size), size - 1);
      const i = line * size + slot;
      if (cover[i] <= 0) continue;
      star.blend(px, py, ink.mix(quiet.at(t[i]), bright.at(t[i]), arm[i]), cover[i]);
    }
  }
  star.paint(pen);
}
