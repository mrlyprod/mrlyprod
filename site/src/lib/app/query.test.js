import { expect, test } from 'bun:test';
import { createElement } from 'react';
import { renderToString } from 'react-dom/server';
import { stamp, useQuery } from './query.js';

const read = (search, defaults) => {
  globalThis.location = { search };
  let state;
  const Probe = () => {
    [state] = useQuery(defaults);
    return null;
  };
  renderToString(createElement(Probe));
  delete globalThis.location;
  return state;
};

test('an empty query value takes the fallback', () => {
  expect(read('?max=', { max: 5 })).toEqual({ max: 5 });
  expect(read('?max=abc', { max: 5 })).toEqual({ max: 5 });
});

test('two stamps in 200 ms write the url once, with the later keys merged over the earlier', () => {
  const queued = [];
  const wrote = [];
  const real = globalThis.setTimeout;
  globalThis.setTimeout = (fn, ms) => queued.push([fn, ms]);
  globalThis.location = { search: '?a=1&c=3', pathname: '/x', hash: '' };
  globalThis.document = { documentElement: { getAttribute: () => null } };
  globalThis.history = { state: null, replaceState: (state, title, url) => wrote.push(url) };
  stamp({ a: 2 });
  stamp({ b: true, c: '' });
  const early = wrote.length;
  queued[0][0]();
  globalThis.setTimeout = real;
  for (const name of ['location', 'document', 'history']) delete globalThis[name];
  expect([queued.map(([, ms]) => ms), early, wrote]).toEqual([[200], 0, ['/x?a=2&b=1']]);
});

test('a stamp on a new page drops the keys the last page left pending', () => {
  const queued = [];
  const wrote = [];
  const real = globalThis.setTimeout;
  globalThis.setTimeout = (fn, ms) => queued.push([fn, ms]);
  globalThis.document = { documentElement: { getAttribute: () => null } };
  globalThis.history = { state: null, replaceState: (state, title, url) => wrote.push(url) };
  globalThis.location = { search: '', pathname: '/old/', hash: '' };
  stamp({ max: 20 });
  globalThis.location = { search: '', pathname: '/new/', hash: '' };
  stamp({ seed: 5 });
  queued[0][0]();
  globalThis.setTimeout = real;
  for (const name of ['location', 'document', 'history']) delete globalThis[name];
  expect(wrote).toEqual(['/new/?seed=5']);
});

test('a write while the router is busy waits and lands once it is done', () => {
  const queued = [];
  const wrote = [];
  let busy = 'true';
  const real = globalThis.setTimeout;
  globalThis.setTimeout = (fn, ms) => queued.push([fn, ms]);
  globalThis.document = { documentElement: { getAttribute: () => busy } };
  globalThis.history = { state: null, replaceState: (state, title, url) => wrote.push(url) };
  globalThis.location = { search: '', pathname: '/x/', hash: '' };
  stamp({ max: 20 });
  queued[0][0]();
  const early = wrote.length;
  busy = null;
  queued[1][0]();
  globalThis.setTimeout = real;
  for (const name of ['location', 'document', 'history']) delete globalThis[name];
  expect([queued.length, early, wrote]).toEqual([2, 0, ['/x/?max=20']]);
});
