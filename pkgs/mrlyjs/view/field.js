// FIELDS

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

export function draw_range(pen, frame, width, height, values, span, ramp) {
  if (width === 0 || height === 0 || values.length < width * height) return;
  const [lo, hi] = span;
  const reach = Math.abs(hi - lo) < 1e-12 ? 1 : hi - lo;
  const colors = new Uint8Array(width * height * 4);
  for (let i = 0; i < width * height; i++) colors.set(ramp.at((values[i] - lo) / reach), i * 4);
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
