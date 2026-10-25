import { useCallback, useRef, useState } from "react";
import { Bar } from "../../lib/app/frame.tsx";
import type { Readout } from "../../lib/app/hooks.ts";
import { useUnit } from "../../lib/app/hooks.ts";
import { Art, Item, Library, Shelf } from "../../lib/app/library.tsx";
import type { Handle, Mathy, WidgetProps } from "../../lib/app/page.tsx";
import { Scene } from "../../lib/app/page.tsx";
import type { Opts, View } from "../../lib/app/run.ts";
import type { Patch, Value } from "../../lib/app/spec.ts";
import { paints } from "../../lib/color.ts";
import type { Mrly } from "../../lib/designs/engine.ts";
import { useTheme } from "../../lib/designs/pick.tsx";
import type { Counts } from "../../lib/two/engine.ts";
import { cap, read, tag } from "../../lib/two/engine.ts";
import type { Pane } from "../../lib/two/parts.tsx";
import { design, Folds, knobs, useReroll, useShots, useStatus } from "../../lib/two/parts.tsx";
import type { Handled, Hands, Two } from "./scene.ts";
import { name as called, make, SHEETS, SPEC, sketch } from "./scene.ts";

/* TYPES */

export type SheetsProps = { value: Value; onChange: (patch: Patch) => void; unit: Mrly };

type Props = WidgetProps<Mathy, Handled>;

/* ROWS */

export const rows = (f: Counts): Readout[] => [
  ["design", tag(f)],
  ["copies", f.copies],
  ["exposed", f.exposed],
  ["one copy exposed", f.one],
  ["fills", f.fills],
  ["voids", f.voids],
  ["tile", `${f.side} by ${f.side}`],
  ["sheet", `${f.cols} by ${f.rows}`],
  ["level", f.level],
  ["corners", f.corners],
  ["edges", f.edges],
  ["euler", f.euler],
  ["dimension", f.dimension.toFixed(4)],
];

/* PANES */

export function panes(value: Value, onChange: (patch: Patch) => void, unit: Mrly | null | undefined): Pane[] {
  return [
    design(value, onChange, unit, SPEC),
    {
      name: "Sheet",
      live: `${value.x} by ${value.y}`,
      body: knobs(["x", "y"], value, onChange, SPEC),
    },
  ];
}

/* SHEETS */

export function Sheets({ value, onChange, unit }: SheetsProps) {
  const tick = useTheme();
  const { base, code } = read(value);
  const deps = [unit, base, code, value.number, value.invert, value.tint, tick];
  return (
    <Library name="Sheets">
      <Shelf grid>
        {SHEETS.map(([label, x, y]) => (
          <Item
            key={label}
            label={label}
            sub={`${x} by ${y}`}
            on={value.x === x && value.y === y}
            onPick={() => onChange({ x, y })}
            art={
              <Art
                deps={[x, y, ...deps]}
                paint={(ctx, w) => sketch(ctx, unit, value, x, y, w, paints(ctx.canvas, String(value.tint)).accent)}
              />
            }
          />
        ))}
      </Shelf>
    </Library>
  );
}

/* WIDGET */

export function Widget({ value, onChange, onReady, onExport, onStatus, units }: Props) {
  const unit = useUnit("math", units?.math);
  const live = useRef<Handle<Two> | null>(null);
  const now = useRef(value);
  now.current = value;
  const [have, setHave] = useState(false);
  useReroll(value, onChange, unit);
  useStatus(live, `${have}:${JSON.stringify(value)}`, onStatus, rows);
  useShots(onExport, live, have, () => called(now.current));
  const scene = useCallback(
    (canvas: HTMLCanvasElement, view: View, opts: Opts) => make(canvas, view, { ...opts, math: unit }),
    [unit],
  );
  const ready = (handle: Handle<Two> | null) => {
    live.current = handle;
    setHave(Boolean(handle));
    if (!handle) return onReady?.(null);
    const at = () => now.current;
    const hands: Hands = {
      seams: () => onChange({ seams: at().seams ? 0 : 1 }),
      deeper: (by) =>
        onChange({
          level: Math.max(
            1,
            Math.min(cap(unit!, Number(at().number), Number(at().x) * Number(at().y)), Number(at().level) + by),
          ),
        }),
      wider: (by) => onChange({ x: Math.max(1, Math.min(12, Number(at().x) + by)) }),
    };
    onReady?.({ ...handle, ...hands });
  };
  return (
    <>
      {unit && <Scene make={scene} value={value} onReady={ready} />}
      {unit && <Sheets value={value} onChange={onChange} unit={unit} />}
      <Bar side="right">
        <Folds groups={panes(value, onChange, unit)} />
      </Bar>
    </>
  );
}
