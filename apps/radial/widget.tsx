import { useCallback, useEffect, useRef, useState } from "react";
import { Bar } from "../../net/sdk/frame.tsx";
import type { Readout } from "../../net/sdk/hooks.ts";
import { useLatest } from "../../net/sdk/hooks.ts";
import { Art, Item, Library, Shelf } from "../../net/sdk/library.tsx";
import type { Handle, Mathy, WidgetProps } from "../../net/sdk/page.tsx";
import { Scene } from "../../net/sdk/page.tsx";
import type { Opts, View } from "../../net/sdk/run.ts";
import type { Patch, Raw, Scalar } from "../../net/sdk/spec.ts";
import type { Mrly } from "../shared/designs/engine.ts";
import { useTheme } from "../shared/designs/pick.tsx";
import type { Stack } from "../shared/spin/engine.ts";
import { BLENDS, degrees, key, sketch, study, title } from "../shared/spin/engine.ts";
import type { Look } from "../shared/spin/scene.ts";
import { Moires, stamp } from "../shared/spin/shelves.tsx";
import { read, Source } from "../shared/spin/source.tsx";
import type { Facts, Orbit, Radial } from "./scene.ts";
import { make, STACKS, wrap } from "./scene.ts";

/* TYPES */

export type RosettesProps = { value: Stack & Look & Raw; onChange: (patch: Patch) => void; math?: Mrly | null };

type Props = WidgetProps<Mathy> & { value: Radial & { seed: number } };

const pct = (x: number) => `${(x * 100).toFixed(1)}%`;

export function rows(facts: Facts | null): Readout[] {
  if (!facts) return [];
  return [
    ["name", title(facts)],
    ["side", String(facts.side)],
    ["copies", String(facts.copies)],
    ["step", `${facts.step}°`],
    ["rotation order", facts.order ? String(facts.order) : "round"],
    ["petals", String(facts.petals)],
    ["order 0", pct(facts.share)],
    ...facts.leading.map((one): Readout => [`order ${one.order}`, pct(one.share)]),
  ];
}

export const lit = (value: Raw, patch: Orbit) =>
  Number(value.full) === patch.full &&
  Number(value.copies) === patch.copies &&
  (patch.full === 1 || Number(value.step) === patch.step);

export function Rosettes({ value, onChange, math }: RosettesProps) {
  const tick = useTheme();
  const src = read(value);
  const thumb = (patch: Partial<Stack>, ...live: Scalar[]) => (
    <Art
      paint={(ctx, w, _h, dpr) =>
        stamp(ctx, sketch(math!, study(math!, src), { ...value, ...patch, samples: 1 }, w), value, dpr)
      }
      deps={[math, key(src), JSON.stringify(patch), value.ramp, value.levels, value.invert, value.tint, tick, ...live]}
    />
  );
  return (
    <>
      <Shelf name="Blends" grid>
        {BLENDS.map(([blend, text]) => (
          <Item
            key={blend}
            label={text}
            on={value.blend === blend}
            onPick={() => onChange({ blend })}
            art={math ? thumb({ blend }, value.copies, value.full, value.step) : undefined}
          />
        ))}
      </Shelf>
      <Shelf name="Stacks" grid>
        {STACKS.map(([text, patch]) => (
          <Item
            key={text}
            label={text}
            sub={`${patch.copies} x ${degrees({ ...value, ...patch })}°`}
            on={lit(value, patch)}
            onPick={() => onChange(patch)}
            art={math ? thumb(patch, value.blend) : undefined}
          />
        ))}
      </Shelf>
    </>
  );
}

export function Widget({ value, onChange, onReady, onStatus, units }: Props) {
  const unit = units?.math;
  const [facts, setFacts] = useState<Facts | null>(null);
  const now = useLatest(value);
  const change = useLatest(onChange);
  const was = useRef(value);
  useEffect(() => {
    const before = was.current;
    was.current = value;
    if (Number(before.full) === 1 && Number(value.full) !== 1 && before.step === value.step)
      change.current({ step: 360 / value.copies });
  }, [value.full, value.step]);
  useEffect(() => onStatus?.(rows(facts)), [facts]);
  const scene = useCallback(
    (canvas: HTMLCanvasElement, view: View, opts: Opts) =>
      make(canvas, view, { ...opts, math: unit, onFacts: setFacts }),
    [unit],
  );
  const ready = (handle: Handle | null) => onReady?.(handle);
  return (
    <>
      {unit && (
        <Scene
          make={scene}
          value={value}
          onReady={ready}
          onClick={() => change.current({ copies: wrap(now.current.copies, 1) })}
        />
      )}
      <Library>
        <Rosettes value={value} onChange={onChange} math={unit} />
        <Moires value={value} onChange={onChange} math={unit} />
      </Library>
      <Bar side="right">
        <Source value={value} onChange={onChange} math={unit} />
      </Bar>
    </>
  );
}
