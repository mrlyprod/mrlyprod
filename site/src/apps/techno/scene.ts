import type { Ready } from "../../lib/app/hooks.ts";
import type { Page } from "../../lib/app/page.tsx";
import type { Opts, View } from "../../lib/app/run.ts";
import type { Option, Raw, Spec, Units, Value } from "../../lib/app/spec.ts";
import { tidy } from "../../lib/app/spec.ts";
import type { Mood, Note, Pattern, Score } from "../../lib/audio/audio.ts";
import { KEYS, pattern, tempo } from "../../lib/audio/audio.ts";
import type { Bus, Song } from "../../lib/audio/tempo.ts";
import { veil } from "../../lib/color.ts";

/* TYPES */

export type Feel = Pick<Song, "bpm" | "key" | "scale" | "density" | "drive" | "swing">;

export type Knobs = Feel & Pick<Song, "bars">;

export type Style = { id: string; name: string; song: Feel };

export type Given = Opts & { audio?: Pick<Bus, "at"> | null };

export type Techno = ReturnType<typeof make>;

export type Hands = { go: () => void; mute: () => void; going: () => boolean };

export type Held = Techno & Hands;

type Tone = (typeof TONES)[number];

type Rect = [number, number, number, number];

const TONES = ["on", "near", "lit", "off"] as const;

const NOTES = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];

export const SPEC: Spec = [
  { key: "bpm", label: "Tempo", kind: "slider", def: 128, min: 118, max: 140, step: 1, unit: "bpm", group: "Song" },
  {
    key: "key",
    label: "Key",
    kind: "pick",
    def: "a",
    options: KEYS.map((key, i): Option => [key, NOTES[i]]),
    group: "Song",
  },
  {
    key: "scale",
    label: "Scale",
    kind: "segment",
    def: "minor",
    options: [
      ["minor", "Min"],
      ["dorian", "Dor"],
      ["phrygian", "Phr"],
    ],
    group: "Song",
  },
  {
    key: "bars",
    label: "Bars",
    kind: "segment",
    def: 32,
    options: [8, 16, 32, 64].map((n): Option => [n, String(n)]),
    group: "Song",
  },
  { key: "density", label: "Density", kind: "slider", def: 0.6, min: 0, max: 1, step: 0.05, group: "Feel" },
  { key: "drive", label: "Drive", kind: "slider", def: 0.4, min: 0, max: 1, step: 0.05, group: "Feel" },
  { key: "swing", label: "Swing", kind: "slider", def: 0.1, min: 0, max: 0.3, step: 0.01, group: "Feel" },
];

export const PAGE = {
  spec: SPEC,
  keys: [
    {
      key: "Enter",
      label: (_value, handle) => (handle?.going?.() ? "Stop" : "Play"),
      does: "Again to stop",
      act: "go",
      button: true,
      menu: "Edit",
    },
    { key: "m", label: "Sound", act: "mute" },
  ],
  actions: { go: (scene) => scene.go?.(), mute: (scene) => scene.mute?.() },
} satisfies Page<Ready & Units, Held>;

export const FEEL: (keyof Feel)[] = ["bpm", "key", "scale", "density", "drive", "swing"];

export const STYLES: Style[] = [
  { id: "acid", name: "Acid", song: { bpm: 128, key: "a", scale: "minor", density: 0.6, drive: 0.4, swing: 0.1 } },
  {
    id: "warehouse",
    name: "Warehouse",
    song: { bpm: 134, key: "d", scale: "phrygian", density: 0.75, drive: 0.6, swing: 0.05 },
  },
  { id: "dub", name: "Dub", song: { bpm: 120, key: "f", scale: "dorian", density: 0.35, drive: 0.2, swing: 0.15 } },
  { id: "peak", name: "Peak time", song: { bpm: 138, key: "c", scale: "minor", density: 0.9, drive: 0.7, swing: 0 } },
  {
    id: "hypnotic",
    name: "Hypnotic",
    song: { bpm: 124, key: "e", scale: "minor", density: 0.45, drive: 0.3, swing: 0.08 },
  },
  {
    id: "detroit",
    name: "Detroit",
    song: { bpm: 126, key: "ds", scale: "dorian", density: 0.55, drive: 0.25, swing: 0.12 },
  },
  { id: "rave", name: "Rave", song: { bpm: 140, key: "g", scale: "dorian", density: 0.8, drive: 0.55, swing: 0.02 } },
  {
    id: "slow",
    name: "Slow burn",
    song: { bpm: 118, key: "a", scale: "phrygian", density: 0.3, drive: 0.5, swing: 0.2 },
  },
];

export const styled = (value: Value) =>
  STYLES.find((one) => FEEL.every((key) => String(one.song[key]) === String(value[key])))?.id ?? "";

const TAIL = 2;
const HEARD = 50;
const ROWS = 4;
const WIDE = 17.5;
const TALL = 6.2;
const PITCH = 1.35;
const SIDE = 0.08;
const STRETCH = 1.7;

/* SONG */

export const song = (value: Raw): Song => ({ ...(tidy(SPEC, value) as Knobs), seed: Number(value.seed) >>> 0 });

export const seconds = (value: Raw) => (song(value).bars * tempo(song(value)).bar) / 1000 + TAIL;

export const score = (value: Song): Score => ({ seed: value.seed >>> 0, mood: { name: "drive", song: song(value) } });

function at(tune: Song, t: number) {
  const time = tempo(tune);
  const i = time.index(Math.max(0, t - HEARD));
  return { bar: Math.floor(i / 16) % tune.bars, step: i % 16 };
}

/* STAMP */

export function stamp(ctx: CanvasRenderingContext2D, w: number, h: number, p: Pattern, accent: string) {
  const pad = Math.round(w * SIDE);
  const cell = (w - 2 * pad) / 16;
  const tall = cell * STRETCH;
  const top = (h - ROWS * tall) / 2;
  ctx.setTransform(1, 0, 0, 1, 0, 0);
  ctx.clearRect(0, 0, w, h);
  [p.kick, p.hat, p.clap, p.bass].forEach((row, r) => {
    for (let s = 0; s < 16; s++) {
      ctx.fillStyle = row[s] ? accent : veil(accent, 0.12);
      ctx.fillRect(pad + s * cell, top + r * tall, Math.max(1, cell - 1), Math.max(1, tall - 1));
    }
  });
}

/* SCENE */

export function make(canvas: HTMLCanvasElement, view: View, opts: Given = {}) {
  const ctx = canvas.getContext("2d")!;
  const tune = song(opts);
  const mood: Mood = { name: "drive", song: tune };
  const audio = opts.audio;
  const kept = new Map<number, Pattern>();
  const bar = (b: number) => {
    if (!kept.has(b)) kept.set(b, pattern(tune.seed, tune, b));
    return kept.get(b)!;
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
    const tones: Record<Tone, string> = {
      on: accent,
      near: veil(accent, 0.55),
      lit: veil(accent, 0.22),
      off: veil(accent, 0.08),
    };
    const rects: Record<Tone, Rect[]> = { on: [], near: [], lit: [], off: [] };
    const put = (tone: Tone, x: number, y: number, a: number, b: number) => rects[tone].push([x, y, a, b]);
    const hits = [p.kick, p.hat, p.clap, p.bass] as const;
    const small = (r: number, s: number) =>
      (r === 1 && hits[r][s] && !p.open[s]) || (r === 3 && hits[r][s] && !hits[r][s].accent);
    const inset = (r: number, s: number) => (small(r, s) ? Math.round(pad * 0.22) : 0);
    const thick = Math.max(1, Math.round(pad * 0.16));
    for (let r = 0; r < ROWS; r++) {
      for (let s = 0; s < 16; s++) {
        const hit = hits[r][s];
        const here = s === now.step;
        const tone = hit ? (here ? "on" : "near") : here ? "lit" : "off";
        const i = inset(r, s);
        put(tone, xs[s] + i, ys[r] + i, pad - 2 * i, pad - 2 * i);
        if (r !== 3 || !(hit as Note | null)?.slide || !p.bass[s - 1]) continue;
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
      put(b === now.bar ? "on" : b < now.bar ? "lit" : "off", a, strip, Math.max(1, z - a), thin);
    }
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, w, h);
    for (const tone of TONES) {
      ctx.fillStyle = tones[tone];
      ctx.beginPath();
      for (const [x, y, a, b] of rects[tone]) ctx.rect(x, y, a, b);
      ctx.fill();
    }
  };
  const draw = () => {
    audio?.at(view.t, mood);
    paint();
  };
  return { draw, size: paint, theme: paint, wake: () => view.wake?.() };
}
