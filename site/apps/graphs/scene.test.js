import { expect, test } from 'bun:test';
import * as math from '../../../pkgs/mrlyjs/math.js';
import { defaults, tidy } from '../../lib/knobs.js';
import { rng } from '../../lib/scene.js';
import { HOLD, SETTLE, TICK, pace } from './engine.js';
import { SPEC, make, tones } from './scene.js';
import { ORBIT, detail } from './space.js';

math.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/math/mrlyjs_math_bg.wasm', import.meta.url)).arrayBuffer() });

const brief = (arg) => (ArrayBuffer.isView(arg) ? arg.length : typeof arg === 'function' ? 'thing' : typeof arg === 'object' && arg !== null ? Object.keys(arg).join(',') : arg);

const pen = (log) =>
  new Proxy(
    {},
    {
      get: (_, key) => (...args) => log.push([key, ...args]),
      set: (_, key, v) => {
        log.push([key, v]);
        return true;
      },
    },
  );

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

const open = (value, { still = false, three = false, seed = 1, w = 800, h = 600 } = {}) => {
  const log = [];
  const view = { rand: rng(seed), look: () => ({ paper: '#000000', accent: '#008cff' }), still, w, h, dpr: 2, t: 0 };
  const scene = make({ getContext: () => pen(log) }, view, { ...defaults(SPEC), seed, dim: 2, base: 3, code: '495', level: 2, graph: 'core', layout: 'grid', grow: 0, ...value, math, three: three ? fake(log) : undefined });
  const at = (t) => {
    view.t = t;
    log.length = 0;
    scene.draw();
    return log;
  };
  return { view, scene, log, at, count: (name) => log.filter(([key]) => key === name).length, names: () => log.map(([key]) => key) };
};

const arcs = (log) => log.filter(([name]) => name === 'arc');

const lines = (log) => log.filter(([name]) => name === 'lineTo');

const NODES = 64;

const BRANCHES = 88;

test('make draws nothing, the first draw lays every branch then every node in the shades of the look, and a draw with nothing new is a no-op', () => {
  const live = open({});
  expect(live.log).toEqual([]);
  expect(live.scene.every).toBe(1000);
  live.at(0);
  expect([lines(live.log).length, arcs(live.log).length]).toEqual([BRANCHES, NODES]);
  const names = live.names();
  expect(names.lastIndexOf('lineTo')).toBeLessThan(names.indexOf('arc'));
  expect(live.log).toContainEqual(['fillStyle', '#008cff']);
  expect(live.log).toContainEqual(['fillStyle', 'rgb(0, 109, 199)']);
  expect(live.log).toContainEqual(['strokeStyle', '#008cff']);
  expect(live.at(1000)).toEqual([]);
  live.scene.size();
  expect(arcs(live.log).length).toBe(NODES);
  live.log.length = 0;
  live.scene.theme();
  expect(arcs(live.log).length).toBe(NODES);
  expect(tones('#008cff', '#000000')).toEqual(['rgb(0, 39, 71)', 'rgb(0, 73, 133)', 'rgb(0, 109, 199)', '#008cff']);
});

test('the network sits centred inside the pad on the lattice, dots and lines sized by the cell', () => {
  const live = open({});
  live.at(0);
  const xs = arcs(live.log).map(([, x]) => x);
  const ys = arcs(live.log).map(([, , y]) => y);
  expect(Math.min(...xs) + Math.max(...xs)).toBeCloseTo(800, 6);
  expect(Math.min(...ys) + Math.max(...ys)).toBeCloseTo(600, 6);
  const k = (600 - 2 * 48) / 8;
  expect(Math.max(...xs) - Math.min(...xs)).toBeCloseTo(8 * k, 6);
  expect(arcs(live.log)[0][3]).toBeCloseTo(0.22 * k, 6);
  expect(live.log).toContainEqual(['lineWidth', 0.09 * k]);
});

test('with a span the reveal grows from the centre as t runs, drawing only what is new, holds, then starts over, and a still shows it whole', () => {
  const live = open({ grow: 4 });
  expect(live.scene.every).toBeUndefined();
  live.at(0);
  expect([arcs(live.log).length, lines(live.log).length]).toEqual([0, 0]);
  live.at(2000);
  const half = arcs(live.log).length;
  const strands = lines(live.log).length;
  expect(half).toBeGreaterThan(4);
  expect(half).toBeLessThan(NODES / 2);
  expect(strands).toBeLessThan(BRANCHES / 2);
  live.at(4000);
  expect([arcs(live.log).length, lines(live.log).length]).toEqual([NODES - half, BRANCHES - strands]);
  expect(live.names()).not.toContain('clearRect');
  expect(live.log).toContainEqual(['globalCompositeOperation', 'destination-over']);
  expect(live.at(4000 + HOLD - 1)).toEqual([]);
  live.at(4000 + HOLD + 500);
  expect(live.names()).toContain('clearRect');
  expect(arcs(live.log).length).toBeLessThan(half);
  const jump = open({ grow: 4 });
  jump.at(4000);
  expect([arcs(jump.log).length, lines(jump.log).length]).toEqual([NODES, BRANCHES]);
  const still = open({ grow: 4 }, { still: true });
  still.at(0);
  expect([arcs(still.log).length, lines(still.log).length]).toEqual([NODES, BRANCHES]);
});

test('a force layout steps the unit from t, so the picture at a t is the same in one jump or by steps, moves the dots, and settles', () => {
  const stepped = open({ layout: 'force' });
  for (let t = 0; t <= 2000; t += 250) stepped.at(t);
  const walked = [...stepped.log];
  const jumped = open({ layout: 'force' });
  expect(jumped.at(2000)).toEqual(walked);
  const rest = open({ layout: 'force' });
  const first = arcs(rest.at(0)).map(([, x, y]) => [x, y]);
  const later = arcs(rest.at(TICK * 50)).map(([, x, y]) => [x, y]);
  expect(later).not.toEqual(first);
  rest.at(TICK * SETTLE);
  expect(rest.at(TICK * SETTLE + TICK * 5)).toEqual([]);
  expect(rest.scene.every).toBeUndefined();
  const still = open({ layout: 'force' }, { still: true });
  still.at(0);
  expect(arcs(still.log).length).toBe(NODES);
  expect(still.at(0)).toEqual([]);
});

test('a blank code, graph and layout roll from the seed and the facts say what was rolled', () => {
  const a = open({ code: '', graph: '', layout: '' }, { seed: 7 });
  const b = open({ code: '', graph: '', layout: '' }, { seed: 7 });
  expect(a.scene.facts).toEqual(b.scene.facts);
  expect(['core', 'edge', 'tunnel']).toContain(a.scene.facts.graph);
  expect(['grid', 'force']).toContain(a.scene.facts.layout);
  expect(a.at(500)).toEqual(b.at(500));
  const seen = new Set(Array.from({ length: 12 }, (_, seed) => open({ code: '', graph: '', layout: '' }, { seed }).scene.facts.code));
  expect(seen.size).toBeGreaterThan(6);
});

test('a lock with no query rolls the page base and a network with branches at every seed', () => {
  const lock = (seed) => {
    const view = { rand: rng(seed), look: () => ({ paper: '#000000', accent: '#008cff' }), still: false, w: 800, h: 600, dpr: 2, t: 0 };
    const scene = make({ getContext: () => pen([]) }, view, { math, seed });
    const { base, branches } = scene.facts;
    scene.stop();
    return `${base} ${branches > 0}`;
  };
  expect(new Set(Array.from({ length: 80 }, (_, seed) => lock(seed)))).toEqual(new Set(['3 true']));
});

test('a force layout too big to settle in one go rests on the grid under reduced motion and runs as asked otherwise', () => {
  expect(open({ layout: 'force' }, { still: true }).scene.facts.layout).toBe('force');
  expect(open({ layout: 'force', level: 3 }, { still: true }).scene.facts.layout).toBe('grid');
  expect(open({ layout: 'force', level: 3 }).scene.facts.layout).toBe('force');
});

test('the svg draws the shown dots and lines at css size in the shades on screen, and the json carries the facts and the network', () => {
  const live = open({});
  live.at(0);
  const text = live.scene.svg();
  expect(text.startsWith('<svg xmlns="http://www.w3.org/2000/svg" width="400" height="300"')).toBe(true);
  expect((text.match(/<circle /g) ?? []).length).toBe(NODES);
  expect((text.match(/L[\d.]+ [\d.]+/g) ?? []).length).toBe(BRANCHES);
  expect(text).toContain('fill="#008cff"');
  expect(text).toContain('stroke="rgb(0, 109, 199)"');
  const part = open({ grow: 4 });
  part.at(2000);
  expect((part.scene.svg().match(/<circle /g) ?? []).length).toBe(arcs(part.log).length);
  const facts = JSON.parse(live.scene.json());
  expect(facts).toMatchObject({ code: '495', nodes: NODES, branches: BRANCHES });
  expect([facts.network.dim, facts.network.nodes.length, facts.network.branches.length]).toEqual([2, NODES, BRANCHES]);
});

test('the level knob stops at the cap of the dim, the graph and the layout, and the spin knob shows only on a cube', () => {
  const level = (value) => tidy(SPEC, { ...defaults(SPEC), level: 6, dim: 2, ...value }).level;
  expect([level({}), level({ layout: 'grid', graph: 'core' }), level({ dim: 3 }), level({ dim: 3, layout: 'grid', graph: 'core' }), level({ number: 7 })]).toEqual([3, 4, 2, 3, 1]);
  const spin = SPEC.find((row) => row.key === 'spin');
  expect([spin.when({ dim: 2 }), spin.when({ dim: 3 }), spin.when({ dim: '3' })]).toEqual([false, true, true]);
});

test('a cube builds one renderer, two instanced meshes of the nodes and the branches in the accent shades, and renders each draw', () => {
  const live = open({ dim: 3, base: 2, code: '23', graph: 'tunnel' }, { three: true });
  expect(live.scene.facts).toMatchObject({ dim: 3, nodes: 329, branches: 600 });
  expect(live.count('new three.WebGLRenderer')).toBe(1);
  expect(live.log.filter(([key]) => key === 'new three.InstancedMesh').map((row) => row[3])).toEqual([329, 600]);
  expect([live.count('three.InstancedMesh.setMatrixAt'), live.count('three.InstancedMesh.setColorAt')]).toEqual([929, 929]);
  expect(live.log).toContainEqual(['three.Color.set', '#008cff']);
  expect(live.log).toContainEqual(['three.InstancedMesh.count=', 329]);
  expect(live.log).toContainEqual(['three.InstancedMesh.count=', 600]);
  expect(live.log).toContainEqual(['three.WebGLRenderer.setSize', 800, 600, false]);
  expect(live.names().at(-1)).toBe('three.WebGLRenderer.render');
  expect(live.scene.every).toBeUndefined();
  live.at(1000);
  expect([live.count('three.InstancedMesh.setMatrixAt'), live.names().at(-1)]).toEqual([0, 'three.WebGLRenderer.render']);
});

test('the cube orbits from t and a seeded start, holds with spin off until a drag turns it, and stop disposes everything', () => {
  const pose = (live, t) => live.at(t).find(([key]) => key === 'three.OrthographicCamera.position.set');
  const live = open({ dim: 3, base: 2, code: '23' }, { three: true });
  const first = pose(live, 0);
  expect(pose(live, ORBIT / 8)).not.toEqual(first);
  expect(pose(live, 0)).toEqual(first);
  expect(pose(open({ dim: 3, base: 2, code: '23' }, { three: true }), 0)).toEqual(first);
  expect(pose(open({ dim: 3, base: 2, code: '23' }, { three: true, seed: 2 }), 0)).not.toEqual(first);
  const held = open({ dim: 3, base: 2, code: '23', spin: 0 }, { three: true });
  expect(held.scene.every).toBe(1000);
  const rest = pose(held, 0);
  expect(held.at(ORBIT / 8).map(([key]) => key)).toEqual(['three.OrthographicCamera.position.set', 'three.OrthographicCamera.lookAt', 'three.WebGLRenderer.render']);
  expect(pose(held, ORBIT / 4)).toEqual(rest);
  held.scene.turn(0.25, 0.1);
  expect(held.names().at(-1)).toBe('three.WebGLRenderer.render');
  held.log.length = 0;
  held.scene.stop();
  expect(held.names()).toEqual(['three.SphereGeometry.dispose', 'three.CylinderGeometry.dispose', 'three.MeshLambertMaterial.dispose', 'three.InstancedMesh.dispose', 'three.InstancedMesh.dispose', 'three.WebGLRenderer.dispose', 'three.WebGLRenderer.forceContextLoss']);
});

test('a cube in a force layout replaces its matrices as the unit steps, and a theme change recolours every instance', () => {
  const live = open({ dim: 3, base: 2, code: '23', layout: 'force' }, { three: true });
  expect(live.count('three.InstancedMesh.setMatrixAt')).toBe(400 + 672);
  expect(live.at(0).filter(([key]) => key === 'three.InstancedMesh.setMatrixAt')).toEqual([]);
  live.at(TICK * 20);
  expect(live.count('three.InstancedMesh.setMatrixAt')).toBe(400 + 672);
  expect(live.at(TICK * 20).filter(([key]) => key === 'three.InstancedMesh.setMatrixAt')).toEqual([]);
  live.at(0);
  expect(live.count('three.InstancedMesh.setMatrixAt')).toBe(400 + 672);
  live.log.length = 0;
  live.scene.theme();
  expect(live.count('three.InstancedMesh.setColorAt')).toBe(400 + 672);
  expect(live.names().at(-1)).toBe('three.WebGLRenderer.render');
});

test('a cube in a grid layout places its matrices once and a grow only changes how many are shown, rendering each draw', () => {
  const live = open({ dim: 3, base: 2, code: '23', layout: 'grid', grow: 4 }, { three: true });
  expect(live.count('three.InstancedMesh.setMatrixAt')).toBe(400 + 672);
  const draws = [1000, 2000, 3000].map((t) => {
    const log = live.at(t);
    return [log.filter(([key]) => key === 'three.InstancedMesh.setMatrixAt').length, log.filter(([key]) => key === 'three.InstancedMesh.count=').length, log.at(-1)[0]];
  });
  expect(draws).toEqual(Array(3).fill([0, 2, 'three.WebGLRenderer.render']));
});

test('the geometry of a cube coarsens as its instances multiply, so the triangles stay near a phone budget', () => {
  expect([detail(329, 600).ball, detail(5000, 15000).ball, detail(19675, 56816).ball]).toEqual([[12, 8], [6, 4], [4, 3]]);
  const live = open({ dim: 3, base: 2, code: '23', graph: 'tunnel' }, { three: true });
  expect(live.log).toContainEqual(['new three.SphereGeometry', 1, 12, 8]);
  expect(live.log).toContainEqual(['new three.CylinderGeometry', 1, 1, 1, 8, 1, false]);
});

test('a cube without three in hand draws nothing until it lands and keeps its facts', () => {
  const live = open({ dim: 3, base: 2, code: '23' });
  live.at(0);
  expect(live.log).toEqual([]);
  expect(live.scene.facts.nodes).toBe(400);
  expect(live.scene.svg()).toBe('');
  live.scene.stop();
});
