import { useCallback, useEffect, useRef, useState } from "react";
import { Group, Slider } from "../../lib/app/controls.jsx";
import { text } from "../../lib/app/export.js";
import { Bar } from "../../lib/app/frame.jsx";
import { useReroll, useUnit } from "../../lib/app/hooks.js";
import { Art, Item, Library, Shelf } from "../../lib/app/library.jsx";
import { Scene } from "../../lib/app/page.jsx";
import { paints } from "../../lib/color.js";
import { roll } from "../../lib/designs/engine.js";
import { read, useTheme } from "../../lib/designs/pick.jsx";
import { rng } from "../../lib/rand.js";
import { DIM } from "../../lib/three/engine.js";
import { make, name, near, plot, shelf, thumb } from "./scene.js";

export function Widget({ value, onChange, onReady, onExport, onStatus, unit }) {
  const math = useUnit("math", unit);
  const live = useRef(null);
  const now = useRef(value);
  const held = useRef(false);
  const [info, setInfo] = useState(null);
  const seen = useRef(null);
  const plane = useRef({ key: `${value.code} ${value.base} ${value.level}`, height: String(value.height ?? "") });
  const tick = useTheme();
  now.current = value;
  seen.current = info;
  const tell = useCallback((next) => setInfo(next), []);
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
  const ready = (handle) => {
    live.current = handle;
    const hands = {
      slide: (by) => {
        const got = seen.current;
        if (!got || got.cut.empty) return;
        onChange({ height: Math.min(got.cut.high, Math.max(got.cut.low, got.heights[0] + by)), both: 0 });
      },
      deeper: (by) => onChange({ level: now.current.level + by }),
    };
    onReady?.(handle && { ...handle, ...hands });
  };
  const spot = (e) => {
    const canvas = e.currentTarget;
    const rect = canvas.getBoundingClientRect();
    return (
      live.current?.seek?.(
        ((e.clientX - rect.left) / rect.width) * canvas.width,
        ((e.clientY - rect.top) / rect.height) * canvas.height,
      ) ?? null
    );
  };
  const go = (height) => height !== null && onChange({ height, both: 0 });
  const down = (e) => {
    const height = spot(e);
    if (height === null) return;
    held.current = true;
    e.currentTarget.setPointerCapture?.(e.pointerId);
    go(height);
  };
  const move = (e) => held.current && go(spot(e));
  const up = () => {
    held.current = false;
  };
  const svg = () => live.current?.svg?.().then((body) => text(body, name(now.current), "svg"));
  useEffect(() => onExport?.(math ? [["SVG", svg]] : []), [Boolean(math)]);
  const scene = useCallback((canvas, view, opts) => make(canvas, view, { ...opts, math, tell }), [math]);
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
                sub={String(cut.counts[h] ?? 0)}
                on={info.heights.some((one) => near(list, one) === h)}
                onPick={() => go(h)}
                art={
                  <Art
                    deps={[tick, info.code, info.base, info.level, h]}
                    paint={(ctx, side) =>
                      thumb(
                        ctx,
                        side,
                        plot(math, info.code, info.base, info.level, h),
                        cut.side,
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
              value={at}
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
