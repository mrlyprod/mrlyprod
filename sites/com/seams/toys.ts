import { existsSync, readdirSync, readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import type { App } from "@mrlyprod/os/kernel/index.ts";
import type { Played } from "./scene.ts";

type Site = { title: string; roots: string[]; wasm?: Record<string, string[]> };

type Toy = { id: string; dir: string };

type Page = {
  spec?: never;
  defaults?: Record<string, string | number>;
  units?: () => Promise<Record<string, unknown>>;
  reroll?: never;
  acts?: never;
  keys?: never;
};

const PLAYED = ["boot", "moves", "step", "deal"];

const isScene = (dir: string) =>
  existsSync(join(dir, "scene.ts")) &&
  existsSync(join(dir, "widget.tsx")) &&
  !["index.tsx", "index.md", "app.ts"].some((page) => existsSync(join(dir, page)));

let ready: Promise<Record<string, unknown>> | null = null;

const prepare = (one: Site) =>
  (ready ??= (async () => {
    Bun.plugin({
      name: "toys",
      setup: (build) => {
        build.module("site:routes", () => ({
          contents: `export const site = ${JSON.stringify(one)};\nexport const rows = [];`,
          loader: "js",
        }));
        build.onLoad({ filter: /\.wasm$/ }, ({ path }) => ({
          exports: { default: `file://${path}` },
          loader: "object",
        }));
      },
    });
    const doors: Record<string, unknown> = {};
    for (const unit of Object.values(one.wasm ?? {}).flat()) doors[unit] = await import(`@mrlyprod/${unit}`);
    return doors;
  })());

const owner = (one: Site) => one.title.toLowerCase().replace(/[^a-z0-9]/g, "");

async function lift(one: Toy, user: string, doors: Record<string, unknown>): Promise<App> {
  const { scene } = await import("./scene.ts");
  const { PAGE } = (await import(join(one.dir, "scene.ts"))) as { PAGE: Page };
  const meta = JSON.parse(readFileSync(join(one.dir, "app.json"), "utf8")) as { title: string; line?: string };
  const loaded = PAGE.units ? await PAGE.units() : null;
  await loaded?.ready;
  const units = loaded ? Object.fromEntries(Object.entries(loaded).filter(([key]) => key !== "ready")) : undefined;
  const file = join(one.dir, "engine.ts");
  const mod = existsSync(file) ? ((await import(file)) as Record<string, unknown>) : {};
  const engine = PLAYED.every((name) => typeof mod[name] === "function") ? (mod as unknown as Played) : undefined;
  return scene({
    engine,
    doors,
    id: one.id,
    user,
    title: meta.title,
    line: meta.line ?? "",
    spec: PAGE.spec,
    defaults: PAGE.defaults,
    reroll: PAGE.reroll,
    acts: PAGE.acts,
    keys: PAGE.keys,
    units,
  });
}

export default async function toys(home: string, ids?: readonly string[]): Promise<App[]> {
  const one = JSON.parse(readFileSync(join(home, "site.json"), "utf8")) as Site;
  const all = one.roots
    .map((root) => resolve(home, root))
    .filter((dir) => existsSync(dir))
    .flatMap((dir) =>
      readdirSync(dir, { withFileTypes: true })
        .filter((item) => item.isDirectory() && isScene(join(dir, item.name)))
        .map((item): Toy => ({ id: item.name, dir: join(dir, item.name) })),
    );
  const want = ids ? all.filter((toy) => ids.includes(toy.id)) : all;
  if (!want.length) return [];
  const doors = await prepare(one);
  return Promise.all(want.map((toy) => lift(toy, owner(one), doors)));
}
