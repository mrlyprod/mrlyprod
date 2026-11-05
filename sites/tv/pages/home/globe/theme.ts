export type Rgb = [number, number, number];

export type Theme = { bg: Rgb; fg: Rgb; accent: Rgb; dim: Rgb; line: Rgb; dark: boolean };

const VARS = ["bg", "fg", "accent", "dim", "line"] as const;
const DARK: Theme = {
  bg: [0.04, 0.04, 0.05],
  fg: [0.94, 0.94, 0.92],
  accent: [1, 0.62, 0.2],
  dim: [0.55, 0.55, 0.55],
  line: [0.2, 0.2, 0.22],
  dark: true,
};
const LIGHT: Theme = {
  bg: [0.99, 0.99, 0.98],
  fg: [0.08, 0.08, 0.09],
  accent: [0.9, 0.42, 0.05],
  dim: [0.5, 0.5, 0.5],
  line: [0.86, 0.86, 0.84],
  dark: false,
};
const SCHEME = "(prefers-color-scheme: dark)";

export const mixRgb = (a: Rgb, b: Rgb, k: number): Rgb => [
  a[0] + (b[0] - a[0]) * k,
  a[1] + (b[1] - a[1]) * k,
  a[2] + (b[2] - a[2]) * k,
];

function probe(value: string): Rgb | null {
  const canvas = document.createElement("canvas");
  canvas.width = canvas.height = 1;
  const ctx = canvas.getContext("2d", { willReadFrequently: true });
  if (!ctx) return null;
  ctx.fillStyle = "#010203";
  ctx.fillStyle = value;
  if (ctx.fillStyle === "#010203") return null;
  ctx.fillRect(0, 0, 1, 1);
  const [r, g, b] = ctx.getImageData(0, 0, 1, 1).data;
  return [r / 255, g / 255, b / 255];
}

export function readTheme(el: HTMLElement): Theme {
  const style = getComputedStyle(el);
  const out = { ...(matchMedia(SCHEME).matches ? DARK : LIGHT) };
  for (const name of VARS) {
    const value = style.getPropertyValue(`--${name}`).trim();
    const rgb = value ? probe(value) : null;
    if (rgb) out[name] = rgb;
  }
  out.dark = out.bg[0] * 0.3 + out.bg[1] * 0.6 + out.bg[2] * 0.1 < 0.5;
  return out;
}

export function watchTheme(el: HTMLElement, onChange: (theme: Theme) => void): () => void {
  const media = matchMedia(SCHEME);
  const tell = () => onChange(readTheme(el));
  media.addEventListener("change", tell);
  const eye = new MutationObserver(tell);
  const watched = { attributes: true, attributeFilter: ["class", "data-theme", "style"] };
  eye.observe(document.documentElement, watched);
  if (document.body) eye.observe(document.body, watched);
  return () => {
    media.removeEventListener("change", tell);
    eye.disconnect();
  };
}
