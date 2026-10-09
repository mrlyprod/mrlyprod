import { expect, test } from 'bun:test';
import { rng } from '../../lib/scene.js';
import { codes } from '../../lib/space/bang.js';
import { FADE, FLOOR, SPAN } from '../../lib/space/deep.js';
import { gl } from '../../lib/space/fake.js';
import { make, pickCode } from './scene.js';

const canvas = (fake) => ({ width: 1, height: 1, getContext: (kind) => (kind === 'webgl2' ? fake : null) });

const view = (t = 0) => ({ rand: rng(7), look: () => ({ paper: '#000000', accent: '#008cff' }), still: false, w: 1280, h: 720, dpr: 1, t });

const scene = (opts, t = 0, fake = gl()) => {
  const at = view(t);
  const made = make(canvas(fake), at, { seed: 7, ...opts });
  made.draw();
  return { made, at, fake };
};

const picture = (at = 26, row = 31) => {
  const out = new Uint8Array(64 * 64 * 4);
  for (let y = 0; y < 64; y++) {
    for (let x = 0; x < 64; x++) {
      const inside = x === at && (y === row || y === row + 1);
      const q = inside ? 0 : Math.round((6 + 0.4 * x + 0.3 * y) * 8);
      out.set([q >> 8, q & 255, inside ? 128 : 0, 3], 4 * (y * 64 + x));
    }
  }
  return out;
};

const sealed = () => {
  const out = new Uint8Array(64 * 64 * 4);
  for (let i = 0; i < 64 * 64; i++) out.set([0, 0, 128, 3], 4 * i);
  return out;
};

const feed = (read) => {
  const fake = gl();
  fake.readPixels = (x, y, w, h, format, type, into) => into.set(read(fake));
  return fake;
};

const island = () => feed(() => picture());

const wander = () =>
  feed((fake) => {
    const { uOff, uU } = fake.draws().at(-1).uniforms;
    const slip = Math.abs(Math.round(uOff[0] / uU[0])) % 17;
    const octave = Math.round(-Math.log2(Math.abs(uU[0])));
    return picture(10 + ((slip + octave) % 40), 10 + ((3 * slip + octave) % 40));
  });

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

test('code -1 draws its code from the gated list by the seed', () => {
  const one = pickCode({ code: -1, n: 3 }, 5);
  expect(codes(3)).toContain(one);
  expect(pickCode({ code: -1, n: 3 }, 5)).toBe(one);
  expect(pickCode({ code: 129, n: 3 }, 5)).toBe(129);
});

test('a code the gate refuses orbits outside with no way in and no doors', () => {
  const { made } = scene({ code: 23, n: 5 });
  expect([made.open, made.marks(), made.said().at(-1)]).toEqual([false, [], 'no way in']);
});

test('set dives half an octave a second and the HUD shows the kind, the depth and auto', () => {
  const { made, at } = scene({ kind: 'set' });
  const home = made.depth();
  at.t = 4000;
  made.draw();
  expect([home, home / made.depth(), made.said()]).toEqual([SPAN, 4, ['set', 'depth 10^0', 'auto']]);
});

test('julia iterates from the pixel with a seeded c whose critical orbit stays bounded', () => {
  const { made, fake } = scene({ kind: 'julia' });
  const [cr, ci] = made.info().c;
  let [x, y] = [0, 0];
  for (let n = 0; n < 2000; n++) [x, y] = [x * x - y * y + cr, 2 * x * y + ci];
  expect(Math.hypot(x, y)).toBeLessThan(2);
  expect(fake.draws().filter((one) => 'uJulia' in one.uniforms).every((one) => one.uniforms.uJulia === 1)).toBe(true);
  expect(made.said()[0]).toBe(`julia ${cr.toFixed(3)}${ci < 0 ? '-' : '+'}${Math.abs(ci).toFixed(3)}i`);
});

test('with no detail in sight the dive fades at the floor into a new one from home', () => {
  const { made, at } = scene({ kind: 'set' });
  const floor = 2000 * Math.log2(SPAN / FLOOR);
  at.t = floor + FADE / 2;
  made.draw();
  expect(made.info().phase).toBe('fade');
  expect(made.depth() / FLOOR).toBeCloseTo(1, 9);
  at.t = floor + FADE;
  made.draw();
  expect([made.info().phase, made.info().dive, made.depth()]).toEqual(['dive', 1, SPAN]);
});

test('a digit on an island marker arrives on its minibrot, holds and starts again', () => {
  const { made, at } = scene({ kind: 'set' }, 0, island());
  const mark = made.marks().find((one) => one.kind === 'island');
  expect(made.pick(mark.number)).toBe(true);
  play(made, at, 14000);
  expect([made.info().phase, made.info().goal, made.said()[2]]).toEqual(['arrive', 3, 'minibrot p 3']);
  expect(made.depth() / SPAN).toBeCloseTo(0.019, 3);
  play(made, at, 20000);
  expect([made.info().phase, made.info().dive]).toEqual(['dive', 1]);
});

test('a jump in t keeps the depth and the marks on a probe that reads the view', () => {
  const live = scene({ kind: 'set' }, 0, wander());
  play(live.made, live.at, 60000);
  const jump = scene({ kind: 'set' }, 60000, wander());
  expect(jump.made.depth()).toBe(live.made.depth());
  expect([live.made.marks().length > 0, jump.made.marks().length > 0]).toEqual([true, true]);
});

test('past octave 24 the aim spreads over frames and ends before the next mark', () => {
  const { made, at } = scene({ kind: 'set' }, 0, wander());
  play(made, at, 47900);
  expect(made.info().aiming).toBe(false);
  play(made, at, 48000);
  expect(made.info().aiming).toBe(true);
  play(made, at, 49900);
  expect(made.info().aiming).toBe(false);
});

test('with auto 0 a probe that turns all interior steers the dive back to the last rim', () => {
  let reads = 0;
  const { made, at } = scene({ kind: 'set', auto: 0 }, 0, feed(() => (reads++ ? sealed() : picture())));
  play(made, at, 1900);
  const before = made.info().centre;
  play(made, at, 3600);
  expect(made.info().centre).not.toEqual(before);
});

test('a tap on a set marker hits within 44 px times the store px per css px', () => {
  const { made } = scene({ kind: 'set' }, 0, island());
  const mark = made.marks()[0];
  expect([made.tap(mark.x + 60, mark.y, 1), made.tap(mark.x + 60, mark.y, 2)]).toEqual([false, true]);
});

test('a digit or a tap on a marker chooses the next door', () => {
  const { made } = scene({ code: 23 });
  const marks = made.marks();
  expect(marks.length).toBeGreaterThan(1);
  const other = marks.find((one) => !one.on);
  expect(made.tap(other.x + 20, other.y)).toBe(true);
  made.draw();
  expect(made.said().at(-1)).toBe(`door ${other.number}`);
  expect([made.pick(9), made.pick(marks[0].number)]).toEqual([false, true]);
  made.draw();
  expect(made.said().at(-1)).toBe(`door ${marks[0].number}`);
});

test('without auto the flight holds before the door until a pick', () => {
  const { made, at } = scene({ code: 23, auto: 0 });
  for (const t of [30000, 60000]) {
    at.t = t;
    made.draw();
  }
  expect(made.said().slice(1)).toEqual(['n 3  level 0', 'pick a door']);
  made.go();
  at.t = 120000;
  made.draw();
  expect(made.said()[1]).not.toBe('n 3  level 0');
});

test('without auto the flight slows into the hold instead of stopping dead', () => {
  const { made, at, fake } = scene({ code: 23, auto: 0 });
  const eye = () => fake.draws().filter((one) => 'uPos' in one.uniforms).at(-1).uniforms.uPos;
  let before = eye();
  const speeds = [];
  for (let t = 50; t <= 8000; t += 50) {
    at.t = t;
    made.draw();
    const now = eye();
    if (t > 1000) speeds.push(Math.hypot(...now.map((v, i) => v - before[i])) / 0.05);
    before = now;
  }
  expect(Math.max(...speeds.slice(1).map((v, i) => Math.abs(v - speeds[i])))).toBeLessThan(0.03);
  expect([speeds[0] > 0.05, speeds.at(-1) < 0.001]).toEqual([true, true]);
});
