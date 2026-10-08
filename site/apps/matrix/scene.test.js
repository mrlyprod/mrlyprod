import { expect, test } from 'bun:test';
import FONT from '../../kit/font/font.json' with { type: 'json' };
import { rng } from '../../lib/scene.js';
import { fell, glyphs, make } from './scene.js';

function stage(still) {
  const marks = [];
  const ctx = { fillText: (text, x, y) => marks.push([text, x, y]), fillRect() {}, setTransform() {} };
  const canvas = { getContext: () => ctx };
  const view = { rand: rng(7), look: () => ({ paper: '#000000', accent: '#00ff00' }), w: 100, h: 100, dpr: 1, t: 0, still };
  return { marks, scene: make(canvas, view, {}) };
}

test('a matrix column resets past the floor on a rare roll', () => {
  expect(fell(101, 100, 0.98)).toBe(true);
  expect(fell(101, 100, 0.97)).toBe(false);
  expect(fell(99, 100, 0.99)).toBe(false);
});

test('the glyph pool is the chosen sets of MrlyFont', () => {
  expect(glyphs('d').join('')).toBe('0123456789');
  expect(glyphs('ld').length).toBe(36);
  const all = glyphs('lds');
  expect([all.includes('#'), all.includes('Z'), all.includes('a'), all.includes(' ')]).toEqual([true, true, false, false]);
});

test('every pooled glyph is 5 by 5', () => {
  for (const char of glyphs('lds')) {
    const { rows } = FONT[char];
    expect([char, rows.length, rows.every((row) => row.length === 5)]).toEqual([char, 5, true]);
  }
  expect([glyphs('l').length, glyphs('d').length, glyphs('s').length]).toEqual([26, 10, 35]);
});

test('matrix opens with a drop in every column, then the trickle', () => {
  const play = () => {
    const { marks, scene } = stage(false);
    const lit = [];
    const seen = [];
    for (let frame = 0; frame < 300; frame++) {
      marks.length = 0;
      scene.draw();
      lit.push(new Set(marks.map(([, x]) => x)).size);
      seen.push(...marks);
    }
    return { lit, seen };
  };
  const { lit, seen } = play();
  expect(lit.slice(0, 5)).toEqual([5, 5, 5, 5, 5]);
  expect(lit[5]).toBeLessThan(5);
  expect(Math.max(...lit.slice(6))).toBeGreaterThan(0);
  expect(play().seen).toEqual(seen);
});

test('under reduced motion the still is the full field, the same on every draw', () => {
  const { marks, scene } = stage(true);
  scene.draw();
  const first = [...marks];
  expect([first.length, new Set(first.map(([, x]) => x)).size, new Set(first.map(([, , y]) => y)).size]).toEqual([25, 5, 5]);
  marks.length = 0;
  scene.draw();
  expect(marks).toEqual(first);
});
