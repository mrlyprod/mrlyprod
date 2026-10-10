import { expect, test } from 'bun:test';
import { rng } from '../../lib/scene.js';
import { codes, route } from '../../lib/space/bang.js';
import { project } from '../../lib/space/camera.js';
import { gl } from '../../lib/space/fake.js';
import { dot, len, sub } from '../../lib/space/vec.js';
import { PAGE, SPEC, approach, come, entry, inside, leave, make, quiet, roll, shine } from './scene.js';
import { arrange, clock } from './score.js';

const L = 60000;
const RANDOM = { from: '', to: '', ship: -1 };
const VIEW = { w: 1280, h: 720 };

const open = (opts = {}, { still = false, webgl = true } = {}) => {
  const fake = gl();
  const pen = [];
  const canvas = { width: 1280, height: 720, getContext: (kind) => (kind === 'webgl2' ? (webgl ? fake : null) : { fillRect: (...args) => pen.push(args) }) };
  const view = { rand: rng(3), look: () => ({ paper: '#000000', accent: '#008cff' }), still, fixed: false, w: 1280, h: 720, dpr: 1, t: 0 };
  return { fake, pen, view, scene: make(canvas, view, { seed: 3, ...opts }) };
};

const play = (live, times) =>
  times.map((t) => {
    live.view.t = t;
    live.scene.draw();
    return [live.scene.state(), live.scene.loop()];
  });

const share = (shot) => {
  const p = project(VIEW, shot.cam, shot.pose.at);
  return [p[0] / VIEW.w, p[1] / VIEW.h, (shot.pose.size / p[2]) * (VIEW.h / (2 * Math.tan(shot.cam.fov / 2))) / VIEW.w];
};

const ahead = (cam) => [...cam.rot.slice(6, 9)];

const ground = (cam, x) => {
  const m = cam.rot;
  const tan = Math.tan(cam.fov / 2);
  let hits = 0;
  for (let y = 0; y < VIEW.h; y++) {
    const u = ((x - VIEW.w / 2) / VIEW.h) * 2 * tan;
    const v = ((VIEW.h / 2 - y) / VIEW.h) * 2 * tan;
    const d = [0, 1, 2].map((i) => m[6 + i] + u * m[i] + v * m[3 + i]);
    const b = dot(cam.pos, d) / len(d);
    hits += b < 0 && b * b - dot(cam.pos, cam.pos) + 1 >= 0 ? 1 : 0;
  }
  return Math.round((hits / VIEW.h) * 100);
};

test('the clock jumps at 9.9 s into a tunnel at 12 s, enters the ship at 14 s, exits 9.7 s and arrives 8 s before the end, then 1.5 s of black', () => {
  expect([clock(30000), clock(L)]).toEqual([
    { fire: 9900, open: 12000, enter: 14000, out: 20300, arrive: 22000, end: 30000, next: 31500 },
    { fire: 9900, open: 12000, enter: 14000, out: 50300, arrive: 52000, end: 60000, next: 61500 },
  ]);
});

test('the arrangement is silent from the cut at 12 s and drops into hyper at 12.45 s', () => {
  const at = arrange(L);
  expect([at(12100).name, at(12500).name]).toEqual(['cut', 'hyper']);
});

test('the outro carries the arp to ARRIVE + 6 s, pad and sub alone to the end, and the 1.5 s of black is a cut', () => {
  const at = arrange(L);
  const { arrive, end, next } = clock(L);
  const seen = [arrive + 5900, arrive + 6100, end - 100, end + 100, next - 100, next + 100].map((t) => {
    const one = at(t);
    return [one.name, Boolean(one.arp)];
  });
  expect(seen).toEqual([['outro', true], ['outro', false], ['outro', false], ['cut', false], ['cut', false], ['intro', false]]);
});

test('a random destination is never the type of a random departure', () => {
  const pairs = Array.from({ length: 20 }, (_, seed) => roll(RANDOM, seed, 0));
  expect(pairs.filter(({ a, b }) => a.kind === b.kind)).toEqual([]);
});

test('a fixed from=earth stays the same world across three loops', () => {
  const worlds = [0, 1, 2].map((loop) => roll({ ...RANDOM, from: 'earth' }, 7, loop).a);
  expect([worlds[0].name, new Set(worlds.map((one) => JSON.stringify(one))).size]).toEqual(['earth', 1]);
});

test('a ship of -1 or 0 is drawn from the gated corridor codes and a hand-set ship stays', () => {
  const drawn = [-1, 0].flatMap((ship) => Array.from({ length: 6 }, (_, loop) => roll({ ...RANDOM, ship }, 5, loop).code));
  expect([drawn.every((code) => codes(3).includes(code)), new Set(drawn).size > 1, roll({ ...RANDOM, ship: 23 }, 5, 4).code]).toEqual([true, true, 23]);
});

test('the departure sits at 1.1 R with the planet filling two thirds of the centre column and the sun 95 degrees off the view', () => {
  const shot = leave(roll(RANDOM, 7, 0), 1000);
  const fwd = [...shot.cam.rot.slice(6, 9)];
  expect([len(shot.cam.pos).toFixed(3), ground(shot.cam, VIEW.w / 2), ground(shot.cam, 0), Math.round((Math.acos(dot(shot.sun, fwd)) * 180) / Math.PI)]).toEqual(['1.100', 67, 52, 95]);
});

test('the ship rises into the frame from below and sits just right of the centre at the camera height at 8 s', () => {
  const rolled = roll(RANDOM, 7, 0);
  const [below, risen, passed] = [1400, 3600, 8000].map((u) => share(leave(rolled, u)));
  expect([below[1] > 1, risen[1] < 0.75, passed[0] > 0.5 && passed[0] < 0.55, len(leave(rolled, 8000).pose.at).toFixed(3)]).toEqual([true, true, true, '1.100']);
});

test('the chase settles behind the ship looking at the destination, a point within 2 degrees of the view at 9.9 s', () => {
  const shot = leave(roll(RANDOM, 7, 0), 9900);
  const angle = (Math.acos(Math.min(dot(shot.goal, [...shot.cam.rot.slice(6, 9)]), 1)) * 180) / Math.PI;
  const [x, y] = share(shot);
  expect([angle < 2, x > 0 && x < 1 && y > 0 && y < 1]).toEqual([true, true]);
});

test('the camera rides the chase into the jump, then one Hermite from 12 s takes it on into the entry key at 14 s with its position, heading and speed', () => {
  const rolled = roll(RANDOM, 3, 0);
  const door = entry(route({ code: rolled.code, n: 3 }, 3));
  const { open: start, enter } = clock(L);
  const span = enter - start;
  const chase = (u) => inside(leave(rolled, u), door).eye;
  const near = (a) => approach(rolled, door, a).cam;
  const [held, first] = [chase(start), near(0)];
  const before = len(sub(held.pos, chase(start - 1).pos));
  const after = len(sub(near(1).pos, first.pos));
  const speed = len(sub(near(span - 1e-3).pos, near(span - 1).pos)) / 0.999e-3;
  expect([len(sub(first.pos, held.pos)) < 1e-9, dot(ahead(first), ahead(held)) > 0.9999, Math.abs(after / before - 1) < 0.01, len(sub(near(span - 1e-6).pos, door.state.pos)) < 1e-4, Math.abs(speed / len(door.speed) - 1) < 0.01, dot(ahead(near(span - 1e-6)), door.state.fwd) > 0.9999]).toEqual([true, true, true, true, true, true]);
});

test('the ship is marched at its true distance into the full target and laid unshifted, and its history drops at 14 s and at the white', () => {
  const live = open();
  const { out } = clock(L);
  const rows = [3000, 3016, 6000, 6016, 11000, 11016, 13968, 13984, 14000, 14016, out + 568, out + 584, out + 600, out + 616].map((t) => {
    const before = live.fake.draws().length;
    play(live, [t]);
    const drawn = live.fake.draws().slice(before);
    const shot = drawn.find((one) => one.fs.includes('probe('));
    const laid = drawn.find((one) => one.fs.includes('texture(uSrc, v) * uK'));
    return [shot.viewport, laid.viewport.slice(0, 2), drawn.find((one) => 'uPast' in one.uniforms).uniforms.uKeep];
  });
  expect(rows.map(([port]) => port.join())).toEqual(rows.map(() => '0,0,1280,720'));
  expect(rows.map(([, at]) => at.join())).toEqual(rows.map(() => '0,0'));
  expect(rows.map(([, , keep]) => keep)).toEqual([0, 0.9, 0, 0.9, 0, 0.9, 0, 0.9, 0, 0.9, 0, 0.9, 0, 0.9]);
});

test('over the approach the haze sun, the door glow and the three-level floor ease into the route frame, so nothing steps at 14 s', () => {
  const rolled = roll(RANDOM, 3, 0);
  const door = entry(route({ code: rolled.code, n: 3 }, 3));
  const [first, last] = [0, 2000 - 1e-3].map((a) => approach(rolled, door, a).frame);
  const glow = (frame) => frame.doors.reduce((sum, one) => sum + one.glow, 0);
  expect([first.sun, first.floor, glow(first), last.sun < 1e-6, last.floor < 1e-5, Math.abs(glow(last) - glow(door.state)) < 1e-6, glow(door.state) > 0]).toEqual([1, 1 / 9, 0, true, true, true, true]);
});

test('the arrival holds the ship 8% wide just right of the limb, then it pulls away to the lower left at 2% by the fade', () => {
  const rolled = roll(RANDOM, 7, 0);
  const [x, y, wide] = share(come(rolled, 0, 0, VIEW));
  const [x2, y2, small] = share(come(rolled, clock(L).end - 1200 - clock(L).arrive, 0, VIEW));
  const shot = come(rolled, 3000, 0, VIEW);
  expect([wide.toFixed(2), x > 0.5 && x < 0.56, Math.abs(y - 0.5) < 0.02, small.toFixed(2), x2 < x && y2 > y, ground(shot.cam, 0.43 * VIEW.w) > 0, ground(shot.cam, 0.47 * VIEW.w)]).toEqual(['0.08', true, true, '0.02', true, true, 0]);
});

test('draws at 6, 11, 13 and 20 s, L - 5 s and L + 2 s land in the departure, jump, approach, tunnel, arrival and the next departure', () => {
  expect(play(open(), [6000, 11000, 13000, 20000, L - 5000, L + 2000])).toEqual([['depart', 0], ['jump', 0], ['approach', 0], ['tunnel', 0], ['arrive', 0], ['depart', 1]]);
});

test('one march draws the ship from outside before, through and after the jump with no cut at 12 s, and flies inside it after 14 s', () => {
  const live = open();
  const marched = (t) => {
    const before = live.fake.draws().length;
    play(live, [t]);
    return live.fake.draws().slice(before).find((one) => one.fs.includes('uHull'))?.uniforms;
  };
  const seen = [3000, 11000, 11995, 12005, 13000, 20000, L - 8000].map(marched);
  const [held, first] = [seen[2], seen[3]];
  const turned = Math.max(...[...held.uRot].map((v, i) => Math.abs(v - first.uRot[i])));
  expect([seen.map((one) => one.uHull), len(sub(held.uPos, first.uPos)) < 0.06, turned < 0.01, live.fake.draws().some((one) => one.fs.includes('uStretch'))]).toEqual([[1, 1, 1, 1, 1, 0, 1], true, true, false]);
});

test('the nose heat glows under the ship, so the sponge covers it and it shows round the hull and through holes, and the plume over it', () => {
  const live = open();
  const order = (t) => {
    const before = live.fake.draws().length;
    play(live, [t]);
    return live.fake
      .draws()
      .slice(before)
      .filter((one) => one.fs.includes('uHeat') || one.fs.includes('texture(uSrc, v) * uK'))
      .map((one) => ('uHeat' in one.uniforms ? (one.uniforms.uHeat > 0 ? 'heat' : 'plume') : 'ship'));
  };
  expect([order(2000), order(11000)]).toEqual([['heat', 'ship'], ['ship', 'plume']]);
});

test('at 7 s the ship takes planetshine from below: the star light at a quarter, times the lit fraction, over the squared distance', () => {
  const live = open();
  play(live, [7000]);
  const sent = live.fake.draws().findLast((one) => one.fs.includes('uShine')).uniforms;
  const shot = leave(live.scene.rolled(), 7000);
  const want = shine(live.scene.rolled().a, shot.pose.at, shot.sun);
  expect([sent.uShine.map((v, i) => Math.abs(v - want[i]) < 1e-6), Math.max(...want) > 0.1, Math.abs(len(sent.uBelow) - 1) < 1e-9]).toEqual([[true, true, true], true, true]);
});

test('a catch-up from 0 to L - 5 s in one draw lands in the arrival with planet B beneath', () => {
  const live = open();
  const begun = performance.now();
  live.view.t = L - 5000;
  live.scene.draw();
  const planets = live.fake.draws().filter((one) => one.fs.includes('uGlare'));
  expect([live.scene.state(), live.scene.stage(), planets.length > 0, performance.now() - begun < 1000]).toEqual(['arrive', 'cruise', true, true]);
});

test('windDown calls onDone within 1.5 s from every act', () => {
  const acts = [6000, 11000, 13000, 20000, L - 9000, L - 5000, L + 500];
  const rows = acts.map((at) => {
    const live = open();
    play(live, [at]);
    const act = live.scene.state();
    let done = null;
    live.scene.windDown(() => (done = live.view.t));
    play(live, [at + 100, at + 500, at + 1000, at + 1500]);
    return [act, done !== null && done - at <= 1500];
  });
  expect(rows).toEqual([['depart', true], ['jump', true], ['approach', true], ['tunnel', true], ['exit', true], ['arrive', true], ['black', true]]);
});

test('hold from the departure jumps after a 400 ms blend and pins the tunnel; from the black it jumps from the next loop', () => {
  const early = open();
  play(early, [3000]);
  early.scene.hold();
  early.scene.hold();
  const pinned = play(early, [3300, 3500, 6000, 13000, 20000, 90000]).map(([act]) => act);
  const late = open();
  play(late, [L + 500]);
  late.scene.hold();
  expect([pinned, early.scene.phase(), play(late, [L + 700, L + 1000])]).toEqual([['depart', 'jump', 'approach', 'tunnel', 'tunnel', 'tunnel'], 'hyper', [['depart', 1], ['jump', 1]]]);
});

test('the scene books the riser at 9.6 s, the hit at 12 s, then the exit riser and the white hit, each 120 ms ahead', () => {
  const booked = [];
  const audio = { at: () => {}, cue: (name, when) => booked.push([name, when, live.view.t]) };
  const live = open({ length: 30, audio });
  const { out } = clock(30000);
  play(live, [...Array.from({ length: 13 }, (_, i) => 9000 + i * 250), ...Array.from({ length: 6 }, (_, i) => out - 250 + i * 150)]);
  expect(booked.map(([name, when, t]) => [name, when, when - t >= 0 && when - t <= 120])).toEqual([['riser', 9600, true], ['hit', 12000, true], ['riser', out, true], ['hit', out + 400, true]]);
});

test('under reduced motion the still is the departure at 6 s, and windDown lands at once', () => {
  const live = open({}, { still: true });
  live.scene.draw();
  let done = false;
  live.scene.windDown(() => (done = true));
  expect([live.scene.state(), done, live.view.t, live.fake.draws().some((one) => one.fs.includes('uGlare'))]).toEqual(['depart', true, 0, true]);
});

test('sound and music leave the value the scene gets, so toggling them never restarts the film', () => {
  const value = { from: '', to: '', ship: -1, length: 60, look: 'studio', sound: 0, music: 1, seed: 9 };
  const keyed = (patch) => JSON.stringify(quiet({ ...value, ...patch }));
  expect([keyed({ sound: 1 }) === keyed({}), keyed({ music: 0 }) === keyed({}), keyed({ length: 90 }) === keyed({}), SPEC.map((row) => row.key)]).toEqual([true, true, false, ['from', 'to', 'ship', 'length', 'look', 'sound', 'music']]);
});

test('Enter is the Again button and m the sound', () => {
  expect(PAGE.keys.map((row) => [row.key, row.act, row.button ?? false])).toEqual([['Enter', 'again', true], ['m', 'mute', false]]);
});

test('without WebGL2 the scene paints black and windDown lands at once', () => {
  const live = open({}, { webgl: false });
  let done = false;
  live.scene.windDown(() => (done = true));
  live.scene.draw();
  expect([done, live.pen.length]).toEqual([true, 1]);
});
