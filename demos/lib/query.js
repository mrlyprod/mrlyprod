import { $ } from './mrly.js';

export function query(ids) {
  const params = new URLSearchParams(location.search);
  for (const id of ids) if (params.has(id)) $(id).value = params.get(id);
  return params;
}

export function share(values) {
  return `?${new URLSearchParams(values)}`;
}

export function stamp(values) {
  const params = new URLSearchParams(location.search);
  for (const [key, value] of Object.entries(values)) {
    if (value === null || value === undefined || value === '') params.delete(key);
    else params.set(key, value);
  }
  const tail = String(params);
  history.replaceState(null, '', location.pathname + (tail ? `?${tail}` : ''));
}
