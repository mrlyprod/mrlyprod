import { palette } from "./palette.gen.js";

/* TYPES */

export type Name = keyof typeof palette;

export type Role = "ground" | "bg" | "panel" | "deep" | "line" | "fg" | "dim" | "accent" | "on-accent";

export type Roles = Record<Name | Role, string>;

export type Tint = { accent?: string; link?: string; theme?: "dark" | "light" };

/* ROLES */

export const dark: Roles = {
  ...palette,
  ground: palette.black,
  bg: "#070707",
  panel: "#111112",
  deep: palette.black,
  line: "#1f1f20",
  fg: palette.white,
  dim: palette.gray,
  accent: palette.indigo,
  "on-accent": palette.white,
};

export const light: Roles = {
  ...palette,
  ground: palette.white,
  bg: "#f8f8f9",
  panel: "#f1f1f2",
  deep: "#e8e8e9",
  line: "#dddddf",
  fg: palette.black,
  dim: "#555558",
  accent: palette.indigo,
  "on-accent": palette.white,
};

/* TINT */

const HUES: Record<string, Name | undefined> = Object.fromEntries(
  (Object.entries(palette) as [Name, string][])
    .filter(([name]) => name !== "black" && name !== "white")
    .map(([name, hex]) => [hex, name]),
);

export function tinted(roles: Roles, { accent, link, theme }: Tint = {}): Roles {
  const out = { ...roles };
  const hex = String(accent ?? "")
    .trim()
    .toLowerCase();
  if (!hex) return out;
  out.accent = hex;
  const hue = HUES[hex];
  if (!hue || hue === "blue") return out;
  [out.blue, out[hue]] = [out[hue], out.blue];
  if (theme === "light" && link) out.blue = String(link).trim().toLowerCase();
  return out;
}
