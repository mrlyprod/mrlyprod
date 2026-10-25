import { useCallback, useEffect, useRef, useState } from "react";
import { text } from "../../lib/app/export.ts";
import type { Had, Readout } from "../../lib/app/hooks.ts";
import { useReroll } from "../../lib/app/hooks.ts";
import type { Paint } from "../../lib/app/library.tsx";
import { Art, Item, Library, Shelf } from "../../lib/app/library.tsx";
import type { Handle, WidgetProps } from "../../lib/app/page.tsx";
import { Scene } from "../../lib/app/page.tsx";
import type { Opts, View } from "../../lib/app/run.ts";
import type { Patch } from "../../lib/app/spec.ts";
import { paints } from "../../lib/color.ts";
import type { Mrly } from "../../lib/designs/engine.ts";
import { next } from "../../lib/designs/engine.ts";
import { useTheme } from "../../lib/designs/pick.tsx";
import { rng } from "../../lib/rand.ts";
import type { Lawful, Told } from "./engine.ts";
import { BASES, cap, curves, laws, looping, names, resolve } from "./engine.ts";
import type { Arcs, Handled, Knobs } from "./scene.ts";
import { file, make, shades, sketch } from "./scene.ts";

/* TYPES */

type Seeded = Knobs & { seed: number };

export type LawsProps = { math: Mrly; value: Knobs; tick: number; onChange: (patch: Patch) => void };

type Props = WidgetProps<Had<"math">, Handled> & { value: Seeded };

type Hooks = Partial<Pick<Props, "onChange" | "onStatus">>;

const SMALL: Partial<Record<number, number>> = { 2: 3, 3: 2 };

export function rows(f: Told | null): Readout[] {
  if (!f) return [];
  const design: Readout = ["design", f.name ? `${f.name} ${f.code}` : `code ${f.code}`];
  if (f.fault) return [design, ["fault", f.fault]];
  const law: Readout[] = f.law
    ? [
        ["law", `L(n) = ${f.law.formula}`],
        [`L(${f.level})`, `${f.law.loops}, the law ${f.law.loops === f.loops ? "meets" : "misses"} the count`],
      ]
    : [["law", "no closed form proved for this code"]];
  return [
    design,
    ["loops", String(f.loops)],
    ["strands", `${f.strands} = 2 x ${f.side}, proved`],
    ["side", String(f.side)],
    ["filled", `${f.filled} of ${f.cells}`],
    ...law,
  ];
}

export function Laws({ math, value, tick, onChange }: LawsProps) {
  const code = resolve(math, value.base, value.code);
  const paint =
    (one: Lawful): Paint =>
    (ctx, w, h, dpr) =>
      sketch(
        ctx,
        w,
        h,
        dpr,
        curves(math, one.code, one.base, SMALL[one.base] ?? 1),
        value,
        shades(ctx.canvas, paints(ctx.canvas, value.tint), value.strand),
      );
  return BASES.map(([base]) => {
    const list = laws(math).filter((one) => one.base === base);
    if (!list.length) return null;
    const level = Math.min(value.level, cap(math, base));
    return (
      <Shelf key={base} name={`Base ${base}`}>
        {list.map((one) => (
          <Item
            key={one.code}
            label={one.name ? `${one.name} ${one.code}` : `code ${one.code}`}
            sub={`${Number(math.arcs.law(one.code, base, level)!.loops)} loops`}
            title={`L(n) = ${one.formula}`}
            on={Number(value.base) === base && code === one.code}
            onPick={() => onChange({ base, code: one.code })}
            art={
              <Art paint={paint(one)} deps={[tick, value.tint, value.strand, value.cells, value.look, value.width]} />
            }
          />
        ))}
      </Shelf>
    );
  });
}

export function Widget({ value, onChange, onReady, onExport, onStatus, units }: Props) {
  const unit = units?.math;
  const tick = useTheme();
  const live = useRef<Handle<Arcs> | null>(null);
  const now = useRef(value);
  now.current = value;
  const hooks = useRef<Hooks>({});
  hooks.current = { onChange, onStatus };
  const [facts, setFacts] = useState<Told | null>(null);
  useReroll(value.seed, (seed) => unit && onChange({ code: looping(unit, value.base, value.level, rng(seed)) }));
  const tell = useCallback((f: Told) => {
    setFacts(f);
    hooks.current.onStatus?.(rows(f));
  }, []);
  const scene = useCallback(
    (canvas: HTMLCanvasElement, view: View, opts: Opts) => make(canvas, view, { ...opts, math: unit, onTell: tell }),
    [unit, tell],
  );
  const step = (by: number) => {
    const { base, code } = now.current;
    hooks.current.onChange!({
      code: next(
        names(unit!, base).map(([one]) => one),
        resolve(unit!, base, code),
        by,
      ),
    });
  };
  const deeper = (by: number) => hooks.current.onChange!({ level: now.current.level + by });
  const ready = (handle: Handle<Arcs> | null) => {
    live.current = handle;
    if (!handle) hooks.current.onStatus?.([]);
    onReady?.(handle && { ...handle, step, deeper });
  };
  const keep = () => {
    const picture = live.current?.svg?.();
    if (picture) text(picture, file(now.current, units), "svg");
  };
  const have = Boolean(unit);
  useEffect(() => onExport?.(have ? [["SVG", keep]] : []), [have]);
  const label = `${facts?.name || "a design"} at level ${value.level} drawn in arcs`;
  return (
    <>
      {unit && (
        <Scene
          make={scene}
          value={value}
          onReady={ready}
          onClick={() => live.current?.trace?.()}
          role="img"
          aria-label={label}
        />
      )}
      {unit && (
        <Library name="Laws">
          <Laws math={unit} value={value} tick={tick} onChange={onChange} />
        </Library>
      )}
    </>
  );
}
