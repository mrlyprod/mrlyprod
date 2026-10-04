import { canvas, ink } from 'mrlyjs/view';
import { dark, light } from '../kit/theme/theme.js';

const SIZE = [1024, 1024];
const scheme = matchMedia('(prefers-color-scheme: dark)');
const doors = new Map();

/* THEME */

function tint() {
  const set = document.documentElement.dataset.theme;
  return ink((set ? set === 'dark' : scheme.matches) ? dark : light);
}

/* UNITS */

function open(name, door, at) {
  if (!doors.has(name)) doors.set(name, door.default({ module_or_path: at(name) }));
  return doors.get(name);
}

/* PLAY */

export async function play(host, figure, at) {
  await Promise.all(Object.entries(figure.units ?? {}).map(([name, door]) => open(name, door, at)));
  const [width, height] = figure.size ?? SIZE;
  const span = figure.loop * 1000;
  const frames = figure.frames ?? 0;
  const start = figure.still ?? 0;
  const node = document.createElement('canvas');
  node.width = width;
  node.height = height;
  node.setAttribute('aria-hidden', 'true');
  const ctx = node.getContext('2d');
  let colors = tint();
  let seen = true;
  let live = false;
  let origin = 0;
  let spent = 0;
  let timer = 0;
  let frame = 0;
  const phase = () => ((live ? performance.now() - origin : spent) / span + start) % 1;
  const paint = () => {
    const now = phase();
    const step = Math.floor(now * frames);
    figure.default(canvas(ctx, width, height, colors.ground), colors, frames ? step / frames : now);
    return frames ? ((step + 1) / frames - now) * span : 0;
  };
  const halt = () => {
    if (!live) return;
    spent = performance.now() - origin;
    live = false;
    clearTimeout(timer);
    cancelAnimationFrame(frame);
  };
  const guard = (work) => {
    try {
      work();
    } catch {
      halt();
      node.remove();
    }
  };
  const tick = () =>
    guard(() => {
      const wait = paint();
      if (frames) timer = setTimeout(tick, Math.max(wait, 16));
      else frame = requestAnimationFrame(tick);
    });
  const wake = () => {
    if (!seen || document.hidden) return halt();
    if (live || !node.isConnected) return;
    live = true;
    origin = performance.now() - spent;
    tick();
  };
  const shade = () =>
    guard(() => {
      colors = tint();
      paint();
    });
  paint();
  host.append(node);
  const eye = new IntersectionObserver(([entry]) => {
    seen = entry.isIntersecting;
    wake();
  });
  eye.observe(host);
  scheme.addEventListener('change', shade);
  window.addEventListener('theme', shade);
  document.addEventListener('visibilitychange', wake);
  wake();
  return () => {
    halt();
    eye.disconnect();
    scheme.removeEventListener('change', shade);
    window.removeEventListener('theme', shade);
    document.removeEventListener('visibilitychange', wake);
    node.remove();
  };
}
