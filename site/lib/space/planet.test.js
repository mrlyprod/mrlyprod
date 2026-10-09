import { expect, test } from 'bun:test';
import { look } from './camera.js';
import { gl } from './fake.js';
import { NAMES, moons, planet, world } from './planet.js';

const VIEW = { w: 1280, h: 720, dpr: 1, t: 0 };

const CAM = look([0, 1, 4], [0, 0, 0]);

const made = (name, seed = 1, opts = {}) => {
  const fake = gl();
  fake.viewport(0, 0, VIEW.w, VIEW.h);
  return { fake, body: planet(fake, VIEW, world(name, seed, opts)) };
};

const count = (fake, verb, target) => fake.log.filter(([key, at]) => key === verb && (target === undefined || at === target)).length;

test('a world is a pure function of its name and seed', () => {
  expect(world('exo', 9)).toEqual(world('exo', 9));
  expect(world('earth', 4)).toEqual(world('earth', 4));
  expect(world('exo', 9)).not.toEqual(world('exo', 10));
});

test('exo rolls terran, desert, ice, lava and gas worlds', () => {
  const kinds = new Set(Array.from({ length: 200 }, (_, seed) => world('exo', seed).kind));
  expect([...kinds].sort()).toEqual(['desert', 'gas', 'ice', 'lava', 'terran']);
});

test('every world builds and draws in one pass, rings and three moons on', () => {
  for (const name of NAMES) {
    const { fake, body } = made(name, 3, { rings: 'on', moons: 3 });
    fake.log.length = 0;
    body.draw(CAM, [1, 0.2, 0.3], 1000);
    expect([name, count(fake, 'drawArrays')]).toEqual([name, 1]);
  }
});

test('exo gas worlds draw from three muted palettes', () => {
  const gas = Array.from({ length: 300 }, (_, seed) => world('exo', seed)).filter((w) => w.kind === 'gas');
  const hsv = (rgb) => {
    const c = rgb.map((v) => v ** (1 / 2.2));
    return (Math.max(...c) - Math.min(...c)) / Math.max(...c);
  };
  expect(new Set(gas.map((w) => w.tone.join())).size).toBe(3);
  expect(Math.max(...gas.flatMap((w) => w.bands.map(([, ...rgb]) => hsv(rgb))))).toBeLessThan(0.6);
});

test('the moon has warm highlands and darker blue-grey maria', () => {
  const [high, sea] = world('moon').colors;
  expect(high[0]).toBeGreaterThan(high[2]);
  expect(sea[2]).toBeGreaterThan(sea[0]);
  expect(sea[1]).toBeLessThan(high[1] / 2);
});

test('moons are a pure function of the world and t, each on its own orbit, three at most', () => {
  const p = world('jupiter', 5);
  expect(moons(p, 1234)).toEqual(moons(p, 1234));
  expect(moons(p, 1234).map(({ pos }) => Math.hypot(...pos).toFixed(9))).toEqual(p.moons.map((m) => m.a.toFixed(9)));
  expect(moons(world('mars', 5, { moons: 3 }), 0).length).toBe(3);
});

test('each map is baked once at make and never on a draw', () => {
  const bakes = (name) => {
    const { fake, body } = made(name);
    for (const t of [0, 500, 1000]) body.draw(CAM, [1, 0, 0], t);
    return count(fake, 'generateMipmap', fake.TEXTURE_CUBE_MAP);
  };
  expect(['earth', 'mars', 'jupiter', 'moon', 'venus'].map(bakes)).toEqual([3, 2, 1, 2, 1]);
});

test('a fourth draw argument k scales the whole output, 1 when left out', () => {
  const { fake, body } = made('earth');
  body.draw(CAM, [1, 0, 0], 0);
  body.draw(CAM, [1, 0, 0], 0, 0.25);
  const [plain, faded] = fake.draws().slice(-2);
  expect([plain.uniforms.uFade, faded.uniforms.uFade, faded.fs.includes('o = vec4(col, 1.0 - left) * uFade;')]).toEqual([1, 0.25, true]);
});

test('drop deletes every texture and the draw program, and the next world of that kind bakes with the same bake program', () => {
  const { fake, body } = made('saturn', 2);
  body.draw(CAM, [1, 0, 0], 0);
  body.drop();
  const programs = count(fake, 'createProgram');
  const next = planet(fake, VIEW, world('jupiter', 2));
  next.draw(CAM, [1, 0, 0], 0);
  const bakes = new Set(fake.log.filter(([key]) => key === 'useProgram').map(([, prog]) => prog).filter((prog) => fake.uniforms(prog).some(({ name }) => name === 'uOff')));
  expect([count(fake, 'deleteTexture') + 2, count(fake, 'deleteProgram'), programs, count(fake, 'createProgram'), bakes.size]).toEqual([count(fake, 'createTexture'), 1, 2, 3, 1]);
});

test('a staged planet starts its links at make and bakes one map a step, mips and draw program on the last; never stepped it builds as before', () => {
  let linked = false;
  const fake = gl();
  const slow = new Proxy(fake, {
    get: (target, key) => {
      if (key === 'getExtension') return (name) => (name === 'KHR_parallel_shader_compile' ? { COMPLETION_STATUS_KHR: 0x91b1 } : target.getExtension(name));
      if (key === 'getProgramParameter') return (prog, what) => (what === 0x91b1 ? linked : target.getProgramParameter(prog, what));
      return target[key];
    },
  });
  slow.viewport(0, 0, VIEW.w, VIEW.h);
  const staged = planet(slow, VIEW, world('earth', 1), { staged: true });
  const state = () => [count(fake, 'createProgram'), fake.draws().length, count(fake, 'generateMipmap', fake.TEXTURE_CUBE_MAP)];
  const steps = [state(), [staged.ensure(), ...state()]];
  linked = true;
  for (let i = 0; i < 3; i++) steps.push([staged.ensure(), ...state()]);
  staged.draw(CAM, [1, 0, 0], 0);
  const plain = made('earth');
  plain.body.draw(CAM, [1, 0, 0], 0);
  const strip = (one) => ({ ...one, units: null });
  expect(steps).toEqual([[4, 0, 0], [false, 4, 0, 0], [false, 4, 6, 0], [false, 4, 12, 0], [true, 4, 18, 3]]);
  expect([staged.ensure(), fake.draws().map(strip)]).toEqual([true, plain.fake.draws().map(strip)]);
});
