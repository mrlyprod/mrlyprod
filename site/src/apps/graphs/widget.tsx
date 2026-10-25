import type { MouseEvent, PointerEvent } from "react";
import { useCallback, useEffect, useRef, useState } from "react";
import { Btn, Group } from "../../lib/app/controls.tsx";
import { text } from "../../lib/app/export.ts";
import { Bar } from "../../lib/app/frame.tsx";
import type { Readout, Three } from "../../lib/app/hooks.ts";
import { useReroll, useThree } from "../../lib/app/hooks.ts";
import { Art, Item, Library, Shelf } from "../../lib/app/library.tsx";
import type { Handle, Mathy, WidgetProps } from "../../lib/app/page.tsx";
import { Scene } from "../../lib/app/page.tsx";
import type { Opts, View } from "../../lib/app/run.ts";
import type { Patch, Value } from "../../lib/app/spec.ts";
import type { Spot } from "../../lib/app/touch.ts";
import { paints } from "../../lib/color.ts";
import type { Mrly } from "../../lib/designs/engine.ts";
import { useTheme } from "../../lib/designs/pick.tsx";
import { rng } from "../../lib/rand.ts";
import type { Facts, Flat, Graph, Seed } from "./engine.ts";
import {
  bounds,
  deal,
  design,
  glimpse,
  graphs,
  solid as isSolid,
  kindOf,
  lay,
  plane,
  SETS,
  spaceOf,
  turn,
} from "./engine.ts";
import type { Graphs, Handled, Net, Relaxed } from "./scene.ts";
import { make, name } from "./scene.ts";

/* TYPES */

export type Told = Pick<
  Facts,
  | "name"
  | "code"
  | "space"
  | "graph"
  | "level"
  | "layout"
  | "nodes"
  | "branches"
  | "tips"
  | "junctions"
  | "pieces"
  | "length"
  | "box"
  | "euler"
>;

export type Cooling = Pick<Relaxed, "ticks" | "energy" | "moved" | "heat">;

export type Seen = { facts: Facts; relax: Relaxed | null };

export type Glance = { value: Seed; graph: Graph };

export type ShelvesProps = { value: Graphs; onChange: (patch: Patch) => void; math?: Mrly | null };

export type PaneProps = {
  value: Value;
  relax?: Pick<Relaxed, "on"> | null;
  onRelax: () => void;
  onReset: () => void;
  onPose: (pose: string) => void;
};

export type Props = Pick<WidgetProps<Mathy, Handled>, "onChange"> &
  Partial<Pick<WidgetProps<Mathy, Handled>, "onReady" | "onExport" | "onStatus" | "units">> & {
    value: Graphs;
    three?: Three | null;
  };

type From = { x: number; y: number; far: number; tap: boolean; pan: boolean };

type Touch = { points: Map<number, Spot>; from: From | null };

const POLL = 250;
const TAP = 6;
const WHEEL = 0.0015;
const DOT = 1.5;
const POSES: [string, string][] = [
  ["corner", "Corner"],
  ["edge", "Edge"],
  ["face", "Face"],
];
const CAMERA: Partial<Record<string, string>> & Record<"eye" | "iso", string> = {
  eye: "perspective",
  iso: "isometric",
};

const sci = (x: number) => Number(x).toExponential(2);

export const note = (f: Told, camera = "eye") =>
  `${f.graph} graph, level ${f.level}, ${f.layout}${f.space === "cube" ? `, ${CAMERA[camera] ?? CAMERA.eye}` : ""}`;

export function rows(f: Told, r: Cooling | null, camera?: string): Readout[] {
  const out: Readout[] = [
    ["design", `${f.name || "bang"} ${f.code}`],
    ["network", note(f, camera)],
    ["nodes", f.nodes],
    ["branches", f.branches],
    ["tips", f.tips],
    ["junctions", f.junctions],
    ["pieces", f.pieces],
    ["length", f.length.toFixed(2)],
    ["box dimension", f.box.toFixed(3)],
    ["euler", f.euler ?? "none"],
  ];
  if (!r) return out;
  return [
    ...out,
    ["ticks", r.ticks],
    ...(r.ticks ? [["energy", sci(r.energy)] satisfies Readout, ["moved", sci(r.moved)] satisfies Readout] : []),
    ["heat", sci(r.heat)],
  ];
}

export function etch(ctx: CanvasRenderingContext2D, seen: Flat, w: number, h: number, dpr: number) {
  const at = plane(seen);
  const box = lay(bounds(at, 2, seen.count), w, h, 4 * dpr);
  ctx.strokeStyle = paints(ctx.canvas).accent;
  ctx.fillStyle = ctx.strokeStyle;
  ctx.lineWidth = dpr;
  ctx.lineCap = "round";
  ctx.beginPath();
  for (let j = 0; j < seen.pairs.length; j += 2) {
    const [a, b] = [seen.pairs[j], seen.pairs[j + 1]];
    ctx.moveTo(box.ox + at[2 * a] * box.k, box.oy + at[2 * a + 1] * box.k);
    ctx.lineTo(box.ox + at[2 * b] * box.k, box.oy + at[2 * b + 1] * box.k);
  }
  ctx.stroke();
  ctx.beginPath();
  for (let i = 0; i < seen.count; i++) {
    const x = box.ox + at[2 * i] * box.k;
    ctx.moveTo(x + DOT * dpr, box.oy + at[2 * i + 1] * box.k);
    ctx.arc(x, box.oy + at[2 * i + 1] * box.k, DOT * dpr, 0, Math.PI * 2);
  }
  ctx.fill();
}

export const lit = (value: Value, patch: Patch) =>
  Object.entries(patch).every(([key, v]) => String(value[key]) === String(v));

export function Shelves({ value, onChange, math }: ShelvesProps) {
  const tick = useTheme();
  const glance = (look: Glance) => (
    <Art
      paint={(ctx, w, h, dpr) => etch(ctx, glimpse(math!, look.value, look.graph), w, h, dpr)}
      deps={[math, spaceOf(look.value), look.value.dim, look.value.base, look.value.code, look.graph, tick]}
    />
  );
  return (
    <>
      <Shelf name="Graphs" grid>
        {graphs(value).map(([graph, text]) => {
          const [head, tail] = text.split(", ");
          return (
            <Item
              key={graph}
              label={head}
              sub={tail}
              on={kindOf(value) === graph}
              onPick={() => onChange({ graph })}
              art={math ? glance({ value, graph }) : undefined}
            />
          );
        })}
      </Shelf>
      <Shelf name="Sets" grid>
        {SETS.map(([text, patch]) => (
          <Item
            key={text}
            label={text}
            on={lit(value, patch)}
            onPick={() => onChange(patch)}
            art={math ? glance({ value: patch, graph: patch.graph }) : undefined}
          />
        ))}
      </Shelf>
    </>
  );
}

export function Pane({ value, relax, onRelax, onReset, onPose }: PaneProps) {
  const running = Boolean(relax?.on);
  return (
    <>
      {value.layout === "force" && (
        <Group name="Relax" after="Layout">
          <Btn on={running} onClick={onRelax}>
            {running ? "Pause" : "Relax"}
          </Btn>
          <Btn onClick={onReset}>Reset</Btn>
        </Group>
      )}
      {isSolid(value) && (
        <Group name="Pose" after="Camera">
          {POSES.map(([pose, label]) => (
            <Btn key={pose} onClick={() => onPose(pose)}>
              {label}
            </Btn>
          ))}
        </Group>
      )}
    </>
  );
}

const same = (a: unknown, b: unknown) => JSON.stringify(a) === JSON.stringify(b);

export function Widget({ value, onChange, onReady, onExport, onStatus, units, three: given }: Props) {
  const math = units?.math;
  const cube = isSolid(value);
  const three = useThree(cube, given);
  const live = useRef<Handle<Net> | null>(null);
  const now = useRef(value);
  const touch = useRef<Touch>({ points: new Map(), from: null });
  const wheel = useRef<(() => void) | null>(null);
  const tell = useRef(onChange);
  const [seen, setSeen] = useState<Seen | null>(null);
  now.current = value;
  tell.current = onChange;
  const sync = () => {
    const handle = live.current;
    const next = handle ? { facts: handle.facts(), relax: handle.relaxed() } : null;
    setSeen((was) => (same(was, next) ? was : next));
  };
  const relax = () => {
    if (now.current.layout !== "force") return tell.current({ layout: "force" });
    live.current?.toggle?.();
    sync();
  };
  const reset = () => {
    live.current?.reset?.();
    sync();
  };
  const ready = (handle: Handle<Net> | null) => {
    if (wheel.current) wheel.current();
    wheel.current = null;
    live.current = handle;
    sync();
    if (!handle) return onReady?.(null);
    const spin = (e: WheelEvent) => {
      if (!live.current?.zoom) return;
      e.preventDefault();
      live.current.zoom(Math.exp(-e.deltaY * WHEEL));
    };
    handle.canvas?.addEventListener?.("wheel", spin, { passive: false });
    wheel.current = () => handle.canvas?.removeEventListener?.("wheel", spin);
    onReady?.({
      ...handle,
      toggle: relax,
      deeper: (by: number) => tell.current({ level: now.current.level + by }),
      next: (by: number) => tell.current({ graph: turn(now.current, by) }),
    });
  };
  useEffect(() => () => wheel.current?.(), []);
  useReroll(value.seed, (seed) => math && tell.current(deal(math, now.current, rng(seed))));
  const have = Boolean(seen);
  useEffect(() => {
    if (!have) return undefined;
    const timer = setInterval(sync, POLL);
    return () => clearInterval(timer);
  }, [have]);
  useEffect(() => onStatus?.(seen ? rows(seen.facts, seen.relax, value.camera) : []), [seen, value.camera]);
  useEffect(() => {
    sync();
    const facts = live.current?.facts?.();
    if (facts && facts.space === spaceOf(value) && design(value).code !== "" && facts.code !== design(value).code)
      onChange({ code: facts.code });
  }, [JSON.stringify(value), have]);
  const keep = (body: string | undefined, kind: string) => body && text(body, name(now.current), kind);
  const svg = () => keep(live.current?.svg?.(), "svg");
  const json = () => keep(live.current?.json?.(), "json");
  useEffect(
    () =>
      onExport?.(
        have
          ? [
              ["SVG", svg],
              ["JSON", json],
            ]
          : [],
      ),
    [have],
  );
  const scene = useCallback(
    (canvas: HTMLCanvasElement, view: View, opts: Opts) => make(canvas, view, { ...opts, math, three }),
    [math, three],
  );
  const down = (e: PointerEvent<HTMLCanvasElement>) => {
    const { points } = touch.current;
    e.currentTarget.setPointerCapture?.(e.pointerId);
    points.set(e.pointerId, [e.clientX, e.clientY]);
    if (points.size === 1) {
      touch.current.from = {
        x: e.clientX,
        y: e.clientY,
        far: 0,
        tap: e.button === 0,
        pan: e.button === 2 || e.shiftKey,
      };
      live.current?.grab?.();
    } else if (touch.current.from) touch.current.from.tap = false;
  };
  const move = (e: PointerEvent<HTMLCanvasElement>) => {
    const { points, from } = touch.current;
    const was = points.get(e.pointerId);
    if (!was || !from) return;
    const box = e.currentTarget.getBoundingClientRect();
    const at: Spot = [e.clientX, e.clientY];
    if (points.size === 2) {
      const [a, b] = [...points.values()];
      points.set(e.pointerId, at);
      const [c, d] = [...points.values()];
      const span = Math.hypot(a[0] - b[0], a[1] - b[1]) || 1;
      live.current?.zoom?.(Math.hypot(c[0] - d[0], c[1] - d[1]) / span);
      live.current?.pan?.((c[0] + d[0] - a[0] - b[0]) / 2 / box.width, (c[1] + d[1] - a[1] - b[1]) / 2 / box.height);
      return;
    }
    points.set(e.pointerId, at);
    from.far = Math.max(from.far, Math.hypot(at[0] - from.x, at[1] - from.y));
    const dx = (at[0] - was[0]) / box.width;
    const dy = (at[1] - was[1]) / box.height;
    if (from.pan) live.current?.pan?.(dx, dy);
    else live.current?.turn?.(dx, dy);
  };
  const up = (e: PointerEvent<HTMLCanvasElement>) => {
    const { points, from } = touch.current;
    if (!points.delete(e.pointerId) || points.size) return;
    live.current?.release?.();
    touch.current.from = null;
    if (!cube && e.type === "pointerup" && from?.tap && from.far < TAP) relax();
  };
  const hush = (e: MouseEvent<HTMLCanvasElement>) => cube && e.preventDefault();
  return (
    <>
      {math && (!cube || three) && (
        <Scene
          make={scene}
          value={value}
          onReady={ready}
          onPointerDown={down}
          onPointerMove={move}
          onPointerUp={up}
          onPointerCancel={up}
          onContextMenu={hush}
        />
      )}
      <Library>
        <Shelves value={value} onChange={onChange} math={math} />
      </Library>
      <Bar side="right">
        <Pane
          value={value}
          relax={seen?.relax}
          onRelax={relax}
          onReset={reset}
          onPose={(pose) => live.current?.snap?.(pose)}
        />
      </Bar>
    </>
  );
}
