import { expect, test } from 'bun:test';
import * as math from '../../../pkgs/mrlyjs/math.js';
import { defaults } from '../../lib/knobs.js';
import { rng } from '../../lib/scene.js';
import { SPEC, make } from './scene.js';
import { ORBIT, angle } from './stage.js';

math.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/math/mrlyjs_math_bg.wasm', import.meta.url)).arrayBuffer() });

const pen = (log) =>
  new Proxy(
    {},
    {
      get: (_, key) => (...args) => log.push([key, ...args.map((arg) => (arg && typeof arg === 'object' ? 'obj' : arg))]),
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

const open = (seed, value, three, still = false) => {
  const log = [];
  const view = { rand: rng(seed), look: () => ({ paper: '#000000', accent: '#008cff' }), still, w: 800, h: 600, dpr: 2, t: 0 };
  const scene = make({ getContext: () => pen(log) }, view, { ...defaults(SPEC), seed, base: 2, code: '23', ...value, math, three: three && fake(log) });
  const at = (t) => {
    view.t = t;
    log.length = 0;
    scene.draw();
    return log;
  };
  const pose = (t) => at(t).find(([key]) => key === 'three.OrthographicCamera.position.set');
  return { log, view, scene, at, pose, count: (name) => log.filter(([key]) => key === name).length, names: () => log.map(([key]) => key) };
};

const FACES = 18048;

test('the solid stage builds one lit mesh of the exposed faces in the accent and renders each draw', () => {
  const live = open(3, { level: 3 }, true);
  expect(live.count('new three.WebGLRenderer')).toBe(1);
  expect(live.log.filter(([key]) => key === 'new three.BufferAttribute')).toEqual([['new three.BufferAttribute', FACES * 12, 3], ['new three.BufferAttribute', FACES * 12, 3], ['new three.BufferAttribute', FACES * 6, 1]]);
  expect([live.count('new three.DirectionalLight'), live.count('new three.AmbientLight'), live.count('new three.Mesh')]).toEqual([2, 1, 1]);
  expect(live.log).toContainEqual(['three.WebGLRenderer.setSize', 800, 600, false]);
  expect(live.log).toContainEqual(['three.MeshLambertMaterial.color.setRGB', 0, 140 / 255, 1, 'thing']);
  expect([live.scene.every, live.scene.facts.fills]).toEqual([undefined, 8000]);
  live.at(0);
  expect(live.names().at(-1)).toBe('three.WebGLRenderer.render');
});

test('with spin on the camera orbits from t and from a seeded start; with spin off it holds until a drag turns it', () => {
  const live = open(3, {}, true);
  const first = live.pose(0);
  expect(live.pose(ORBIT / 8)).not.toEqual(first);
  expect(live.pose(0)).toEqual(first);
  expect(open(3, {}, true).pose(0)).toEqual(first);
  expect(open(4, {}, true).pose(0)).not.toEqual(first);
  const held = open(3, { spin: 0 }, true);
  const rest = held.pose(0);
  expect([held.pose(ORBIT / 8), held.scene.every]).toEqual([rest, 1000]);
  held.log.length = 0;
  held.scene.turn(0.25, 0.1);
  expect([held.names().at(-1), held.log.find(([key]) => key === 'three.OrthographicCamera.position.set')]).toEqual(['three.WebGLRenderer.render', expect.not.arrayContaining(rest.slice(1))]);
  expect(angle(1, 0.5, ORBIT / 4, 1)).toBeCloseTo(1.5 + Math.PI / 2, 9);
  expect(angle(1, 0.5, ORBIT / 4, 0)).toBe(1.5);
});

test('a resize or a theme change refits or recolours and draws again, and stop disposes everything', () => {
  const live = open(3, {}, true);
  live.log.length = 0;
  live.scene.size();
  expect(live.names()).toEqual(['three.WebGLRenderer.setSize', ...['left', 'right', 'top', 'bottom'].map((side) => `three.OrthographicCamera.${side}=`), 'three.OrthographicCamera.updateProjectionMatrix', 'three.OrthographicCamera.position.set', 'three.OrthographicCamera.lookAt', 'three.WebGLRenderer.render']);
  live.log.length = 0;
  live.scene.theme();
  expect(live.names()).toEqual(['three.MeshLambertMaterial.color.setRGB', 'three.OrthographicCamera.position.set', 'three.OrthographicCamera.lookAt', 'three.WebGLRenderer.render']);
  live.log.length = 0;
  live.scene.stop();
  expect(live.names()).toEqual(['three.BufferGeometry.dispose', 'three.MeshLambertMaterial.dispose', 'three.WebGLRenderer.dispose', 'three.WebGLRenderer.forceContextLoss']);
});

test('an empty design adds no mesh, and a solid without three in hand is a blank stage that keeps its facts and its obj', () => {
  const none = open(1, { code: '0' }, true);
  expect([none.count('new three.Mesh'), none.scene.facts.fills]).toEqual([0, 0]);
  const { scene, log } = open(1, {});
  scene.draw();
  expect([log, scene.every, scene.svg]).toEqual([[], undefined, undefined]);
  expect(scene.facts).toMatchObject({ code: '23', name: 'carpet', fills: 8000, surface: 18048 });
  expect(scene.obj().startsWith('v 0 0 0')).toBe(true);
});

test('a flat view draws its cut once on the 2D context, rests, repaints on a theme or a size, and writes the same picture as svg', async () => {
  const live = open(1, { view: 'slice' });
  expect(live.scene.every).toBe(1000);
  expect(live.scene.cut).toEqual({ reading: 'z 14 of 27', fills: 64, voids: 665, euler: 64 });
  live.scene.draw();
  expect(live.count('fillRect')).toBe(729);
  expect(live.at(500)).toEqual([]);
  live.scene.theme();
  expect(live.count('fillRect')).toBe(729);
  live.log.length = 0;
  live.scene.size();
  expect(live.count('fillRect')).toBe(729);
  const text = await live.scene.svg();
  expect(text.startsWith('<svg')).toBe(true);
  expect([text.includes('width="400" height="300"'), (text.match(/<rect /g) ?? []).length]).toEqual([true, 729]);
  expect(text).toContain('fill="#008cff"');
  const hex = open(1, { view: 'hex' });
  hex.scene.draw();
  expect(hex.count('fill')).toBe(4374);
  const diagonal = open(1, { view: 'diagonal' });
  diagonal.scene.draw();
  expect([diagonal.count('arc'), diagonal.scene.obj]).toEqual([306, undefined]);
});

test('under reduced motion the solid draws the start pose and a draw with no new time repeats it', () => {
  const live = open(3, {}, true, true);
  const first = live.pose(0);
  expect(live.pose(0)).toEqual(first);
  expect(first.slice(1).every(Number.isFinite)).toBe(true);
});
