import type { PointerEvent } from "react";
import { useCallback, useEffect, useRef, useState } from "react";
import { paints } from "../../net/sdk/color.ts";
import { Fold } from "../../net/sdk/controls.tsx";
import { text } from "../../net/sdk/export.ts";
import { Bar } from "../../net/sdk/frame.tsx";
import { gist, Knob } from "../../net/sdk/knobs.tsx";
import { Art, Item, Library, Shelf } from "../../net/sdk/library.tsx";
import type { Handle, WidgetProps } from "../../net/sdk/page.tsx";
import { Scene } from "../../net/sdk/page.tsx";
import type { Fix, Opts, View } from "../../net/sdk/run.ts";
import type { Patch, Units, Value } from "../../net/sdk/spec.ts";
import type { Mrly } from "../shared/designs/engine.ts";
import { useTheme } from "../shared/designs/pick.tsx";
import type { Glance, Roll } from "./engine.ts";
import { fit, glance, resolve, TRACKS } from "./engine.ts";
import type { Played, Spirograph, Stat } from "./scene.ts";
import { file, make, parked, SPEC, scrub, status } from "./scene.ts";

/* TYPES */

export type TracksProps = { value: Spirograph; math: Mrly | null | undefined; onChange: (patch: Patch) => void };

export type PaneProps = { value: Value; at: number; math: Mrly | null | undefined; onChange: (patch: Patch) => void };

export type Etcher = Pick<
  CanvasRenderingContext2D,
  "strokeStyle" | "lineWidth" | "lineJoin" | "beginPath" | "moveTo" | "lineTo" | "stroke"
> & { canvas: Element | null };

type Props = Partial<WidgetProps<Units, Played>> &
  Pick<WidgetProps<Units, Played>, "value" | "onChange"> & { value: Spirograph };

type Held = { x: number; at: number; moved: boolean };

export const FOLDS: [string, string[]][] = [
  ["Track", ["kind", "sides", "ring", "wheel", "laps"]],
  ["Draw", ["at", "ink", "fill"]],
  ["Wheel", ["number", "level", "mode", "reach", "jitter"]],
];

const SLOP = 6;

const LINE = 1;

const rowOf = (key: string) => SPEC.find((row) => row.key === key)!;

export const shown = (keys: string[], value: Value) => keys.map(rowOf).filter((row) => !row.need || row.need(value));

export const lit = (value: Value, patch: Value) =>
  Object.keys(patch)
    .filter((key) => rowOf(key).need?.(patch) ?? true)
    .every((key) => value[key] === patch[key]);

export const sub = ({ kind, ring, wheel, sides }: Roll) =>
  kind === "in" || kind === "out" ? `R/r = ${ring}/${wheel}` : kind === "line" ? "on a line" : `${sides} sides`;

export function etch(ctx: Etcher, { count, samples, trace, frame }: Glance, w: number, h: number, dpr: number) {
  const box = fit(frame, w, h, 4 * dpr);
  ctx.strokeStyle = paints(ctx.canvas).accent;
  ctx.lineWidth = LINE * dpr;
  ctx.lineJoin = "round";
  ctx.beginPath();
  for (let k = 0; k < count; k++) {
    for (let i = 0; i < samples; i++) {
      const j = (k * samples + i) * 2;
      const x = box.ox + trace[j] * box.k;
      const y = box.oy - trace[j + 1] * box.k;
      if (i) ctx.lineTo(x, y);
      else ctx.moveTo(x, y);
    }
  }
  ctx.stroke();
}

export function Tracks({ value, math, onChange }: TracksProps) {
  const tick = useTheme();
  const glimpse = (patch: Roll) => (
    <Art
      paint={(ctx, w, h, dpr) => etch(ctx, glance(math!, value, resolve(math!, value), patch), w, h, dpr)}
      deps={[
        math,
        patch.kind,
        patch.ring,
        patch.wheel,
        patch.sides,
        patch.laps,
        value.base,
        value.code,
        value.number,
        value.mode,
        value.reach,
        value.jitter,
        value.seed,
        tick,
      ]}
    />
  );
  return (
    <Shelf name="Tracks" grid>
      {TRACKS.map(([name, patch]) => (
        <Item
          key={name}
          label={name}
          sub={sub(patch)}
          on={lit(value, patch)}
          onPick={() => onChange(patch)}
          art={math ? glimpse(patch) : undefined}
        />
      ))}
    </Shelf>
  );
}

export function Pane({ value, at, math, onChange }: PaneProps) {
  const values: Value = { ...value, at };
  const units = math ? { math } : undefined;
  return (
    <Bar side="right">
      {FOLDS.map(([fold, keys]) => {
        const rows = shown(keys, value);
        return (
          <Fold key={fold} name={fold} live={gist(rows, values)}>
            {rows.map((row) => (
              <Knob key={row.key} row={row} value={values[row.key]} values={values} units={units} onChange={onChange} />
            ))}
          </Fold>
        );
      })}
    </Bar>
  );
}

export function Widget({ value, onChange, onReady, onExport, onStatus, units }: Props) {
  const unit = units?.math;
  const [stat, setStat] = useState<Stat | null>(null);
  const live = useRef<Handle<Played> | null>(null);
  const now = useRef(value);
  now.current = value;
  const change = useRef(onChange);
  change.current = onChange;
  const was = useRef<Stat | null>(null);
  const touch = useRef<Held | null>(null);
  const tell = useCallback((next: Stat) => {
    if (was.current?.playing && !next.playing) {
      const at = parked(next.part);
      if (at !== now.current.at) change.current({ at });
    }
    was.current = next;
    setStat(next);
  }, []);
  const scene = useCallback(
    (canvas: HTMLCanvasElement, view: View, opts: Opts) => make(canvas, view, { ...opts, math: unit, tell }),
    [unit, tell],
  );
  const ready = (handle: Handle<Played> | null) => {
    live.current = handle;
    const fix = (next?: Fix | null) => {
      if (next?.step) handle!.take();
      handle!.fix?.(next);
    };
    onReady?.(handle && { ...handle, fix });
  };
  const toggle = () => live.current?.toggle();
  const keep = () => {
    const picture = live.current?.svg?.();
    if (picture) text(picture, file(now.current, units), "svg");
  };
  const have = Boolean(stat);
  useEffect(() => onExport?.(have ? [["SVG", keep]] : []), [have]);
  const rows = status(stat);
  const told = JSON.stringify(rows);
  useEffect(() => onStatus?.(rows), [told]);
  const playing = Boolean(stat?.playing);
  const at = playing ? parked(stat!.part) : value.at;
  const down = (e: PointerEvent<HTMLCanvasElement>) => {
    if (e.button) return;
    e.currentTarget.setPointerCapture?.(e.pointerId);
    touch.current = { x: e.clientX, at, moved: false };
  };
  const move = (e: PointerEvent<HTMLCanvasElement>) => {
    const held = touch.current;
    if (!held || (!held.moved && Math.abs(e.clientX - held.x) < SLOP)) return;
    held.moved = true;
    const next = scrub(held.at, e.clientX - held.x, e.currentTarget.clientWidth);
    if (next !== now.current.at) change.current({ at: next });
  };
  const up = () => {
    const held = touch.current;
    touch.current = null;
    if (held && !held.moved) toggle();
  };
  const cancel = () => {
    touch.current = null;
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
          onPointerCancel={cancel}
        />
      )}
      <Library>
        <Tracks value={value} math={unit} onChange={onChange} />
      </Library>
      <Pane value={value} at={at} math={unit} onChange={onChange} />
    </>
  );
}
