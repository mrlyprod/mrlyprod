import { site } from 'site:routes';
import { run } from '../lib/app/run.js';
import { calm } from './prefs.js';
import { all, find, load, on, roll } from './savers.js';

const RANDOM = 'random';
const SEEDS = 4294967296;
const CAP = 1500;
const QUIET = 2000;

const number = (text) => (text.trim() !== '' && Number.isFinite(Number(text)) ? Number(text) : text);

function parse(query) {
  return Object.fromEntries([...new URLSearchParams(query)].map(([key, text]) => [key, number(text)]));
}

function exits(scene) {
  let go = null;
  let left = false;
  let still = 0;
  let cap = 0;
  const done = () => {
    clearTimeout(cap);
    const then = go;
    go = null;
    left = false;
    then?.();
  };
  const fall = () => {
    clearTimeout(still);
    left = true;
    if (!scene.windDown) return done();
    cap = setTimeout(done, CAP);
    scene.windDown(done);
  };
  const stay = () => {
    scene.hold();
    clearTimeout(still);
    still = setTimeout(fall, QUIET);
  };
  const key = (e) => {
    if (go && !left && scene.hold && !e.metaKey && !e.ctrlKey && !e.altKey) stay();
  };
  const shove = (e) => {
    if (!go || left || (e.type === 'wheel' ? Math.abs(e.deltaX) > Math.abs(e.deltaY) : e.touches?.length > 1)) return;
    fall();
  };
  const ears = [['keydown', key], ['wheel', shove], ['touchmove', shove]];
  for (const [type, fn] of ears) window.addEventListener(type, fn, true);
  const leave = (then) => {
    if (left) return;
    go = then;
    if (calm()) return done();
    if (scene.hold) stay();
    else fall();
  };
  const end = () => {
    for (const [type, fn] of ears) window.removeEventListener(type, fn, true);
    clearTimeout(still);
    clearTimeout(cap);
    go = null;
  };
  return { leave, end };
}

function choose(pick, pool) {
  const at = pick.indexOf('?');
  const head = at < 0 ? pick : pick.slice(0, at);
  const named = head === RANDOM ? null : find(head);
  if (named) return { pick, entry: named, query: at < 0 ? '' : pick.slice(at + 1) };
  if (!head) return { pick: '', entry: find(site.desk) ?? all()[0] ?? null, query: '' };
  const kept = (pool ?? on()).map(find).filter(Boolean);
  const list = kept.length ? kept : all();
  return { pick: RANDOM, entry: list[Math.floor(Math.random() * list.length)] ?? null, query: '' };
}

export async function lock(canvas, pick = '', { pool } = {}) {
  const chosen = choose(pick, pool);
  const loaded = chosen.entry ? await load(chosen.entry.id) : null;
  if (!loaded) return Object.assign(() => {}, { pick: chosen.pick, id: '', as: '', leave: (then) => then?.() });
  const { entry, scene, saver } = loaded;
  const seed = Math.floor(Math.random() * SEEDS);
  const value = chosen.query ? { seed, ...parse(chosen.query) } : roll(saver, seed);
  const held = scene.units ? await scene.units() : {};
  await held.ready;
  const { ready, ...units } = held;
  const stop = run(canvas, scene.make, { ...value, ...units }, { lock: true, still: calm() || scene.PAGE?.still });
  const { leave, end } = exits(stop.scene);
  const close = () => {
    end();
    stop();
  };
  return Object.assign(close, stop, { pick: chosen.pick, id: entry.id, as: `${entry.id}?${new URLSearchParams(value)}`, leave });
}
