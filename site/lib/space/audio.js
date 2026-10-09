import { rng } from '../scene.js';
import { KEYS, SONG, STEPS, tempo } from './tempo.js';

export { KEYS, SONG, bus, tempo } from './tempo.js';

/* NUMBERS */

const RATE = 44100;
const FLOOR = 0.0001;
const LEAD = 0.05;
const DRIFT = 0.08;
const AHEAD = 0.12;
const DIP = 0.3;
const MUTE = 0.3;
const GATE = 0.08;
const LOUD = 0.9;
const ROOT = 36;
const SALT = 0x9e3779b9;
const GOLD = 0.6180339887;

const SCALES = { minor: [0, 2, 3, 5, 7, 8, 10], dorian: [0, 2, 3, 5, 7, 9, 10], phrygian: [0, 1, 3, 5, 7, 8, 10] };

const PROGRESSIONS = [[0, 5, 3, 4], [0, 3, 5, 4], [0, 6, 5, 3], [0, 2, 5, 4], [5, 3, 0, 4], [0, 0, 5, 6]];

const DEGREES = [0, 0, 0, 0, 7, 2, 4, 5, 7, 0];

const MOODS = {
  drift: { pad: 1, sub: 1, hat: 0.5 },
  build: { pad: 1, kick: 1, hat: 1, roll: 1 },
  drive: { kick: 1, hat: 1, clap: 1, acid: 1, pad: 1 },
  drop: {},
  land: { pad: 1, sub: 1 },
  acid: { kick: 1, hat: 1, clap: 1, acid: 1 },
};

/* NOISE */

const NOISES = new WeakMap();

export function noise(ctx, seed = 1) {
  let kept = NOISES.get(ctx);
  if (!kept) NOISES.set(ctx, (kept = new Map()));
  if (!kept.has(seed)) {
    const rand = rng(seed);
    const buffer = ctx.createBuffer(1, ctx.sampleRate * 2, ctx.sampleRate);
    const data = buffer.getChannelData(0);
    for (let i = 0; i < data.length; i++) data[i] = rand() * 2 - 1;
    kept.set(seed, buffer);
  }
  return kept.get(seed);
}

function impulse(ctx, seed, seconds = 3.2, decay = 2.2) {
  const n = Math.floor(ctx.sampleRate * seconds);
  const buffer = ctx.createBuffer(2, n, ctx.sampleRate);
  for (let c = 0; c < 2; c++) {
    const rand = rng((seed ^ Math.imul(c + 1, SALT)) >>> 0);
    const data = buffer.getChannelData(c);
    for (let i = 0; i < n; i++) data[i] = (rand() * 2 - 1) * (1 - i / n) ** decay;
  }
  return buffer;
}

function curve(drive) {
  if (!(drive > 0)) return null;
  const k = 1 + drive * 12;
  const top = Math.tanh(k);
  const out = new Float32Array(1024);
  for (let i = 0; i < out.length; i++) out[i] = Math.tanh(k * ((i / (out.length - 1)) * 2 - 1)) / top;
  return out;
}

/* SHAPES */

const hz = (note) => 440 * 2 ** ((note - 69) / 12);

const tune = (args) => 2 ** ((args.pitch ?? 0) / 12);

const shine = (args) => 2 ** (((args.bright ?? 0.5) - 0.5) * 3);

function env(param, when, peak, attack, hold, release) {
  param.setValueAtTime(FLOOR, when);
  param.linearRampToValueAtTime(peak, when + attack);
  if (hold > 0) param.setValueAtTime(peak, when + attack + hold);
  param.exponentialRampToValueAtTime(FLOOR, when + attack + hold + release);
  return when + attack + hold + release;
}

function rise(param, when, peak, dur) {
  param.setValueAtTime(FLOOR, when);
  param.exponentialRampToValueAtTime(peak, when + dur);
  param.linearRampToValueAtTime(FLOOR, when + dur + 0.03);
  return when + dur + 0.03;
}

function filter(ctx, type, frequency, q) {
  const f = ctx.createBiquadFilter();
  f.type = type;
  f.frequency.value = frequency;
  if (q !== undefined) f.Q.value = q;
  return f;
}

function source(ctx, args, loop = false) {
  const s = ctx.createBufferSource();
  s.buffer = noise(ctx, args.seed);
  s.loop = loop;
  return s;
}

function osc(ctx, type, frequency, detune = 0) {
  const o = ctx.createOscillator();
  o.type = type;
  o.frequency.value = frequency;
  if (detune) o.detune.value = detune;
  return o;
}

function sends(ctx, node, args) {
  for (const [to, amount] of [[args.verb, args.space], [args.echo, args.echoes]]) {
    if (!to || !(amount > 0)) continue;
    const g = ctx.createGain();
    g.gain.value = amount;
    node.connect(g);
    g.connect(to);
  }
}

function dip(param, when) {
  param.cancelScheduledValues(when);
  param.setValueAtTime(1, when);
  param.linearRampToValueAtTime(DIP, when + 0.01);
  param.linearRampToValueAtTime(1, when + 0.15);
}

/* VOICES */

export function kick(ctx, out, when, args = {}) {
  const level = args.level ?? 1;
  const k = tune(args);
  const body = osc(ctx, 'sine', 150 * k);
  body.frequency.setValueAtTime(150 * k, when);
  body.frequency.exponentialRampToValueAtTime(45 * k, when + 0.06);
  const amp = ctx.createGain();
  const end = env(amp.gain, when, level, 0.002, 0.01, 0.4);
  body.connect(amp).connect(out);
  body.start(when);
  body.stop(end + 0.02);
  const click = source(ctx, args);
  const snap = ctx.createGain();
  env(snap.gain, when, 0.35 * level, 0.001, 0, 0.012);
  click.connect(filter(ctx, 'highpass', 3000 * shine(args))).connect(snap).connect(out);
  click.start(when, args.at ?? 0);
  click.stop(when + 0.03);
  if (args.duck) dip(args.duck, when);
}

export function hat(ctx, out, when, args = {}) {
  const decay = args.open ? 0.25 : 0.04;
  const s = source(ctx, args);
  const amp = ctx.createGain();
  const end = env(amp.gain, when, args.level ?? 0.12, 0.001, 0, decay);
  s.connect(filter(ctx, 'highpass', 7000 * shine(args))).connect(amp).connect(out);
  s.start(when, args.at ?? 0);
  s.stop(end + 0.02);
}

export function clap(ctx, out, when, args = {}) {
  const level = args.level ?? 0.35;
  const s = source(ctx, args);
  const amp = ctx.createGain();
  const g = amp.gain;
  g.setValueAtTime(FLOOR, when);
  for (const at of [0, 0.011]) {
    g.linearRampToValueAtTime(level, when + at + 0.001);
    g.exponentialRampToValueAtTime(level * 0.1, when + at + 0.01);
  }
  g.linearRampToValueAtTime(level, when + 0.023);
  g.exponentialRampToValueAtTime(FLOOR, when + 0.2);
  s.connect(filter(ctx, 'bandpass', 1500 * shine(args), 1.5)).connect(amp).connect(out);
  sends(ctx, amp, args);
  s.start(when, args.at ?? 0);
  s.stop(when + 0.22);
}

export function acid(ctx, out, when, args = {}) {
  const k = tune(args);
  const dur = args.dur ?? 0.12;
  const accent = args.accent ? 1 : 0;
  const cutoff = (args.cutoff ?? 400) * shine(args) * (accent ? 1.6 : 1);
  const level = (args.level ?? 0.2) * (accent ? Math.SQRT2 : 1);
  const to = hz(args.note ?? 45) * k;
  const o = osc(ctx, args.wave ?? 'sawtooth', to);
  if (args.from !== undefined) {
    o.frequency.setValueAtTime(hz(args.from) * k, when);
    o.frequency.exponentialRampToValueAtTime(to, when + 0.06);
  } else o.frequency.setValueAtTime(to, when);
  const low = [filter(ctx, 'lowpass', cutoff, args.q ?? 10), filter(ctx, 'lowpass', cutoff, args.q ?? 10)];
  for (const f of low) {
    f.frequency.setValueAtTime(cutoff * (accent ? 6 : 4), when);
    f.frequency.exponentialRampToValueAtTime(cutoff, when + Math.max(0.05, dur));
  }
  const amp = ctx.createGain();
  const end = env(amp.gain, when, level, 0.003, dur * 0.6, dur * 0.4 + 0.03);
  o.connect(low[0]).connect(low[1]).connect(amp).connect(out);
  sends(ctx, amp, args);
  o.start(when);
  o.stop(end + 0.02);
}

export function pad(ctx, out, when, args = {}) {
  const k = tune(args);
  const lit = shine(args);
  const dur = args.dur ?? 4;
  const level = args.level ?? 0.05;
  const attack = args.attack ?? Math.min(0.9, dur * 0.3);
  const release = args.release ?? 1.2;
  const f = filter(ctx, 'lowpass', 700 * lit);
  f.frequency.setValueAtTime(700 * lit, when);
  f.frequency.linearRampToValueAtTime(1600 * lit, when + dur * 0.6);
  f.frequency.linearRampToValueAtTime(600 * lit, when + dur + release);
  const amp = ctx.createGain();
  amp.gain.setValueAtTime(FLOOR, when);
  amp.gain.linearRampToValueAtTime(level, when + attack);
  amp.gain.setValueAtTime(level, when + dur);
  amp.gain.linearRampToValueAtTime(FLOOR, when + dur + release);
  for (const note of args.notes ?? [57, 60, 64, 67]) {
    for (const cents of [-9, 9]) {
      const o = osc(ctx, 'sawtooth', hz(note) * k, cents);
      o.connect(f);
      o.start(when);
      o.stop(when + dur + release + 0.05);
    }
  }
  f.connect(amp).connect(out);
  sends(ctx, amp, args);
}

export function riser(ctx, out, when, args = {}) {
  const k = tune(args);
  const lit = shine(args);
  const dur = args.dur ?? 1.8;
  const level = args.level ?? 0.4;
  const s = source(ctx, args, true);
  const band = filter(ctx, 'bandpass', 200, 1.5);
  band.frequency.setValueAtTime(200, when);
  band.frequency.exponentialRampToValueAtTime(5000 * lit, when + dur);
  const air = ctx.createGain();
  const end = rise(air.gain, when, level, dur);
  s.connect(band).connect(air).connect(out);
  sends(ctx, air, args);
  s.start(when);
  s.stop(end + 0.02);
  const low = filter(ctx, 'lowpass', 300, 4);
  low.frequency.setValueAtTime(300, when);
  low.frequency.exponentialRampToValueAtTime(6000 * lit, when + dur);
  const whine = ctx.createGain();
  rise(whine.gain, when, level * 0.35, dur);
  for (const cents of [-14, 0, 14]) {
    const o = osc(ctx, 'sawtooth', hz(43) * k, cents);
    o.frequency.setValueAtTime(hz(43) * k, when);
    o.frequency.exponentialRampToValueAtTime(hz(67) * k, when + dur);
    o.connect(low);
    o.start(when);
    o.stop(end + 0.02);
  }
  low.connect(whine).connect(out);
}

export function drone(ctx, out, when, args = {}) {
  const k = tune(args);
  const lit = shine(args);
  const dur = Math.max(1, args.dur ?? 4);
  const level = args.level ?? 0.3;
  const end = when + dur;
  const low = filter(ctx, 'lowpass', 380 * lit, 4);
  const wobble = osc(ctx, 'sine', 0.23);
  const depth = ctx.createGain();
  depth.gain.value = 220 * lit;
  wobble.connect(depth).connect(low.frequency);
  const amp = ctx.createGain();
  amp.gain.setValueAtTime(FLOOR, when);
  amp.gain.linearRampToValueAtTime(level, when + 0.4);
  amp.gain.setValueAtTime(level, end - 0.6);
  amp.gain.linearRampToValueAtTime(FLOOR, end);
  const saws = [-8, 0, 8].map((cents) => osc(ctx, 'sawtooth', 55 * k, cents));
  for (const o of saws) o.connect(low);
  const s = source(ctx, args, true);
  const band = filter(ctx, 'bandpass', 900 * lit, 0.8);
  const drift = osc(ctx, 'sine', 0.11);
  const sway = ctx.createGain();
  sway.gain.value = 500 * lit;
  drift.connect(sway).connect(band.frequency);
  const wind = ctx.createGain();
  wind.gain.value = 0.35;
  s.connect(band).connect(wind).connect(amp);
  low.connect(amp).connect(out);
  sends(ctx, amp, args);
  for (const one of [...saws, wobble, drift, s]) {
    one.start(when);
    one.stop(end + 0.05);
  }
}

export function hit(ctx, out, when, args = {}) {
  const k = tune(args);
  const level = args.level ?? 0.9;
  const white = args.white ? 1 : 0;
  const sub = osc(ctx, 'sine', 60 * k);
  sub.frequency.setValueAtTime(60 * k, when);
  sub.frequency.exponentialRampToValueAtTime(30 * k, when + 0.6);
  const body = ctx.createGain();
  const end = env(body.gain, when, level, 0.003, 0.05, 0.55);
  sub.connect(body).connect(out);
  sub.start(when);
  sub.stop(end + 0.02);
  const s = source(ctx, args);
  const low = filter(ctx, 'lowpass', 8000 * shine(args));
  low.frequency.setValueAtTime(8000 * shine(args), when);
  low.frequency.exponentialRampToValueAtTime(300, when + 0.3 + 0.4 * white);
  const crack = ctx.createGain();
  const tail = env(crack.gain, when, level * (0.6 + 0.2 * white), 0.001, 0.01, 0.35 + 0.45 * white);
  s.connect(low).connect(crack).connect(out);
  sends(ctx, crack, { ...args, space: Math.max(args.space ?? 0, 0.3) });
  s.start(when, args.at ?? 0);
  s.stop(tail + 0.02);
}

export function swell(ctx, out, when, args = {}) {
  const dur = args.dur ?? 0.4;
  const s = source(ctx, args, true);
  const band = filter(ctx, 'bandpass', 5000, 2);
  band.frequency.setValueAtTime(5000 * shine(args), when);
  band.frequency.exponentialRampToValueAtTime(200, when + dur);
  const amp = ctx.createGain();
  const end = rise(amp.gain, when, args.level ?? 0.5, dur);
  s.connect(band).connect(amp).connect(out);
  sends(ctx, amp, args);
  s.start(when);
  s.stop(end + 0.02);
}

export function tick(ctx, out, when, args = {}) {
  const o = osc(ctx, 'sine', 2400 * tune(args));
  const amp = ctx.createGain();
  env(amp.gain, when, args.level ?? 0.3, 0.001, 0, 0.007);
  o.connect(amp).connect(out);
  o.start(when);
  o.stop(when + 0.01);
}

const VOICES = { kick, hat, clap, acid, pad, riser, drone, hit, swell, tick };

/* EFFECTS */

export const EFFECTS = {
  jump: {
    span: (n) => 1.8 * n + 0.8,
    play: (ctx, out, when, a) => {
      riser(ctx, out, when, { ...a, dur: 1.8 * a.length });
      hit(ctx, out, when + 1.8 * a.length, a);
    },
  },
  tunnel: { span: (n) => 4 * n, play: (ctx, out, when, a) => drone(ctx, out, when, { ...a, dur: 4 * a.length }) },
  exit: {
    span: (n) => 0.8 * n + 1.2,
    play: (ctx, out, when, a) => {
      swell(ctx, out, when, { ...a, dur: 0.8 * a.length });
      hit(ctx, out, when + 0.8 * a.length, { ...a, white: true });
    },
  },
  hit: { span: () => 0.8, play: hit },
  tick: { span: () => 0.02, play: tick },
  arrive: { span: (n) => 4 * n + 1.2, play: (ctx, out, when, a) => pad(ctx, out, when, { level: 0.09, ...a, dur: 4 * a.length }) },
};

/* PATTERN */

export function pattern(seed, knobs = {}, bar = 0) {
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
  const tone = (d) => root + 12 * Math.floor(d / 7) + steps[d % 7];
  const chord = [0, 2, 4, 6].map((k) => tone(degree + k) + 12);
  const out = { kick: [], hat: [], open: [], clap: [], bass: [], chord };
  rolls.forEach(([a, b, c, d, e], s) => {
    out.kick.push(s % 4 === 0 || (s === 14 && density > 0.7 && a < 0.35));
    out.open.push(s % 4 === 2 || (fill && s === 15));
    out.hat.push(out.open[s] || (s % 2 === 1 ? b < 0.3 + 0.6 * density : s % 4 === 0 && b < 0.5 * density));
    out.clap.push(s === 4 || s === 12 || (fill && s > 12 && density > 0.3) || (s % 4 === 3 && c < 0.15 * density));
    const on = s % 4 === 2 || (s % 4 !== 0 && d < 0.15 + 0.55 * density);
    out.bass.push(on ? { note: tone(degree + DEGREES[Math.floor(c * DEGREES.length)]) - (degree >= 4 ? 12 : 0), accent: e < 0.3, slide: e > 0.82 } : null);
  });
  return out;
}

/* PLAYER */

function graph(ctx, seed) {
  const gain = (value) => {
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
  grit.oversample = '2x';
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
  const dull = filter(ctx, 'lowpass', 2800);
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

function ramp(param, to, now, over) {
  param.cancelScheduledValues(now);
  if (!over) {
    param.setValueAtTime(to, now);
    return;
  }
  param.setValueAtTime(param.value, now);
  param.linearRampToValueAtTime(to, now + over);
}

const named = (mood) => (typeof mood === 'string' ? mood : (mood?.name ?? null));

function player(ctx, seed) {
  const g = graph(ctx, seed);
  const base = { seed, verb: g.verb, echo: g.echo };
  let given = null;
  let song = SONG;
  let time = tempo(song);
  let bars = new Map();
  let mood = null;
  let want = null;
  let lit = 1;
  let scored = 1;
  const bar = (b) => {
    if (!bars.has(b)) bars.set(b, pattern(song.seed, song, b));
    return bars.get(b);
  };
  const tuneTo = (next) => {
    if (!next || next === given) return false;
    given = next;
    song = { ...SONG, ...next };
    time = tempo(song);
    bars = new Map();
    g.delay.delayTime.value = (3 * time.six) / 1000;
    g.grit.curve = curve(song.drive);
    return true;
  };
  tuneTo(SONG);
  const step = (i, when, hear = true) => {
    if (i % STEPS === 0 || !MOODS[mood]) mood = want;
    if (!scored) return;
    const parts = MOODS[mood];
    if (!parts || !hear) return;
    const b = Math.floor(i / STEPS) % song.bars;
    const s = i % STEPS;
    const p = bar(b);
    const at = ((i * GOLD) % 1) * 1.6;
    if (parts.kick && p.kick[s]) kick(ctx, g.drums, when, { ...base, duck: g.duck.gain, at });
    if (parts.hat && p.hat[s]) hat(ctx, g.drums, when, { ...base, open: p.open[s], level: (p.open[s] ? 0.11 : 0.06) * parts.hat, at });
    if (parts.clap && p.clap[s]) clap(ctx, g.drums, when, { ...base, space: 0.2, at });
    if (parts.roll && s % (s < 8 ? 4 : s < 12 ? 2 : 1) === 0) clap(ctx, g.drums, when, { ...base, level: 0.1 + 0.25 * (s / STEPS), space: 0.25, at });
    const note = p.bass[s];
    if (parts.acid && note) {
      const sweep = 0.5 + 0.5 * Math.sin((2 * Math.PI * ((b % 8) * STEPS + s)) / 128);
      const from = note.slide ? p.bass[s - 1]?.note : undefined;
      acid(ctx, g.grit, when, { ...base, note: note.note, from, accent: note.accent, dur: (time.six / 1000) * (note.slide ? 1 : 0.8), cutoff: 220 + (300 + 900 * song.drive) * sweep, q: 8 + 4 * song.drive, echoes: 0.22 });
    }
    if (parts.sub && s === 0) acid(ctx, g.music, when, { ...base, note: p.chord[0] - 24, wave: 'sine', dur: (time.bar / 1000) * 0.9, cutoff: 220, q: 0, level: 0.35 });
    if (parts.pad && s === 0) pad(ctx, g.music, when, { ...base, notes: p.chord, dur: time.bar / 1000 - 0.05, level: 0.035, attack: 0.3, release: 0.8, space: 0.35 });
  };
  const cue = (name, when, args = {}) => {
    const a = { length: 1, ...base, verb: g.wet, echo: g.dub, ...args };
    const effect = EFFECTS[name];
    const voice = VOICES[name];
    if (!effect && !voice) return;
    let out = name === 'drone' || name === 'tunnel' ? g.hum : g.sfx;
    if (a.drive > 0) {
      const shaper = ctx.createWaveShaper();
      shaper.curve = curve(a.drive);
      shaper.oversample = '2x';
      shaper.connect(out);
      out = shaper;
    }
    if (effect) effect.play(ctx, out, when, { notes: bar(0).chord, ...a });
    else voice(ctx, out, when, a);
  };
  const feel = (m, now) => {
    want = named(m);
    if (!Number.isFinite(m?.flicker)) return;
    const target = 1 + 0.35 * m.flicker;
    if (Math.abs(target - lit) <= 0.01) return;
    lit = target;
    g.hum.gain.setTargetAtTime(target, now, 0.03);
  };
  const level = (sound, music, now, over) => {
    scored = music ? 1 : 0;
    ramp(g.out.gain, sound || music ? LOUD : 0, now, over ? MUTE : 0);
    ramp(g.gate.gain, music ? 1 : 0, now, over ? GATE : 0);
    for (const one of [g.sfx, g.wet, g.dub]) ramp(one.gain, sound ? 1 : 0, now, over ? MUTE : 0);
  };
  return { g, step, cue, feel, level, tune: tuneTo, time: () => time, song: () => song };
}

/* ENGINE */

const live = () => new (globalThis.AudioContext ?? globalThis.webkitAudioContext)({ latencyHint: 'interactive' });

const offline = (channels, length, rate) => new OfflineAudioContext(channels, length, rate);

export function engine({ seed = 1, make = live } = {}) {
  let ctx = null;
  let p = null;
  let anchor = null;
  let last = -Infinity;
  let next = 0;
  let sound = 1;
  let music = 1;
  let gone = false;
  const map = (t) => anchor.c + (t - anchor.t) / 1000;
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
      if (typeof document !== 'undefined') document.addEventListener('visibilitychange', hidden);
      try {
        if (globalThis.navigator?.audioSession) globalThis.navigator.audioSession.type = 'playback';
      } catch {}
    }
    if (ctx.state === 'suspended') ctx.resume?.();
  };
  const set = (value = {}) => {
    const was = [sound, music];
    if ('sound' in value) sound = value.sound ? 1 : 0;
    if ('music' in value) music = value.music ? 1 : 0;
    if (p && (sound !== was[0] || music !== was[1])) p.level(sound, music, ctx.currentTime, true);
  };
  const at = (t, mood) => {
    if (!p || t === last) return;
    const now = ctx.currentTime;
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
    for (let when = map(time.time(next)); when <= now + AHEAD; when = map(time.time(next))) p.step(next++, when, when >= now);
  };
  const cue = (name, t, args) => {
    if (!p) return;
    const now = ctx.currentTime;
    if (!anchor) anchor = { t, c: now + LEAD };
    p.cue(name, Math.max(map(t), now + 0.01), args);
  };
  const stop = () => {
    gone = true;
    if (typeof document !== 'undefined') document.removeEventListener('visibilitychange', hidden);
    ctx?.close?.();
    ctx = null;
    p = null;
  };
  return { start, set, at, cue, stream: () => p?.g.stream?.stream ?? null, stop };
}

/* FILES */

export async function render(seconds, score = {}, make = offline) {
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

export function wav(buffer) {
  const channels = buffer.numberOfChannels;
  const frames = buffer.length;
  const rate = buffer.sampleRate;
  const size = frames * channels * 2;
  const bytes = new ArrayBuffer(44 + size);
  const view = new DataView(bytes);
  const word = (at, text) => [...text].forEach((c, i) => view.setUint8(at + i, c.charCodeAt(0)));
  word(0, 'RIFF');
  view.setUint32(4, 36 + size, true);
  word(8, 'WAVE');
  word(12, 'fmt ');
  view.setUint32(16, 16, true);
  view.setUint16(20, 1, true);
  view.setUint16(22, channels, true);
  view.setUint32(24, rate, true);
  view.setUint32(28, rate * channels * 2, true);
  view.setUint16(32, channels * 2, true);
  view.setUint16(34, 16, true);
  word(36, 'data');
  view.setUint32(40, size, true);
  const pcm = new Int16Array(bytes, 44);
  const data = Array.from({ length: channels }, (_, c) => buffer.getChannelData(c));
  for (let i = 0, at = 0; i < frames; i++) {
    for (let c = 0; c < channels; c++) pcm[at++] = Math.max(-32768, Math.min(32767, Math.round(data[c][i] * 32767)));
  }
  return new Blob([bytes], { type: 'audio/wav' });
}
