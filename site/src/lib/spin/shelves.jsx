import { Art, Item, Shelf } from "../app/library.jsx";
import { paints } from "../color.js";
import { useTheme } from "../designs/pick.jsx";
import { edge, field, PRESETS, paint, peak, stops } from "./engine.js";
import { place, sheet } from "./scene.js";
import { read } from "./source.jsx";

const NOTES = { heatmap: "sum of layers", weave: "parity of layers", hive: "hex lattice", carpet: "base 3" };

export function stamp(ctx, { values, top, side }, look, dpr) {
  const colors = paints(ctx.canvas, look.tint);
  const layer = sheet(paint(values, top, stops(look.ramp, colors), look.levels, look.invert), side);
  place(ctx, layer, { x: side / 2, y: side / 2, r: side / 2 - dpr }, edge(colors), dpr);
}

export const toMoire = (value, preset) => read({ ...read(value), source: "moire", preset });

export function Moires({ value, onChange, math }) {
  const src = read(value);
  const tick = useTheme();
  const look = { ramp: value.ramp, levels: value.levels, invert: value.invert, tint: value.tint };
  const art = (name) => (
    <Art
      paint={(ctx, w, _h, dpr) => {
        const values = field(math, name, src.limit, w);
        stamp(ctx, { values, top: peak(values) || 1, side: w }, look, dpr);
      }}
      deps={[math, name, src.limit, look.ramp, look.levels, look.invert, look.tint, tick]}
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
          art={math ? art(name) : undefined}
        />
      ))}
    </Shelf>
  );
}
