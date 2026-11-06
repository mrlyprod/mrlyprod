import { paints } from "@mrlyprod/ui/kit/color.ts";
import { Group, Slider } from "@mrlyprod/ui/kit/controls.tsx";
import type { Mrly } from "@mrlyprod/ui/kit/designs/engine.ts";
import { CRATES } from "@mrlyprod/ui/kit/designs/engine.ts";
import { useTheme } from "@mrlyprod/ui/kit/designs/pick.tsx";
import { text } from "@mrlyprod/ui/kit/export.ts";
import { Bar } from "@mrlyprod/ui/kit/frame.tsx";
import { useCrates } from "@mrlyprod/ui/kit/hooks.ts";
import { Art, Item, Library, Shelf } from "@mrlyprod/ui/kit/library.tsx";
import type { Handle, WidgetProps } from "@mrlyprod/ui/kit/page.tsx";
import { Scene } from "@mrlyprod/ui/kit/page.tsx";
import type { Opts, View } from "@mrlyprod/ui/kit/run.ts";
import type { Seeded } from "@mrlyprod/ui/kit/spec.ts";
import type { PointerEvent } from "react";
import { useCallback, useEffect, useRef, useState } from "react";
import type { Cuts, Info } from "./scene.ts";
import { make, name, near, plot, shelf, thumb } from "./scene.ts";

type Props = WidgetProps<Mrly, Cuts> & { value: Seeded; unit?: Mrly | null };

type Live = { seek?: (x: number, y: number) => number | null; svg?: () => Promise<string> };

export function Widget({ value, onChange, onReady, onExport, onStatus, unit }: Props) {
  const mrly = useCrates(CRATES, unit);
  const live = useRef<Live | null>(null);
  const now = useRef(value);
  const held = useRef(false);
  const [info, setInfo] = useState<Info | null>(null);
  const plane = useRef({ key: `${value.code} ${value.base} ${value.level}`, height: String(value.height ?? "") });
  const tick = useTheme();
  now.current = value;
  const tell = useCallback((next: Info) => setInfo(next), []);
  useEffect(() => onStatus?.(info?.rows ?? []), [info]);
  useEffect(() => {
    const was = plane.current;
    const next = { key: `${value.code} ${value.base} ${value.level}`, height: String(value.height ?? "") };
    plane.current = next;
    if (was.key !== next.key && was.height === next.height && next.height !== "") onChange({ height: "" });
  }, [value.code, value.base, value.level, value.height]);
  const cut = info?.cut;
  const at = info?.heights?.[0];
  const ready = (handle: Handle<Cuts> | null) => {
    live.current = handle;
    onReady?.(handle);
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
  useEffect(() => onExport?.(mrly ? [["SVG", svg]] : []), [Boolean(mrly)]);
  const scene = useCallback(
    (canvas: HTMLCanvasElement, view: View, opts: Opts) => make(canvas, view, { ...opts, mrly, tell }),
    [mrly],
  );
  const both = Number(value.both) === 1;
  const list = info && cut ? shelf(cut) : [];
  return (
    <>
      {mrly && (
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
      {mrly && list.length > 0 && (
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
                        plot(mrly, info!.code, info!.base, info!.level, h),
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
