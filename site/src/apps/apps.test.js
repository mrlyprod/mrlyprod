import { expect, spyOn, test } from 'bun:test';
import { existsSync, readdirSync, readFileSync } from 'node:fs';
import { join } from 'node:path';
import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { label } from '../lib/app/keys.jsx';
import { tidy } from '../lib/app/spec.js';
import * as scene from '../lib/app/page.jsx';
import { savers as order } from '../../scripts/build.ts';
import { Settings } from '../shell/parts.jsx';

const APPS = readdirSync(import.meta.dir, { withFileTypes: true })
  .filter((one) => one.isDirectory() && existsSync(join(import.meta.dir, one.name, 'app.json')))
  .map((one) => ({ id: one.name, ...JSON.parse(readFileSync(join(import.meta.dir, one.name, 'app.json'), 'utf8')) }));

const savers = order(APPS);

const made = APPS.filter((one) => existsSync(join(import.meta.dir, one.id, 'scene.js')) && !one.deep);

const at = (path) => new URL(path, import.meta.url);

test('every saver holds its scene and its widget', () => {
  expect(savers.filter((one) => !existsSync(join(import.meta.dir, one.id, 'scene.js')) || !existsSync(join(import.meta.dir, one.id, 'widget.jsx'))).map((one) => one.id)).toEqual([]);
});

test('every saver scene exports make', async () => {
  expect(savers.length).toBeGreaterThan(0);
  for (const { id } of savers) expect([id, typeof (await import(`./${id}/scene.js`)).make]).toEqual([id, 'function']);
});

test("a saver scene that draws with wasm exports units, a wasm() of its units, which the lock awaits", async () => {
  for (const { id } of savers) {
    const { units } = await import(`./${id}/scene.js`);
    if (units === undefined) continue;
    expect([id, typeof units, /^export const units = wasm\('[a-z]+'(, '[a-z]+')*\);$/m.test(await Bun.file(at(`./${id}/scene.js`)).text())]).toEqual([id, 'function', true]);
  }
});

test('settings offer random and each saver in order, the desk first and picked', () => {
  const picks = /data-saver-pick[^>]*>(.*?)<\/select>/.exec(renderToStaticMarkup(createElement(Settings, { savers })))[1];
  expect([...picks.matchAll(/value="([^"]*)"/g)].map((found) => found[1])).toEqual(['random', ...savers.map((one) => one.id)]);
  expect(/<option value="([^"]*)" selected/.exec(picks)?.[1]).toBe(savers[0].id);
});

test('the chrome and the lock name no saver', async () => {
  for (const file of ['../shell/chrome.js', '../shell/parts.jsx', '../shell/lock.js']) {
    const src = await Bun.file(at(file)).text();
    for (const { id } of savers) expect([file, id, src.includes(`'${id}'`)]).toEqual([file, id, false]);
  }
});

test("every app's value holds a seed, and an app whose scene has a spec pages it and starts from defaults it keeps as they are", async () => {
  const spy = spyOn(scene, 'page');
  for (const { id } of made) await import(`./${id}/index.jsx`);
  const calls = spy.mock.calls;
  spy.mockRestore();
  for (const { id } of made) {
    const { Widget } = await import(`./${id}/widget.jsx`);
    const { SPEC } = await import(`./${id}/scene.js`);
    const [, defaults, { spec } = {}] = calls.find(([one]) => one === Widget) ?? [];
    const value = scene.values(defaults, spec);
    expect([id, 'seed' in value, spec === SPEC]).toEqual([id, true, true]);
    if (spec) expect([id, { ...value, ...tidy(spec, value) }]).toEqual([id, value]);
  }
});

test("every app's keys map binds Space to pause and to nothing else", async () => {
  for (const { id } of made) {
    const { PAGE } = await import(`./${id}/scene.js`);
    const rows = scene.keymap(PAGE?.keys ?? []).filter((row) => [].concat(row.key).some((key) => label(key) === 'Space'));
    expect([id, rows.map((row) => row.act)]).toEqual([id, ['pause']]);
  }
});
