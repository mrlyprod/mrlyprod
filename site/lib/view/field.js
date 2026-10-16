import { Ramp } from "./ink.js";

export { patch } from "./pixels.js";

// FIELDS

const clamp = (v) => (v < 0 ? 0 : v > 1 ? 1 : v);
const byte = (v) => {
  const r = Math.round(v);
  return r > 255 ? 255 : r > 0 ? r : 0;
};

function range(values) {
  let lo = Number.MAX_VALUE;
  let hi = -Number.MAX_VALUE;
  for (let i = 0; i < values.length; i++) {
    if (values[i] < lo) lo = values[i];
    if (values[i] > hi) hi = values[i];
  }
  return hi - lo < 1e-12 ? [lo, lo + 1] : [lo, hi];
}

export function draw(pen, frame, width, height, values, ramp) {
  draw_range(pen, frame, width, height, values, range(values), ramp);
}

function tint(colors, values, lo, reach, ramp) {
  const n = colors.length >> 2;
  if (ramp.at !== Ramp.prototype.at) {
    for (let i = 0; i < n; i++) colors.set(ramp.at((values[i] - lo) / reach), i * 4);
    return colors;
  }
  const stops = ramp.stops.map((c) => [c[0], c[1], c[2], c[3]]);
  const last = stops.length - 1;
  if (last < 1) {
    const flat = last ? [...ramp.ground] : stops[0];
    for (let i = 0; i < n; i++) colors.set(flat, i * 4);
    return colors;
  }
  for (let i = 0, o = 0; i < n; i++, o += 4) {
    const s = clamp((values[i] - lo) / reach) * last;
    const k = Math.min(Math.floor(s) || 0, last - 1);
    const f = clamp(s - k);
    const a = stops[k];
    const b = stops[k + 1];
    colors[o] = byte(a[0] + (b[0] - a[0]) * f);
    colors[o + 1] = byte(a[1] + (b[1] - a[1]) * f);
    colors[o + 2] = byte(a[2] + (b[2] - a[2]) * f);
    colors[o + 3] = byte(a[3] + (b[3] - a[3]) * f);
  }
  return colors;
}

export function draw_range(pen, frame, width, height, values, span, ramp) {
  if (width === 0 || height === 0 || values.length < width * height) return;
  const [lo, hi] = span;
  const reach = Math.abs(hi - lo) < 1e-12 ? 1 : hi - lo;
  const colors = tint(new Uint8Array(width * height * 4), values, lo, reach, ramp);
  pen.image(frame.x, frame.y, frame.w, frame.h, { shape: [height, width], colors });
}

export function sample(pen, frame, resolution, f, ramp) {
  if (resolution === 0) return;
  const values = new Float64Array(resolution * resolution);
  for (let row = 0; row < resolution; row++) {
    for (let col = 0; col < resolution; col++) values[row * resolution + col] = f((col + 0.5) / resolution, (row + 0.5) / resolution);
  }
  draw(pen, frame, resolution, resolution, values, ramp);
}
