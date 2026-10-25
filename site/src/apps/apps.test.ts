import { afterAll, expect, spyOn, test } from "bun:test";
import { existsSync, mkdtempSync, readdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import type { ComponentType } from "react";
import { createElement } from "react";
import { renderToStaticMarkup } from "react-dom/server";
import type { App, Host } from "../../scripts/build.ts";
import { entry, NAMES, savers as order, SHELL } from "../../scripts/build.ts";
import { Frame } from "../lib/app/frame.tsx";
import type { Ready, Thunk } from "../lib/app/hooks.ts";
import { label } from "../lib/app/keys.tsx";
import type { Page, WidgetProps } from "../lib/app/page.tsx";
import * as scene from "../lib/app/page.tsx";
import type { Make } from "../lib/app/run.ts";
import type { Patch, Spec } from "../lib/app/spec.ts";
import { tidy } from "../lib/app/spec.ts";
import type { Rng } from "../lib/rand.ts";
import { rng } from "../lib/rand.ts";

type Row = Omit<App, "id" | "dir" | "page" | "icon">;

type Saver = { id: string; title: string; pick: (rand: Rng) => Patch };

type Scene = { make: Make; SAVERS?: readonly Saver[]; PAGE?: Page; SPEC?: Spec; units?: Thunk<Ready> };

type Staged = Pick<WidgetProps, "value" | "onChange"> & {
  units?: Record<string, unknown>;
  unit?: unknown;
  three?: object;
};

type Shown = { Widget: ComponentType<Staged> };

const APPS = readdirSync(import.meta.dir, { withFileTypes: true })
  .filter((one) => one.isDirectory() && existsSync(join(import.meta.dir, one.name, "app.json")))
  .map((one) => ({
    id: one.name,
    ...(JSON.parse(readFileSync(join(import.meta.dir, one.name, "app.json"), "utf8")) as Row),
  }));

const has = (id: string, name: "scene" | "widget") => existsSync(join(import.meta.dir, id, NAMES[name]));

const HOSTS: Host[] = [
  ...APPS.filter((one) => (["scene", "widget"] as const).every((name) => has(one.id, name))).map((one) => ({
    id: one.id,
    title: one.title,
    saver: one.saver,
    file: join(import.meta.dir, one.id, NAMES.scene),
  })),
  SHELL,
];

const savers = () => order(HOSTS, (file) => require(file)).list;

const held = () => [...new Set(savers().map((one) => one.app))].map((app) => HOSTS.find((one) => one.id === app)!);

const each = async () => {
  const out: { id: string; mod: Scene; saver: Saver }[] = [];
  for (const host of held()) {
    const mod: Scene = await import(host.file);
    for (const saver of mod.SAVERS ?? [{ id: host.id, title: host.title, pick: () => ({}) }])
      out.push({ id: `${host.id}/${saver.id}`, mod, saver });
  }
  return out;
};

const made = APPS.filter((one) => has(one.id, "scene") && !one.deep);

const at = (path: string) => new URL(path, import.meta.url);

const TEMP = mkdtempSync(join(tmpdir(), "mrly-apps-"));

afterAll(() => rmSync(TEMP, { recursive: true, force: true }));

const load = (id: string): Promise<unknown> => {
  const file = join(TEMP, `${id}.js`);
  writeFileSync(file, entry(join(import.meta.dir, id)));
  return import(file);
};

test("every SAVERS entry is valid", () => {
  expect(() => order(HOSTS, (file) => require(file))).not.toThrow();
});

test("every saver app's scene exports make", async () => {
  expect(held().length).toBeGreaterThan(1);
  for (const host of held()) {
    const { make }: Scene = await import(host.file);
    expect([host.id, typeof make]).toEqual([host.id, "function"]);
  }
});

test("a saver scene that draws with wasm exports units, a wasm() of its units, which the lock awaits", async () => {
  for (const host of held()) {
    const { units }: Scene = await import(host.file);
    if (units === undefined) continue;
    expect([
      host.id,
      typeof units,
      /^export const units = wasm\("[a-z]+"(, "[a-z]+")*\);$/m.test(await Bun.file(host.file).text()),
    ]).toEqual([host.id, "function", true]);
  }
});

test("a pick gives the same value for the same seed and never calls Math.random", async () => {
  const list = await each();
  const spy = spyOn(Math, "random");
  const odd = list
    .filter(({ saver }) => [1, 2, 3].some((seed) => !Bun.deepEquals(saver.pick(rng(seed)), saver.pick(rng(seed)))))
    .map((one) => one.id);
  const calls = spy.mock.calls.length;
  spy.mockRestore();
  expect([odd, calls]).toEqual([[], 0]);
});

test("tidy keeps every spec key a pick sets", async () => {
  for (const { id, mod, saver } of await each()) {
    if (!mod.SPEC) continue;
    for (const seed of [1, 2, 3]) {
      const value: Patch = { seed, ...saver.pick(rng(seed)) };
      const kept = tidy(mod.SPEC, value);
      const set = Object.keys(value).filter((key) => key in kept);
      expect<(string | number | Patch)[]>([id, seed, Object.fromEntries(set.map((key) => [key, kept[key]]))]).toEqual([
        id,
        seed,
        Object.fromEntries(set.map((key) => [key, value[key]])),
      ]);
    }
  }
});

test("the chrome and the lock name no saver", async () => {
  const names = [...new Set(savers().flatMap((one) => [one.id, ...(one.app === SHELL.id ? [] : [one.app])]))];
  for (const file of [
    "../shell/chrome.ts",
    "../shell/desk.ts",
    "../shell/prefs.ts",
    "../shell/parts.tsx",
    "../shell/lock.ts",
    "../shell/savers.ts",
    "../shell/idle.ts",
    "../pages/settings/screensaver.tsx",
  ]) {
    const src = await Bun.file(at(file)).text();
    for (const name of names)
      expect([file, name, new RegExp(`["']${name}["']`).test(src)]).toEqual([file, name, false]);
  }
});

test("every app's value holds a seed, and an app whose scene has a spec pages it and starts from defaults it keeps as they are", async () => {
  const spy = spyOn(scene, "page");
  for (const { id } of made) await load(id);
  const calls = spy.mock.calls;
  spy.mockRestore();
  for (const { id } of made) {
    const { Widget }: Shown = await import(`./${id}/${NAMES.widget}`);
    const { SPEC }: Scene = await import(`./${id}/${NAMES.scene}`);
    const [, defaults = {}, { spec } = {}] = calls.find(([one]) => one === Widget) ?? [];
    const value = scene.values(defaults, spec);
    expect([id, "seed" in value, spec === SPEC]).toEqual([id, true, true]);
    if (spec) expect([id, { ...value, ...tidy(spec, value) }]).toEqual([id, value]);
  }
});

test("every app's keys map binds Space to pause and to nothing else", async () => {
  for (const { id } of made) {
    const { PAGE }: Scene = await import(`./${id}/${NAMES.scene}`);
    const rows = scene.keymap(PAGE?.keys ?? []).filter((row) => [row.key].flat().some((key) => label(key) === "Space"));
    expect([id, rows.map((row) => row.act)]).toEqual([id, ["pause"]]);
  }
});

test("every widget handed its wasm units by the page mounts no stage before them and one after", async () => {
  const spy = spyOn(scene, "page");
  for (const { id } of made) await load(id);
  const calls = spy.mock.calls;
  spy.mockRestore();
  const wrong: string[] = [];
  const frame = (Widget: ComponentType<Staged>, props: Omit<Staged, "onChange">) =>
    renderToStaticMarkup(
      createElement(Frame, { label: "Stage" }, createElement(Widget, { onChange: () => {}, ...props })),
    );
  for (const { id } of made) {
    const [source, widget] = await Promise.all([
      Bun.file(at(`./${id}/${NAMES.scene}`)).text(),
      Bun.file(at(`./${id}/${NAMES.widget}`)).text(),
    ]);
    const names = [...`${source}\n${widget}`.matchAll(/\bwasm\(([^)]*)\)/g)].flatMap((call) =>
      [...call[1].matchAll(/["']([a-z]+)["']/g)].map((hit) => hit[1]),
    );
    const one = /useUnit\(["']([a-z]+)["']/.exec(widget)?.[1];
    const takes =
      /export function Widget\(\{([^}]*)\}/
        .exec(widget)?.[1]
        .split(",")
        .map((part) => part.trim().split(":")[0]) ?? [];
    if ((!names.length && !one) || !takes.some((prop) => prop === "units" || prop === "unit")) continue;
    const { Widget }: Shown = await import(`./${id}/${NAMES.widget}`);
    const { SPEC }: Scene = await import(`./${id}/${NAMES.scene}`);
    const [, defaults = {}, { spec } = {}] = calls.find(([w]) => w === Widget) ?? [];
    const value = scene.values(defaults, spec ?? SPEC);
    const units = Object.fromEntries(
      await Promise.all(
        [...new Set([...names, ...(one ? [one] : [])])].map(
          async (name): Promise<[string, unknown]> => [name, await import(`@mrlyprod/mrly/${name}`)],
        ),
      ),
    );
    if (
      frame(Widget, { value }).includes("<canvas") ||
      !frame(Widget, { value, units, unit: one && units[one], three: {} }).includes("<canvas")
    )
      wrong.push(id);
  }
  expect(wrong).toEqual([]);
});
