import { paints } from "@mrlyprod/ui/kit/color.ts";
import { useTheme } from "@mrlyprod/ui/kit/designs/pick.tsx";
import { text } from "@mrlyprod/ui/kit/export.ts";
import type { Readout } from "@mrlyprod/ui/kit/hooks.ts";
import { Art, Item, Library, Shelf } from "@mrlyprod/ui/kit/library.tsx";
import type { Handle, WidgetProps } from "@mrlyprod/ui/kit/page.tsx";
import { Scene } from "@mrlyprod/ui/kit/page.tsx";
import type { Opts, View } from "@mrlyprod/ui/kit/run.ts";
import type { Patch, Seeded, Value } from "@mrlyprod/ui/kit/spec.ts";
import { useCallback, useEffect, useRef, useState } from "react";
import type { Bar, Doors, Facts, FactsOf, Kind, Reading } from "./engine.ts";
import { CUT, FEATURED, holds, inks, place, READINGS, said, stepped, thumb } from "./engine.ts";
import type { Handled, Hands, Stage } from "./scene.ts";
import { file, make } from "./scene.ts";

export type DesignsProps = { units: Doors; value: Value; tick: number; onChange: (patch: Patch) => void };

type Props = WidgetProps<Doors, Handled> & { value: Seeded };

const STEP = 900;

const ART = 64;

const LABEL =
  "A design read along a polynomial or the primes: the stream as a raster, its battery, or its bias beside the major arcs";

const big = (n: number) => `2^${Math.round(Math.log2(n))} = ${n}`;

const fixed = (x: number, places = 2) => x.toFixed(places);

const zed = (one: Bar | null) =>
  one
    ? `z ${fixed(one.z)} at ${one.label}${one.head ? (one.head.exact ? ", an exact near-head" : ", a near-head with a carry") : ""}`
    : "none";

const head = (f: Facts): Readout[] => [
  ["stream", `${f.title} along ${f.formula}`],
  ["N", big(f.count)],
];

const biased = (f: Facts): Readout => ["bias", `S(N) = ${f.bias}, z ${fixed(f.z)}`];

export const ROWS: { [K in Kind]: (f: FactsOf<K>) => Readout[] } = {
  plaid: (f) => [
    ...head(f),
    biased(f),
    ["row", `${f.width} wide, ${f.rows} rows`],
    ["plus", `${f.plus} of ${f.count}`],
    ["near-heads", said(f.heads)],
  ],
  battery: (f) => [
    ...head(f),
    ["verdict", f.verdict],
    ["shift", zed(f.largest.shift)],
    ["dilation", zed(f.largest.dilation)],
    ["kernel", zed(f.largest.kernel)],
    ["square", f.largest.square ? `z ${fixed(f.largest.square.z)}` : "none"],
    ["lit", f.lit.length ? f.lit.map((one) => `${one.label} ${fixed(one.z)}`).join(", ") : "nothing above 3"],
    ["near-heads", said(f.heads)],
  ],
  arcs: (f) => [
    ...head(f),
    [
      "residual",
      f.residual === null
        ? f.pending
          ? "summing the arcs"
          : "none"
        : `${fixed(f.residual, 1)}, z ${fixed(f.residual / Math.sqrt(f.count))}`,
    ],
    biased(f),
    [
      "major arcs",
      f.predicted === null
        ? f.pending
          ? "summing"
          : "none"
        : `${fixed(f.predicted, 1)}, z ${fixed(f.predicted / Math.sqrt(f.count))}`,
    ],
    [
      "arcs",
      f.arcs
        ? f.arcs.length
          ? f.arcs.map((one) => `${one.num}/${one.den} at eta ${one.eta.toFixed(4)}`).join(", ")
          : `none below ${CUT}`
        : "-",
    ],
    ["near-heads", said(f.heads)],
  ],
};

export const tell = <K extends Kind>(f: FactsOf<K>) => ROWS[f.view as K](f);

export function Designs({ units, value, tick, onChange }: DesignsProps) {
  const reading = value.reading as Reading;
  return (
    <>
      <Shelf name="Designs" grid>
        {FEATURED.map((one) => (
          <Item
            key={one.label}
            label={one.label}
            on={holds(value, one.pick)}
            onPick={() => onChange(place(one.pick))}
            art={
              <Art
                px={ART}
                paint={(ctx, w, h) => thumb(units.readings, one.pick, reading, ctx, w, h, inks(paints(ctx.canvas)))}
                deps={[tick, reading, units]}
              />
            }
          />
        ))}
      </Shelf>
      <Shelf name="Readings">
        {READINGS.map(([key, name]) => (
          <Item
            key={key}
            label={name}
            sub={key === "Primes" ? "the consecutive primes" : `y(n) = x(${name})`}
            on={reading === key}
            onPick={() => onChange({ reading: key })}
          />
        ))}
      </Shelf>
    </>
  );
}

export function Widget({ value, onChange, onReady, onExport, onStatus, units }: Props) {
  const tick = useTheme();
  const [facts, setFacts] = useState<Facts | null>(null);
  const live = useRef<Handle<Stage> | null>(null);
  const now = useRef(value);
  now.current = value;
  const hooks = useRef(onChange);
  hooks.current = onChange;
  const timer = useRef<ReturnType<typeof setInterval> | 0>(0);
  const wrote = useRef<number | null>(null);
  const play = (on: boolean) => {
    clearInterval(timer.current);
    timer.current = 0;
    if (!on) return;
    wrote.current = Number(now.current.power);
    timer.current = setInterval(() => {
      const patch = stepped(now.current, 1);
      wrote.current = patch.power;
      hooks.current(patch);
    }, STEP);
  };
  const keep = (kind: "battery" | "bias") => {
    const body = live.current?.csv?.(kind);
    if (body) text(body, `${file(now.current, units)}-${kind}`, "csv");
  };
  const rows = facts ? tell(facts) : [];
  useEffect(() => onStatus?.(rows), [JSON.stringify(rows)]);
  useEffect(
    () =>
      onExport?.(
        facts
          ? [
              ["Battery CSV", () => keep("battery")],
              ["Bias CSV", () => keep("bias")],
            ]
          : [],
      ),
    [Boolean(facts)],
  );
  useEffect(() => () => play(false), []);
  useEffect(() => {
    if (timer.current && Number(value.power) !== wrote.current) play(false);
  }, [value.power]);
  const scene = useCallback(
    (canvas: HTMLCanvasElement, view: View, opts: Opts) => make(canvas, view, { ...opts, ...units, onFacts: setFacts }),
    [units],
  );
  const ready = (handle: Handle<Stage> | null) => {
    live.current = handle;
    setFacts(handle?.facts() ?? null);
    if (!handle) return onReady?.(null);
    const hands: Hands = {
      toggle: () => play(!timer.current),
      playing: () => Boolean(timer.current),
    };
    onReady?.({ ...handle, ...hands });
  };
  const tap = () => hooks.current(stepped(now.current, 1));
  return (
    <>
      {units && <Scene make={scene} value={value} onReady={ready} onClick={tap} role="img" aria-label={LABEL} />}
      {units && (
        <Library name="Readings">
          <Designs units={units} value={value} tick={tick} onChange={onChange} />
        </Library>
      )}
    </>
  );
}
