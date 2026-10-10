import { num } from '../../lib/svg.js';

const circle = ([cx, cy, r]) => `<circle cx="${num(cx)}" cy="${num(cy)}" r="${num(r)}"/>`;

export function discs(list, fill) {
  return list.length ? `<g fill="${fill}">${list.map(circle).join('')}</g>` : '';
}

export function rings(list, stroke, width) {
  return list.length ? `<g fill="none" stroke="${stroke}" stroke-width="${num(width)}">${list.map(circle).join('')}</g>` : '';
}

export function labels(list, fill, face) {
  const body = list.map(([x, y, size, text]) => `<text x="${num(x)}" y="${num(y)}" font-size="${num(size)}">${text}</text>`);
  return body.length ? `<g fill="${fill}" font-family="${face}" text-anchor="middle" dominant-baseline="central">${body.join('')}</g>` : '';
}

export function box(x, y, w, h, fill) {
  return `<rect x="${num(x)}" y="${num(y)}" width="${num(w)}" height="${num(h)}" fill="${fill}"/>`;
}

export function rules(list, stroke, width) {
  const body = list.map(([x0, y0, x1, y1, alpha]) => `<line x1="${num(x0)}" y1="${num(y0)}" x2="${num(x1)}" y2="${num(y1)}"${alpha === undefined ? '' : ` stroke-opacity="${num(alpha)}"`}/>`);
  return body.length ? `<g stroke="${stroke}" stroke-width="${num(width)}">${body.join('')}</g>` : '';
}
