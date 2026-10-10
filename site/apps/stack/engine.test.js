import { expect, test } from 'bun:test';
import * as math from '../../../pkgs/mrlyjs/math.js';
import { defaults } from '../../lib/knobs.js';
import { rng } from '../../lib/scene.js';
import { BEAT, HOLD, LEAST, LIVE, READ, SETS, SWEEP, WEIGHTS, angle, bins, colorizer, design, eyeOf, field, label, live, odd, paint, shown, spun, study } from './engine.js';
import { DESIGN, SPEC } from './scene.js';

math.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/math/mrlyjs_math_bg.wasm', import.meta.url)).arrayBuffer() });

const value = (over) => ({ ...defaults(SPEC), ...DESIGN, seed: 1, ...over });

const plain = () => 0;

const LOOK = { paper: '#ffffff', accent: '#008cff' };

const nans = (data) => data.reduce((sum, v) => sum + (Number.isNaN(v) ? 1 : 0), 0);

test('the odd scales run from one to the limit, an even limit rounded up', () => {
  expect(odd(9)).toEqual([1, 3, 5, 7, 9]);
  expect(odd(8)).toEqual([1, 3, 5, 7, 9]);
  expect(odd(1)).toEqual([1]);
});

test('the turn sweeps from the increment at the sweep rate and holds it in a still', () => {
  expect(angle(1, 0, false)).toBe(1);
  expect(angle(1, 1000, false)).toBe(1 + SWEEP);
  expect(angle(359, (2000 / SWEEP) * 1000 / 1000, false)).toBeCloseTo(1);
  expect(angle(22.5, 5000, true)).toBe(22.5);
});

test('the layers shown climb one a beat, hold at the top, then start over, and a still shows them all', () => {
  expect(shown(5, 0, false)).toBe(1);
  expect(shown(5, BEAT * 2 + 1, false)).toBe(3);
  expect(shown(5, BEAT * 5 + HOLD / 2, false)).toBe(5);
  expect(shown(5, BEAT * 5 + HOLD + 1, false)).toBe(1);
  expect(shown(5, 0, true)).toBe(5);
});

test('a ramp is one colour a level, paper to accent, through the contrast of the paper, or in bands', () => {
  expect(bins('tint', 3, '#ffffff', '#008cff')).toEqual(['#ffffff', '#80c6ff', '#008cff']);
  expect(bins('ink', 3, '#ffffff', '#008cff')).toEqual(['#ffffff', '#008cff', '#000000']);
  expect(bins('ink', 2, '#000000', '#008cff').at(-1)).toBe('#ffffff');
  expect(bins('bands', 5, 'rgb(255, 255, 255)', '#008cff')).toEqual(['#ffffff', '#008cff', '#ffffff', '#008cff', '#ffffff']);
  expect(colorizer('tint', 2, LOOK)).toEqual({ Bins: { background: '#ffffff', ramp: ['#ffffff', '#008cff'] } });
});

test('a typed code is kept modulo the space, a blank one rolls from the seed, the level is clamped', () => {
  expect(design(math, { base: 3, code: '495', level: 1 }, rng(1))).toEqual({ base: 3, code: '495', level: 1, name: 'carpet' });
  expect(design(math, { base: 2, code: '23', level: 9 }, rng(1))).toMatchObject({ base: 2, code: '7', level: 5, name: 'carpet' });
  expect(design(math, { base: 3, code: '', level: 0 }, rng(1))).toEqual(design(math, { base: 3, code: 'abc', level: -2 }, rng(1)));
  expect(design(math, { base: 3, code: '', level: 1 }, rng(1)).code).not.toBe(design(math, { base: 3, code: '', level: 1 }, rng(2)).code);
});

test('the moire plan reads the witness of the top scale: 41 is prime, 9 shares a factor with 3', () => {
  const prime = study(math, value({ kind: 'moire', code: '495', limit: 41 }), rng(1));
  expect(prime.facts).toMatchObject({ kind: 'moire', code: '495', name: 'carpet', base: 3, level: 1, layers: 21, limit: 41, max: 0, at: 0, prime: true });
  expect([prime.merge, prime.lattice, prime.level]).toEqual(['Sum', 'Square', 1]);
  const nine = study(math, value({ kind: 'moire', code: '495', limit: 9, combine: 'hive' }), rng(1));
  expect([nine.facts.prime, nine.facts.at, nine.lattice, nine.merge]).toEqual([false, 3, 'Hex', 'Sum']);
  expect(nine.facts.max).toBeCloseTo(0.2192645, 6);
});

test('the tourbillon plan reads the period, the classes, the pairs and the stats of the increment', () => {
  const plan = study(math, value({ kind: 'tourbillon', limit: 21, increment: 1 }), plain);
  expect(plan.facts).toMatchObject({ kind: 'tourbillon', layers: 11, limit: 21, set: 'odd', weights: 'plain', period: 90, classes: 11, pairs: 0 });
  expect(plan.facts.centre).toBeCloseTo(5 / 11);
  expect(plan.facts.mean).toBeGreaterThan(0);
  const eye = study(math, value({ kind: 'tourbillon', limit: 21, increment: 22.5 }), plain);
  expect([eye.facts.period, eye.facts.classes, eye.facts.pairs]).toEqual([4, 4, 10]);
  expect(study(math, value({ kind: 'tourbillon', limit: 21, increment: 0.1 }), plain).facts.period).toBe(null);
});

test('the seed rolls the layer set and the weights of a tourbillon, and the plan reads the layers the set keeps', () => {
  expect(spun(plain)).toEqual({ set: 'odd', weights: 'plain' });
  expect(spun(() => 0.999)).toEqual({ set: SETS.at(-1), weights: WEIGHTS.at(-1) });
  expect(spun(rng(1))).not.toEqual(spun(rng(5)));
  const primes = study(math, value({ kind: 'tourbillon', limit: 41 }), () => 0.3);
  expect([primes.set, primes.weights, primes.layers, primes.facts.layers, primes.facts.scales.slice(0, 4)]).toEqual(['primes', 'plain', 12, 12, [3, 5, 7, 11]]);
  const mobius = study(math, value({ kind: 'tourbillon', limit: 41 }), () => 0.5);
  expect([mobius.set, mobius.weights, mobius.facts.mean < 0]).toEqual(['squarefree', 'mobius', true]);
});

test('the star plan reads the decay of the ghost star over its cuts, the slope only at a count divisible by four', () => {
  const plan = study(math, value({ kind: 'star', limit: 41 }), rng(1));
  expect(plan.facts).toMatchObject({ kind: 'star', layers: 21, limit: 41, slope: null, branch: 'odd' });
  expect(plan.facts.scaled).toBeCloseTo(-0.918065, 5);
  const four = study(math, value({ kind: 'star', limit: 39 }), rng(1));
  expect(four.facts.slope).toBeCloseTo(-0.25226, 4);
});

test('a moire field stacks the first k scales on a square of the size', () => {
  const plan = study(math, value({ kind: 'moire', code: '495', limit: 41 }), rng(1));
  const one = field(math, plan, 1, 64);
  expect([one.size, one.data.length, one.field.max()]).toEqual([64, 4096, 1]);
  const all = field(math, plan, 21, 64);
  expect([all.field.max(), nans(all.data), all.field.min() >= 11 && all.field.min() <= 13]).toEqual([21, 0, true]);
  one.field.free();
  all.field.free();
});

test('a tourbillon field is masked to the disc and a star field to the hexagon, the outside NaN', () => {
  const disc = field(math, study(math, value({ kind: 'tourbillon', limit: 21 }), rng(1)), 1, 64);
  expect([disc.size, nans(disc.data) > 0, nans(disc.data) < 4096 / 4, Number.isNaN(disc.data[0])]).toEqual([64, true, true, true]);
  const cut = field(math, study(math, value({ kind: 'star', limit: 9 }), rng(1)), 5, 64);
  const inside = 4096 - nans(cut.data);
  expect([cut.size, Number.isNaN(cut.data[0]), inside > 4096 * 0.4, inside < 4096 * 0.6]).toEqual([64, true, true, true]);
  expect([cut.field.max(), cut.field.min()]).toEqual([5, 1]);
  disc.field.free();
  cut.field.free();
});

test('a star cut is exact at every scale: the hexagon ink of the deepest layer alone is the cut ink law', () => {
  const star = new math.six.star.Star('23');
  for (const limit of [41, 99]) {
    const plan = study(math, value({ kind: 'star', limit }), rng(1));
    const only = { ...plan, numbers: [limit] };
    const cut = field(math, only, 1, 384);
    let inked = 0;
    let read = 0;
    for (const v of cut.data) {
      if (Number.isNaN(v)) continue;
      read++;
      inked += v;
    }
    expect(inked / read).toBeCloseTo(star.hexagon(limit).value(), 2);
    cut.field.free();
  }
  star.free();
  const meet = field(math, study(math, value({ kind: 'star', limit: 9, combine: 'and' }), rng(1)), 5, 64);
  const parity = field(math, study(math, value({ kind: 'star', limit: 9, combine: 'xor' }), rng(1)), 5, 64);
  expect([meet.field.max(), meet.field.min(), parity.field.max(), parity.field.min()]).toEqual([1, 0, 1, 0]);
  meet.field.free();
  parity.field.free();
});

test('the paint is one colour a level with a clear alpha wherever the field is NaN', () => {
  const plan = study(math, value({ kind: 'tourbillon', limit: 21 }), rng(1));
  const made = field(math, plan, 1, 32);
  const image = paint(math, made, colorizer('tint', 4, LOOK), 4, 0);
  expect(image.shape).toEqual([32, 32]);
  expect(image.colors[3]).toBe(0);
  const centre = (16 * 32 + 16) * 4;
  expect(Array.from(image.colors.slice(centre, centre + 4))).toEqual([0, 140, 255, 255]);
  const flipped = paint(math, made, colorizer('tint', 4, LOOK), 4, 1);
  expect(Array.from(flipped.colors.slice(centre, centre + 4))).toEqual([255, 255, 255, 255]);
  made.field.free();
});

test('an eye names its quarter turn and the increment finds its eye on the lattice', () => {
  const eyes = math.tourbillon.eyes(6);
  expect(label(eyes[0])).toBe('0 deg');
  expect(label(eyes.find((eye) => eye.angle === 22.5))).toBe('22.5 deg, 90/4');
  expect(eyeOf(eyes, 22.5)).toBe(String(eyes.findIndex((eye) => eye.angle === 22.5)));
  expect(eyeOf(eyes, 22.4)).toBe('');
  expect(eyes.every((eye) => Number.isInteger(eye.angle * 2))).toBe(true);
});

test('the live raster shrinks with the layer count so a frame costs what the default does, and never under the least', () => {
  expect([live(1), live(11), live(21), live(50), live(1000)]).toEqual([LIVE, LIVE, LIVE, 124, LEAST]);
  expect(live(25) % 2).toBe(0);
  expect(READ).toBeLessThanOrEqual(128);
});
