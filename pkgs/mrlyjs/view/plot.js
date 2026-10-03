// RANGES

function span(values, n) {
  let lo = Number.MAX_VALUE;
  let hi = -Number.MAX_VALUE;
  for (let i = 0; i < n; i++) {
    if (values[i] < lo) lo = values[i];
    if (values[i] > hi) hi = values[i];
  }
  return hi - lo < 1e-12 ? [lo - 0.5, hi + 0.5] : [lo, hi];
}

// MARKS

export function bars(pen, frame, values, gap, color) {
  if (values.length === 0) return;
  let peak = 0;
  for (let i = 0; i < values.length; i++) if (Math.abs(values[i]) > peak) peak = Math.abs(values[i]);
  if (peak <= 0) return;
  const slot = frame.w / values.length;
  const pad = (gap * slot) / 2;
  for (let i = 0; i < values.length; i++) {
    const h = frame.h * (Math.abs(values[i]) / peak);
    pen.rect(frame.x + i * slot + pad, frame.y + frame.h - h, slot - 2 * pad, h, color);
  }
}

export function dots(pen, pts, r, color) {
  for (const [x, y] of pts) pen.disc(x, y, r, color);
}

export function rings(pen, center, radii, thick, color) {
  for (const r of radii) pen.ring(center[0], center[1], r, thick, color);
}

export function staircase(pen, frame, values, thick, color) {
  if (values.length === 0) return;
  const [lo, hi] = span(values, values.length);
  const slot = frame.w / values.length;
  const pts = [];
  for (let i = 0; i < values.length; i++) {
    const y = frame.y + frame.h * (1 - (values[i] - lo) / (hi - lo));
    pts.push([frame.x + i * slot, y], [frame.x + (i + 1) * slot, y]);
  }
  pen.polyline(pts, thick, color);
}

export function curve(pen, frame, xs, ys, thick, color) {
  const n = Math.min(xs.length, ys.length);
  if (n < 2) return;
  const [xlo, xhi] = span(xs, n);
  const [ylo, yhi] = span(ys, n);
  const pts = [];
  for (let i = 0; i < n; i++) {
    pts.push([frame.x + (frame.w * (xs[i] - xlo)) / (xhi - xlo), frame.y + frame.h * (1 - (ys[i] - ylo) / (yhi - ylo))]);
  }
  pen.polyline(pts, thick, color);
}

export function axis(pen, frame, color) {
  const thick = Math.max(Math.min(frame.w, frame.h) / 512, 1);
  const { x, y, w, h } = frame;
  pen.polyline([[x, y], [x + w, y], [x + w, y + h], [x, y + h], [x, y]], thick, color);
}

export function baseline(pen, frame, color) {
  const thick = Math.max(Math.min(frame.w, frame.h) / 512, 1);
  pen.segment([frame.x, frame.y + frame.h], [frame.x + frame.w, frame.y + frame.h], thick, color);
}
