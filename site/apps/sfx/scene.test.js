import { expect, test } from 'bun:test';
import { defaults } from '../../lib/knobs.js';
import { rng } from '../../lib/scene.js';
import { audio } from '../../lib/space/fake.js';
import { SPEC, analyse, band, make, seconds } from './scene.js';

const pen = (log) =>
  new Proxy(
    {},
    {
      get: (_, key) => (...args) => log.push([key, ...args]),
      set: (_, key, value) => {
        log.push([key, value]);
        return true;
      },
    },
  );

const sine = (hertz, level, length = 22050, rate = 44100) => {
  const buffer = audio().createBuffer(2, length, rate);
  for (let c = 0; c < 2; c++) buffer.getChannelData(c).set(Float32Array.from({ length }, (_, i) => level * Math.sin((2 * Math.PI * hertz * i) / rate)));
  return buffer;
};

const open = (value = {}) => {
  const log = [];
  const heard = [];
  const asked = [];
  const view = { rand: rng(7), look: () => ({ paper: '#000000', accent: '#008cff' }), still: false, w: 640, h: 360, dpr: 1, t: 0 };
  const bus = { at: (...args) => heard.push(['at', ...args]), cue: (...args) => heard.push(['cue', ...args]) };
  const render = (...args) => {
    asked.push(args);
    return Promise.resolve(sine(440, 0.5));
  };
  const scene = make({ getContext: () => pen(log) }, view, { ...defaults(SPEC), seed: 7, ...value, audio: bus, render });
  const frame = (t) => {
    view.t = t;
    log.length = 0;
    scene.draw();
    return [...log];
  };
  return { scene, view, heard, asked, frame };
};

test('play cues the effect on the bus at t with the shape of the value', () => {
  const live = open({ effect: 'exit', pitch: -3 });
  live.frame(1200);
  live.scene.cue();
  expect(live.heard).toEqual([['at', 1200], ['cue', 'exit', 1200, { pitch: -3, length: 1, bright: 0.5, drive: 0.2, space: 0.4 }]]);
});

test('the scene renders its effect once for its seconds and draws the waveform it gets', async () => {
  const live = open({ effect: 'tunnel', length: 2 });
  const blank = live.frame(0);
  await new Promise((done) => setTimeout(done, 0));
  expect(live.asked).toEqual([[seconds({ effect: 'tunnel', length: 2 }), { seed: 7, cues: [['tunnel', 0, { pitch: 0, length: 2, bright: 0.5, drive: 0.2, space: 0.4 }]] }]]);
  expect(live.frame(0).filter(([key]) => key === 'rect').length).toBeGreaterThan(blank.filter(([key]) => key === 'rect').length + 100);
});

test('the playhead moves only while the effect plays', () => {
  const live = open({ effect: 'hit' });
  const long = seconds({ effect: 'hit', length: 1 }) * 1000;
  expect(live.frame(500)).toEqual(live.frame(100));
  live.scene.cue();
  expect(live.frame(600)).not.toEqual(live.frame(900));
  expect(live.frame(500 + long + 10)).toEqual(live.frame(500 + long + 400));
});

test('analyse reads a sine at its level and in its band', () => {
  const seen = analyse(sine(1000, 0.5), 8, 48);
  expect([Math.max(...seen.peaks).toFixed(2), Math.min(...seen.peaks).toFixed(2)]).toEqual(['0.50', '-0.50']);
  const column = Array.from(seen.heat.slice(4 * 48, 5 * 48));
  expect(column.indexOf(Math.max(...column))).toBe(band(1000));
});
