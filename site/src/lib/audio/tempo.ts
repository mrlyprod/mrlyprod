import type { Engine, Mood } from "./audio.ts";
import type { Args } from "./voices.ts";

/* TYPES */

export type Song = {
  seed: number;
  bpm: number;
  key: string;
  scale: string;
  bars: number;
  density: number;
  drive: number;
  swing: number;
};

export type Tempo = {
  six: number;
  bar: number;
  time: (i: number) => number;
  index: (t: number) => number;
  first: (t: number) => number;
};

export type Bus = {
  at: (t: number, mood?: Mood) => void;
  cue: (name: string, t: number, args?: Args) => void;
  attach: (one: Engine) => void;
  detach: () => void;
};

/* NUMBERS */

export const STEPS = 16;

export const KEYS = ["c", "cs", "d", "ds", "e", "f", "fs", "g", "gs", "a", "as", "b"];

export const SONG: Song = {
  seed: 1,
  bpm: 128,
  key: "a",
  scale: "minor",
  bars: 32,
  density: 0.6,
  drive: 0.4,
  swing: 0.1,
};

/* TEMPO */

export function tempo(song: Song = SONG): Tempo {
  const six = 15000 / song.bpm;
  const swing = song.swing ?? 0;
  const time = (i: number) => (i + (i % 2) * swing) * six;
  const index = (t: number) => {
    const i = Math.max(0, Math.floor(t / six));
    return time(i) > t ? Math.max(0, i - 1) : i;
  };
  const first = (t: number) => {
    let i = Math.max(0, Math.ceil(t / six) - 1);
    while (time(i) < t) i++;
    return i;
  };
  return { six, bar: six * STEPS, time, index, first };
}

/* BUS */

export function bus(): Bus {
  let engine: Engine | null = null;
  return {
    at: (t, mood) => engine?.at(t, mood),
    cue: (name, t, args) => engine?.cue(name, t, args),
    attach: (one) => {
      engine = one;
    },
    detach: () => {
      engine = null;
    },
  };
}
