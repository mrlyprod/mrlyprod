import { expect, test } from 'bun:test';
import { renderToStaticMarkup } from 'react-dom/server';
import { Keys } from './keys.jsx';

const MAP = [
  { key: ' ', label: 'Jump', act: 'jump', button: true },
  { key: 'r', label: 'Reroll', act: 'reroll' },
  { key: ['ArrowUp', 'w'], label: 'Up', act: 'up' },
  { key: '?', label: 'Keys', act: 'keys' },
];

test('the keys card lists every row of the map, its keycaps in words beside its action', () => {
  const html = renderToStaticMarkup(<Keys map={MAP} />);
  const rows = [...html.matchAll(/<dt>(.*?)<\/dt><dd>(.*?)<\/dd>/g)].map(([, caps, does]) => [[...caps.matchAll(/<kbd>([^<]+)<\/kbd>/g)].map((found) => found[1]), does]);
  expect(rows).toEqual([[['Space'], 'Jump'], [['R'], 'Reroll'], [['Up', 'W'], 'Up'], [['?'], 'Keys']]);
});
