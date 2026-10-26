import type { PointerEvent } from "react";
import { useCallback, useEffect, useRef, useState } from "react";
import { Group, Slider } from "../../lib/app/controls.tsx";
import { text } from "../../lib/app/export.ts";
import { Bar } from "../../lib/app/frame.tsx";
import { useReroll, useUnit } from "../../lib/app/hooks.ts";
import { Art, Item, Library, Shelf } from "../../lib/app/library.tsx";
import type { Handle, Mathy, WidgetProps } from "../../lib/app/page.tsx";
import { Scene } from "../../lib/app/page.tsx";
import type { Opts, View } from "../../lib/app/run.ts";
import type { Seeded } from "../../lib/app/spec.ts";
import { paints } from "../../lib/color.ts";
import type { Mrly } from "../../lib/designs/engine.ts";
import { roll } from "../../lib/designs/engine.ts";
import { read, useTheme } from "../../lib/designs/pick.tsx";
import { rng } from "../../lib/rand.ts";
import { DIM } from "../../lib/three/engine.ts";
import type { Cuts, Handled, Hands, Info } from "./scene.ts";
import { make, name, near, plot, shelf, thumb } from "./scene.ts";

/* TYPES */

type Props = WidgetProps<Mathy, Handled> & { value: Seeded; unit?: Mrly | null };

type Live = { seek?: (x: number, y: number) => number | null; svg?: () => Promise<string> };

export function Widget({ value, onChange, onReady, onExport, onStatus, unit }: Props) {
  const math = useUnit("math", unit);
  const live = useRef<Live | null>(null);
  const now = useRef(value);
  const held = useRef(false);
  const [info, setInfo] = useState<Info | null>(null);
  const seen = useRef<Info | null>(null);
  const plane = useRef({ key: `${value.code} ${value.base} ${value.level}`, height: String(value.height ?? "") });
  const tick = useTheme();
  now.current = value;
  seen.current = info;
  const tell = useCallback((next: Info) => setInfo(next), []);
  useEffect(() => onStatus?.(info?.rows ?? []), [info]);
  useEffect(() => {
    const was = plane.current;
    const next = { key: `${value.code} ${value.base} ${value.level}`, height: String(value.height ?? "") };
    plane.current = next;
    if (was.key !== next.key && was.height === next.height && next.height !== "") onChange({ height: "" });
  }, [value.code, value.base, value.level, value.height]);
  useReroll(
    value.seed,
    (seed) => math && onChange({ code: roll(math, DIM, read(now.current).base, rng(seed)), height: "" }),
  );
  const cut = info?.cut;
  const at = info?.heights?.[0];
  const ready = (handle: Handle<Cuts> | null) => {
    live.current = handle;
    const hands: Hands = {
      slide: (by) => {
        const got = seen.current;
        if (!got || got.cut.empty) return;
        onChange({ height: Math.min(got.cut.high, Math.max(got.cut.low, got.heights[0] + by)), both: 0 });
      },
      deeper: (by) => onChange({ level: Number(now.current.level) + by }),
    };
    onReady?.(handle && { ...handle, ...hands });
  };
  const spot = (e: PointerEvent<HTMLCanvasElement>) => {
    const canvas = e.currentTarget;
    const rect = canvas.getBoundingClientRect();
    return (
      live.current?.seek?.(
        ((e.clientX - rect.left) / rect.width) * canvas.width,
        ((e.clientY - rect.top) / rect.height) * canvas.height,
      ) ?? null
    );
  };
  const go = (height: number | null) => height !== null && onChange({ height, both: 0 });
  const down = (e: PointerEvent<HTMLCanvasElement>) => {
    const height = spot(e);
    if (height === null) return;
    held.current = true;
    e.currentTarget.setPointerCapture?.(e.pointerId);
    go(height);
  };
  const move = (e: PointerEvent<HTMLCanvasElement>) => held.current && go(spot(e));
  const up = () => {
    held.current = false;
  };
  const svg = () => live.current?.svg?.().then((body) => text(body, name(now.current), "svg"));
  useEffect(() => onExport?.(math ? [["SVG", svg]] : []), [Boolean(math)]);
  const scene = useCallback(
    (canvas: HTMLCanvasElement, view: View, opts: Opts) => make(canvas, view, { ...opts, math, tell }),
    [math],
  );
  const both = Number(value.both) === 1;
  const list = info && cut ? shelf(cut) : [];
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
      {math && list.length > 0 && (
        <Library>
          <Shelf name="Heights" grid>
            {list.map((h) => (
              <Item
                key={h}
                label={String(h)}
                sub={String(cut!.counts[h] ?? 0)}
                on={info!.heights.some((one) => near(list, one) === h)}
                onPick={() => go(h)}
                art={
                  <Art
                    deps={[tick, info!.code, info!.base, info!.level, h]}
                    paint={(ctx, side) =>
                      thumb(
                        ctx,
                        side,
                        plot(math, info!.code, info!.base, info!.level, h),
                        cut!.side,
                        paints(ctx.canvas).accent,
                      )
                    }
                  />
                }
              />
            ))}
          </Shelf>
        </Library>
      )}
      <Bar side="right">
        {cut && !cut.empty && (
          <Group name="Plane">
            <Slider
              label="Height"
              value={at!}
              min={cut.low}
              max={cut.high}
              step={1}
              disabled={both}
              onChange={(v) => onChange({ height: v })}
            />
          </Group>
        )}
      </Bar>
    </>
  );
}
