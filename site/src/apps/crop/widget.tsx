import type { PointerEvent } from "react";
import { useCallback, useRef, useState } from "react";
import { Bar } from "../../lib/app/frame.tsx";
import type { Readout } from "../../lib/app/hooks.ts";
import { useUnit } from "../../lib/app/hooks.ts";
import { Art, Item, Library, Shelf } from "../../lib/app/library.tsx";
import type { Handle, Mathy, WidgetProps } from "../../lib/app/page.tsx";
import { Scene } from "../../lib/app/page.tsx";
import type { Opts, View } from "../../lib/app/run.ts";
import type { Patch, Value } from "../../lib/app/spec.ts";
import type { Mrly } from "../../lib/designs/engine.ts";
import { useTheme } from "../../lib/designs/pick.tsx";
import { read, tag } from "../../lib/two/engine.ts";
import type { Pane } from "../../lib/two/parts.tsx";
import { design, Folds, useReroll, useShots, useStatus } from "../../lib/two/parts.tsx";
import { RMAX, SHAPES, top } from "./engine.ts";
import type { Crop, Facts, Handled, Hands } from "./scene.ts";
import { name as called, make, SPEC, sketch, WINDOWS } from "./scene.ts";

/* TYPES */

export type CropsProps = { value: Value; onChange: (patch: Patch) => void; unit: Mrly };

type Props = WidgetProps<Mathy, Handled>;

/* ROWS */

export function rows(f: Facts): Readout[] {
  const crop: Readout[] = [
    ["design", tag(f)],
    ["in", f.in],
    ["cut", f.cut],
    ["out", f.out],
    ["exposed before", f.before],
    ["exposed after", f.after],
    ["side", f.side],
    ["policy", f.note],
    ["level", f.level],
  ];
  const c = f.circle;
  if (!c) return crop;
  return [
    ["N(r)", c.seen],
    ["inside(r)", c.inside],
    ["C(r)", c.cut],
    ["count side", c.side],
    ...crop,
    ["r", c.r],
    ["d", c.d.toFixed(7)],
    ["fill of one tile", c.mass],
  ];
}

/* PANES */

export const panes = (value: Value, onChange: (patch: Patch) => void, unit: Mrly | null | undefined): Pane[] => [
  design(value, onChange, unit, SPEC),
];

/* CROPS */

const wears = (value: Value, patch: Value) => Object.entries(patch).every(([key, v]) => value[key] === v);

export function Crops({ value, onChange, unit }: CropsProps) {
  const tick = useTheme();
  const [find, setFind] = useState("");
  const { base, code } = read(value);
  const want = find.trim().toLowerCase();
  const kept = <T,>(list: [string, T][]) => list.filter(([label]) => label.toLowerCase().includes(want));
  const shapes = kept(SHAPES.map(([shape, label]): [string, { shape: string }] => [label, { shape }]));
  const windows = kept(WINDOWS);
  const art = (patch: Value, deps: readonly unknown[]) => (
    <Art
      deps={[unit, base, code, value.number, value.tint, tick, ...deps]}
      paint={(ctx, w, _h, dpr) => sketch(ctx, unit, { ...value, ...patch }, w, dpr)}
    />
  );
  return (
    <Library name="Crops" find={find} onFind={setFind} hint="Search crops">
      {shapes.length > 0 && (
        <Shelf name="Shapes" grid>
          {shapes.map(([label, patch]) => (
            <Item
              key={label}
              label={label}
              on={wears(value, patch)}
              onPick={() => onChange(patch)}
              art={art(patch, [patch.shape, value.radius, value.mode, value.policy, value.edge])}
            />
          ))}
        </Shelf>
      )}
      {windows.length > 0 && (
        <Shelf name="Windows" grid>
          {windows.map(([label, patch]) => (
            <Item
              key={label}
              label={label}
              on={wears(value, patch)}
              onPick={() => onChange(patch)}
              art={art(patch, Object.values(patch))}
            />
          ))}
        </Shelf>
      )}
    </Library>
  );
}

/* WIDGET */

export function Widget({ value, onChange, onReady, onExport, onStatus, units }: Props) {
  const unit = useUnit("math", units?.math);
  const live = useRef<Handle<Crop> | null>(null);
  const now = useRef(value);
  now.current = value;
  const grip = useRef(false);
  const [have, setHave] = useState(false);
  useReroll(value, onChange, unit);
  useStatus(live, `${have}:${JSON.stringify(value)}`, onStatus, rows);
  useShots(onExport, live, have, () => called(now.current));
  const scene = useCallback(
    (canvas: HTMLCanvasElement, view: View, opts: Opts) => make(canvas, view, { ...opts, math: unit }),
    [unit],
  );
  const ready = (handle: Handle<Crop> | null) => {
    live.current = handle;
    setHave(Boolean(handle));
    if (!handle) return onReady?.(null);
    const at = () => now.current;
    const hands: Hands = {
      flip: () => onChange({ mode: at().mode === "anti" ? "crop" : "anti" }),
      grow: (by) => onChange({ radius: Math.max(0, Math.min(RMAX, Number(at().radius) + by)) }),
      deeper: (by) =>
        onChange({ level: Math.max(1, Math.min(top(unit!, Number(at().number)), Number(at().level) + by)) }),
    };
    onReady?.({ ...handle, ...hands });
  };
  const point = (e: PointerEvent<HTMLCanvasElement>): [number, number] => {
    const canvas = e.currentTarget;
    const box = canvas.getBoundingClientRect();
    return [
      ((e.clientX - box.left) * canvas.width) / (box.width || 1),
      ((e.clientY - box.top) * canvas.height) / (box.height || 1),
    ];
  };
  const move = (e: PointerEvent<HTMLCanvasElement>) => {
    if (!grip.current) return;
    const patch = live.current?.drag?.(...point(e));
    if (patch && patch.radius !== now.current.radius) onChange(patch);
  };
  const down = (e: PointerEvent<HTMLCanvasElement>) => {
    if (!live.current?.drag) return;
    grip.current = true;
    e.currentTarget.setPointerCapture?.(e.pointerId);
    move(e);
  };
  const up = () => {
    grip.current = false;
  };
  return (
    <>
      {unit && (
        <Scene
          make={scene}
          value={value}
          onReady={ready}
          onPointerDown={down}
          onPointerMove={move}
          onPointerUp={up}
          onPointerCancel={up}
        />
      )}
      {unit && <Crops value={value} onChange={onChange} unit={unit} />}
      <Bar side="right">
        <Folds groups={panes(value, onChange, unit)} />
      </Bar>
    </>
  );
}
