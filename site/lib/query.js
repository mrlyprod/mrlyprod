import { useState } from 'react';

export function share(values) {
  return `?${new URLSearchParams(values)}`;
}

const WAIT = 200;

let pending = null;

let timer = 0;

const write = () => {
  const values = pending;
  pending = null;
  timer = 0;
  if (document.documentElement.getAttribute('aria-busy') === 'true') return;
  const params = new URLSearchParams(location.search);
  for (const [key, value] of Object.entries(values)) {
    if (value === null || value === undefined || value === '' || value === false) params.delete(key);
    else params.set(key, value === true ? 1 : value);
  }
  const tail = String(params);
  history.replaceState(history.state, '', location.pathname + (tail ? `?${tail}` : '') + location.hash);
};

export function stamp(values) {
  pending = { ...pending, ...values };
  if (!timer) timer = setTimeout(write, WAIT);
}

export function useQuery(defaults) {
  const [state, setState] = useState(() => {
    const params = new URLSearchParams(location.search);
    const first = { ...defaults };
    for (const [key, fallback] of Object.entries(defaults)) {
      if (!params.has(key)) continue;
      const raw = params.get(key);
      first[key] = typeof fallback === 'number' ? (raw.trim() && Number.isFinite(+raw) ? +raw : fallback) : typeof fallback === 'boolean' ? raw === '1' : raw;
    }
    return first;
  });
  const set = (patch) => {
    stamp(patch);
    setState((old) => ({ ...old, ...patch }));
  };
  return [state, set];
}
