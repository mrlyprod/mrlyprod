import { expect, test } from 'bun:test';
import { renderToStaticMarkup } from 'react-dom/server';
import * as math from '@mrlyprod/mrly/math';
import { defaults } from '../../lib/app/spec.js';
import { Knobs } from '../../lib/app/knobs.jsx';
import { rng } from '../../lib/rand.js';
import { SPEC, TOP, layout, make, name, scrub, study } from './scene.js';

const value = (over) => ({ ...defaults(SPEC), seed: 1, base: 2, code: '23', ...over });

const row = (rows, name) => rows.find(([one]) => one === name)?.[1];

const pen = (log) =>
  new Proxy(
    {},
    {
      get: (_, key) => (key === 'measureText' ? () => ({ width: 10 }) : (...args) => log.push([key, ...args])),
      set: (_, key, v) => {
        log.push([`${key}=`, v]);
        return true;
      },
    },
  );

const open = (over = {}) => {
  const log = [];
  const told = [];
  const view = { rand: rng(1), look: () => ({ paper: '#000000', accent: '#008cff' }), still: false, w: 800, h: 600, dpr: 1, t: 0 };
  const scene = make({ getContext: () => pen(log) }, view, { ...value(over), math, tell: (one) => told.push(one) });
  return { log, told, view, scene, count: (name) => log.filter(([key]) => key === name).length };
};

test("the page opens on the carpet's middle hexagon of side 3, its closed form and its exact split between carpet and net", () => {
  const plan = study(math, value({}));
  expect(plan.rows.slice(0, 9)).toEqual([['Name', 'carpet 23'], ['Side', 3], ['Triangles', 54], ['Closed form', '54'], ['Filled', 42], ['Empty', 12], ['Pieces', 1], ['Holes', 1], ['Largest piece', 42]]);
  expect(plan.rows.slice(-5)).toEqual([['Carpet', 42], ['Net', 12], ['Together', 54], ['Hexagon', 54], ['Partition', 'exact']]);
});

test('the side runs the odd sides to 31: the carpet falls into pieces at odd k and closes round holes at even k', () => {
  const at = (k) => study(math, value({ k })).rows;
  expect(row(at(TOP), 'Side')).toBe(31);
  expect([3, 4, 5, 6].map((k) => [k, row(at(k), 'Pieces'), row(at(k), 'Holes')])).toEqual([[3, 7, 0], [4, 1, 7], [5, 19, 0], [6, 1, 19]]);
});

test('a fractal tile grows its own level, capped by the tile, and drops the partition past level 1', () => {
  const plan = study(math, value({ tile: 3, level: 9 }));
  expect([row(plan.rows, 'Side'), row(plan.rows, 'Closed form'), row(plan.rows, 'Partition')]).toEqual([81, '39366', undefined]);
  expect(study(math, value({ tile: 5, level: 9 })).rows[1]).toEqual(['Side', 25]);
});

test('the scene is still: it paints the hexagon alone, one fill a triangle, and repaints the same canvas at once on a set', () => {
  const live = open();
  expect([live.scene.still, live.scene.every]).toEqual([true, undefined]);
  live.scene.draw();
  expect([live.count('fillRect'), live.count('fillText'), live.count('fill')]).toEqual([0, 0, 54]);
  live.log.length = 0;
  expect(live.scene.set(value({ k: 4 }))).toBe(true);
  expect([live.count('clearRect'), row(live.told.at(-1).rows, 'Holes')]).toEqual([1, 7]);
});

test('the picture takes the whole stage at any aspect, a phone included', () => {
  const [wide, tall] = [layout(1600, 900), layout(390, 844)];
  expect([wide.x, wide.w, wide.h, tall.x, tall.y, tall.w, tall.h].map((v) => +v.toFixed(1))).toEqual([36, 1528, 828, 15.6, 15.6, 358.8, 812.8]);
});

test('a drag across the stage picks the side under it, from 1 at the left edge to 16 at the right', () => {
  expect([scrub(0, 800), scrub(400, 800), scrub(799, 800), scrub(-20, 800), scrub(900, 800)]).toEqual([1, 9, TOP, 1, TOP]);
  expect(open().scene.seek(799)).toBe(TOP);
});

test('the svg holds the hexagon alone, one polygon a triangle', async () => {
  const text = await open().scene.svg();
  expect([text.startsWith('<svg'), (text.match(/<polygon /g) ?? []).length]).toEqual([true, 54]);
});

test('the level knob shows only for a tile and runs to the tile cap', () => {
  const html = (over) => renderToStaticMarkup(<Knobs spec={SPEC} value={value(over)} units={{ math }} onChange={() => {}} />);
  expect([html({}).includes('<span>Level</span>'), html({}).includes('<span>Side k</span>')]).toEqual([false, true]);
  expect([/<span>Level<\/span><span class="num">1<\/span><input type="range" min="1" max="(\d)"/.exec(html({ tile: 3 }))[1], /max="(\d)"/.exec(html({ tile: 5 }).split('<span>Level</span>')[1])[1]]).toEqual(['4', '2']);
});

test('an export is named by the design and the side or tile it shows', () => {
  expect([name(value({})), name(value({ tile: 3, level: 2 }))]).toEqual(['slices-23-k2', 'slices-23-t3-l2']);
});
