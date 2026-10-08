import { expect, test } from 'bun:test';
import { createElement } from 'react';
import { renderToString } from 'react-dom/server';
import { useQuery } from './query.js';

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
