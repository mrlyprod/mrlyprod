import { expect, test } from 'bun:test';
import { rng } from '../../lib/scene.js';
import { gl } from '../../lib/space/fake.js';
import { PAGE, make } from './scene.js';

const open = (opts = {}, { paper = '#000000', accent = '#008cff', webgl = true, bad = null } = {}) => {
  const fake = gl({ bad });
  const pen = [];
  const canvas = { width: 1280, height: 720, getContext: (kind) => (kind === 'webgl2' ? (webgl ? fake : null) : { fillRect: (...args) => pen.push(args) }) };
  const view = { rand: rng(opts.seed ?? 7), look: () => ({ paper, accent }), still: false, w: 1280, h: 720, dpr: 1, t: 0 };
  return { fake, pen, view, scene: make(canvas, view, { seed: 7, ...opts }) };
};

const play = (live, times) => {
  for (const t of times) {
    live.view.t = t;
    live.scene.draw();
  }
};

const tape = (opts, times = [0, 600, 1500, 2600]) => {
  const live = open(opts);
  const frames = [];
  live.scene.trigger();
  for (const t of times) {
    live.fake.log.length = 0;
    play(live, [t]);
    frames.push(live.fake.log.filter(([key]) => key === 'bufferSubData').map(([, , , data, , length]) => Array.from(data.slice(0, length))));
  }
  return frames;
};

test('one seed and one set of knobs paint one star field', () => {
  const knobs = { speed: 1.5, wind: 0.5 };
  expect(tape({ seed: 7, ...knobs })).toEqual(tape({ seed: 7, ...knobs }));
  expect(tape({ seed: 7, ...knobs })).not.toEqual(tape({ seed: 8, ...knobs }));
});

test('density multiplies the 6000 stars of a desk', () => {
  const stars = (opts) => open(opts).fake.log.find(([key]) => key === 'bufferData')[2] / 32;
  expect([stars({}), stars({ stars: 0.5 }), stars({ stars: 2 })]).toEqual([6000, 3000, 12000]);
});

test('an empty or NaN knob takes its default', () => {
  const plain = tape({});
  expect(tape({ speed: '', idle: NaN, stars: 'x', look: 'foam', hyper: 7 })).toEqual(plain);
});

test('the ground is black in both themes', () => {
  const night = open({}, { paper: '#000000' });
  const day = open({}, { paper: '#ffffff' });
  play(night, [0, 100]);
  play(day, [0, 100]);
  const clears = day.fake.log.filter(([key]) => key === 'clearColor').map((call) => call.slice(1));
  expect([day.fake.log.length === night.fake.log.length, new Set(clears.map(String))]).toEqual([true, new Set(['0,0,0,1'])]);
});

test('a trigger while one plays fires its onDone with the first at the white', () => {
  const live = open();
  const seen = [];
  live.scene.trigger(() => seen.push(['first', live.view.t]));
  play(live, [0, 50]);
  live.scene.trigger(() => seen.push(['second', live.view.t]));
  play(live, [1000, 3000]);
  live.view.t = 3400;
  live.scene.exit();
  play(live, [3400, 3800, 4000]);
  expect(seen).toEqual([['first', 3800], ['second', 3800]]);
});

test('stop fires every onDone still waiting and lets the context go', () => {
  const live = open();
  let done = 0;
  live.scene.trigger(() => done++);
  live.scene.trigger(() => done++);
  play(live, [0, 50]);
  live.scene.stop();
  expect([done, live.fake.log.some(([key]) => key === 'loseContext')]).toEqual([2, true]);
});

test('Enter jumps from cruise and exits once under way', () => {
  const live = open({ hyper: 'no' });
  const jump = () => PAGE.actions.jump(live.scene);
  play(live, [0]);
  jump();
  play(live, [1000]);
  const going = live.scene.phase();
  jump();
  play(live, [1050]);
  expect([going, live.scene.phase(), live.scene.stage()]).toEqual(['jump', 'exit', 'bloom']);
});

test('without WebGL2 the scene paints black and a trigger lands at once', () => {
  const live = open({}, { webgl: false });
  const seen = [];
  live.scene.trigger(() => seen.push('done'), () => seen.push('jump'));
  live.scene.draw();
  expect([seen, live.pen.length, live.scene.phase()]).toEqual([['jump', 'done'], 1, 'cruise']);
});

test('a shader that fails to compile leaves a black scene whose trigger lands at once', () => {
  const live = open({}, { bad: 'uColor' });
  const seen = [];
  live.scene.trigger(() => seen.push('done'), () => seen.push('jump'));
  live.scene.draw();
  expect([seen, live.scene.phase()]).toEqual([['jump', 'done'], 'cruise']);
});

const uniform = (live, name) => live.fake.log.filter(([key, one]) => key.startsWith('uniform') && one?.name === name).map((call) => call[2]);

const jump = (opts, shell) => {
  const live = open(opts, shell);
  live.scene.trigger();
  for (let t = 0; t <= 2400; t += 100) play(live, [t]);
  return live;
};

test('classic lays white streaks over black and builds no cloud', () => {
  const live = jump({ look: 'classic' });
  const halos = uniform(live, 'uHalo');
  const cloud = (one) => uniform(one, 'uHalo').some((halo) => [...halo].some((c) => c !== 1));
  expect([halos.length > 0, halos.every((halo) => [...halo].every((c) => c === 1)), cloud(jump({})), live.fake.draws().some(({ fs }) => fs.includes('uMilk'))]).toEqual([true, true, true, false]);
});
