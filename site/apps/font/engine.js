import { pick } from '../../lib/scene.js';

export const LIMIT = 80;

export const SAMPLES = ['hello, world!', 'mrlyprod', 'simple, beautiful, useful', 'one desk', 'the 108 pens', 'mrly.net'];

export const KINDS = [
  { id: 'upper', name: 'Uppers', door: 'uppers' },
  { id: 'lower', name: 'Lowers', door: 'lowers' },
  { id: 'digit', name: 'Digits', door: 'digits' },
  { id: 'extra', name: 'Extras', door: 'extras' },
  { id: 'special', name: 'Specials', door: 'specials' },
];

export const sample = (rand) => pick(rand, SAMPLES);

export const cut = (text) => String(text ?? '').slice(0, LIMIT);

export function slug(text) {
  const words = cut(text).toLowerCase().replace(/[^a-z0-9]+/g, '-').replace(/^-+|-+$/g, '');
  return words.slice(0, 24).replace(/-+$/, '') || 'text';
}

/* PLAN */

export function plan(font, { text, pad = 1, hold = 25, loop = 1 }) {
  const written = cut(text);
  const write = font.animate(written, pad);
  const anim = loop ? font.cycle(write, font.merge(written, pad), hold) : write;
  return { text: written, rows: anim.rows, cols: anim.cols, fps: anim.fps, frames: anim.frames, still: write.frames.length - 1 };
}

export function at({ frames, fps }, t, speed = 1, loop = 1) {
  const n = frames.length;
  if (n < 2) return 0;
  const k = Math.floor((t / 1000) * fps * speed + 1e-9);
  return loop ? k % n : Math.min(k, n - 1);
}

/* GALLERY */

function lit(rows) {
  const cells = [];
  rows.forEach((row, r) => {
    for (let c = 0; c < row.length; c++) if (row[c] === '1') cells.push(r * row.length + c);
  });
  return cells;
}

const lead = (rows) => Math.max(0, [...(rows[0] ?? '')].findIndex((_, c) => rows.some((row) => row[c] === '1')));

export function study(font, glyph) {
  const { char, rows } = glyph;
  const cols = rows[0]?.length ?? 0;
  const strokes = font.strokes(char).length;
  const name = font.name_of(char).toLowerCase();
  const order = font.path(char).map(([r, c]) => r * cols + c + lead(rows));
  return { char, label: char.trim() ? char : name, name, rows: rows.length, cols, cells: lit(rows), order, strokes, floor: font.floor(font.trim(rows)), lifts: Math.max(0, strokes - 1) };
}

export function gallery(font) {
  return KINDS.map((kind) => ({
    ...kind,
    glyphs: font[kind.door]().map((glyph) => {
      const seen = study(font, glyph);
      glyph.free?.();
      return seen;
    }),
  }));
}
