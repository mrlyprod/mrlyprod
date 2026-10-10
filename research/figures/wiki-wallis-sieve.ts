import * as num from "mrlyjs/num";
import type { Ink, Pen } from "mrlyjs/view";

export const units = { num };

const LEVELS = 3;
const SPAN = 840;

export default function draw(pen: Pen, ink: Ink) {
  const word = num.sieve.odd_word(LEVELS);
  const [side, sites] = num.sieve.raster(word);
  const holes = num.sieve.punctures(word, 2);
  if (side !== 105) throw new Error(`wiki-wallis-sieve: side ${side}, want 105`);
  const alive = sites.filter((bit) => bit === 1).length;
  if (alive !== 9216) throw new Error(`wiki-wallis-sieve: ${alive} surviving sites, want 9216`);
  if (holes.length !== 3 * 201) throw new Error(`wiki-wallis-sieve: ${holes.length / 3} holes, want 201`);
  const tones = [ink.yellow, ink.orange, ink.blue];
  const sizes = [...new Set(Array.from({ length: holes.length / 3 }, (_, hole) => Number(holes[3 * hole + 2])))].sort((a, b) => b - a);
  if (sizes.length !== tones.length) throw new Error(`wiki-wallis-sieve: ${sizes.length} hole widths, want ${tones.length}`);
  const unit = SPAN / side;
  const edge = (pen.width - SPAN) / 2;
  pen.rect(edge, edge, SPAN, SPAN, ink.line);
  for (let i = 0; i < holes.length; i += 3) {
    const wide = Number(holes[i + 2]);
    pen.rect(edge + Number(holes[i + 1]) * unit, edge + Number(holes[i]) * unit, wide * unit, wide * unit, tones[sizes.indexOf(wide)]);
  }
}
