import { useState } from "react";
import { Art, Item, Library, Shelf } from "../../lib/app/library.jsx";
import { marks } from "../../lib/app/svg.js";
import { paints } from "../../lib/color.js";
import { useTheme } from "../../lib/designs/pick.jsx";
import { SLOT } from "./engine.js";

const ALL = "all";

const FLAG = "flag";

const MARGIN = 2;

export function ink(ctx, glyph, w, h, color) {
  const unit = Math.max(1, Math.floor(w / (SLOT + MARGIN)));
  const x = Math.floor((w - SLOT * unit) / 2) + Math.floor((SLOT - glyph.cols) / 2) * unit;
  const y = Math.floor((h - SLOT * unit) / 2) + Math.floor((SLOT - glyph.rows) / 2) * unit;
  ctx.setTransform(1, 0, 0, 1, 0, 0);
  ctx.clearRect(0, 0, w, h);
  ctx.fillStyle = color;
  for (const [col, row] of marks(glyph.frames.at(-1) ?? [], glyph.cols))
    ctx.fillRect(x + col * unit, y + row * unit, unit, unit);
}

export function shelved(kinds, kind, query) {
  const want = query.trim().toLowerCase();
  const has = (glyph) => !want || glyph.char.toLowerCase() === want || (want.length > 1 && glyph.name.includes(want));
  const shown = (one, glyph) => (kind === ALL || kind === one.id || (kind === FLAG && glyph.flag)) && has(glyph);
  return kinds
    .map((one) => ({ id: one.id, name: one.name, glyphs: one.glyphs.filter((glyph) => shown(one, glyph)) }))
    .filter((one) => one.glyphs.length > 0);
}

export function Pens({ kinds, tint, onPick }) {
  const [kind, setKind] = useState(ALL);
  const [query, setQuery] = useState("");
  const theme = useTheme();
  const chips = [[ALL, "All"], ...kinds.map((one) => [one.id, one.name]), [FLAG, "Flagged"]];
  return (
    <Library
      name="Pens"
      find={query}
      onFind={setQuery}
      hint="A, b, 7 or a name"
      chips={chips}
      chip={kind}
      onChip={setKind}
    >
      {shelved(kinds, kind, query).map((one) => (
        <Shelf key={one.id} name={kind === FLAG ? "Flagged" : one.name} grid>
          {one.glyphs.map((glyph) => (
            <Item
              key={glyph.char}
              label={glyph.label}
              sub={`${glyph.strokes} of ${glyph.floor}`}
              title={`${glyph.name}, ${glyph.strokes} strokes of a floor of ${glyph.floor}${glyph.flag ? `, ${glyph.flag}` : ""}: add it to the text`}
              onPick={() => onPick(glyph.char)}
              art={
                <Art
                  paint={(ctx, w, h) => ink(ctx, glyph, w, h, paints(ctx.canvas, tint).accent)}
                  deps={[glyph.char, tint, theme]}
                />
              }
            />
          ))}
        </Shelf>
      ))}
    </Library>
  );
}
