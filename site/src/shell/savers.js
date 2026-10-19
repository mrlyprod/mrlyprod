import { scenes } from 'apps:scenes';
import { site } from 'site:routes';
import { rng } from '../lib/rand.js';
import { offs } from './prefs.js';

/* REGISTRY */

export const all = () => site.savers ?? [];

export function find(id) {
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

export async function load(id) {
  const entry = find(id);
  if (!entry || !Object.hasOwn(scenes, entry.app)) return null;
  const scene = await scenes[entry.app]();
  const local = entry.id.slice(entry.app.length + 1);
  const saver = scene.SAVERS?.find((one) => one.id === local) ?? { id: local, title: entry.title, pick: () => ({}) };
  return { entry, scene, saver };
}

/* VALUE */

export const roll = (saver, seed) => ({ seed, ...saver.pick(rng(seed)) });

export function shuffle(ids, rand = Math.random) {
  const out = [...ids];
  for (let i = out.length - 1; i > 0; i--) {
    const j = Math.floor(rand() * (i + 1));
    [out[i], out[j]] = [out[j], out[i]];
  }
  return out;
}
