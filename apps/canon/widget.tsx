import type { MouseEvent, PointerEvent } from "react";
import { useCallback, useRef, useState } from "react";
import { paints } from "../../net/sdk/color.ts";
import type { Readout } from "../../net/sdk/hooks.ts";
import { useStatus } from "../../net/sdk/hooks.ts";
import { Art, Item, Library, Shelf } from "../../net/sdk/library.tsx";
import type { Handle, Mathy, WidgetProps } from "../../net/sdk/page.tsx";
import { Scene } from "../../net/sdk/page.tsx";
import type { Opts, View } from "../../net/sdk/run.ts";
import type { Mrly } from "../shared/designs/engine.ts";
import { useTheme } from "../shared/designs/pick.tsx";
import { grown, HOME, ONE } from "./engine.ts";
import type { Canon, Held, Told } from "./scene.ts";
import { make } from "./scene.ts";

/* TYPES */

type Props = Partial<WidgetProps<Mathy, Held>> & Pick<WidgetProps<Mathy, Held>, "value" | "onChange">;

export const CODES = [HOME, "495", "186", "341", "94", "179", "313", "510"];

const THUMB = 3;
const SIDE = 3;
const GAP = 0.1;

const named = (who: number) => (who === ONE ? "squares" : "dots");

export function rows(told: Told | null): Readout[] {
  if (!told) return [];
  const [one, two] = told.points;
  const end: Readout =
    told.over === "draw"
      ? ["result", "draw"]
      : told.over
        ? ["won", told.over === "one" ? "squares" : "dots"]
        : ["to move", named(told.turn)];
  return [["squares", one], ["dots", two], end];
}

export function sample(ctx: CanvasRenderingContext2D, math: Mrly, code: string, w: number, accent: string) {
  const cells = grown(math, SIDE, THUMB, code);
  const n = SIDE ** THUMB;
  const room = w * (1 - 2 * GAP);
  const at = (i: number) => Math.round(w * GAP + (i * room) / n);
  ctx.fillStyle = accent;
  ctx.beginPath();
  for (let i = 0; i < cells.length; i++) {
    if (cells[i] !== "1") continue;
    const x = i % n;
    const y = Math.floor(i / n);
    ctx.rect(at(x), at(y), at(x + 1) - at(x), at(y + 1) - at(y));
  }
  ctx.fill();
}

export function Widget({ value, onChange, onReady, onStatus, units }: Props) {
  const unit = units?.math;
  const live = useRef<Handle<Canon> | null>(null);
  const now = useRef(value);
  const change = useRef(onChange);
  now.current = value;
  change.current = onChange;
  const [told, setTold] = useState<Told | null>(null);
  const tick = useTheme();
  const scene = useCallback(
    (canvas: HTMLCanvasElement, view: View, opts: Opts) => make(canvas, view, { ...opts, math: unit, onTell: setTold }),
    [unit],
  );
  const ready = (handle: Handle<Canon> | null) => {
    live.current = handle;
    onReady?.(handle && { ...handle, again: () => change.current({ seed: (Number(now.current.seed) + 1) >>> 0 }) });
  };
  const at = (e: PointerEvent<HTMLCanvasElement> | MouseEvent<HTMLCanvasElement>): [number, number] => {
    const box = e.currentTarget.getBoundingClientRect();
    return [(e.clientX - box.left) / (box.width || 1), (e.clientY - box.top) / (box.height || 1)];
  };
  const hover = (e: PointerEvent<HTMLCanvasElement>) => {
    if (e.pointerType === "mouse") live.current?.hover(...at(e));
  };
  const tap = (e: MouseEvent<HTMLCanvasElement>) => live.current?.tap(...at(e));
  useStatus(onStatus, rows(told));
  const code = String(value.code ?? "").trim() || HOME;
  return (
    <>
      {unit && (
        <Scene
          make={scene}
          value={value}
          onReady={ready}
          onPointerMove={hover}
          onClick={tap}
          role="img"
          aria-label="A fractal board two players claim, every claim echoing at every scale"
        />
      )}
      {unit && (
        <Library name="Designs">
          <Shelf grid>
            {CODES.map((one) => (
              <Item
                key={one}
                label={one}
                on={code === one}
                onPick={() => onChange({ code: one === HOME ? "" : one })}
                art={
                  <Art
                    paint={(ctx, w) => sample(ctx, unit, one, w, paints(ctx.canvas, value.tint as string).accent)}
                    deps={[one, value.tint, tick]}
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
