import { site } from "site:routes";
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

export type Live<U = unknown> = Omit<Model<U>, "user"> & Ports;

const user = site.title.toLowerCase().replace(/[^a-z0-9]/g, "");

declare global {
  interface Window {
    door?: Door;
  }
}

const WAIT = 600;

/* DOOR */

const WRITES = ["set", "reroll"];

const framed = () => new Promise<void>((done) => requestAnimationFrame(() => requestAnimationFrame(() => done())));

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

export function door<U>(live: () => Live<U>): Door {
  const get = () => ({ ...live(), user });
  return {
    id: get().id,
    list: () => list(get()),
    read: () => read(get()),
    call: async (name, args) => {
      call(get(), get(), name, args);
      await (WRITES.includes(name.split(".").at(-1)!) ? settled() : framed());
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
