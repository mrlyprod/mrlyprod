import { expect, test } from 'bun:test';
import * as math from '@mrlyprod/mrly/math';
import { defaults } from '../../lib/app/spec.js';
import { rng } from '../../lib/rand.js';
import { OUT } from '../../lib/spin/engine.js';
import { PAD, SOURCE } from '../../lib/spin/scene.js';
import { PAGE, SPEC, make, units } from './scene.js';

const pen = (log) =>
  new Proxy(
    {},
    {
      get: (_, key) => (...args) => log.push([key, ...args.map((arg) => (arg && typeof arg === 'object' ? 'layer' : arg))]),
      set: (_, key, v) => {
        log.push([key, v]);
        return true;
      },
    },
  );

const images = [];

globalThis.OffscreenCanvas = class {
  constructor(w, h) {
    this.width = w;
    this.height = h;
  }
  getContext() {
    return pen([]);
  }
};

globalThis.ImageData = class {
  constructor(data, w, h) {
    this.data = data;
    this.width = w;
    images.push(this);
  }
};

const open = (value, { unit = math, w = 1600, h = 900 } = {}) => {
  const log = [];
  const facts = [];
  const view = { rand: rng(1), look: () => ({ paper: '#000000', accent: '#008cff' }), still: false, w, h, dpr: 2, t: 0 };
  const scene = make({ getContext: () => pen(log) }, view, { ...defaults(SOURCE), ...defaults(SPEC), seed: 1, level: 2, math: unit, onFacts: (one) => facts.push(one), ...value });
  const draw = () => {
    log.length = 0;
    scene.draw();
    return [...log];
  };
  return { scene, facts, draw, view };
};

const draws = (log) => log.filter(([name]) => name === 'drawImage');

const reach = (w, h) => Math.min(w, h) / 2 - PAD * 2;

test('the stack alone is the stage: the rosette of the copies in one disc at any aspect, no text, no bars', () => {
  const live = open({});
  const log = live.draw();
  expect(images.at(-1).width).toBe(OUT);
  expect(draws(log)).toEqual([['drawImage', 'layer', 800 - reach(1600, 900), 450 - reach(1600, 900), 2 * reach(1600, 900), 2 * reach(1600, 900)]]);
  expect(log.some(([name]) => name === 'fillText' || name === 'fillRect')).toBe(false);
  expect(draws(open({}, { w: 780, h: 1688 }).draw())).toEqual([['drawImage', 'layer', 390 - reach(780, 1688), 844 - reach(780, 1688), 2 * reach(780, 1688), 2 * reach(780, 1688)]]);
  expect(live.facts.at(-1)).toMatchObject({ name: 'carpet', copies: 6, step: 60, order: 4, petals: 12 });
});

test('the stack is a still: a stack change repaints the same canvas from a new rosette, a size lays it again from the same one', () => {
  const live = open({});
  live.draw();
  expect(live.scene.still).toBe(true);
  const before = images.length;
  live.scene.set({ copies: 4, full: 0, step: 45 });
  expect(draws(live.draw())).toHaveLength(1);
  expect(images.length).toBe(before + 1);
  expect(live.facts.at(-1)).toMatchObject({ copies: 4, step: 45, petals: 'partial orbit' });
  live.view.w = 900;
  live.scene.size();
  expect(draws(live.draw())).toEqual([['drawImage', 'layer', 450 - reach(900, 900), 450 - reach(900, 900), 2 * reach(900, 900), 2 * reach(900, 900)]]);
  expect(images.length).toBe(before + 1);
});

test('an export is named by the source, its design and the copies', () => {
  expect([PAGE.name({ ...defaults(SOURCE), copies: 6 }, { math }), PAGE.name({ ...defaults(SOURCE), copies: 6 })]).toEqual(['radial-flat-495-6', 'radial-flat-carpet-6']);
});

test('the page takes a design of either dimension, the blend lives in the library, and the empty code opens the carpet', () => {
  const { when, ...slot } = PAGE.design;
  expect([slot, PAGE.units]).toEqual([{ dim: 'dim', base: 'base', code: 'code' }, units]);
  expect([when({ source: 'flat' }), when({ source: 'moire' })]).toEqual([true, false]);
  expect(SPEC.find((row) => row.key === 'blend').when({})).toBe(false);
  expect(open({}).facts.at(-1)).toMatchObject({ name: 'carpet', order: 4 });
});

test('Enter snaps back to a full turn only from a partial orbit; the arrows step the copies and the step', () => {
  const enter = (full) => PAGE.keys.filter((row) => row.key === 'Enter' && row.when({ full })).length;
  expect([enter(1), enter(0)]).toEqual([0, 1]);
  expect(PAGE.keys.filter((row) => row.menu).map((row) => row.act)).toEqual(['whole', 'copies']);
  const got = [];
  const hands = { whole: () => got.push('whole'), copies: (by) => got.push(by), step: (by) => got.push(by) };
  PAGE.actions.copies(hands, { key: 'ArrowLeft' });
  PAGE.actions.step(hands, { key: 'ArrowUp' });
  PAGE.actions.whole(hands);
  expect(got).toEqual([-1, 0.5, 'whole']);
});

test('without the unit the scene clears and throws nothing', () => {
  expect(open({}, { unit: null }).draw()).toEqual([['clearRect', 0, 0, 1600, 900]]);
});
