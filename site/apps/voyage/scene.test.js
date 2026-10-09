import { expect, test } from 'bun:test';
import { rng } from '../../lib/scene.js';
import { bus, engine } from '../../lib/space/audio.js';
import { project } from '../../lib/space/camera.js';
import { audio as sounds, gl } from '../../lib/space/fake.js';
import { dot, len, mul, norm, sub } from '../../lib/space/vec.js';
import { PAGE, SPEC, make, orbit, quiet } from './scene.js';
import { BAR, levels, mixer } from './score.js';

const open = (opts = {}, { paper = '#000000', webgl = true, still = false, fixed = false } = {}) => {
  const fake = gl();
  const pen = [];
  const canvas = { width: 1280, height: 720, getContext: (kind) => (kind === 'webgl2' ? (webgl ? fake : null) : { fillRect: (...args) => pen.push(args) }) };
  const view = { rand: rng(3), look: () => ({ paper, accent: '#008cff' }), still, fixed, w: 1280, h: 720, dpr: 1, t: 0 };
  return { fake, pen, view, scene: make(canvas, view, { seed: 3, player: true, ...opts }) };
};

const play = (live, from, to, step = 100) => {
  for (let t = from; t <= to; t += step) {
    live.view.t = t;
    live.scene.draw();
  }
};

const visits = (live, times) =>
  times.map((t) => {
    live.view.t = t;
    live.scene.draw();
    return [t, live.scene.here(), live.scene.state(), live.scene.stage()];
  });

const planets = (fake) => fake.draws().filter((one) => one.fs.includes('uGlare')).length;

const world = (fake) => fake.draws().filter((one) => one.fs.includes('uGlare')).at(-1);

const degrees = (a, b) => (Math.acos(Math.min(Math.max(dot(a, b), -1), 1)) * 180) / Math.PI;

const marches = (draws) => draws.filter((one) => one.fs.includes('uNear')).map((one) => [one.uniforms.uOut].flat()[0]);

const into = (live, step = 100) => {
  const deep = live.scene.system.bodies.find((one) => one.kind === 'bang');
  live.scene.select(deep.id);
  live.scene.jump();
  for (let t = live.view.t + step; t < 60000; t += step) {
    live.view.t = t;
    live.scene.draw();
    if (live.scene.here() === deep.id && live.scene.state() === 'stay') break;
  }
  return deep;
};

const listen = (opts, value = { sound: 1, music: 1 }) => {
  const ctx = sounds();
  const box = engine({ seed: 3, make: () => ctx });
  box.start();
  box.set(levels(value));
  const cues = [];
  const line = bus();
  line.attach({ at: box.at, cue: (name, t, args) => (cues.push([name, t]), box.cue(name, t, args)) });
  return { ...open({ ...opts, audio: line }), ctx, box, cues };
};

const hear = (live, from, to, step = 50) => {
  for (let t = from; t <= to; t += step) {
    live.ctx.currentTime = t / 1000;
    live.view.t = t;
    live.scene.draw();
  }
};

const kicks = (ctx) => ctx.find('Oscillator').filter((one) => one.type === 'sine' && one.frequency.value === 150).map((one) => Math.round((one.started[0] - 0.05) * 1e6) / 1e3);

const acids = (ctx) => {
  const grit = ctx.find('WaveShaper')[0];
  return ctx.find('Gain').filter((one) => one.outputs.includes(grit)).map((one) => Math.round((one.gain.calls[0][2] - 0.05) * 1e6) / 1e3);
};

const onBar = (t) => Math.abs(t / BAR - Math.round(t / BAR)) < 1e-6;

test('auto is a pure function of the seed and t: one draw at 90 s equals 900 stepped draws', () => {
  const fresh = open({ mode: 'auto' });
  const stepped = open({ mode: 'auto' });
  const seen = new Set();
  fresh.view.t = 90000;
  fresh.scene.draw();
  for (let t = 100; t <= 90000; t += 100) {
    stepped.view.t = t;
    stepped.scene.draw();
    seen.add(stepped.scene.here());
  }
  const read = (live) => {
    const { uPos, uRot } = world(live.fake).uniforms;
    return [live.scene.here(), live.scene.state(), live.scene.stage(), live.scene.picked(), live.scene.said(), uPos, uRot];
  };
  expect([read(fresh), seen.size >= 3]).toEqual([read(stepped), true]);
});

test('a fresh auto draw at 700 s lands where 100 ms play does, past a long catch-up', () => {
  const fresh = open({ seed: 2, bangs: 4, mode: 'auto' });
  const stepped = open({ seed: 2, bangs: 4, mode: 'auto' });
  fresh.view.t = 700000;
  fresh.scene.draw();
  play(stepped, 100, 700000);
  const read = (live) => [live.scene.here(), live.scene.state(), live.scene.stage(), live.scene.picked(), live.scene.said()];
  expect(read(fresh)).toEqual(read(stepped));
});

test('without a player the scene is the saver, so the lock gets the auto voyage', () => {
  const times = [10000, 40000, 70000];
  expect(visits(open({ player: false }), times)).toEqual(visits(open({ mode: 'auto' }), times));
});

test('windDown calls onDone within 1.5 s from every stage, and before the tunnel bakes no world', () => {
  const scout = open({ mode: 'auto' });
  const starts = new Map();
  for (let t = 0; t <= 30000; t += 25) {
    scout.view.t = t;
    scout.scene.draw();
    if (!starts.has(scout.scene.stage())) starts.set(scout.scene.stage(), t);
  }
  const stages = ['cruise', 'wind', 'stretch', 'pile', 'flash', 'tunnel', 'bloom', 'white', 'decay', 'snap'];
  const rows = stages.map((want) => {
    const live = open({ mode: 'auto' });
    const at = starts.get(want);
    live.view.t = at;
    live.scene.draw();
    const stage = live.scene.stage();
    const before = live.fake.draws().length;
    let done = null;
    live.scene.windDown(() => (done = live.view.t));
    play(live, at + 25, at + 1500, 25);
    const baked = live.fake.draws().slice(before).some((one) => one.fs.includes('uOff'));
    return [stage, done !== null && done - at <= 1500, stages.indexOf(want) < 5 && baked];
  });
  expect(rows).toEqual(stages.map((want) => [want, true, false]));
});

test('hold jumps from the stay to the picked world after a 400 ms charge and pins the tunnel until windDown', () => {
  const live = open();
  play(live, 0, 1000);
  const pick = live.scene.picked();
  live.scene.hold();
  const charge = [1300, 1400].map((t) => (play(live, t, t), live.scene.state()));
  play(live, 1500, 30000);
  const pinned = [live.scene.state(), live.scene.phase()];
  let done = null;
  live.scene.windDown(() => (done = live.view.t));
  play(live, 30100, 32000);
  expect([charge, pinned, done - 30000 <= 450, live.scene.here()]).toEqual([['charge', 'jump'], ['tunnel', 'hyper'], true, pick]);
});

test("a tap within 44 CSS px of a marker picks it through the hud's pick at fit's scale, and a second tap jumps", () => {
  const live = open();
  play(live, 0, 500);
  const mark = live.scene.marks().find((one) => !one.on);
  const far = [mark.x + 300, mark.y + 300];
  const missed = live.scene.tap(...far);
  const wide = live.scene.tap(...far, 20);
  const hit = live.scene.tap(mark.x + 20, mark.y - 15);
  const chosen = live.scene.picked();
  live.scene.tap(mark.x, mark.y);
  play(live, 600, 700);
  expect([missed, wide, hit, chosen, live.scene.state()]).toEqual([false, true, true, mark.id, 'charge']);
});

test('in auto a world picked from the list replaces the planned jump', () => {
  const live = open({ mode: 'auto', bangs: 0 });
  play(live, 0, 200);
  const from = live.scene.here();
  const other = live.scene.system.bodies.find((one) => one.id !== from && one.id !== live.scene.picked()).id;
  live.scene.select(other);
  for (let t = 300; t < 60000 && live.scene.here() === from; t += 100) play(live, t, t);
  expect(live.scene.here()).toBe(other);
});

test('a jump toward a world near the sun turns at least 15 degrees off the sun', () => {
  const live = open({ bangs: 0 });
  play(live, 0, 200);
  const bodies = live.scene.system.bodies;
  const here = bodies[live.scene.here()];
  const sun = norm(mul(here.pos, -1));
  const target = bodies.filter((one) => one.id !== here.id).sort((a, b) => degrees(norm(sub(a.pos, here.pos)), sun) - degrees(norm(sub(b.pos, here.pos)), sun))[0];
  live.scene.select(target.id);
  live.scene.jump();
  for (let t = 250; live.scene.state() !== 'jump'; t += 50) play(live, t, t);
  const { uRot } = world(live.fake).uniforms;
  expect([degrees(norm(sub(target.pos, here.pos)), sun) < 15, degrees([uRot[6], uRot[7], uRot[8]], sun) > 14.999]).toEqual([true, true]);
});

test("markers stay inside the title-safe box below the HUD lines, and out of the sun's glare", () => {
  const live = open({ bangs: 0 }, { fixed: true });
  play(live, 0, 500);
  const { uRot } = world(live.fake).uniforms;
  const cam = { pos: [0, 0, 0], rot: uRot, fov: 1 };
  const bodies = live.scene.system.bodies;
  const here = bodies[live.scene.here()];
  const sun = project(live.view, cam, norm(mul(here.pos, -1)));
  const burn = (0.15 * 720) / (2 * Math.tan(0.5));
  const raw = bodies.filter((one) => one.id !== here.id).map((one) => project(live.view, cam, norm(sub(one.pos, here.pos))));
  const marks = live.scene.marks();
  const box = { left: 77 + 16, right: 1280 - 77 - 16, top: 94 + 3 * 24 + 16, bottom: 720 - 130 - 16 };
  const inside = marks.every((one) => one.x >= box.left && one.x <= box.right && one.y >= box.top && one.y <= box.bottom);
  const clear = marks.every((one) => Math.hypot(Math.max(Math.abs(one.x - sun[0]) - 16, 0), Math.max(Math.abs(one.y - sun[1]) - 19, 0)) >= burn - 1e-6);
  expect([raw.some((p) => p && Math.hypot(p[0] - sun[0], p[1] - sun[1]) < burn), marks.length, inside, clear]).toEqual([true, bodies.length - 1, true, true]);
});

test("a slow stay renders the world at planets' next notch, the discs in that target's pixels", () => {
  const live = open();
  play(live, 0, 1500, 50);
  const draws = live.fake.draws();
  const discs = draws.filter((one) => one.vs.includes('aDisc.z + 1.5')).at(-1);
  expect([draws.find((one) => one.fs.includes('uGlare')).viewport, world(live.fake).viewport, discs.viewport, discs.uniforms.uRes]).toEqual([[0, 0, 1280, 720], [0, 0, 1088, 612], [0, 0, 1088, 612], [1088, 612]]);
});

test('Enter jumps, the arrows step through the other worlds and a digit picks one', () => {
  const live = open({ worlds: 6 });
  play(live, 0, 200);
  const here = live.scene.here();
  const steps = Array.from({ length: 5 }, () => (PAGE.actions.next(live.scene), live.scene.picked()));
  const digit = here === 0 ? 2 : 1;
  PAGE.actions.pick(live.scene, { key: String(digit) });
  const picked = live.scene.picked();
  PAGE.actions.jump(live.scene);
  play(live, 300, 400);
  expect([new Set(steps).size, steps.includes(here), picked, live.scene.state()]).toEqual([5, false, digit - 1, 'charge']);
  expect(PAGE.keys.map((row) => [row.act, row.button ?? false])).toEqual([['jump', true], ['prev', false], ['next', false], ['pick', false], ['mute', false]]);
});

test('the destination planet starts its links at the charge, bakes at most one map a tunnel frame, and is drawn only from the decay', () => {
  const live = open({ bangs: 0 });
  play(live, 0, 200);
  live.scene.jump();
  const rows = [];
  for (let t = 300; t <= 14000; t += 50) {
    live.view.t = t;
    const before = live.fake.draws().length;
    const logged = live.fake.log.length;
    live.scene.draw();
    const drawn = live.fake.draws().slice(before);
    const linked = live.fake.log.slice(logged).some(([key, , source]) => key === 'shaderSource' && source.includes('uGlare'));
    rows.push([live.scene.stage(), drawn.filter((one) => one.fs.includes('uGlare')).length, drawn.filter((one) => one.fs.includes('uOff')).length, linked, live.scene.state()]);
  }
  const by = (stage) => rows.filter((row) => row[0] === stage);
  const seen = (stage) => by(stage).filter((row) => row[1] > 0).length;
  const baked = rows.filter((row) => row[2] > 0);
  expect([seen('flash'), seen('tunnel'), seen('bloom'), seen('white'), seen('decay') > 0, seen('decay') < by('decay').length, seen('snap') === by('snap').length]).toEqual([0, 0, 0, 0, true, true, true]);
  expect([rows.find((row) => row[3])[4], baked[0][0], baked.length > 1, baked.every((row) => row[2] <= 6 && row[0] === 'tunnel')]).toEqual(['charge', 'tunnel', true, true]);
});

test('the arrival lands at 3.2 R facing the planet, closes to 2.6 R over 18 s, and the sun clears the limb at 8 s', () => {
  const body = { pos: [1.2, 0.05, -0.7] };
  const sun = norm(mul(body.pos, -1));
  const miss = (u) => {
    const { pos } = orbit(body, u);
    const b = dot(pos, sun);
    return [Math.sqrt(dot(pos, pos) - b * b), b];
  };
  const { pos, cam } = orbit(body, 0);
  expect([len(pos).toFixed(6), len(orbit(body, 18000).pos).toFixed(6), len(orbit(body, 40000).pos).toFixed(6)]).toEqual(['3.200000', '2.600000', '2.600000']);
  expect(dot([cam.rot[6], cam.rot[7], cam.rot[8]], norm(mul(pos, -1)))).toBeCloseTo(1, 6);
  expect(miss(8000)[0]).toBeCloseTo(1, 9);
  expect([miss(7000)[0] < 1, miss(9000)[0] > 1, miss(8000)[1] < 0]).toEqual([true, true, true]);
});

test('after the sunrise the orbit eases from 0.05 to 0.12 rad a second, so the world is half lit by 18.5 s', () => {
  const body = { pos: [1.2, 0.05, -0.7] };
  const sun = norm(mul(body.pos, -1));
  const angle = (u) => Math.acos(-dot(norm(orbit(body, u).pos), sun));
  const rate = (u) => (angle(u + 100) - angle(u)) / 0.1;
  expect([rate(5000).toFixed(3), rate(25000).toFixed(3), ((1 - Math.cos(angle(18500))) / 2).toFixed(2)]).toEqual(['0.050', '0.120', '0.50']);
});

test('the jump cues the riser, hit, drone, swell, white hit and arrival pad at their times through an engine', () => {
  const ctx = sounds();
  const box = engine({ seed: 3, make: () => ctx });
  box.start();
  const line = bus();
  const log = [];
  line.attach({ at: box.at, cue: (name, t, args) => (log.push([name, Math.round(t)]), box.cue(name, t, args)) });
  const live = open({ audio: line });
  play(live, 0, 200);
  live.scene.step(1);
  live.scene.jump();
  play(live, 300, 12000);
  const [, , , , out] = log.map(([, t]) => t);
  const from = log[0][1];
  expect(log.map(([name, t]) => [name, t - from])).toEqual([['tick', 0], ['riser', 1200], ['hit', 3000], ['drone', 3300], ['swell', out - from], ['hit', out - from + 400], ['arrive', out - from + 1700]]);
  expect(ctx.find('Oscillator').length).toBeGreaterThan(0);
});

test('sound and music leave the value the scene gets, so toggling them never remounts it', () => {
  const value = { worlds: 5, bangs: 2, mode: 'play', sound: 0, music: 1, look: 'cloud', hud: 1, seed: 9 };
  const keyed = (patch) => JSON.stringify(quiet({ ...value, ...patch }));
  expect([keyed({ sound: 1 }) === keyed({}), keyed({ music: 0 }) === keyed({}), keyed({ worlds: 6 }) === keyed({}), SPEC.map((row) => row.key)]).toEqual([true, true, false, ['worlds', 'bangs', 'mode', 'sound', 'music', 'look', 'hud', 'totem']]);
  const loud = open({ sound: 1, music: 0 });
  const calm = open({ sound: 0, music: 1 });
  play(loud, 0, 1000);
  play(calm, 0, 1000);
  expect(loud.fake.log.length).toBe(calm.fake.log.length);
});

test('totem -1 builds nothing, and totem 23 draws over the sky lit from the star', () => {
  const none = open({ totem: -1 });
  const idol = open({ totem: 23 });
  play(none, 0, 300);
  play(idol, 0, 300);
  const built = (live) => live.fake.log.some(([key, , source]) => key === 'shaderSource' && source.includes('uPlume'));
  const drawn = (live) => live.fake.draws().filter((one) => one.fs.includes('uPlume'));
  const star = norm(mul(idol.scene.system.bodies[idol.scene.here()].pos, -1));
  expect([built(none), drawn(none).length, drawn(idol).length, drawn(idol).at(-1).uniforms.uDir.map((v, i) => Math.abs(v - star[i]) < 1e-6)]).toEqual([false, 0, 4, [true, true, true]]);
});

test('the ground is black in both themes', () => {
  for (const paper of ['#000000', '#ffffff']) {
    const live = open({}, { paper });
    play(live, 0, 200);
    expect(new Set(live.fake.log.filter(([key]) => key === 'clearColor').map((call) => call.slice(1).join()))).toEqual(new Set(['0,0,0,1']));
  }
});

test('under reduced motion a jump paints the arrival at once and windDown fires at once', () => {
  const live = open({ bangs: 0 }, { still: true });
  live.scene.draw();
  const pick = live.scene.picked();
  live.scene.jump();
  let done = false;
  live.scene.windDown(() => (done = true));
  expect([live.scene.here(), live.scene.state(), done, live.view.t, planets(live.fake) > 0]).toEqual([pick, 'stay', true, 0, true]);
});

test('under reduced motion a jump to a bang world paints its frame cube still, from 2.2 n outside', () => {
  const live = open({ seed: 2, bangs: 4 }, { still: true });
  live.scene.draw();
  const deep = live.scene.system.bodies.find((one) => one.kind === 'bang');
  live.scene.select(deep.id);
  const before = live.fake.draws().length;
  live.scene.jump();
  const inner = live.scene.inside();
  expect([live.scene.here(), live.view.t, marches(live.fake.draws().slice(before)), len(inner.cam.pos).toFixed(6)]).toEqual([deep.id, 0, [1], (2.2 * deep.n).toFixed(6)]);
});

test('a bang world is built when the tunnel opens, revealed at 2.2 n from the decay, entered after 2.5 s and flown with door markers', () => {
  const live = open({ seed: 2, bangs: 4 });
  play(live, 0, 200);
  const deep = live.scene.system.bodies.find((one) => one.kind === 'bang');
  live.scene.select(deep.id);
  live.scene.jump();
  const rows = [];
  for (let t = 250; t <= 22000; t += 50) {
    live.view.t = t;
    const before = live.fake.draws().length;
    const logged = live.fake.log.length;
    live.scene.draw();
    const inner = live.scene.inside();
    rows.push({ t, stage: live.scene.stage(), state: live.scene.state(), here: live.scene.here(), built: live.fake.log.slice(logged).some(([key]) => key === 'texImage3D'), march: marches(live.fake.draws().slice(before)), away: inner ? len(inner.cam.pos) : null, said: live.scene.said(), marks: live.scene.marks() });
  }
  const land = rows.find((row) => row.here === deep.id);
  const exit = rows.filter((row) => ['decay', 'snap'].includes(row.stage));
  const enter = rows.find((row) => row.t >= land.t + 2500);
  const stay = rows.find((row) => row.here === deep.id && row.state === 'stay');
  expect([rows.find((row) => row.built).stage, rows.filter((row) => ['flash', 'tunnel', 'bloom', 'white'].includes(row.stage)).every((row) => !row.march.length), exit.some((row) => row.march.length), exit.flatMap((row) => row.march).every((out) => out === 1)]).toEqual(['tunnel', true, true, true]);
  expect([Math.abs(land.away - 2.2 * deep.n) < 0.01, Math.abs(enter.away - deep.n / 2) < 0.03, stay.t - land.t, stay.said[1].startsWith(`${deep.name} n ${deep.n} level`)]).toEqual([true, true, 4000, true]);
  expect([stay.marks.length > 0, stay.marks.every((one) => one.id >= 100 && /^[1-4]$/.test(one.label)), stay.marks.filter((one) => one.on).length]).toEqual([true, true, 1]);
});

test('inside a bang world a digit or a tap picks a door, the destinations still pick worlds, and Enter jumps out', () => {
  const live = open({ seed: 2, bangs: 4 });
  play(live, 0, 200);
  into(live);
  const mark = live.scene.marks()[0];
  const doors = [PAGE.actions.pick(live.scene, { key: '1' }), PAGE.actions.pick(live.scene, { key: '9' }), live.scene.tap(mark.x + 10, mark.y)];
  const world = live.scene.system.bodies.find((one) => one.kind === 'planet' && one.id !== live.scene.picked());
  live.scene.choose(world.id);
  const picked = live.scene.picked();
  PAGE.actions.jump(live.scene);
  const from = live.view.t;
  play(live, from + 100, from + 20000);
  expect([doors, picked, live.scene.here()]).toEqual([[true, false, true], world.id, world.id]);
});

test('hold and windDown keep working from inside a bang world', () => {
  const down = open({ seed: 2, bangs: 4 });
  into(down);
  const at = down.view.t;
  let done = null;
  down.scene.windDown(() => (done = down.view.t));
  play(down, at + 50, at + 1000, 50);
  const held = open({ seed: 2, bangs: 4 });
  into(held);
  const from = held.view.t;
  held.scene.hold();
  play(held, from + 100, from + 30000);
  expect([done !== null && done - at <= 450, held.scene.state(), held.scene.phase()]).toEqual([true, 'tunnel', 'hyper']);
});

test('auto stays 20 to 32 s in a bang world and 14 to 22 s at a planet, give or take half a bar, and holds the tunnel 6 s to a bang world', () => {
  const cues = [];
  const line = bus();
  line.attach({ at: () => {}, cue: (name, t) => cues.push([name, t]) });
  const live = open({ seed: 2, bangs: 4, mode: 'auto', audio: line });
  const where = [];
  for (let t = 0; t <= 300000; t += 100) {
    live.view.t = t;
    live.scene.draw();
    if (live.scene.here() !== where.at(-1)) where.push(live.scene.here());
  }
  const at = (name) => cues.filter((row) => row[0] === name).map((row) => row[1]);
  const [lands, risers, drones, swells] = ['arrive', 'riser', 'drone', 'swell'].map(at);
  const bodies = live.scene.system.bodies;
  const stops = lands.slice(0, -1).map((t, i) => [risers[i + 1] - 1200 - t, bodies[where[i + 1]].kind]);
  const deep = stops.filter((row) => row[1] === 'bang').map((row) => row[0]);
  const near = stops.filter((row) => row[1] === 'planet').map((row) => row[0]);
  const holds = swells.map((t, i) => t - drones[i]);
  expect([deep.length > 0, deep.every((ms) => ms >= 20000 - BAR / 2 && ms <= 32000 + BAR / 2), near.every((ms) => ms >= 14000 - BAR / 2 && ms <= 22000 + BAR / 2)]).toEqual([true, true, true]);
  expect(holds.some((ms) => Math.abs(ms - 6000) <= BAR / 2)).toBe(true);
});

test('the score lands on bar lines: the build before the tunnel, the drive at its opening, the drop at the exit', () => {
  const live = listen({ seed: 2, bangs: 4, mode: 'auto' });
  hear(live, 0, 90000);
  const at = (name) => live.cues.filter((row) => row[0] === name).map((row) => row[1]);
  const beats = kicks(live.ctx);
  const runs = [];
  for (const t of beats) {
    const last = runs.at(-1);
    if (last && t - last[1] <= BAR / 4 + 1e-6) last[1] = t;
    else runs.push([t, t]);
  }
  const silent = at('swell').every((t) => !beats.some((k) => k >= t && k < t + BAR));
  expect([at('drone').length >= 3, at('drone').every(onBar), at('swell').every(onBar), silent]).toEqual([true, true, true, true]);
  expect([runs.length >= 3, runs.every(([a, b]) => onBar(a) && onBar(b + BAR / 4))]).toEqual([true, true]);
});

test('the score plays acid inside a bang world and none at a planet', () => {
  const live = listen({ seed: 2, bangs: 4, mode: 'auto' });
  const moods = [];
  for (let t = 0; t <= 76000; t += 50) {
    hear(live, t, t);
    moods.push([t, live.scene.here(), live.scene.mood()]);
  }
  const deep = live.scene.system.bodies.filter((one) => one.kind === 'bang').map((one) => one.id);
  const inside = moods.filter(([, here, mood]) => deep.includes(here) && mood === 'acid');
  const notes = acids(live.ctx);
  const from = inside[0][0] + BAR;
  const to = inside.at(-1)[0];
  expect([inside.length > 200, notes.filter((t) => t >= from && t <= to).length > 20, notes.filter((t) => t > 2000 && t < 12000).length]).toEqual([true, true, 0]);
});

test("a take's stream carries the score and the effects, and sound 0 keeps the effects off while the music plays", () => {
  const ctx = sounds();
  const box = engine({ seed: 3, make: () => ctx });
  let value = { sound: 1, music: 1 };
  const line = mixer(() => value);
  line.attach(box);
  box.start();
  box.set(levels(value));
  const live = { ...open({ seed: 2, bangs: 0, mode: 'auto', audio: line }), ctx };
  hear(live, 0, 22000);
  const out = ctx.find('MediaStreamDestination')[0];
  const reach = (node, seen = new Set()) => node === out || (!seen.has(node) && (seen.add(node), node.outputs.some((next) => next.kind && reach(next, seen))));
  const risers = () => ctx.find('Oscillator').filter((one) => one.type === 'sawtooth' && one.detune.value === 14 && Math.abs(one.frequency.value - 440 * 2 ** (-26 / 12)) < 1e-6);
  const beats = () => ctx.find('Oscillator').filter((one) => one.type === 'sine' && one.frequency.value === 150);
  const carried = [box.stream() === out.stream, risers().length > 0 && risers().every((one) => reach(one)), beats().length > 0 && beats().every((one) => reach(one))];
  value = { sound: 0, music: 1 };
  box.set(levels(value));
  const before = [risers().length, beats().length];
  hear(live, 22050, 60000);
  expect([...carried, levels(value), risers().length === before[0], beats().length > before[1]]).toEqual([true, true, true, { sound: 0, music: 1 }, true, true]);
});

test('without WebGL2 the scene paints black and windDown lands at once', () => {
  const live = open({}, { webgl: false });
  let done = false;
  live.scene.windDown(() => (done = true));
  live.scene.draw();
  expect([done, live.pen.length, live.scene.phase()]).toEqual([true, 1, 'cruise']);
});
