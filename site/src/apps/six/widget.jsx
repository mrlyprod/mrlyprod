import { useCallback, useEffect, useRef, useState } from "react";
import { text } from "../../lib/app/export.js";
import { useReroll } from "../../lib/app/hooks.js";
import { Art, Item, Library, Shelf } from "../../lib/app/library.jsx";
import { Scene } from "../../lib/app/page.jsx";
import { paints } from "../../lib/color.js";
import { read, useTheme } from "../../lib/designs/pick.jsx";
import { copies, holds, PRESETS, reroll, sample, titled, VIEWS } from "./engine.js";
import { make, name, sketch } from "./scene.js";

const ORDER = VIEWS.map(([key]) => key);

const by = ([w, h]) => `${w} x ${h}`;

export function rows(f) {
  if (!f) return [];
  return [
    ["design", f.name ? `${f.name} ${f.code}` : f.title],
    ["side", f.side],
    ["level", `${f.level} of ${f.top}`],
    ["copies", f.copies],
    ["filled", f.fills],
    ["exposed", f.exposed],
    ["one copy exposed", f.alone],
    ["tile", by(f.tile)],
    ["sheet", by(f.sheet)],
    ["triangles", f.triangles],
    ["empty", f.voids],
    ["pieces", f.pieces],
    ["holes", f.holes],
    ["corners", f.corners],
    ["edges", f.edges],
    ["euler", f.euler],
  ].map(([name, v]) => [name, String(v)]);
}

export const label = (f) => `${f ? f.name || f.title : "a cube design"} repeated into a sheet`;

export function Sheets({ math, value, tick, onChange }) {
  const paint = (one) => (ctx, w, h) =>
    sketch(ctx, w, h, sample(math, { ...value, ...read(value), ...one }), paints(ctx.canvas, value.tint));
  return (
    <Shelf grid>
      {PRESETS.map((one) => (
        <Item
          key={titled(one)}
          label={titled(one)}
          sub={`${copies(math, { ...value, ...one })} copies`}
          on={holds(value, one)}
          onPick={() => onChange(one)}
          art={
            <Art
              paint={paint(one)}
              deps={[
                tick,
                value.code,
                value.base,
                value.number,
                value.view,
                value.turn,
                value.invert,
                value.overhang,
                value.tint,
              ]}
            />
          }
        />
      ))}
    </Shelf>
  );
}

export function Widget({ value, onChange, onReady, onExport, onStatus, units }) {
  const unit = units?.math;
  const tick = useTheme();
  const [facts, setFacts] = useState(null);
  const live = useRef(null);
  const now = useRef(value);
  const change = useRef(onChange);
  now.current = value;
  change.current = onChange;
  useReroll(value.seed, (seed) => unit && onChange({ code: reroll(unit, read(value).base, seed) }));
  useEffect(() => onStatus?.(rows(facts)), [facts]);
  const scene = useCallback(
    (canvas, view, opts) => make(canvas, view, { ...opts, math: unit, onFacts: setFacts }),
    [unit],
  );
  const hands = {
    crop: () => change.current({ overhang: now.current.overhang === 1 ? 0 : 1 }),
    view: (step) =>
      change.current({ view: ORDER[(ORDER.indexOf(now.current.view) + step + ORDER.length) % ORDER.length] }),
    deeper: (step) => change.current({ level: now.current.level + step }),
  };
  const ready = (handle) => {
    live.current = handle;
    onReady?.(handle && { ...handle, ...hands });
  };
  const keep = (kind, tail) => () => {
    const picture = live.current?.[kind]?.();
    if (picture) text(picture, `${name(now.current, units)}${tail}`, "svg");
  };
  const have = Boolean(unit);
  useEffect(
    () =>
      onExport?.(
        have
          ? [
              ["SVG", keep("svg", "")],
              ["SVG tile", keep("tile", "-tile")],
            ]
          : [],
      ),
    [have],
  );
  return (
    <>
      {unit && (
        <Scene
          make={scene}
          value={value}
          onReady={ready}
          role="img"
          aria-label={label(facts)}
          onClick={() => hands.view(1)}
        />
      )}
      {unit && (
        <Library name="Sheets">
          <Sheets math={unit} value={value} tick={tick} onChange={onChange} />
        </Library>
      )}
    </>
  );
}
