import { Art, Item, Library, Shelf } from "@mrlyprod/ui/kit/library.tsx";
import type { WidgetProps } from "@mrlyprod/ui/kit/page.tsx";
import { Scene } from "@mrlyprod/ui/kit/page.tsx";
import type { Opts, View } from "@mrlyprod/ui/kit/run.ts";
import type { Units } from "@mrlyprod/ui/kit/spec.ts";
import { useCallback, useMemo, useRef } from "react";
import type { World } from "../shared/space/exo.ts";
import { globe } from "../shared/space/globe.ts";
import type { Named } from "../shared/space/worlds.ts";
import { NAMES, world } from "../shared/space/worlds.ts";
import type { Knobs, Planets } from "./scene.ts";
import { make, named } from "./scene.ts";

export type Asked = Knobs & { seed: number };

type Props = Pick<WidgetProps<Units, Planets>, "value" | "onChange" | "onReady"> & { value: Asked };

const tag = (params: World) => {
  if (params.ring) return "rings";
  const n = params.moons.length;
  return n ? `${n} moon${n > 1 ? "s" : ""}` : "";
};

export function Widget({ value, onChange, onReady }: Props) {
  const now = useRef(value);
  now.current = value;
  const made = useCallback(
    (canvas: HTMLCanvasElement, view: View, opts: Opts) => make(canvas, view, { ...opts, live: now }),
    [],
  );
  const { world: current, moons, rings, seed } = value;
  const close = useMemo(
    () => NAMES.filter((name) => name !== "exo").map((name): [string, Named] => [name, world(name, 0)]),
    [],
  );
  const far = useMemo(() => world("exo", seed), [seed]);
  const tile = (name: string, params: World, deps: readonly unknown[]) => (
    <Item
      key={name}
      label={named(name)}
      sub={name === "exo" ? far.kind : tag(params)}
      on={name === current}
      onPick={() => onChange({ world: name })}
      art={<Art paint={(ctx, w, h) => globe(ctx, w, h, params)} deps={deps} />}
    />
  );
  return (
    <>
      <Scene make={made} value={{ world: current, moons, rings, seed }} onReady={onReady} />
      <Library name="Worlds">
        <Shelf name="Solar system" grid>
          {close.map(([name, params]) => tile(name, params, [name]))}
        </Shelf>
        <Shelf name="Beyond" grid>
          {tile("exo", far, [seed])}
        </Shelf>
      </Library>
    </>
  );
}
