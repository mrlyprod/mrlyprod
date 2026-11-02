import type { PointerEvent } from "react";
import { useCallback, useEffect, useRef, useState } from "react";
import { text } from "../../net/sdk/export.ts";
import type { Readout } from "../../net/sdk/hooks.ts";
import { useStatus } from "../../net/sdk/hooks.ts";
import type { Paint } from "../../net/sdk/library.tsx";
import { Art, Item, Library, Shelf } from "../../net/sdk/library.tsx";
import type { Handle, WidgetProps } from "../../net/sdk/page.tsx";
import { Scene } from "../../net/sdk/page.tsx";
import type { Opts, View } from "../../net/sdk/run.ts";
import type { Patch, Seeded, Value } from "../../net/sdk/spec.ts";
import { useTheme } from "../shared/designs/pick.tsx";
import type { Doors, Source } from "./engine.ts";
import { games, HEXES, MOSAICS, study, VIEWS } from "./engine.ts";
import type { Facts, Stage } from "./scene.ts";
import { file, make, sketch, tidied } from "./scene.ts";

/* TYPES */

export type ShelvesProps = { units: Doors; value: Value; tick: number; onChange: (patch: Patch) => void };

type Props = WidgetProps<Doors, Stage> & { value: Seeded };

type Entry = [string, Patch, string?];

/* ROWS */

const places = (x: number, n = 4) => x.toFixed(n);

export function rows(facts: Facts | null, source: Source | null): Readout[] {
  if (!facts || !source) return [];
  const out: Readout[] = [];
  if (source.error) out.push(["note", source.error]);
  out.push(
    ["draws", facts.n],
    ["distance", `${places(facts.distance)} from ${facts.law}`],
    ["painted", `${facts.painted} of ${facts.total} cells at level ${facts.k}`],
  );
  if (facts.mode === "bruijn")
    out.push([
      "laps",
      `${facts.laps} of ${facts.lap} draws, ${facts.perfect ? "every cell painted exactly once per lap" : "a cell painted twice in a lap"}`,
    ]);
  if (facts.checked)
    out.push([
      "register",
      `${facts.agree} of ${facts.checked} cells agree with the last ${facts.k} draws${facts.agree === facts.checked ? "" : ", the rest a rounding at a cell edge"}`,
    ]);
  if (facts.outside) out.push(["outside the law", `${facts.outside} points in cells of law zero`]);
  if (facts.stuck) out.push(["stuck", "no draw is allowed from here"]);
  if (facts.lost) out.push(["law", "the chain loses its mass, so no law is read"]);
  return [...out, ...source.facts];
}

/* LIBRARY */

const SHELVES: [string, Entry[]][] = [
  ["Sources", VIEWS.map(([view, name]) => [name, { view }])],
  ["Hex bases", HEXES.map(([hex, name]) => [name, { view: "hex", hex }])],
  [
    "Mosaics",
    MOSAICS.map((one) => [
      one.name,
      { view: "mosaic", mosaic: one.key },
      `base ${one.base}, ${one.masks.length} slots`,
    ]),
  ],
];

const shelves = (units: Doors): [string, Entry[]][] => [
  ...SHELVES,
  [
    "Rules",
    games(units.math)
      .filter((one) => one.key !== "typed")
      .map((one) => [one.name, { view: "rule", game: one.key }, `width ${one.width}`]),
  ],
];

const shows = (value: Value, patch: Patch) => Object.entries(patch).every(([key, v]) => value[key] === v);

export function Shelves({ units, value, tick, onChange }: ShelvesProps) {
  const paint =
    (patch: Patch): Paint =>
    (ctx, w, h, dpr) =>
      sketch(ctx, w, h, dpr, units, { ...value, ...patch });
  return shelves(units).map(([name, items]) => (
    <Shelf key={name} name={name}>
      {items.map(([label, patch, sub]) => (
        <Item
          key={label}
          label={label}
          sub={sub}
          on={shows(value, patch)}
          onPick={() => onChange(patch)}
          art={
            <Art
              paint={paint(patch)}
              deps={[tick, value.base, value.code, value.side, value.cut, value.word, value.letters]}
            />
          }
        />
      ))}
    </Shelf>
  ));
}

/* WIDGET */

export function Widget({ value, onChange, onReady, onExport, onStatus, units }: Props) {
  const tick = useTheme();
  const live = useRef<Handle<Stage> | null>(null);
  const now = useRef(value);
  now.current = value;
  const hooks = useRef(onChange);
  hooks.current = onChange;
  const [facts, setFacts] = useState<Facts | null>(null);
  const source = units ? study(units, tidied(value, units)) : null;
  useStatus(onStatus, rows(facts, source));
  useEffect(() => {
    const sheet = () => {
      const made = live.current?.sheet?.();
      if (made) text(made, file(now.current), "csv");
    };
    onExport?.([["CSV", sheet]]);
  }, []);
  const scene = useCallback(
    (canvas: HTMLCanvasElement, view: View, opts: Opts) => make(canvas, view, { ...opts, ...units, onFacts: setFacts }),
    [units],
  );
  const ready = (handle: Handle<Stage> | null) => {
    live.current = handle;
    onReady?.(handle);
  };
  const tap = (e: PointerEvent<HTMLCanvasElement>) => {
    const got = live.current;
    if (!got?.tap) return;
    const box = e.currentTarget.getBoundingClientRect();
    const k = e.currentTarget.width / (box.width || 1);
    got.tap((e.clientX - box.left) * k, (e.clientY - box.top) * k);
  };
  return (
    <>
      {units && (
        <Scene
          make={scene}
          value={value}
          onReady={ready}
          onPointerUp={tap}
          role="img"
          aria-label={source ? `The chaos game of ${source.title}` : "Chaos"}
        />
      )}
      {units && (
        <Library name="Games">
          <Shelves units={units} value={value} tick={tick} onChange={onChange} />
        </Library>
      )}
    </>
  );
}
