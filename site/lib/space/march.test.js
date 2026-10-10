import { expect, test } from 'bun:test';
import { rgb, rng } from '../scene.js';
import { cells } from './bang.js';
import { look } from './camera.js';
import { gl } from './fake.js';
import { GRADES, LOOKS, RAMPS, boxes, march, ramp, tone } from './march.js';

const view = (extra = {}) => ({ rand: rng(7), look: () => ({ paper: '#000000', accent: '#008cff' }), still: false, w: 2560, h: 1440, dpr: 1, t: 0, ...extra });

const CAM = look([0, 0, 0.2], [0, 0, 1]);

const FRAME = { frame: 0, scale: 1 / 9, fog: 0.24, near: [0, 0, 0, 0], doors: [], hue: 0.2 };

const TONE = tone('', 7, '#008cff');

const SUNG = { code: 23, n: 3 };

const marched = (fake) => fake.draws().filter((one) => 'uLo' in one.uniforms);

const resolved = (fake) => fake.draws().filter((one) => 'uPast' in one.uniforms);

const run = (opts = {}, at = view(), fake = gl()) => ({ fake, at, one: march(fake, at, SUNG, opts) });

test('each of the three looks compiles its own define into the march, and an unknown look falls back to studio', () => {
  const heads = [...LOOKS, 'hyper'].map((look) => {
    const { fake, one } = run({ look });
    one.draw(CAM, FRAME, TONE);
    return marched(fake)[0].fs.split('\n')[1];
  });
  expect([LOOKS, Object.keys(GRADES), heads]).toEqual([['graphic', 'studio', 'haze'], LOOKS, ['#define GRAPHIC', '#define STUDIO', '#define HAZE', '#define STUDIO']]);
});

test('alpha is hard in every look: 0 only for a ray that misses the root or leaves it, 1 for a wall, a fog-lost ray or one out of steps', () => {
  for (const look of LOOKS) {
    const { fake, one } = run({ look });
    one.draw(CAM, FRAME, TONE);
    const { fs } = marched(fake)[0];
    expect([look, fs.match(/\bo = .*;/g), fs.match(/\bgone = [^;]*;/g)]).toEqual([look, ['o = vec4(col, gone ? 0.0 : 1.0);'], ['gone = shell.y < shell.x;', 'gone = t > shell.y;']]);
  }
});

test('with TAA cells under 1.5 px of the pinned target are solid, without it under 3.5 px', () => {
  const lod = (fake) => {
    const { fs, uniforms } = marched(fake)[0];
    return [/#define LOD (\S+)/.exec(fs)[1], uniforms.uRes];
  };
  const taa = run();
  taa.one.draw(CAM, FRAME, TONE);
  const raw = gl({ ext: ['WEBGL_lose_context'] });
  march(raw, view(), SUNG).draw(CAM, FRAME, TONE);
  expect([lod(taa.fake), lod(raw)[0]]).toEqual([['1.5', marched(taa.fake)[0].viewport.slice(2)], '3.5']);
});

/* MODEL */

const N = 3;
const ALL = [-1, -1, -1, -1];
const PIX = (2 * Math.tan(1.25 / 2)) / 822;
const HAZE = 5.5 / (0.24 * 0.25);

function trace(ro, rd, { scale = 1, steps = 200, lod = 1.5, near = ALL, edge = null, hull = 0, floor = 0 } = {}) {
  const { lo, hi } = boxes(cells(23, N), N);
  const [low, high] = edge ?? [[1e6, 1e6, 1e6], [1e6, 1e6, 1e6]];
  const sides = rd.map((v, i) => {
    const d = Math.abs(v) < 1e-6 ? 1e-6 : v;
    const a = (-low[i] * N - N / 2 - ro[i]) / d;
    const b = ((high[i] + 1) * N - N / 2 - ro[i]) / d;
    return [Math.min(a, b), Math.max(a, b)];
  });
  const enter = Math.max(0, ...sides.map(([a]) => a));
  const leave = Math.min(...sides.map(([, b]) => b));
  const kept = (q) => {
    if (Math.max(...q.map(Math.abs)) > 2) return q.every((v, i) => v >= -low[i] && v <= high[i]);
    const i = (q[0] + 2) * 25 + (q[1] + 2) * 5 + q[2] + 2;
    return ((near[i >> 5] >> (i & 31)) & 1) === 1;
  };
  const exit = (a, b, s) => Math.max(0, Math.min(...rd.map((v, i) => (v >= 0 ? b[i] : a[i]) / Math.max(Math.abs(v), 1e-6)))) * s;
  const probe = (p0, foot) => {
    const q = p0.map((v) => Math.floor(v / N));
    let p = p0.map((v, i) => v - q[i] * N);
    if (q.some(Boolean) && !kept(q)) return { solid: false, step: exit(p, p.map((v) => N - v), 1) };
    let s = 1;
    for (let k = 0; k < 12; k++) {
      const c = p.map((v) => Math.min(Math.max(Math.floor(v), 0), N - 1));
      const i = (c[2] * N + c[1]) * N + c[0];
      if (lo[i * 4] !== 255) {
        const from = [1, 2, 3].map((j) => lo[i * 4 + j]);
        const to = [0, 1, 2].map((j) => hi[i * 4 + j]);
        const x = s < foot ? Math.log(foot / s) / Math.log(N) : 0;
        const [low, high] = [from.map((v) => (v > 0 ? 1 : 0)), to.map((v) => (v < N ? 1 : 0))];
        const cut = to.map((v, j) => (x * (v - from[j])) / Math.max(low[j] + high[j], 1));
        const a = p.map((v, j) => v - from[j] - cut[j] * low[j]);
        const b = p.map((v, j) => to[j] - v - cut[j] * high[j]);
        return Math.min(...a, ...b) < 0 ? { solid: true } : { solid: false, step: exit(a, b, s) };
      }
      if (s < foot) break;
      p = p.map((v, j) => (v - c[j]) * N);
      s /= N;
    }
    return { solid: true };
  };
  const far = scale * HAZE;
  const start = enter * hull;
  let t = enter;
  let gone = leave < enter;
  for (let i = 0; i < steps; i++) {
    if (gone || t - start > far) break;
    const foot = Math.max(t * PIX, 1e-7);
    const at = ro.map((v, j) => v + rd[j] * t + N / 2);
    const r = probe(at, Math.max(foot * lod, floor));
    if (r.solid) return { hit: true, alpha: 1, t, steps: i + 1 };
    t += r.step + foot * 0.5;
    gone = t > leave;
  }
  return { hit: false, alpha: gone ? 0 : 1, gone, t };
}

const unit = (v) => v.map((x) => x / Math.hypot(...v));

test('the sponge never ends: a ray down the tunnel past the 5x5x5 neighbourhood hits a wall 40 cells out', () => {
  const ray = trace([0, 0, 0], unit([0.012, 0.007, 1]));
  expect([ray.hit, ray.t > 7.5 && ray.t < 60]).toEqual([true, true]);
});

test('a grazing ray 0.05 cells off a tunnel wall reaches it 25 cells on, well inside the desk step budget', () => {
  const ray = trace([0.45, 0.3, 0], unit([0.002, 0, 1]));
  expect([ray.hit, ray.t > 20, ray.steps < 100]).toEqual([true, true, true]);
});

test('a ray that leaves the root over a skim face is see-through, and one that runs out of steps stays opaque', () => {
  const sky = trace([0, 1.8, 0], unit([0.1, 1, 0.3]), { edge: [[1e6, 1e6, 1e6], [1e6, 0, 1e6]] });
  const short = trace([0.45, 0.3, 0], unit([0.002, 0, 1]), { steps: 3 });
  expect([sky.alpha, short.hit, short.alpha]).toEqual([0, false, 1]);
});

const SHIP = { near: [0, 1 << 30, 0, 0], edge: [[0, 0, 0], [0, 0, 0]], hull: 1 };

test('the ship seen from outside is its silhouette exactly: a hole and a ray passing beside it see through, a wall is opaque, from 3 to 60 cells', () => {
  const seen = [3, 10, 20, 40, 60].map((d) => [
    trace([0.1, 0.2, -d], [0, 0, 1], SHIP).alpha,
    trace([-1.2, 0.9, -d], [0, 0, 1], SHIP).alpha,
    trace([-d, 0.4, -2], unit([0.2, 0, -1]), SHIP).alpha,
  ]);
  expect(seen).toEqual(Array.from({ length: 5 }, () => [0, 1, 0]));
});

test('past the pixel limit a hole closes from its rim inward as the footprint grows by a level, so the detail never pops', () => {
  const seen = [1.5, 2.5, 3.2].map((floor) => [[-0.2, 0], [0, 0]].map(([x, y]) => trace([x, y, -2], [0, 0, 1], { ...SHIP, floor }).alpha));
  expect(seen).toEqual([[0, 0], [1, 0], [1, 1]]);
});

test('a floor of a ninth keeps the ship to three levels: its first hole stays open and a fifth-level hole shuts', () => {
  const corner = -1.5 + 1 / 54;
  const seen = [[0.1, 0.2], [corner, corner]].map(([x, y]) => [0, 1 / 9].map((floor) => trace([x, y, -2], [0, 0, 1], { ...SHIP, floor }).alpha));
  expect(seen).toEqual([[0, 0], [0, 1]]);
});

test('a draw marches into its own target with two attachments, and layer lays it over with premultiplied alpha in one fill', () => {
  const { fake, one } = run();
  one.draw(CAM, FRAME, TONE);
  const into = fake.log.filter(([key]) => key === 'framebufferTexture2D').map(([, , slot]) => slot);
  expect([into.includes(fake.COLOR_ATTACHMENT1), fake.log.some(([key, list]) => key === 'drawBuffers' && list.length === 2)]).toEqual([true, true]);
  fake.bindFramebuffer(fake.FRAMEBUFFER, 'scene');
  const before = fake.log.length;
  one.layer(0.5);
  const after = fake.log.slice(before);
  const fills = fake.draws().filter((draw) => 'uK' in draw.uniforms);
  expect([fills.length, fills[0].uniforms.uK, fills[0].fb, after.some(([key, a, b]) => key === 'blendFunc' && a === fake.ONE && b === fake.ONE_MINUS_SRC_ALPHA)]).toEqual([1, 0.5, 'scene', true]);
});

test('quality pins the march to 0.5, 1.2 or 1.8 MP with 96, 200 or 260 steps and 2, 8 or 12 AO taps, and auto is the tier', () => {
  const pinned = (quality, at = view()) => {
    const { fake, one } = run({ quality }, at);
    one.draw(CAM, FRAME, TONE);
    const draw = marched(fake)[0];
    const [, , w, h] = draw.viewport;
    return [Math.round((w * h) / 1e5) / 10, Number(/#define STEPS (\d+)/.exec(draw.fs)[1]), draw.uniforms.uTaps];
  };
  expect(['low', '', 'high'].map((q) => pinned(q))).toEqual([[0.5, 96, 2], [1.2, 200, 8], [1.8, 260, 12]]);
  expect(pinned('', view({ w: 1170, h: 2532, dpr: 3 }))).toEqual([0.3, 96, 2]);
});

test('the jitter walks a Halton (2, 3) sequence of the pixel position that repeats every 8 frames', () => {
  const { fake, one, at } = run();
  for (let i = 0; i < 16; i++) {
    at.t = i * 16;
    one.draw(CAM, FRAME, TONE);
  }
  const seen = marched(fake).map((draw) => draw.uniforms.uJitter.map((v) => +v.toFixed(4)));
  expect([seen.slice(0, 8), new Set(seen.slice(0, 8).map(String)).size, seen.every(([x, y]) => Math.abs(x) <= 0.5 && Math.abs(y) <= 0.5)]).toEqual([seen.slice(8), 8, true]);
  expect(seen[0]).toEqual([0, -0.1667]);
});

test('a step of t over 100 ms, a step back, a resize or a flip of outside drops the history, and under view.fixed a drop draws 8 jittered passes', () => {
  const live = run();
  const keeps = [];
  for (const [t, w, outside] of [[0, 2560], [16, 2560], [216, 2560], [232, 2560], [200, 2560], [216, 2560], [232, 1280], [248, 1280, true], [264, 1280, true], [280, 1280]]) {
    Object.assign(live.at, { t, w });
    const before = resolved(live.fake).length;
    live.one.draw(CAM, { ...FRAME, outside }, TONE);
    keeps.push(resolved(live.fake).slice(before).map((draw) => +draw.uniforms.uKeep.toFixed(3)));
  }
  expect(keeps).toEqual([[0], [0.9], [0], [0.9], [0], [0.9], [0], [0], [0.9], [0]]);
  const take = run({}, view({ fixed: true }));
  take.one.draw(CAM, FRAME, TONE);
  take.at.t = 40;
  take.one.draw(CAM, FRAME, TONE);
  expect([marched(take.fake).length, resolved(take.fake).map((draw) => +draw.uniforms.uKeep.toFixed(3))]).toEqual([9, [0, 0.5, 0.667, 0.75, 0.8, 0.833, 0.857, 0.875, 0.9]]);
});

test('seen from outside, the march and its resolve are scissored to the root on screen plus 3 px and the layer to that box inside it', () => {
  const { fake, one } = run();
  one.draw(look([0, 0, -20], [0, 0, 0]), { ...FRAME, scale: 1, edge: [[0, 0, 0], [0, 0, 0]], outside: true, hull: 1 }, TONE);
  const [w, h] = marched(fake)[0].viewport.slice(2);
  fake.viewport(0, 0, 730, 411);
  one.layer(1);
  const cuts = fake.log.filter(([key]) => key === 'scissor').map(([, ...box]) => box);
  expect([w, h, cuts]).toEqual([1461, 822, [[666, 347, 129, 128], [334, 174, 62, 63]]]);
});

test('across a rebase the previous camera is carried into the new frame by toFrame, and without it the history drops', () => {
  const asked = [];
  const toFrame = (q, frame) => {
    asked.push([q.j, frame]);
    return q.p.map((v) => v * 3);
  };
  const keep = (opts) => {
    const { fake, one, at } = run(opts);
    one.draw(CAM, FRAME, TONE);
    at.t = 16;
    one.draw(CAM, { ...FRAME, frame: 1 }, TONE);
    return resolved(fake).at(-1).uniforms;
  };
  const carried = keep({ toFrame });
  expect([asked, carried.uKeep, carried.uFrom.map((v) => +v.toFixed(6))]).toEqual([[[0, 1]], 0.9, [0, 0, 0.6]]);
  expect(keep({}).uKeep).toBe(0);
});

test('without a float colour buffer the march draws one pass with no history, no second attachment and no depth', () => {
  const fake = gl({ ext: ['WEBGL_lose_context'] });
  const at = view();
  const one = march(fake, at, SUNG);
  for (const t of [0, 16, 32]) {
    at.t = t;
    one.draw(CAM, FRAME, TONE);
  }
  expect([marched(fake).length, resolved(fake).length, fake.log.some(([, , slot]) => slot === fake.COLOR_ATTACHMENT1), one.depth()]).toEqual([3, 0, false, null]);
});

test('ramp gives four linear colours by palette name, by the accent, or seeded', () => {
  const linear = (hex) => rgb(hex).map((v) => (v / 255) ** 2.2 * 0.45);
  expect([ramp('fire', 7, '#008cff')[0], ramp('accent', 7, '#008cff')[2]]).toEqual([linear(RAMPS.fire[0]), linear('#008cff')]);
  expect([ramp('', 7, '#008cff').length, ramp('', 7, '#008cff')]).toEqual([4, ramp('', 7, '#ff0000')]);
  expect(ramp('', 8, '#008cff')).not.toEqual(ramp('', 7, '#008cff'));
});
