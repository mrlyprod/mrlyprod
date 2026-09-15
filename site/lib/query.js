import { useState } from 'react';

export function share(values) {
  return `?${new URLSearchParams(values)}`;
}

export function stamp(values) {
  const params = new URLSearchParams(location.search);
  for (const [key, value] of Object.entries(values)) {
    if (value === null || value === undefined || value === '' || value === false) params.delete(key);
    else params.set(key, value === true ? 1 : value);
  }
  const tail = String(params);
  history.replaceState(null, '', location.pathname + (tail ? `?${tail}` : ''));
}

export function useQuery(defaults) {
  const [state, setState] = useState(() => {
    const params = new URLSearchParams(location.search);
    const first = { ...defaults };
    for (const [key, fallback] of Object.entries(defaults)) {
      if (!params.has(key)) continue;
      const raw = params.get(key);
      first[key] = typeof fallback === 'number' ? +raw : typeof fallback === 'boolean' ? raw === '1' : raw;
    }
    return first;
  });
  const set = (patch) => {
    stamp(patch);
    setState((old) => ({ ...old, ...patch }));
  };
  return [state, set];
}
