const live = new Map();
const up = new Map();
const seen = new Map();

const named = (root) => [...new Set([...root.querySelectorAll('[data-island]')].map((host) => host.dataset.island))];

const text = (src, cache) => fetch(src, { cache }).then((reply) => (reply.ok ? reply.text() : ''), () => '');

function keep(src) {
  if (!seen.has(src)) seen.set(src, text(src, 'force-cache'));
}

async function pull(src) {
  if (seen.has(src) && (await seen.get(src)) !== (await text(src, 'default'))) throw new Error(`islands: ${src} changed under this session`);
  const entry = await import(src);
  keep(src);
  return entry;
}

export const load = (root = document) => Promise.all(named(root).map(pull));

export function mount(root = document) {
  const jobs = [];
  for (const host of root.querySelectorAll('[data-island]')) {
    if (live.has(host)) continue;
    const src = host.dataset.island;
    const job = import(src).then((entry) => {
      keep(src);
      if (live.get(host) !== job) return;
      entry.mount(host);
      up.set(host, entry);
    });
    live.set(host, job);
    jobs.push(job);
  }
  return Promise.all(jobs);
}

export function unmount(root = document) {
  for (const host of [...live.keys()]) {
    if (root !== document && host.isConnected && !root.contains(host)) continue;
    live.delete(host);
    up.get(host)?.unmount(host);
    up.delete(host);
  }
}
