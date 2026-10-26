import type { ReactNode, RefObject } from "react";
import { createContext, useContext, useEffect, useRef, useState } from "react";
import type { Units as Loaded, Name, Unit } from "../units/index.ts";
import { wasm } from "../units/index.ts";

/* TYPES */

export type Readout = readonly [string, ReactNode];

export type Ear = (seed: number) => void;

export type Roller = { listen: (ear: Ear) => () => void; tell: (seed: number) => void };

export type Ready = { ready: unknown };

export type Thunk<U extends Ready> = () => Promise<U>;

export type Bag<U extends Ready> = Omit<U, "ready">;

export type Had<N extends Name> = { [K in N]: Unit<K> };

export type Three = typeof import("three");

const ONE: { [N in Name]?: Thunk<Loaded<N>> } = {};

export function useLatest<T>(value: T): RefObject<T> {
  const at = useRef(value);
  at.current = value;
  return at;
}

export function useStatus(onStatus: ((rows: Readout[]) => void) | undefined, rows: Readout[]): void {
  const key = JSON.stringify(rows);
  useEffect(() => onStatus?.(rows), [key]);
}

export const Rolls = createContext<Roller | null>(null);

export function rolls(): Roller {
  const ears = new Set<Ear>();
  return {
    listen: (ear) => {
      ears.add(ear);
      return () => ears.delete(ear);
    },
    tell: (seed) => {
      for (const ear of ears) ear(seed);
    },
  };
}

export function useReroll(seed: number, roll: Ear): void {
  const told = useContext(Rolls);
  const was = useRef(seed);
  const latest = useLatest(roll);
  useEffect(() => told?.listen((next) => latest.current(next)), [told]);
  useEffect(() => {
    if (told || was.current === seed) return;
    was.current = seed;
    latest.current(seed);
  }, [seed, told]);
}

export function useUnits<N extends Name>(
  units: Thunk<Loaded<N>> | null | undefined,
  given?: Had<N> | null,
): Had<N> | null;
export function useUnits<U extends Ready>(units: Thunk<U> | null | undefined, given?: Bag<U> | null): Bag<U> | null;
export function useUnits<U extends Ready>(units: Thunk<U> | null | undefined, given?: Bag<U> | null): Bag<U> | null {
  const [held, hold] = useState<Bag<U> | null>(null);
  useEffect(() => {
    if (given || !units) return undefined;
    let on = true;
    units().then(async ({ ready, ...rest }) => {
      await ready;
      if (on) hold(rest);
    });
    return () => {
      on = false;
    };
  }, [units, given]);
  return given ?? held;
}

export function useUnit<N extends Name>(name: N, given?: Unit<N> | null): Unit<N> | null {
  (ONE as Partial<Record<N, Thunk<Loaded<N>>>>)[name] ??= wasm(name);
  const held = useUnits(given ? null : ONE[name]);
  return given ?? held?.[name] ?? null;
}

export function useThree(wanted: boolean, given?: Three | null): Three | null {
  const [three, setThree] = useState<Three | null>(null);
  useEffect(() => {
    if (given || !wanted || three) return undefined;
    let on = true;
    import("three").then((mod) => on && setThree(mod));
    return () => {
      on = false;
    };
  }, [given, wanted, three]);
  return given ?? three;
}
