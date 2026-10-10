import { scenes } from 'apps:scenes';
import { run } from './scene.js';

const RANDOM = 'random';
const STILL = '(prefers-reduced-motion: reduce)';
const CAP = 1500;
const QUIET = 2000;

const number = (text) => (text.trim() !== '' && Number.isFinite(Number(text)) ? Number(text) : text);

function parse(query) {
  return Object.fromEntries([...new URLSearchParams(query)].map(([key, text]) => [key, number(text)]));
}

function exits(scene) {
  let go = null;
  let left = false;
  let calm = 0;
  let cap = 0;
  const done = () => {
    clearTimeout(cap);
    const then = go;
    go = null;
    then?.();
  };
  const fall = () => {
    clearTimeout(calm);
    left = true;
    if (!scene.windDown) return done();
    cap = setTimeout(done, CAP);
    scene.windDown(done);
  };
  const stay = () => {
    scene.hold();
    clearTimeout(calm);
    calm = setTimeout(fall, QUIET);
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
    if (typeof matchMedia === 'function' && matchMedia(STILL).matches) {
      left = true;
      return done();
    }
    if (scene.hold) stay();
    else fall();
  };
  const end = () => {
    for (const [type, fn] of ears) window.removeEventListener(type, fn, true);
    clearTimeout(calm);
    clearTimeout(cap);
    go = null;
  };
  return { leave, end };
}

export async function lock(canvas, pick = '') {
  const ids = Object.keys(scenes);
  const at = pick.indexOf('?');
  const head = at < 0 ? pick : pick.slice(0, at);
  const known = Object.hasOwn(scenes, head);
  const kept = known || !head ? pick : RANDOM;
  const id = known ? head : kept && ids.length ? ids[Math.floor(Math.random() * ids.length)] : '';
  if (!id) return Object.assign(() => {}, { pick: kept });
  const scene = await scenes[id]();
  const held = scene.units ? await scene.units() : {};
  await held.ready;
  const { ready, ...units } = held;
  const stop = run(canvas, scene.make, { ...(known && at >= 0 ? parse(pick.slice(at + 1)) : {}), ...units });
  const { leave, end } = exits(stop.scene);
  const close = () => {
    end();
    stop();
  };
  return Object.assign(close, stop, { pick: kept, leave });
}
