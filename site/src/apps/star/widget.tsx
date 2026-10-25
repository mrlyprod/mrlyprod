import { useCallback, useMemo, useRef, useState } from "react";
import { Bar } from "../../lib/app/frame.tsx";
import type { Readout } from "../../lib/app/hooks.ts";
import { useLatest, useReroll, useStatus, useUnit } from "../../lib/app/hooks.ts";
import type { Paint } from "../../lib/app/library.tsx";
import { Art, Item, Library, Shelf } from "../../lib/app/library.tsx";
import type { Handle, Mathy, WidgetProps } from "../../lib/app/page.tsx";
import { Scene } from "../../lib/app/page.tsx";
import type { Scene as Made, Opts, View } from "../../lib/app/run.ts";
import type { Patch } from "../../lib/app/spec.ts";
import { useTouch } from "../../lib/app/touch.ts";
import { paints } from "../../lib/color.ts";
import type { Mrly } from "../../lib/designs/engine.ts";
import { useTheme } from "../../lib/designs/pick.tsx";
import { fixed } from "../../lib/moire/parts.tsx";
import { stamp } from "../../lib/moire/sheet.ts";
import type { Rng } from "../../lib/rand.ts";
import { rng } from "../../lib/rand.ts";
import { cutter, DEEP, LEAST, MOST, STACKS, swatch, TILE } from "./engine.ts";
import type { Fault, Reading, Star } from "./scene.ts";
import { make } from "./scene.ts";

/* TYPES */

export type Told = Partial<Fault & Reading>;

export type ShelvesProps = { value: Star; onChange: (patch: Patch) => void; math?: Mrly | null };

type Props = WidgetProps<Mathy, Made> & { value: Star; unit?: Mrly | null };

const sign = (n: number) => (n > 0 ? `+${n}` : String(n));

export const drawn = (rand: Rng) => ({ layers: LEAST + 2 * Math.floor(rand() * ((MOST - LEAST) / 2 + 1)) });

export function rows(f: Told | null): Readout[] {
  if (!f) return [];
  if (f.error) return [["error", f.error]];
  return [
    ["layers", f.layers!],
    ["excess x L", fixed(f.scaled!, 6)],
    ["slope", f.slope === null ? "needs L divisible by four" : fixed(f.slope!, 6)],
    ["width law", fixed(f.target!, 6)],
    ["exact", `${f.exact} of ${f.layers} layers`],
    ["plus (ln L)/4", fixed(f.logged!, 10)],
    ["C", fixed(f.constant!, 10)],
    ["L mod 4", f.branch!],
    ["residual x L^2", fixed(f.residual!, 8)],
    ["predicted", f.predicted === null ? "odd L has none" : fixed(f.predicted!, 8)],
    ["deepest side", f.deepest!],
    ["chi_8 at the deepest", sign(f.chi!)],
    ["arm ink", fixed(f.arm!, 6)],
    ["hexagon ink", fixed(f.hexagon!, 6)],
    ["excess", fixed(f.excess!, 6)],
    ["band", f.half === 0 ? "the exact diameter x = y" : `${f.half} cells about it`],
    ["arm shares", f.ribbon!.map((one) => `${one.arm}${one.exact ? "" : ` (law ${one.law})`}`).join(" ")],
    ["ladder", f.ladder!.map((one) => `L ${one.count}: ${fixed(one.scaled, 6)}`).join(", ")],
    ["ink range", `${fixed(f.low!, 4)} to ${fixed(f.high!, 4)}`],
    ["pixels", `${f.size} by ${f.size}`],
  ];
}

export function Shelves({ value, onChange, math }: ShelvesProps) {
  const tick = useTheme();
  const cuts = useMemo(() => (math ? cutter(math) : null), [math]);
  const look = [value.ramp, value.levels, value.invert, value.tint, tick];
  const thumb =
    (layers: number): Paint =>
    (ctx, w) => {
      const size = Math.min(w, TILE);
      stamp(ctx, swatch(math!, cuts!, value, layers, size, paints(ctx.canvas, value.tint)), size);
    };
  return (
    <Library>
      {math &&
        STACKS.map(([name, list]) => (
          <Shelf key={name} name={name} grid>
            {list.map((layers) => (
              <Item
                key={layers}
                label={`L ${layers}`}
                sub={`${layers % 4} mod 4`}
                on={value.layers === layers}
                onPick={() => onChange({ layers })}
                art={<Art px={64} paint={thumb(layers)} deps={[layers, ...look]} />}
              />
            ))}
          </Shelf>
        ))}
      {math && (
        <Shelf name="Deep">
          {DEEP.map((layers) => (
            <Item
              key={layers}
              label={`L ${layers}`}
              sub="slow"
              on={value.layers === layers}
              onPick={() => onChange({ layers })}
            />
          ))}
        </Shelf>
      )}
    </Library>
  );
}

export function Widget({ value, onChange, onReady, onStatus, unit }: Props) {
  const math = useUnit("math", unit);
  const [facts, setFacts] = useState<Told | null>(null);
  const handle = useRef<Handle | null>(null);
  const change = useLatest(onChange);
  const made = useCallback(
    (canvas: HTMLCanvasElement, view: View, opts: Opts) => make(canvas, view, { ...opts, math, onFacts: setFacts }),
    [math],
  );
  const ready = (scene: Handle | null) => {
    handle.current = scene;
    onReady?.(scene);
  };
  const touch = useTouch(handle, change);
  useReroll(value.seed, (seed) => change.current(drawn(rng(seed))));
  useStatus(onStatus, rows(facts));
  return (
    <>
      {math && <Scene make={made} value={value} onReady={ready} {...touch} />}
      <Bar side="status">{!math && <span>loading</span>}</Bar>
      <Shelves value={value} onChange={onChange} math={math} />
    </>
  );
}
