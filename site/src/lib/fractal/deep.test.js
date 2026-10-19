import { expect, test } from 'bun:test';
import { rng } from '../rand.js';
import { FLOOR } from './consts.js';
import { fixed } from './orbit.js';
import { FADE, SPAN, deep } from './deep.js';
import { gl } from '../../../test/fake.js';

const FRAME = 1000 / 60;

const HOME = { xMin: -2, xMax: 1, yMin: -1.5, yMax: 1.5 };

const VIEW = { rand: rng(7), look: () => ({ paper: '#000000', accent: '#008cff' }), still: false, w: 1280, h: 720, dpr: 1, t: 0 };

test('the home orbit goes to the GPU whole, as an RG32F texture 4096 wide', () => {
  const fake = gl();
  const sent = [];
  fake.texImage2D = (...args) => sent.push(args.map((one) => (one instanceof Float32Array ? one.slice() : one)));
  deep(fake, VIEW, { kind: 'set', seed: 3 });
  const [, , inner, w, , , format, type, data] = sent.find((one) => one[2] === fake.RG32F);
  expect([inner, w, format, type, data[2], data[3]]).toEqual([fake.RG32F, 4096, fake.RG, fake.FLOAT, -0.75, 0]);
});

const paints = (fake) => fake.draws().filter((one) => 'uAlpha' in one.uniforms && one.fb === null);

test('the classic palette paints the ground and the accent straight to the canvas, its colour cycling a 256th a frame', () => {
  const fake = gl();
  deep(fake, { ...VIEW, look: () => ({ paper: '#102030', accent: '#ff8000' }), t: 256 * FRAME }, { palette: 'classic' }).draw();
  const [one] = paints(fake);
  expect([one.uniforms.uGround, one.uniforms.uAccent, one.uniforms.uTime, 'uRamp' in one.uniforms]).toEqual([[16 / 255, 32 / 255, 48 / 255], [1, 128 / 255, 0], 1, false]);
});

test('the view turns 1/2048 rad a frame the way the seed picks, in every palette', () => {
  const angle = (palette, seed) => {
    const fake = gl();
    deep(fake, { ...VIEW, t: 2048 * FRAME }, { palette, seed }).draw();
    const { uU } = paints(fake)[0].uniforms;
    return Math.atan2(uU[1], uU[0]);
  };
  expect([angle('classic', 0), angle('classic', 1), angle('', 0), angle('fire', 1)].map((v) => +v.toFixed(6))).toEqual([-1, 1, -1, 1]);
});

test('a home window opens twice as wide and fits the canvas, so a 3 by 3 window shows 6 on the short side either way up', () => {
  const span = (w, h, opts) => {
    const fake = gl();
    deep(fake, { ...VIEW, w, h }, opts).draw();
    return +(Math.hypot(...paints(fake)[0].uniforms.uU) * Math.min(w, h)).toFixed(9);
  };
  expect([span(1280, 720, { home: HOME }), span(720, 1280, { home: HOME }), span(1280, 720)]).toEqual([6, 6, SPAN]);
});

test('julia takes its c from the presets by seed, each new dive turns to the next, and a julia dive without presets throws', () => {
  const presets = [[-0.4, 0.6], [-0.8, 0.156], [0.285, 0.01]];
  const fake = gl();
  const sent = [];
  fake.texImage2D = (...args) => args[2] === fake.RG32F && sent.push([...args[8].slice(0, 4)]);
  const at = { ...VIEW, t: 0 };
  const made = deep(fake, at, { kind: 'julia', seed: 2, presets });
  made.draw();
  at.t = 2000 * Math.log2(SPAN / FLOOR) + FADE;
  made.draw();
  const cs = sent.filter(([zr, zi]) => zr === 0 && zi === 0).map(([, , cr, ci]) => [cr, ci].map((v) => +v.toFixed(5)));
  expect([cs[0], cs.at(-1)]).toEqual([presets[2], presets[0]]);
  expect(() => deep(gl(), VIEW, { kind: 'julia' })).toThrow('presets');
});

const drawn = (fake) => fake.draws().filter((one) => 'uAlpha' in one.uniforms);

test('a store over the cap draws into a target at the pinned size and copies it to the canvas, a store under it draws to the canvas', () => {
  const area = (w, h, dpr) => {
    const fake = gl();
    deep(fake, { ...VIEW, w, h, dpr }, { palette: 'classic' }).draw();
    const [one] = drawn(fake);
    const copy = fake.draws().at(-1);
    return [one.fb === null, one.viewport[2] * one.viewport[3], copy.fb, copy.viewport];
  };
  const [desk, phone, under] = [area(2560, 1600, 1), area(780, 1688, 2), area(1280, 800, 1)];
  expect([desk[0], phone[0], under[0]]).toEqual([false, false, true]);
  expect([desk[1] / 1.2e6, phone[1] / 0.5e6].map((v) => +v.toFixed(2))).toEqual([1, 1]);
  expect([desk[2], desk[3], phone[3], under[3]]).toEqual([null, [0, 0, 2560, 1600], [0, 0, 780, 1688], [0, 0, 1280, 800]]);
});

const lazy = (state) => {
  const fake = gl();
  const sync = { kind: 'Sync' };
  fake.fenceSync = () => sync;
  fake.clientWaitSync = () => state.wait;
  fake.readPixels = (x, y, w, h, format, type, into) => state.reads.push(into === 0 ? 'buffer' : 'now');
  fake.getBufferSubData = () => {
    state.gets += 1;
  };
  return fake;
};

test('the probe a stop ahead reads back through a buffer and a fence and lands once signaled, while a probe the path needs now or a fixed view reads at once', () => {
  const state = { wait: 0x911b, reads: [], gets: 0 };
  const at = { ...VIEW, t: 0 };
  const made = deep(lazy(state), at);
  made.draw();
  at.t = 100;
  made.draw();
  expect([state.reads, state.gets]).toEqual([['now', 'now', 'buffer'], 0]);
  state.wait = 0x911a;
  at.t = 200;
  made.draw();
  expect(state.gets).toBe(1);
  const still = { wait: 0x911b, reads: [], gets: 0 };
  deep(lazy(still), { ...VIEW, fixed: true }).draw();
  expect([still.reads.every((one) => one === 'now'), still.gets]).toEqual([true, 0]);
});

test('every deep palette draws to the canvas in display colours, with no linear-light pow 2.2 for a post chain', () => {
  const linear = ['', 'accent', 'fire', 'ice', 'mono'].map((palette) => {
    const fake = gl();
    deep(fake, VIEW, { palette }).draw();
    const [one] = paints(fake);
    return [one.fb, /pow\(|2\.2/.test(one.fs)];
  });
  expect(linear).toEqual(Array.from({ length: 5 }, () => [null, false]));
});

const zigzag = (n) => {
  const out = new Uint8Array(64 * 64 * 4);
  const cx = n % 2 ? 38 : 26;
  for (let y = 0; y < 64; y++) {
    for (let x = 0; x < 64; x++) {
      const inside = (x === cx || x === cx + 1) && (y === 38 || y === 39);
      const q = inside ? 0 : Math.round((6 + 0.4 * x + 0.3 * y) * 8);
      out.set([q >> 8, q & 255, inside ? 128 : 0, 3], 4 * (y * 64 + x));
    }
  }
  return out;
};

const zigzags = (fake = gl()) => {
  let reads = 0;
  fake.readPixels = (x, y, w, h, format, type, into) => into && into.set(zigzag(reads++));
  fake.getBufferSubData = (where, at, into) => into.set(zigzag(reads++));
  return fake;
};

const fly = (fake, to, step, opts = {}, each = () => {}) => {
  const at = { ...VIEW, t: 0 };
  const made = deep(fake, at, { seed: 1, ...opts });
  for (let n = 0; n * step <= to; n++) {
    at.t = n * step;
    made.draw();
    each();
  }
  return paints(fake).filter((one) => one.uniforms.uAlpha === 1);
};

test('across a zigzag of targets the centre glides under the cap, its lateral speed in widths a second never jumping between frames and never stopping', () => {
  const frames = fly(zigzags(), 20000, FRAME);
  const span = (one) => Math.hypot(...one.uniforms.uU) * VIEW.h;
  const speeds = frames.slice(1).map((one, n) => (one.units[0] === frames[n].units[0] ? [0, 1].map((i) => ((one.uniforms.uOff[i] - frames[n].uniforms.uOff[i]) / span(one)) * (1000 / FRAME)) : null));
  const jumps = speeds.slice(1).flatMap((one, n) => (one && speeds[n] ? [Math.hypot(one[0] - speeds[n][0], one[1] - speeds[n][1])] : []));
  const sizes = speeds.map((one) => (one ? Math.hypot(...one) : null));
  const later = sizes.slice(240).filter((one) => one !== null);
  expect([Math.max(...jumps) < 0.005, Math.max(...sizes.filter((one) => one !== null)) <= 0.12, Math.min(...later) > 0.03, later.length > 900]).toEqual([true, true, true, true]);
});

test('the plan holds two stops ahead of the camera while each probe lands a frame late, so no read after the first two waits', () => {
  const fake = zigzags();
  const read = fake.readPixels;
  let frame = 0;
  let waits = 0;
  fake.readPixels = (...args) => {
    if (args[6] !== 0) waits += 1;
    read(...args);
  };
  fake.fenceSync = () => ({ frame });
  fake.clientWaitSync = (sync) => (frame > sync.frame ? 0x911a : 0x911b);
  fly(fake, 20000, 50, {}, () => {
    frame += 1;
  });
  expect(waits).toBe(2);
});

test('a jump in t lands on the centre that playing to it reaches, for the same probe results', () => {
  const line = (fake) => {
    const [, , , data, , length] = fake.log.findLast((one) => one[0] === 'bufferSubData');
    return Array.from(data.slice(0, length));
  };
  const played = zigzags();
  fly(played, 10000, 100, { path: 1 });
  const jumped = zigzags();
  deep(jumped, { ...VIEW, t: 10000 }, { seed: 1, path: 1 }).draw();
  expect(line(jumped)).toEqual(line(played));
});

const pen = (fake) => {
  const draws = fake.draws().filter((one) => one.vs.includes('aShape'));
  const [, , , data, , length] = fake.log.findLast((one) => one[0] === 'bufferSubData');
  const shapes = Array.from({ length: length / 12 }, (_, i) => Array.from(data.slice(i * 12, i * 12 + 12), (v) => +v.toFixed(4)));
  return { draws, last: Boolean(fake.draws().at(-1)?.vs.includes('aShape')), shapes };
};

test('path 1 draws the plan in one draw after the fractal: a 3 px accent line on a 5 px ground line, a ring at the centre and at each stop ahead, the next one half filled', () => {
  const fake = zigzags();
  deep(fake, { ...VIEW, t: 1000 }, { path: 1, palette: 'classic' }).draw();
  const { draws, last, shapes } = pen(fake);
  const line = shapes.filter((one) => one[4] === 0);
  const rings = shapes.filter((one) => one[4] > 0);
  const accent = [0, +(140 / 255).toFixed(4), 1, 1];
  expect([draws.length, last, [...new Set(line.map((one) => [one[5], ...one.slice(8)].join()))]]).toEqual([1, true, [[2.5, 0, 0, 0, 1].join(), [1.5, ...accent].join()]]);
  expect(rings.map((one) => one.slice(4, 7))).toEqual([[5, 1.75, 0], [5, 0.75, 0], [8, 2, 0], [8, 1, 0.5], ...rings.slice(4).map((one, i) => [8, i % 2 ? 1 : 2, 0])]);
  const none = zigzags();
  deep(none, { ...VIEW, t: 1000 }, { path: 0 }).draw();
  expect(none.draws().filter((one) => one.vs.includes('aShape')).length).toBe(0);
});

test('a still view holds as time runs, flies once from where it rests, and comes to rest at the bottom span', () => {
  const still = { centre: [fixed(-0.7463), fixed(0.1102)], span: 0.01, angle: 0.5 };
  const at = { ...VIEW, t: 0 };
  const made = deep(gl(), at, { seed: 1, still, bottom: 1e-4 });
  made.draw();
  at.t = 60000;
  made.draw();
  const held = [made.flying(), made.at() === still];
  made.fly();
  const from = made.at();
  at.t += 20000;
  made.draw();
  const end = made.at();
  expect([...held, from.span, +from.angle.toFixed(9), made.flying(), +end.span.toExponential(6)]).toEqual([false, true, 0.01, 0.5, false, 1e-4]);
});
