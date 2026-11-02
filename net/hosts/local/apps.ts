import { existsSync, readdirSync, readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import type { App } from "../../kernel/index.ts";
import type { Played } from "../../sdk/apps.ts";
import { HOME, REPO } from "../web/scripts/home.ts";

/* TYPES */

export type Site = { title: string; roots: string[]; cli?: string };

export type Found = { id: string; dir: string; kind: "scene" | "system" };

const CRATES = ["core", "font", "life", "math", "num", "tile"];

const PLAYED = ["boot", "moves", "step", "deal"];

/* HOME */

export { HOME, REPO };

export const site = (home = HOME): Site => JSON.parse(readFileSync(join(home, "site.json"), "utf8"));

export const user = (one: Site) => one.title.toLowerCase().replace(/[^a-z0-9]/g, "");

export const cli = (one: Site) => one.cli ?? "mrlynet";

/* PREPARE */

let ready: Promise<Record<string, unknown>> | null = null;

export const prepare = (home = HOME) =>
  (ready ??= (async () => {
    const one = site(home);
    Bun.plugin({
      name: "local",
      setup: (build) => {
        build.module("site:routes", () => ({
          contents: `export const site = ${JSON.stringify(one)};\nexport const rows = [];`,
          loader: "js",
        }));
      },
    });
    const doors: Record<string, unknown> = {};
    for (const unit of CRATES) {
      const wasm = join(REPO, "crates", unit, "js", "pkg", `mrly_${unit}_js_bg.wasm`);
      if (!existsSync(wasm)) continue;
      const door = (await import(`@mrlyprod/${unit}`)) as { initSync: (o: { module: Buffer }) => unknown };
      door.initSync({ module: readFileSync(wasm) });
      doors[unit] = door;
    }
    return doors;
  })());

/* FIND */

export const SYSTEM = "app.ts";

const isScene = (dir: string) =>
  existsSync(join(dir, "scene.ts")) &&
  existsSync(join(dir, "widget.tsx")) &&
  !["index.tsx", "index.md"].some((page) => existsSync(join(dir, page)));

const kindOf = (dir: string): Found["kind"] | null =>
  existsSync(join(dir, SYSTEM)) ? "system" : isScene(dir) ? "scene" : null;

export function found(home = HOME): Found[] {
  const out: Found[] = [];
  for (const root of site(home).roots) {
    const dir = resolve(home, root);
    if (!existsSync(dir)) continue;
    for (const one of readdirSync(dir, { withFileTypes: true })) {
      const kind = one.isDirectory() ? kindOf(join(dir, one.name)) : null;
      if (kind) out.push({ id: one.name, dir: join(dir, one.name), kind });
    }
  }
  return out.sort((a, b) => a.id.localeCompare(b.id));
}

/* INSTALL */

type Page = {
  spec?: never;
  defaults?: Record<string, string | number>;
  units?: () => Promise<Record<string, unknown>>;
  reroll?: never;
  acts?: never;
  keys?: never;
};

async function lift(one: Found, owner: string, doors: Record<string, unknown>): Promise<App> {
  const { scene } = await import("../../sdk/apps.ts");
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
    user: owner,
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

export async function system(one: Pick<Found, "id" | "dir">): Promise<App> {
  const made = ((await import(join(one.dir, SYSTEM))) as { default?: App }).default;
  if (!made || made.id !== one.id)
    throw new Error(`apps: ${join(one.dir, SYSTEM)} must export default an app() whose id is ${one.id}`);
  return made;
}

export async function install(ids?: readonly string[], home = HOME): Promise<App[]> {
  const doors = await prepare(home);
  const all = found(home);
  const want = ids ? all.filter((one) => ids.includes(one.id)) : all;
  return Promise.all(want.map((one) => (one.kind === "system" ? system(one) : lift(one, user(site(home)), doors))));
}
