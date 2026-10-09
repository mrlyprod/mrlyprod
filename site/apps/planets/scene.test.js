import { expect, test } from 'bun:test';
import { rng } from '../../lib/scene.js';
import { project } from '../../lib/space/camera.js';
import { gl } from '../../lib/space/fake.js';
import { NAMES, world } from '../../lib/space/planet.js';
import { dot, len, norm } from '../../lib/space/vec.js';
import { CAMS, PAGE, make, next, shot } from './scene.js';

const EARTH = world('earth', 1);

const tilt = (params, [x, y, z]) => {
  const c = Math.cos(params.tilt);
  const s = Math.sin(params.tilt);
  return [c * x - s * y, s * x + c * y, z];
};

const open = (opts, paper = '#000000', size = {}) => {
  const fake = gl();
  const view = { rand: rng(1), look: () => ({ paper, accent: '#008cff' }), still: false, w: 1280, h: 720, dpr: 1, t: 0, ...size };
  const scene = make({ getContext: (kind) => (kind === 'webgl2' ? fake : null) }, view, opts);
  return { fake, view, scene };
};

const far = (cam, t, params = EARTH) => len(shot(params, { cam, seed: 4, t }).cam.pos);

test('the orbit circles at 3.2 R and comes round in 90 s', () => {
  expect([0, 30000, 61000].map((t) => far('orbit', t).toFixed(9))).toEqual(['3.200000000', '3.200000000', '3.200000000']);
  shot(EARTH, { seed: 4, t: 90000 }).cam.pos.forEach((v, i) => expect(v).toBeCloseTo(shot(EARTH, { seed: 4, t: 0 }).cam.pos[i], 9));
});

test('the approach closes from 12 R to 2.4 R over 40 s, then starts again', () => {
  expect([far('approach', 0), far('approach', 39999.999), far('approach', 40000)].map((v) => v.toFixed(4))).toEqual(['12.0000', '2.4000', '12.0000']);
});

test('the flyby passes 1.3 R from the centre and the sun clears the limb at 60% of 30 s', () => {
  const miss = (t) => {
    const { cam, sun } = shot(EARTH, { cam: 'flyby', seed: 4, t });
    const b = dot(cam.pos, sun);
    return Math.sqrt(dot(cam.pos, cam.pos) - b * b);
  };
  const closest = Math.min(...Array.from({ length: 301 }, (_, i) => far('flyby', i * 100)));
  expect(closest).toBeCloseTo(1.3, 3);
  expect(miss(18000)).toBeCloseTo(1, 9);
  expect([miss(17000) < 1, miss(19000) > 1]).toEqual([true, true]);
});

test('the sun stands at the phase angle the knob names', () => {
  for (const cam of ['orbit', 'approach']) {
    for (const deg of [0, 60, 135, 180]) {
      const { cam: view, sun } = shot(EARTH, { cam, sun: deg, seed: 2, t: 7000 });
      expect([cam, deg, ((Math.acos(Math.max(-1, Math.min(1, dot(sun, norm(view.pos))))) * 180) / Math.PI).toFixed(6)]).toEqual([cam, deg, deg.toFixed(6)]);
    }
  }
});

test('the orbit backs off until the whole ring is in frame, wide or tall', () => {
  const saturn = world('saturn', 1);
  const ring = Array.from({ length: 72 }, (_, i) => tilt(saturn, [saturn.ring.outer * Math.cos(i / 11.46), 0, saturn.ring.outer * Math.sin(i / 11.46)]));
  for (const [w, h] of [[1280, 720], [390, 640]]) {
    for (const t of [0, 20000, 50000]) {
      const { cam } = shot(saturn, { seed: 3, t, aspect: w / h });
      const seen = ring.map((p) => project({ w, h }, cam, p)).filter((at) => at && at[0] >= 0 && at[0] <= w && at[1] >= 0 && at[1] <= h);
      expect([w, t, seen.length]).toEqual([w, t, ring.length]);
    }
  }
  expect(len(shot(EARTH, { aspect: 390 / 640 }).cam.pos)).toBeCloseTo(3.2, 9);
});

test('every camera draws a frame through the post chain', () => {
  for (const cam of CAMS) {
    const { fake, view, scene } = open({ world: 'saturn', cam, seed: 5 });
    view.t = 25000;
    scene.draw();
    expect([cam, fake.log.filter(([key]) => key === 'drawArrays').length > 0]).toEqual([cam, true]);
    scene.stop();
  }
});

test('a phone bakes 512 faces, gathers 6 air samples and starts at 0.75 scale', () => {
  const { fake, scene } = open({ world: 'earth' }, '#000000', { w: 1170, h: 2532, dpr: 3 });
  const made = fake.draws().length;
  scene.draw();
  const bakes = fake.draws().filter(({ fs }) => fs.includes('#define MAP'));
  const frame = fake.draws().slice(made);
  expect([...new Set(bakes.map(({ viewport }) => viewport[2]))]).toEqual([512]);
  expect(frame.find(({ fs }) => fs.includes('samplerCube uSurface')).fs).toContain('#define STEPS 6');
  expect(frame[0].viewport).toEqual([0, 0, 878, 1899]);
});

test('a live sun, camera and speed reach the next frame without a new texture', () => {
  const live = { current: { world: 'earth', sun: 60 } };
  const { fake, view, scene } = open({ world: 'earth', live });
  const planet = () => fake.draws().filter(({ fs }) => fs.includes('samplerCube uSurface')).at(-1).uniforms;
  scene.draw();
  const made = fake.log.filter(([key]) => key === 'createTexture').length;
  live.current = { world: 'earth', sun: 150, cam: 'approach', speed: 2 };
  view.t = 1000;
  scene.draw();
  const { uSun, uPos } = planet();
  expect([fake.log.filter(([key]) => key === 'createTexture').length, Math.round((Math.acos(dot(uSun, norm(uPos))) * 180) / Math.PI)]).toEqual([made, 150]);
});

test('a stall over 100 ms never drops a notch', () => {
  const { fake, view, scene } = open({ world: 'earth' });
  for (const t of [0, 16, 32, 1532, 1548, 1564]) {
    view.t = t;
    scene.draw();
  }
  expect(fake.draws().filter(({ fs }) => fs.includes('samplerCube uSurface')).at(-1).viewport).toEqual([0, 0, 1280, 720]);
});

test('the ground is black in both themes', () => {
  for (const paper of ['#000000', '#ffffff']) {
    const { fake, scene } = open({ world: 'earth' }, paper);
    scene.draw();
    expect(fake.log.filter(([key]) => key === 'clearColor').map((call) => call.slice(1))).toEqual([[0, 0, 0, 1]]);
  }
});

test('without WebGL2 the scene paints black and fires nothing', () => {
  const log = [];
  const pen = new Proxy({}, { get: (_, key) => (...args) => log.push([key, ...args]), set: (_, key, v) => log.push([key, v]) });
  const scene = make({ width: 10, height: 10, getContext: (kind) => (kind === '2d' ? pen : null) }, { w: 10, h: 10, dpr: 1, t: 0 }, {});
  scene.draw();
  expect(log).toEqual([['fillStyle', '#000'], ['fillRect', 0, 0, 10, 10]]);
});

test('Enter and the arrows step through the worlds and wrap', () => {
  expect([next('earth', 1), next('earth', -1), next('exo', 1), next('nowhere', 1)]).toEqual(['mars', 'exo', 'earth', 'earth']);
  expect(PAGE.keys.map((row) => [row.act, row.button ?? false])).toEqual([['next', true], ['prev', false], ['next', false]]);
});
