import { afterAll, expect, spyOn, test } from 'bun:test';
import { existsSync, mkdtempSync, readdirSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { label } from '../lib/app/keys.jsx';
import { tidy } from '../lib/app/spec.js';
import { rng } from '../lib/rand.js';
import * as scene from '../lib/app/page.jsx';
import { SHELL, entry, savers as order } from '../../scripts/build.ts';
import { Frame } from '../lib/app/frame.jsx';

const APPS = readdirSync(import.meta.dir, { withFileTypes: true })
  .filter((one) => one.isDirectory() && existsSync(join(import.meta.dir, one.name, 'app.json')))
  .map((one) => ({ id: one.name, ...JSON.parse(readFileSync(join(import.meta.dir, one.name, 'app.json'), 'utf8')) }));

const HOSTS = [...APPS.filter((one) => ['scene.js', 'widget.jsx'].every((name) => existsSync(join(import.meta.dir, one.id, name)))).map((one) => ({ id: one.id, title: one.title, saver: one.saver, file: join(import.meta.dir, one.id, 'scene.js') })), SHELL];

const savers = () => order(HOSTS, (file) => require(file)).list;

const held = () => [...new Set(savers().map((one) => one.app))].map((app) => HOSTS.find((one) => one.id === app));

const each = async () => {
  const out = [];
  for (const host of held()) {
    const mod = await import(host.file);
    for (const saver of mod.SAVERS ?? [{ id: host.id, title: host.title, pick: () => ({}) }]) out.push({ id: `${host.id}/${saver.id}`, mod, saver });
  }
  return out;
};

const made = APPS.filter((one) => existsSync(join(import.meta.dir, one.id, 'scene.js')) && !one.deep);

const at = (path) => new URL(path, import.meta.url);

const TEMP = mkdtempSync(join(tmpdir(), 'mrly-apps-'));

afterAll(() => rmSync(TEMP, { recursive: true, force: true }));

const load = (id) => {
  const file = join(TEMP, `${id}.js`);
  writeFileSync(file, entry(join(import.meta.dir, id)));
  return import(file);
};

test('every SAVERS entry is valid', () => {
  expect(() => order(HOSTS, (file) => require(file))).not.toThrow();
});

test("every saver app's scene exports make", async () => {
  expect(held().length).toBeGreaterThan(1);
  for (const host of held()) expect([host.id, typeof (await import(host.file)).make]).toEqual([host.id, 'function']);
});

test("a saver scene that draws with wasm exports units, a wasm() of its units, which the lock awaits", async () => {
  for (const host of held()) {
    const { units } = await import(host.file);
    if (units === undefined) continue;
    expect([host.id, typeof units, /^export const units = wasm\('[a-z]+'(, '[a-z]+')*\);$/m.test(await Bun.file(host.file).text())]).toEqual([host.id, 'function', true]);
  }
});

test('a pick gives the same value for the same seed and never calls Math.random', async () => {
  const list = await each();
  const spy = spyOn(Math, 'random');
  const odd = list.filter(({ saver }) => [1, 2, 3].some((seed) => !Bun.deepEquals(saver.pick(rng(seed)), saver.pick(rng(seed))))).map((one) => one.id);
  const calls = spy.mock.calls.length;
  spy.mockRestore();
  expect([odd, calls]).toEqual([[], 0]);
});

test('tidy keeps every spec key a pick sets', async () => {
  for (const { id, mod, saver } of await each()) {
    if (!mod.SPEC) continue;
    for (const seed of [1, 2, 3]) {
      const value = { seed, ...saver.pick(rng(seed)) };
      const kept = tidy(mod.SPEC, value);
      const set = Object.keys(value).filter((key) => key in kept);
      expect([id, seed, Object.fromEntries(set.map((key) => [key, kept[key]]))]).toEqual([id, seed, Object.fromEntries(set.map((key) => [key, value[key]]))]);
    }
  }
});

test('the chrome and the lock name no saver', async () => {
  const names = [...new Set(savers().flatMap((one) => [one.id, ...(one.app === SHELL.id ? [] : [one.app])]))];
  for (const file of ['../shell/chrome.js', '../shell/desk.js', '../shell/prefs.js', '../shell/parts.jsx', '../shell/lock.js', '../shell/savers.js', '../shell/idle.js', '../pages/settings/screensaver.jsx']) {
    const src = await Bun.file(at(file)).text();
    for (const name of names) expect([file, name, src.includes(`'${name}'`)]).toEqual([file, name, false]);
  }
});

test("every app's value holds a seed, and an app whose scene has a spec pages it and starts from defaults it keeps as they are", async () => {
  const spy = spyOn(scene, 'page');
  for (const { id } of made) await load(id);
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

test('every widget handed its wasm units by the page mounts no stage before them and one after', async () => {
  const spy = spyOn(scene, 'page');
  for (const { id } of made) await load(id);
  const calls = spy.mock.calls;
  spy.mockRestore();
  const wrong = [];
  const frame = (Widget, props) => renderToStaticMarkup(createElement(Frame, { label: 'Stage' }, createElement(Widget, { onChange: () => {}, ...props })));
  for (const { id } of made) {
    const [source, widget] = await Promise.all([Bun.file(at(`./${id}/scene.js`)).text(), Bun.file(at(`./${id}/widget.jsx`)).text()]);
    const names = [...`${source}\n${widget}`.matchAll(/\bwasm\(([^)]*)\)/g)].flatMap((call) => [...call[1].matchAll(/'([a-z]+)'/g)].map((hit) => hit[1]));
    const one = /useUnit\('([a-z]+)'/.exec(widget)?.[1];
    const takes = /export function Widget\(\{([^}]*)\}/.exec(widget)?.[1].split(',').map((part) => part.trim().split(':')[0]) ?? [];
    if ((!names.length && !one) || !takes.some((prop) => prop === 'units' || prop === 'unit')) continue;
    const { Widget } = await import(`./${id}/widget.jsx`);
    const { SPEC } = await import(`./${id}/scene.js`);
    const [, defaults, { spec } = {}] = calls.find(([w]) => w === Widget) ?? [];
    const value = scene.values(defaults, spec ?? SPEC);
    const units = Object.fromEntries(await Promise.all([...new Set([...names, ...(one ? [one] : [])])].map(async (name) => [name, await import(`@mrlyprod/mrly/${name}`)])));
    if (frame(Widget, { value }).includes('<canvas') || !frame(Widget, { value, units, unit: one && units[one], three: {} }).includes('<canvas')) wrong.push(id);
  }
  expect(wrong).toEqual([]);
});
