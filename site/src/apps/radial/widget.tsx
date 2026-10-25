import { useCallback, useEffect, useRef, useState } from "react";
import { Bar } from "../../lib/app/frame.tsx";
import type { Readout } from "../../lib/app/hooks.ts";
import { useLatest } from "../../lib/app/hooks.ts";
import { Art, Item, Library, Shelf } from "../../lib/app/library.tsx";
import type { Handle, Mathy, WidgetProps } from "../../lib/app/page.tsx";
import { Scene } from "../../lib/app/page.tsx";
import type { Opts, View } from "../../lib/app/run.ts";
import type { Patch, Raw, Scalar } from "../../lib/app/spec.ts";
import type { Mrly } from "../../lib/designs/engine.ts";
import { useTheme } from "../../lib/designs/pick.tsx";
import type { Rng } from "../../lib/rand.ts";
import type { Src, Stack } from "../../lib/spin/engine.ts";
import { BLENDS, COPIES, degrees, drawn, key, sketch, study, title } from "../../lib/spin/engine.ts";
import type { Look } from "../../lib/spin/scene.ts";
import { Moires, stamp } from "../../lib/spin/shelves.tsx";
import { read, Source, useReroll } from "../../lib/spin/source.tsx";
import type { Facts, Orbit, Radial, Rosette } from "./scene.ts";
import { make, STACKS } from "./scene.ts";

/* TYPES */

export type Rolled = Partial<Src> & Pick<Stack, "copies" | "blend"> & { full: number };

export type RosettesProps = { value: Stack & Look & Raw; onChange: (patch: Patch) => void; math?: Mrly | null };

type Props = WidgetProps<Mathy, Rosette> & { value: Radial & { seed: number } };

const pct = (x: number) => `${(x * 100).toFixed(1)}%`;

const BLEND = BLENDS.map(([one]) => one).filter((one) => one !== "meet");

export const wrap = (copies: number, by: number) => ((copies - 1 + by + COPIES) % COPIES) + 1;

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

export function rolled(math: Mrly, value: Raw, rand: Rng): Rolled {
  return {
    ...drawn(math, read(value), rand),
    copies: 1 + Math.floor(rand() * COPIES),
    blend: BLEND[Math.floor(rand() * BLEND.length)],
    full: 1,
  };
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
  useReroll(value.seed, (rand) => unit && change.current(rolled(unit, now.current, rand)));
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
  const copies = (by: number, loop: boolean) => {
    const at = now.current.copies;
    change.current({ copies: loop ? wrap(at, by) : Math.min(COPIES, Math.max(1, at + by)) });
  };
  const ready = (handle: Handle | null) => {
    const hands = {
      whole: () => change.current({ full: 1 }),
      copies: (by: number) => copies(by, false),
      step: (by: number) => {
        const { full: whole, step, copies: n } = now.current;
        change.current({ full: 0, step: Math.min(360, Math.max(0, (Number(whole) === 1 ? 360 / n : step) + by)) });
      },
    };
    onReady?.(handle && { ...handle, ...hands });
  };
  return (
    <>
      {unit && <Scene make={scene} value={value} onReady={ready} onClick={() => copies(1, true)} />}
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
