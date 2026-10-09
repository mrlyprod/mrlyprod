import { tidy } from '../../lib/knobs.js';
import { veil } from '../../lib/scene.js';
import { KEYS, pattern, tempo } from '../../lib/space/audio.js';

const NOTES = ['C', 'C#', 'D', 'D#', 'E', 'F', 'F#', 'G', 'G#', 'A', 'A#', 'B'];

export const SPEC = [
  { key: 'bpm', label: 'Tempo', kind: 'slider', def: 128, min: 118, max: 140, step: 1, unit: 'bpm', group: 'Song' },
  { key: 'key', label: 'Key', kind: 'pick', def: 'a', options: KEYS.map((key, i) => [key, NOTES[i]]), group: 'Song' },
  { key: 'scale', label: 'Scale', kind: 'segment', def: 'minor', options: [['minor', 'Min'], ['dorian', 'Dor'], ['phrygian', 'Phr']], group: 'Song' },
  { key: 'bars', label: 'Bars', kind: 'segment', def: 32, options: [8, 16, 32, 64].map((n) => [n, String(n)]), group: 'Song' },
  { key: 'density', label: 'Density', kind: 'slider', def: 0.6, min: 0, max: 1, step: 0.05, group: 'Feel' },
  { key: 'drive', label: 'Drive', kind: 'slider', def: 0.4, min: 0, max: 1, step: 0.05, group: 'Feel' },
  { key: 'swing', label: 'Swing', kind: 'slider', def: 0.1, min: 0, max: 0.3, step: 0.01, group: 'Feel' },
];

export const PAGE = {
  spec: SPEC,
  keys: [
    { key: 'Enter', label: 'Play', does: 'Again to stop', act: 'go' },
    { key: 'm', label: 'Sound', act: 'mute' },
  ],
  actions: { go: (scene) => scene.go?.(), mute: (scene) => scene.mute?.() },
};

const TAIL = 2;
const HEARD = 50;
const ROWS = 4;
const WIDE = 17.5;
const TALL = 6.2;
const PITCH = 1.35;

/* SONG */

export const song = (value) => ({ ...tidy(SPEC, value), seed: value.seed >>> 0 });

export const seconds = (value) => (song(value).bars * tempo(song(value)).bar) / 1000 + TAIL;

export const score = (value) => ({ seed: value.seed >>> 0, mood: { name: 'drive', song: song(value) } });

function at(tune, t) {
  const time = tempo(tune);
  const i = time.index(Math.max(0, t - HEARD));
  return { bar: Math.floor(i / 16) % tune.bars, step: i % 16 };
}

/* SCENE */

export function make(canvas, view, opts = {}) {
  const ctx = canvas.getContext('2d');
  const tune = song(opts);
  const mood = { name: 'drive', song: tune };
  const audio = opts.audio;
  const kept = new Map();
  const bar = (b) => {
    if (!kept.has(b)) kept.set(b, pattern(tune.seed, tune, b));
    return kept.get(b);
  };
  const paint = () => {
    const { accent } = view.look();
    const { w, h } = view;
    const now = at(tune, view.t);
    const p = bar(now.bar);
    const cell = Math.max(4, Math.floor(Math.min((w * 0.88) / WIDE, (h * 0.7) / TALL)));
    const gap = Math.max(1, Math.round(cell * 0.14));
    const pad = cell - gap;
    const x0 = Math.floor((w - WIDE * cell) / 2);
    const y0 = Math.floor((h - TALL * cell) / 2);
    const xs = Array.from({ length: 16 }, (_, s) => x0 + Math.round((s + Math.floor(s / 4) * 0.5) * cell));
    const ys = Array.from({ length: ROWS }, (_, r) => y0 + Math.round(r * PITCH * cell));
    const tones = { on: accent, near: veil(accent, 0.55), lit: veil(accent, 0.22), off: veil(accent, 0.08) };
    const rects = { on: [], near: [], lit: [], off: [] };
    const put = (tone, x, y, a, b) => rects[tone].push([x, y, a, b]);
    const hits = [p.kick, p.hat, p.clap, p.bass];
    const small = (r, s) => (r === 1 && hits[r][s] && !p.open[s]) || (r === 3 && hits[r][s] && !hits[r][s].accent);
    const inset = (r, s) => (small(r, s) ? Math.round(pad * 0.22) : 0);
    const thick = Math.max(1, Math.round(pad * 0.16));
    for (let r = 0; r < ROWS; r++) {
      for (let s = 0; s < 16; s++) {
        const hit = hits[r][s];
        const here = s === now.step;
        const tone = hit ? (here ? 'on' : 'near') : here ? 'lit' : 'off';
        const i = inset(r, s);
        put(tone, xs[s] + i, ys[r] + i, pad - 2 * i, pad - 2 * i);
        if (r !== 3 || !hit?.slide || !p.bass[s - 1]) continue;
        const from = xs[s - 1] + pad - inset(r, s - 1);
        put(tone, from, ys[r] + Math.round((pad - thick) / 2), xs[s] + i - from, thick);
      }
    }
    const strip = y0 + Math.round((ROWS * PITCH + 0.45) * cell);
    const long = xs[15] + pad - x0;
    const thin = Math.max(2, Math.round(cell * 0.22));
    for (let b = 0; b < tune.bars; b++) {
      const a = x0 + Math.round((b * long) / tune.bars);
      const z = x0 + Math.round(((b + 1) * long) / tune.bars) - Math.max(1, Math.round(gap / 2));
      put(b === now.bar ? 'on' : b < now.bar ? 'lit' : 'off', a, strip, Math.max(1, z - a), thin);
    }
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, w, h);
    for (const [tone, list] of Object.entries(rects)) {
      ctx.fillStyle = tones[tone];
      ctx.beginPath();
      for (const [x, y, a, b] of list) ctx.rect(x, y, a, b);
      ctx.fill();
    }
  };
  const draw = () => {
    audio?.at(view.t, mood);
    paint();
  };
  return { draw, size: paint, theme: paint, wake: () => view.wake?.() };
}
