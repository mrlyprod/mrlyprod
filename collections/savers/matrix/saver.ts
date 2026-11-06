import FONT from "@mrlyprod/ui/kit/font/font.gen.json" with { type: "json" };
import type { Look, Play } from "../play.ts";
import { hash, inks, play, smooth } from "../play.ts";

const SET = /^[0-9A-Z]$/;

const TONES = 32;

const SIDE = 5;

const WIDE = 10;

const TALL = 8;

const CELL = [2, 8];

const DENSITY = 225;

const LEAD = 1;

const CYCLE = 90;

const SLOW = 48;

const FAST = 24;

const SWAY = 0.15;

const TAIL = [0.15, 0.4];

const FADE = 0.75;

const PEAK = { dark: [0.65, 1], light: [0.75, 1] };

const GOLD = 0.618033988749895;

const mix = ([a, b]: number[], k: number) => a + (b - a) * k;

const sized = (w: number, h: number, px: number) =>
  Math.round(Math.min(CELL[1], Math.max(CELL[0], Math.round(Math.sqrt(w * h) / px / DENSITY))) * px);

export default function matrix(canvas: HTMLCanvasElement, look: Look): Play {
  const ctx = canvas.getContext("2d")!;
  const { css } = inks(look);
  const { seed, theme } = look;
  const marks = Object.entries(FONT as Record<string, { rows: string[] }>)
    .filter(([key]) => SET.test(key))
    .map(([, { rows }]) => rows.flatMap((row, y) => [...row].flatMap((on, x) => (on === "1" ? [[x, y]] : []))));
  const n = marks.length;
  const tones = Array.from({ length: TONES + 1 }, (_, i) => css(i / TONES));
  const turn = hash(seed);
  return play(canvas, (t, px) => {
    const { width: w, height: h } = canvas;
    const cell = sized(w, h, px);
    const m = (WIDE - SIDE) * cell;
    const fw = w - 2 * m - SIDE * cell;
    const fh = h - 2 * m - SIDE * cell;
    const cols = Math.max(1, Math.floor(fw / (WIDE * cell)) + 1);
    const rows = Math.max(1, Math.floor(fh / (TALL * cell)) + 1);
    const dx = fw / Math.max(1, cols - 1);
    const dy = fh / Math.max(1, rows - 1);
    const paths = tones.map(() => new Path2D());
    for (let c = 0; c < cols; c++) {
      const off = ((c * GOLD + turn) % 1) * CYCLE;
      const pass = Math.floor((t - off) / CYCLE);
      const v = hash(seed, c, pass * 8 + 1);
      const tail = rows * mix(TAIL, hash(seed, c, pass * 8 + 2));
      const cross = mix([SLOW, FAST], v);
      const start = pass * CYCLE + off + (CYCLE - cross) * (0.5 + (hash(seed, c, pass * 8 + 5) - 0.5) * SWAY);
      const head = ((t - start) * (rows + tail + LEAD)) / cross - LEAD;
      const top = mix(PEAK[theme], v);
      const x0 = m + Math.round(c * dx);
      const last = Math.min(rows - 1, Math.floor(head + LEAD));
      for (let r = Math.max(0, Math.ceil(head - tail)); r <= last; r++) {
        const d = head - r;
        const k = (d < 0 ? smooth((d + LEAD) / LEAD) : (1 - d / tail) ** FADE) * top;
        const tone = Math.round(k * TONES);
        if (!tone) continue;
        const y0 = m + Math.round(r * dy);
        const i = Math.floor(hash(seed, c * 4096 + r, pass * 8 + 7) * n);
        for (const [x, y] of marks[i]) paths[tone].rect(x0 + x * cell, y0 + y * cell, cell, cell);
      }
    }
    ctx.fillStyle = tones[0];
    ctx.fillRect(0, 0, w, h);
    for (let i = 1; i <= TONES; i++) {
      ctx.fillStyle = tones[i];
      ctx.fill(paths[i]);
    }
  });
}
