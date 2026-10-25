import type { Scene, View } from "../lib/app/run.ts";
import { blend } from "../lib/color.ts";
import type { Rng } from "../lib/rand.ts";
import type { Saver } from "./savers.ts";

/* TYPES */

type Flip = (r: number, c: number) => [number, number];

type Inks = { paper: string; tones: string[] };

const N = 5;

const FEW = 9;

const MANY = 16;

const TRIES = 100;

const SMALL = 8;

const BIG = 14;

const ACROSS = 72;

const SOFT = 0.5;

const PLAIN = "#.#.# .#.#. #.#.# .#.#. #.#.#";

const FLIPS: Flip[][] = [
  [(r, c) => [r, N - 1 - c]],
  [(r, c) => [r, N - 1 - c], (r, c) => [N - 1 - r, c]],
  [(r, c) => [c, N - 1 - r]],
  [(r, c) => [r, N - 1 - c], (r, c) => [c, r]],
];

/* TILE */

const orbits = (maps: Flip[]) => {
  const seen = new Set<number>();
  const out: number[][] = [];
  for (let i = 0; i < N * N; i++) {
    if (seen.has(i)) continue;
    const orbit = [i];
    seen.add(i);
    for (let k = 0; k < orbit.length; k++) {
      for (const map of maps) {
        const [r, c] = map(Math.floor(orbit[k] / N), orbit[k] % N);
        const j = r * N + c;
        if (seen.has(j)) continue;
        seen.add(j);
        orbit.push(j);
      }
    }
    out.push(orbit);
  }
  return out;
};

const GROUPS = FLIPS.map(orbits);

const fair = (cells: number[]) => {
  const count = cells.filter(Boolean).length;
  const rows = Array.from({ length: N }, (_, r) => cells.slice(r * N, r * N + N).join(""));
  const cols = Array.from({ length: N }, (_, c) => cells.filter((_, i) => i % N === c).join(""));
  const edges = [rows[0], rows[N - 1], cols[0], cols[N - 1]].every((line) => line.includes("1"));
  return count >= FEW && count <= MANY && edges && new Set(rows).size > 1 && new Set(cols).size > 1;
};

const swap = (cells: number[]) => cells.map((_, i) => cells[(i % N) * N + Math.floor(i / N)]);

export function tile(rand: Rng): number[] {
  for (let n = 0; n < TRIES; n++) {
    const cells: number[] = new Array(N * N).fill(0);
    for (const orbit of GROUPS[Math.floor(rand() * GROUPS.length)])
      if (rand() < 0.5) for (const i of orbit) cells[i] = 1;
    if (fair(cells)) return rand() < 0.5 ? cells : swap(cells);
  }
  return [...PLAIN.replaceAll(" ", "")].map((c) => (c === "#" ? 1 : 0));
}

/* SCENE */

export const SAVERS: Saver[] = [{ id: "tiles", title: "Tiles", pick: () => ({}) }];

export const PAGE = { still: true };

export function make(canvas: HTMLCanvasElement, view: View): Scene {
  const ctx = canvas.getContext("2d")!;
  const cells = tile(view.rand);
  let cell = 1;
  let inks: Inks | null = null;
  const size = () => {
    const short = Math.min(view.w, view.h) / view.dpr;
    cell = Math.max(1, Math.round(Math.min(BIG, Math.max(SMALL, Math.round(short / ACROSS))) * view.dpr));
  };
  const theme = () => {
    const { paper, accent } = view.look();
    inks = { paper, tones: [accent, blend(paper, accent, SOFT)] };
  };
  const draw = () => {
    const { w, h } = view;
    const side = cell * N;
    const cx = Math.floor((w - side) / 2);
    const cy = Math.floor((h - side) / 2);
    const ki = Math.ceil(cx / side);
    const kj = Math.ceil(cy / side);
    const x0 = cx - ki * side;
    const y0 = cy - kj * side;
    const cols = Math.ceil((w - x0) / cell);
    const rows = Math.ceil((h - y0) / cell);
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.fillStyle = inks!.paper;
    ctx.fillRect(0, 0, w, h);
    inks!.tones.forEach((tone, k) => {
      ctx.beginPath();
      for (let v = 0; v < rows; v++) {
        const line = (v % N) * N;
        const band = Math.floor(v / N) + ki + kj + k;
        let from = -1;
        for (let u = 0; u <= cols; u++) {
          const on = u < cols && cells[line + (u % N)] === 1 && ((Math.floor(u / N) + band) & 1) === 0;
          if (on && from < 0) from = u;
          else if (!on && from >= 0) {
            ctx.rect(x0 + from * cell, y0 + v * cell, (u - from) * cell, cell);
            from = -1;
          }
        }
      }
      ctx.fillStyle = tone;
      ctx.fill();
    });
  };
  size();
  theme();
  return { draw, size, theme, still: true };
}
