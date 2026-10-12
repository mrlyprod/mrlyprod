import { expect, test } from 'bun:test';
import * as life from '../../../pkgs/mrlyjs/life.js';
import * as math from '../../../pkgs/mrlyjs/math.js';
import { defaults, tidy } from '../../lib/knobs.js';
import { rng } from '../../lib/scene.js';
import { keymap } from '../../lib/scene.jsx';
import { PAGE, PRESETS, SPEC, make } from './scene.js';

life.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/life/mrlyjs_life_bg.wasm', import.meta.url)).arrayBuffer() });
math.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/math/mrlyjs_math_bg.wasm', import.meta.url)).arrayBuffer() });

const pen = (log) =>
  new Proxy(
    {},
    {
      get: (_, key) => (...args) => log.push([key, ...args]),
      set: (_, key, value) => {
        log.push([key, value]);
        return true;
      },
    },
  );

const open = (value = {}, { w = 640, h = 480 } = {}) => {
  const log = [];
  const told = [];
  const start = { ...defaults(SPEC), seed: 7, size: 128, ...value };
  const view = { rand: rng(7), look: () => ({ paper: '#000000', accent: '#008cff' }), still: false, w, h, dpr: 1, t: 0 };
  const scene = make({ getContext: () => pen(log) }, view, { ...start, life, math, onStat: (stat) => told.push(stat) });
  return { log, told, scene, last: () => told[told.length - 1], set: (patch) => scene.set(Object.assign(start, patch)) };
};

test('the soup runs to the still at once, at the steps asked, and a draw with nothing new draws nothing', () => {
  const run = open({ steps: 12 });
  expect(run.log.length).toBe(0);
  run.scene.draw();
  const still = life.chladni.run(life.design_mask(2, 7, 3, 3), [0.283, 0.375], [0.283, 0.483], 128, 12, 0.45, 7);
  expect([run.last().gen, run.last().live]).toEqual([12, still.types.reduce((a, b) => a + b, 0)]);
  run.log.length = 0;
  run.scene.draw();
  expect(run.log.length).toBe(0);
});

test('the still alone takes the stage at any aspect, with nothing drawn beside it', () => {
  for (const [w, h] of [[640, 480], [390, 844]]) {
    const run = open({ steps: 4 }, { w, h });
    run.scene.draw();
    const side = Math.min(w, h) * 0.96;
    expect(run.log.filter(([verb]) => verb === 'fillRect')).toEqual([['fillRect', (w - side) / 2, (h - side) / 2, side, side]]);
    expect(run.log.some(([verb]) => ['arc', 'stroke', 'fillText', 'drawImage'].includes(verb))).toBe(false);
  }
});

test('step adds one generation to the still, and run goes back to it', () => {
  const run = open({ steps: 12 });
  const live = run.last().live;
  run.scene.step();
  expect(run.last().gen).toBe(13);
  run.scene.rerun();
  expect([run.last().gen, run.last().live]).toEqual([12, live]);
});

test('a window or steps change reruns the soup on the same mask, and a mask change reads the new mask', () => {
  const run = open({ steps: 8 });
  const first = run.last();
  run.set({ steps: 9 });
  expect([run.last().gen, run.last().span]).toEqual([9, 27]);
  run.set({ level: 2 });
  expect([run.last().span, run.last().cells, run.last().label]).toEqual([9, 64, 'carpet, side 3, level 2']);
  expect(first.cells).toBe(512);
});

test('the readouts give the windows as whole counts of the mask and the peak rings of both spectra', () => {
  const read = open({ ...Object.fromEntries(PRESETS)['Conway'], steps: 4 }).last();
  expect([read.birth, read.survive, read.cells, read.span, read.maskPeak > 0, read.wave > 0]).toEqual(['3', '2 to 3', 8, 3, true, true]);
});

test('the presets round-trip through the value, and the level caps by the size', () => {
  for (const [, preset] of PRESETS) expect(tidy(SPEC, { ...defaults(SPEC), ...preset })).toMatchObject(preset);
  const row = SPEC.find((one) => one.key === 'level');
  expect([row.options({ number: 3, size: 128 }).length, row.options({ number: 3, size: 256 }).length, row.options({ number: 7, size: 256 }).length]).toEqual([4, 5, 2]);
});

test('the scene is still, so the page binds no Space', () => {
  const run = open({ steps: 4 });
  expect([run.scene.still, keymap(PAGE.keys, [], { still: run.scene.still }).some((row) => row.key === ' ')]).toEqual([true, false]);
});

test('the export is named by the preset or the windows, and the soup seed', () => {
  const value = { ...defaults(SPEC), seed: 7 };
  expect([PAGE.name(value), PAGE.name({ ...value, born: '0.3', stay: '0.2-0.4' })]).toEqual(['chladni-bugs-7', 'chladni-b0.3-s0.2-0.4-7']);
});

test('a first run that fails tells its error and draws no board', () => {
  const run = open({ code: '0' });
  run.scene.draw();
  expect([run.told, run.log.some(([verb]) => verb === 'fillRect')]).toEqual([[{ error: 'the mask is empty.' }], false]);
});
