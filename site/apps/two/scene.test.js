import { expect, test } from 'bun:test';
import * as math from '@mrlyprod/mrly/math';
import { asked, defaults } from '../../lib/knobs.js';
import { rng } from '../../lib/scene.js';
import { design, lay } from './engine.js';
import { PAGE, SPEC, make, name, plan, second } from './scene.js';

const pen = (log) =>
  new Proxy(
    {},
    {
      get: (_, key) => (...args) => log.push([key, ...args]),
      set: (_, key, v) => {
        log.push([key, v]);
        return true;
      },
    },
  );

const pages = [];

globalThis.OffscreenCanvas = class {
  constructor(w, h) {
    this.width = w;
    this.height = h;
    this.log = [];
    pages.push(this);
  }
  getContext() {
    return pen(this.log);
  }
  convertToBlob(options) {
    return Promise.resolve({ type: options.type, page: this });
  }
};

const ACCENT = '#008cff';

const start = (over) => ({ ...defaults(SPEC), seed: 1, base: 3, code: '495', ...over });

const open = (value, w = 800, h = 600) => {
  const log = [];
  const view = { rand: rng(1), look: () => ({ paper: '#000000', accent: ACCENT }), still: false, w, h, dpr: 2, t: 0 };
  const scene = make({ getContext: () => pen(log) }, view, { ...start(value), math });
  const quiet = log.length;
  scene.draw();
  const made = log.slice();
  const at = () => {
    log.length = 0;
    scene.draw();
    return log;
  };
  return { view, scene, log, made, quiet, at };
};

const fills = (log, color) => {
  const out = [];
  let ink = null;
  for (const [name, ...args] of log) {
    if (name === 'fillStyle') ink = args[0];
    if (name === 'rect' && ink === color) out.push(args);
  }
  return out;
};

const cells = (list) => list.reduce((sum, [, , w]) => sum + Math.round(w / 12), 0);

test('make draws nothing, and the first draw lays the uncropped carpet sheet in the accent over faint voids', () => {
  const live = open({});
  expect(live.quiet).toBe(0);
  expect(live.made.slice(1, 3)).toEqual([['fillStyle', '#000000'], ['fillRect', 0, 0, 800, 600]]);
  expect(cells(fills(live.made, ACCENT))).toBe(25 * math.two.fills(math.two.create('495', 3, 2, 0, 3)));
  expect(cells(fills(live.made, 'rgba(0, 140, 255, 0.16)')) + cells(fills(live.made, ACCENT))).toBe(45 * 45);
  expect(live.at()).toEqual([]);
});

test('the scene is still: a draw repaints only after a resize, a theme change or a set, and a set paints at once', () => {
  const live = open({});
  expect(live.scene.still).toBe(true);
  for (const poke of [() => live.scene.size(), () => live.scene.theme(), () => live.scene.set(start({ x: 3 }))]) {
    live.log.length = 0;
    poke();
    expect(fills(live.log, ACCENT).length).toBeGreaterThan(0);
    expect(live.at()).toEqual([]);
  }
});

test('the sheet sits centred inside nine tenths of the short side on whole pixels', () => {
  const live = open({});
  const box = lay(45, 45, 800, 600, 0.9);
  const drawn = [...fills(live.made, ACCENT), ...fills(live.made, 'rgba(0, 140, 255, 0.16)')];
  expect(drawn.every((row) => row.every(Number.isInteger))).toBe(true);
  expect(Math.min(...drawn.map(([x]) => x))).toBe(box.x);
  expect(Math.min(...drawn.map(([, y]) => y))).toBe(box.y);
  expect(Math.max(...drawn.map(([x, , w]) => x + w))).toBe(box.x + 45 * box.px);
  expect(Math.max(...drawn.map(([, y, , h]) => y + h))).toBe(box.y + 45 * box.px);
});

test('seams draw where the copies meet in a second ink that is not the accent', () => {
  const plain = open({});
  const seamed = open({ seams: 1 });
  const ink = second(null, '', ACCENT);
  expect(ink).not.toBe(ACCENT);
  expect(fills(plain.made, ink)).toEqual([]);
  const drawn = fills(seamed.made, ink);
  expect(drawn.length).toBeGreaterThan(0);
  expect(drawn.every(([, , w, h]) => Math.min(w, h) === 4)).toBe(true);
  expect(second(null, 'orange', ACCENT)).not.toBe(second(null, '', ACCENT));
});

test('a set rebuilds only from the first stage it changed and keeps the rest', () => {
  const stages = {};
  const first = plan(math, start({}), rng(1), stages);
  const tint = plan(math, start({ tint: 'pink', invert: 1 }), rng(1), stages);
  expect(tint.tile).toBe(first.tile);
  expect(tint.block).toBe(first.block);
  expect(tint.look).not.toBe(first.look);
  const wide = plan(math, start({ x: 6 }), rng(1), stages);
  expect(wide.tile).toBe(first.tile);
  expect(wide.block).not.toBe(first.block);
  const deep = plan(math, start({ x: 6, level: 1 }), rng(1), stages);
  expect(deep.tile).not.toBe(first.tile);
});

test('a code rolled from a blank stays put through every change but the seed and the base', () => {
  const stages = {};
  const rand = rng(9);
  const code = plan(math, start({ code: '' }), rand, stages).value.code;
  expect(plan(math, start({ code: '', x: 2, seams: 1 }), rand, stages).value.code).toBe(code);
  expect(plan(math, start({ code: '', seed: 2 }), rng(2), stages).value.code).toBe(design(math, 3, '', rng(2)));
});

test('the level is kept as asked in the url and drawn at the cap of the sheet, so lowering the copies brings it back', () => {
  expect(asked(SPEC, start({ level: 5, x: 12, y: 12 }), { math }).level).toBe(5);
  expect(plan(math, start({ level: 5, x: 12, y: 12 }), rng(1)).value.level).toBe(3);
  const small = plan(math, start({ level: 5, x: 1, y: 1 }), rng(1));
  expect([small.value.level, small.facts().side]).toEqual([5, 243]);
});

test('the svg is one rect per run of the lit cells in the accent with no ground', () => {
  const live = open({});
  const text = live.scene.svg();
  expect(text.startsWith('<svg xmlns="http://www.w3.org/2000/svg" width="450" height="450" viewBox="0 0 450 450"')).toBe(true);
  expect(text.match(/<rect /g)).toHaveLength(fills(live.made, ACCENT).length);
  expect(text).toContain(`fill="${ACCENT}"`);
  expect(text).not.toContain('fill="#000000"');
});

test('a shot draws the sheet edge to edge at the size over the paper and hands back a blob of the kind', async () => {
  const live = open({ x: 6, y: 3 });
  const blob = await live.scene.shot(1024);
  const page = pages.at(-1);
  expect([page.width, page.height, blob.type]).toEqual([1024, 512, 'image/png']);
  expect(page.log.slice(0, 2)).toEqual([['fillStyle', '#000000'], ['fillRect', 0, 0, 1024, 512]]);
  const drawn = [...fills(page.log, ACCENT), ...fills(page.log, 'rgba(0, 140, 255, 0.16)')];
  expect(Math.max(...drawn.map(([x, , w]) => x + w))).toBe(1024);
  expect(Math.max(...drawn.map(([, y, , h]) => y + h))).toBe(512);
  expect((await live.scene.shot(512, 'webp')).type).toBe('image/webp');
});

test('an export is named by the design and the sheet it shows', () => {
  expect([name(start({ x: 3, y: 9 })), name(start({ code: '' }))]).toEqual(['two-495-3x9', 'two-1-5x5']);
  expect(PAGE.name).toBe(name);
});

test('the primary flips the seams and names what it will do', () => {
  const rows = (value) => PAGE.keys.filter((row) => row.button && row.when(value)).map((row) => row.label);
  expect(rows({ seams: 0 })).toEqual(['Seams']);
  expect(rows({ seams: 1 })).toEqual(['Hide seams']);
});

test('under invert the counts follow the lit voids the picture shows', () => {
  const plain = plan(math, start({}), rng(1));
  const inverse = plan(math, start({ invert: 1 }), rng(1));
  const facts = inverse.facts();
  expect([facts.fills, facts.voids]).toEqual([plain.facts().voids, plain.facts().fills]);
  expect(facts.one).toBe(Number(math.two.perimeter(math.cell.models.invert(plain.tile))));
});
