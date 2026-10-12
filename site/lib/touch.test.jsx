import { expect, test } from 'bun:test';
import { renderToStaticMarkup } from 'react-dom/server';
import { useTouch } from './touch.js';

test('a pointer hands the scene its spot as it goes down and moves, and the lift after it, each patch to the page', () => {
  let hands = null;
  const Probe = ({ handle, change }) => {
    hands = useTouch(handle, change);
    return null;
  };
  const heard = [];
  const sent = [];
  const handle = { current: { touch: (u, v, start) => (heard.push(['touch', u, v, start]), start ? null : { u }), lift: (u, v) => (heard.push(['lift', u, v]), { up: true }) } };
  renderToStaticMarkup(<Probe handle={handle} change={{ current: (one) => sent.push(one) }} />);
  const box = { getBoundingClientRect: () => ({ left: 0, top: 0, width: 200, height: 100 }), setPointerCapture: () => {} };
  const at = (x, y) => ({ button: 0, clientX: x, clientY: y, pointerId: 1, currentTarget: box });
  hands.onPointerMove(at(10, 10));
  hands.onPointerDown(at(50, 50));
  hands.onPointerMove(at(100, 50));
  hands.onPointerUp(at(100, 50));
  hands.onPointerUp(at(100, 50));
  expect(heard).toEqual([['touch', 0.25, 0.5, true], ['touch', 0.5, 0.5, false], ['lift', 0.5, 0.5]]);
  expect(sent).toEqual([{ u: 0.5 }, { up: true }]);
});
