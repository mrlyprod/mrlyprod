import type { Take } from "../../src/lib/app/record.ts";
import { record } from "../../src/lib/app/record.ts";
import { run } from "../../src/lib/app/run.ts";
import type { Rng } from "../../src/lib/rand.ts";
import { rng } from "../../src/lib/rand.ts";
import type { Name, Units } from "../../src/lib/units/index.ts";

/* TYPES */

export type Size = [number, number];

export type Cue = [number, string];

export type Look = { paper: string; accent: string };

export type Opts = Record<string, unknown>;

export type Doors = Partial<Units<Name>>;

export type Sized = Partial<Pick<HTMLCanvasElement, "width" | "height">>;

export type View = {
  rand: Rng;
  look: () => Partial<Look>;
  still: boolean;
  w: number;
  h: number;
  dpr: number;
  t: number;
  fixed: boolean;
};

export type Scene = Record<string, unknown> & { draw: () => unknown; font?: string; ready?: unknown };

export type Make<C = HTMLCanvasElement> = (canvas: C, view: View, opts: Opts) => Scene;

export type Reel = {
  seed: number;
  opts?: Opts;
  cues?: Cue[];
  from?: number;
  step: number;
  frames: number;
  look: Partial<Look>;
};

type Shared = { name: string; seed: number; opts?: Opts; cues?: Cue[]; from?: number; frames: number; look: Look };

type Still = Shared & { video?: false; step: number };

type Moving = Shared & { video: true; fps: number };

export type Config = Still | Moving;

type Info = { t: number; late: number };

type Held = { frame: Size; step: number; after: (info: Info) => void };

/* SHEET */

export const SIZE: Size = [1280, 720];

const TILE: Size = [320, 180];

const COLS = 6;

export const sheet = (frames: number): Size => [COLS * TILE[0], Math.ceil(frames / COLS) * TILE[1]];

export const tile = (i: number): Size => [(i % COLS) * TILE[0], Math.floor(i / COLS) * TILE[1]];

/* ROLL */

function play(scene: Scene, view: Pick<View, "t">, due: Cue[], t: number) {
  while (due.length && due[0][0] <= t) {
    const [at, name] = due.shift()!;
    if (typeof scene[name] !== "function") throw new Error(`film: the scene has no ${name}()`);
    view.t = at;
    scene[name]();
  }
  view.t = t;
}

const order = (cues: Cue[]) => [...cues].sort((a, b) => a[0] - b[0]);

export async function roll<C extends Sized>(
  make: Make<C>,
  canvas: C,
  { seed, opts = {}, cues = [], from = 0, step, frames, look }: Reel,
  each: (i: number) => void,
): Promise<void> {
  [canvas.width, canvas.height] = SIZE;
  const view: View = {
    rand: rng(seed),
    look: () => look,
    still: false,
    w: SIZE[0],
    h: SIZE[1],
    dpr: 1,
    t: from,
    fixed: true,
  };
  const scene = make(canvas, view, { ...opts, seed });
  if (scene.font && globalThis.document?.fonts) await document.fonts.load(`1em ${JSON.stringify(scene.font)}`);
  await scene.ready;
  const due = order(cues);
  for (let i = 0; i < frames; i++) {
    play(scene, view, due, from + i * step);
    scene.draw();
    each(i);
    await new Promise((r) => setTimeout(r, 0));
  }
}

/* TAPE */

function tape(
  make: Make,
  canvas: HTMLCanvasElement,
  { name, seed, opts = {}, cues = [], from = 0, fps, frames, look }: Moving,
) {
  const step = 1000 / fps;
  const due = order(cues);
  canvas.style.setProperty("--art", look.paper);
  canvas.style.setProperty("--accent", look.accent);
  const staged = (at: HTMLCanvasElement, view: View, given: Opts) => {
    view.t = from - step;
    const scene = make(at, view, given);
    return {
      ...scene,
      draw: () => {
        play(scene, view, due, view.t);
        return scene.draw();
      },
    };
  };
  const stop = run(canvas, staged, { ...opts, seed, step, frame: SIZE });
  let drawn = 0;
  let take: Take | null = null;
  const fix = (held: Held | null) =>
    stop.fix(
      held && {
        ...held,
        after: (info: Info) => {
          if (drawn >= frames) return take!.stop();
          drawn += 1;
          held.after(info);
        },
      },
    );
  take = record({ canvas, fix }, { ratio: "16:9", fps, name, size: SIZE });
  return take.done.finally(stop).then((file) => new Promise<string>((r) => setTimeout(() => r(file), 2000)));
}

/* FILM */

export async function film(make: Make, units?: () => Promise<Doors>) {
  const read = JSON.parse(document.getElementById("film")!.textContent) as Config;
  const held: Doors = units ? await units() : {};
  await held.ready;
  const { ready, ...doors } = held;
  const config = { ...read, opts: { ...read.opts, ...doors } };
  const canvas = document.createElement("canvas");
  document.body.setAttribute("aria-busy", "true");
  try {
    if (config.video) {
      document.body.append(canvas);
      return await tape(make, canvas, config);
    }
    const out = document.createElement("canvas");
    [out.width, out.height] = sheet(config.frames);
    const pen = out.getContext("2d")!;
    pen.imageSmoothingQuality = "high";
    document.body.append(out);
    await roll(make, canvas, config, (i) => pen.drawImage(canvas, ...tile(i), ...TILE));
  } finally {
    document.body.removeAttribute("aria-busy");
  }
}
