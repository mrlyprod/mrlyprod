import { createRoot } from 'react-dom/client';

export function embed(name, views) {
  if (typeof document === 'undefined') return;
  for (const figure of document.querySelectorAll(`figure.widget[data-demo="${name}"]`)) {
    const View = views[figure.dataset.view];
    const mount = figure.querySelector('.mount');
    if (View && mount) createRoot(mount).render(<View />);
  }
}
