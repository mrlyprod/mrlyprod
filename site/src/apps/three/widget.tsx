import type { CSSProperties, PointerEvent } from "react";
import { useCallback, useEffect, useRef, useState } from "react";
import { text } from "../../lib/app/export.ts";
import type { Had, Three } from "../../lib/app/hooks.ts";
import { useReroll, useThree, useUnit } from "../../lib/app/hooks.ts";
import { Art, Item, Library, Shelf } from "../../lib/app/library.tsx";
import type { Handle, WidgetProps } from "../../lib/app/page.tsx";
import { Scene } from "../../lib/app/page.tsx";
import type { Make } from "../../lib/app/run.ts";
import type { Patch, Seeded, Value } from "../../lib/app/spec.ts";
import type { Spot } from "../../lib/app/touch.ts";
import { paints } from "../../lib/color.ts";
import type { Mrly } from "../../lib/designs/engine.ts";
import { roll } from "../../lib/designs/engine.ts";
import { read, useTheme } from "../../lib/designs/pick.tsx";
import { rng } from "../../lib/rand.ts";
import { DIM } from "../../lib/three/engine.ts";
import type { Pose } from "../../lib/three/thumbs.ts";
import { BLOCK, block, glance, POSES } from "../../lib/three/thumbs.ts";
import type { Hands, Held, Sponge, Told } from "./scene.ts";
import { CROPS, make, name } from "./scene.ts";

/* TYPES */

export type Props = Pick<WidgetProps<Had<"math">, Held>, "onChange"> & { value: Seeded } & Partial<
    Pick<WidgetProps<Had<"math">, Held>, "onReady" | "onExport" | "onStatus">
  > & {
    unit?: Mrly | null;
    three?: Three | null;
  };

type Grip = { points: Map<number, Spot>; mode: "turn" | "pan"; last: Spot; at: number };

const WHEEL = 0.0015;
const FRESH = 80;
const LOOKS: [Pose, string, string][] = [
  ["corner", "Corner", "1"],
  ["edge", "Edge", "2"],
  ["face", "Face", "3"],
];
const COMBINE = ["sum", "xor"];

const warn: CSSProperties = {
  position: "absolute",
  top: 16,
  left: "50%",
  transform: "translateX(-50%)",
  maxWidth: "80%",
  textAlign: "center",
  fontSize: 13,
  opacity: 0.85,
  pointerEvents: "none",
};

export const hands = (at: () => Value, onChange: (patch: Patch) => void): Hands => ({
  deeper: (by) => onChange({ level: (at().level as number) + by }),
  widen: (by) => onChange({ radius: (at().radius as number) + by }),
  toggle: () => onChange({ spin: Number(at().spin) === 1 ? 0 : 1 }),
  playing: () => Number(at().spin) === 1,
});

export function Widget({ value, onChange, onReady, onExport, onStatus, unit, three: given }: Props) {
  const math = useUnit("math", unit);
  const three = useThree(true, given);
  const live = useRef<Handle<Sponge> | null>(null);
  const now = useRef(value);
  const touch = useRef<Grip>({ points: new Map(), mode: "turn", last: [0, 0], at: 0 });
  const [info, setInfo] = useState<Told | null>(null);
  const tick = useTheme();
  now.current = value;
  const tell = useCallback((next: Told) => setInfo(next), []);
  useEffect(() => onStatus?.(info?.rows ?? []), [info]);
  const rolled = (seed: number) => {
    if (!math) return;
    const base = read(now.current).base;
    const rand = rng(seed);
    const patch: Patch = { code: roll(math, DIM, base, rand) };
    if (now.current.source === "field") patch.combine = COMBINE[Math.floor(rand() * COMBINE.length)];
    onChange(patch);
  };
  useReroll(value.seed, rolled);
  const mode = useRef(`${value.source} ${Boolean(value.crop)}`);
  useEffect(() => {
    const next = `${value.source} ${Boolean(value.crop)}`;
    if (mode.current === next) return;
    const calm = value.source === "field" || Boolean(value.crop);
    const was = mode.current;
    mode.current = next;
    if (calm && was === "design false") onChange({ spin: 0 });
  }, [value.source, value.crop]);
  const wheel = useRef<(() => void) | null>(null);
  const ready = (handle: Handle<Sponge> | null) => {
    live.current = handle;
    wheel.current?.();
    wheel.current = null;
    const canvas = handle?.canvas;
    if (canvas?.addEventListener) {
      const wheeled = (e: WheelEvent) => {
        e.preventDefault();
        live.current?.zoom?.(Math.exp(-e.deltaY * WHEEL));
      };
      const menu = (e: MouseEvent) => e.preventDefault();
      canvas.addEventListener("wheel", wheeled, { passive: false });
      canvas.addEventListener("contextmenu", menu);
      wheel.current = () => {
        canvas.removeEventListener("wheel", wheeled);
        canvas.removeEventListener("contextmenu", menu);
      };
    }
    onReady?.(handle && { ...handle, ...hands(() => now.current, onChange) });
  };
  useEffect(() => () => wheel.current?.(), []);
  const down = (e: PointerEvent<HTMLCanvasElement>) => {
    const state = touch.current;
    state.points.set(e.pointerId, [e.clientX, e.clientY]);
    state.mode = e.button === 2 || e.shiftKey ? "pan" : "turn";
    state.last = [0, 0];
    e.currentTarget.setPointerCapture?.(e.pointerId);
  };
  const move = (e: PointerEvent<HTMLCanvasElement>) => {
    const state = touch.current;
    const from = state.points.get(e.pointerId);
    if (!from) return;
    const rect = e.currentTarget.getBoundingClientRect();
    const scene = live.current;
    if (state.points.size === 2) {
      const gap = () => {
        const [a, b] = [...state.points.values()];
        return Math.hypot(a[0] - b[0], a[1] - b[1]);
      };
      const before = gap();
      state.points.set(e.pointerId, [e.clientX, e.clientY]);
      if (before > 0) scene?.zoom?.(gap() / before);
      scene?.pan?.((e.clientX - from[0]) / rect.width / 2, (e.clientY - from[1]) / rect.height / 2);
      return;
    }
    const dx = (e.clientX - from[0]) / rect.width;
    const dy = (e.clientY - from[1]) / rect.height;
    state.points.set(e.pointerId, [e.clientX, e.clientY]);
    if (state.mode === "pan") scene?.pan?.(dx, dy);
    else scene?.turn?.(dx, dy);
    state.last = [dx, dy];
    state.at = e.timeStamp;
  };
  const up = (e: PointerEvent<HTMLCanvasElement>) => {
    const state = touch.current;
    const had = state.points.size;
    state.points.delete(e.pointerId);
    if (had === 1 && state.mode === "turn" && e.timeStamp - state.at < FRESH) live.current?.fling?.(...state.last);
  };
  const keep = (body: string | undefined, kind: string) => body && text(body, name(now.current), kind);
  const obj = () => keep(live.current?.obj?.(), "obj");
  useEffect(() => onExport?.(math ? [["OBJ", obj]] : []), [Boolean(math)]);
  const scene = useCallback<Make<Sponge>>(
    (canvas, view, opts) => make(canvas, view, { ...opts, math, three, tell }),
    [math, three],
  );
  const tone = (canvas: HTMLCanvasElement) => paints(canvas, value.tint as string);
  return (
    <>
      {math && three && (
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
      {info?.error && <p style={warn}>{info.error}</p>}
      {math && (
        <Library>
          <Shelf name="Look from" grid>
            {LOOKS.map(([key, label, hint]) => (
              <Item
                key={key}
                label={label}
                sub={hint}
                onPick={() => live.current?.look?.(key)}
                art={
                  <Art
                    deps={[tick, value.tint]}
                    paint={(ctx, side) => glance(ctx, BLOCK, POSES[key], side, tone(ctx.canvas))}
                  />
                }
              />
            ))}
          </Shelf>
          <Shelf name="Crop" grid>
            {CROPS.map(([key, label]) => (
              <Item
                key={key || "none"}
                label={label}
                on={(value.crop || "") === key}
                onPick={() => onChange({ crop: key })}
                art={
                  <Art
                    deps={[tick, value.tint, key]}
                    paint={(ctx, side) => glance(ctx, block(math, key), POSES.corner, side, tone(ctx.canvas))}
                  />
                }
              />
            ))}
          </Shelf>
        </Library>
      )}
    </>
  );
}
