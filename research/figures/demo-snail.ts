import * as math from "mrlyjs/math";
import * as num from "mrlyjs/num";
import type { Ink, Pen } from "mrlyjs/view";

export const units = { math, num };

const TOP = 300;
const BASE = 2;
const CODE = 7;
const MARGIN = 0.07;
const LEVELS = [1, 2, 4, 8, 16, 32, 64, 128, 45];

type Shell = ReturnType<typeof num.spiral.snail>;
type Art = ReturnType<typeof math.two.create>;

let memo: { shell: Shell; art: (Art | null)[] } | undefined;

function build() {
  if (memo) return memo;
  const shell = num.spiral.snail(BASE, TOP, "Every");
  if (shell.tiles.length !== 300) throw new Error(`demo-snail: ${shell.tiles.length} tiles, want 300`);
  if (shell.primes !== 62) throw new Error(`demo-snail: ${shell.primes} primes, want 62`);
  if (shell.levels.join() !== LEVELS.join()) throw new Error(`demo-snail: levels ${shell.levels}, want ${LEVELS}`);
  if (shell.area !== 5345865n) throw new Error(`demo-snail: area ${shell.area}, want 5345865`);
  const art = shell.levels.map((_, level) => {
    if (level === 0) return null;
    const cell = math.two.create(CODE, BASE, level, 0, 2);
    if (cell.shape[0] !== cell.shape[1]) throw new Error(`demo-snail: level ${level} cell ${cell.shape}, want a square`);
    return cell;
  });
  memo = { shell, art };
  return memo;
}

export default function draw(pen: Pen, ink: Ink) {
  const { shell, art } = build();
  const frame = pen.area(MARGIN);
  const wide = shell.high[0] - shell.low[0];
  const tall = shell.high[1] - shell.low[1];
  if (wide !== 4608 || tall !== 4352) throw new Error(`demo-snail: box ${wide} by ${tall}, want 4608 by 4352`);
  const unit = Math.min(frame.w / wide, frame.h / tall);
  const left = frame.x + (frame.w - wide * unit) / 2;
  const foot = frame.y + (frame.h + tall * unit) / 2;
  const at = (x: number, y: number): [number, number] => [left + (x - shell.low[0]) * unit, foot - (y - shell.low[1]) * unit];
  const path = shell.tiles.map((tile) => {
    const half = tile.side / 2;
    return at(tile.x + half, tile.y + half);
  });
  pen.polyline(path, 1.6, ink.line);
  let drawn = 0;
  for (const tile of shell.tiles) {
    if (tile.level === 0) continue;
    const cell = art[tile.level]!;
    const side = cell.shape[1];
    const tone = tile.prime ? ink.blue : ink.dim;
    for (let row = 0; row < side; row++) {
      for (let col = 0; col < side; col++) {
        if (cell.types[row * side + col] === 0) continue;
        const [x, y] = at(tile.x + col, tile.y + (side - row));
        pen.rect(x, y, unit, unit, tone);
        drawn++;
      }
    }
  }
  if (drawn !== 631167) throw new Error(`demo-snail: ${drawn} cells drawn, want 631167`);
}
