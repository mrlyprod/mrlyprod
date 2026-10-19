import { start } from '../../lib/git/client.ts';

/* MARKDOWN */

async function md(text, link) {
  const [{ front, render }, katex] = await Promise.all([import('../../lib/md/md.ts'), text.includes('$') ? import('katex') : null]);
  const math = katex ? (tex, display) => katex.default.renderToString(tex, { output: 'mathml', throwOnError: false, displayMode: display }) : undefined;
  return render(front(text).body, { math, link });
}

const paint = () => import('../../lib/git/code.ts').then((one) => one.paint);

/* VIEWER */

export const view = (host, after) => start({ tree: '/git.json', mount: host, md, paint, after });

const live = new Map();

export function mount(host) {
  if (!live.has(host)) live.set(host, view(host));
}

export function unmount(host) {
  live.get(host)?.();
  live.delete(host);
}
