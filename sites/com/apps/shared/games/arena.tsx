import { Btn } from "@mrlyprod/ui/kit/controls.tsx";
import { Bar } from "@mrlyprod/ui/kit/frame.tsx";
import { useStatus } from "@mrlyprod/ui/kit/hooks.ts";
import type { Handle, WidgetProps } from "@mrlyprod/ui/kit/page.tsx";
import { Scene } from "@mrlyprod/ui/kit/page.tsx";
import type { Opts, View } from "@mrlyprod/ui/kit/run.ts";
import type { Units } from "@mrlyprod/ui/kit/spec.ts";
import { useTouch } from "@mrlyprod/ui/kit/touch.ts";
import type { MouseEvent, ReactNode } from "react";
import { useCallback, useRef, useState } from "react";
import type { Given, Played, Told } from "./stage.ts";

export type ArenaProps<S extends Played, T extends Told> = Partial<
  Pick<WidgetProps<Units, S>, "onReady" | "onStatus">
> &
  Pick<WidgetProps<Units, S>, "value"> & {
    make: (canvas: HTMLCanvasElement, view: View, opts: Given<T>) => S;
    more?: (told: T, scene: Handle<S> | null) => ReactNode;
  };

const NOBODY = { current: null };

export function Arena<S extends Played, T extends Told = Told>({
  make,
  more,
  value,
  onReady,
  onStatus,
}: ArenaProps<S, T>) {
  const live = useRef<Handle<S> | null>(null);
  const [told, tell] = useState<T>({ rows: [], over: false } as unknown as T);
  const game = useCallback(
    (canvas: HTMLCanvasElement, view: View, opts: Opts) => make(canvas, view, { ...opts, tell }),
    [make],
  );
  const hands = useTouch(live, NOBODY);
  useStatus(onStatus, told.rows);
  const ready = (handle: Handle<S> | null) => {
    live.current = handle;
    onReady?.(handle);
  };
  const menu = (e: MouseEvent<HTMLCanvasElement>) => {
    const alt = live.current?.alt;
    if (!alt) return;
    e.preventDefault();
    const box = e.currentTarget.getBoundingClientRect();
    alt((e.clientX - box.left) / box.width, (e.clientY - box.top) / box.height);
  };
  return (
    <>
      <Scene make={game} value={value} onReady={ready} onContextMenu={menu} {...hands} />
      {more?.(told, live.current)}
      {told.over && (
        <Bar side="actions">
          <Btn primary onClick={() => live.current?.restart()}>
            Again
          </Btn>
        </Bar>
      )}
    </>
  );
}
