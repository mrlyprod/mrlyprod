import { dark, light } from "@mrlyprod/draw/theme.ts";

export type Look = { theme: "light" | "dark"; tint: string; seed: number };

export type Play = { freeze: () => Promise<void>; resume: () => void; stop: () => void };

export type Saver = (canvas: HTMLCanvasElement, look: Look) => Play;

export type Draw = (t: number, px: number) => void;

export type Rgb = [number, number, number];

export type Inks = { rgb: (k: number) => Rgb; css: (k: number) => string };

type Clock = { t: number; gain: number; to: number; stop: () => void };

const EASE = 2000;

const LAG = 100;

const SHARP = 2;

const CLOCKS = new WeakMap<HTMLCanvasElement, Clock>();

export const smooth = (k: number) => k * k * (3 - 2 * k);

const calm = () =>
  document.documentElement.dataset.motion === "reduce" || matchMedia("(prefers-reduced-motion: reduce)").matches;

function scramble(n: number) {
  let s = Math.imul(n ^ (n >>> 16), 0x85ebca6b);
  s = Math.imul(s ^ (s >>> 13), 0xc2b2ae35);
  return (s ^ (s >>> 16)) >>> 0;
}

export const hash = (a: number, b = 0, c = 0) => scramble(scramble(scramble(a + 0x9e3779b9) ^ b) ^ c) / 4294967296;

export function rng(seed: number): () => number {
  let n = 0;
  return () => hash(seed, n++);
}

export function inks({ theme, tint }: Look): Inks {
  const parse = (hex: string) => [1, 3, 5].map((i) => Number.parseInt(hex.slice(i, i + 2), 16) / 255);
  const paper = parse(theme === "dark" ? dark.ground : light.ground);
  const ink = parse(tint);
  const rgb = (k: number) => paper.map((v, i) => v + (ink[i] - v) * k) as Rgb;
  return { rgb, css: (k) => `rgb(${rgb(k).map((v) => Math.round(v * 255))})` };
}

export function play(canvas: HTMLCanvasElement, draw: Draw, most = Number.POSITIVE_INFINITY): Play {
  const was = CLOCKS.get(canvas);
  const clock: Clock = { t: was?.t ?? 0, gain: was?.gain ?? 0, to: was?.to ?? 1, stop: () => stop() };
  const still = calm();
  const waits: (() => void)[] = [];
  let px = 1;
  let frame = 0;
  let last = 0;
  let gone = false;
  let ease = { from: 0, to: 1, at: 0 };
  const fit = () => {
    px = Math.min(devicePixelRatio || 1, SHARP, Math.sqrt(most / (canvas.clientWidth * canvas.clientHeight || 1)));
    const w = Math.max(1, Math.round(canvas.clientWidth * px));
    const h = Math.max(1, Math.round(canvas.clientHeight * px));
    if (canvas.width === w && canvas.height === h) return false;
    canvas.width = w;
    canvas.height = h;
    return true;
  };
  const paint = () => draw(clock.t / 1000, px);
  const settle = () => {
    for (const done of waits.splice(0)) done();
  };
  const hold = () => {
    cancelAnimationFrame(frame);
    frame = 0;
    last = 0;
    clock.gain = 0;
    settle();
  };
  const loop = (now: number) => {
    frame = 0;
    if (!last) ease.at = last = now;
    clock.gain = ease.from + (ease.to - ease.from) * smooth(Math.min(1, (now - ease.at) / EASE));
    clock.t += Math.min(now - last, LAG) * clock.gain;
    last = now;
    try {
      paint();
    } finally {
      if (!ease.to && !clock.gain) hold();
      else frame = requestAnimationFrame(loop);
    }
  };
  const turn = (to: number) => {
    clock.to = to;
    if (ease.to !== to || !frame) ease = { from: clock.gain, to, at: last };
    if (!frame && to && !still && !gone) frame = requestAnimationFrame(loop);
  };
  const tape = new ResizeObserver(() => {
    if (!canvas.isConnected) return stop();
    if (fit()) paint();
  });
  const stop = () => {
    gone = true;
    hold();
    tape.disconnect();
  };
  was?.stop();
  CLOCKS.set(canvas, clock);
  fit();
  paint();
  tape.observe(canvas);
  turn(clock.to);
  return {
    freeze: () =>
      new Promise<void>((done) => {
        waits.push(done);
        if (!frame || document.hidden) hold();
        else turn(0);
      }),
    resume: () => {
      settle();
      turn(1);
    },
    stop,
  };
}
