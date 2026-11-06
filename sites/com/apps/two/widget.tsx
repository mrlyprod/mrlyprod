import { paints } from "@mrlyprod/ui/kit/color.ts";
import type { Mrly } from "@mrlyprod/ui/kit/designs/engine.ts";
import { CRATES } from "@mrlyprod/ui/kit/designs/engine.ts";
import { useTheme } from "@mrlyprod/ui/kit/designs/pick.tsx";
import { Bar } from "@mrlyprod/ui/kit/frame.tsx";
import type { Readout } from "@mrlyprod/ui/kit/hooks.ts";
import { useCrates } from "@mrlyprod/ui/kit/hooks.ts";
import { Art, Item, Library, Shelf } from "@mrlyprod/ui/kit/library.tsx";
import type { Handle, WidgetProps } from "@mrlyprod/ui/kit/page.tsx";
import { Scene } from "@mrlyprod/ui/kit/page.tsx";
import type { Opts, View } from "@mrlyprod/ui/kit/run.ts";
import type { Patch, Seeded, Value } from "@mrlyprod/ui/kit/spec.ts";
import { useCallback, useRef, useState } from "react";
import type { Counts } from "../shared/two/engine.ts";
import { read, tag } from "../shared/two/engine.ts";
import type { Pane } from "../shared/two/parts.tsx";
import { design, Folds, knobs, useShots, useStatus } from "../shared/two/parts.tsx";
import type { Two } from "./scene.ts";
import { name as called, make, SHEETS, SPEC, sketch } from "./scene.ts";

export type SheetsProps = { value: Value; onChange: (patch: Patch) => void; unit: Mrly };

type Props = WidgetProps<Mrly, Two> & { value: Seeded };

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

export function Widget({ value, onChange, onReady, onExport, onStatus, units }: Props) {
  const unit = useCrates(CRATES, units);
  const live = useRef<Handle<Two> | null>(null);
  const now = useRef(value);
  now.current = value;
  const [have, setHave] = useState(false);
  useStatus(live, `${have}:${JSON.stringify(value)}`, onStatus, rows);
  useShots(onExport, live, have, () => called(now.current));
  const scene = useCallback(
    (canvas: HTMLCanvasElement, view: View, opts: Opts) => make(canvas, view, { ...opts, mrly: unit }),
    [unit],
  );
  const ready = (handle: Handle<Two> | null) => {
    live.current = handle;
    setHave(Boolean(handle));
    onReady?.(handle);
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
