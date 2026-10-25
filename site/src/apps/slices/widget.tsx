import type { PointerEvent } from "react";
import { useCallback, useEffect, useRef, useState } from "react";
import { text } from "../../lib/app/export.ts";
import type { Readout } from "../../lib/app/hooks.ts";
import { useReroll, useUnit } from "../../lib/app/hooks.ts";
import { Art, Item, Library, Shelf } from "../../lib/app/library.tsx";
import type { Handle, Mathy, WidgetProps } from "../../lib/app/page.tsx";
import { Scene } from "../../lib/app/page.tsx";
import type { Opts, View } from "../../lib/app/run.ts";
import { paints } from "../../lib/color.ts";
import type { Mrly } from "../../lib/designs/engine.ts";
import { roll } from "../../lib/designs/engine.ts";
import { read, useTheme } from "../../lib/designs/pick.tsx";
import { rng } from "../../lib/rand.ts";
import { DIM } from "../../lib/three/engine.ts";
import type { Handled, Hands, Slices, Told } from "./scene.ts";
import { codesOf, glance, make, name, TOP, thumb } from "./scene.ts";

/* TYPES */

type Props = WidgetProps<Mathy, Handled> & { unit?: Mrly | null };

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
  useReroll(
    Number(value.seed),
    (seed) => math && onChange({ code: roll(math, DIM, read(now.current).base, rng(seed)) }),
  );
  const ready = (handle: Handle<Slices> | null) => {
    live.current = handle;
    const hands: Hands = {
      step: (by) =>
        onChange({
          k: Math.min(TOP, Math.max(1, Number(now.current.tile) ? Number(now.current.k) : Number(now.current.k) + by)),
          tile: 0,
        }),
      deeper: (by) => onChange({ level: Number(now.current.level) + by }),
    };
    onReady?.(handle && { ...handle, ...hands });
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
