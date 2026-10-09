import { expect, test } from 'bun:test';
import { rng } from '../scene.js';
import { look } from './camera.js';
import { gl } from './fake.js';
import { FILM, film, hyper } from './hyper.js';
import { sky } from './sky.js';

const view = (still = false) => ({ rand: rng(7), look: () => ({ paper: '#000000', accent: '#008cff' }), still, w: 1280, h: 720, dpr: 1, t: 0 });

const open = (opts = {}, still = false) => {
  const at = view(still);
  const fake = gl();
  return { view: at, fake, trip: hyper(fake, at, { seed: 7, sky: { density: 0.1 }, ...opts }) };
};

const play = (live, times) => {
  for (const t of times) {
    live.view.t = t;
    live.trip.draw();
  }
};

const runs = (list) => list.filter((one, n) => JSON.stringify(one.slice(0, -1)) !== JSON.stringify(list[n - 1]?.slice(0, -1)));

test('film maps the old host keys onto the clock and keeps the default for junk', () => {
  expect(film({})).toEqual(FILM);
  expect(film({ wind: 0.5, run: 2, punch: 0.4, snap: 0.6, flash: 0.1, fade: 2.2 })).toEqual({ ...FILM, wind: 500, stretch: 1200, pile: 800, flash: 400, bloom: 600, white: 100, decay: 1800, snap: 400 });
  for (const junk of ['', ' ', -1, NaN, 'x', null, undefined]) expect(film({ wind: junk, run: junk, punch: junk, snap: junk, flash: junk, fade: junk })).toEqual(FILM);
  expect(film({ run: 1.5, fade: 1.1 })).toEqual(FILM);
});

test('a voyage keeps the film clock, with onJump at the tunnel and onDone at the white', () => {
  const live = open();
  const seen = [];
  const stages = [];
  live.trip.trigger(() => seen.push(['done', live.view.t]), () => seen.push(['jump', live.view.t]));
  for (let t = 0; t <= 8000; t += 50) {
    live.view.t = t;
    if (t === 6000) live.trip.exit();
    play(live, [t]);
    stages.push([live.trip.stage(), live.trip.phase(), t]);
  }
  expect(runs(stages)).toEqual([
    ['wind', 'wind', 0], ['stretch', 'jump', 300], ['pile', 'jump', 1200], ['flash', 'jump', 1800], ['tunnel', 'hyper', 2100],
    ['bloom', 'exit', 6000], ['white', 'exit', 6400], ['decay', 'exit', 6600], ['snap', 'exit', 7500], ['cruise', 'cruise', 7700],
  ]);
  expect(seen).toEqual([['jump', 2100], ['done', 6400]]);
});

test('an exit in the jump blooms at once and an early exit in the tunnel waits out its 1200 ms', () => {
  const bloom = (at) => {
    const live = open();
    live.trip.trigger();
    for (let t = 0; t <= 6000; t += 50) {
      live.view.t = t;
      if (t === at) live.trip.exit();
      play(live, [t]);
      if (live.trip.stage() === 'bloom') return t;
    }
    return null;
  };
  expect([bloom(1000), bloom(2500), bloom(4000)]).toEqual([1000, 3300, 4000]);
});

test('windDown calls onDone within 450 ms from every stage, never waiting out the tunnel', () => {
  const after = (at, voyage = true) => {
    const live = open();
    if (voyage) live.trip.trigger();
    let t = 0;
    for (; t <= at; t += 50) {
      live.view.t = t;
      if (t === 4000 && at > 4000) live.trip.exit();
      play(live, [t]);
    }
    const stage = live.trip.stage();
    let done = -1;
    live.trip.windDown(() => (done = live.view.t));
    for (; t <= at + 450 && done < 0; t += 50) play(live, [t]);
    return [stage, done >= at && done - at <= 450];
  };
  const seen = [after(1000, false), ...[100, 600, 1500, 1900, 2300].map((at) => after(at)), after(4000), ...[4100, 4450, 5000, 5600, 6500].map((at) => after(at))];
  expect(seen.map(([stage]) => stage)).toEqual(['cruise', 'wind', 'stretch', 'pile', 'flash', 'tunnel', 'tunnel', 'bloom', 'white', 'decay', 'snap', 'cruise']);
  expect(seen.every(([, ok]) => ok)).toBe(true);
});

test('windDown during an early exit still waiting out the tunnel calls onDone 400 ms on', () => {
  const live = open();
  let done = -1;
  live.trip.trigger();
  for (let t = 0; t <= 3400; t += 50) {
    live.view.t = t;
    if (t === 2500) live.trip.exit();
    if (t === 2600) live.trip.windDown(() => (done = live.view.t));
    play(live, [t]);
  }
  expect(done).toBe(3000);
});

test('hold keeps a saver voyage in the tunnel past its stay and starts a jump from cruise', () => {
  const live = open({ mode: 'yes' });
  const seen = [];
  for (let t = 0; t <= 26000; t += 100) {
    live.view.t = t;
    if (t === 15000) live.trip.hold();
    play(live, [t]);
    if (t === 15000 || t === 26000) seen.push(live.trip.stage());
  }
  const idle = open({ mode: 'no' });
  play(idle, [0, 1000]);
  idle.trip.hold();
  play(idle, [1100]);
  expect([...seen, idle.trip.stage()]).toEqual(['tunnel', 'tunnel', 'wind']);
});

test('a tunnel that fails to compile falls back to the classic look and the jump still lands', () => {
  const at = view();
  const fake = gl({ bad: 'uMilk' });
  const trip = hyper(fake, at, { seed: 7, sky: { density: 0.1 } });
  const seen = [];
  trip.trigger(() => seen.push(['done', at.t]), () => seen.push(['jump', at.t]));
  for (let t = 0; t <= 3800; t += 50) {
    at.t = t;
    if (t === 2500) trip.exit();
    trip.draw();
  }
  expect([seen, fake.draws().some(({ fs }) => fs.includes('uMilk'))]).toEqual([[['jump', 2100], ['done', 3700]], false]);
});

test('under reduced motion trigger paints the tunnel and calls both at once, and exit and windDown paint cruise at once', () => {
  const calm = open({}, true);
  const seen = [];
  calm.trip.trigger(() => seen.push('done'), () => seen.push('jump'));
  const held = calm.trip.stage();
  calm.trip.exit(() => seen.push('exit'));
  const left = calm.trip.stage();
  calm.trip.windDown(() => seen.push('down'));
  expect([held, left, seen]).toEqual(['tunnel', 'cruise', ['jump', 'done', 'exit', 'down']]);
});

test('with sky.next the exit lands on the sky of seed + 1', () => {
  const live = open({ sky: { density: 0.1, next: true }, mode: 'no' });
  live.trip.trigger();
  play(live, [0, 3000]);
  live.trip.exit();
  play(live, [3500, 4000, 5100]);
  live.fake.log.length = 0;
  play(live, [5200]);
  const drawn = live.fake.log.find(([key]) => key === 'bufferSubData');
  const fresh = gl();
  const other = sky(fresh, live.view, { seed: 8, density: 0.1 });
  fresh.viewport(0, 0, 1280, 720);
  other.draw(look([0, 0, 0], [0, 0, 1]), live.trip.flow());
  const made = fresh.log.find(([key]) => key === 'bufferSubData');
  expect(drawn[3].slice(0, drawn[5])).toEqual(made[3].slice(0, made[5]));
});

test('the flash and the white are exposure through the post chain over a black clear', () => {
  const live = open();
  live.trip.trigger();
  play(live, [0, 1000]);
  const gains = (t) => {
    live.fake.log.length = 0;
    play(live, [t]);
    return live.fake.log.filter(([key, at]) => key === 'uniform1f' && at.name === 'uGain').map((call) => call[2]);
  };
  const flash = gains(1840);
  play(live, [3000]);
  live.trip.exit();
  play(live, [3400, 3800]);
  const white = gains(3900);
  const clears = live.fake.log.filter(([key]) => key === 'clearColor').map((call) => call.slice(1));
  expect([flash, white, clears]).toEqual([[32, 32], [32, 32], [[0, 0, 0, 1]]]);
});

const voyages = (seed, mode, still = false) => {
  const at = view(still);
  const trip = hyper(gl(), at, { seed, mode, sky: { density: 0.05 } });
  const seen = [];
  let was = 'cruise';
  for (let t = 0; t <= 90000; t += 100) {
    at.t = t;
    trip.draw();
    const now = trip.phase();
    if (now !== was) seen.push([now, t]);
    was = now;
  }
  const times = (name) => seen.filter(([one]) => one === name).map(([, t]) => t);
  const starts = times('wind');
  const backs = times('cruise');
  return { starts, gaps: starts.map((t, n) => t - (backs[n - 1] ?? 0)), stays: times('exit').map((t, n) => t - times('hyper')[n]) };
};

test('the saver voyages after 12 s with yes and seeded gaps of 6 to 30 s with mix, each holding the tunnel a seeded 4 to 10 s; no and reduced motion never', () => {
  const yes = voyages(7, 'yes');
  const mix = voyages(7, 'mix');
  const fits = ({ gaps, stays }, [a, b]) => gaps.every((gap) => gap >= a && gap <= b + 100) && stays.every((stay) => stay >= 4000 && stay <= 10100);
  expect([yes.starts[0], yes.starts.length > 2, fits(yes, [12000, 12000]), mix.starts.length > 1, fits(mix, [6000, 30000])]).toEqual([12000, true, true, true, true]);
  expect(voyages(7, 'mix')).toEqual(mix);
  expect(voyages(8, 'mix')).not.toEqual(mix);
  expect([voyages(7, 'no').starts, voyages(7, 'yes', true).starts]).toEqual([[], []]);
});

test('the cues reach the audio bus 120 ms ahead of their phases', () => {
  const heard = [];
  const live = open({ audio: { cue: (name, at, args) => heard.push([name, at, live.view.t, args]) } });
  live.trip.trigger();
  for (let t = 0; t <= 3000; t += 20) play(live, [t]);
  live.trip.exit();
  for (let t = 3020; t <= 3800; t += 20) play(live, [t]);
  expect(heard).toEqual([
    ['riser', 0, 0, { dur: 1.8 }], ['hit', 1800, 1680, {}], ['drone', 2100, 1980, { dur: 4 }],
    ['swell', 3300, 3180, { dur: 0.4 }], ['hit', 3700, 3580, { white: true }],
  ]);
});

test('over draws on the scene target after the sky in every phase, before the post chain, and at() gives the phase and its progress', () => {
  const live = open();
  const seen = [];
  live.trip.trigger();
  for (const t of [0, 750, 1500, 1950, 3000]) {
    live.view.t = t;
    const before = live.fake.draws().length;
    live.trip.draw(undefined, null, (cam) => seen.push([live.trip.at(), cam.fov > 0, live.fake.draws().length - before > 0]));
    const drawn = live.fake.draws().slice(before);
    expect(drawn.at(-1).fb).toBe(null);
  }
  expect(seen.map(([at, aimed, after]) => [at.name, +at.k.toFixed(3), aimed, after])).toEqual([
    ['wind', 0, true, true],
    ['stretch', 0.5, true, true],
    ['pile', 0.5, true, true],
    ['flash', 0.5, true, true],
    ['tunnel', 0, true, true],
  ]);
});

test('after a windDown from the stretch or the pile the bloom and the white keep the world beneath, fading it out through the white', () => {
  const kept = (at) => {
    const live = open();
    const seen = [];
    live.trip.trigger();
    for (let t = 0; t <= at + 600; t += 50) {
      live.view.t = t;
      if (t === at) live.trip.windDown();
      live.trip.draw(undefined, (k) => seen.push([live.trip.stage(), +k.toFixed(3)]));
    }
    return seen.filter(([stage]) => stage === 'bloom' || stage === 'white');
  };
  const bloom = (k) => Array(8).fill(['bloom', k]);
  expect([kept(600), kept(1500), kept(3000)]).toEqual([
    [...bloom(1), ['white', 1], ['white', 1], ['white', 0.667], ['white', 0.333]],
    [...bloom(0.5), ['white', 0.5], ['white', 0.5], ['white', 0.333], ['white', 0.167]],
    [],
  ]);
});

test('the budget ticks only in the flash, tunnel and bloom, so a slow cruise opens the tunnel at the first notch', () => {
  const live = open();
  for (let t = 0; t <= 4000; t += 50) play(live, [t]);
  live.trip.trigger();
  for (let t = 4016; t <= 6400; t += 16) play(live, [t]);
  const tunnel = live.fake.draws().filter(({ fs }) => fs.includes('uMilk'));
  expect([tunnel.length > 0, tunnel[0]?.viewport]).toEqual([true, [0, 0, 960, 540]]);
});

test('draw renders the scene at the scale it is given, full size when left out', () => {
  const live = open();
  const ports = [];
  const beneath = () => ports.push(Array.from(live.fake.getParameter(live.fake.VIEWPORT)));
  live.trip.draw(undefined, beneath);
  live.trip.draw(undefined, beneath, null, 0.5);
  expect(ports).toEqual([[0, 0, 1280, 720], [0, 0, 640, 360]]);
});
