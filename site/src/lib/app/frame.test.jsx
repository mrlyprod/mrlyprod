import { expect, test } from 'bun:test';
import { renderToStaticMarkup } from 'react-dom/server';
import { classed, find, mount, text } from '../../../test/dom.js';
import { Bar, calm, Frame, floats, pushed, slot } from './frame.jsx';

test('on the server a frame is its stage alone: a filled Bar claims its side only after layout', () => {
  expect(renderToStaticMarkup(<Frame label="Sky"><canvas /><Bar side="right"><input /></Bar><Bar side="status">seed 7</Bar></Frame>)).toBe('<section class="frame" aria-label="Sky" style="--left:280px;--right:280px"><div class="canvas"><canvas></canvas></div></section>');
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

test('open panes push the canvas by the wider one on both sides, and the panes float once the canvas would be too narrow', () => {
  const widths = { left: 240, right: 300 };
  expect([pushed({ left: true, right: false }, widths, 10), pushed({ left: true, right: true }, widths, 10), pushed({}, widths, 10)]).toEqual([260, 320, 0]);
  expect([floats(1000, 320), floats(1000, 330)]).toEqual([false, true]);
});

test('the tail bar lands after every right bar, whatever mounts first, and opens the right pane on its own', async () => {
  const pane = await mount(async (draw, host) => {
    const at = () => find(host, classed('right'));
    await draw(<Frame label="Sky"><Bar side="tail">seed</Bar></Frame>);
    const alone = text(at());
    await draw(<Frame label="Sky"><Bar side="tail">seed</Bar><Bar side="right">look</Bar></Frame>);
    return [alone, text(at())];
  });
  expect(pane).toEqual(['seed', 'lookseed']);
});
