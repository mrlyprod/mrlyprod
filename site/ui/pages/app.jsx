import { apps } from 'site:apps';
import { escape } from '../../kit/md/text.ts';

const live = new WeakMap();

export function render(row, site, host) {
  const title = escape(row.title);
  host.innerHTML = `<div id="root" data-app="${escape(row.meta.id)}"></div><div><h1>${title}</h1><figure class="opener" data-figure="${escape(row.figure)}"><canvas role="img" aria-label="${title}"></canvas></figure><noscript><p>This app draws in the browser and needs JavaScript.</p></noscript></div>`;
}

export async function mount(host) {
  const at = host.querySelector('#root');
  const thunk = apps[at.dataset.app];
  if (!thunk) throw new Error(`app: no chunk for ${at.dataset.app}`);
  const entry = await thunk();
  if (!at.isConnected || live.has(host)) return;
  entry.mount(at);
  live.set(host, () => entry.unmount(at));
}

export function unmount(host) {
  live.get(host)?.();
  live.delete(host);
}
