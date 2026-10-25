/* KEEP */

const prefix = () => (typeof document !== "undefined" && document.documentElement?.dataset?.prefix) || "mrly-";

export function held(name: string): unknown;
export function held<T>(name: string, fallback: T): T;
export function held<T>(name: string, fallback: T | null = null): T | null {
  try {
    const raw = localStorage.getItem(`${prefix()}${name}`);
    return raw === null ? fallback : JSON.parse(raw);
  } catch {
    return fallback;
  }
}

export function hold(name: string, value: unknown): void {
  try {
    if (value === null || value === undefined) localStorage.removeItem(`${prefix()}${name}`);
    else localStorage.setItem(`${prefix()}${name}`, JSON.stringify(value));
  } catch {}
}
