import { expect, spyOn, test } from 'bun:test';
import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { label } from '../lib/keys.js';
import { tidy } from '../lib/knobs.js';
import * as scene from '../lib/scene.jsx';
import { Settings } from '../ui/chrome.jsx';
import APPS from './apps.json';

const savers = APPS.filter((one) => one.kind === 'saver');

const at = (path) => new URL(path, import.meta.url);

test('the app list holds unique root words, each with a title', () => {
  expect(new Set(APPS.map((one) => one.id)).size).toBe(APPS.length);
  expect(APPS.filter((one) => !/^[a-z][a-z0-9]*$/.test(one.id) || !one.title)).toEqual([]);
});

test('every saver scene exports make', async () => {
  expect(savers.length).toBeGreaterThan(0);
  for (const { id } of savers) expect([id, typeof (await import(`./${id}/scene.js`)).make]).toEqual([id, 'function']);
});

test('settings offer none, random and each saver row in order', () => {
  const picks = /data-saver-pick[^>]*>(.*?)<\/select>/.exec(renderToStaticMarkup(createElement(Settings, { savers })))[1];
  expect([...picks.matchAll(/value="([^"]*)"/g)].map((found) => found[1])).toEqual(['', 'random', ...savers.map((one) => one.id)]);
});

test('the chrome and the lock name no saver', async () => {
  for (const file of ['../ui/chrome.js', '../ui/chrome.jsx', '../lib/lock.js']) {
    const src = await Bun.file(at(file)).text();
    for (const { id } of savers) expect([file, id, src.includes(`'${id}'`)]).toEqual([file, id, false]);
  }
});

test("every app's value holds twelve keys or fewer, and an app whose scene has a spec pages it and starts from defaults it keeps as they are", async () => {
  const spy = spyOn(scene, 'page');
  const made = APPS.filter((one) => one.kind !== 'tool');
  for (const { id } of made) await import(`./${id}/index.jsx`);
  const calls = spy.mock.calls;
  spy.mockRestore();
  for (const { id } of made) {
    const { Widget } = await import(`./${id}/widget.jsx`);
    const { SPEC } = await import(`./${id}/scene.js`);
    const [, defaults, { spec } = {}] = calls.find(([one]) => one === Widget) ?? [];
    const value = scene.values(defaults, spec);
    expect([id, Object.keys(value).length <= 12, 'seed' in value, spec === SPEC]).toEqual([id, true, true, true]);
    if (spec) expect([id, { ...value, ...tidy(spec, value) }]).toEqual([id, value]);
  }
});

test("every app's keys map binds Space to pause and to nothing else", async () => {
  for (const { id } of APPS.filter((one) => one.kind !== 'tool')) {
    const { PAGE } = await import(`./${id}/scene.js`);
    const rows = scene.keymap(PAGE?.keys ?? []).filter((row) => [].concat(row.key).some((key) => label(key) === 'Space'));
    expect([id, rows.map((row) => row.act)]).toEqual([id, ['pause']]);
  }
});
