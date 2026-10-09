import { expect, test } from 'bun:test';
import { defaults } from '../../lib/knobs.js';
import { rng } from '../../lib/scene.js';
import { tempo } from '../../lib/space/audio.js';
import { SPEC, make, song } from './scene.js';

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

const open = (value = {}) => {
  const log = [];
  const heard = [];
  const view = { rand: rng(value.seed ?? 7), look: () => ({ paper: '#000000', accent: '#008cff' }), still: false, w: 800, h: 450, dpr: 1, t: 0 };
  const scene = make({ getContext: () => pen(log) }, view, { ...defaults(SPEC), seed: 7, ...value, audio: { at: (...args) => heard.push(args) } });
  const frame = (t) => {
    view.t = t;
    log.length = 0;
    scene.draw();
    return [...log];
  };
  return { heard, frame };
};

test('each draw hands the bus t and the drive mood with the song of the value', () => {
  const live = open({ bpm: 120, key: 'd' });
  live.frame(0);
  live.frame(16);
  const tune = song({ ...defaults(SPEC), seed: 7, bpm: 120, key: 'd' });
  expect(live.heard).toEqual([[0, { name: 'drive', song: tune }], [16, { name: 'drive', song: tune }]]);
});

test('the grid holds within a step and moves at the next', () => {
  const live = open();
  const time = tempo(song({ ...defaults(SPEC), seed: 7 }));
  const at = (i) => time.time(i) + 50;
  expect(live.frame(at(5) + 5)).toEqual(live.frame(at(5) + 60));
  expect(live.frame(at(6) + 5)).not.toEqual(live.frame(at(5) + 5));
});

test('one seed draws one grid; another seed another', () => {
  expect(open({ seed: 7 }).frame(900)).toEqual(open({ seed: 7 }).frame(900));
  expect(open({ seed: 8 }).frame(900)).not.toEqual(open({ seed: 7 }).frame(900));
});
