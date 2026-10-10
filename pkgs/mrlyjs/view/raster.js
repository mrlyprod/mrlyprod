import { CLEAR, board } from "./frame.js";

const TAU = 2 * Math.PI;
const MAX = Number.MAX_VALUE;

// GEOMETRY

const fmin = (a, b) => (b < a || a !== a ? b : a);
const fmax = (a, b) => (b > a || a !== a ? b : a);
const clamp = (v, lo, hi) => (v < lo ? lo : v > hi ? hi : v);

function boxSdf(px, py, cx, cy, hw, hh) {
  const qx = Math.abs(px - cx) - hw;
  const qy = Math.abs(py - cy) - hh;
  const ox = fmax(qx, 0);
  const oy = fmax(qy, 0);
  return Math.sqrt(ox * ox + oy * oy) + fmin(fmax(qx, qy), 0);
}

function segmentSdf(px, py, a, b) {
  const vx = b[0] - a[0];
  const vy = b[1] - a[1];
  const wx = px - a[0];
  const wy = py - a[1];
  const len = vx * vx + vy * vy;
  const t = len <= Number.EPSILON ? 0 : clamp((wx * vx + wy * vy) / len, 0, 1);
  const dx = wx - t * vx;
  const dy = wy - t * vy;
  return Math.sqrt(dx * dx + dy * dy);
}

function polygonSdf(px, py, pts) {
  let dist = MAX;
  let inside = false;
  for (let i = 0; i < pts.length; i++) {
    const a = pts[i];
    const b = pts[(i + 1) % pts.length];
    dist = fmin(dist, segmentSdf(px, py, a, b));
    if (a[1] > py !== b[1] > py && px < a[0] + ((py - a[1]) / (b[1] - a[1])) * (b[0] - a[0])) inside = !inside;
  }
  return inside ? -dist : dist;
}

const FAR = 0.5 + 1e-6;
const own = (c) => [c[0], c[1], c[2], c[3]];

function lines(pts) {
  const out = new Float64Array(pts.length * 3);
  for (let i = 0; i < pts.length; i++) {
    const a = pts[i];
    const b = pts[(i + 1) % pts.length];
    const dx = b[0] - a[0];
    const dy = b[1] - a[1];
    const len = Math.sqrt(dx * dx + dy * dy);
    if (!(len > 0 && len < Infinity && Number.isFinite(a[0] + a[1]))) return null;
    out[i * 3] = -dy / len;
    out[i * 3 + 1] = dx / len;
    out[i * 3 + 2] = (dy * a[0] - dx * a[1]) / len;
  }
  return out;
}

function bounds(pts) {
  let x0 = MAX;
  let y0 = MAX;
  let x1 = -MAX;
  let y1 = -MAX;
  for (const p of pts) {
    x0 = fmin(x0, p[0]);
    y0 = fmin(y0, p[1]);
    x1 = fmax(x1, p[0]);
    y1 = fmax(y1, p[1]);
  }
  return [x0, y0, x1, y1];
}

// RASTER

export function raster(width, height, ground = CLEAR) {
  const colors = new Uint8ClampedArray(width * height * 4);
  new Uint32Array(colors.buffer).fill(new Uint32Array(Uint8ClampedArray.from(ground.slice(0, 4), Math.round).buffer)[0]);
  const swatch = [0, 0, 0, 0];

  function blend(x, y, c, cover) {
    if (x >= width || y >= height) return;
    const a = (c[3] / 255) * clamp(cover, 0, 1);
    if (a <= 0) return;
    const i = (y * width + x) * 4;
    const keep = 1 - a;
    colors[i] = Math.round(c[0] * a + colors[i] * keep);
    colors[i + 1] = Math.round(c[1] * a + colors[i + 1] * keep);
    colors[i + 2] = Math.round(c[2] * a + colors[i + 2] * keep);
    colors[i + 3] = Math.round((a + (colors[i + 3] / 255) * keep) * 255);
  }

  function shade(ax0, ay0, ax1, ay1, color, sdf) {
    const c = own(color);
    const x0 = fmax(Math.floor(ax0 - 1), 0);
    const y0 = fmax(Math.floor(ay0 - 1), 0);
    const x1 = Math.min(fmax(Math.ceil(ax1 + 1), 0), width);
    const y1 = Math.min(fmax(Math.ceil(ay1 + 1), 0), height);
    for (let py = y0; py < y1; py++) {
      for (let px = x0; px < x1; px++) blend(px, py, c, 0.5 - sdf(px + 0.5, py + 0.5));
    }
  }

  function polygon(pts, color) {
    if (pts.length < 3) return;
    const [bx0, by0, bx1, by1] = bounds(pts);
    const near = lines(pts);
    if (!near) return shade(bx0, by0, bx1, by1, color, (px, py) => polygonSdf(px, py, pts));
    const c = own(color);
    const n = pts.length;
    const x0 = fmax(Math.floor(bx0 - 1), 0);
    const y0 = fmax(Math.floor(by0 - 1), 0);
    const x1 = Math.min(fmax(Math.ceil(bx1 + 1), 0), width);
    const y1 = Math.min(fmax(Math.ceil(by1 + 1), 0), height);
    const lo = new Float64Array(n);
    const hi = new Float64Array(n);
    const cross = new Float64Array(n);
    for (let py = y0; py < y1; py++) {
      const cy = py + 0.5;
      let k = 0;
      for (let i = 0; i < n; i++) {
        const a = pts[i];
        const b = pts[(i + 1) % n];
        if (a[1] > cy !== b[1] > cy) cross[k++] = a[0] + ((cy - a[1]) / (b[1] - a[1])) * (b[0] - a[0]);
        const nx = near[i * 3];
        const off = near[i * 3 + 1] * cy + near[i * 3 + 2];
        const p = (-FAR - off) / nx;
        const q = (FAR - off) / nx;
        lo[i] = nx === 0 ? (Math.abs(off) <= FAR ? -Infinity : Infinity) : Math.min(p, q) - 1e-6;
        hi[i] = nx === 0 ? (Math.abs(off) <= FAR ? Infinity : -Infinity) : Math.max(p, q) + 1e-6;
      }
      for (let px = x0; px < x1; px++) {
        const cx = px + 0.5;
        let close = false;
        for (let i = 0; i < n && !close; i++) close = cx >= lo[i] && cx <= hi[i];
        if (close) {
          blend(px, py, c, 0.5 - polygonSdf(cx, cy, pts));
          continue;
        }
        let inside = false;
        for (let j = 0; j < k; j++) if (cx < cross[j]) inside = !inside;
        if (inside) blend(px, py, c, 1);
      }
    }
  }

  return {
    ...board(width, height),
    pixels: () => ({ shape: [height, width], colors }),
    rect(x, y, w, h, c) {
      const cx = x + w / 2;
      const cy = y + h / 2;
      const hw = w / 2;
      const hh = h / 2;
      shade(x, y, x + w, y + h, c, (px, py) => boxSdf(px, py, cx, cy, hw, hh));
    },
    round_rect(x, y, w, h, r, c) {
      const cx = x + w / 2;
      const cy = y + h / 2;
      const rr = fmax(fmin(fmin(r, w / 2), h / 2), 0);
      const hw = w / 2 - rr;
      const hh = h / 2 - rr;
      shade(x, y, x + w, y + h, c, (px, py) => boxSdf(px, py, cx, cy, hw, hh) - rr);
    },
    disc(cx, cy, r, c) {
      shade(cx - r, cy - r, cx + r, cy + r, c, (px, py) => {
        const dx = px - cx;
        const dy = py - cy;
        return Math.sqrt(dx * dx + dy * dy) - r;
      });
    },
    ring(cx, cy, r, thick, c) {
      const outer = r + thick / 2;
      shade(cx - outer, cy - outer, cx + outer, cy + outer, c, (px, py) => {
        const dx = px - cx;
        const dy = py - cy;
        return Math.abs(Math.sqrt(dx * dx + dy * dy) - r) - thick / 2;
      });
    },
    segment(a, b, thick, c) {
      const half = thick / 2;
      const [x0, y0, x1, y1] = bounds([a, b]);
      shade(x0 - half, y0 - half, x1 + half, y1 + half, c, (px, py) => segmentSdf(px, py, a, b) - half);
    },
    polyline(pts, thick, color) {
      if (pts.length < 2) return;
      const c = own(color);
      const half = thick / 2;
      const pad = half + 1;
      const [bx0, by0, bx1, by1] = bounds(pts);
      const x0 = fmax(Math.floor(bx0 - pad), 0);
      const y0 = fmax(Math.floor(by0 - pad), 0);
      const x1 = Math.min(fmax(Math.ceil(bx1 + pad), 0), width);
      const y1 = Math.min(fmax(Math.ceil(by1 + pad), 0), height);
      if (x1 <= x0 || y1 <= y0) return;
      const span = x1 - x0;
      const mask = new Float64Array(span * (y1 - y0));
      for (let k = 0; k + 1 < pts.length; k++) {
        const a = pts[k];
        const b = pts[k + 1];
        const [ax0, ay0, ax1, ay1] = bounds([a, b]);
        const sx0 = fmax(Math.floor(ax0 - pad), x0);
        const sy0 = fmax(Math.floor(ay0 - pad), y0);
        const sx1 = Math.min(fmax(Math.ceil(ax1 + pad), 0), x1);
        const sy1 = Math.min(fmax(Math.ceil(ay1 + pad), 0), y1);
        for (let py = sy0; py < sy1; py++) {
          const row = (py - y0) * span;
          for (let px = sx0; px < sx1; px++) {
            const cover = clamp(0.5 - (segmentSdf(px + 0.5, py + 0.5, a, b) - half), 0, 1);
            if (cover > mask[row + px - x0]) mask[row + px - x0] = cover;
          }
        }
      }
      for (let py = y0; py < y1; py++) {
        const row = (py - y0) * span;
        for (let px = x0; px < x1; px++) {
          const cover = mask[row + px - x0];
          if (cover > 0) blend(px, py, c, cover);
        }
      }
    },
    triangle(a, b, c, color) {
      polygon([a, b, c], color);
    },
    polygon,
    arc(center, r, angles, thick, c) {
      const [cx, cy] = center;
      const [from, to] = angles;
      const half = thick / 2;
      const outer = r + half;
      const span = Math.abs(to - from);
      const lo = to >= from ? from : to;
      const hi = to >= from ? to : from;
      const ends = [
        [cx + r * Math.cos(lo), cy + r * Math.sin(lo)],
        [cx + r * Math.cos(hi), cy + r * Math.sin(hi)],
      ];
      const reach = fmin(span, TAU);
      shade(cx - outer, cy - outer, cx + outer, cy + outer, c, (px, py) => {
        let turn = Math.atan2(py - cy, px - cx) - lo;
        while (turn < 0) turn += TAU;
        if (turn <= reach) {
          const dx = px - cx;
          const dy = py - cy;
          return Math.abs(Math.sqrt(dx * dx + dy * dy) - r) - half;
        }
        let d = MAX;
        for (const e of ends) {
          const dx = px - e[0];
          const dy = py - e[1];
          d = fmin(d, Math.sqrt(dx * dx + dy * dy));
        }
        return d - half;
      });
    },
    image(x, y, w, h, pixels) {
      const [sh, sw] = pixels.shape;
      const src = pixels.colors;
      const cover = pixels.cover;
      if (!(sw > 0 && sh > 0) || src.length < sw * sh * 4 || (cover && cover.length < sw * sh)) return;
      const x0 = fmax(Math.ceil(x), 0);
      const y0 = fmax(Math.ceil(y), 0);
      const x1 = Math.min(fmax(Math.floor(x + w), 0), width);
      const y1 = Math.min(fmax(Math.floor(y + h), 0), height);
      for (let py = y0; py < y1; py++) {
        const row = Math.min(Math.floor(((py + 0.5 - y) / h) * sh), sh - 1);
        for (let px = x0; px < x1; px++) {
          const col = Math.min(Math.floor(((px + 0.5 - x) / w) * sw), sw - 1);
          const j = (row * sw + col) * 4;
          swatch[0] = src[j];
          swatch[1] = src[j + 1];
          swatch[2] = src[j + 2];
          swatch[3] = src[j + 3];
          blend(px, py, swatch, cover ? cover[row * sw + col] : 1);
        }
      }
    },
  };
}
