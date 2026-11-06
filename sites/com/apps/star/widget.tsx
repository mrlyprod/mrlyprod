import { paints } from "@mrlyprod/ui/kit/color.ts";
import type { Mrly } from "@mrlyprod/ui/kit/designs/engine.ts";
import { CRATES } from "@mrlyprod/ui/kit/designs/engine.ts";
import { useTheme } from "@mrlyprod/ui/kit/designs/pick.tsx";
import { Bar } from "@mrlyprod/ui/kit/frame.tsx";
import type { Readout } from "@mrlyprod/ui/kit/hooks.ts";
import { useCrates, useLatest, useStatus } from "@mrlyprod/ui/kit/hooks.ts";
import type { Paint } from "@mrlyprod/ui/kit/library.tsx";
import { Art, Item, Library, Shelf } from "@mrlyprod/ui/kit/library.tsx";
import type { Handle, WidgetProps } from "@mrlyprod/ui/kit/page.tsx";
import { Scene } from "@mrlyprod/ui/kit/page.tsx";
import type { Scene as Made, Opts, View } from "@mrlyprod/ui/kit/run.ts";
import type { Patch } from "@mrlyprod/ui/kit/spec.ts";
import { useTouch } from "@mrlyprod/ui/kit/touch.ts";
import { useCallback, useMemo, useRef, useState } from "react";
import { fixed } from "../shared/moire/parts.tsx";
import { stamp } from "../shared/moire/sheet.ts";
import { cutter, DEEP, STACKS, swatch, TILE } from "./engine.ts";
import type { Facts, Star } from "./scene.ts";
import { make } from "./scene.ts";

export type ShelvesProps = { value: Star; onChange: (patch: Patch) => void; mrly?: Mrly | null };

type Props = WidgetProps<Mrly, Made> & { value: Star; unit?: Mrly | null };

const sign = (n: number) => (n > 0 ? `+${n}` : String(n));

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

export function Shelves({ value, onChange, mrly }: ShelvesProps) {
  const tick = useTheme();
  const cuts = useMemo(() => (mrly ? cutter(mrly) : null), [mrly]);
  const look = [value.ramp, value.levels, value.invert, value.tint, tick];
  const thumb =
    (layers: number): Paint =>
    (ctx, w) => {
      const size = Math.min(w, TILE);
      stamp(ctx, swatch(mrly!, cuts!, value, layers, size, paints(ctx.canvas, value.tint)), size);
    };
  return (
    <Library>
      {mrly &&
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
      {mrly && (
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
  const mrly = useCrates(CRATES, unit);
  const [facts, setFacts] = useState<Facts | null>(null);
  const handle = useRef<Handle | null>(null);
  const change = useLatest(onChange);
  const made = useCallback(
    (canvas: HTMLCanvasElement, view: View, opts: Opts) => make(canvas, view, { ...opts, mrly, onFacts: setFacts }),
    [mrly],
  );
  const ready = (scene: Handle | null) => {
    handle.current = scene;
    onReady?.(scene);
  };
  const touch = useTouch(handle, change);
  useStatus(onStatus, rows(facts));
  return (
    <>
      {mrly && <Scene make={made} value={value} onReady={ready} {...touch} />}
      <Bar side="status">{!mrly && <span>loading</span>}</Bar>
      <Shelves value={value} onChange={onChange} mrly={mrly} />
    </>
  );
}
