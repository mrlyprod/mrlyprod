import { figures, units } from 'live:figures';

const still = matchMedia('(prefers-reduced-motion: reduce)').matches;

/* MOUNT */

async function mount(host) {
  try {
    const [figure, { play }] = await Promise.all([figures[host.dataset.live](), import('../lib/figure.js')]);
    await play(host, figure, (unit) => new URL(units[unit], import.meta.url));
  } catch {}
}

/* WATCH */

function watch() {
  const eye = new IntersectionObserver((entries) => {
    for (const entry of entries) {
      if (!entry.isIntersecting) continue;
      eye.unobserve(entry.target);
      mount(entry.target);
    }
  });
  for (const host of document.querySelectorAll('figure[data-live]')) if (Object.hasOwn(figures, host.dataset.live)) eye.observe(host);
}

if (!still) {
  if (document.readyState === 'complete') watch();
  else window.addEventListener('load', watch, { once: true });
}
