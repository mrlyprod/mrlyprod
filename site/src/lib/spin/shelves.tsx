import { Art, Item, Shelf } from "../app/library.tsx";
import type { Raw } from "../app/spec.ts";
import { paints } from "../color.ts";
import type { Mrly } from "../designs/engine.ts";
import { useTheme } from "../designs/pick.tsx";
import type { Raster, Src } from "./engine.ts";
import { edge, field, PRESETS, paint, peak, stops } from "./engine.ts";
import type { Look } from "./scene.ts";
import { place, sheet } from "./scene.ts";
import { read } from "./source.tsx";

/* TYPES */

export type MoiresProps = { value: Look & Raw; onChange: (value: Src) => void; math?: Mrly | null };

const NOTES: Partial<Record<string, string>> = {
  heatmap: "sum of layers",
  weave: "parity of layers",
  hive: "hex lattice",
  carpet: "base 3",
};

export function stamp(ctx: CanvasRenderingContext2D, { values, top, side }: Raster, look: Look, dpr: number): void {
  const colors = paints(ctx.canvas, look.tint);
  const layer = sheet(paint(values, top, stops(look.ramp, colors), look.levels, look.invert), side);
  place(ctx, layer, { x: side / 2, y: side / 2, r: side / 2 - dpr }, edge(colors), dpr);
}

export const toMoire = (value: Raw, preset: string) => read({ ...read(value), source: "moire", preset });

export function Moires({ value, onChange, math }: MoiresProps) {
  const src = read(value);
  const tick = useTheme();
  const look = { ramp: value.ramp, levels: value.levels, invert: value.invert, tint: value.tint };
  const art = (unit: Mrly, name: string) => (
    <Art
      paint={(ctx, w, _h, dpr) => {
        const values = field(unit, name, src.limit, w);
        stamp(ctx, { values, top: peak(values) || 1, side: w }, look, dpr);
      }}
      deps={[unit, name, src.limit, look.ramp, look.levels, look.invert, look.tint, tick]}
    />
  );
  return (
    <Shelf name="Moire" grid>
      {PRESETS.map((name) => (
        <Item
          key={name}
          label={name}
          sub={NOTES[name]}
          on={src.source === "moire" && src.preset === name}
          onPick={() => onChange(toMoire(value, name))}
          art={math ? art(math, name) : undefined}
        />
      ))}
    </Shelf>
  );
}
