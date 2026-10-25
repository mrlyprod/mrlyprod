import type { MouseEvent, PointerEvent } from "react";
import { useCallback, useEffect, useRef, useState } from "react";
import type { Readout } from "../../lib/app/hooks.ts";
import { useLatest, useReroll, useStatus } from "../../lib/app/hooks.ts";
import { Art, Item, Library, Shelf } from "../../lib/app/library.tsx";
import type { Handle, WidgetProps } from "../../lib/app/page.tsx";
import { Scene } from "../../lib/app/page.tsx";
import type { Fix, Opts, View } from "../../lib/app/run.ts";
import type { Units, Value } from "../../lib/app/spec.ts";
import { paints } from "../../lib/color.ts";
import { useTheme } from "../../lib/designs/pick.tsx";
import type { Pair } from "../../lib/fractal/orbit.ts";
import { float } from "../../lib/fractal/orbit.ts";
import type { View as Sight } from "../../lib/fractal/view.ts";
import { pan, zoomAt, zoomOf } from "../../lib/fractal/view.ts";
import type { Kind } from "./places.ts";
import { KINDS, PLACES, roll, thumb } from "./places.ts";
import type { Flight, Steered } from "./scene.ts";
import { at, go, limits, make, seat, spot, where } from "./scene.ts";

/* TYPES */

type Props = Partial<WidgetProps<Units, Steered>> & Pick<WidgetProps<Units, Steered>, "value" | "onChange">;

type PlacesProps = Pick<WidgetProps<Units, Steered>, "value" | "onChange">;

type Rect = Pick<DOMRect, "left" | "top" | "width" | "height">;

type Pointer = Pick<MouseEvent, "clientX" | "clientY">;

type Grab = { points: Map<number, Pair>; from: Sight | null | undefined; start: Pair[] | null };

type Mine = { view: Sight | null; wrote: string[] };

const WHEEL = 1 / 400;
const LINE = 16;
const KEPT = 8;
const short = (v: number) => Number(v.toPrecision(7));
const three = (v: number) => +v.toFixed(3);
const complex = ([re, im]: readonly number[]) => `${three(re)}${im < 0 ? "-" : "+"}${three(Math.abs(im))}i`;

/* VIEW */

export function point(box: Rect, e: Pointer): Pair {
  const side = Math.min(box.width, box.height) || 1;
  return [(e.clientX - box.left - box.width / 2) / side, (box.top + box.height / 2 - e.clientY) / side];
}

export function scale(v: Sight, p: readonly number[], k: number, [least, most]: readonly number[]) {
  const span = Math.min(most, Math.max(least, v.span / k));
  return zoomAt(v, p, v.span / span);
}

export function pinch(from: Sight, [a, b]: readonly Pair[], [c, d]: readonly Pair[], edges: readonly number[]) {
  const mid = (p: Pair, q: Pair): Pair => [(p[0] + q[0]) / 2, (p[1] + q[1]) / 2];
  const gap = (p: Pair, q: Pair) => Math.hypot(p[0] - q[0], p[1] - q[1]) || 1e-9;
  const [m0, m1] = [mid(a, b), mid(c, d)];
  return pan(scale(from, m0, gap(c, d) / gap(a, b), edges), [m1[0] - m0[0], m1[1] - m0[1]]);
}

const mag = (zoom: number) => {
  const n = 2 ** zoom;
  return `×${n < 1000 ? +n.toPrecision(3) : n.toExponential(1).replace("e+", "e")}`;
};

export const rows = (value: Value, v: Sight): Readout[] => [
  ["zoom", mag(zoomOf(v.span))],
  ["x", String(short(float(v.centre[0])))],
  ["y", String(short(float(v.centre[1])))],
  ...(value.kind === "julia"
    ? [["c", complex([short(value.cx as number), short(value.cy as number)])] satisfies Readout]
    : []),
];

/* PLACES */

function Places({ value, onChange }: PlacesProps) {
  const tick = useTheme();
  const [find, setFind] = useState("");
  const want = find.trim().toLowerCase();
  return (
    <Library name="Places" find={find} onFind={setFind}>
      {KINDS.map(([kind, name]) => {
        const list = PLACES.filter((one) => one.kind === kind && one.name.toLowerCase().includes(want));
        if (!list.length) return null;
        return (
          <Shelf key={kind} name={name} grid>
            {list.map((one) => (
              <Item
                key={one.id}
                label={one.name}
                sub={one.c ? `c ${complex(one.c)}` : mag(one.zoom)}
                on={at(value, one)}
                onPick={() => onChange(go(one))}
                art={
                  <Art
                    deps={[one.id, value.tint, tick]}
                    paint={(ctx, w, h) => thumb(ctx, one, w, h, paints(ctx.canvas, value.tint as string))}
                  />
                }
              />
            ))}
          </Shelf>
        );
      })}
    </Library>
  );
}

/* WIDGET */

export function Widget({ value, onChange, onReady, onStatus }: Props) {
  const live = useRef<Handle<Flight> | null>(null);
  const now = useLatest(value);
  const change = useLatest(onChange);
  const grab = useRef<Grab>({ points: new Map(), from: null, start: null });
  const mine = useRef<Mine>({ view: null, wrote: [] });
  const forget = () => (mine.current = { view: null, wrote: [] });
  const [flight, setFlight] = useState<Sight | null>(null);
  if (mine.current.view && !mine.current.wrote.includes(where(value))) forget();
  useStatus(onStatus, rows(value, flight ?? seat(value)));
  useReroll(value.seed as number, (seed) => onChange(go(roll(now.current.kind, seed))));
  const rest = useCallback((place: Value | null) => {
    live.current?.pause();
    setFlight(null);
    if (!place) return;
    mine.current = { view: null, wrote: [] };
    change.current(place);
  }, []);
  const tell = useCallback((v: Sight) => setFlight(v), []);
  const scene = useCallback(
    (canvas: HTMLCanvasElement, view: View, opts: Opts) =>
      make(canvas, view, { ...opts, onRest: rest, onTell: tell }) as Flight,
    [],
  );
  const base = () => mine.current.view ?? live.current?.at();
  const put = (v: Sight) => {
    const patch = spot(now.current.kind as Kind, v);
    const kept = where({ ...now.current, ...patch });
    mine.current = { view: v, wrote: [...mine.current.wrote, kept].slice(-KEPT) };
    change.current(patch);
  };
  const land = () => {
    const handle = live.current;
    if (!handle?.flying()) return;
    const place = handle.halt();
    handle.pause();
    setFlight(null);
    forget();
    change.current(place);
  };
  const zoom = (p: readonly number[], k: number) => {
    land();
    const v = base();
    if (v) put(scale(v, p, k, limits(now.current)));
  };
  const box = () => live.current?.canvas.getBoundingClientRect();
  const wheel = useLatest((e: WheelEvent) => {
    e.preventDefault();
    zoom(point(box()!, e), 2 ** (-e.deltaY * (e.deltaMode ? LINE : 1) * WHEEL));
  });
  const unwheel = useRef(() => {});
  useEffect(() => () => unwheel.current(), []);
  const ready = (handle: Handle<Flight> | null) => {
    unwheel.current();
    live.current = handle;
    if (!handle) return onReady?.(null);
    const turn = (e: WheelEvent) => wheel.current(e);
    handle.canvas.addEventListener("wheel", turn, { passive: false });
    unwheel.current = () => handle.canvas.removeEventListener("wheel", turn);
    const toggle = () => {
      if (handle.flying()) return land();
      handle.fly();
      handle.play();
    };
    const fix = (next?: Fix | null) => {
      if (next?.step && !handle.flying()) handle.fly();
      if (next?.step) handle.play();
      handle.fix(next);
    };
    const julia = () => {
      const [cx, cy] = handle.at().centre.map((one) => Number(float(one).toFixed(9)));
      change.current({ kind: "julia", cx, cy, ...go({ kind: "julia", x: "0", y: "0", zoom: 0 }) });
    };
    onReady?.({
      ...handle,
      toggle,
      fix,
      julia,
      playing: () => handle.flying(),
      zoom: (by) => zoom([0, 0], 2 ** by),
    });
  };
  const restart = () => {
    const g = grab.current;
    g.from = base();
    g.start = [...g.points.values()];
  };
  const down = (e: PointerEvent<HTMLCanvasElement>) => {
    if (e.button || !live.current) return;
    e.currentTarget.setPointerCapture?.(e.pointerId);
    land();
    grab.current.points.set(e.pointerId, point(box()!, e));
    restart();
  };
  const move = (e: PointerEvent<HTMLCanvasElement>) => {
    const g = grab.current;
    if (!g.points.has(e.pointerId) || !g.from) return;
    g.points.set(e.pointerId, point(box()!, e));
    const [a, b] = g.start!;
    const [c, d] = [...g.points.values()];
    put(b && d ? pinch(g.from, [a, b], [c, d], limits(now.current)) : pan(g.from, [c[0] - a[0], c[1] - a[1]]));
  };
  const up = (e: PointerEvent<HTMLCanvasElement>) => {
    if (!grab.current.points.delete(e.pointerId)) return;
    restart();
  };
  const twice = (e: MouseEvent<HTMLCanvasElement>) => zoom(point(box()!, e), e.shiftKey ? 0.5 : 2);
  return (
    <>
      <Scene
        make={scene}
        value={value}
        onReady={ready}
        onPointerDown={down}
        onPointerMove={move}
        onPointerUp={up}
        onPointerCancel={up}
        onDoubleClick={twice}
        style={{ cursor: "grab" }}
      />
      <Places value={value} onChange={onChange} />
    </>
  );
}
