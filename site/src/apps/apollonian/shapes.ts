import { num } from "../../lib/app/svg.ts";

/* TYPES */

export type Disc = [number, number, number];

export type Label = [number, number, number, string];

export type Rule = [number, number, number, number];

export type Rect = [number, number, number, number];

export type Brush = { fill?: string; stroke?: string; width?: number };

const circle = ([cx, cy, r]: Disc) => `<circle cx="${num(cx)}" cy="${num(cy)}" r="${num(r)}"/>`;

export function circles(list: readonly Disc[], { fill = "none", stroke = "none", width = 1 }: Brush): string {
  return list.length
    ? `<g fill="${fill}" stroke="${stroke}" stroke-width="${num(width)}">${list.map(circle).join("")}</g>`
    : "";
}

export function labels(list: readonly Label[], fill: string, face: string): string {
  const body = list.map(
    ([x, y, size, text]) => `<text x="${num(x)}" y="${num(y)}" font-size="${num(size)}">${text}</text>`,
  );
  return body.length
    ? `<g fill="${fill}" font-family="${face}" text-anchor="middle" dominant-baseline="central">${body.join("")}</g>`
    : "";
}

export function rules(list: readonly Rule[], stroke: string, width: number): string {
  const body = list.map(
    ([x0, y0, x1, y1]) => `<line x1="${num(x0)}" y1="${num(y0)}" x2="${num(x1)}" y2="${num(y1)}"/>`,
  );
  return body.length ? `<g stroke="${stroke}" stroke-width="${num(width)}">${body.join("")}</g>` : "";
}

export function clip(id: string, [x, y, w, h]: Rect, body: string): string {
  return body
    ? `<clipPath id="${id}"><rect x="${num(x)}" y="${num(y)}" width="${num(w)}" height="${num(h)}"/></clipPath><g clip-path="url(#${id})">${body}</g>`
    : "";
}
