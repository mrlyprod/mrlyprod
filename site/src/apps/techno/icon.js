import { plate } from "../../lib/plate.js";

export const units = {};

const STEPS = 16;
const BEAT = 4;
const LIT = 4;
const GAP = 0.16;
const ROUND = 0.22;
const TRACK = "x.xx.x..x.x..xx.";
const HITS = 8;

export default function draw(pen) {
  if (TRACK.length !== STEPS) throw new Error(`techno: ${TRACK} is not ${STEPS} steps`);
  const hits = [...TRACK].filter((c) => c === "x").length;
  if (hits !== HITS) throw new Error(`techno: the track lights ${hits} steps, want ${HITS}`);
  const { box, paint } = plate(pen, "red", 0.2);
  const unit = box.w / (BEAT + (BEAT - 1) * GAP);
  const gap = unit * GAP;
  const r = unit * ROUND;
  for (let s = 0; s < STEPS; s++) {
    const x = box.x + (s % BEAT) * (unit + gap);
    const y = box.y + Math.floor(s / BEAT) * (unit + gap);
    const on = TRACK[s] === "x";
    const now = Math.floor(s / BEAT) === Math.floor(LIT / BEAT);
    pen.round_rect(x, y, unit, unit, r, on ? paint.white : now ? paint.light : paint.deep);
  }
}
