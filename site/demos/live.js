import { figures, units } from 'live:figures';

const still = matchMedia('(prefers-reduced-motion: reduce)').matches;

const live = new Map();

/* PLAY */

async function show(host, held) {
  try {
    const [figure, { play }] = await Promise.all([figures[host.dataset.live](), import('../lib/figure.js')]);
    if (live.get(host) !== held) return;
    const stop = await play(host, figure, (unit) => new URL(units[unit], import.meta.url));
    if (live.get(host) === held) held.stop = stop;
    else stop();
  } catch {}
}

/* ISLAND */

export function mount(host) {
  if (still || live.has(host) || !Object.hasOwn(figures, host.dataset.live)) return;
  const held = { stop: null, quit: null };
  const eye = new IntersectionObserver((entries) => {
    if (!entries.some((entry) => entry.isIntersecting)) return;
    eye.disconnect();
    show(host, held);
  });
  const watch = () => eye.observe(host);
  if (document.readyState === 'complete') watch();
  else window.addEventListener('load', watch, { once: true });
  held.quit = () => {
    window.removeEventListener('load', watch);
    eye.disconnect();
    held.stop?.();
  };
  live.set(host, held);
}

export function unmount(host) {
  const held = live.get(host);
  if (!held) return;
  live.delete(host);
  held.quit();
}
