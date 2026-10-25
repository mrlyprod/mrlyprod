import { HUES, tints } from "./theme/hues.ts";
import { palette } from "./theme/palette.gen.js";
import type { Name } from "./theme/theme.ts";
import { light as DAY, dark as NIGHT } from "./theme/theme.ts";

/* TYPES */

export type Paints = { paper: string; accent: string; ink: string };

export function rgb(value: string | readonly number[]): number[] {
  const text = String(value).trim();
  if (text.startsWith("#")) {
    const hex = text.length < 7 ? [...text.slice(1, 4)].map((c) => c + c).join("") : text.slice(1, 7);
    const n = Number.parseInt(hex, 16);
    return [(n >> 16) & 255, (n >> 8) & 255, n & 255];
  }
  const parts = text.match(/-?\d+(\.\d+)?/g);
  return parts ? parts.slice(0, 3).map(Number) : [0, 0, 0];
}

export const bright = (color: string | readonly number[]) => rgb(color).reduce((a, b) => a + b, 0) > 384;

export function veil(color: string, alpha: number): string {
  const [r, g, b] = rgb(color);
  return `rgba(${r}, ${g}, ${b}, ${alpha})`;
}

export { HUES };

export const TINTS = tints("Site");

export function paints(canvas: Element | null | undefined, tint = ""): Paints {
  const night = typeof matchMedia === "function" && matchMedia("(prefers-color-scheme: dark)").matches;
  const base = night ? NIGHT : DAY;
  const css = typeof getComputedStyle === "function" && canvas ? getComputedStyle(canvas) : null;
  const read = (name: string, fallback: string) => css?.getPropertyValue(name).trim() || fallback;
  const paper = read("--art", read("--ground", base.ground ?? palette.white));
  const accent = HUES.includes(tint)
    ? read(`--${tint}`, palette[tint as Name])
    : read("--accent", base.accent ?? palette.blue);
  const ink = read("--fg", base.fg ?? palette.black);
  return { paper, accent, ink };
}

export const mix = (a: readonly number[], b: readonly number[], t: number) =>
  `rgb(${a.map((v, i) => Math.round(v + (b[i] - v) * t)).join(", ")})`;

export const blend = (a: string, b: string, t: number) => mix(rgb(a), rgb(b), t);

export const hex = (c: readonly number[]) =>
  `#${c
    .slice(0, 3)
    .map((v) =>
      Math.round(Math.min(255, Math.max(0, v)))
        .toString(16)
        .padStart(2, "0"),
    )
    .join("")}`;
