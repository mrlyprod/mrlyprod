import { expect, test } from 'bun:test';
import { cards, hrefOf, slugOf, sourceOf } from './scene.js';

test('the grid names no slug, a topic path names its folder, and a path outside research names none', () => {
  expect(['/research/', '/research/farey/', '/research/farey', '/research/a/b/', '/about/'].map(slugOf)).toEqual(['', 'farey', 'farey', 'a/b', '']);
});

test('a slug makes its route and its README, and a card is the title over that route', () => {
  expect([hrefOf('farey'), sourceOf('farey')]).toEqual(['/research/farey/', 'research/farey/README.md']);
  expect(cards([{ slug: 'farey', title: 'Farey', figure: 'research-farey' }])).toEqual([{ name: 'Farey', href: '/research/farey/', figure: 'research-farey' }]);
});
