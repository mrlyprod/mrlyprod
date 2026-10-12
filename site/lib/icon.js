import { frame, mix } from 'mrlyjs/view';

const POWER = 5;
const SPAN = 0.92;
const SHEEN = 0.18;
const RIM = 0.22;
const WHITE = [255, 255, 255, 255];
const BLACK = [0, 0, 0, 255];

const plates = new Map();

function pixels(size, hue) {
  const key = `${size}:${hue.join()}`;
  if (plates.has(key)) return plates.get(key);
  const top = mix(hue, WHITE, 0.28);
  const bottom = mix(hue, BLACK, 0.3);
  const a = (size * SPAN) / 2;
  const c = size / 2;
  const colors = new Uint8ClampedArray(size * size * 4);
  for (let y = 0; y < size; y++) {
    const t = y / (size - 1);
    const row = mix(top, bottom, t);
    for (let x = 0; x < size; x++) {
      const nx = Math.abs(x + 0.5 - c) / a;
      const ny = Math.abs(y + 0.5 - c) / a;
      const d = ((nx ** POWER + ny ** POWER) ** (1 / POWER) - 1) * a;
      const cover = d > 0.5 ? 0 : d < -0.5 ? 1 : 0.5 - d;
      if (cover <= 0) continue;
      const sheen = Math.max(0, 1 - ((x - c * 0.7) ** 2 + (y - c * 0.5) ** 2) / (a * a * 1.1)) * SHEEN;
      const rim = d > -2.5 ? RIM * (1 - (-d) / 2.5) : 0;
      const lift = Math.min(1, sheen + rim);
      const i = (y * size + x) * 4;
      colors[i] = row[0] + (255 - row[0]) * lift;
      colors[i + 1] = row[1] + (255 - row[1]) * lift;
      colors[i + 2] = row[2] + (255 - row[2]) * lift;
      colors[i + 3] = 255 * cover;
    }
  }
  const made = { shape: [size, size], colors };
  plates.set(key, made);
  return made;
}

export function plate(pen, ink, hue = 'blue', inset = 0.2) {
  const tone = ink[hue];
  if (!tone) throw new Error(`icon: no hue ${hue}`);
  const size = Math.min(pen.width, pen.height);
  const x = (pen.width - size) / 2;
  const y = (pen.height - size) / 2;
  pen.image(x, y, size, size, pixels(size, tone));
  const pad = size * inset;
  const box = frame(x + pad, y + pad, size - 2 * pad, size - 2 * pad);
  const paint = { white: WHITE, light: mix(tone, WHITE, 0.6), deep: mix(tone, BLACK, 0.45), ink: mix(tone, BLACK, 0.75), shadow: [0, 0, 0, 90], tone };
  return { box, paint, size };
}

export function shadow(pen, size, draw) {
  const lift = size * 0.012;
  draw([0, 0, 0, 40], lift * 2);
  draw([0, 0, 0, 60], lift);
  draw(null, 0);
}
