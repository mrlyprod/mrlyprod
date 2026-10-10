import { expect, test } from 'bun:test';
import { drop, due, fit, OPENER, queue, scan, TILE } from './figure.js';

test('a host canvas is its css box times the device pixels, capped at 512 for a tile and 1024 for an opener, in the figure aspect', () => {
  expect(fit(150, 2, TILE)).toEqual([300, 300]);
  expect(fit(300, 3, TILE)).toEqual([512, 512]);
  expect(fit(400, 2, OPENER)).toEqual([800, 800]);
  expect(fit(700, 2, OPENER)).toEqual([1024, 1024]);
  expect(fit(700, 2, OPENER, [512, 512])).toEqual([512, 512]);
  expect(fit(300, 1, OPENER, [1200, 630])).toEqual([300, 158]);
});

test('the queue draws one host a frame, nearest first, and a route change drops the rest', async () => {
  const frames = [];
  const drawn = [];
  const line = queue({ near: (one) => one.gap, paint: (one) => drawn.push(one.name), frame: (fn) => frames.push(fn), cancel: () => {} });
  const tick = async () => {
    frames.shift()?.();
    await new Promise((done) => setTimeout(done, 0));
  };
  for (const [name, gap] of [['far', 900], ['near', 0], ['mid', 300]]) line.add({ name, gap });
  await tick();
  await tick();
  line.drop();
  await tick();
  await tick();
  expect([drawn, line.size, frames.length]).toEqual([['near', 'mid'], 0, 0]);
});

test('a seen host is due again when its ink changed or it loops, so a looping figure resumes on intersect and on a visible tab', () => {
  const host = (seen, key, loop) => ({ seen, key, loop });
  expect([due(host(true, 'a', false), 'a'), due(host(true, 'b', false), 'a'), due(host(true, 'a', true), 'a'), due(host(false, 'b', true), 'a')]).toEqual([false, true, true, false]);
});

test('a route change frees every host canvas', () => {
  const real = globalThis.IntersectionObserver;
  globalThis.IntersectionObserver = class {
    observe() {}
    disconnect() {}
  };
  const node = { width: 800, height: 800 };
  const host = { dataset: { figure: 'site-home' }, closest: () => null, querySelector: () => node, setAttribute() {}, removeAttribute() {} };
  scan({ querySelectorAll: () => [host] });
  drop();
  globalThis.IntersectionObserver = real;
  expect([node.width, node.height]).toEqual([0, 0]);
});
