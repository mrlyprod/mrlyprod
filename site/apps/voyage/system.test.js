import { expect, test } from 'bun:test';
import { codes } from '../../lib/space/bang.js';
import { world } from '../../lib/space/planet.js';
import { NAMES, kind, system } from './system.js';

test('one seed gives one system and another seed another', () => {
  expect(system(7, { worlds: 6 })).toEqual(system(7, { worlds: 6 }));
  expect(system(7, { worlds: 6 })).not.toEqual(system(8, { worlds: 6 }));
});

test('the star burns at 3500 to 9000 K and the worlds circle at 0.35 x 1.75^k AU within 15%', () => {
  for (let seed = 0; seed < 40; seed++) {
    for (const worlds of [3, 5, 9]) {
      const { star, bodies: all } = system(seed, { worlds, bangs: 4 });
      const bodies = all.filter((one) => one.kind === 'planet');
      const off = bodies.map((one, k) => Math.abs(Math.hypot(...one.pos) / (0.35 * 1.75 ** k) - 1));
      expect([seed, worlds, bodies.length, star.temp >= 3500 && star.temp <= 9000, Math.max(...off) <= 0.15 + 1e-9]).toEqual([seed, worlds, worlds, true, true]);
    }
  }
});

test('names are unique: the star of two or three syllables, its planets B onward by orbit', () => {
  for (let seed = 0; seed < 40; seed++) {
    const { star, bodies } = system(seed, { worlds: 9 });
    const names = [star.name, ...bodies.map((one) => one.name)];
    expect([seed, new Set(names).size, /^[A-Z]{4,9}$/.test(star.name), bodies.map((one) => one.name.slice(star.name.length)).join('')]).toEqual([seed, 10, true, ' B C D E F G H I J']);
  }
});

test('each type comes from the exo weights, gas likelier past 2 AU, and its world rolls that type', () => {
  const near = { gas: 0, all: 0 };
  const far = { gas: 0, all: 0 };
  for (let seed = 0; seed < 120; seed++) {
    for (const one of system(seed, { worlds: 7 }).bodies) {
      expect([one.type, world('exo', one.world).kind]).toEqual([one.type, one.type]);
      const side = one.a > 2 ? far : near;
      side.all += 1;
      if (one.type === 'gas') side.gas += 1;
    }
  }
  expect(far.gas / far.all).toBeGreaterThan(1.8 * (near.gas / near.all));
});

test('bang worlds sit at fixed directions 20 to 60 degrees off the ecliptic, gated codes, a quarter at n 5, pure by seed', () => {
  const rows = [];
  const fives = [];
  for (let seed = 0; seed < 200; seed++) {
    const { bodies } = system(seed, { worlds: 5, bangs: 4 });
    const deep = bodies.filter((one) => one.kind === 'bang');
    for (const one of deep) {
      const lat = (Math.asin(one.dir[1]) * 180) / Math.PI;
      rows.push([Math.abs(Math.hypot(...one.dir) - 1) < 1e-9, Math.abs(lat) >= 20 - 1e-9 && Math.abs(lat) <= 60 + 1e-9, codes(one.n).includes(one.code), one.pos.every((v, i) => Math.abs(v - one.dir[i] * one.a) < 1e-9)]);
      fives.push(one.n === 5);
    }
    expect([seed, deep.length, deep.map((one) => one.id), new Set(deep.map((one) => one.code)).size]).toEqual([seed, 4, [5, 6, 7, 8], 4]);
  }
  expect(rows.every((row) => row.every(Boolean))).toBe(true);
  expect(Math.abs(fives.filter(Boolean).length / fives.length - 0.25)).toBeLessThan(0.06);
  expect(system(2, { worlds: 5, bangs: 4 })).toEqual(system(2, { worlds: 5, bangs: 4 }));
  expect(system(2, { worlds: 5, bangs: 4 }).bodies.slice(5)).not.toEqual(system(3, { worlds: 5, bangs: 4 }).bodies.slice(5));
  expect(system(2, { worlds: 5, bangs: 4 }).bodies.slice(0, 5)).toEqual(system(2, { worlds: 5, bangs: 0 }).bodies);
});

test('a bang world is named by its mrly name and code, else BANG and its code, with N and its n', () => {
  const seen = new Map();
  for (let seed = 0; seed < 300; seed++) {
    for (const one of system(seed, { worlds: 3, bangs: 4 }).bodies.filter((body) => body.kind === 'bang')) {
      expect(kind(one)).toBe(`n ${one.n}`);
      seen.set(one.code, one.name);
    }
  }
  const want = [...seen.keys()].map((code) => [code, NAMES[code] ? `${NAMES[code].toUpperCase()} ${code}` : `BANG ${code}`]);
  expect([...seen.entries()]).toEqual(want);
  expect([seen.get(23), [...seen.values()].some((name) => name.startsWith('BANG '))]).toEqual(['CARPET 23', true]);
});
