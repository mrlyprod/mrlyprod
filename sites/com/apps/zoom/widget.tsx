import type { Mrly } from "@mrlyprod/ui/kit/designs/engine.ts";
import { Thumb, useTheme } from "@mrlyprod/ui/kit/designs/pick.tsx";
import { useLatest } from "@mrlyprod/ui/kit/hooks.ts";
import type { Chip } from "@mrlyprod/ui/kit/library.tsx";
import { Item, Library, Shelf } from "@mrlyprod/ui/kit/library.tsx";
import type { Handle, WidgetProps } from "@mrlyprod/ui/kit/page.tsx";
import { Scene } from "@mrlyprod/ui/kit/page.tsx";
import type { Opts, View } from "@mrlyprod/ui/kit/run.ts";
import { useCallback, useMemo, useState } from "react";
import { codes } from "../shared/space/bang.ts";
import type { Knobs, Zoom, Zoomed } from "./scene.ts";
import { make } from "./scene.ts";

export type Asked = Knobs & { seed: number };

type Props = Pick<WidgetProps<Mrly, Zoomed>, "units" | "value" | "onChange" | "onReady"> & { value: Asked };

const POOLS: Chip[] = [
  ["all", "All"],
  ["flyable", "Flyable"],
  ["named", "Named"],
  ["family", "Family"],
];

const MANY = 12;

export function Widget({ units, value, onChange, onReady }: Props) {
  const mrly = units;
  const [q, setQ] = useState("");
  const theme = useTheme();
  const tell = useLatest(onChange);
  const { pick, ...flown } = value;
  const pool = useMemo(() => (mrly ? codes(mrly, value.n, pick) : []), [mrly, value.n, pick]);
  const names = useMemo(
    () => new Map(pool.map((one): [number, string] => [one, mrly!.math.name.of(String(one), 3, 2) ?? ""])),
    [mrly, pool],
  );
  const want = q.trim().toLowerCase();
  const shown = want
    ? pool.filter((one) => String(one).startsWith(want) || names.get(one)!.toLowerCase().includes(want))
    : pool;
  const scene = useCallback(
    (canvas: HTMLCanvasElement, view: View, opts: Opts) => make(canvas, view, { ...opts, mrly: units }),
    [units],
  );
  const ready = (handle: Handle<Zoom> | null) => onReady?.(handle);
  if (!mrly) return null;
  return (
    <>
      <Scene make={scene} value={flown} onReady={ready} />
      <Library
        name="Shapes"
        find={q}
        onFind={pool.length > MANY ? setQ : undefined}
        chips={POOLS}
        chip={pick}
        onChip={(next) => tell.current({ pick: next })}
      >
        <Shelf name={`${shown.length} of 255`} grid>
          {shown.map((one) => (
            <Item
              key={one}
              label={String(one)}
              sub={names.get(one)}
              on={one === value.code}
              onPick={() => tell.current({ code: one })}
              art={<Thumb mrly={mrly} dim={3} base={2} code={String(one)} tick={theme} />}
            />
          ))}
        </Shelf>
      </Library>
    </>
  );
}
