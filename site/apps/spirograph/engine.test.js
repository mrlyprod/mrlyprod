import { expect, test } from 'bun:test';
import * as math from '../../../pkgs/mrlyjs/math.js';
import { defaults } from '../../lib/knobs.js';
import { rng } from '../../lib/scene.js';
import { BANDS, COVER, DENSITY, HOLD, LEAST, MOST, RISE, bands, bits, design, fit, legal, roll, seat, shade, share, span, study } from './engine.js';
import { SPEC } from './scene.js';

math.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/math/mrlyjs_math_bg.wasm', import.meta.url)).arrayBuffer() });

const value = (over) => ({ ...defaults(SPEC), seed: 1, ...over });

test('a rolled code is a base-3 plane design with three fills or more, the same for one seed', () => {
  const codes = Array.from({ length: 50 }, (_, i) => roll(rng(i)));
  expect(codes.every((code) => code >= 1 && code <= 510 && bits(code) >= 3)).toBe(true);
  expect(roll(rng(9))).toBe(roll(rng(9)));
});

test('a readable code is kept, an unreadable or empty one gives the seeded design', () => {
  expect(design(math, value({ code: '495' }), rng(1))).toBe('495');
  expect(design(math, value({ code: 'carpet' }), rng(1))).toBe(String(roll(rng(1))));
  expect(design(math, value({ code: '' }), rng(1))).toBe(String(roll(rng(1))));
});

test('a level the crate refuses to seat steps down until it seats', () => {
  const corners = seat(math, value({ number: 7, level: 3, mode: 'corners' }), '495');
  expect([corners.level, corners.pencils.length, corners.width]).toEqual([2, 2356, 49]);
  const small = seat(math, value({ number: 3, level: 3, mode: 'corners' }), '495');
  expect([small.level, small.pencils.length]).toEqual([3, 688]);
});

test('a wheel inside a ring is made smaller than it', () => {
  expect(legal('in', 7, 3, 4)).toEqual([7, 3]);
  expect(legal('in', 3, 7, 4)).toEqual([3, 2]);
  expect(legal('in', 1, 1, 4)).toEqual([2, 1]);
  expect(legal('out', 3, 7, 4)).toEqual([3, 7]);
});

test('a wheel inside a polygon is made to fit, the crate agreeing on every setting the sliders allow', () => {
  expect(legal('polyin', 7, 3, 4)).toEqual([7, 3]);
  expect(legal('polyin', 5, 3, 3)).toEqual([5, 2]);
  expect(legal('polyin', 4, 3, 4)).toEqual([4, 2]);
  expect(legal('polyin', 6, 3, 3)).toEqual([6, 2]);
  expect(legal('polyin', 1, 1, 3)).toEqual([3, 1]);
  const s = math.spirograph;
  for (let ring = 1; ring <= 24; ring++) {
    for (let wheel = 1; wheel <= 24; wheel++) {
      for (let sides = 3; sides <= 12; sides++) {
        const [r, w] = legal('polyin', ring, wheel, sides);
        expect([ring, wheel, sides, r >= ring, w >= 1 && w <= wheel, s.track('polyin', r, w, sides, 1).wheel]).toEqual([ring, wheel, sides, true, true, w]);
        if (w < wheel && r * Math.cos(Math.PI / sides) - (w + 1) < -1e-6) expect(() => s.track('polyin', r, w + 1, sides, 1)).toThrow();
      }
    }
  }
});

test('the carpet of 3 inside 7 over 3 seats 8 pencils on 8 curves with 1288 nodes and a shape on the disc', () => {
  const plan = study(math, value({ code: '495' }), rng(1));
  expect(plan.facts).toMatchObject({ code: '495', level: 1, pencils: 8, fills: 8, distinct: 8, nodes: 1288, ratio: [7, 3], orbits: 3, turns: 4, round: true });
  expect(plan.facts.cover).toBeGreaterThan(0.7);
  expect(plan.trace.length).toBe(8 * plan.samples * 2);
  expect([plan.beats, plan.samples]).toEqual([4, Math.ceil(DENSITY * 4) + 1]);
  expect([plan.shape.side, plan.shape.mask.length, plan.shape.covered]).toEqual([COVER.side, COVER.side ** 2, plan.facts.cover]);
  expect(plan.shape.disc).toMatchObject({ x: 0, y: 0 });
  expect(plan.shape.disc.radius).toBeCloseTo(5.8);
});

test('the samples follow the orbits of the centre when they outnumber the turns of the wheel', () => {
  const plan = study(math, value({ code: '495', ring: 24, wheel: 23 }), rng(1));
  expect([plan.turns, plan.facts.orbits, plan.beats, plan.samples]).toEqual([1, 23, 23, Math.ceil(DENSITY * 23) + 1]);
});

test('a line track has no nodes, no cover and no shape, and a full design seats no void pencil', () => {
  const line = study(math, value({ code: '495', kind: 'line' }), rng(1));
  expect([line.facts.nodes, line.facts.cover, line.facts.round, line.shape]).toEqual([null, null, false, null]);
  const none = study(math, value({ code: '511', mode: 'void' }), rng(1));
  expect([none.facts.pencils, none.trace.length, none.facts.distinct, none.shape]).toEqual([0, 0, 0, null]);
});

test('a polygon too tight for the wheel still lays a track', () => {
  const plan = study(math, value({ code: '495', kind: 'polyin', ring: 5, wheel: 3, sides: 3 }), rng(1));
  expect([plan.track.kind, plan.track.wheel, plan.facts.pencils]).toEqual(['polyin', 2, 8]);
});

test('the span runs at half a beat a second between three seconds and a minute', () => {
  expect([span(1), span(7), span(23), span(437)]).toEqual([LEAST, 14000, 46000, MOST]);
});

test('the share drawn rises to one over the span, holds, then starts over', () => {
  const length = span(7);
  expect(share(0, length, false)).toBe(0);
  expect(share(length / 2, length, false)).toBe(0.5);
  expect(share(length, length, false)).toBe(1);
  expect(share(length + HOLD / 2, length, false)).toBe(1);
  expect(share(length + HOLD + length / 4, length, false)).toBe(0.25);
  expect(share(0, length, true)).toBe(1);
});

test('the shape fades in over the first second of the hold and is whole in a still', () => {
  const length = span(7);
  expect([shade(length / 2, length, false), shade(length, length, false)]).toEqual([0, 0]);
  expect(shade(length + RISE / 2, length, false)).toBe(0.5);
  expect([shade(length + RISE, length, false), shade(length + HOLD - 1, length, false)]).toEqual([1, 1]);
  expect(shade(length + HOLD, length, false)).toBe(0);
  expect(shade(0, length, true)).toBe(1);
});

test('seats nearer the rim take the higher band', () => {
  const seats = bands([{ x: 0.1, y: 0 }, { x: 0.5, y: 0.5 }, { x: 0, y: 1 }]);
  expect(seats).toEqual([0, Math.floor(((Math.SQRT1_2 - 0.1) / 0.9) * BANDS), BANDS - 1]);
  expect(bands([{ x: 0.3, y: 0 }, { x: 0, y: 0.3 }])).toEqual([BANDS - 1, BANDS - 1]);
});

test('the fit centres the frame on the stage inside the padding', () => {
  const box = fit([-7, -7, 7, 7], 400, 300, 10);
  expect(box.k).toBe(20);
  expect([box.ox, box.oy]).toEqual([200, 150]);
  expect([box.ox + 7 * box.k, box.oy - 7 * box.k]).toEqual([340, 10]);
});
