import { useCallback, useRef, useState } from "react";
import { Bar } from "../../lib/app/frame.jsx";
import { useUnit } from "../../lib/app/hooks.js";
import { Art, Item, Library, Shelf } from "../../lib/app/library.jsx";
import { Scene } from "../../lib/app/page.jsx";
import { paints } from "../../lib/color.js";
import { useTheme } from "../../lib/designs/pick.jsx";
import { cap, read, tag } from "../../lib/two/engine.js";
import { design, Folds, knobs, useReroll, useShots, useStatus } from "../../lib/two/parts.jsx";
import { name as called, make, SHEETS, SPEC, sketch } from "./scene.js";

/* ROWS */

export const rows = (f) => [
  ["design", tag(f)],
  ["copies", f.copies],
  ["exposed", f.exposed],
  ["one copy exposed", f.one],
  ["fills", f.fills],
  ["voids", f.voids],
  ["tile", `${f.side} by ${f.side}`],
  ["sheet", `${f.cols} by ${f.rows}`],
  ["level", f.level],
  ["corners", f.corners],
  ["edges", f.edges],
  ["euler", f.euler],
  ["dimension", f.dimension.toFixed(4)],
];

/* PANES */

export function panes(value, onChange, unit) {
  return [
    design(value, onChange, unit, SPEC),
    {
      name: "Sheet",
      live: `${value.x} by ${value.y}`,
      body: knobs(["x", "y"], value, onChange, SPEC),
    },
  ];
}

/* SHEETS */

export function Sheets({ value, onChange, unit }) {
  const tick = useTheme();
  const { base, code } = read(value);
  const deps = [unit, base, code, value.number, value.invert, value.tint, tick];
  return (
    <Library name="Sheets">
      <Shelf grid>
        {SHEETS.map(([label, x, y]) => (
          <Item
            key={label}
            label={label}
            sub={`${x} by ${y}`}
            on={value.x === x && value.y === y}
            onPick={() => onChange({ x, y })}
            art={
              <Art
                deps={[x, y, ...deps]}
                paint={(ctx, w) => sketch(ctx, unit, value, x, y, w, paints(ctx.canvas, value.tint).accent)}
              />
            }
          />
        ))}
      </Shelf>
    </Library>
  );
}

/* WIDGET */

export function Widget({ value, onChange, onReady, onExport, onStatus, units }) {
  const unit = useUnit("math", units?.math);
  const live = useRef(null);
  const now = useRef(value);
  now.current = value;
  const [have, setHave] = useState(false);
  useReroll(value, onChange, unit);
  useStatus(live, `${have}:${JSON.stringify(value)}`, onStatus, rows);
  useShots(onExport, live, have, () => called(now.current));
  const scene = useCallback((canvas, view, opts) => make(canvas, view, { ...opts, math: unit }), [unit]);
  const ready = (handle) => {
    live.current = handle;
    setHave(Boolean(handle));
    if (!handle) return onReady?.(null);
    const at = () => now.current;
    const hands = {
      seams: () => onChange({ seams: at().seams ? 0 : 1 }),
      deeper: (by) =>
        onChange({ level: Math.max(1, Math.min(cap(unit, at().number, at().x * at().y), at().level + by)) }),
      wider: (by) => onChange({ x: Math.max(1, Math.min(12, at().x + by)) }),
    };
    onReady?.({ ...handle, ...hands });
  };
  return (
    <>
      {unit && <Scene make={scene} value={value} onReady={ready} />}
      {unit && <Sheets value={value} onChange={onChange} unit={unit} />}
      <Bar side="right">
        <Folds groups={panes(value, onChange, unit)} />
      </Bar>
    </>
  );
}
