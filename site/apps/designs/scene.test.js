import { expect, test } from 'bun:test';
import * as math from '../../../pkgs/mrlyjs/math.js';
import { defaults } from '../../lib/knobs.js';
import { rng } from '../../lib/scene.js';
import { SPEC, make } from './scene.js';

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

const brief = (arg) => (ArrayBuffer.isView(arg) ? arg.length : typeof arg === 'function' ? 'thing' : typeof arg === 'object' && arg !== null ? Object.keys(arg).join(',') : arg);

const fake = (log) => {
  const any = (path) =>
    new Proxy(function () {}, {
      get: (_, key) => (typeof key === 'symbol' ? undefined : any(`${path}.${key}`)),
      set: (_, key, value) => {
        log.push([`${path}.${key}=`, brief(value)]);
        return true;
      },
      apply: (_, __, args) => {
        log.push([path, ...args.map(brief)]);
        return any(path);
      },
      construct: (_, args) => {
        log.push([`new ${path}`, ...args.map(brief)]);
        return any(path);
      },
    });
  return any('three');
};

const open = (seed, value, three) => {
  const log = [];
  const view = { rand: rng(seed), look: () => ({ paper: '#000000', accent: '#008cff' }), still: false, w: 800, h: 600, dpr: 2, t: 0 };
  const scene = make({ getContext: () => pen(log) }, view, { ...defaults(SPEC), ...value, math, three: three && fake(log) });
  return { log, view, scene, count: (name) => log.filter(([key]) => key === name).length, names: () => log.map(([key]) => key) };
};

test('the flat stage draws one rect per run of the grown design, once, rests between ticks, and repaints on a theme or a size', () => {
  const live = open(1, { dim: 2, base: 2, code: '7', level: 2 });
  live.scene.draw();
  expect([live.count('rect'), live.count('fill'), live.scene.every]).toEqual([20, 1, 1000]);
  live.log.length = 0;
  live.view.t = 500;
  live.scene.draw();
  expect(live.log).toEqual([]);
  live.scene.theme();
  expect(live.count('rect')).toBe(20);
  live.log.length = 0;
  live.scene.size();
  expect(live.count('rect')).toBe(20);
});

test('the solid stage builds one mesh of the exposed faces, orbits the camera with t from a seeded start, and disposes on stop', () => {
  const live = open(3, { dim: 3, base: 2, code: '23', level: 1 }, true);
  const faces = Number(math.three.census(math.three.create(23, 3, 1, 2)).surface);
  expect(live.count('new three.WebGLRenderer')).toBe(1);
  expect(live.log.filter(([key]) => key === 'new three.BufferAttribute')).toEqual([['new three.BufferAttribute', faces * 12, 3], ['new three.BufferAttribute', faces * 12, 3], ['new three.BufferAttribute', faces * 6, 1]]);
  expect(live.log).toContainEqual(['three.WebGLRenderer.setSize', 800, 600, false]);
  expect(live.scene.every).toBe(undefined);
  const at = (t) => {
    live.view.t = t;
    live.log.length = 0;
    live.scene.draw();
    return live.log.find(([key]) => key === 'three.OrthographicCamera.position.set');
  };
  const first = at(0);
  expect(live.names().at(-1)).toBe('three.WebGLRenderer.render');
  expect(at(50)).not.toEqual(first);
  expect(at(0)).toEqual(first);
  const fresh = (seed) => {
    const again = open(seed, { dim: 3, base: 2, code: '23', level: 1 }, true);
    again.scene.draw();
    return again.log;
  };
  expect(fresh(3)).toContainEqual(first);
  expect(fresh(4)).not.toContainEqual(first);
  live.log.length = 0;
  live.scene.turn(0.25, 0);
  expect([live.names().at(-1), live.log.find(([key]) => key === 'three.OrthographicCamera.position.set')]).toEqual(['three.WebGLRenderer.render', expect.not.arrayContaining(first.slice(1))]);
  live.log.length = 0;
  live.scene.stop();
  expect(live.names()).toEqual(['three.BufferGeometry.dispose', 'three.MeshLambertMaterial.dispose', 'three.WebGLRenderer.dispose', 'three.WebGLRenderer.forceContextLoss']);
});

test('a cube without three in hand is a blank stage that still reads its counts', () => {
  const { scene, log } = open(1, { dim: 3, base: 2, code: '23', level: 5 });
  scene.draw();
  expect([log, scene.every, scene.svg]).toEqual([[], undefined, undefined]);
  expect(scene.info).toMatchObject({ dim: 3, base: 2, code: '23', level: 3, side: 27, name: 'carpet', title: 'bang dim 3, code 23', rep: '23', orbit: 8, fills: 8000, voids: 11683, surface: 18048 });
  expect(scene.info.dimension).toBeCloseTo(Math.log(20) / Math.log(3), 6);
});

test('a blank code rolls one from the seed, and a code past the space wraps into it', () => {
  const rolled = (seed) => open(seed, { dim: 2, base: 2, code: '' }).scene.info.code;
  expect(rolled(9)).toBe(rolled(9));
  expect(new Set(Array.from({ length: 12 }, (_, seed) => rolled(seed + 1))).size).toBeGreaterThan(1);
  expect(open(1, { dim: 2, base: 2, code: '23' }).scene.info.code).toBe('7');
});

test('the still of a plane exports as rects over the ground', () => {
  const flat = open(1, { dim: 2, base: 2, code: '7', level: 1 }).scene.svg();
  expect(flat.startsWith('<svg')).toBe(true);
  expect(flat.match(/<rect /g).length).toBe(5);
  expect(flat).toContain('fill="#000000"');
});
