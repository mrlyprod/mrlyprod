import { expect, test } from 'bun:test';
import * as math from '@mrlyprod/mrly/math';
import { defaults } from '../../lib/app/spec.js';
import { RATIO, bounds, cropped, fit, mesh, reroll, studio, wrap } from './engine.js';
import { SPEC } from './scene.js';

const CARPET = { ...defaults(SPEC), seed: 1, base: 2, code: '23' };

const read = (value) => studio(math)({ ...CARPET, ...value });

test('the carpet sheet multiplies its fills by the copies while its exposed edges fall short', () => {
  const { facts } = read({});
  expect(facts).toMatchObject({ code: '23', name: 'carpet', title: 'bang dim 3, code 23', level: 2, top: 2, side: 9, copies: 25, tile: [35, 18], sheet: [143, 99], triangles: 12150, fills: 7650, voids: 4500, exposed: 3042, alone: 162, euler: 1 });
  expect(facts.fills).toBe(25 * read({ x: 1, y: 1 }).facts.fills);
  expect(facts.exposed).toBeLessThan(25 * facts.alone);
});

test('the crop trims the overhang and fills with it, only on a rectangle of two or more each way', () => {
  expect(read({ overhang: 0 }).facts).toMatchObject({ triangles: 10287, fills: 6345, voids: 3942, exposed: 2565 });
  expect(read({ overhang: 0, x: 1 }).facts.fills).toBe(5 * read({ x: 1, y: 1 }).facts.fills);
  expect(cropped({ ...CARPET, overhang: 0, sheet: 'disc' })).toBe(false);
});

test('each view gives its own hexagon, and invert swaps the fills and the voids', () => {
  const one = (value) => read({ x: 1, y: 1, ...value }).facts;
  expect(one({})).toMatchObject({ fills: 306, voids: 180, pieces: 1, holes: 7 });
  expect(one({ view: 'pro' })).toMatchObject({ fills: 384, voids: 102 });
  expect(one({ view: 'iso' })).toMatchObject({ fills: 486, voids: 0, pieces: 1, holes: 0 });
  expect([one({ invert: 1 }).fills, one({ invert: 1 }).voids]).toEqual([180, 306]);
});

test('a disc of rings is the other sheet, one, seven and nineteen copies', () => {
  expect([0, 1, 2].map((radius) => read({ sheet: 'disc', radius }).facts.copies)).toEqual([1, 7, 19]);
  expect(read({ sheet: 'disc', radius: 1 }).facts.fills).toBe(7 * 306);
});

test('the level stops at the cap the side and the copies leave', () => {
  expect(read({ number: 9, level: 3 }).facts).toMatchObject({ level: 1, top: 1, side: 9 });
  expect(read({ x: 12, y: 12 }).facts).toMatchObject({ level: 1, top: 1, side: 3 });
  expect(read({ x: 1, y: 1, level: 3 }).facts).toMatchObject({ level: 3, top: 3, side: 27 });
});

test('a studio rebuilds only the layers a change reaches', () => {
  const study = studio(math);
  const a = study(CARPET);
  const b = study({ ...CARPET, x: 6 });
  expect([b.one === a.one, b.sheet === a.sheet, b.facts.copies]).toEqual([true, false, 30]);
  const c = study({ ...CARPET, x: 6, view: 'iso' });
  expect([c.cube === a.cube, c.one === b.one]).toEqual([true, false]);
  expect(study({ ...CARPET, x: 6, view: 'iso', tint: 'red', turn: 1 }).facts).toBe(c.facts);
});

test('a blank code and a reroll both roll the code from the seed', () => {
  const rolled = (seed) => read({ code: '', seed }).facts.code;
  expect(rolled(5)).toBe(reroll(math, 2, 5));
  expect(new Set([1, 2, 3, 4, 5, 6].map(rolled)).size).toBeGreaterThan(1);
  expect(rolled(5)).not.toBe('0');
});

test('a framed sheet stands the right way up, two across four down tall and four across two down wide', () => {
  const aspect = (x, y, view) => {
    const [x0, y0, x1, y1] = mesh(math, read({ x, y, level: 1, view }).drawn, false).box;
    return (x1 - x0) / (y1 - y0);
  };
  expect([aspect(2, 4, 'cut') < 1, aspect(4, 2, 'cut') > 1, aspect(2, 4, 'iso') < 1, aspect(4, 2, 'iso') > 1]).toEqual([true, true, true, true]);
});

test('the mesh groups the triangles by type in paint order at the true aspect, turned on request', () => {
  const one = math.six.cut_design('23', 3, 1, 2);
  const flat = mesh(math, one, false);
  expect(flat.groups.map((group) => [group.name, group.points.length / 6])).toEqual([['VOID', 12], ['FILL', 42]]);
  expect(flat.box.map((v) => Math.round(v * 1000) / 1000)).toEqual([0, 0, 12, Math.round(12 * RATIO * 1000) / 1000]);
  expect(flat.across).toBe(true);
  const turned90 = mesh(math, one, true);
  expect(turned90.box.map((v) => Math.round(v * 1000) / 1000)).toEqual([-Math.round(12 * RATIO * 1000) / 1000, 0, 0, 12]);
  const iso = mesh(math, math.six.iso_design('23', 3, 1, 2), false);
  expect(iso.groups.map((group) => group.name)).toEqual(['UP', 'LEFT', 'RIGHT']);
  expect(iso.groups.reduce((sum, group) => sum + group.points.length / 6, 0)).toBe(54);
});

test('fit centres a box inside the canvas with its padding', () => {
  expect(fit([0, 0, 12, 6], 200, 100, 10)).toEqual({ k: 80 / 6, ox: 100 - 6 * (80 / 6), oy: 50 - 3 * (80 / 6) });
  expect(bounds([])).toEqual([0, 0, 1, 1]);
});

test('wrap gives an svg its true aspect and turns it a quarter when asked', () => {
  const text = '<svg width="36" height="24" viewBox="0 0 36 24" xmlns="http://www.w3.org/2000/svg">\n<polygon points="0,0 2,4 4,0" fill="#008cff" stroke="none"/>\n</svg>';
  const flat = wrap(text, true, false);
  expect(flat).toBe(`<svg xmlns="http://www.w3.org/2000/svg" width="36" height="20.78" viewBox="0 0 36 20.78"><g transform="scale(1 0.866)"><polygon points="0,0 2,4 4,0" fill="#008cff" stroke="none"/></g></svg>`);
  const spun = wrap(text, true, true);
  expect(spun.startsWith('<svg xmlns="http://www.w3.org/2000/svg" width="20.78" height="36" viewBox="0 0 20.78 36"><g transform="translate(20.785 0) rotate(90) scale(1 0.866)">')).toBe(true);
  expect(wrap('<p>no</p>', true, false)).toBe('<p>no</p>');
});
