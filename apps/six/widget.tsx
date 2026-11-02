import { useCallback, useEffect, useRef, useState } from "react";
import { paints } from "../../net/sdk/color.ts";
import { text } from "../../net/sdk/export.ts";
import type { Readout } from "../../net/sdk/hooks.ts";
import { Art, Item, Library, Shelf } from "../../net/sdk/library.tsx";
import type { Handle, Mathy, WidgetProps } from "../../net/sdk/page.tsx";
import { Scene } from "../../net/sdk/page.tsx";
import type { Opts, View } from "../../net/sdk/run.ts";
import type { Patch, Seeded, Value } from "../../net/sdk/spec.ts";
import type { Mrly } from "../shared/designs/engine.ts";
import { read, useTheme } from "../shared/designs/pick.tsx";
import type { Facts, Knobs, Preset } from "./engine.ts";
import { copies, holds, PRESETS, sample, titled } from "./engine.ts";
import type { Sheet } from "./scene.ts";
import { make, name, sketch, turn } from "./scene.ts";

/* TYPES */

type Props = Partial<WidgetProps<Mathy, Sheet>> & Pick<WidgetProps<Mathy, Sheet>, "onChange"> & { value: Seeded };

type SheetsProps = { math: Mrly; value: Value; tick: number; onChange: (patch: Patch) => void };

const by = ([w, h]: readonly number[]) => `${w} x ${h}`;

export function rows(f: Facts | null): Readout[] {
  if (!f) return [];
  return (
    [
      ["design", f.name ? `${f.name} ${f.code}` : f.title],
      ["side", f.side],
      ["level", `${f.level} of ${f.top}`],
      ["copies", f.copies],
      ["filled", f.fills],
      ["exposed", f.exposed],
      ["one copy exposed", f.alone],
      ["tile", by(f.tile)],
      ["sheet", by(f.sheet)],
      ["triangles", f.triangles],
      ["empty", f.voids],
      ["pieces", f.pieces],
      ["holes", f.holes],
      ["corners", f.corners],
      ["edges", f.edges],
      ["euler", f.euler],
    ] satisfies Readout[]
  ).map(([name, v]): Readout => [name, String(v)]);
}

export const label = (f: Pick<Facts, "name" | "title"> | null) =>
  `${f ? f.name || f.title : "a cube design"} repeated into a sheet`;

export function Sheets({ math, value, tick, onChange }: SheetsProps) {
  const paint = (one: Preset) => (ctx: CanvasRenderingContext2D, w: number, h: number) =>
    sketch(
      ctx,
      w,
      h,
      sample(math, { ...(value as Knobs), ...read(value), ...one }),
      paints(ctx.canvas, value.tint as string),
    );
  return (
    <Shelf grid>
      {PRESETS.map((one) => (
        <Item
          key={titled(one)}
          label={titled(one)}
          sub={`${copies(math, { ...(value as Knobs), ...one })} copies`}
          on={holds(value, one)}
          onPick={() => onChange(one)}
          art={
            <Art
              paint={paint(one)}
              deps={[
                tick,
                value.code,
                value.base,
                value.number,
                value.view,
                value.turn,
                value.invert,
                value.overhang,
                value.tint,
              ]}
            />
          }
        />
      ))}
    </Shelf>
  );
}

export function Widget({ value, onChange, onReady, onExport, onStatus, units }: Props) {
  const unit = units?.math;
  const tick = useTheme();
  const [facts, setFacts] = useState<Facts | null>(null);
  const live = useRef<Handle<Sheet> | null>(null);
  const now = useRef(value);
  const change = useRef(onChange);
  now.current = value;
  change.current = onChange;
  useEffect(() => onStatus?.(rows(facts)), [facts]);
  const scene = useCallback(
    (canvas: HTMLCanvasElement, view: View, opts: Opts) =>
      make(canvas, view, { ...opts, math: unit, onFacts: setFacts }),
    [unit],
  );
  const ready = (handle: Handle<Sheet> | null) => {
    live.current = handle;
    onReady?.(handle);
  };
  const keep = (kind: "svg" | "tile", tail: string) => () => {
    const picture = live.current?.[kind]?.();
    if (picture) text(picture, `${name(now.current, units)}${tail}`, "svg");
  };
  const have = Boolean(unit);
  useEffect(
    () =>
      onExport?.(
        have
          ? [
              ["SVG", keep("svg", "")],
              ["SVG tile", keep("tile", "-tile")],
            ]
          : [],
      ),
    [have],
  );
  return (
    <>
      {unit && (
        <Scene
          make={scene}
          value={value}
          onReady={ready}
          role="img"
          aria-label={label(facts)}
          onClick={() => change.current(turn(now.current, 1))}
        />
      )}
      {unit && (
        <Library name="Sheets">
          <Sheets math={unit} value={value} tick={tick} onChange={onChange} />
        </Library>
      )}
    </>
  );
}
