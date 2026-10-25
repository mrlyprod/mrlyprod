import { rng } from "../rand.ts";

/* TYPES */

export type Args = {
  seed?: number;
  pitch?: number;
  bright?: number;
  level?: number;
  dur?: number;
  at?: number;
  duck?: AudioParam;
  verb?: AudioNode;
  echo?: AudioNode;
  space?: number;
  echoes?: number;
  open?: boolean;
  note?: number;
  from?: number;
  accent?: boolean;
  cutoff?: number;
  q?: number;
  wave?: OscillatorType;
  notes?: number[];
  attack?: number;
  release?: number;
  white?: boolean;
  length?: number;
  drive?: number;
};

export type Cue = Args & { length: number };

export type Voice = (ctx: BaseAudioContext, out: AudioNode, when: number, args?: Args) => void;

export type Effect = {
  span: (n: number) => number;
  play: (ctx: BaseAudioContext, out: AudioNode, when: number, a: Cue) => void;
};

/* NUMBERS */

const FLOOR = 0.0001;
const DIP = 0.3;
export const SALT = 0x9e3779b9;
export const ARP_LEVEL = 0.16;
const ARP_SPACE = 0.35;
const ARP_ECHO = 0.3;

export const CUT = "cut";

/* NOISE */

const NOISES = new WeakMap<BaseAudioContext, Map<number, AudioBuffer>>();

export function noise(ctx: BaseAudioContext, seed = 1): AudioBuffer {
  let kept = NOISES.get(ctx);
  if (!kept) {
    kept = new Map();
    NOISES.set(ctx, kept);
  }
  if (!kept.has(seed)) {
    const rand = rng(seed);
    const buffer = ctx.createBuffer(1, ctx.sampleRate * 2, ctx.sampleRate);
    const data = buffer.getChannelData(0);
    for (let i = 0; i < data.length; i++) data[i] = rand() * 2 - 1;
    kept.set(seed, buffer);
  }
  return kept.get(seed)!;
}

export function impulse(ctx: BaseAudioContext, seed: number, seconds = 3.2, decay = 2.2): AudioBuffer {
  const n = Math.floor(ctx.sampleRate * seconds);
  const buffer = ctx.createBuffer(2, n, ctx.sampleRate);
  for (let c = 0; c < 2; c++) {
    const rand = rng((seed ^ Math.imul(c + 1, SALT)) >>> 0);
    const data = buffer.getChannelData(c);
    for (let i = 0; i < n; i++) data[i] = (rand() * 2 - 1) * (1 - i / n) ** decay;
  }
  return buffer;
}

export function curve(drive: number): Float32Array<ArrayBuffer> | null {
  if (!(drive > 0)) return null;
  const k = 1 + drive * 12;
  const top = Math.tanh(k);
  const out = new Float32Array(1024);
  for (let i = 0; i < out.length; i++) out[i] = Math.tanh(k * ((i / (out.length - 1)) * 2 - 1)) / top;
  return out;
}

/* SHAPES */

const hz = (note: number) => 440 * 2 ** ((note - 69) / 12);

const tune = (args: Args) => 2 ** ((args.pitch ?? 0) / 12);

const shine = (args: Args) => 2 ** (((args.bright ?? 0.5) - 0.5) * 3);

function env(param: AudioParam, when: number, peak: number, attack: number, hold: number, release: number): number {
  param.setValueAtTime(FLOOR, when);
  param.linearRampToValueAtTime(peak, when + attack);
  if (hold > 0) param.setValueAtTime(peak, when + attack + hold);
  param.exponentialRampToValueAtTime(FLOOR, when + attack + hold + release);
  return when + attack + hold + release;
}

function rise(param: AudioParam, when: number, peak: number, dur: number): number {
  param.setValueAtTime(FLOOR, when);
  param.exponentialRampToValueAtTime(peak, when + dur);
  param.linearRampToValueAtTime(FLOOR, when + dur + 0.03);
  return when + dur + 0.03;
}

export function filter(ctx: BaseAudioContext, type: BiquadFilterType, frequency: number, q?: number): BiquadFilterNode {
  const f = ctx.createBiquadFilter();
  f.type = type;
  f.frequency.value = frequency;
  if (q !== undefined) f.Q.value = q;
  return f;
}

function source(ctx: BaseAudioContext, args: Args, loop = false): AudioBufferSourceNode {
  const s = ctx.createBufferSource();
  s.buffer = noise(ctx, args.seed);
  s.loop = loop;
  return s;
}

function osc(ctx: BaseAudioContext, type: OscillatorType, frequency: number, detune = 0): OscillatorNode {
  const o = ctx.createOscillator();
  o.type = type;
  o.frequency.value = frequency;
  if (detune) o.detune.value = detune;
  return o;
}

function sends(ctx: BaseAudioContext, node: AudioNode, args: Args): void {
  for (const [to, amount] of [
    [args.verb, args.space],
    [args.echo, args.echoes],
  ] as const) {
    if (!to || !((amount ?? 0) > 0)) continue;
    const g = ctx.createGain();
    g.gain.value = amount!;
    node.connect(g);
    g.connect(to);
  }
}

function dip(param: AudioParam, when: number): void {
  param.cancelScheduledValues(when);
  param.setValueAtTime(1, when);
  param.linearRampToValueAtTime(DIP, when + 0.01);
  param.linearRampToValueAtTime(1, when + 0.15);
}

/* VOICES */

export function kick(ctx: BaseAudioContext, out: AudioNode, when: number, args: Args = {}): void {
  const level = args.level ?? 1;
  const k = tune(args);
  const body = osc(ctx, "sine", 150 * k);
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
  click
    .connect(filter(ctx, "highpass", 3000 * shine(args)))
    .connect(snap)
    .connect(out);
  click.start(when, args.at ?? 0);
  click.stop(when + 0.03);
  if (args.duck) dip(args.duck, when);
}

export function hat(ctx: BaseAudioContext, out: AudioNode, when: number, args: Args = {}): void {
  const decay = args.open ? 0.25 : 0.04;
  const s = source(ctx, args);
  const amp = ctx.createGain();
  const end = env(amp.gain, when, args.level ?? 0.12, 0.001, 0, decay);
  s.connect(filter(ctx, "highpass", 7000 * shine(args)))
    .connect(amp)
    .connect(out);
  s.start(when, args.at ?? 0);
  s.stop(end + 0.02);
}

export function clap(ctx: BaseAudioContext, out: AudioNode, when: number, args: Args = {}): void {
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
  s.connect(filter(ctx, "bandpass", 1500 * shine(args), 1.5))
    .connect(amp)
    .connect(out);
  sends(ctx, amp, args);
  s.start(when, args.at ?? 0);
  s.stop(when + 0.22);
}

export function acid(ctx: BaseAudioContext, out: AudioNode, when: number, args: Args = {}): void {
  const k = tune(args);
  const dur = args.dur ?? 0.12;
  const accent = args.accent ? 1 : 0;
  const cutoff = (args.cutoff ?? 400) * shine(args) * (accent ? 1.6 : 1);
  const level = (args.level ?? 0.2) * (accent ? Math.SQRT2 : 1);
  const to = hz(args.note ?? 45) * k;
  const o = osc(ctx, args.wave ?? "sawtooth", to);
  if (args.from !== undefined) {
    o.frequency.setValueAtTime(hz(args.from) * k, when);
    o.frequency.exponentialRampToValueAtTime(to, when + 0.06);
  } else o.frequency.setValueAtTime(to, when);
  const low = [filter(ctx, "lowpass", cutoff, args.q ?? 10), filter(ctx, "lowpass", cutoff, args.q ?? 10)];
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

export function pad(ctx: BaseAudioContext, out: AudioNode, when: number, args: Args = {}): void {
  const k = tune(args);
  const lit = shine(args);
  const dur = args.dur ?? 4;
  const level = args.level ?? 0.05;
  const attack = args.attack ?? Math.min(0.9, dur * 0.3);
  const release = args.release ?? 1.2;
  const f = filter(ctx, "lowpass", 700 * lit);
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
      const o = osc(ctx, "sawtooth", hz(note) * k, cents);
      o.connect(f);
      o.start(when);
      o.stop(when + dur + release + 0.05);
    }
  }
  f.connect(amp).connect(out);
  sends(ctx, amp, args);
}

export function arp(ctx: BaseAudioContext, out: AudioNode, when: number, args: Args = {}): void {
  const k = tune(args);
  const cutoff = (args.cutoff ?? 1600) * shine(args);
  const low = filter(ctx, "lowpass", cutoff, 2);
  low.frequency.setValueAtTime(cutoff, when);
  low.frequency.exponentialRampToValueAtTime(Math.max(200, cutoff * 0.3), when + 0.25);
  const amp = ctx.createGain();
  env(amp.gain, when, args.level ?? ARP_LEVEL, 0.003, 0.02, 0.22);
  for (const cents of [-7, 7]) {
    const o = osc(ctx, "sawtooth", hz(args.note ?? 69) * k, cents);
    o.connect(low);
    o.start(when);
    o.stop(when + 0.4);
  }
  low.connect(amp).connect(out);
  sends(ctx, amp, { space: ARP_SPACE, echoes: ARP_ECHO, ...args });
}

export function bass(ctx: BaseAudioContext, out: AudioNode, when: number, args: Args = {}): void {
  const k = tune(args);
  const dur = args.dur ?? 0.2;
  const cutoff = (args.cutoff ?? 400) * shine(args);
  const note = hz(args.note ?? 33) * k;
  const low = [filter(ctx, "lowpass", cutoff, 3), filter(ctx, "lowpass", cutoff, 3)];
  for (const f of low) {
    f.frequency.setValueAtTime(cutoff * 2.5, when);
    f.frequency.exponentialRampToValueAtTime(cutoff * 0.6, when + dur);
  }
  const saw = osc(ctx, "sawtooth", note);
  const square = osc(ctx, "square", note / 2);
  const half = ctx.createGain();
  half.gain.value = 0.5;
  const amp = ctx.createGain();
  const end = env(amp.gain, when, args.level ?? 0.4, 0.004, dur * 0.5, dur * 0.5);
  saw.connect(low[0]);
  square.connect(half).connect(low[0]);
  low[0].connect(low[1]).connect(amp).connect(out);
  sends(ctx, amp, args);
  for (const o of [saw, square]) {
    o.start(when);
    o.stop(end + 0.02);
  }
}

export function riser(ctx: BaseAudioContext, out: AudioNode, when: number, args: Args = {}): void {
  const k = tune(args);
  const lit = shine(args);
  const dur = args.dur ?? 1.8;
  const level = args.level ?? 0.4;
  const s = source(ctx, args, true);
  const band = filter(ctx, "bandpass", 200, 1.5);
  band.frequency.setValueAtTime(200, when);
  band.frequency.exponentialRampToValueAtTime(5000 * lit, when + dur);
  const air = ctx.createGain();
  const end = rise(air.gain, when, level, dur);
  s.connect(band).connect(air).connect(out);
  sends(ctx, air, args);
  s.start(when);
  s.stop(end + 0.02);
  const low = filter(ctx, "lowpass", 300, 4);
  low.frequency.setValueAtTime(300, when);
  low.frequency.exponentialRampToValueAtTime(6000 * lit, when + dur);
  const whine = ctx.createGain();
  rise(whine.gain, when, level * 0.35, dur);
  for (const cents of [-14, 0, 14]) {
    const o = osc(ctx, "sawtooth", hz(43) * k, cents);
    o.frequency.setValueAtTime(hz(43) * k, when);
    o.frequency.exponentialRampToValueAtTime(hz(67) * k, when + dur);
    o.connect(low);
    o.start(when);
    o.stop(end + 0.02);
  }
  low.connect(whine).connect(out);
}

export function drone(ctx: BaseAudioContext, out: AudioNode, when: number, args: Args = {}): void {
  const k = tune(args);
  const lit = shine(args);
  const dur = Math.max(1, args.dur ?? 4);
  const level = args.level ?? 0.3;
  const end = when + dur;
  const low = filter(ctx, "lowpass", 380 * lit, 4);
  const wobble = osc(ctx, "sine", 0.23);
  const depth = ctx.createGain();
  depth.gain.value = 220 * lit;
  wobble.connect(depth).connect(low.frequency);
  const amp = ctx.createGain();
  amp.gain.setValueAtTime(FLOOR, when);
  amp.gain.linearRampToValueAtTime(level, when + 0.4);
  amp.gain.setValueAtTime(level, end - 0.6);
  amp.gain.linearRampToValueAtTime(FLOOR, end);
  const saws = [-8, 0, 8].map((cents) => osc(ctx, "sawtooth", 55 * k, cents));
  for (const o of saws) o.connect(low);
  const s = source(ctx, args, true);
  const band = filter(ctx, "bandpass", 900 * lit, 0.8);
  const drift = osc(ctx, "sine", 0.11);
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

export function hit(ctx: BaseAudioContext, out: AudioNode, when: number, args: Args = {}): void {
  const k = tune(args);
  const level = args.level ?? 0.9;
  const white = args.white ? 1 : 0;
  const sub = osc(ctx, "sine", 60 * k);
  sub.frequency.setValueAtTime(60 * k, when);
  sub.frequency.exponentialRampToValueAtTime(30 * k, when + 0.6);
  const body = ctx.createGain();
  const end = env(body.gain, when, level, 0.003, 0.05, 0.55);
  sub.connect(body).connect(out);
  sub.start(when);
  sub.stop(end + 0.02);
  const s = source(ctx, args);
  const low = filter(ctx, "lowpass", 8000 * shine(args));
  low.frequency.setValueAtTime(8000 * shine(args), when);
  low.frequency.exponentialRampToValueAtTime(300, when + 0.3 + 0.4 * white);
  const crack = ctx.createGain();
  const tail = env(crack.gain, when, level * (0.6 + 0.2 * white), 0.001, 0.01, 0.35 + 0.45 * white);
  s.connect(low).connect(crack).connect(out);
  sends(ctx, crack, { ...args, space: Math.max(args.space ?? 0, 0.3) });
  s.start(when, args.at ?? 0);
  s.stop(tail + 0.02);
}

export function swell(ctx: BaseAudioContext, out: AudioNode, when: number, args: Args = {}): void {
  const dur = args.dur ?? 0.4;
  const s = source(ctx, args, true);
  const band = filter(ctx, "bandpass", 5000, 2);
  band.frequency.setValueAtTime(5000 * shine(args), when);
  band.frequency.exponentialRampToValueAtTime(200, when + dur);
  const amp = ctx.createGain();
  const end = rise(amp.gain, when, args.level ?? 0.5, dur);
  s.connect(band).connect(amp).connect(out);
  sends(ctx, amp, args);
  s.start(when);
  s.stop(end + 0.02);
}

export function tick(ctx: BaseAudioContext, out: AudioNode, when: number, args: Args = {}): void {
  const o = osc(ctx, "sine", 2400 * tune(args));
  const amp = ctx.createGain();
  env(amp.gain, when, args.level ?? 0.3, 0.001, 0, 0.007);
  o.connect(amp).connect(out);
  o.start(when);
  o.stop(when + 0.01);
}

export const VOICES: Record<string, Voice> = { kick, hat, clap, acid, pad, arp, bass, riser, drone, hit, swell, tick };

/* EFFECTS */

export const EFFECTS: Record<string, Effect> = {
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
  arrive: {
    span: (n) => 4 * n + 1.2,
    play: (ctx, out, when, a) => pad(ctx, out, when, { level: 0.09, ...a, dur: 4 * a.length }),
  },
};
