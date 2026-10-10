import { Grid, hex, plot } from '../../../pkgs/mrlyjs/view/index.js';

export const DOT = 0.42;
const SPACING = Math.SQRT2;

export function fit(points, frame, pad = 0) {
  let [lx, ly, hx, hy] = [Infinity, Infinity, -Infinity, -Infinity];
  for (const [x, y] of points) {
    lx = Math.min(lx, x - pad);
    ly = Math.min(ly, y - pad);
    hx = Math.max(hx, x + pad);
    hy = Math.max(hy, y + pad);
  }
  const sx = Math.max(hx - lx, 1e-9);
  const sy = Math.max(hy - ly, 1e-9);
  const scale = Math.min(frame.w / sx, frame.h / sy);
  const ox = frame.x + (frame.w - sx * scale) / 2;
  const oy = frame.y + (frame.h - sy * scale) / 2;
  return { scale, place: ([x, y]) => [ox + (x - lx) * scale, oy + (y - ly) * scale] };
}

export function paint(pen, frame, cut, ink) {
  if (!cut) return;
  if (cut.kind === 'slice') new Grid(frame.square(), cut.side, cut.side, 0).paint(pen, cut.flat, (type) => (type ? ink.fill : ink.faint));
  else if (cut.kind === 'hex') hex.draw(pen, frame, cut.hex, 0, (type) => (type === cut.codes.fill ? ink.fill : type === cut.codes.void ? ink.faint : null));
  else if (cut.kind === 'diagonal' && cut.points.length) {
    const { scale, place } = fit(cut.points, frame, DOT * SPACING);
    plot.dots(pen, cut.points.map(place), DOT * SPACING * scale, ink.fill);
  }
}
