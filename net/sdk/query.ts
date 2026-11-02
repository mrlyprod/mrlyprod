import { useState } from "react";

/* TYPES */

export type Query = Record<string, string | number | boolean>;

export type Stamped = Record<string, string | number | boolean | null | undefined>;

export function share(values: Stamped): string {
  return `?${new URLSearchParams(Object.entries(values).map(([key, value]) => [key, String(value)]))}`;
}

const WAIT = 200;

let pending: Stamped | null = null;

let timer: ReturnType<typeof setTimeout> | 0 = 0;

let path = "";

const write = () => {
  if (document.documentElement.getAttribute("aria-busy") === "true") {
    timer = setTimeout(write, WAIT);
    return;
  }
  const values = pending!;
  pending = null;
  timer = 0;
  if (location.pathname !== path) return;
  const params = new URLSearchParams(location.search);
  for (const [key, value] of Object.entries(values)) {
    if (value === null || value === undefined || value === "" || value === false) params.delete(key);
    else params.set(key, String(value === true ? 1 : value));
  }
  const tail = String(params);
  history.replaceState(history.state, "", location.pathname + (tail ? `?${tail}` : "") + location.hash);
  dispatchEvent(new Event("stamp"));
};

export function stamp(values: Stamped): void {
  if (path !== location.pathname) pending = null;
  pending = { ...pending, ...values };
  path = location.pathname;
  if (!timer) timer = setTimeout(write, WAIT);
}

export function useQuery<T extends Query>(defaults: T): [T, (patch: Partial<T>) => void] {
  const [state, setState] = useState(() => {
    const params = new URLSearchParams(location.search);
    const first = { ...defaults };
    const held: Query = first;
    for (const [key, fallback] of Object.entries(defaults)) {
      if (!params.has(key)) continue;
      const raw = params.get(key)!;
      held[key] =
        typeof fallback === "number"
          ? raw.trim() && Number.isFinite(+raw)
            ? +raw
            : fallback
          : typeof fallback === "boolean"
            ? raw === "1"
            : raw;
    }
    return first;
  });
  const set = (patch: Partial<T>) => {
    stamp(patch);
    setState((old) => ({ ...old, ...patch }));
  };
  return [state, set];
}
