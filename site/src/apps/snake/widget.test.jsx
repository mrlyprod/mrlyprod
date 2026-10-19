import { expect, test } from 'bun:test';
import { sample } from './widget.jsx';

const pen = (log) => new Proxy({}, { get: (_, key) => (...args) => log.push([key, ...args]), set: (_, key, v) => (log.push([key, v]), true) });

test('a piece sample draws its design once, and the random piece draws four designs in the corners', () => {
  const fills = (design) => {
    const log = [];
    sample(pen(log), design, 100, '#008cff');
    return log.filter(([verb]) => verb === 'fill').length;
  };
  expect([fills('carpet'), fills('')]).toEqual([1, 4]);
});
