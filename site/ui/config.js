import { HUES } from './hues.js';

const rule = (hue, at) => `:root${at} { --accent: var(--${hue}); --link: var(--${hue}-link); }`;

export function tintCss(tint) {
  const lines = HUES.includes(tint) ? [rule(tint, ':not([data-tint])')] : [];
  for (const hue of HUES) lines.push(rule(hue, `[data-tint="${hue}"]`));
  return `${lines.join('\n')}\n`;
}
