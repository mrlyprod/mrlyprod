import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import type { Had, Readout } from "../../lib/app/hooks.ts";
import type { Chip } from "../../lib/app/library.tsx";
import { Art, Item, Library, Shelf } from "../../lib/app/library.tsx";
import type { Handle, WidgetProps } from "../../lib/app/page.tsx";
import { Scene } from "../../lib/app/page.tsx";
import type { Opts, View } from "../../lib/app/run.ts";
import type { Patch, Scalar } from "../../lib/app/spec.ts";
import { useTouch } from "../../lib/app/touch.ts";
import { paints } from "../../lib/color.ts";
import { useTheme } from "../../lib/designs/pick.tsx";
import { flat } from "../../lib/designs/thumb.ts";
import { carpets, cell, hits, inks, ladder, rungOf, swatch } from "./engine.ts";
import type { Chladni, Knobs, Stat, Step } from "./scene.ts";
import { make } from "./scene.ts";

/* TYPES */

export type Held = Had<"num" | "math">;

export type ShelvesProps = { value: Knobs; onChange: (patch: Patch) => void; units?: Held | null };

type Props = WidgetProps<Held, Chladni> & { value: Knobs };

const SIGNS: Record<number, string> = { 1: "+", "-1": "-" };
const KINDS: Chip[] = [
  ["plate", "Plates"],
  ["carpet", "Carpets"],
];

export function rows(stat: Stat | null | undefined): Readout[] {
  if (!stat) return [];
  if (stat.error) return [["error", stat.error]];
  if (stat.view === "carpet") {
    return [
      ["k", `${stat.k} of ${stat.count}`],
      ["lambda", stat.lambda.toFixed(6)],
      ["domains", `${stat.domains} of ${stat.bound}`],
      ["grains left", String(stat.grains)],
    ];
  }
  return [
    ["figure", `(${[stat.n, stat.m, SIGNS[stat.sign!]].filter((one) => one !== undefined).join(", ")})`],
    ["pitch", stat.pitch === null ? "none" : stat.pitch!.toFixed(3)],
    ["grains", String(stat.grains)],
  ];
}

export function Shelves({ value, onChange, units }: ShelvesProps) {
  const [kind, setKind] = useState<Scalar>(value.view);
  const [find, setFind] = useState("");
  const tick = useTheme();
  useEffect(() => setKind(value.view), [value.view]);
  const rungs = useMemo(() => (units?.num ? ladder(units.num) : []), [units]);
  const designs = useMemo(() => (units?.math ? carpets(units.math) : []), [units]);
  const at = value.view === "plate" && units?.num ? rungOf(units.num, value) : -1;
  const dye = (ctx: CanvasRenderingContext2D) => inks(paints(ctx.canvas, value.tint));
  return (
    <Library chips={KINDS} chip={kind} onChip={setKind} find={find} onFind={kind === "plate" ? setFind : undefined}>
      {kind === "plate" && (
        <Shelf name="Plates" grid>
          {rungs.map(
            (one, i) =>
              hits(one, find) && (
                <Item
                  key={`${one.n},${one.m},${one.sign}`}
                  label={`${one.n}, ${one.m}`}
                  sub={one.n === one.m ? "" : SIGNS[one.sign]}
                  on={i === at}
                  onPick={() => onChange({ view: "plate", n: one.n, m: one.m, sign: one.sign })}
                  art={
                    <Art
                      px={64}
                      paint={(ctx, w) =>
                        ctx.putImageData(new ImageData(swatch(units!.num, one, dye(ctx), w), w, w), 0, 0)
                      }
                      deps={[i, value.tint, tick]}
                    />
                  }
                />
              ),
          )}
        </Shelf>
      )}
      {kind === "carpet" && (
        <Shelf name="Carpets" grid>
          {designs.map((one) => (
            <Item
              key={`${one.number}-${one.code}`}
              label={one.name}
              sub={`no. ${one.number}`}
              on={value.view === "carpet" && value.number === one.number && value.code === one.code}
              onPick={() => onChange({ view: "carpet", number: one.number, code: one.code, level: one.level })}
              art={
                <Art
                  px={64}
                  paint={(ctx, w) => flat(ctx, cell(units!.math, one), 0, 0, w, paints(ctx.canvas, value.tint).accent)}
                  deps={[one.number, one.code, value.tint, tick]}
                />
              }
            />
          ))}
        </Shelf>
      )}
    </Library>
  );
}

export function Widget({ value, onChange, onReady, onStatus, units }: Props) {
  const live = useRef<Handle<Chladni> | null>(null);
  const change = useRef(onChange);
  const status = useRef(onStatus);
  change.current = onChange;
  status.current = onStatus;
  const tell = useCallback((stat: Stat) => status.current?.(rows(stat)), []);
  const push = useCallback((patch: Step) => change.current?.(patch), []);
  const scene = useCallback(
    (canvas: HTMLCanvasElement, view: View, opts: Opts) =>
      make(canvas, view, { ...opts, ...units, onTell: tell, onWalk: push }),
    [units, tell, push],
  );
  const ready = (handle: Handle<Chladni> | null) => {
    live.current = handle;
    if (!handle) status.current?.([]);
    onReady?.(handle);
  };
  const touch = useTouch(live, change);
  if (!units?.num) return null;
  return (
    <>
      <Scene
        make={scene}
        value={value}
        onReady={ready}
        {...touch}
        role="img"
        aria-label="Sand on a Chladni plate"
        style={{ cursor: "crosshair" }}
      />
      <Shelves value={value} onChange={onChange} units={units} />
    </>
  );
}
