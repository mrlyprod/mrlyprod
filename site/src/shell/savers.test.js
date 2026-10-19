import { expect, mock, test } from 'bun:test';

const SITE = {
  desk: 'two/still',
  savers: [
    { id: 'two/still', app: 'two', name: 'Two', title: 'Still' },
    { id: 'two/drift', app: 'two', name: 'Two', title: 'Drift' },
    { id: 'one/one', app: 'one', name: 'One', title: 'One' },
  ],
};

mock.module('site:routes', () => ({ site: SITE, rows: [] }));
mock.module('apps:scenes', () => ({ scenes: {} }));

const { KEY } = await import('./prefs.js');

const { find, on, roll, shuffle } = await import('./savers.js');

const { rng } = await import('../lib/rand.js');

test('find takes a saver id, or an app id for its first saver', () => {
  expect([find('two/drift')?.id, find('two')?.id, find('one')?.id]).toEqual(['two/drift', 'two/still', 'one/one']);
});

test('find answers null for an unknown id', () => {
  expect([find('gone'), find('two/gone'), find('')]).toEqual([null, null, null]);
});

test('on lists every saver not switched off', () => {
  globalThis.localStorage = { getItem: (key) => (key === KEY.off ? '["two/drift"]' : null) };
  try {
    expect(on()).toEqual(['two/still', 'one/one']);
  } finally {
    delete globalThis.localStorage;
  }
});

test('roll picks the same value for the same seed', () => {
  const saver = { pick: (rand) => ({ a: rand(), b: Math.floor(rand() * 100) }) };
  const one = roll(saver, 9);
  expect([one.seed, roll(saver, 9)]).toEqual([9, one]);
});

test('shuffle returns a new permutation of the ids and leaves them be', () => {
  const ids = Array.from({ length: 12 }, (_, i) => `app/${i}`);
  const kept = [...ids];
  const mixed = shuffle(ids, rng(5));
  expect([mixed !== ids, mixed.toSorted(), ids]).toEqual([true, kept.toSorted(), kept]);
});
