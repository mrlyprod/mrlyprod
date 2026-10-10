import { palette } from './palette.js';

export const dark = { ...palette, ground: palette.black, bg: '#070707', panel: '#111112', deep: palette.black, line: '#1f1f20', fg: palette.white, dim: palette.gray, accent: palette.indigo, 'on-accent': palette.white };

export const light = { ...palette, ground: palette.white, bg: '#f8f8f9', panel: '#f1f1f2', deep: '#e8e8e9', line: '#dddddf', fg: palette.black, dim: '#555558', accent: palette.indigo, 'on-accent': palette.white };

/* TINT */

const HUES = Object.fromEntries(Object.entries(palette).filter(([name]) => name !== 'black' && name !== 'white').map(([name, hex]) => [hex, name]));

export function tinted(roles, { accent, link, theme } = {}) {
  const out = { ...roles };
  const hex = String(accent ?? '').trim().toLowerCase();
  if (!hex) return out;
  out.accent = hex;
  const hue = HUES[hex];
  if (!hue || hue === 'blue') return out;
  [out.blue, out[hue]] = [out[hue], out.blue];
  if (theme === 'light' && link) out.blue = String(link).trim().toLowerCase();
  return out;
}
