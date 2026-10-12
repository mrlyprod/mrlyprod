import { num } from '../../lib/svg.js';

const circle = ([cx, cy, r]) => `<circle cx="${num(cx)}" cy="${num(cy)}" r="${num(r)}"/>`;

export function circles(list, { fill = 'none', stroke = 'none', width = 1 }) {
  return list.length ? `<g fill="${fill}" stroke="${stroke}" stroke-width="${num(width)}">${list.map(circle).join('')}</g>` : '';
}

export function labels(list, fill, face) {
  const body = list.map(([x, y, size, text]) => `<text x="${num(x)}" y="${num(y)}" font-size="${num(size)}">${text}</text>`);
  return body.length ? `<g fill="${fill}" font-family="${face}" text-anchor="middle" dominant-baseline="central">${body.join('')}</g>` : '';
}

export function rules(list, stroke, width) {
  const body = list.map(([x0, y0, x1, y1]) => `<line x1="${num(x0)}" y1="${num(y0)}" x2="${num(x1)}" y2="${num(y1)}"/>`);
  return body.length ? `<g stroke="${stroke}" stroke-width="${num(width)}">${body.join('')}</g>` : '';
}

export function clip(id, [x, y, w, h], body) {
  return body ? `<clipPath id="${id}"><rect x="${num(x)}" y="${num(y)}" width="${num(w)}" height="${num(h)}"/></clipPath><g clip-path="url(#${id})">${body}</g>` : '';
}
