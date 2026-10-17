import * as math from "@mrlyprod/mrly/math";
import { plate } from "../../lib/plate.js";

export const units = { math };

const CARPET = "23";
const LAYERS = 3;
const SIZE = 384;
const FAINT = 44;
const CURVE = 1.6;
const WANT = { exact: 3 };

let held = null;

function stacked() {
  if (held) return held;
  const spec = math.moire.Spec.new(CARPET, 2, 3);
  const frame = math.moire.frame([1, 1, 1], 0.5);
  const sum = new Float32Array(SIZE * SIZE);
  let inside = null;
  for (let n = 1; n < 2 * LAYERS; n += 2) {
    const volume = math.moire.volume(spec, [n], "Sum", 1, n);
    const [cut, mask] = volume.plane(frame, SIZE);
    volume.free();
    inside ??= mask;
    for (let i = 0; i < sum.length; i++) sum[i] += cut[i];
  }
  const star = new math.six.star.Star(CARPET);
  let exact = 0;
  for (let n = 1; n < 2 * LAYERS; n += 2) {
    const arm = star.arm(n, 0).reduced().join("/");
    const law = math.six.star.arm_law(n).reduced().join("/");
    exact += arm === law ? 1 : 0;
  }
  star.free();
  if (exact !== WANT.exact) throw new Error(`star: ${exact} of ${LAYERS} arms on the law, want ${WANT.exact}`);
  let low = Infinity;
  let high = -Infinity;
  for (let i = 0; i < sum.length; i++) {
    if (!inside[i]) continue;
    low = Math.min(low, sum[i]);
    high = Math.max(high, sum[i]);
  }
  const colors = new Uint8ClampedArray(SIZE * SIZE * 4);
  const span = high - low || 1;
  for (let i = 0; i < sum.length; i++) {
    if (!inside[i]) continue;
    const t = 1 - (sum[i] - low) / span;
    colors.set([255, 255, 255, Math.round(FAINT + (255 - FAINT) * t ** CURVE)], i * 4);
  }
  held = { shape: [SIZE, SIZE], colors };
  return held;
}

export default function draw(pen) {
  const { box } = plate(pen, "indigo", 0.18);
  pen.image(box.x, box.y, box.w, box.h, stacked());
}
