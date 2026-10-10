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

const LIVE = [
  { key: 'dim', kind: 'segment', def: 2, options: [2, 3], group: 'Design' },
  { key: 'level', kind: 'slider', def: 3, min: 1, max: (value) => (value.dim === 3 ? 3 : 5), step: 1, group: 'Design' },
  { key: 'kind', kind: 'segment', def: 'moire', options: ['moire', 'star'], group: 'Stack' },
  { key: 'combine', kind: 'pick', def: 'sum', options: (value) => (value.kind === 'moire' ? ['sum', 'hive'] : ['sum']), group: 'Stack' },
  { key: 'rule', kind: 'slider', def: 110, min: 0, max: 255, step: 1, group: 'Rule', when: (value) => value.kind === 'star' },
  { key: 'born', kind: 'text', def: '3', group: 'Rule', when: (value) => value.kind === 'star' },
  { key: 'set', kind: 'list', def: 'l', options: (value) => (value.kind === 'moire' ? ['l', 'd'] : ['l']), sep: '', group: 'Sky' },
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

test('a max that is a function of the value clamps by the other keys, tidied first', () => {
  expect([tidy(LIVE, { dim: 2, level: 5 }).level, tidy(LIVE, { dim: 3, level: 5 }).level, tidy(LIVE, { dim: '3', level: '9' }).level, tidy(LIVE, { dim: 7, level: 5 }).level]).toEqual([5, 3, 3, 5]);
  expect(tidy(LIVE, defaults(LIVE))).toEqual(defaults(LIVE));
});

test('a function max or options reads only tidied keys, so a raw url value cannot reach it', () => {
  const seen = [];
  const spec = [
    { key: 'dim', kind: 'segment', def: 2, options: [2, 3] },
    { key: 'level', kind: 'slider', def: 3, min: 1, max: (value) => (seen.push(value.dim), 5), step: 1 },
    { key: 'combine', kind: 'pick', def: 'sum', options: (value) => (seen.push(value.dim), ['sum', 'hive']) },
  ];
  expect(tidy(spec, { dim: 0, level: 4, combine: 'hive' })).toEqual({ dim: 2, level: 4, combine: 'hive' });
  expect(tidy(spec, { dim: '', level: 9 }).level).toBe(5);
  expect(tidy(spec, { dim: -1 }).dim).toBe(2);
  expect(new Set(seen)).toEqual(new Set([2]));
});

test('a function row reads the function rows before it tidied and those after it at their defaults, and tidy stays a fixed point', () => {
  const spec = [
    { key: 'dim', kind: 'segment', def: 2, options: [2, 3] },
    { key: 'start', kind: 'slider', def: 10, min: 10, max: (value) => value.finish, step: 10 },
    { key: 'level', kind: 'slider', def: 1, min: 1, max: (value) => (value.dim === 3 ? 3 : 5), step: 1 },
    { key: 'finish', kind: 'slider', def: 20, min: 10, max: (value) => value.level * 10, step: 10 },
  ];
  const once = tidy(spec, { dim: 2, start: 90, level: 4, finish: 90 });
  expect([once, tidy(spec, { dim: 3, level: 5, finish: 90 }).finish, tidy(spec, { level: 'x', finish: 90 }).finish]).toEqual([{ dim: 2, start: 20, level: 4, finish: 40 }, 30, 10]);
  expect(tidy(spec, once)).toEqual(once);
});

test('a missing or junk number takes its default clamped to its range, so tidy stays a fixed point', () => {
  const spec = [
    { key: 'number', kind: 'segment', def: 3, options: [3, 5] },
    { key: 'level', kind: 'slider', def: 4, min: 1, max: (value) => (value.number === 5 ? 2 : 4), step: 1 },
  ];
  const once = tidy(spec, { number: 5 });
  expect([once.level, tidy(spec, { number: 5, level: 'x' }).level, tidy(spec, { level: '' }).level, tidy(spec, once)]).toEqual([2, 2, 4, once]);
});

test('options that are a function of the value admit by the other keys, for a pick and for a list', () => {
  expect([tidy(LIVE, { kind: 'moire', combine: 'hive' }).combine, tidy(LIVE, { kind: 'star', combine: 'hive' }).combine, tidy(LIVE, { kind: 'bogus', combine: 'hive' }).combine]).toEqual(['hive', 'sum', 'hive']);
  expect([tidy(LIVE, { kind: 'moire', set: 'dl' }).set, tidy(LIVE, { kind: 'star', set: 'dl' }).set, tidy(LIVE, { kind: 'star', set: 'd' }).set]).toEqual(['dl', 'l', 'l']);
});

test('a row whose when is false leaves describe, and an emptied group goes with it', () => {
  const keys = (value) => describe(LIVE, value).map(({ name, rows }) => [name, rows.map((row) => row.key)]);
  expect(keys({ kind: 'star' })).toEqual([['Design', ['dim', 'level']], ['Stack', ['kind', 'combine']], ['Rule', ['rule', 'born']], ['Sky', ['set']]]);
  expect(keys({ kind: 'moire' })).toEqual([['Design', ['dim', 'level']], ['Stack', ['kind', 'combine']], ['Sky', ['set']]]);
  expect(tidy(LIVE, { kind: 'moire', rule: 9, born: '36' })).toMatchObject({ rule: 9, born: '36' });
});
