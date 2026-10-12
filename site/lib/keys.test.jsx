import { expect, test } from 'bun:test';
import { renderToStaticMarkup } from 'react-dom/server';
import { Hints, Info, Keys } from './keys.jsx';

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

test('two rows on one act or one label read as one hint with both caps and one card row', () => {
  const map = [
    { key: ' ', label: 'Play', act: 'pause' },
    { key: 'Enter', label: 'Play', act: 'go', button: true },
    { key: ['ArrowRight', 'ArrowDown'], label: 'Next', act: 'next' },
    { key: 'n', label: 'Next world', act: 'next' },
    { key: 'r', label: 'Reroll', act: 'reroll' },
  ];
  const caps = (html) => [...html.matchAll(/<kbd>([^<]+)<\/kbd>/g)].map((found) => found[1]);
  const plain = (html) => html.replace(/<kbd>[^<]*<\/kbd>/g, '');
  const hints = [...renderToStaticMarkup(<Hints map={map} />).matchAll(/<span>(.*?)<\/span>/g)].map(([, inner]) => [caps(inner), plain(inner)]);
  const card = [...renderToStaticMarkup(<Keys map={map} />).matchAll(/<dt>(.*?)<\/dt><dd>(.*?)<\/dd>/g)].map(([, inner, does]) => [caps(inner), does]);
  expect([hints, card]).toEqual([[[['Space', 'Enter'], 'Play'], [['Right', 'N'], 'Next'], [['R'], 'Reroll']], [[['Space', 'Enter'], 'Play'], [['Right', 'Down', 'N'], 'Next'], [['R'], 'Reroll']]]);
});

test('the info card lists each readout a widget hands up, its label beside its value', () => {
  const html = renderToStaticMarkup(<Info rows={[['Cells', 420], ['Gen', '12']]} />);
  expect(html).toContain('<dialog class="modal info" aria-label="Info">');
  expect([...html.matchAll(/<dt>(.*?)<\/dt><dd>(.*?)<\/dd>/g)].map(([, name, value]) => [name, value])).toEqual([['Cells', '420'], ['Gen', '12']]);
});

test('the info card opens with the about paragraphs, then the readouts; with no readouts it holds no list', () => {
  const html = renderToStaticMarkup(<Info about={['A carpet of cells.', 'Tap to paint.']} rows={[['Cells', 420]]} />);
  expect([...html.matchAll(/<p>([^<]+)<\/p>|<dt>([^<]+)<\/dt>/g)].map((found) => found[1] ?? found[2])).toEqual(['A carpet of cells.', 'Tap to paint.', 'Cells']);
  expect(renderToStaticMarkup(<Info about={['A carpet.']} />)).not.toContain('<dl>');
});

test('an about paragraph turns each markdown link into an anchor and keeps the text around it', () => {
  const html = renderToStaticMarkup(<Info about={['See [the Farey demo](/apollonian/) and [the note](/research/stack/).', 'Plain.']} />);
  expect([...html.matchAll(/<p>(.*?)<\/p>/g)].map((found) => found[1])).toEqual(['See <a href="/apollonian/">the Farey demo</a> and <a href="/research/stack/">the note</a>.', 'Plain.']);
});
