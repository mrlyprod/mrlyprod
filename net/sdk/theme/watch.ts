import { useEffect, useState } from "react";

/* WATCH */

export function useTheme(): number {
  const [tick, setTick] = useState(0);
  useEffect(() => {
    const bump = () => setTick((was) => was + 1);
    const shade = typeof matchMedia === "function" ? matchMedia("(prefers-color-scheme: dark)") : null;
    addEventListener("theme", bump);
    shade?.addEventListener("change", bump);
    return () => {
      removeEventListener("theme", bump);
      shade?.removeEventListener("change", bump);
    };
  }, []);
  return tick;
}
