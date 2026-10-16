import { expect, test } from 'bun:test';
import { renderToStaticMarkup } from 'react-dom/server';
import * as num from '@mrlyprod/mrly/num';
import { Frame } from '../../lib/frame.jsx';
import { study } from './engine.js';
import { rows, Widget } from './widget.jsx';

const value = { seed: 1, root: 'strip', cap: 2048, ink: 'size', look: 'wash', labels: 0, tint: '' };

test('the widget mounts its stage on the units the page loaded', () => {
  const page = (units) => renderToStaticMarkup(<Frame label="Apollonian"><Widget value={value} onChange={() => {}} units={units} /></Frame>);
  expect(page(undefined)).not.toContain('<canvas');
  expect(page({ num })).toContain('<canvas');
});

test('the readouts tell the census, the Ford count against the line, the stack against Q(Q + 1)/2, the tapped circle first', () => {
  const facts = { ...study(num, 'strip', 2048).facts, circle: null };
  const said = Object.fromEntries(rows(facts));
  expect(said).toMatchObject({ root: 'strip (0, 0, 2, 2)', 'N(T)': '2448 to T = 2048', Ford: '323 of 323 on the line', stack: 'Q = 32', brightness: '528 of Q(Q + 1)/2 = 528', 'nodes lit': '323', touched: '323', missed: '0', 'off-Ford below 2048': '0', drawn: '2450', quadruples: '2449', broken: '0', strayed: '0', 'off-Ford': '0' });
  expect(said.exponent).toBeUndefined();
  expect(rows({ ...facts, circle: { k: 8, x: 4, y: 1, line: true, ford: true, num: 1, den: 2 } })[0]).toEqual(['circle', '(8, 4, 1) Ford at 1/2']);
});

test('a bounded root says it has no line', () => {
  const round = Object.fromEntries(rows(study(num, '-1,2,2,3', 32).facts));
  expect([round.line, round.Ford, round['off-Ford'], round.stack, round.brightness]).toEqual(['none in this root, so no Ford circles and no stack', undefined, undefined, undefined, undefined]);
  expect(rows(null)).toEqual([]);
});
