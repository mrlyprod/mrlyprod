import type { Paints } from "@mrlyprod/ui/kit/color.ts";
import { paints, veil } from "@mrlyprod/ui/kit/color.ts";
import type { Readout, Ready } from "@mrlyprod/ui/kit/hooks.ts";
import type { Fired, Key } from "@mrlyprod/ui/kit/keys.tsx";
import type { Handle, Page } from "@mrlyprod/ui/kit/page.tsx";
import type { Board, Box, Scene as Made, Opts, View } from "@mrlyprod/ui/kit/run.ts";
import { board } from "@mrlyprod/ui/kit/run.ts";
import type { Units } from "@mrlyprod/ui/kit/spec.ts";

export type Told = { rows: Readout[]; over: boolean };

export type Given<T extends Told = Told> = Opts & { tell?: (told: T) => void };

export type Area = Board & { line: number };

export type Inks = Paints & { hue: (name: string) => string };

export type Played = Made & { restart: () => void; alt?: (u: number, v: number) => void };

export type Aimed = Played & { nudge: (key?: string) => void; press: () => void };

export type Steered = Played & { steer: (key?: string) => void };

export type Cursor = {
  at: () => number;
  nudge: (key?: string) => void;
  hide: () => void;
  ring: (ctx: CanvasRenderingContext2D, box: Board, color: string) => void;
};

export type Holder = { press: (key?: string) => void; top: () => string | null; stop: () => void };

export type Swiper = {
  touch: (u: number, v: number, start: boolean) => null;
  lift: () => null;
};

export const UP = 0;
export const RIGHT = 1;
export const DOWN = 2;
export const LEFT = 3;

export const TURNS: Record<string, number> = {
  ArrowUp: UP,
  ArrowRight: RIGHT,
  ArrowDown: DOWN,
  ArrowLeft: LEFT,
  w: UP,
  d: RIGHT,
  s: DOWN,
  a: LEFT,
};

const ARROWS = Object.keys(TURNS).filter((key) => key.length > 1);
const WASD = Object.keys(TURNS).filter((key) => key.length === 1);

const STEPS = [
  [0, -1],
  [1, 0],
  [0, 1],
  [-1, 0],
];

const LINE = 0.08;
const FIT = 0.96;
const TEXT = 0.6;
const SWIPE = 24;
const RING = 0.06;
const GROUND = 0.07;
const GAP = 0.05;

const clamp = (v: number, low: number, high: number) => Math.min(high, Math.max(low, v));

export const lower = (key = "") => (key.length === 1 ? key.toLowerCase() : key);

export function arena(view: Box, cols: number, rows: number): Area {
  const line = Math.round(Math.min(view.w, view.h) * LINE);
  const box = board({ w: view.w, h: view.h - 2 * line }, cols, rows, FIT);
  return { ...box, y: box.y + line, line };
}

export function cellAt(view: Box, box: Board, cols: number, u: number, v: number): number {
  const col = Math.floor((u * view.w - box.x) / box.cell);
  const row = Math.floor((v * view.h - box.y) / box.cell);
  return col < 0 || col >= cols || row < 0 || row * box.cell >= box.h ? -1 : row * cols + col;
}

export function inks(canvas: Element, view: Pick<View, "look">): Inks {
  const seen = new Map<string, string>();
  const hue = (name: string) => {
    if (!seen.has(name)) seen.set(name, paints(canvas, name).accent);
    return seen.get(name)!;
  };
  return { ...view.look(), hue };
}

export function clear(ctx: CanvasRenderingContext2D, view: Box) {
  ctx.setTransform(1, 0, 0, 1, 0, 0);
  ctx.globalAlpha = 1;
  ctx.clearRect(0, 0, view.w, view.h);
}

export function heading(
  ctx: CanvasRenderingContext2D,
  canvas: Element,
  view: Box,
  box: Area,
  text: string,
  color: string,
) {
  ctx.font = `${Math.round(box.line * TEXT)}px ${getComputedStyle(canvas).fontFamily}`;
  ctx.textAlign = "center";
  ctx.textBaseline = "middle";
  ctx.fillStyle = color;
  ctx.fillText(text, view.w / 2, box.y - box.line / 2, view.w);
}

export function slab(
  ctx: CanvasRenderingContext2D,
  box: Board,
  cols: number,
  at: number,
  color: string,
  gap = GAP,
): [number, number, number] {
  const inset = Math.round(box.cell * gap);
  const x = box.x + (at % cols) * box.cell + inset;
  const y = box.y + Math.floor(at / cols) * box.cell + inset;
  const side = box.cell - 2 * inset;
  ctx.fillStyle = color;
  ctx.fillRect(x, y, side, side);
  return [x, y, side];
}

export const ground = (inked: Paints) => veil(inked.ink, GROUND);

export function pixels(ctx: CanvasRenderingContext2D, rows: readonly string[], x: number, y: number, side: number) {
  const tall = rows.length;
  const wide = Math.max(...rows.map((row) => row.length));
  const unit = Math.max(1, Math.floor(side / Math.max(tall, wide)));
  const left = Math.round(x + (side - unit * wide) / 2);
  const top = Math.round(y + (side - unit * tall) / 2);
  for (const [r, row] of rows.entries())
    for (const [c, bit] of [...row].entries())
      if (bit === "1") ctx.fillRect(left + c * unit, top + r * unit, unit, unit);
}

export function cursor(cols: number, rows: number, start = Math.floor((rows * cols) / 2)): Cursor {
  let at = -1;
  return {
    at: () => at,
    nudge: (key) => {
      if (at < 0) {
        at = start;
        return;
      }
      const [dx, dy] = STEPS[TURNS[lower(key)]] ?? [0, 0];
      const col = clamp((at % cols) + dx, 0, cols - 1);
      const row = clamp(Math.floor(at / cols) + dy, 0, rows - 1);
      at = row * cols + col;
    },
    hide: () => {
      at = -1;
    },
    ring: (ctx, box, color) => {
      if (at < 0) return;
      const width = Math.max(2, Math.round(box.cell * RING));
      ctx.strokeStyle = color;
      ctx.lineWidth = width;
      ctx.strokeRect(
        box.x + (at % cols) * box.cell + width / 2,
        box.y + Math.floor(at / cols) * box.cell + width / 2,
        box.cell - width,
        box.cell - width,
      );
    },
  };
}

export function swiper(view: Pick<View, "w" | "h" | "dpr">, go: (dir: number) => void): Swiper {
  let from: [number, number] | null = null;
  return {
    touch: (u, v, start) => {
      if (start || !from) {
        from = [u, v];
        return null;
      }
      const dx = ((u - from[0]) * view.w) / view.dpr;
      const dy = ((v - from[1]) * view.h) / view.dpr;
      if (Math.max(Math.abs(dx), Math.abs(dy)) < SWIPE) return null;
      go(Math.abs(dx) > Math.abs(dy) ? (dx > 0 ? RIGHT : LEFT) : dy > 0 ? DOWN : UP);
      from = [u, v];
      return null;
    },
    lift: () => {
      from = null;
      return null;
    },
  };
}

export function holder(): Holder {
  const down: string[] = [];
  const up = (e: KeyboardEvent) => {
    const at = down.indexOf(lower(e.key));
    if (at >= 0) down.splice(at, 1);
  };
  const clean = () => {
    down.length = 0;
  };
  addEventListener("keyup", up);
  addEventListener("blur", clean);
  return {
    press: (key) => {
      const one = lower(key);
      if (one && !down.includes(one)) down.push(one);
    },
    top: () => down.at(-1) ?? null,
    stop: () => {
      removeEventListener("keyup", up);
      removeEventListener("blur", clean);
    },
  };
}

const NEW = { key: "r", label: "New game", act: "reroll" };
const HELP = { key: "?", label: "Keys", act: "keys" };

export const aimed = <S extends Aimed>() =>
  ({
    gestures: true,
    keys: [
      { key: ARROWS, label: "Move", act: "arrows" },
      { key: ["Enter", " "], label: "Pick", act: "press" },
      NEW,
      HELP,
    ] as Key<Handle<S>>[],
    actions: {
      arrows: (scene: Handle<S>, e: Fired) => scene.nudge(e.key),
      press: (scene: Handle<S>) => scene.press(),
    },
  }) satisfies Page<Ready & Units, S>;

export const steered = <S extends Steered>(label = "Move") =>
  ({
    gestures: true,
    keys: [{ key: ARROWS, label, act: "arrows" }, NEW, HELP, { key: WASD, label, act: "wasd" }] as Key<Handle<S>>[],
    actions: {
      arrows: (scene: Handle<S>, e: Fired) => scene.steer(e.key),
      wasd: (scene: Handle<S>, e: Fired) => scene.steer(e.key),
    },
  }) satisfies Page<Ready & Units, S>;
