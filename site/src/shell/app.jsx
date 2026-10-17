import { apps, own } from 'site:apps';
import { escape } from '../lib/md/text.ts';

const APP = { wide: true, bare: true, sheet: 'mrly' };

const live = new WeakMap();

const chunk = (id) => (apps[id] ? apps[id]() : Promise.reject(new Error(`app: no chunk for ${id}`)));

const still = {
  layout: APP,
  render(row, site, host) {
    const title = escape(row.title);
    const figure = row.figure ? `<figure class="opener" data-figure="${escape(row.figure)}"><canvas role="img" aria-label="${title}"></canvas></figure>` : '';
    host.innerHTML = `<div id="root" data-app="${escape(row.meta.id)}"></div><div><h1>${title}</h1>${figure}<noscript><p>This app draws in the browser and needs JavaScript.</p></noscript></div>`;
  },
  async mount(host, context) {
    const at = host.querySelector('#root');
    const entry = await chunk(at.dataset.app);
    if (!at.isConnected || live.has(host)) return;
    entry.mount(at, context);
    live.set(host, () => entry.unmount(at));
  },
  unmount(host) {
    live.get(host)?.();
    live.delete(host);
  },
};

export function page(row) {
  return own.has(row.meta.id) ? chunk(row.meta.id) : still;
}
