import { expect, test } from 'bun:test';
import { rng } from '../../lib/scene.js';
import { gl } from '../../lib/space/fake.js';
import { make } from './scene.js';

const canvas = (fake) => ({ width: 1, height: 1, getContext: (kind) => (kind === 'webgl2' ? fake : null) });

const VIEW = { rand: rng(7), look: () => ({ paper: '#102030', accent: '#ff8000' }), still: false, w: 1280, h: 720, dpr: 1, t: 0 };

const paints = (fake) => fake.draws().filter((one) => one.uniforms.uJulia === 1 && one.fb === null);

test('the julia dive draws to the canvas at the start and a minute in', () => {
  const fake = gl();
  const at = { ...VIEW };
  const scene = make(canvas(fake), at, { seed: 7 });
  scene.draw();
  at.t = 60000;
  scene.draw();
  expect(paints(fake).length).toBe(2);
  scene.stop();
});

test('the knobs map: palette empty to classic and seeded to the seeded ramp, path 1 to one more draw, speed 2 to twice the zoom', () => {
  const drawn = (opts, t = 0) => {
    const fake = gl();
    make(canvas(fake), { ...VIEW, t }, { seed: 7, ...opts }).draw();
    const [one] = paints(fake);
    return { fake, one, span: +Math.hypot(...one.uniforms.uU).toFixed(12) };
  };
  const looks = ['', 'seeded'].map((palette) => ['uGround', 'uRamp'].map((name) => name in drawn({ palette }).one.uniforms));
  const pens = [0, 1].map((path) => drawn({ path }).fake.draws().filter((one) => one.vs.includes('aShape')).length);
  expect([looks, pens, drawn({ speed: 2 }, 2000).span]).toEqual([[[true, false], [false, true]], [0, 1], drawn({ speed: 1 }, 4000).span]);
});
