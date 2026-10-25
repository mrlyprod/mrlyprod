import { useCallback, useMemo, useState } from "react";
import { useLatest, useReroll } from "../../lib/app/hooks.ts";
import type { Chip } from "../../lib/app/library.tsx";
import { Item, Library, Shelf } from "../../lib/app/library.tsx";
import type { Handle, Mathy, WidgetProps } from "../../lib/app/page.tsx";
import { Scene } from "../../lib/app/page.tsx";
import type { Opts, View } from "../../lib/app/run.ts";
import { Thumb, useTheme } from "../../lib/designs/pick.tsx";
import { codes, pickCode } from "../../lib/space/bang.ts";
import type { Knobs, Zoom, Zoomed } from "./scene.ts";
import { make, step } from "./scene.ts";

/* TYPES */

export type Asked = Knobs & { seed: number };

type Props = Pick<WidgetProps<Mathy, Zoomed>, "units" | "value" | "onChange" | "onReady"> & { value: Asked };

const POOLS: Chip[] = [
  ["all", "All"],
  ["flyable", "Flyable"],
  ["named", "Named"],
  ["family", "Family"],
];

const MANY = 12;

export function Widget({ units, value, onChange, onReady }: Props) {
  const math = units?.math;
  const [q, setQ] = useState("");
  const theme = useTheme();
  const now = useLatest(value);
  const tell = useLatest(onChange);
  const { pick, ...flown } = value;
  const pool = useMemo(() => (math ? codes(math, value.n, pick) : []), [math, value.n, pick]);
  const names = useMemo(
    () => new Map(pool.map((one): [number, string] => [one, math!.name.of(String(one), 3, 2) ?? ""])),
    [math, pool],
  );
  const want = q.trim().toLowerCase();
  const shown = want
    ? pool.filter((one) => String(one).startsWith(want) || names.get(one)!.toLowerCase().includes(want))
    : pool;
  useReroll(value.seed, (seed) => math && tell.current({ code: pickCode(math, { ...now.current, code: -1 }, seed) }));
  const scene = useCallback(
    (canvas: HTMLCanvasElement, view: View, opts: Opts) => make(canvas, view, { ...opts, ...units }),
    [units],
  );
  const ready = (handle: Handle<Zoom> | null) =>
    onReady?.(
      handle && {
        ...handle,
        step: (by: number) =>
          tell.current({ code: step(codes(math!, now.current.n, now.current.pick), now.current.code, by) }),
      },
    );
  if (!math) return null;
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
              art={<Thumb math={math} dim={3} base={2} code={String(one)} tick={theme} />}
            />
          ))}
        </Shelf>
      </Library>
    </>
  );
}
