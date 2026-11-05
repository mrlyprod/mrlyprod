import type { Mrly } from "@mrlyprod/ui/kit/designs/engine.ts";
import { useTheme } from "@mrlyprod/ui/kit/designs/pick.tsx";
import { Bar } from "@mrlyprod/ui/kit/frame.tsx";
import type { Readout } from "@mrlyprod/ui/kit/hooks.ts";
import { useUnit } from "@mrlyprod/ui/kit/hooks.ts";
import { Art, Item, Library, Shelf } from "@mrlyprod/ui/kit/library.tsx";
import type { Handle, Mathy, WidgetProps } from "@mrlyprod/ui/kit/page.tsx";
import { Scene } from "@mrlyprod/ui/kit/page.tsx";
import type { Opts, View } from "@mrlyprod/ui/kit/run.ts";
import type { Patch, Seeded, Value } from "@mrlyprod/ui/kit/spec.ts";
import type { PointerEvent } from "react";
import { useCallback, useRef, useState } from "react";
import { read, tag } from "../shared/two/engine.ts";
import type { Pane } from "../shared/two/parts.tsx";
import { design, Folds, useShots, useStatus } from "../shared/two/parts.tsx";
import { SHAPES } from "./engine.ts";
import type { Crop, Facts } from "./scene.ts";
import { name as called, make, SPEC, sketch, WINDOWS } from "./scene.ts";

export type CropsProps = { value: Value; onChange: (patch: Patch) => void; unit: Mrly };

type Props = WidgetProps<Mathy, Crop> & { value: Seeded };

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

export const panes = (value: Value, onChange: (patch: Patch) => void, unit: Mrly | null | undefined): Pane[] => [
  design(value, onChange, unit, SPEC),
];

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

export function Widget({ value, onChange, onReady, onExport, onStatus, units }: Props) {
  const unit = useUnit("math", units?.math);
  const live = useRef<Handle<Crop> | null>(null);
  const now = useRef(value);
  now.current = value;
  const grip = useRef(false);
  const [have, setHave] = useState(false);
  useStatus(live, `${have}:${JSON.stringify(value)}`, onStatus, rows);
  useShots(onExport, live, have, () => called(now.current));
  const scene = useCallback(
    (canvas: HTMLCanvasElement, view: View, opts: Opts) => make(canvas, view, { ...opts, math: unit }),
    [unit],
  );
  const ready = (handle: Handle<Crop> | null) => {
    live.current = handle;
    setHave(Boolean(handle));
    onReady?.(handle);
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
