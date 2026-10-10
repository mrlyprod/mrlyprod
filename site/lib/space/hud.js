import { letters } from '../../kit/font/font.js';
import { palette } from '../../kit/theme/palette.js';
import { rgb } from '../scene.js';
import { blend, program, quads } from './gl2.js';

const CELL = 3;
const LEAD = 3;
const MARGIN = 16;
const TOP = 0.13;
const BOTTOM = 0.18;
const SIDE = 0.06;
const MOST = 4000;
const SHADOW = 0.6;

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

const INK = rgb(palette.white).map((v) => v / 255);

const PAPER = [0, 0, 0];

export function safe(view) {
  const pad = Math.round(MARGIN * (view.dpr || 1));
  if (!view.fixed) return { top: pad, bottom: pad, side: pad };
  return { top: Math.round(view.h * TOP), bottom: Math.round(view.h * BOTTOM), side: Math.max(pad, Math.round(view.w * SIDE)) };
}

export function hud(gl, view) {
  const cells = program(gl, PAINT, BOX);
  const boxes = quads(gl, { aRect: 4, aColor: 4 }, MOST);
  const raster = new Map();
  let text = {};
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
  const draw = () => {
    const edge = safe(view);
    const out = boxes.data;
    let count = 0;
    const put = (x, y, w, h, color, alpha = 1) => {
      if (count >= MOST) return;
      out.set([x, y, w, h, ...color, alpha], count++ * 8);
    };
    const stamp = (line, x, y, cell) => {
      const { rows, cols, grid } = glyphs(line);
      const fall = Math.max(1, Math.round(cell / 3));
      for (let r = 0; r < rows; r++) {
        for (let c = 0; c < cols; c++) {
          if (!grid[r][c]) continue;
          put(x + c * cell + fall, y + r * cell + fall, cell, cell, PAPER, SHADOW);
          put(x + c * cell, y + r * cell, cell, cell, INK);
        }
      }
    };
    const widest = Math.max(1, ...Object.values(text).flat().map((line) => glyphs(line).cols));
    const big = Math.max(1, Math.min(Math.round(CELL * (view.dpr || 1)), Math.floor((view.w - 2 * edge.side) / widest)));
    const step = big * (5 + LEAD);
    for (const [corner, rows] of Object.entries(text)) {
      const right = corner.endsWith('r');
      const bottom = corner.startsWith('b');
      rows.forEach((line, i) => {
        const wide = glyphs(line).cols * big;
        const x = right ? view.w - edge.side - wide : edge.side;
        const y = bottom ? view.h - edge.bottom - (rows.length - i) * step + big * LEAD : edge.top + i * step;
        stamp(line, Math.round(x), Math.round(y), big);
      });
    }
    gl.bindFramebuffer(gl.FRAMEBUFFER, null);
    gl.viewport(0, 0, view.w, view.h);
    cells.set({ uRes: [view.w, view.h] });
    boxes.draw(cells, count, 'alpha');
    blend(gl, null);
  };
  const drop = () => {
    cells.drop();
    boxes.drop();
  };
  return { lines, draw, drop };
}
