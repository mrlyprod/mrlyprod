import type { ReactNode } from "react";
import { createRoot } from "react-dom/client";

/* TYPES */

export type Opened = { at?: Element; node: ReactNode; close?: () => void };

export type Open<C> = (host: HTMLElement, context: C) => Opened | null | undefined;

export type Island<C> = { mount: (host: HTMLElement, context: C) => void; unmount: (host: HTMLElement) => void };

export function island<C = unknown>(open: Open<C>): Island<C> {
  const live = new Map<HTMLElement, () => void>();
  return {
    mount(host, context) {
      if (live.has(host)) return;
      const opened = open(host, context);
      if (!opened) return;
      const { at = host, node, close } = opened;
      const root = createRoot(at);
      root.render(node);
      live.set(host, () => {
        root.unmount();
        close?.();
      });
    },
    unmount(host) {
      live.get(host)?.();
      live.delete(host);
    },
  };
}
