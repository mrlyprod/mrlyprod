import { useCallback, useMemo, useRef, useState } from "react";
import { paints } from "../../net/sdk/color.ts";
import { Bar } from "../../net/sdk/frame.tsx";
import type { Readout } from "../../net/sdk/hooks.ts";
import { useLatest, useReroll, useStatus, useUnit } from "../../net/sdk/hooks.ts";
import type { Paint } from "../../net/sdk/library.tsx";
import { Art, Item, Library, Shelf } from "../../net/sdk/library.tsx";
import type { Handle, Mathy, WidgetProps } from "../../net/sdk/page.tsx";
import { Scene } from "../../net/sdk/page.tsx";
import type { Rng } from "../../net/sdk/rand.ts";
import { rng } from "../../net/sdk/rand.ts";
import type { Scene as Made, Opts, View } from "../../net/sdk/run.ts";
import type { Patch } from "../../net/sdk/spec.ts";
import { useTouch } from "../../net/sdk/touch.ts";
import type { Mrly } from "../shared/designs/engine.ts";
import { useTheme } from "../shared/designs/pick.tsx";
import { fixed } from "../shared/moire/parts.tsx";
import { stamp } from "../shared/moire/sheet.ts";
import { cutter, DEEP, LEAST, MOST, STACKS, swatch, TILE } from "./engine.ts";
import type { Facts, Star } from "./scene.ts";
import { make } from "./scene.ts";

/* TYPES */

export type ShelvesProps = { value: Star; onChange: (patch: Patch) => void; math?: Mrly | null };

type Props = WidgetProps<Mathy, Made> & { value: Star; unit?: Mrly | null };

const sign = (n: number) => (n > 0 ? `+${n}` : String(n));

export const drawn = (rand: Rng) => ({ layers: LEAST + 2 * Math.floor(rand() * ((MOST - LEAST) / 2 + 1)) });

export function rows(f: Facts | null): Readout[] {
  if (!f) return [];
  if ("error" in f) return [["error", f.error]];
  return [
    ["layers", f.layers],
    ["excess x L", fixed(f.scaled, 6)],
    ["slope", f.slope === null ? "needs L divisible by four" : fixed(f.slope, 6)],
    ["width law", fixed(f.target, 6)],
    ["exact", `${f.exact} of ${f.layers} layers`],
    ["plus (ln L)/4", fixed(f.logged, 10)],
    ["C", fixed(f.constant, 10)],
    ["L mod 4", f.branch],
    ["residual x L^2", fixed(f.residual, 8)],
    ["predicted", f.predicted === null ? "odd L has none" : fixed(f.predicted, 8)],
    ["deepest side", f.deepest],
    ["chi_8 at the deepest", sign(f.chi)],
    ["arm ink", fixed(f.arm, 6)],
    ["hexagon ink", fixed(f.hexagon, 6)],
    ["excess", fixed(f.excess, 6)],
    ["band", f.half === 0 ? "the exact diameter x = y" : `${f.half} cells about it`],
    ["arm shares", f.ribbon.map((one) => `${one.arm}${one.exact ? "" : ` (law ${one.law})`}`).join(" ")],
    ["ladder", f.ladder.map((one) => `L ${one.count}: ${fixed(one.scaled, 6)}`).join(", ")],
    ["ink range", `${fixed(f.low, 4)} to ${fixed(f.high, 4)}`],
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
  const [facts, setFacts] = useState<Facts | null>(null);
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
