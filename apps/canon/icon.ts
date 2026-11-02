import type { Color, Pen, Point } from "@mrlyprod/draw";
import { mix } from "@mrlyprod/draw";
import * as math from "@mrlyprod/math";
import { plate } from "../../net/sdk/plate.ts";
import { rng } from "../../net/sdk/rand.ts";
import type { State } from "./engine.ts";
import { boot, echoes, fills, grown, HOME, ONE, spot, starts, step, TWO, tally } from "./engine.ts";
import { pick } from "./policy.ts";

export const units = { math };

const NUMBER = 3;
const LEVEL = 2;
const SEED = 5;
const PLAYS = 13;
const GAP = 0.14;
const CORNER = 0.26;
const THREAD = 0.05;
const GHOST: Color = [255, 255, 255, 70];
const SHADE: Color = [255, 255, 255, 30];
const DUSK = 40;

let game: State | null = null;

function played(): State {
  if (game) return game;
  const motif = grown(math, NUMBER, 1, HOME);
  const rand = rng(SEED);
  let state = boot({ number: NUMBER, level: LEVEL, motif, seed: SEED });
  for (let i = 0; i < PLAYS; i++) state = step(state, pick(state, "smart", rand)!);
  if (state.over || !state.board.includes("1") || !state.board.includes("2"))
    throw new Error("canon: the figure game is not half played");
  game = state;
  return state;
}

export default function draw(pen: Pen) {
  const { box: frame, paint } = plate(pen, "purple", 0.16);
  const state = played();
  const side = NUMBER ** LEVEL;
  const c = frame.w / side;
  const dots = mix(paint.deep, paint.ink, 0.35);
  const f = fills(state.motif).length;
  const at = starts(f, LEVEL);
  const { owners } = tally(state);
  const shade: Color[] = [GHOST, SHADE, [...dots.slice(0, 3), DUSK] as Color];
  const centre = (cell: number): Point => {
    const [x, y] = spot(state, cell);
    return [frame.x + (x + 0.5) * c, frame.y + (y + 0.5) * c];
  };
  for (let d = 0; d < LEVEL; d++)
    for (let p = 0; p < at[d + 1] - at[d]; p++) {
      const who = owners[at[d] + p];
      const count = f ** (LEVEL - d);
      for (let cell = p * count; who && cell < (p + 1) * count; cell++) {
        const [x, y] = centre(cell);
        pen.rect(x - c / 2, y - c / 2, c, c, shade[who]);
      }
    }
  const mover = state.turn === ONE ? TWO : ONE;
  const echo = echoes(state, state.last).map(centre);
  if (echo.length > 1) pen.polyline(echo, c * THREAD, mover === ONE ? paint.white : dots);
  const size = c * (1 - 2 * GAP);
  for (let cell = 0; cell < state.board.length; cell++) {
    const [x, y] = centre(cell);
    const who = state.board.charCodeAt(cell) - 48;
    if (who === TWO) pen.disc(x, y, size / 2, dots);
    else pen.round_rect(x - size / 2, y - size / 2, size, size, size * CORNER, who === ONE ? paint.white : GHOST);
  }
}
