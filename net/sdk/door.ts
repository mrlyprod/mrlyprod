import { useEffect, useRef } from "react";
import type { Patch } from "./spec.ts";
import type { Model, Ports, Read, Verb } from "./verbs.ts";
import { call, list, read } from "./verbs.ts";

/* TYPES */

export type Door = {
  id: string;
  list: () => Verb[];
  read: () => Read;
  call: (name: string, args?: Patch) => Promise<Read>;
};

export type Live<U = unknown> = Model<U> & Ports;

declare global {
  interface Window {
    door?: Door;
  }
}

const WAIT = 600;

/* DOOR */

const settled = () =>
  new Promise<void>((done) => {
    const end = () => {
      clearTimeout(timer);
      removeEventListener("stamp", end);
      requestAnimationFrame(() => done());
    };
    const timer = setTimeout(end, WAIT);
    addEventListener("stamp", end);
  });

export function door<U>(get: () => Live<U>): Door {
  return {
    id: get().id,
    list: () => list(get()),
    read: () => read(get()),
    call: async (name, args) => {
      call(get(), get(), name, args);
      await settled();
      return read(get());
    },
  };
}

export function useDoor<U>(live: Live<U>): void {
  const at = useRef(live);
  at.current = live;
  useEffect(() => {
    const one = door(() => at.current);
    window.door = one;
    return () => {
      if (window.door === one) delete window.door;
    };
  }, []);
}
