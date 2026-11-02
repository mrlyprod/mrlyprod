import type { RefObject } from "react";
import { useLayoutEffect, useRef, useState } from "react";
import { flushSync } from "react-dom";

/* ROOM */

type Loose = { detail: number; currentTarget: { blur: () => void } };

export const classes = (...names: (string | undefined)[]) => names.filter(Boolean).join(" ") || undefined;

export const loose = (e: Loose) => e.detail > 0 && e.currentTarget.blur();

const natural = (el: HTMLElement) => {
  const was = el.style.width;
  el.style.width = "max-content";
  const need = el.getBoundingClientRect().width;
  el.style.width = was;
  return need;
};

export function useRoom<T extends HTMLElement>(
  key: string,
  where: (el: T) => boolean = () => true,
): [RefObject<T | null>, boolean] {
  const at = useRef<T>(null);
  const need = useRef({ key, width: 0 });
  const [narrow, setNarrow] = useState(false);
  useLayoutEffect(() => {
    const el = at.current;
    if (!el || !where(el) || typeof ResizeObserver !== "function") return undefined;
    const check = () => {
      if (!narrow) need.current = { key, width: natural(el) };
      setNarrow(need.current.key === key && need.current.width > el.getBoundingClientRect().width);
    };
    check();
    const eye = new ResizeObserver(() => flushSync(check));
    eye.observe(el);
    return () => eye.disconnect();
  }, [narrow, key]);
  return [at, narrow];
}
