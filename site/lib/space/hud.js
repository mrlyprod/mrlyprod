import { letters } from '../../kit/font/font.js';
import { palette } from '../../kit/theme/palette.js';
import { rgb } from '../scene.js';
import { blend, program, quads } from './gl2.js';

const HIT = 44;
const RING = 16;
const STROKE = 2;
const CELL = 3;
const SMALL = 3;
const LEAD = 3;
const MARGIN = 16;
const TOP = 0.13;
const BOTTOM = 0.18;
const SIDE = 0.06;
const MOST = 4000;
const BUSY = 8;
const SHADOW = 0.6;
const FAINT = 0.45;

const BOX = `#version 300 es
in vec4 aRect;
in vec4 aColor;
uniform vec2 uRes;
out vec4 vColor;
void main() {
  vec2 c = vec2(float(gl_VertexID & 1), float(gl_VertexID >> 1));
  vec2 p = aRect.xy + c * aRect.zw;
  vColor = aColor;
  gl_Position = vec4(p.x / uRes.x * 2.0 - 1.0, 1.0 - p.y / uRes.y * 2.0, 0.0, 1.0);
}
`;

const PAINT = `#version 300 es
precision mediump float;
in vec4 vColor;
out vec4 o;
void main() {
  o = vec4(vColor.rgb * vColor.a, vColor.a);
}
`;

const LOOP = `#version 300 es
in vec4 aRing;
in vec4 aColor;
uniform vec2 uRes;
out vec2 vAt;
out vec3 vRing;
out vec4 vColor;
void main() {
  vec2 c = vec2(float(gl_VertexID & 1), float(gl_VertexID >> 1)) * 2.0 - 1.0;
  float reach = aRing.z + aRing.w + 2.0;
  vec2 p = aRing.xy + c * reach;
  vAt = c * reach;
  vRing = aRing.zwz;
  vColor = aColor;
  gl_Position = vec4(p.x / uRes.x * 2.0 - 1.0, 1.0 - p.y / uRes.y * 2.0, 0.0, 1.0);
}
`;

const RINGS = `#version 300 es
precision mediump float;
in vec2 vAt;
in vec3 vRing;
in vec4 vColor;
out vec4 o;
void main() {
  float d = length(vAt);
  float ring = clamp(vRing.y * 0.5 + 0.5 - abs(d - vRing.x), 0.0, 1.0);
  float a = ring * vColor.a;
  o = vec4(vColor.rgb * a, a);
}
`;

const DISCS = `#version 300 es
precision mediump float;
in vec2 vAt;
in vec3 vRing;
in vec4 vColor;
out vec4 o;
void main() {
  float d = length(vAt);
  float a = clamp(vRing.x + 0.5 - d, 0.0, 1.0) * vColor.a;
  o = vec4(vColor.rgb * a, a);
}
`;

const INK = rgb(palette.white).map((v) => v / 255);

const PAPER = [0, 0, 0];

export function safe(view) {
  const pad = Math.round(MARGIN * (view.dpr || 1));
  if (!view.fixed) return { top: pad, bottom: pad, side: pad };
  return { top: Math.round(view.h * TOP), bottom: Math.round(view.h * BOTTOM), side: Math.max(pad, Math.round(view.w * SIDE)) };
}

export function fit(rect, w, h, cx, cy) {
  const k = Math.min(rect.width / w, rect.height / h) || 1;
  const left = rect.left + (rect.width - w * k) / 2;
  const top = rect.top + (rect.height - h * k) / 2;
  return [(cx - left) / k, (cy - top) / k, 1 / k];
}

export function hud(gl, view) {
  const cells = program(gl, PAINT, BOX);
  const rings = program(gl, RINGS, LOOP);
  const discs = program(gl, DISCS, LOOP);
  const boxes = quads(gl, { aRect: 4, aColor: 4 }, MOST);
  const loops = quads(gl, { aRing: 4, aColor: 4 }, BUSY);
  const raster = new Map();
  let text = {};
  let list = [];
  const glyphs = (line) => {
    if (!raster.has(line)) {
      if (raster.size > 64) raster.clear();
      raster.set(line, letters(line));
    }
    return raster.get(line);
  };
  const lines = (rows, corner = 'tl') => {
    text = { ...text, [corner]: (rows ?? []).filter((row) => row !== null && row !== undefined && row !== '').map((row) => String(row).toUpperCase()) };
  };
  const marks = (next) => {
    list = (next ?? []).filter((one) => Number.isFinite(one.x) && Number.isFinite(one.y));
  };
  const pick = (x, y, k = view.dpr || 1) => {
    let best = null;
    let near = HIT * k;
    for (const one of list) {
      const d = Math.hypot(one.x - x, one.y - y);
      if (d <= near) {
        near = d;
        best = one.id;
      }
    }
    return best;
  };
  const draw = () => {
    const unit = view.dpr || 1;
    const accent = rgb(view.look().accent).map((v) => v / 255);
    const edge = safe(view);
    const out = boxes.data;
    let count = 0;
    const put = (x, y, w, h, color, alpha = 1) => {
      if (count >= MOST) return;
      out.set([x, y, w, h, ...color, alpha], count++ * 8);
    };
    const stamp = (line, x, y, cell, color, shade, alpha = 1) => {
      const { rows, cols, grid } = glyphs(line);
      const fall = Math.max(1, Math.round(cell / 3));
      for (let r = 0; r < rows; r++) {
        for (let c = 0; c < cols; c++) {
          if (!grid[r][c]) continue;
          if (shade) put(x + c * cell + fall, y + r * cell + fall, cell, cell, PAPER, SHADOW);
          put(x + c * cell, y + r * cell, cell, cell, color, alpha);
        }
      }
      return [cols * cell, rows * cell];
    };
    const big = Math.max(1, Math.round(CELL * unit));
    const step = big * (5 + LEAD);
    for (const [corner, rows] of Object.entries(text)) {
      const right = corner.endsWith('r');
      const bottom = corner.startsWith('b');
      rows.forEach((line, i) => {
        const wide = glyphs(line).cols * big;
        const x = right ? view.w - edge.side - wide : edge.side;
        const y = bottom ? view.h - edge.bottom - (rows.length - i) * step + big * LEAD : edge.top + i * step;
        stamp(line, Math.round(x), Math.round(y), big, INK, true);
      });
    }
    const tags = list.slice(0, BUSY);
    const ring = tags.filter((one) => (one.r ?? RING) > 0);
    const small = Math.max(1, Math.round(SMALL * unit));
    const data = loops.data;
    ring.forEach((one, i) => data.set([one.x, one.y, (one.r ?? RING) * unit, STROKE * unit, ...accent, one.dim ? FAINT : 1], i * 8));
    for (const one of tags) {
      const label = String(one.label ?? '');
      const { rows, cols } = glyphs(label);
      stamp(label, Math.round(one.x - (cols * small) / 2), Math.round(one.y - (rows * small) / 2), small, one.on ? PAPER : INK, false, one.dim ? FAINT : 1);
    }
    gl.bindFramebuffer(gl.FRAMEBUFFER, null);
    gl.viewport(0, 0, view.w, view.h);
    rings.set({ uRes: [view.w, view.h] });
    loops.draw(rings, ring.length, 'alpha');
    const filled = ring.filter((one) => one.on);
    filled.forEach((one, i) => data.set([one.x, one.y, (one.r ?? RING) * unit - STROKE * unit * 0.5, STROKE * unit, ...accent, one.dim ? FAINT : 1], i * 8));
    discs.set({ uRes: [view.w, view.h] });
    loops.draw(discs, filled.length, 'alpha');
    cells.set({ uRes: [view.w, view.h] });
    boxes.draw(cells, count, 'alpha');
    blend(gl, null);
  };
  const drop = () => {
    for (const one of [cells, rings, discs]) one.drop();
    boxes.drop();
    loops.drop();
  };
  return { lines, marks, draw, pick, drop };
}
