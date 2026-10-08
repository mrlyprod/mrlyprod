import { expect, test } from 'bun:test';
import { rng } from '../../lib/scene.js';
import { make } from './scene.js';

const pen = (log) =>
  new Proxy(
    {},
    {
      get: (_, key) => (...args) => {
        log.push([key, ...args]);
        return { addColorStop: (...stop) => log.push(['stop', ...stop]) };
      },
      set: (_, key, value) => {
        log.push([key, value]);
        return true;
      },
    },
  );

const open = (seed, still, opts) => {
  const log = [];
  const view = { rand: rng(seed), look: () => ({ paper: '#000000', accent: '#008cff' }), still, w: 640, h: 480, dpr: 1, t: 0 };
  return { log, view, scene: make({ getContext: () => pen(log) }, view, opts) };
};

const paint = (seed, opts) => {
  const { log, scene } = open(seed, true, opts);
  scene.draw();
  return log;
};

test('one seed and one set of knobs paint one star field', () => {
  const knobs = { max: 20, wind: 0.5 };
  expect(paint(7, knobs)).toEqual(paint(7, knobs));
  expect(paint(7, knobs)).not.toEqual(paint(8, knobs));
});

test('density multiplies the star count of the area rule, 400 to 1400 at 1', () => {
  const stars = (opts) => Math.round(paint(7, opts).filter(([key]) => key === 'fill').length / 20) * 10;
  expect([stars({}), stars({ stars: 0.5 }), stars({ stars: 2 })]).toEqual([400, 200, 800]);
});

const play = (live, times) => {
  for (const t of times) {
    live.view.t = t;
    live.scene.draw();
  }
};

test('an empty or NaN knob takes its default', () => {
  const tape = (opts) => {
    const live = open(7, false, opts);
    live.scene.trigger();
    play(live, [0, 600, 900]);
    return live.log;
  };
  expect(tape({ max: '' })).toEqual(tape({}));
  expect(tape({ max: NaN })).toEqual(tape({}));
});

test('trigger calls onDone at once under reduced motion', () => {
  const calm = open(7, true, {});
  let calls = 0;
  calm.scene.trigger(() => calls++);
  expect(calls).toBe(1);
});

const runs = (list) => list.filter((one, n) => one !== list[n - 1]);

test('a voyage winds, jumps, holds hyperspace until exit, then exits to cruise, with onJump at hyperspace and onDone at the flash', () => {
  const live = open(7, false, {});
  const seen = [];
  const phases = [];
  const note = (what) => () => seen.push([what, live.view.t]);
  const step = (times) => {
    for (const t of times) {
      play(live, [t]);
      phases.push(live.scene.phase());
    }
  };
  live.scene.trigger(note('done'), note('jump'));
  step([0, 100, 600, 1300, 5000, 20000]);
  live.scene.exit();
  step([20100, 20600, 21000, 22000]);
  expect([runs(phases), seen]).toEqual([['wind', 'jump', 'hyper', 'exit', 'cruise'], [['jump', 1300], ['done', 20600]]]);
});

test('a trigger while one plays fires its onDone with the first at the flash', () => {
  const live = open(7, false, {});
  const seen = [];
  live.scene.trigger(() => seen.push(['first', live.view.t]));
  play(live, [0, 50]);
  live.scene.trigger(() => seen.push(['second', live.view.t]));
  play(live, [400, 2000]);
  live.scene.exit();
  play(live, [2400, 2510, 4000]);
  expect(seen).toEqual([['first', 2510], ['second', 2510]]);
});

test('windDown calls onDone within 1.5 s: at once under reduced motion, at the exit flash from cruise or hyperspace', () => {
  const calm = open(7, true, {});
  let still = -1;
  calm.scene.windDown(() => (still = calm.view.t));
  const after = (from) => {
    const live = open(7, false, { hyper: 'no' });
    let at = -1;
    if (from) live.scene.trigger();
    play(live, [0, from]);
    live.scene.windDown(() => (at = live.view.t));
    for (let t = from; t <= from + 1500 && at < 0; t += 50) play(live, [t]);
    return at - from;
  };
  const waits = [after(0), after(3000)];
  expect([still, waits.every((wait) => wait >= 0 && wait <= 1500)]).toEqual([0, true]);
});

test('stop fires every onDone still waiting', () => {
  const live = open(7, false, {});
  let done = 0;
  live.scene.trigger(() => done++);
  live.scene.trigger(() => done++);
  play(live, [0, 50]);
  live.scene.stop();
  expect(done).toBe(2);
});

const voyages = (seed, opts, still = false) => {
  const view = { rand: rng(seed), look: () => ({ paper: '#000000', accent: '#008cff' }), still, w: 640, h: 480, dpr: 1, t: 0 };
  const scene = make({ getContext: () => pen([]) }, view, { seed, stars: 0.25, ...opts });
  const seen = [];
  let was = 'cruise';
  for (let t = 0; t <= 90000; t += 100) {
    view.t = t;
    scene.draw();
    const now = scene.phase();
    if (now !== was) seen.push([now, t]);
    was = now;
  }
  const at = (name) => seen.filter(([one]) => one === name).map(([, t]) => t);
  const starts = at('wind');
  const backs = at('cruise');
  return { starts, gaps: starts.map((t, n) => t - (backs[n - 1] ?? 0)), stays: at('exit').map((t, n) => t - at('hyper')[n]) };
};

test('hyper yes voyages after 12 s of cruise, mix after seeded gaps of 6 to 30 s, each holding hyperspace a seeded 4 to 10 s; no and reduced motion never', () => {
  const yes = voyages(7, { hyper: 'yes' });
  const mix = voyages(7, {});
  const fits = ({ gaps, stays }, [a, b]) => gaps.every((gap) => gap >= a && gap <= b + 100) && stays.every((stay) => stay >= 4000 && stay <= 10100);
  expect([yes.starts[0], yes.starts.length > 2, fits(yes, [12000, 12000]), mix.starts.length > 1, fits(mix, [6000, 30000])]).toEqual([12000, true, true, true, true]);
  expect(voyages(7, { hyper: 'mix' })).toEqual(mix);
  expect(voyages(8, { hyper: 'mix' })).not.toEqual(mix);
  expect([voyages(7, { hyper: 'no' }).starts, voyages(7, { hyper: 'yes' }, true).starts]).toEqual([[], []]);
});
