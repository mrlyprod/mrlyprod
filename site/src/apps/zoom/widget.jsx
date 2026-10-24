import { useCallback, useMemo, useState } from "react";
import { useLatest, useReroll } from "../../lib/app/hooks.js";
import { Item, Library, Shelf } from "../../lib/app/library.jsx";
import { Scene } from "../../lib/app/page.jsx";
import { Thumb, useTheme } from "../../lib/designs/pick.jsx";
import { codes, pickCode } from "../../lib/space/bang.js";
import { make, step } from "./scene.js";

const POOLS = [
  ["all", "All"],
  ["flyable", "Flyable"],
  ["named", "Named"],
  ["family", "Family"],
];

const MANY = 12;

export function Widget({ units, value, onChange, onReady }) {
  const math = units?.math;
  const [q, setQ] = useState("");
  const theme = useTheme();
  const now = useLatest(value);
  const tell = useLatest(onChange);
  const { pick, ...flown } = value;
  const pool = useMemo(() => (math ? codes(math, value.n, pick) : []), [math, value.n, pick]);
  const names = useMemo(() => new Map(pool.map((one) => [one, math.name.of(String(one), 3, 2) ?? ""])), [math, pool]);
  const want = q.trim().toLowerCase();
  const shown = want
    ? pool.filter((one) => String(one).startsWith(want) || names.get(one).toLowerCase().includes(want))
    : pool;
  useReroll(value.seed, (seed) => math && tell.current({ code: pickCode(math, { ...now.current, code: -1 }, seed) }));
  const scene = useCallback((canvas, view, opts) => make(canvas, view, { ...opts, ...units }), [units]);
  const ready = (handle) =>
    onReady?.(
      handle && {
        ...handle,
        step: (by) => tell.current({ code: step(codes(math, now.current.n, now.current.pick), now.current.code, by) }),
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
