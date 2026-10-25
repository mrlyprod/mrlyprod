import { HUES } from "./hues.ts";

const rule = (hue: string, at: string) => `:root${at} { --accent: var(--${hue}); --link: var(--${hue}-link); }`;

export function tintCss(tint: string) {
  const lines = HUES.includes(tint) ? [rule(tint, ":not([data-tint])")] : [];
  for (const hue of HUES) lines.push(rule(hue, `[data-tint="${hue}"]`));
  return `${lines.join("\n")}\n`;
}
