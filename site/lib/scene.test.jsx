import { expect, test } from 'bun:test';
import { resume, toggle } from './scene.jsx';

const handle = () => {
  const seen = [];
  return { seen, pause: () => seen.push('pause'), play: () => seen.push('play') };
};

test('Space during a take ends the take and leaves the scene alone; otherwise it flips pause', () => {
  const one = handle();
  const cuts = [];
  const cut = () => cuts.push('cut');
  const flips = [toggle(one, false, true, cut), toggle(one, false, false, cut), toggle(one, true, false, cut)];
  expect([flips, one.seen, cuts]).toEqual([[false, true, false], ['pause', 'play'], ['cut']]);
});

test('a take begun while paused plays the scene first and clears the pause', () => {
  const one = handle();
  expect([resume(one, true), resume(one, false), one.seen]).toEqual([false, false, ['play']]);
});
