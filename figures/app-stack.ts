import * as math from "mrlyjs/math";
import type { Ink, Pen } from "mrlyjs/view";

export const units = { math };

const TOP = 55;
const SIZE = 512;
const TURN = 1;
const LEVELS = 16;
const MARGIN = 0.05;
const WANT = { layers: 28, centre: 0.5, period: 90 };

type Spun = { data: Float32Array; low: number; high: number };

let held: Spun | null = null;

function spun(): Spun {
  if (held) return held;
  const layers = math.tourbillon.layers(TOP, "degrees", TURN, "odd", "plain", 1);
  const data = math.tourbillon.field(TOP, SIZE, "degrees", TURN, "odd", "plain", "cells", "mean", 1);
  const read = math.tourbillon.stats(data, SIZE, TOP, "degrees", TURN, "odd", "plain", "mean", 1);
  if (layers.length !== WANT.layers || read.layers !== WANT.layers || read.centre !== WANT.centre || read.period !== WANT.period || data.length !== SIZE * SIZE) throw new Error(`app-stack: ${layers.length} layers, centre ${read.centre}, period ${read.period}, want ${WANT.layers}, ${WANT.centre}, ${WANT.period}`);
  held = { data, low: read.low, high: read.high };
  return held;
}

export default function draw(pen: Pen, ink: Ink) {
  const { data, low, high } = spun();
  const ramp = new ink.Ramp([ink.ground, ink.blue, ink.fg]);
  const tones = Array.from({ length: LEVELS }, (_, i) => ramp.at(i / (LEVELS - 1)));
  const colors = new Uint8ClampedArray(SIZE * SIZE * 4);
  const span = high - low || 1;
  for (let i = 0; i < data.length; i++) {
    const v = data[i];
    if (Number.isNaN(v)) continue;
    colors.set(tones[Math.min(LEVELS - 1, Math.floor(((v - low) / span) * LEVELS))], i * 4);
  }
  const box = pen.frame(MARGIN);
  pen.image(box.x, box.y, box.w, box.h, { shape: [SIZE, SIZE], colors });
}
