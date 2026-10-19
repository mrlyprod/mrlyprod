import { expect, mock, test } from 'bun:test';

mock.module('site:icons', () => ({ arts: {}, defs: '' }));
mock.module('site:routes', () => ({ rows: [], site: { launchpad: { sections: [], dock: [] } } }));

const { clamp } = await import('./widgets.jsx');

test('the desk menu stays inside the viewport, 8px from each edge', () => {
  const view = { w: 400, h: 300 };
  expect([clamp({ x: 100, y: 50 }, 192, 80, view), clamp({ x: 390, y: 290 }, 192, 80, view), clamp({ x: 0, y: 0 }, 500, 400, view)]).toEqual([{ x: 100, y: 50 }, { x: 200, y: 212 }, { x: 8, y: 8 }]);
});
