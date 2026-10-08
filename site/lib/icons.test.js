import { expect, test } from 'bun:test';
import { ICONS } from './icons.js';

const NAMES = ['close', 'export', 'knobs', 'paneLeft', 'paneRight', 'reroll'];

const ARITY = { m: 2, l: 2, h: 1, v: 1, c: 6, s: 4, q: 4, t: 2, a: 7, z: 0 };

const NUMBER = /^-?(?:\d+\.?\d*|\.\d+)$/;

function parse(d) {
  const segments = [...d.matchAll(/([a-z])([^a-z]*)/gi)];
  if (segments.map((s) => s[0]).join('') !== d) return false;
  return segments.every(([, cmd, args]) => {
    const arity = ARITY[cmd.toLowerCase()];
    const numbers = args.split(/[\s,]+|(?=-)/).filter(Boolean);
    const sized = arity === 0 ? numbers.length === 0 : numbers.length > 0 && numbers.length % arity === 0;
    return arity !== undefined && sized && numbers.every((n) => NUMBER.test(n) && Math.abs(Number(n)) <= 24);
  });
}

test('the six icons are paths that start with M, parse, and sit on the 24 grid', () => {
  expect(Object.keys(ICONS).sort()).toEqual(NAMES);
  for (const name of NAMES) {
    expect(ICONS[name].startsWith('M')).toBe(true);
    expect(parse(ICONS[name])).toBe(true);
  }
});
