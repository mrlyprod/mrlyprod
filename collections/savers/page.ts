import { palette } from "@mrlyprod/draw/palette.gen.js";
import type { Look, Play, Saver } from "./play.ts";

type Theme = Look["theme"];

type Size = [number, number];

type Beat = { ms: number; verb?: "freeze" | "resume" | "flip" };

const THEMES: Theme[] = ["dark", "light"];

const TINTS = [palette.indigo, palette.orange, palette.mint];

const HUES = Object.values(palette).slice(2, -1);

const SEEDS = [1, 2, 3];

const TIMES = [0, 20, 60, 300, 1200, 3600];

const SEED: Size = [2, 2];

const COST = 30;

const HOME: Look = { theme: "dark", tint: palette.indigo, seed: 1 };

const STEP = 1000 / 60;

const GAP = 4;

const EYE: Size = [96, 60];

const BEATS: Beat[] = [
  { ms: 5000 },
  { ms: 3500, verb: "freeze" },
  { ms: 3500, verb: "resume" },
  { ms: 2000, verb: "flip" },
];

const EVERY = 500;

const real = requestAnimationFrame.bind(window);

let now = 0;

let queue: { id: number; fn: FrameRequestCallback }[] = [];

let ids = 0;

const breathe = () => new Promise<void>((done) => real(() => real(() => done())));

function fake() {
  window.requestAnimationFrame = (fn) => {
    queue.push({ id: ++ids, fn });
    return ids;
  };
  window.cancelAnimationFrame = (id) => {
    queue = queue.filter((one) => one.id !== id);
  };
}

function tick(ms: number) {
  now += ms;
  const due = queue;
  queue = [];
  for (const { fn } of due) fn(now);
}

function advance(ms: number, step = 100) {
  for (let left = ms; left > 1e-6; left -= step) tick(Math.min(step, left));
}

async function stage([w, h]: Size) {
  for (const old of document.querySelectorAll("canvas"))
    old.getContext("webgl2")?.getExtension("WEBGL_lose_context")?.loseContext();
  const canvas = document.createElement("canvas");
  canvas.style.cssText = `display:block;width:${w}px;height:${h}px`;
  document.body.replaceChildren(canvas);
  await breathe();
  return canvas;
}

function board(count: number, cols: number, [w, h]: Size) {
  const k = devicePixelRatio;
  const out = document.createElement("canvas");
  out.width = (cols * (w + GAP) + GAP) * k;
  out.height = (Math.ceil(count / cols) * (h + GAP) + GAP) * k;
  const ctx = out.getContext("2d")!;
  ctx.fillStyle = palette.gray;
  ctx.fillRect(0, 0, out.width, out.height);
  const put = (canvas: HTMLCanvasElement, i: number) =>
    ctx.drawImage(canvas, (GAP + (i % cols) * (w + GAP)) * k, (GAP + Math.floor(i / cols) * (h + GAP)) * k);
  return { out, put };
}

function eye() {
  const small = document.createElement("canvas");
  [small.width, small.height] = EYE;
  const ctx = small.getContext("2d", { willReadFrequently: true })!;
  let before: Uint8ClampedArray | null = null;
  return (canvas: HTMLCanvasElement) => {
    ctx.drawImage(canvas, 0, 0, ...EYE);
    const { data } = ctx.getImageData(0, 0, ...EYE);
    let sum = 0;
    if (before) for (let i = 0; i < data.length; i++) sum += Math.abs(data[i] - before[i]);
    before = data;
    return sum / data.length;
  };
}

function shots(saver: Saver) {
  fake();
  const aged = async (look: Look, [w, h]: Size, ms: number) => {
    const canvas = await stage(SEED);
    saver(canvas, look);
    advance(ms);
    canvas.style.width = `${w}px`;
    canvas.style.height = `${h}px`;
    await breathe();
    tick(STEP);
    return canvas;
  };
  const sheet = async (all: Look[], cols: number, size: Size, at: (i: number) => number) => {
    const { out, put } = board(all.length, cols, size);
    for (const [i, look] of all.entries()) put(await aged(look, size, at(i)), i);
    return out.toDataURL();
  };
  const looks = (size: Size, ms: number) => {
    const all = THEMES.flatMap((theme) => TINTS.flatMap((tint) => SEEDS.map((seed) => ({ theme, tint, seed }))));
    return sheet(all, SEEDS.length, size, () => ms);
  };
  const hues = (size: Size, ms: number) => {
    const all = THEMES.flatMap((theme) => HUES.map((tint) => ({ theme, tint, seed: 4 })));
    return sheet(all, HUES.length, size, () => ms);
  };
  const times = (size: Size) => {
    const all = THEMES.flatMap((theme) => TIMES.map(() => ({ ...HOME, theme })));
    return sheet(all, TIMES.length, size, (i) => TIMES[i % TIMES.length] * 1000);
  };
  const full = async (look: Partial<Look>, size: Size, ms: number) => {
    const canvas = await aged({ ...HOME, ...look }, size, ms);
    const from = performance.now();
    advance(COST * STEP, STEP);
    return { png: canvas.toDataURL(), cost: (performance.now() - from) / COST };
  };
  const eases = async (size: Size) => {
    const canvas = await stage(size);
    const total = BEATS.reduce((sum, beat) => sum + beat.ms, 0);
    const { out, put } = board(Math.floor(total / EVERY), 8, size);
    const see = eye();
    const curve: number[] = [];
    const frames: string[] = [];
    let look = HOME;
    let playing: Play = saver(canvas, look);
    let ticks = 0;
    let cell = 0;
    for (const { ms, verb } of BEATS) {
      if (verb === "flip") look = { ...look, theme: "light" };
      if (verb === "flip") playing = saver(canvas, look);
      else if (verb) playing[verb]();
      for (let n = Math.round(ms / STEP); n > 0; n--) {
        tick(STEP);
        ticks++;
        if (verb !== "flip") curve.push(see(canvas));
        if (ticks * STEP + 1e-6 < (cell + 1) * EVERY) continue;
        put(canvas, cell++);
        frames.push(canvas.toDataURL());
      }
    }
    return { png: out.toDataURL(), curve, frames };
  };
  const first = async (look: Partial<Look>, size: Size, calm = false) => {
    document.documentElement.dataset.motion = calm ? "reduce" : "";
    const canvas = await stage(size);
    saver(canvas, { ...HOME, ...look });
    const start = canvas.toDataURL();
    advance(3000);
    document.documentElement.dataset.motion = "";
    return { start, later: canvas.toDataURL() };
  };
  const resize = async (from: Size, to: Size) => {
    const canvas = await stage(from);
    saver(canvas, HOME);
    advance(3000);
    canvas.style.width = `${to[0]}px`;
    canvas.style.height = `${to[1]}px`;
    await breathe();
    const fit = canvas.width === Math.round(to[0] * devicePixelRatio);
    tick(STEP);
    const png = canvas.toDataURL();
    return { fit, blank: new Set(png).size < 20, png };
  };
  Object.assign(window, { shots: { looks, hues, times, full, eases, first, resize } });
}

function live(saver: Saver) {
  const look = { ...HOME, theme: matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light" } as Look;
  const canvas = document.createElement("canvas");
  canvas.style.cssText = "position:fixed;inset:0;width:100%;height:100%";
  document.body.append(canvas);
  let playing = saver(canvas, look);
  let frozen = false;
  const again = () => {
    playing = saver(canvas, look);
    if (frozen) playing.freeze();
  };
  const keys: Record<string, () => void> = {
    " ": () => {
      frozen = !frozen;
      if (frozen) playing.freeze();
      else playing.resume();
    },
    t: () => {
      look.theme = look.theme === "dark" ? "light" : "dark";
      again();
    },
    c: () => {
      look.tint = HUES[(HUES.indexOf(look.tint) + 1) % HUES.length];
      again();
    },
    s: () => {
      look.seed++;
      again();
    },
  };
  document.addEventListener("keydown", (event) => keys[event.key]?.());
}

export const boot = (saver: Saver) => (location.hash === "#shots" ? shots(saver) : live(saver));
