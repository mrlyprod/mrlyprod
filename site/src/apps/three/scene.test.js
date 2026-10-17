import { expect, test } from 'bun:test';
import * as math from '@mrlyprod/mrly/math';
import { defaults } from '../../lib/app/spec.js';
import { rng } from '../../lib/rand.js';
import { SPEC, make, name } from './scene.js';
import { LOOKS, ORBIT, angle } from './stage.js';

const brief = (arg) => (ArrayBuffer.isView(arg) ? arg.length : typeof arg === 'function' ? 'thing' : Array.isArray(arg) ? arg.length : typeof arg === 'object' && arg !== null ? Object.keys(arg).join(',') : arg);

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

const open = (value = {}, { three = true, still = false } = {}) => {
  const log = [];
  const told = [];
  const view = { rand: rng(1), look: () => ({ paper: '#000000', accent: '#008cff' }), still, w: 800, h: 600, dpr: 2, t: 0 };
  const base = { ...defaults(SPEC), seed: 1, base: 2, code: '23' };
  const scene = make({}, view, { ...base, ...value, math, three: three && fake(log), tell: (one) => told.push(one) });
  const step = (patch) => {
    log.length = 0;
    scene.set({ ...base, ...value, ...patch });
    scene.draw();
    return log;
  };
  const pose = (t) => {
    view.t = t;
    log.length = 0;
    scene.draw();
    return log.find(([key]) => key.endsWith('Camera.position.set'));
  };
  const made = (name) => log.filter(([key]) => key === `new three.${name}`);
  return { log, told, view, scene, step, pose, made, last: () => told.at(-1) };
}

const row = (told, name) => told.rows.find(([one]) => one === name)?.[1];

test('the page opens on the carpet sponge: one renderer, one lit mesh of its exposed faces, and its counts told', () => {
  const live = open();
  expect([live.made('PerspectiveCamera').length, live.made('MeshStandardMaterial').length]).toEqual([1, 1]);
  expect(live.made('WebGLRenderer')).toEqual([['new three.WebGLRenderer', 'canvas,alpha,antialias,preserveDrawingBuffer']]);
  expect(live.made('BufferAttribute').map(([, size]) => size)).toEqual([18048 * 12, 18048 * 12, 18048 * 6]);
  expect([live.made('HemisphereLight').length, live.made('DirectionalLight').length, live.made('Mesh').length]).toEqual([1, 2, 1]);
  expect(live.last().rows.slice(0, 7)).toEqual([['Name', 'carpet 23'], ['Side', 27], ['Filled', '8000'], ['Empty', '11683'], ['Exposed', '18048'], ['Fill ratio', '0.4064'], ['Dimension', '2.7268']]);
  expect([row(live.last(), 'Euler'), row(live.last(), 'Vertices')]).toEqual(['-1408', 15616]);
});

test('a value change keeps the renderer: opacity only repaints the material, a level rebuilds the mesh alone', () => {
  const live = open();
  const faint = live.step({ opacity: 0.5 });
  expect(faint.filter(([key]) => key.startsWith('new three.') && key !== 'new three.Plane').map(([key]) => key)).toEqual([]);
  expect(faint).toContainEqual(['three.MeshStandardMaterial.opacity=', 0.5]);
  expect(faint).toContainEqual(['three.MeshStandardMaterial.depthWrite=', false]);
  const deeper = live.step({ level: 2 });
  expect([deeper.some(([key]) => key === 'new three.WebGLRenderer'), deeper.filter(([key]) => key === 'new three.BufferGeometry').length]).toEqual([false, 1]);
  expect(deeper).toContainEqual(['three.BufferGeometry.dispose']);
  expect(row(live.last(), 'Side')).toBe(9);
});

test('cubes draw one orange instance per filled cell', () => {
  const live = open({ draw: 'cubes' });
  expect(live.made('InstancedMesh').map((one) => one[3])).toEqual([8000]);
  expect(live.log).toContainEqual(['three.MeshStandardMaterial.color.setRGB', 1, 143 / 255, 44 / 255, 'thing']);
});

test('a crop tells its census and nothing to draw beside the solid, and the exact edge clips the whole mesh with the walls of the box', () => {
  const live = open({ crop: 'ball' });
  expect([row(live.last(), 'Filled in'), row(live.last(), 'Filled cut'), row(live.last(), 'Filled out')]).toEqual([2320, 1224, 4456]);
  expect(Object.keys(live.last())).toEqual(['rows', 'error']);
  const exact = live.step({ crop: 'box', radius: 30, edge: 1 });
  expect(exact.filter(([key]) => key === 'new three.Plane').length).toBe(6);
  expect(exact.find(([key]) => key === 'new three.BufferAttribute')[1]).toBe(18048 * 12);
});

test('a solid past the face guard says what to lower and draws nothing', () => {
  const live = open({ number: 5, level: 3 });
  expect(live.last().rows[0]).toEqual(['Error', '1308798 faces is more than this page draws; lower the level.']);
  expect(live.made('Mesh')).toEqual([]);
});

test('the circle count leads the readouts with N, inside and C at r and the count side, so every circle knob moves the status', () => {
  const live = open({ circle: 1, count: 4, r: 2 });
  expect(live.last().rows.slice(0, 5)).toEqual([['N(r)', 4], ['Inside(r)', 1], ['C(r)', 6], ['Count side', 81], ['Name', 'carpet 23']]);
  live.step({ circle: 1, count: 4, r: 3 });
  expect(live.last().rows[0]).toEqual(['N(r)', 13]);
  live.step({ circle: 1, count: 3, r: 3 });
  expect([live.last().rows[3], row(live.last(), 'Fill of one tile')]).toEqual([['Count side', 27], 20]);
  live.step({ circle: 1, count: 3, r: 500 });
  expect(row(live.last(), 'r')).toBe(26);
});

test('the field draws the shell at the threshold and paints the diagonal plane as a texture, with its stats', () => {
  const live = open({ source: 'field', threshold: 4 });
  expect(live.last().rows).toEqual([['Name', 'carpet 23'], ['Layers', 4], ['Voxels', 262144], ['At or above', 76608], ['Faces', 42408], ['Field', '1 to 4']]);
  expect([live.made('DataTexture').map((one) => one.slice(2, 4)), live.made('MeshBasicMaterial').length]).toEqual([[[384, 384]], 1]);
  const two = live.step({ source: 'field', threshold: 4, cx: 1, px: 0.25 });
  expect([two.filter(([key]) => key === 'new three.DataTexture').length, two.filter(([key]) => key === 'new three.BufferAttribute').map(([, size]) => size)]).toEqual([2, [18, 12, 18, 12]]);
});

test('spin orbits the camera from t and turning it off holds the pose it reached', () => {
  const live = open();
  const first = live.pose(0);
  expect(live.pose(ORBIT / 8)).not.toEqual(first);
  const held = live.pose(ORBIT / 4);
  const after = live.step({ spin: 0 }).find(([key]) => key.endsWith('Camera.position.set'));
  live.view.t = ORBIT / 2;
  live.log.length = 0;
  live.scene.turn(0, 0);
  expect([after, live.log.find(([key]) => key.endsWith('Camera.position.set'))]).toEqual([held, held]);
  expect(angle(1, ORBIT / 4, 1)).toBeCloseTo(1 + Math.PI / 2, 9);
});

test('a drag turns, a fling coasts to a stop, the wheel zooms and a look snaps to the corner', () => {
  const live = open({ spin: 0 });
  const rest = live.pose(0);
  live.scene.turn(0.1, 0);
  const turned = live.pose(0);
  expect(turned).not.toEqual(rest);
  live.scene.fling(0.05, 0);
  const a = live.pose(0);
  const b = live.pose(0);
  expect(a).not.toEqual(b);
  for (let i = 0; i < 300; i++) live.pose(0);
  expect(live.pose(0)).toBeUndefined();
  live.scene.zoom(2);
  expect(live.log).toContainEqual(['three.OrthographicCamera.zoom=', 2]);
  live.log.length = 0;
  live.scene.look('corner');
  const [, x, y, z] = live.log.find(([key]) => key.endsWith('Camera.position.set'));
  expect([Math.atan2(y, x), Math.atan2(z, Math.hypot(x, y))].map((v) => +v.toFixed(6))).toEqual(LOOKS.corner.map((v) => +v.toFixed(6)));
});

test('a theme change repaints in the new accent, and without three the stage is blank but still tells and exports', () => {
  const live = open();
  live.log.length = 0;
  live.scene.theme();
  live.scene.draw();
  expect(live.log.some(([key]) => key === 'three.MeshStandardMaterial.color.setRGB')).toBe(true);
  const bare = open({}, { three: false });
  bare.scene.draw();
  expect([bare.log, row(bare.last(), 'Filled'), bare.scene.obj().startsWith('v ')]).toEqual([[], '8000', true]);
});

test('under reduced motion a draw repeats the opening pose', () => {
  const live = open({}, { still: true });
  live.scene.turn(0, 0);
  const first = live.log.find(([key]) => key.endsWith('Camera.position.set'));
  live.log.length = 0;
  live.scene.turn(0, 0);
  expect(live.log.find(([key]) => key.endsWith('Camera.position.set'))).toEqual(first);
});

test('an export is named by the design and what the stage shows of it', () => {
  const base = { ...defaults(SPEC), seed: 1, base: 2, code: '23' };
  expect([name(base), name({ ...base, crop: 'ball' }), name({ ...base, source: 'field', code: '' })]).toEqual(['three-23-l3', 'three-23-l3-ball', 'three-23-field']);
});
