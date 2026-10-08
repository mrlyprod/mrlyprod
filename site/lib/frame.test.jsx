import { expect, test } from 'bun:test';
import { renderToStaticMarkup } from 'react-dom/server';
import { Bar, calm, Frame, slot } from './frame.jsx';

test('on the server a frame is its stage alone: a filled Bar claims its side only after layout', () => {
  expect(renderToStaticMarkup(<Frame label="Sky"><canvas /><Bar side="right"><input /></Bar><Bar side="status">seed 7</Bar></Frame>)).toBe('<section class="frame" aria-label="Sky"><div class="canvas"><canvas></canvas></div></section>');
});

test("outside a frame the right bar lands in the shell's #right .controls and no other side lands anywhere", () => {
  const controls = { id: 'controls' };
  const doc = { querySelector: (selector) => (selector === '#right .controls' ? controls : null) };
  expect([slot(null, 'right', doc), slot(null, 'status', doc), slot(null, 'left', doc)]).toEqual([controls, null, null]);
});

test('the right bar lets go of a control a pointer changed and keeps one a key stepped', () => {
  const on = {};
  const box = { addEventListener: (name, fn) => (on[name] = fn), removeEventListener: (name) => delete on[name] };
  const blurred = [];
  const change = (by) => on.change({ target: { blur: () => blurred.push(by) } });
  const quit = calm(box);
  on.keydown();
  change('key');
  on.pointerdown();
  change('pointer');
  on.keydown();
  change('key again');
  quit();
  expect([blurred, Object.keys(on)]).toEqual([['pointer'], []]);
});
