import { expect, test } from 'bun:test';
import { hit } from './keys.js';
import { keymap, live, menu, resume, toggle } from './scene.jsx';

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

test('the Export menu holds the video rows of a recording page, then the rows a widget hands up', () => {
  const began = [];
  const rows = menu(true, (at) => began.push(at), [['SVG', () => {}], ['OBJ', () => {}]]);
  expect(rows.map(([text]) => text)).toEqual(['Video 9:16', 'Video 16:9', 'SVG', 'OBJ']);
  rows[1][1]();
  expect(began).toEqual(['16:9']);
  expect(menu(false, () => {}, [['CSV', () => {}]]).map(([text]) => text)).toEqual(['CSV']);
  expect(menu(false, () => {})).toEqual([]);
});

test('a keys row whose when is false leaves the map for that value, so it is no primary, no hint and no live key', () => {
  const keys = [
    { key: 'Enter', label: 'Again', act: 'again', button: true, when: (value) => !value.roll },
    { key: ['ArrowUp', 'ArrowDown'], label: 'Reach', act: 'reach', when: (value) => !value.roll },
    { key: 's', label: 'Step', act: 'step' },
  ];
  const map = (roll) => keymap(live(keys, { roll }));
  expect([map(1).map((row) => row.label), map(0).map((row) => row.label)]).toEqual([['Pause', 'Step', 'Reroll', 'Zen', 'Keys'], ['Pause', 'Again', 'Reach', 'Step', 'Reroll', 'Zen', 'Keys']]);
  expect([hit(map(1), { key: 'Enter' }), hit(map(0), { key: 'Enter' })?.act]).toEqual([null, 'again']);
});

test('a hidden app row on r gives the key back to the base row', () => {
  const keys = [{ key: 'r', label: 'Roll', act: 'roll', when: (value) => !value.roll }];
  const act = (roll) => hit(keymap(live(keys, { roll })), { key: 'r' })?.act;
  expect([act(0), act(1)]).toEqual(['roll', 'reroll']);
});
