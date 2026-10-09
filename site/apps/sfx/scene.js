import { tidy } from '../../lib/knobs.js';
import { veil } from '../../lib/scene.js';
import { EFFECTS, render } from '../../lib/space/audio.js';

const named = (word) => word[0].toUpperCase() + word.slice(1);

export const NAMES = Object.keys(EFFECTS);

export const SPEC = [
  { key: 'effect', label: 'Effect', kind: 'pick', def: 'jump', options: NAMES.map((name) => [name, named(name)]), group: 'Effect' },
  { key: 'pitch', label: 'Pitch', kind: 'slider', def: 0, min: -12, max: 12, step: 1, unit: 'st', group: 'Shape' },
  { key: 'length', label: 'Length', kind: 'slider', def: 1, min: 0.5, max: 2, step: 0.1, unit: 'x', group: 'Shape' },
  { key: 'bright', label: 'Bright', kind: 'slider', def: 0.5, min: 0, max: 1, step: 0.05, group: 'Shape' },
  { key: 'drive', label: 'Drive', kind: 'slider', def: 0.2, min: 0, max: 1, step: 0.05, group: 'Shape' },
  { key: 'space', label: 'Space', kind: 'slider', def: 0.4, min: 0, max: 1, step: 0.05, group: 'Shape' },
];

export const PAGE = {
  spec: SPEC,
  keys: [
    { key: 'Enter', label: 'Play', act: 'go', button: true },
    { key: ['ArrowLeft', 'ArrowUp'], label: 'Previous', act: 'prev' },
    { key: ['ArrowRight', 'ArrowDown'], label: 'Next', act: 'next' },
    { key: 'm', label: 'Sound', act: 'mute' },
  ],
  actions: { go: (scene) => scene.go?.(), prev: (scene) => scene.step?.(-1), next: (scene) => scene.step?.(1), mute: (scene) => scene.mute?.() },
};

const TAIL = 1.5;
const PEAKS = 1024;
const COLS = 240;
const BANDS = 48;
const LOW = 40;
const HIGH = 16000;
const WINDOW = 1024;
const RANGE = 60;
const LEVELS = 8;

/* SOUND */

const shape = (value) => ({ pitch: value.pitch, length: value.length, bright: value.bright, drive: value.drive, space: value.space });

export const seconds = (value) => EFFECTS[value.effect].span(value.length) + TAIL;

export const score = (value) => ({ seed: value.seed >>> 0, cues: [[value.effect, 0, shape(value)]] });

/* LOOK */

export const band = (f, rows = BANDS) => Math.floor((rows * Math.log(f / LOW)) / Math.log(HIGH / LOW));

const edge = (r, rows) => LOW * (HIGH / LOW) ** (r / rows);

function fft(re, im) {
  const n = re.length;
  for (let i = 1, j = 0; i < n; i++) {
    let bit = n >> 1;
    for (; j & bit; bit >>= 1) j ^= bit;
    j ^= bit;
    if (i < j) {
      [re[i], re[j]] = [re[j], re[i]];
      [im[i], im[j]] = [im[j], im[i]];
    }
  }
  for (let size = 2; size <= n; size <<= 1) {
    const turn = (-2 * Math.PI) / size;
    const [wr, wi] = [Math.cos(turn), Math.sin(turn)];
    for (let i = 0; i < n; i += size) {
      let [cr, ci] = [1, 0];
      for (let k = 0; k < size / 2; k++) {
        const a = i + k;
        const b = a + size / 2;
        const xr = re[b] * cr - im[b] * ci;
        const xi = re[b] * ci + im[b] * cr;
        re[b] = re[a] - xr;
        im[b] = im[a] - xi;
        re[a] += xr;
        im[a] += xi;
        [cr, ci] = [cr * wr - ci * wi, cr * wi + ci * wr];
      }
    }
  }
}

export function analyse(buffer, cols = COLS, rows = BANDS) {
  const n = buffer.length;
  const rate = buffer.sampleRate;
  const mono = new Float32Array(n);
  for (let c = 0; c < buffer.numberOfChannels; c++) {
    const data = buffer.getChannelData(c);
    for (let i = 0; i < n; i++) mono[i] += data[i] / buffer.numberOfChannels;
  }
  const peaks = new Float32Array(PEAKS * 2);
  for (let x = 0; x < PEAKS; x++) {
    let [lo, hi] = [0, 0];
    for (let i = Math.floor((x * n) / PEAKS), end = Math.floor(((x + 1) * n) / PEAKS); i < end; i++) {
      lo = Math.min(lo, mono[i]);
      hi = Math.max(hi, mono[i]);
    }
    peaks[2 * x] = lo;
    peaks[2 * x + 1] = hi;
  }
  const spans = Array.from({ length: rows }, (_, r) => {
    const [lo, hi] = [edge(r, rows), edge(r + 1, rows)].map((f) => (f * WINDOW) / rate);
    const near = Math.round((lo + hi) / 2);
    return Math.ceil(hi) > Math.ceil(lo) ? [Math.ceil(lo), Math.ceil(hi)] : [near, near + 1];
  });
  const power = new Float64Array(cols * rows);
  const re = new Float64Array(WINDOW);
  const im = new Float64Array(WINDOW);
  for (let c = 0; c < cols; c++) {
    const from = Math.floor(((c + 0.5) * n) / cols) - WINDOW / 2;
    for (let i = 0; i < WINDOW; i++) {
      re[i] = (mono[from + i] ?? 0) * (0.5 - 0.5 * Math.cos((2 * Math.PI * i) / (WINDOW - 1)));
      im[i] = 0;
    }
    fft(re, im);
    spans.forEach(([a, b], r) => {
      let sum = 0;
      for (let k = a; k < b; k++) sum += re[k] * re[k] + im[k] * im[k];
      power[c * rows + r] = sum / (b - a);
    });
  }
  const db = power.map((p) => 10 * Math.log10(p + 1e-12));
  const top = Math.max(...db);
  return { peaks, cols, rows, heat: Float32Array.from(db, (v) => Math.min(1, Math.max(0, (v - top + RANGE) / RANGE))) };
}

/* SCENE */

export function make(canvas, view, opts = {}) {
  const ctx = canvas.getContext('2d');
  const value = { ...tidy(SPEC, opts), seed: opts.seed >>> 0 };
  const audio = opts.audio;
  const long = seconds(value) * 1000;
  let shown = null;
  let played = null;
  let gone = false;
  const paint = () => {
    const { accent } = view.look();
    const { w, h, dpr } = view;
    const side = Math.round(w * 0.06);
    const top = Math.round(h * 0.1);
    const wide = w - 2 * side;
    const tall = h - 2 * top;
    const wave = Math.round(tall * 0.48);
    const mid = top + Math.round(wave / 2);
    const below = top + Math.round(tall * 0.56);
    const deep = top + tall - below;
    const line = Math.max(1, Math.round(dpr));
    const k = played === null ? -1 : (view.t - played) / long;
    const playing = k >= 0 && k < 1;
    const head = side + Math.round(Math.min(1, Math.max(0, k)) * wide);
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, w, h);
    ctx.fillStyle = veil(accent, 0.25);
    ctx.fillRect(side, mid, wide, line);
    ctx.fillRect(side, below + deep, wide, line);
    if (shown) {
      const bar = Math.max(2, Math.round(3 * dpr));
      const gap = Math.max(1, Math.round(dpr));
      const count = Math.floor(wide / (bar + gap));
      for (const lit of playing ? [true, false] : [true]) {
        ctx.fillStyle = lit ? accent : veil(accent, 0.45);
        ctx.beginPath();
        for (let b = 0; b < count; b++) {
          const x = side + b * (bar + gap);
          if (playing && x < head !== lit) continue;
          let [lo, hi] = [0, 0];
          for (let p = Math.floor((b * PEAKS) / count), end = Math.max(p + 1, Math.floor(((b + 1) * PEAKS) / count)); p < end; p++) {
            lo = Math.min(lo, shown.peaks[2 * p]);
            hi = Math.max(hi, shown.peaks[2 * p + 1]);
          }
          const y0 = Math.round(mid - hi * (wave / 2));
          const y1 = Math.round(mid - lo * (wave / 2));
          ctx.rect(x, y0, bar, Math.max(line, y1 - y0));
        }
        ctx.fill();
      }
      const xs = Array.from({ length: shown.cols + 1 }, (_, c) => side + Math.round((c * wide) / shown.cols));
      const ys = Array.from({ length: shown.rows + 1 }, (_, r) => below + deep - Math.round((r * deep) / shown.rows));
      for (let level = 1; level <= LEVELS; level++) {
        ctx.fillStyle = veil(accent, (level / LEVELS) ** 1.5);
        ctx.beginPath();
        for (let c = 0; c < shown.cols; c++) {
          for (let r = 0; r < shown.rows; r++) {
            if (Math.round(shown.heat[c * shown.rows + r] * LEVELS) === level) ctx.rect(xs[c], ys[r + 1], xs[c + 1] - xs[c], ys[r] - ys[r + 1]);
          }
        }
        ctx.fill();
      }
    }
    if (playing) {
      ctx.fillStyle = accent;
      ctx.fillRect(head, top, Math.max(2, Math.round(1.5 * dpr)), below + deep - top);
    }
  };
  const draw = () => {
    audio?.at(view.t);
    paint();
  };
  const cue = () => {
    played = view.t;
    audio?.cue(value.effect, view.t, shape(value));
  };
  const made = (buffer) => {
    if (gone) return;
    shown = analyse(buffer);
    paint();
  };
  Promise.resolve()
    .then(() => (opts.render ?? render)(long / 1000, score(value)))
    .then(made)
    .catch(() => {});
  const stop = () => {
    gone = true;
  };
  return { draw, size: paint, theme: paint, cue, stop };
}
