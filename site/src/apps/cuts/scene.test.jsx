import { expect, test } from 'bun:test';
import { renderToStaticMarkup } from 'react-dom/server';
import * as math from '@mrlyprod/mrly/math';
import { defaults } from '../../lib/app/spec.js';
import { Knobs } from '../../lib/app/knobs.jsx';
import { rng } from '../../lib/rand.js';
import { SPEC, heights, layout, lid, make, name, profile, scrub, study } from './scene.js';

const value = (over) => ({ ...defaults(SPEC), seed: 1, base: 2, code: '126', height: '', ...over });

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
  return { log, told, scene, count: (name) => log.filter(([key]) => key === name).length };
};

test('the gasket cube meets 3^level cells at every height of its support, and opens on the central height', () => {
  const cut = profile(math, '126', 2, 5);
  expect([cut.low, cut.high, cut.least, cut.most, cut.central, cut.side]).toEqual([31, 62, 243, 243, [46, 47], 32]);
  expect([heights(cut, {}), heights(cut, { height: '3' }), heights(cut, { height: '50' }), heights(cut, { height: '99', both: 0 }), heights(cut, { both: 1 })]).toEqual([[46], [31], [50], [62], [46, 47]]);
});

test('the readings name the height, its count and its offset in binary, and count the points drawn', () => {
  const plan = study(math, value({ height: 50 }));
  expect(plan.rows.slice(0, 4)).toEqual([['Name', 'code 126'], ['Height', '50'], ['This slice', '243'], ['Offset in binary', '10011']]);
  expect([row(plan.rows, 'Non-empty'), row(plan.rows, 'Points drawn'), row(study(math, value({ both: 1 })).rows, 'Points drawn')]).toEqual(['32 of 32', '243', '486']);
  expect([row(study(math, value({ view: 'section' })).rows, 'Points drawn'), study(math, value({ code: '0' })).rows.at(-1)]).toEqual(['-', ['Support', 'none']]);
});

test('points reach level 9 and the section level 6', () => {
  expect([lid(math, 'points'), lid(math, 'section')]).toEqual([9, 6]);
  const html = (view) => /<span>Level<\/span><span class="num">\d+<\/span><input type="range" min="1" max="(\d)"/.exec(renderToStaticMarkup(<Knobs spec={SPEC} value={value({ view })} units={{ math }} onChange={() => {}} />))[1];
  expect([html('points'), html('section')]).toEqual(['9', '6']);
});

test('the scene is still, and the stage holds the cut alone: no bars beside the points, and the section paints as triangles', () => {
  const live = open();
  expect([live.scene.still, live.scene.every]).toEqual([true, undefined]);
  live.scene.draw();
  expect([live.count('fillRect'), live.count('fillText')]).toEqual([0, 0]);
  const section = open({ view: 'section', level: 3 });
  section.scene.draw();
  expect([section.count('fillRect'), section.count('fill') >= 27]).toEqual([0, true]);
});

test('the picture takes the whole stage at any aspect, a phone included', () => {
  const tall = layout(390, 844);
  expect([tall.x, tall.y, tall.w, tall.h].map((v) => +v.toFixed(1))).toEqual([15.6, 15.6, 358.8, 812.8]);
});

test('a drag across the stage picks the height under it, and the points export as the crate drew them', async () => {
  const live = open();
  const cut = profile(math, '126', 2, 5);
  expect([scrub(cut, 0, 800), scrub(cut, 800, 800), scrub(cut, -9, 800), live.scene.seek(400), scrub({ empty: true }, 10, 800)]).toEqual([31, 62, 31, 47, null]);
  expect(await live.scene.svg()).toBe(math.three.diagonal_svg('126', 2, 5, 2, Uint32Array.of(46), 16));
});

test('a new height keeps the last points on the stage until its own picture lands', () => {
  const loads = [];
  const was = globalThis.Image;
  globalThis.Image = class {
    constructor() {
      Object.assign(this, { naturalWidth: 512, naturalHeight: 512 });
      loads.push(this);
    }
  };
  try {
    const live = open();
    loads[0].onload();
    live.log.length = 0;
    live.scene.set(value({ height: 50 }));
    const drawn = () => live.log.filter(([key]) => key === 'drawImage').map(([, one]) => loads.indexOf(one));
    expect([loads.length, drawn()]).toEqual([2, [0]]);
    loads[1].onload();
    expect(drawn()).toEqual([0, 1]);
  } finally {
    globalThis.Image = was;
  }
});

test('tint shows only for the section, since the crate colours the points itself', () => {
  const tint = SPEC.find((one) => one.key === 'tint');
  expect([tint.when({ view: 'points' }), tint.when({ view: 'section' })]).toEqual([false, true]);
});

test('an export is named by the design, the level and the view it shows', () => {
  expect([name(value({})), name(value({ view: 'section', level: 3 }))]).toEqual(['cuts-126-l5-points', 'cuts-126-l3-section']);
});
