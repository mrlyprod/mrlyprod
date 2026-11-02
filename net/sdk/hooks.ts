import type { ReactNode, RefObject } from "react";
import { useEffect, useRef, useState } from "react";
import type { Units as Loaded, Name, Unit } from "../../apps/shared/units/index.ts";
import { wasm } from "../../apps/shared/units/index.ts";

/* TYPES */

export type Readout = readonly [string, ReactNode];

export type Ready = { ready: unknown };

export type Thunk<U extends Ready> = () => Promise<U>;

export type Bag<U extends Ready> = Omit<U, "ready">;

export type Had<N extends Name> = { [K in N]: Unit<K> };

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
