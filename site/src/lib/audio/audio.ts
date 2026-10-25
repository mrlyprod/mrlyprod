import { rng } from "../rand.ts";
import type { Song, Tempo } from "./tempo.ts";
import { KEYS, SONG, STEPS, tempo } from "./tempo.ts";
import type { Args } from "./voices.ts";
import {
  ARP_LEVEL,
  acid,
  arp,
  bass,
  CUT,
  clap,
  curve,
  EFFECTS,
  filter,
  hat,
  impulse,
  kick,
  pad,
  SALT,
  VOICES,
} from "./voices.ts";

export { bus, KEYS, SONG, tempo } from "./tempo.ts";

/* TYPES */

export type Note = { note: number; accent: boolean; slide: boolean };

export type Pattern = {
  kick: boolean[];
  hat: boolean[];
  open: boolean[];
  clap: boolean[];
  bass: (Note | null)[];
  chord: number[];
  arp: number[];
  skip: boolean[];
  octave: number[];
};

export type Parts = {
  pad?: number;
  sub?: number;
  hat?: number;
  kick?: number;
  roll?: number;
  clap?: number;
  acid?: number;
  bass?: number;
  arp?: number;
  arpLevel?: number;
};

export type Part = { name?: string; swell?: number; kick?: boolean; hat?: boolean; arp?: boolean };

export type Arrange = (ms: number) => Part | null | undefined;

export type Mood = { name?: string | null; song?: Partial<Song>; arrange?: Arrange; flicker?: number };

export type Levels = { sound?: number | boolean; music?: number | boolean };

export type Engine = {
  start: () => void;
  set: (value?: Levels) => void;
  at: (t: number, mood?: Mood) => void;
  cue: (name: string, t: number, args?: Args) => void;
  stream: () => MediaStream | null;
  stop: () => void;
};

export type Options = { seed?: number; make?: () => AudioContext };

export type Score = { seed?: number; mood?: Mood; cues?: [name: string, t: number, args?: Args][] };

export type Offline = (channels: number, length: number, rate: number) => OfflineAudioContext;

type Context = BaseAudioContext & Partial<Pick<AudioContext, "createMediaStreamDestination">>;

type Held = { i: number; ms: number; inside: boolean };

type Anchor = { t: number; c: number };

type Player = ReturnType<typeof player>;

/* NUMBERS */

const RATE = 44100;
const LEAD = 0.05;
const DRIFT = 0.08;
const AHEAD = 0.12;
const MUTE = 0.3;
const GATE = 0.08;
const LOUD = 0.9;
const ROOT = 36;
const REEL = 0x7f4a7c15;
const GOLD = 0.6180339887;
const PICKS = [0, 1, 2, 3, 1, 2];
const SKIPS = 0.2;
const LIFTS = 0.25;
const SEAL = 0.01;
const SCAN = 400;
const STRIDE = 10;
const BISECT = 24;
const OPEN_HZ = 500;
const SWELL_HZ = 3200;
const BASS_SHARE = 0.5;
const OCTAVE = 12;

const SCALES: Record<string, number[]> = {
  minor: [0, 2, 3, 5, 7, 8, 10],
  dorian: [0, 2, 3, 5, 7, 9, 10],
  phrygian: [0, 1, 3, 5, 7, 8, 10],
};

const PROGRESSIONS = [
  [0, 5, 3, 4],
  [0, 3, 5, 4],
  [0, 6, 5, 3],
  [0, 2, 5, 4],
  [5, 3, 0, 4],
  [0, 0, 5, 6],
];

const DEGREES = [0, 0, 0, 0, 7, 2, 4, 5, 7, 0];

const MOODS: Record<string, Parts> = {
  drift: { pad: 1, sub: 1, hat: 0.5 },
  build: { pad: 1, kick: 1, hat: 1, roll: 1 },
  drive: { kick: 1, hat: 1, clap: 1, acid: 1, pad: 1 },
  drop: {},
  land: { pad: 1, sub: 1 },
  acid: { kick: 1, hat: 1, clap: 1, acid: 1 },
};

const FILM: Record<string, Parts> = {
  intro: { pad: 1, sub: 1 },
  rise: { pad: 1, sub: 1, arp: 2 },
  hats: { pad: 1, sub: 1, arp: 2, hat: 1 },
  boost: { pad: 1, sub: 1, kick: 1, hat: 1, arp: 1 },
  [CUT]: {},
  hyper: { pad: 1, sub: 1, kick: 1, hat: 1, bass: 1, arp: 1, arpLevel: 0.14 },
  breakdown: { pad: 1, sub: 1, hat: 1, bass: 1, arp: 1, arpLevel: 0.14 },
  outro: { pad: 1, sub: 1, arp: 2, arpLevel: 0.1 },
};

/* PATTERN */

export function pattern(seed: number, knobs: Partial<Song> = {}, bar = 0): Pattern {
  const { key, scale, density } = { ...SONG, ...knobs };
  const rand = rng(seed);
  const steps = SCALES[scale] ?? SCALES.minor;
  const root = ROOT + Math.max(0, KEYS.indexOf(key));
  const progression = PROGRESSIONS[Math.floor(rand() * PROGRESSIONS.length)];
  const rolls = Array.from({ length: STEPS }, () => [rand(), rand(), rand(), rand(), rand()]);
  const section = Math.floor(bar / 8);
  if (section) {
    const turn = rng((seed ^ Math.imul(section, SALT)) >>> 0);
    for (let n = 0; n < 3; n++) rolls[Math.floor(turn() * STEPS)] = [turn(), turn(), turn(), turn(), turn()];
  }
  const fill = bar % 8 === 7;
  const degree = progression[bar % 4];
  const tone = (d: number) => root + 12 * Math.floor(d / 7) + steps[d % 7];
  const chord = [0, 2, 4, 6].map((k) => tone(degree + k) + 12);
  const reel = rng((seed ^ REEL) >>> 0);
  const picks = Array.from({ length: STEPS }, () => PICKS[Math.floor(reel() * PICKS.length)]);
  const skip = Array.from({ length: STEPS }, () => reel() < SKIPS);
  const octave = Array.from({ length: STEPS }, () => (reel() < LIFTS ? OCTAVE : 0));
  const out: Pattern = { kick: [], hat: [], open: [], clap: [], bass: [], chord, arp: picks, skip, octave };
  rolls.forEach(([a, b, c, d, e], s) => {
    out.kick.push(s % 4 === 0 || (s === 14 && density > 0.7 && a < 0.35));
    out.open.push(s % 4 === 2 || (fill && s === 15));
    out.hat.push(out.open[s] || (s % 2 === 1 ? b < 0.3 + 0.6 * density : s % 4 === 0 && b < 0.5 * density));
    out.clap.push(s === 4 || s === 12 || (fill && s > 12 && density > 0.3) || (s % 4 === 3 && c < 0.15 * density));
    const on = s % 4 === 2 || (s % 4 !== 0 && d < 0.15 + 0.55 * density);
    out.bass.push(
      on
        ? {
            note: tone(degree + DEGREES[Math.floor(c * DEGREES.length)]) - (degree >= 4 ? 12 : 0),
            accent: e < 0.3,
            slide: e > 0.82,
          }
        : null,
    );
  });
  return out;
}

/* PLAYER */

const section = (part: Part): Parts | undefined =>
  part.name === "outro"
    ? { ...FILM.outro, kick: part.kick ? 1 : 0, hat: part.hat ? 1 : 0, arp: part.arp ? FILM.outro.arp : 0 }
    : FILM[part.name ?? ""];

function edge(inside: (t: number) => boolean, a: number, b: number): number {
  for (let n = 0; n < BISECT; n++) {
    const m = (a + b) / 2;
    if (inside(m) === inside(a)) a = m;
    else b = m;
  }
  return b;
}

function reach(inside: (t: number) => boolean, from: number): number {
  let b = from;
  for (let n = 0; n < SCAN && inside(b); n++) b += STRIDE;
  return edge(inside, b - STRIDE, b);
}

function graph(ctx: Context, seed: number) {
  const gain = (value: number) => {
    const g = ctx.createGain();
    g.gain.value = value;
    return g;
  };
  const out = gain(0);
  const press = ctx.createDynamicsCompressor();
  press.threshold.value = -10;
  press.ratio.value = 12;
  press.attack.value = 0.003;
  press.release.value = 0.15;
  const master = gain(1);
  const gate = gain(1);
  const duck = gain(1);
  const grit = ctx.createWaveShaper();
  grit.oversample = "2x";
  const music = gain(0.7);
  const drums = gain(0.9);
  const sfx = gain(1);
  const hum = gain(1);
  const verb = ctx.createConvolver();
  verb.buffer = impulse(ctx, seed);
  const back = gain(0.5);
  const echo = gain(1);
  const wet = gain(1);
  const dub = gain(1);
  wet.connect(verb);
  dub.connect(echo);
  const delay = ctx.createDelay(2);
  const loop = gain(0.45);
  const dull = filter(ctx, "lowpass", 2800);
  grit.connect(music);
  music.connect(duck).connect(gate);
  drums.connect(gate);
  echo.connect(delay).connect(dull).connect(loop).connect(delay);
  dull.connect(gate);
  verb.connect(back).connect(master);
  hum.connect(sfx);
  gate.connect(master);
  sfx.connect(master);
  master.connect(press).connect(out).connect(ctx.destination);
  const stream = ctx.createMediaStreamDestination?.() ?? null;
  if (stream) out.connect(stream);
  return { out, gate, duck, grit, music, drums, sfx, hum, verb, echo, wet, dub, delay, stream };
}

function ramp(param: AudioParam, to: number, now: number, over: number): void {
  param.cancelScheduledValues(now);
  if (!over) {
    param.setValueAtTime(to, now);
    return;
  }
  param.setValueAtTime(param.value, now);
  param.linearRampToValueAtTime(to, now + over);
}

const named = (mood: string | Mood | null | undefined) => (typeof mood === "string" ? mood : (mood?.name ?? null));

function player(ctx: Context, seed: number) {
  const g = graph(ctx, seed);
  const base = { seed, verb: g.verb, echo: g.echo };
  let given: Partial<Song> | null = null;
  let song: Song = SONG;
  let time: Tempo = tempo(song);
  let bars = new Map<number, Pattern>();
  let mood: string | null = null;
  let want: string | null = null;
  let arranged: Arrange | null = null;
  let held: Held | null = null;
  let lit = 1;
  let scored = 1;
  const bar = (b: number) => {
    if (!bars.has(b)) bars.set(b, pattern(song.seed, song, b));
    return bars.get(b)!;
  };
  const tuneTo = (next: Partial<Song> | null | undefined) => {
    if (!next || next === given) return false;
    given = next;
    song = { ...SONG, ...next };
    time = tempo(song);
    bars = new Map<number, Pattern>();
    held = null;
    g.delay.delayTime.value = (3 * time.six) / 1000;
    g.grit.curve = curve(song.drive);
    return true;
  };
  tuneTo(SONG);
  const seal = (i: number, ms: number, when: number, inside: boolean) => {
    const prev = held;
    held = { i, ms, inside };
    if (!inside || (prev?.i === i - 1 && prev.inside)) return;
    const within = (t: number) => arranged!(t)?.name === CUT;
    const from = prev?.i === i - 1 ? edge(within, prev.ms, ms) : ms;
    const to = reach(within, ms);
    const at = (t: number) => when + (t - ms) / 1000;
    const param = g.gate.gain;
    param.setValueAtTime(1, at(from));
    param.linearRampToValueAtTime(0, at(from) + SEAL);
    param.setValueAtTime(0, at(to) - SEAL);
    param.linearRampToValueAtTime(1, at(to));
  };
  const step = (i: number, when: number, hear = true) => {
    if (arranged) mood = null;
    else if (i % STEPS === 0 || !MOODS[mood ?? ""]) mood = want;
    if (!scored || !hear) return;
    const ms = time.time(i);
    const part: Part | null = arranged ? (arranged(ms) ?? {}) : null;
    if (part) seal(i, ms, when, part.name === CUT);
    const parts = part ? section(part) : MOODS[mood ?? ""];
    if (!parts) return;
    const b = Math.floor(i / STEPS) % song.bars;
    const s = i % STEPS;
    const p = bar(b);
    const at = ((i * GOLD) % 1) * 1.6;
    const cutoff = OPEN_HZ + SWELL_HZ * (part?.swell ?? 0);
    if (parts.kick && p.kick[s]) kick(ctx, g.drums, when, { ...base, duck: g.duck.gain, at });
    if (parts.hat && p.hat[s])
      hat(ctx, g.drums, when, { ...base, open: p.open[s], level: (p.open[s] ? 0.11 : 0.06) * parts.hat, at });
    if (parts.clap && p.clap[s]) clap(ctx, g.drums, when, { ...base, space: 0.2, at });
    if (parts.roll && s % (s < 8 ? 4 : s < 12 ? 2 : 1) === 0)
      clap(ctx, g.drums, when, { ...base, level: 0.1 + 0.25 * (s / STEPS), space: 0.25, at });
    const note = p.bass[s];
    if (parts.acid && note) {
      const sweep = 0.5 + 0.5 * Math.sin((2 * Math.PI * ((b % 8) * STEPS + s)) / 128);
      const from = note.slide ? p.bass[s - 1]?.note : undefined;
      acid(ctx, g.grit, when, {
        ...base,
        note: note.note,
        from,
        accent: note.accent,
        dur: (time.six / 1000) * (note.slide ? 1 : 0.8),
        cutoff: 220 + (300 + 900 * song.drive) * sweep,
        q: 8 + 4 * song.drive,
        echoes: 0.22,
      });
    }
    if (parts.sub && s === 0)
      acid(ctx, g.music, when, {
        ...base,
        note: p.chord[0] - 24,
        wave: "sine",
        dur: (time.bar / 1000) * 0.9,
        cutoff: 220,
        q: 0,
        level: 0.35,
      });
    const [c0, c1, c2] = p.chord;
    if (parts.pad && s === 0) {
      const mix = part
        ? { notes: [c0, c1, c2, c0 + OCTAVE] }
        : { notes: p.chord, level: 0.035, attack: 0.3, release: 0.8 };
      pad(ctx, g.music, when, { ...base, ...mix, dur: time.bar / 1000 - 0.05, space: 0.35 });
    }
    if (parts.bass && p.bass[s])
      bass(ctx, g.music, when, {
        ...base,
        note: c0 - OCTAVE + p.octave[s],
        dur: (time.six / 1000) * 0.85,
        cutoff: cutoff * BASS_SHARE,
      });
    if (parts.arp && s % parts.arp === 0 && !p.skip[s])
      arp(ctx, g.music, when, {
        ...base,
        note: p.chord[p.arp[s]] + OCTAVE + (s % 8 === 7 ? OCTAVE : 0),
        cutoff,
        level: parts.arpLevel ?? ARP_LEVEL,
      });
  };
  const cue = (name: string, when: number, args: Args = {}) => {
    const a = { length: 1, ...base, verb: g.wet, echo: g.dub, ...args };
    const effect = EFFECTS[name];
    const voice = VOICES[name];
    if (!effect && !voice) return;
    let out: AudioNode = name === "drone" || name === "tunnel" ? g.hum : g.sfx;
    if ((a.drive ?? 0) > 0) {
      const shaper = ctx.createWaveShaper();
      shaper.curve = curve(a.drive!);
      shaper.oversample = "2x";
      shaper.connect(out);
      out = shaper;
    }
    if (effect) effect.play(ctx, out, when, { notes: bar(0).chord, ...a });
    else voice(ctx, out, when, a);
  };
  const feel = (m: Mood | undefined, now: number) => {
    want = named(m);
    arranged = typeof m?.arrange === "function" ? m.arrange : null;
    if (!Number.isFinite(m?.flicker)) return;
    const target = 1 + 0.35 * m!.flicker!;
    if (Math.abs(target - lit) <= 0.01) return;
    lit = target;
    g.hum.gain.setTargetAtTime(target, now, 0.03);
  };
  const level = (sound: number, music: number, now: number, over: boolean) => {
    scored = music ? 1 : 0;
    ramp(g.out.gain, sound || music ? LOUD : 0, now, over ? MUTE : 0);
    ramp(g.gate.gain, music ? 1 : 0, now, over ? GATE : 0);
    for (const one of [g.sfx, g.wet, g.dub]) ramp(one.gain, sound ? 1 : 0, now, over ? MUTE : 0);
  };
  return { g, step, cue, feel, level, tune: tuneTo, time: () => time, song: () => song };
}

/* ENGINE */

const live = () => new (globalThis.AudioContext ?? globalThis.webkitAudioContext)({ latencyHint: "interactive" });

const offline = (channels: number, length: number, rate: number) => new OfflineAudioContext(channels, length, rate);

export function engine({ seed = 1, make = live }: Options = {}): Engine {
  let ctx: AudioContext | null = null;
  let p: Player | null = null;
  let anchor: Anchor | null = null;
  let last = -Infinity;
  let next = 0;
  let sound = 1;
  let music = 1;
  let gone = false;
  const map = (t: number) => anchor!.c + (t - anchor!.t) / 1000;
  const hidden = () => {
    if (!ctx) return;
    if (document.hidden) ctx.suspend?.();
    else ctx.resume?.();
  };
  const start = () => {
    if (gone) return;
    if (!ctx) {
      ctx = make();
      p = player(ctx, seed);
      p.level(sound, music, ctx.currentTime, false);
      if (typeof document !== "undefined") document.addEventListener("visibilitychange", hidden);
      try {
        if (globalThis.navigator?.audioSession) globalThis.navigator.audioSession.type = "playback";
      } catch {}
    }
    if (ctx.state === "suspended") ctx.resume?.();
  };
  const set = (value: Levels = {}) => {
    const was = [sound, music];
    if ("sound" in value) sound = value.sound ? 1 : 0;
    if ("music" in value) music = value.music ? 1 : 0;
    if (p && (sound !== was[0] || music !== was[1])) p.level(sound, music, ctx!.currentTime, true);
  };
  const at = (t: number, mood?: Mood) => {
    if (!p || t === last) return;
    const now = ctx!.currentTime;
    if (p.tune(mood?.song)) {
      anchor = null;
      next = 0;
    }
    const back = t < last;
    if (!anchor || back || Math.abs(map(t) - (now + LEAD)) > DRIFT) {
      anchor = { t, c: now + LEAD };
      const from = p.time().first(t);
      next = back ? from : Math.max(next, from);
    }
    last = t;
    p.feel(mood, now);
    const time = p.time();
    for (let when = map(time.time(next)); when <= now + AHEAD; when = map(time.time(next)))
      p.step(next++, when, when >= now);
  };
  const cue = (name: string, t: number, args?: Args) => {
    if (!p) return;
    const now = ctx!.currentTime;
    if (!anchor) anchor = { t, c: now + LEAD };
    p.cue(name, Math.max(map(t), now + 0.01), args);
  };
  const stop = () => {
    gone = true;
    if (typeof document !== "undefined") document.removeEventListener("visibilitychange", hidden);
    ctx?.close?.();
    ctx = null;
    p = null;
  };
  return { start, set, at, cue, stream: () => p?.g.stream?.stream ?? null, stop };
}

/* RENDER */

export async function render(seconds: number, score: Score = {}, make: Offline = offline): Promise<AudioBuffer> {
  const ctx = make(2, Math.max(1, Math.ceil(seconds * RATE)), RATE);
  const p = player(ctx, score.seed ?? 1);
  p.level(1, 1, 0, false);
  if (score.mood) {
    p.tune(score.mood.song);
    p.feel(score.mood, 0);
    const time = p.time();
    const count = p.song().bars * STEPS;
    for (let i = 0; i < count && time.time(i) < seconds * 1000; i++) p.step(i, time.time(i) / 1000);
  }
  for (const [name, t, args] of score.cues ?? []) p.cue(name, t / 1000, args);
  return ctx.startRendering();
}
