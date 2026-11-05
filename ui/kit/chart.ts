import type { Paints } from "./color.ts";
import { veil } from "./color.ts";

export type Point = { at: number; value: number };

export type Bar = { label: string; value: number };

export type Note = { kind: "note"; text: string; label?: undefined; points?: undefined; bars?: undefined };

export type Line = { kind: "line"; label: string; points: Point[]; text?: undefined; bars?: undefined };

export type Bars = { kind: "bars"; label: string; bars: Bar[]; text?: undefined; points?: undefined };

export type Model = Note | Line | Bars;

export type Look = Paints & { dim: string; line: string; font: string };

const HOUR = 3600;
const TICK = 6;
const STEPS = [1, 2, 5, 10];

export const count = (n: number | null | undefined) => Math.round(n ?? 0).toLocaleString("en-US");

const clock = (at: number) => {
  const d = new Date(at * 1000);
  return `${String(d.getHours()).padStart(2, "0")}:${String(d.getMinutes()).padStart(2, "0")}`;
};

export function scale(max: number) {
  const raw = Math.max(1, max) / 3;
  const ten = 10 ** Math.floor(Math.log10(raw));
  const step = Math.max(1, ten * STEPS.find((one) => one * ten >= raw)!);
  const n = Math.min(3, Math.max(2, Math.ceil(max / step)));
  return { step, top: n * step, n };
}

export function ticks(from: number, to: number) {
  const out: number[] = [];
  const d = new Date(from * 1000);
  d.setMinutes(0, 0, 0);
  for (let t = d.getTime() / 1000; t <= to; t += HOUR) {
    if (t >= from && new Date(t * 1000).getHours() % TICK === 0) out.push(t);
  }
  return out;
}

const type = (
  ctx: CanvasRenderingContext2D,
  look: Look,
  dpr: number,
  px: number,
  color: string,
  align: CanvasTextAlign = "left",
  base: CanvasTextBaseline = "alphabetic",
  weight = "",
) => {
  ctx.font = `${weight}${px * dpr}px ${look.font}`;
  ctx.fillStyle = color;
  ctx.textAlign = align;
  ctx.textBaseline = base;
};

function note(ctx: CanvasRenderingContext2D, w: number, h: number, dpr: number, text: string, look: Look) {
  type(ctx, look, dpr, 16, look.dim, "center", "middle");
  ctx.fillText(text, w / 2, h / 2);
}

function chart(
  ctx: CanvasRenderingContext2D,
  w: number,
  h: number,
  dpr: number,
  model: Line,
  look: Look,
  hover: number | null,
) {
  const { points } = model;
  const pad = 24 * dpr;
  const { step, top, n } = scale(Math.max(...points.map((one) => one.value)));
  type(ctx, look, dpr, 12, look.dim, "right", "middle");
  const gutter = Math.max(...Array.from({ length: n + 1 }, (_, i) => ctx.measureText(count(i * step)).width));
  const l = pad + gutter + 8 * dpr;
  const r = w - pad;
  const t = pad + 32 * dpr;
  const b = h - pad - 20 * dpr;
  const from = points[0].at;
  const to = points.at(-1)!.at;
  const x = (at: number) => (to === from ? (l + r) / 2 : l + ((at - from) / (to - from)) * (r - l));
  const y = (v: number) => b - (v / top) * (b - t);
  for (let i = 0; i <= n; i++) {
    ctx.fillStyle = look.line;
    ctx.fillRect(l, Math.round(y(i * step)), r - l, dpr);
    type(ctx, look, dpr, 12, look.dim, "right", "middle");
    ctx.fillText(count(i * step), l - 8 * dpr, y(i * step));
  }
  type(ctx, look, dpr, 12, look.dim, "center", "top");
  for (const at of ticks(from, to)) ctx.fillText(clock(at), x(at), b + 8 * dpr);
  type(ctx, look, dpr, 12, look.dim, "left", "alphabetic");
  ctx.fillText(model.label, l, pad + 12 * dpr);
  ctx.beginPath();
  ctx.moveTo(x(from), b);
  for (const one of points) ctx.lineTo(x(one.at), y(one.value));
  ctx.lineTo(x(to), b);
  ctx.closePath();
  ctx.fillStyle = veil(look.accent, 0.16);
  ctx.fill();
  ctx.beginPath();
  points.forEach((one, i) => {
    if (i) ctx.lineTo(x(one.at), y(one.value));
    else ctx.moveTo(x(one.at), y(one.value));
  });
  ctx.strokeStyle = look.accent;
  ctx.lineWidth = 2 * dpr;
  ctx.lineJoin = "round";
  ctx.lineCap = "round";
  ctx.stroke();
  const peak = points.reduce((best, one) => (one.value > best.value ? one : best));
  type(ctx, look, dpr, 12, look.ink, "center", "bottom", "600 ");
  const half = ctx.measureText(count(peak.value)).width / 2;
  ctx.fillText(count(peak.value), Math.min(r - half, Math.max(l + half, x(peak.at))), y(peak.value) - 8 * dpr);
  if (hover == null || hover < l - pad || hover > r + pad) return;
  const near = points.reduce((best, one) => (Math.abs(x(one.at) - hover) < Math.abs(x(best.at) - hover) ? one : best));
  const px = x(near.at);
  const py = y(near.value);
  ctx.fillStyle = look.dim;
  ctx.fillRect(Math.round(px), t, dpr, b - t);
  ctx.beginPath();
  ctx.arc(px, py, 4 * dpr, 0, 2 * Math.PI);
  ctx.fillStyle = look.accent;
  ctx.fill();
  ctx.strokeStyle = look.paper;
  ctx.lineWidth = 2 * dpr;
  ctx.stroke();
  type(ctx, look, dpr, 12, look.ink, "left", "top", "600 ");
  const lines = [clock(near.at), count(near.value)];
  const tw = Math.max(...lines.map((one) => ctx.measureText(one).width)) + 16 * dpr;
  const th = 44 * dpr;
  const tx = px + 12 * dpr + tw > w ? px - 12 * dpr - tw : px + 12 * dpr;
  const ty = Math.max(0, Math.min(h - th, py - th / 2));
  ctx.beginPath();
  ctx.roundRect(tx, ty, tw, th, 6 * dpr);
  ctx.fillStyle = look.paper;
  ctx.fill();
  ctx.strokeStyle = look.line;
  ctx.lineWidth = dpr;
  ctx.stroke();
  type(ctx, look, dpr, 12, look.dim, "left", "top");
  ctx.fillText(lines[0], tx + 8 * dpr, ty + 6 * dpr);
  type(ctx, look, dpr, 12, look.ink, "left", "top", "600 ");
  ctx.fillText(lines[1], tx + 8 * dpr, ty + 24 * dpr);
}

function bars(ctx: CanvasRenderingContext2D, w: number, h: number, dpr: number, model: Bars, look: Look) {
  const pad = 24 * dpr;
  const gap = 2 * dpr;
  type(ctx, look, dpr, 12, look.dim, "right", "middle");
  const gutter = Math.max(...model.bars.map((one) => ctx.measureText(one.label).width));
  const tail = Math.max(...model.bars.map((one) => ctx.measureText(count(one.value)).width));
  const l = pad + gutter + 12 * dpr;
  const track = w - pad - tail - 8 * dpr - l;
  const t = pad + 32 * dpr;
  const room = h - pad - t;
  const n = model.bars.length;
  const thick = Math.max(dpr, Math.min(28 * dpr, (room - gap * (n - 1)) / n));
  const top = t + (room - (thick * n + gap * (n - 1))) / 2;
  const max = Math.max(1, ...model.bars.map((one) => one.value));
  type(ctx, look, dpr, 12, look.dim, "left", "alphabetic");
  ctx.fillText(model.label, pad, pad + 12 * dpr);
  model.bars.forEach((one, i) => {
    const y = top + i * (thick + gap);
    const long = Math.max(dpr, (one.value / max) * track);
    ctx.beginPath();
    ctx.roundRect(l, y, long, thick, [0, 4 * dpr, 4 * dpr, 0]);
    ctx.fillStyle = look.accent;
    ctx.fill();
    type(ctx, look, dpr, 12, look.dim, "right", "middle");
    ctx.fillText(one.label, l - 12 * dpr, y + thick / 2);
    type(ctx, look, dpr, 12, look.ink, "left", "middle");
    ctx.fillText(count(one.value), l + long + 8 * dpr, y + thick / 2);
  });
}

export function draw(
  ctx: CanvasRenderingContext2D,
  w: number,
  h: number,
  dpr: number,
  model: Model,
  look: Look,
  hover: number | null = null,
) {
  ctx.setTransform(1, 0, 0, 1, 0, 0);
  ctx.fillStyle = look.paper;
  ctx.fillRect(0, 0, w, h);
  if (model.kind === "line") chart(ctx, w, h, dpr, model, look, hover);
  else if (model.kind === "bars") bars(ctx, w, h, dpr, model, look);
  else note(ctx, w, h, dpr, model.text, look);
}
