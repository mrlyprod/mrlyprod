import { useCallback, useEffect, useRef, useState } from "react";
import { text } from "../../lib/app/export.ts";
import type { Readout } from "../../lib/app/hooks.ts";
import { useReroll } from "../../lib/app/hooks.ts";
import { Art, Item, Library, Shelf } from "../../lib/app/library.tsx";
import type { Handle, Mathy, WidgetProps } from "../../lib/app/page.tsx";
import { Scene } from "../../lib/app/page.tsx";
import type { Opts, View } from "../../lib/app/run.ts";
import type { Patch, Value } from "../../lib/app/spec.ts";
import { paints } from "../../lib/color.ts";
import type { Mrly } from "../../lib/designs/engine.ts";
import { read, useTheme } from "../../lib/designs/pick.tsx";
import type { Facts, Knobs, Look, Preset } from "./engine.ts";
import { copies, holds, PRESETS, reroll, sample, titled, VIEWS } from "./engine.ts";
import type { Hands, Held, Sheet } from "./scene.ts";
import { make, name, sketch } from "./scene.ts";

/* TYPES */

type Props = Partial<WidgetProps<Mathy, Held>> & Pick<WidgetProps<Mathy, Held>, "value" | "onChange">;

type SheetsProps = { math: Mrly; value: Value; tick: number; onChange: (patch: Patch) => void };

const ORDER = VIEWS.map(([key]) => key);

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
  useReroll(value.seed as number, (seed) => unit && onChange({ code: reroll(unit, read(value).base, seed) }));
  useEffect(() => onStatus?.(rows(facts)), [facts]);
  const scene = useCallback(
    (canvas: HTMLCanvasElement, view: View, opts: Opts) =>
      make(canvas, view, { ...opts, math: unit, onFacts: setFacts }),
    [unit],
  );
  const hands: Hands = {
    crop: () => change.current({ overhang: now.current.overhang === 1 ? 0 : 1 }),
    view: (step) =>
      change.current({ view: ORDER[(ORDER.indexOf(now.current.view as Look) + step + ORDER.length) % ORDER.length] }),
    deeper: (step) => change.current({ level: (now.current.level as number) + step }),
  };
  const ready = (handle: Handle<Sheet> | null) => {
    live.current = handle;
    onReady?.(handle && { ...handle, ...hands });
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
          onClick={() => hands.view(1)}
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
