import { expect, test } from 'bun:test';
import { hit, label } from './keys.js';

const MAP = [
  { key: ' ', label: 'Jump', act: 'jump', button: true },
  { key: 'r', label: 'Reroll', act: 'reroll' },
  { key: ['ArrowUp', 'w'], label: 'Up', act: 'up' },
  { key: '?', label: 'Keys', act: 'keys' },
];

const el = (name) => ({ closest: (selector) => (selector.split(',').some((part) => part.trim().split(/[:[]/)[0] === name) ? {} : null) });

const press = (key, more = {}) => ({ key, target: el('canvas'), ...more });

const act = (e, open) => hit(MAP, e, open)?.act ?? null;

test('a key finds its row, letters in either case', () => {
  expect([act(press(' ')), act(press('R')), act(press('w')), act(press('ArrowUp')), act(press('?', { shiftKey: true })), act(press('x'))]).toEqual(['jump', 'reroll', 'up', 'up', 'keys', null]);
});

test('meta, ctrl or alt leave the key alone', () => {
  expect([act(press('r', { metaKey: true })), act(press('r', { ctrlKey: true })), act(press('r', { altKey: true }))]).toEqual([null, null, null]);
});

test('a key typed in a text field is ignored', () => {
  expect([act(press('r', { target: el('textarea') })), act(press('r', { target: el('input') })), act(press('r', { target: el('select') }))]).toEqual([null, null, null]);
});

test('a key on a control in a bar is left to it', () => {
  expect([act(press('ArrowUp', { target: el('.controls') })), act(press(' ', { target: el('.controls') }))]).toEqual([null, null]);
});

test('an open dialog ignores the key', () => {
  expect([act(press('r'), () => true), act(press('r'), () => false)]).toEqual([null, 'reroll']);
});

test('Space or Enter on a focused button or link is left to it', () => {
  expect([act(press(' ', { target: el('button') })), act(press(' ', { target: el('a') })), act(press('r', { target: el('button') }))]).toEqual([null, null, 'reroll']);
});

test('label turns key names into words', () => {
  expect([' ', 'ArrowLeft', 'ArrowRight', 'ArrowUp', 'ArrowDown', 'Enter', 'Escape', 'r', '?'].map(label)).toEqual(['Space', 'Left', 'Right', 'Up', 'Down', 'Enter', 'Esc', 'R', '?']);
});
