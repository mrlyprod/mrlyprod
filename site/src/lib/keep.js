/* KEEP */

const prefix = () => (typeof document !== 'undefined' && document.documentElement?.dataset?.prefix) || 'mrly-';

export function held(name, fallback = null) {
  try {
    const raw = localStorage.getItem(`${prefix()}${name}`);
    return raw === null ? fallback : JSON.parse(raw);
  } catch {
    return fallback;
  }
}

export function hold(name, value) {
  try {
    if (value === null || value === undefined) localStorage.removeItem(`${prefix()}${name}`);
    else localStorage.setItem(`${prefix()}${name}`, JSON.stringify(value));
  } catch {}
}
