import { expect, test } from 'bun:test';
import { defaults, describe, tidy } from './knobs.js';

const SPEC = [
  { key: 'seed', kind: 'number', def: 7, min: 0, step: 1, group: 'Seed' },
  { key: 'max', label: 'Jump speed', kind: 'slider', def: 14, min: 4, max: 30, step: 1, group: 'Jump' },
  { key: 'idle', label: 'Cruise', kind: 'slider', def: 0.015, min: 0, max: 0.1, step: 0.005, group: 'Jump' },
  { key: 'size', kind: 'slider', def: 16, min: 8, max: 32, step: 2, group: 'Board' },
  { key: 'wrap', kind: 'toggle', def: 1, group: 'Board' },
  { key: 'play', kind: 'segment', def: 'me', options: ['me', 'smart', 'silly'], group: 'Board' },
  { key: 'number', kind: 'pick', def: '', options: [['', 'mixed'], 3, 5, 7, 9], group: 'Board' },
  { key: 'tint', kind: 'text', def: '', group: 'Sky' },
  { key: 'set', kind: 'list', def: 'lds', options: ['l', 'd', 's'], sep: '', group: 'Sky' },
  { key: 'pieces', kind: 'list', def: '', options: ['carpet', 'net', 'void'], group: 'Sky' },
];

const one = (key, v) => tidy(SPEC, { [key]: v })[key];

test('defaults are each row def, and an empty value tidies to them', () => {
  const want = { seed: 7, max: 14, idle: 0.015, size: 16, wrap: 1, play: 'me', number: '', tint: '', set: 'lds', pieces: '' };
  expect(defaults(SPEC)).toEqual(want);
  expect(tidy(SPEC, {})).toEqual(want);
  expect(tidy(SPEC, undefined)).toEqual(want);
});

test('a number clamps to its range', () => {
  expect([one('max', 99), one('max', -3), one('max', '30'), one('seed', -5), one('seed', 4294967295)]).toEqual([30, 4, 30, 0, 4294967295]);
});

test('a number snaps to its step from min', () => {
  expect([one('size', 13), one('size', '15'), one('idle', 0.031), one('idle', 0.0174), one('max', 7.6)]).toEqual([14, 16, 0.03, 0.015, 8]);
});

test('empty, NaN and an unknown option take the default', () => {
  expect([one('max', ''), one('max', '  '), one('max', 'abc'), one('max', NaN), one('max', null), one('max', Infinity)]).toEqual([14, 14, 14, 14, 14, 14]);
  expect([one('play', 'clever'), one('play', ''), one('number', '4'), one('number', '5'), one('number', '')]).toEqual(['me', 'me', '', 5, '']);
});

test('a toggle is 0 or 1', () => {
  expect([one('wrap', '0'), one('wrap', 1), one('wrap', true), one('wrap', false), one('wrap', 5), one('wrap', 'yes')]).toEqual([0, 1, 1, 0, 1, 1]);
});

test('text is trimmed', () => {
  expect([one('tint', '  red '), one('tint', '   ')]).toEqual(['red', '']);
});

test('a list keeps allowed items once, in order', () => {
  expect([one('pieces', 'void.x.carpet.void'), one('pieces', ['net', 'net']), one('set', 'sxdl'), one('set', 'xyz')]).toEqual(['void.carpet', 'net', 'sdl', 'lds']);
});

test('unknown keys are dropped and values are numbers or strings', () => {
  const value = tidy(SPEC, { ghost: 1, wrap: true, max: '12' });
  expect('ghost' in value).toBe(false);
  expect(Object.values(value).every((v) => typeof v === 'number' || typeof v === 'string')).toBe(true);
});

test('describe returns the groups in first-seen order', () => {
  expect(describe(SPEC).map(({ name, rows }) => [name, rows.map((row) => row.key)])).toEqual([
    ['Seed', ['seed']],
    ['Jump', ['max', 'idle']],
    ['Board', ['size', 'wrap', 'play', 'number']],
    ['Sky', ['tint', 'set', 'pieces']],
  ]);
});
