import { paints } from "@mrlyprod/ui/kit/color.ts";
import type { Mrly } from "@mrlyprod/ui/kit/designs/engine.ts";
import { useTheme } from "@mrlyprod/ui/kit/designs/pick.tsx";
import { Bar } from "@mrlyprod/ui/kit/frame.tsx";
import type { Readout } from "@mrlyprod/ui/kit/hooks.ts";
import { useLatest, useStatus, useUnit } from "@mrlyprod/ui/kit/hooks.ts";
import type { Paint } from "@mrlyprod/ui/kit/library.tsx";
import { Art, Item, Library, Shelf } from "@mrlyprod/ui/kit/library.tsx";
import type { Handle, Mathy, WidgetProps } from "@mrlyprod/ui/kit/page.tsx";
import { Scene } from "@mrlyprod/ui/kit/page.tsx";
import type { Opts, View } from "@mrlyprod/ui/kit/run.ts";
import type { Patch } from "@mrlyprod/ui/kit/spec.ts";
import { useTouch } from "@mrlyprod/ui/kit/touch.ts";
import { useCallback, useRef, useState } from "react";
import type { Preset } from "../shared/moire/engine.ts";
import { matrix, presetOf, presets, swatch } from "../shared/moire/engine.ts";
import { stamp } from "../shared/moire/sheet.ts";
import type { Facts, Moire, Player } from "./scene.ts";
import { make } from "./scene.ts";

export type ShelvesProps = { value: Moire; onChange: (patch: Patch) => void; math?: Mrly | null };

type Props = WidgetProps<Mathy, Player> & { value: Moire; unit?: Mrly | null };

const exact = (r: number) => (r === 0 ? "0" : r.toFixed(9));

export function rows(f: Facts | null): Readout[] {
  if (!f) return [];
  if ("error" in f) return [["error", f.error]];
  if (f.view === "field") {
    return [
      ["design", f.name ? `${f.name} ${f.code}` : `code ${f.code}`],
      ["layers", f.layers],
      ["scales", f.scales.join(" ")],
      ["pixels", `${f.size} by ${f.size}`],
    ];
  }
  return [
    ["scale m", f.row || "none"],
    ["scale n", f.mate || "none earlier"],
    ["r", f.r === null ? "none" : exact(f.r)],
    [
      "row of m",
      !f.row ? "none" : f.prime ? "clear against every earlier scale" : `largest ${exact(f.max)} at scale ${f.at}`,
    ],
    ["matrix", `${f.k} by ${f.k}`],
  ];
}

const title = (name: string) => name.charAt(0).toUpperCase() + name.slice(1);

export function split(n: number) {
  for (let d = 2; d * d <= n; d++) if (n % d === 0) return `${d} x ${n / d}`;
  return "prime";
}

export function Shelves({ value, onChange, math }: ShelvesProps) {
  const tick = useTheme();
  const list = math ? presets(math) : [];
  const preset = presetOf(list, value);
  const look = [value.limit, value.ramp, value.levels, value.invert, value.tint, tick];
  const thumb =
    (one: Omit<Preset, "name">): Paint =>
    (ctx, w) =>
      stamp(ctx, swatch(math!, { ...value, ...one }, w, paints(ctx.canvas, value.tint)), w);
  return (
    <Library>
      {math && value.view === "matrix" && (
        <Shelf name="Scales">
          {matrix(math, value.limit).scales.map((m) => (
            <Item
              key={m}
              label={`m ${m}`}
              sub={split(m)}
              on={value.scale === m}
              onPick={() => onChange({ scale: m })}
            />
          ))}
        </Shelf>
      )}
      {math && value.view !== "matrix" && (
        <Shelf name="Presets" grid>
          {list.map(({ name, ...one }) => (
            <Item
              key={name}
              label={title(name)}
              on={preset === name}
              onPick={() => onChange(one)}
              art={<Art px={64} paint={thumb(one)} deps={[name, ...look]} />}
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
  const handle = useRef<Handle<Player> | null>(null);
  const change = useLatest(onChange);
  const made = useCallback(
    (canvas: HTMLCanvasElement, view: View, opts: Opts) =>
      make(canvas, view, { ...opts, math, onFacts: setFacts, onLimit: (limit) => change.current({ limit }) }),
    [math],
  );
  const ready = (scene: Handle<Player> | null) => {
    handle.current = scene;
    onReady?.(scene);
  };
  const touch = useTouch(handle, change);
  useStatus(onStatus, rows(facts));
  return (
    <>
      {math && <Scene make={made} value={value} onReady={ready} {...touch} />}
      <Bar side="status">{!math && <span>loading</span>}</Bar>
      <Shelves value={value} onChange={onChange} math={math} />
    </>
  );
}
