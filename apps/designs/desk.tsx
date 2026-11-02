import { useEffect, useMemo, useRef, useState } from "react";
import { paints } from "../../net/sdk/color.ts";
import { useUnit } from "../../net/sdk/hooks.ts";
import type { Desk } from "../../net/sdk/rows.ts";
import type { Mrly } from "../shared/designs/engine.ts";
import { classes, dimension } from "../shared/designs/engine.ts";
import { useTheme } from "../shared/designs/pick.tsx";
import { thumb } from "../shared/designs/thumb.ts";

/* TYPES */

export type Daily = { dim: number; base: number; code: string; label: string; href: string };

const MINUTE = 60000;
const SPACES: [number, number][] = [
  [2, 2],
  [3, 2],
  [2, 3],
];

const two = (n: number) => String(n).padStart(2, "0");

/* DAY */

export const today = (now = new Date()) =>
  `${now.getUTCFullYear()}-${two(now.getUTCMonth() + 1)}-${two(now.getUTCDate())}`;

export function daily(day: string, n: number) {
  let h = 0x811c9dc5;
  for (const c of day) h = Math.imul(h ^ c.charCodeAt(0), 0x01000193) >>> 0;
  return n > 0 ? h % n : 0;
}

export function chosen(math: Mrly, day: string): Daily | null {
  const pool = SPACES.flatMap(([dim, base]) =>
    (classes(math, dim, base) ?? [])
      .map((one) => String(one.code))
      .filter((code) => {
        const d = dimension(math, dim, base, code);
        return d > dim / 2 && d < dim;
      })
      .map((code) => ({ dim, base, code })),
  );
  const one = pool[daily(day, pool.length)];
  if (!one) return null;
  const name = math.name.of(one.code, one.dim, one.base) ?? "code";
  return {
    ...one,
    label: `${name} ${one.code} · ${one.dim}D${one.base === 3 ? ", base 3" : ""}`,
    href: `/designs/?dim=${one.dim}&base=${one.base}&code=${one.code}`,
  };
}

function useDay() {
  const [day, setDay] = useState(() => today());
  useEffect(() => {
    const check = () => !document.hidden && setDay(today());
    const timer = setInterval(check, MINUTE);
    document.addEventListener("visibilitychange", check);
    return () => {
      clearInterval(timer);
      document.removeEventListener("visibilitychange", check);
    };
  }, []);
  return day;
}

function Day() {
  const math = useUnit("math");
  const tick = useTheme();
  const day = useDay();
  const pick = useMemo(() => (math ? chosen(math, day) : null), [math, day]);
  const art = useRef<HTMLCanvasElement>(null);
  useEffect(() => {
    const canvas = art.current;
    if (!canvas || !pick) return;
    const side = Math.round((canvas.getBoundingClientRect().width || 96) * Math.min(devicePixelRatio || 1, 2));
    canvas.width = side;
    canvas.height = side;
    thumb(canvas.getContext("2d")!, math!, pick.dim, pick.base, pick.code, side, paints(canvas));
  }, [pick, tick]);
  return (
    <a className="fill" href={pick?.href ?? "/designs/"}>
      <h2>Design of the day</h2>
      <canvas className="art" ref={art} aria-hidden="true" />
      <p>{pick?.label ?? "Designs"}</p>
    </a>
  );
}

/* WIDGET */

export const widget: Desk = { name: "Design of the day", size: "small", Body: Day };
