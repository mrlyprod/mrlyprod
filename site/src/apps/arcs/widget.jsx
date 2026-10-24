import { useCallback, useEffect, useRef, useState } from "react";
import { text } from "../../lib/app/export.js";
import { useReroll } from "../../lib/app/hooks.js";
import { Art, Item, Library, Shelf } from "../../lib/app/library.jsx";
import { Scene } from "../../lib/app/page.jsx";
import { paints } from "../../lib/color.js";
import { next } from "../../lib/designs/engine.js";
import { useTheme } from "../../lib/designs/pick.jsx";
import { rng } from "../../lib/rand.js";
import { BASES, cap, curves, laws, looping, names, resolve } from "./engine.js";
import { file, make, shades, sketch } from "./scene.js";

const SMALL = { 2: 3, 3: 2 };

export function rows(f) {
  if (!f) return [];
  const design = ["design", f.name ? `${f.name} ${f.code}` : `code ${f.code}`];
  if (f.fault) return [design, ["fault", f.fault]];
  const law = f.law
    ? [
        ["law", `L(n) = ${f.law.formula}`],
        [`L(${f.level})`, `${f.law.loops}, the law ${f.law.loops === f.loops ? "meets" : "misses"} the count`],
      ]
    : [["law", "no closed form proved for this code"]];
  return [
    design,
    ["loops", String(f.loops)],
    ["strands", `${f.strands} = 2 x ${f.side}, proved`],
    ["side", String(f.side)],
    ["filled", `${f.filled} of ${f.cells}`],
    ...law,
  ];
}

export function Laws({ math, value, tick, onChange }) {
  const code = resolve(math, value.base, value.code);
  const paint = (one) => (ctx, w, h, dpr) =>
    sketch(
      ctx,
      w,
      h,
      dpr,
      curves(math, one.code, one.base, SMALL[one.base] ?? 1),
      value,
      shades(ctx.canvas, paints(ctx.canvas, value.tint), value.strand),
    );
  return BASES.map(([base]) => {
    const list = laws(math).filter((one) => one.base === base);
    if (!list.length) return null;
    const level = Math.min(value.level, cap(math, base));
    return (
      <Shelf key={base} name={`Base ${base}`}>
        {list.map((one) => (
          <Item
            key={one.code}
            label={one.name ? `${one.name} ${one.code}` : `code ${one.code}`}
            sub={`${Number(math.arcs.law(one.code, base, level).loops)} loops`}
            title={`L(n) = ${one.formula}`}
            on={Number(value.base) === base && code === one.code}
            onPick={() => onChange({ base, code: one.code })}
            art={
              <Art paint={paint(one)} deps={[tick, value.tint, value.strand, value.cells, value.look, value.width]} />
            }
          />
        ))}
      </Shelf>
    );
  });
}

export function Widget({ value, onChange, onReady, onExport, onStatus, units }) {
  const unit = units?.math;
  const tick = useTheme();
  const live = useRef(null);
  const now = useRef(value);
  now.current = value;
  const hooks = useRef({});
  hooks.current = { onChange, onStatus };
  const [facts, setFacts] = useState(null);
  useReroll(value.seed, (seed) => unit && onChange({ code: looping(unit, value.base, value.level, rng(seed)) }));
  const tell = useCallback((f) => {
    setFacts(f);
    hooks.current.onStatus?.(rows(f));
  }, []);
  const scene = useCallback(
    (canvas, view, opts) => make(canvas, view, { ...opts, math: unit, onTell: tell }),
    [unit, tell],
  );
  const step = (by) => {
    const { base, code } = now.current;
    hooks.current.onChange({
      code: next(
        names(unit, base).map(([one]) => one),
        resolve(unit, base, code),
        by,
      ),
    });
  };
  const deeper = (by) => hooks.current.onChange({ level: now.current.level + by });
  const ready = (handle) => {
    live.current = handle;
    if (!handle) hooks.current.onStatus?.([]);
    onReady?.(handle && { ...handle, step, deeper });
  };
  const keep = () => {
    const picture = live.current?.svg?.();
    if (picture) text(picture, file(now.current, units), "svg");
  };
  const have = Boolean(unit);
  useEffect(() => onExport?.(have ? [["SVG", keep]] : []), [have]);
  const label = `${facts?.name || "a design"} at level ${value.level} drawn in arcs`;
  return (
    <>
      {unit && (
        <Scene
          make={scene}
          value={value}
          onReady={ready}
          onClick={() => live.current?.trace?.()}
          role="img"
          aria-label={label}
        />
      )}
      {unit && (
        <Library name="Laws">
          <Laws math={unit} value={value} tick={tick} onChange={onChange} />
        </Library>
      )}
    </>
  );
}
