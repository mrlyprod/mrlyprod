import type { PointerEvent } from "react";
import { useCallback, useRef, useState } from "react";
import { Btn } from "../../lib/app/controls.tsx";
import { Bar } from "../../lib/app/frame.tsx";
import { useStatus } from "../../lib/app/hooks.ts";
import { Art, Item, Library, Shelf } from "../../lib/app/library.tsx";
import type { Handle, WidgetProps } from "../../lib/app/page.tsx";
import { Scene } from "../../lib/app/page.tsx";
import type { Opts, View } from "../../lib/app/run.ts";
import type { Units } from "../../lib/app/spec.ts";
import { paints } from "../../lib/color.ts";
import { useTheme } from "../../lib/designs/pick.tsx";
import { DESIGNS, ink, tile } from "../../lib/designs/tiles.ts";
import type { Over } from "./engine.ts";
import { DOWN, LEFT, RIGHT, UP } from "./engine.ts";
import type { Snake } from "./scene.ts";
import { make, named, PARTS, PIECES } from "./scene.ts";

/* TYPES */

type Props = Partial<WidgetProps<Units, Snake>> & Pick<WidgetProps<Units, Snake>, "value" | "onChange">;

const SWIPE = 24;

const CELLS = 5;

const GAP = 0.12;

export function sample(ctx: CanvasRenderingContext2D, design: string, w: number, accent: string) {
  ctx.fillStyle = accent;
  const room = w * (1 - 2 * GAP);
  if (design) {
    ink(ctx, tile(design, CELLS), w * GAP, w * GAP, room);
    return;
  }
  for (const [i, one] of DESIGNS.slice(0, 4).entries())
    ink(ctx, tile(one, CELLS), w * GAP + (i % 2) * (room / 2), w * GAP + Math.floor(i / 2) * (room / 2), room / 2);
}

export function Widget({ value, onChange, onReady, onStatus }: Props) {
  const live = useRef<Handle<Snake> | null>(null);
  const from = useRef<[number, number] | null>(null);
  const [score, setScore] = useState(0);
  const [over, setOver] = useState<Over>(null);
  const tick = useTheme();
  const game = useCallback(
    (canvas: HTMLCanvasElement, view: View, opts: Opts) =>
      make(canvas, view, { ...opts, player: true, onScore: setScore, onOver: setOver }),
    [],
  );
  const ready = (handle: Handle<Snake> | null) => {
    live.current = handle;
    onReady?.(handle);
  };
  const down = (e: PointerEvent<HTMLCanvasElement>) => {
    from.current = [e.clientX, e.clientY];
    e.currentTarget.setPointerCapture?.(e.pointerId);
  };
  const move = (e: PointerEvent<HTMLCanvasElement>) => {
    if (!from.current) return;
    const dx = e.clientX - from.current[0];
    const dy = e.clientY - from.current[1];
    if (Math.max(Math.abs(dx), Math.abs(dy)) < SWIPE) return;
    live.current?.turn(Math.abs(dx) > Math.abs(dy) ? (dx > 0 ? RIGHT : LEFT) : dy > 0 ? DOWN : UP);
    from.current = [e.clientX, e.clientY];
  };
  const up = () => {
    from.current = null;
  };
  const said = over === "won" ? "won" : over ? "game over" : "score";
  useStatus(onStatus, [[said, score]]);
  return (
    <>
      <Scene
        make={game}
        value={value}
        onReady={ready}
        onPointerDown={down}
        onPointerMove={move}
        onPointerUp={up}
        onPointerCancel={up}
      />
      <Library name="Pieces">
        {PARTS.map((part) => (
          <Shelf key={part} name={named(part)} grid>
            {PIECES.map(([design, label]) => (
              <Item
                key={design || "random"}
                label={label}
                on={value[part] === design}
                onPick={() => onChange({ [part]: design })}
                art={
                  <Art
                    paint={(ctx, w) => sample(ctx, design, w, paints(ctx.canvas, String(value.tint)).accent)}
                    deps={[design, value.tint, tick]}
                  />
                }
              />
            ))}
          </Shelf>
        ))}
      </Library>
      {over && value.play === "me" && (
        <Bar side="actions">
          <Btn primary onClick={() => live.current?.restart()}>
            Retry
          </Btn>
        </Bar>
      )}
    </>
  );
}
