import { scenes } from "apps:scenes";
import { site } from "site:routes";
import type { Saver as Listed } from "../../scripts/build.ts";
import type { Ready, Thunk } from "../lib/app/hooks.ts";
import type { Page } from "../lib/app/page.tsx";
import type { Make } from "../lib/app/run.ts";
import type { Patch, Spec } from "../lib/app/spec.ts";
import type { Rng } from "../lib/rand.ts";
import { rng } from "../lib/rand.ts";
import { offs } from "./prefs.ts";

/* TYPES */

export type Saver = { id: string; title: string; pick: (rand: Rng) => Patch; still?: number };

export type Scene = {
  make: Make;
  SAVERS?: readonly Saver[];
  PAGE?: Page;
  SPEC?: Spec;
  units?: Thunk<Ready>;
};

export type Loaded = { entry: Listed; scene: Scene; saver: Saver; still: number };

const LONGEST = 10000;

export const STILL = 1000;

/* REGISTRY */

export const all = (): Listed[] => site.savers ?? [];

export function find(id: string): Listed | null {
  if (!id) return null;
  const list = all();
  return list.find((one) => one.id === id) ?? list.find((one) => one.app === id) ?? null;
}

export function on() {
  const off = new Set(offs());
  return all()
    .map((one) => one.id)
    .filter((id) => !off.has(id));
}

export async function load(id: string): Promise<Loaded | null> {
  const entry = find(id);
  if (!entry || !Object.hasOwn(scenes, entry.app)) return null;
  const scene = await scenes[entry.app]();
  const local = entry.id.slice(entry.app.length + 1);
  const saver = scene.SAVERS?.find((one) => one.id === local) ?? { id: local, title: entry.title, pick: () => ({}) };
  return { entry, scene, saver, still: still(entry, saver) };
}

function still(entry: Listed, saver: Saver) {
  const ms = saver.still ?? STILL;
  if (!Number.isInteger(ms) || ms < 1 || ms > LONGEST)
    throw new Error(`savers: ${entry.id} sets still to ${ms}; it is a whole number of ms from 1 to ${LONGEST}`);
  return ms;
}

/* VALUE */

export const roll = (saver: Pick<Saver, "pick">, seed: number) => ({ seed, ...saver.pick(rng(seed)) });

export function shuffle<T>(ids: readonly T[], rand = Math.random): T[] {
  const out = [...ids];
  for (let i = out.length - 1; i > 0; i--) {
    const j = Math.floor(rand() * (i + 1));
    [out[i], out[j]] = [out[j]!, out[i]!];
  }
  return out;
}
