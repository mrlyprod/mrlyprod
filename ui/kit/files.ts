import type { Json } from "@mrlyprod/os/kernel/index.ts";
import { useSyncExternalStore } from "react";

type Held = { value: Json | null; ears: Set<() => void>; stop: () => void };

const EVERY = 60 * 1000;

const files = new Map<string, Held>();

const grab = async (url: string): Promise<Json | null> => {
  try {
    const reply = await fetch(url, { cache: "no-store" });
    return reply.ok ? ((await reply.json()) as Json) : null;
  } catch {
    return null;
  }
};

function watch(url: string, held: Held) {
  let timer: ReturnType<typeof setInterval> | 0 = 0;
  const refresh = async () => {
    held.value = (await grab(url)) ?? held.value;
    for (const ear of held.ears) ear();
  };
  const run = () => {
    clearInterval(timer);
    if (document.hidden) return;
    void refresh();
    timer = setInterval(refresh, EVERY);
  };
  document.addEventListener("visibilitychange", run);
  run();
  return () => {
    clearInterval(timer);
    document.removeEventListener("visibilitychange", run);
  };
}

const heldOf = (url: string) => {
  let held = files.get(url);
  if (!held) {
    held = { value: null, ears: new Set(), stop: () => {} };
    files.set(url, held);
  }
  return held;
};

const subscribe = (url: string | null) => (ear: () => void) => {
  if (!url) return () => {};
  const held = heldOf(url);
  if (!held.ears.size) held.stop = watch(url, held);
  held.ears.add(ear);
  return () => {
    held.ears.delete(ear);
    if (!held.ears.size) held.stop();
  };
};

export const useFile = (url: string | null): Json | null => {
  const snap = () => (url ? heldOf(url).value : null);
  return useSyncExternalStore(subscribe(url), snap, snap);
};

const entries = (value: Json | null | undefined): [string, Json][] =>
  value && typeof value === "object" ? Object.entries(value) : [];

export function dig(value: Json | null | undefined, parts: string[]): Json | null {
  if (value === undefined || value === null) return null;
  if (!parts.length) return value;
  const [head, ...rest] = parts;
  if (head !== "*") return value && typeof value === "object" && !Array.isArray(value) ? dig(value[head!], rest) : null;
  const rowOf = (key: string, row: Json): Json =>
    row && typeof row === "object" && !Array.isArray(row) ? { key, ...row } : { key, value: row };
  return entries(value).flatMap(([key, one]): Json[] => {
    const found = dig(one, rest);
    return Array.isArray(found) ? found.map((row) => rowOf(key, row)) : [rowOf(key, found)];
  });
}
