import { sprite } from "@mrlyprod/sprites";
import type { Look, Play } from "../play.ts";
import { hash, inks, play, rng } from "../play.ts";

const COARSE = 3;

const FINE = 5;

const CELL = 2;

const SQUARE = 0.6;

const SLOPES = [
  [1, 1],
  [2, 1],
  [1, 2],
];

const WAVES = [0.7, 1];

const PACE = 1 / 270;

const FADE = 3.5;

const STEPS = 64;

function lead(length: number, span: number) {
  const cut = (length / 2) % (span / 2);
  const off = Math.round(cut < span / 4 ? cut : cut + span / 2) % span;
  return off === 0 ? 0 : off - span;
}

export default function tiles(canvas: HTMLCanvasElement, look: Look): Play {
  const ctx = canvas.getContext("2d")!;
  const { css } = inks(look);
  const rand = rng(look.seed);
  const side = COARSE * FINE;
  const coarse = sprite("carpet", COARSE).cells;
  const fine = sprite("carpet", FINE).cells;
  const wave = WAVES[Math.floor(rand() * WAVES.length)];
  const [a, b] = SLOPES[Math.floor(rand() * SLOPES.length)];
  const turn = Math.floor(rand() * 4);
  const ux = (turn & 1 ? -a : a) / Math.hypot(a, b);
  const uy = (turn & 2 ? -b : b) / Math.hypot(a, b);
  const start = rand();
  let states: OffscreenCanvas[] = [];
  const paint = (unit: number, span: number) => {
    states = Array.from({ length: STEPS + 1 }, (_, q) => {
      const tile = new OffscreenCanvas(span, span);
      const pen = tile.getContext("2d")!;
      pen.fillStyle = css(0);
      pen.fillRect(0, 0, span, span);
      for (let i = 0; i < COARSE * COARSE; i++) {
        const x = (i % COARSE) * FINE;
        const y = Math.floor(i / COARSE) * FINE;
        if (!coarse[i]) {
          pen.fillStyle = css(SQUARE);
          pen.fillRect(x * unit, y * unit, FINE * unit, FINE * unit);
          continue;
        }
        pen.fillStyle = css(q / STEPS);
        for (let j = 0; j < FINE * FINE; j++)
          if (!fine[j]) pen.fillRect((x + (j % FINE)) * unit, (y + Math.floor(j / FINE)) * unit, unit, unit);
      }
      return tile;
    });
  };
  return play(canvas, (t, px) => {
    const { width: w, height: h } = canvas;
    const unit = Math.max(1, Math.round(CELL * px));
    const span = unit * side;
    if (states[0]?.width !== span) paint(unit, span);
    const reach = wave * Math.hypot(w, h);
    const gap = span / (Math.hypot(a, b) * reach);
    const soft = 2 * gap * Math.max(a + b, Math.ceil((FADE * PACE) / gap));
    const phase = (start + t * PACE) % 1;
    for (let y = lead(h, span); y < h; y += span)
      for (let x = lead(w, span); x < w; x += span) {
        const u = ((x + span / 2 - w / 2) * ux + (y + span / 2 - h / 2) * uy) / reach - phase;
        const v = (Math.abs(2 * (u - Math.floor(u)) - 1) - 0.5) / soft + 0.5;
        const q = Math.floor(Math.min(1, Math.max(0, v)) * STEPS + hash(x, y));
        ctx.drawImage(states[q], x, y);
      }
  });
}
