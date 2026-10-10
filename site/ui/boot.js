(() => {
  const d = document.documentElement;
  d.classList.add('js');
  try {
    for (const k of ['theme', 'font', 'tint']) {
      const v = localStorage.getItem(`mrly-${k}`);
      if (v) d.dataset[k] = v;
    }
    if (['/', '/index.html'].includes(location.pathname) && CSS.supports('animation-timeline', 'scroll()')) d.dataset.welcome = localStorage.getItem('mrly-welcome') || matchMedia('(prefers-reduced-motion: reduce)').matches ? 'shut' : 'open';
  } catch {}
})();
