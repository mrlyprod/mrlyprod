import { expect, test } from 'bun:test';
import { rng } from '../../lib/scene.js';
import { gl } from '../../lib/space/fake.js';
import { PAGE, SPEC, make } from './scene.js';

const canvas = (fake) => ({ width: 1, height: 1, getContext: (kind) => (kind === 'webgl2' ? fake : null) });

const view = (t = 0) => ({ rand: rng(7), look: () => ({ paper: '#000000', accent: '#008cff' }), still: false, w: 1280, h: 720, dpr: 1, t });

const scene = (opts, t = 0, fake = gl()) => {
  const at = view(t);
  const made = make(canvas(fake), at, { seed: 7, ...opts });
  made.draw();
  return { made, at, fake };
};

const play = (made, at, to, step = 100) => {
  for (let t = at.t + step; t <= to; t += step) {
    at.t = t;
    made.draw();
  }
};

test('without WebGL2 the scene is the blank one and still draws', () => {
  const made = make(canvas(null), view(), { seed: 1 });
  expect(() => made.draw()).not.toThrow();
  expect(made.phase()).toBe('cruise');
});

test('the value is ten knobs and the seed, studio by default, and the page keeps only the default keys', () => {
  expect(SPEC.map((one) => one.key)).toEqual(['code', 'n', 'pick', 'speed', 'path', 'pilot', 'look', 'palette', 'quality', 'hyper']);
  expect([SPEC.find((one) => one.key === 'look').def, SPEC.find((one) => one.key === 'look').options.map(([key]) => key), PAGE]).toEqual(['studio', ['graphic', 'studio', 'haze'], { spec: SPEC, record: true }]);
});

test('the HUD teaches the code in five lines: name, level, bits and fill, dimension and class, and the mode', () => {
  const { made } = scene({ code: 23 });
  expect(made.said()).toEqual(['carpet 23', 'n 3  level 0', 'bits 00010111  fill 20/27', 'dim 2.73  class 23', 'corridor']);
  expect(scene({ code: 60 }).made.said()[0]).toBe('bang 60');
  expect(scene({ code: 255 }).made.said().at(-1)).toBe('skim');
});

test('the flight is automatic: the scene offers no picking and the HUD draws no markers', () => {
  const { made, fake } = scene({ code: 23 });
  expect([made.tap, made.pick, made.go, made.choose, made.marks, fake.draws().some((one) => one.vs.includes('aRing'))]).toEqual([undefined, undefined, undefined, undefined, undefined, false]);
});

test('code 0 draws nothing and the HUD reads empty', () => {
  const { made, fake } = scene({ code: 0 });
  expect([made.said(), made.trails(), fake.draws().some((one) => 'uLo' in one.uniforms)]).toEqual([['empty'], [], false]);
});

test('path on draws the route ahead as one lit rail in one more draw, path off draws none', () => {
  const rails = (path) => {
    const { made, fake } = scene({ code: 23, path }, 5000);
    return [made.trails().map((one) => one.points.length), fake.draws().filter((one) => 'uHide' in one.uniforms).length];
  };
  expect([rails(1), rails(0)]).toEqual([[[32], 1], [[], 0]]);
});

test('the pilot knob flies the route its own way', () => {
  const turn = (pilot) => scene({ code: 23, pilot }, 30000).fake.draws().find((one) => 'uLo' in one.uniforms).uniforms.uRot;
  expect(turn('coaster')).not.toEqual(turn('steady'));
});

test('each look grades through its own post look, and hyper lays the march over a tunnel already settled at t 0', () => {
  const vignette = (look) => scene({ code: 23, look }).fake.draws().find((one) => 'uVignette' in one.uniforms).uniforms.uVignette;
  expect([...['graphic', 'studio', 'haze'].map(vignette), vignette(undefined)]).toEqual([0.25, 0.3, 0.4, 0.3]);
  const draws = scene({ code: 23, hyper: 1 }).fake.draws();
  const at = (name) => draws.findIndex((one) => name in one.uniforms);
  expect([draws[at('uMilk')].uniforms.uMilk, at('uMilk') < at('uK'), at('uK') < at('uVignette'), draws[at('uVignette')].uniforms.uVignette]).toEqual([0.3, true, true, 0.3]);
});

test('a fresh draw at t lands where play does', () => {
  const live = scene({ code: 23, pilot: 'fighter' });
  play(live.made, live.at, 20000);
  const jump = scene({ code: 23, pilot: 'fighter' }, 20000);
  expect([jump.made.said(), jump.made.trails()]).toEqual([live.made.said(), live.made.trails()]);
});
