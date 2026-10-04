import * as math from "mrlyjs/math";
import type { Ink, Pen } from "mrlyjs/view";

export const units = { math };

const BASE = 2;
const DIMENSION = 2;
const CELLS = 4;
const PATTERNS = 16;
const CLASSES = 6;
const FIXED = 48;

let memo: number[][] | undefined;

function classes() {
  if (memo) return memo;
  const moves = math.bang.baseq.group(BASE, DIMENSION);
  let fixed = 0;
  for (const move of moves) {
    for (let mask = 0; mask < PATTERNS; mask++) if (Number(math.bang.baseq.carry(move, mask)) === mask) fixed++;
  }
  if (fixed !== FIXED) throw new Error(`wiki-burnsides-lemma: ${fixed} fixed points, want ${FIXED}`);
  if (fixed / moves.length !== CLASSES) throw new Error(`wiki-burnsides-lemma: ${fixed} fixed points over ${moves.length} moves, want ${CLASSES} classes`);
  const orbits = math.bang.baseq.representatives(BASE, DIMENSION).map(([code, size]) => {
    const orbit = math.bang.baseq.orbit(moves, code).map(Number);
    if (orbit.length !== size) throw new Error(`wiki-burnsides-lemma: the class of ${code} holds ${orbit.length}, want ${size}`);
    return orbit;
  });
  if (orbits.length !== CLASSES) throw new Error(`wiki-burnsides-lemma: ${orbits.length} classes, want ${CLASSES}`);
  const seen = new Set(orbits.flat());
  if (orbits.flat().length !== PATTERNS || seen.size !== PATTERNS) throw new Error(`wiki-burnsides-lemma: the classes do not partition ${PATTERNS} patterns`);
  memo = orbits;
  return memo;
}

export default function draw(pen: Pen, ink: Ink) {
  const orbits = classes();
  const frame = pen.frame(0.08);
  const rows = frame.rows(CLASSES);
  const widest = Math.max(...orbits.map((orbit) => orbit.length));
  const slot = frame.w / widest;
  const stamp = Math.min(slot * 0.62, rows[0].h * 0.74);
  const pad = stamp * 0.06;
  orbits.forEach((orbit, k) => {
    const row = rows[k];
    const run = orbit.length * slot;
    const left = row.x + (row.w - run) / 2;
    const top = row.y + (row.h - stamp) / 2;
    orbit.forEach((mask, place) => {
      const x = left + place * slot + (slot - stamp) / 2;
      const half = (stamp - pad) / 2;
      for (let cell = 0; cell < CELLS; cell++) {
        const tone = (mask >> cell) & 1 ? ink.blue : ink.dim;
        pen.rect(x + (cell % 2) * (half + pad), top + Math.floor(cell / 2) * (half + pad), half, half, tone);
      }
    });
  });
}
