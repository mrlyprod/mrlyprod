import { createContext, useContext, useEffect, useRef, useState } from "react";
import { wasm } from "../units/index.js";

const ONE = {};

export function useLatest(value) {
  const at = useRef(value);
  at.current = value;
  return at;
}

export function useStatus(onStatus, rows) {
  const key = JSON.stringify(rows);
  useEffect(() => onStatus?.(rows), [key]);
}

export const Rolls = createContext(null);

export function rolls() {
  const ears = new Set();
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

export function useReroll(seed, roll) {
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

export function useUnits(units, given) {
  const [held, hold] = useState(null);
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

export function useUnit(name, given) {
  ONE[name] ??= wasm(name);
  const held = useUnits(ONE[name], given);
  return given ?? held?.[name] ?? null;
}

export function useThree(wanted, given) {
  const [three, setThree] = useState(null);
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
