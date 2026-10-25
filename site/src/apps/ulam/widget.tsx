import type { MouseEvent } from "react";
import { useCallback, useEffect, useRef, useState } from "react";
import type { Action } from "../../lib/app/controls.tsx";
import { text } from "../../lib/app/export.ts";
import { Bar } from "../../lib/app/frame.tsx";
import type { Readout } from "../../lib/app/hooks.ts";
import { useReroll, useUnit } from "../../lib/app/hooks.ts";
import { Art, Item, Library, Shelf } from "../../lib/app/library.tsx";
import type { Handle, WidgetProps } from "../../lib/app/page.tsx";
import { Scene } from "../../lib/app/page.tsx";
import type { Opts, View } from "../../lib/app/run.ts";
import type { Units } from "../../lib/app/spec.ts";
import { paints } from "../../lib/color.ts";
import { useTheme } from "../../lib/designs/pick.tsx";
import { rng } from "../../lib/rand.ts";
import type { Entry, Num, Run, Setting } from "./engine.ts";
import { factors, formula, holds, POLYS, REACH, SPIRALS, shuffle, step } from "./engine.ts";
import type { Handled, Knobs, Spot, Told, Ulam } from "./scene.ts";
import { make, named, sketch } from "./scene.ts";

/* TYPES */

type Props = Partial<WidgetProps<Units, Handled>> &
  Pick<WidgetProps<Units, Handled>, "value" | "onChange"> & { value: Knobs & { seed: number }; unit?: Num | null };

const ART = 112;

const pct = (x: number, places = 1) => `${(x * 100).toFixed(places)}%`;

const LABEL =
  "The sheet: the whole numbers wound from one on the lattice, the marked ones lit and the quadratic in orange and blue";

export function streak(line: Pick<Run, "count" | "streak" | "next">): string {
  if (!line.count) return "off the sheet";
  if (line.streak === line.count) return `all ${line.count} prime`;
  return `${line.streak}, then ${line.next!.n} = ${factors(line.next!.factors)}`;
}

export const values = (line: Pick<Run, "values" | "more">) =>
  line.values.length ? `${line.values.join(" ")}${line.more ? " …" : ""}` : "none on the sheet";

export function verdict(pick: Spot): string {
  if (pick.n === 1) return "one, neither prime nor composite";
  return pick.prime ? "prime" : `composite, ${factors(pick.factors)}`;
}

export function readouts(f: Told): Readout[] {
  const pick: Readout[] = f.pick?.off
    ? [["Cell", "off the sheet"]]
    : f.pick
      ? [
          ["Cell", `${f.pick.n} on ring ${f.pick.ring} at ${f.pick.x}, ${f.pick.y}`],
          ["Verdict", verdict(f.pick)],
        ]
      : [];
  const line: Readout[] = f.line
    ? [
        ["Values", values(f.line)],
        ["Streak", streak(f.line)],
        ["Hits", `${f.line.hits} of ${f.line.count}`],
        ["Share", pct(f.line.share)],
      ]
    : [];
  const lit: Readout[] = f.mark === "prime" ? [] : [["Lit", f.lit]];
  return [
    ...(f.line ? [["Line", formula(f.a, f.b, f.c)] satisfies Readout] : []),
    ...pick,
    ...line,
    ["Numbers", f.top],
    ["Primes", f.primes],
    ["Density", pct(f.density, 2)],
    ...lit,
  ];
}

export function Widget({ value, onChange, onReady, onExport, onStatus, unit: given }: Props) {
  const unit = useUnit("num", given);
  const [facts, setFacts] = useState<Told | null>(null);
  const live = useRef<Handle<Ulam> | null>(null);
  const now = useRef(value);
  now.current = value;
  const change = useRef(onChange);
  change.current = onChange;
  const tick = useTheme();
  useReroll(value.seed, (next) => unit && change.current(shuffle(unit, rng(next))));
  const scene = useCallback(
    (canvas: HTMLCanvasElement, view: View, opts: Opts) =>
      make(canvas, view, { ...opts, num: unit, onFacts: setFacts }),
    [unit],
  );
  const ready = (handle: Handle<Ulam> | null) => {
    live.current = handle;
    setFacts(handle?.facts ?? null);
    const line = (by: number) => {
      const { a, c } = now.current;
      const next = a > 0 ? step(unit!, c, by, REACH) : c;
      if (next !== c) change.current({ c: next });
    };
    onReady?.(handle && { ...handle, line });
  };
  const tap = (e: MouseEvent<HTMLCanvasElement>) => {
    const box = e.currentTarget.getBoundingClientRect();
    live.current?.pick((e.clientX - box.left) / box.width, (e.clientY - box.top) / box.height);
  };
  const keep = (kind: "svg" | "csv") => {
    const body = live.current?.[kind]?.();
    if (body) text(body, named(now.current), kind);
  };
  const have = Boolean(facts);
  const lined = Boolean(facts?.line);
  useEffect(
    () =>
      onExport?.(
        have ? [["SVG", () => keep("svg")], ...(lined ? [["CSV", () => keep("csv")] satisfies Action] : [])] : [],
      ),
    [have, lined],
  );
  const rows = facts ? readouts(facts) : [];
  const said = JSON.stringify(rows);
  useEffect(() => onStatus?.(rows), [said]);
  const shelf = <S extends Setting>(name: string, list: Entry<S>[], sub: (one: Entry<S>) => string) => (
    <Shelf name={name} grid>
      {list.map((one) => (
        <Item
          key={one.id}
          label={one.name}
          sub={sub(one)}
          title={one.note}
          on={holds(value, one.set)}
          onPick={() => change.current(one.set)}
          art={
            <Art
              px={ART}
              paint={(ctx, w, h, dpr) => sketch(unit!, one.set, ctx, w, h, dpr, paints(ctx.canvas, value.tint))}
              deps={[unit, value.tint, tick]}
            />
          }
        />
      ))}
    </Shelf>
  );
  return (
    <>
      {unit && (
        <Library>
          {shelf("Spirals", SPIRALS, (one) => one.note)}
          {shelf("Prime polynomials", POLYS, (one) => formula(one.set.a, one.set.b, one.set.c))}
        </Library>
      )}
      {unit && <Scene make={scene} value={value} onReady={ready} onClick={tap} aria-label={LABEL} />}
      {!unit && (
        <Bar side="status">
          <span>loading</span>
        </Bar>
      )}
    </>
  );
}
