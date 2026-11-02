import type { PointerEvent } from "react";
import { useCallback, useEffect, useRef, useState } from "react";
import { paints } from "../../net/sdk/color.ts";
import { text } from "../../net/sdk/export.ts";
import type { Readout } from "../../net/sdk/hooks.ts";
import { useUnit } from "../../net/sdk/hooks.ts";
import { Art, Item, Library, Shelf } from "../../net/sdk/library.tsx";
import type { Handle, Mathy, WidgetProps } from "../../net/sdk/page.tsx";
import { Scene } from "../../net/sdk/page.tsx";
import type { Opts, View } from "../../net/sdk/run.ts";
import type { Seeded } from "../../net/sdk/spec.ts";
import type { Mrly } from "../shared/designs/engine.ts";
import { useTheme } from "../shared/designs/pick.tsx";
import type { Slices, Told } from "./scene.ts";
import { codesOf, glance, make, name, TOP, thumb } from "./scene.ts";

/* TYPES */

type Props = WidgetProps<Mathy, Slices> & { value: Seeded; unit?: Mrly | null };

type Live = { seek?: (x: number, y: number) => number; svg?: () => Promise<string> };

const SIDES = Array.from({ length: TOP }, (_, i) => i + 1);

export function Widget({ value, onChange, onReady, onExport, onStatus, unit }: Props) {
  const math = useUnit("math", unit);
  const live = useRef<Live | null>(null);
  const now = useRef(value);
  const held = useRef(false);
  const [rows, setRows] = useState<Readout[]>([]);
  const tick = useTheme();
  now.current = value;
  const tell = useCallback((next: Told) => setRows(next.rows), []);
  useEffect(() => onStatus?.(rows), [rows]);
  const ready = (handle: Handle<Slices> | null) => {
    live.current = handle;
    onReady?.(handle);
  };
  const at = (e: PointerEvent<HTMLCanvasElement>) => {
    const canvas = e.currentTarget;
    const rect = canvas.getBoundingClientRect();
    return (
      live.current?.seek?.(
        ((e.clientX - rect.left) / rect.width) * canvas.width,
        ((e.clientY - rect.top) / rect.height) * canvas.height,
      ) ?? null
    );
  };
  const go = (k: number | null) =>
    k !== null && (k !== now.current.k || Number(now.current.tile) !== 0) && onChange({ k, tile: 0 });
  const down = (e: PointerEvent<HTMLCanvasElement>) => {
    const k = at(e);
    if (k === null) return;
    held.current = true;
    e.currentTarget.setPointerCapture?.(e.pointerId);
    go(k);
  };
  const move = (e: PointerEvent<HTMLCanvasElement>) => held.current && go(at(e));
  const up = () => {
    held.current = false;
  };
  const svg = () => live.current?.svg?.().then((body) => text(body, name(now.current), "svg"));
  useEffect(() => onExport?.(math ? [["SVG", svg]] : []), [Boolean(math)]);
  const scene = useCallback(
    (canvas: HTMLCanvasElement, view: View, opts: Opts) => make(canvas, view, { ...opts, math, tell }),
    [math],
  );
  const tiled = Number(value.tile) !== 0;
  return (
    <>
      {math && (
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
      {math && (
        <Library>
          <Shelf name="Sides" grid>
            {SIDES.map((k) => (
              <Item
                key={k}
                label={`k ${k}`}
                sub={`side ${2 * k - 1}`}
                on={!tiled && value.k === k}
                onPick={() => go(k)}
                art={
                  <Art
                    deps={[tick, value.base, value.code, k]}
                    paint={(ctx, side) =>
                      thumb(ctx, side, glance(math, value, k), codesOf(math), paints(ctx.canvas).accent)
                    }
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
