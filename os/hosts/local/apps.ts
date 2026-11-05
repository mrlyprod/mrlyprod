import { existsSync, readdirSync, readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import type { App } from "../../kernel/index.ts";
import type { Handlers } from "../bus.ts";

export type Site = { title: string; roots: string[]; cli?: string; seams?: { apps?: string; wishes?: string } };

export type Found = { id: string; dir: string };

type Seam = { default: (home: string, ids?: readonly string[]) => Promise<App[]> };

export const HOME = resolve(process.env.MRLY_ROOT ?? process.cwd());

export const SYSTEM = "app.ts";

export function site(home = HOME): Site {
  const file = join(home, "site.json");
  if (!existsSync(file)) throw new Error(`no site.json at ${home}; run from an instance, or set MRLY_ROOT to one`);
  return JSON.parse(readFileSync(file, "utf8")) as Site;
}

export const cli = (one: Site) => one.cli ?? one.title.toLowerCase().replace(/[^a-z0-9]/g, "");

export async function handlers(home = HOME): Promise<Handlers> {
  const file = site(home).seams?.wishes;
  return file ? ((await import(resolve(home, file))) as { default: Handlers }).default : {};
}

export function found(home = HOME): Found[] {
  const out = new Map<string, Found>();
  for (const root of site(home).roots) {
    const dir = resolve(home, root);
    if (!existsSync(dir)) continue;
    for (const one of readdirSync(dir, { withFileTypes: true }))
      if (one.isDirectory() && existsSync(join(dir, one.name, SYSTEM)))
        out.set(one.name, { id: one.name, dir: join(dir, one.name) });
  }
  return [...out.values()].sort((a, b) => a.id.localeCompare(b.id));
}

export async function system(one: Found): Promise<App> {
  const made = ((await import(join(one.dir, SYSTEM))) as { default?: App }).default;
  if (!made || made.id !== one.id)
    throw new Error(`apps: ${join(one.dir, SYSTEM)} must export default an app() whose id is ${one.id}`);
  return made;
}

const seamed = async (home: string, ids?: readonly string[]) => {
  const file = site(home).seams?.apps;
  return file ? ((await import(resolve(home, file))) as Seam).default(home, ids) : [];
};

export async function install(ids?: readonly string[], home = HOME): Promise<App[]> {
  const all = found(home);
  const want = ids ? all.filter((one) => ids.includes(one.id)) : all;
  const made = [...(await Promise.all(want.map(system))), ...(await seamed(home, ids))];
  return [...new Map(made.map((one) => [one.id, one])).values()].sort((a, b) => a.id.localeCompare(b.id));
}
