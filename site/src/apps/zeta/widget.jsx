import { useCallback, useEffect, useRef } from "react";
import { text } from "../../lib/app/export.js";
import { Bar } from "../../lib/app/frame.jsx";
import { useReroll, useUnit } from "../../lib/app/hooks.js";
import { Art, Item, Library, Shelf } from "../../lib/app/library.jsx";
import { Scene } from "../../lib/app/page.jsx";
import { paints } from "../../lib/color.js";
import { useTheme } from "../../lib/designs/pick.jsx";
import { REACH, roll, shelves, study, ZEROS } from "./engine.js";
import { inked, make, PAGE, sketch } from "./scene.js";

const SWIPE = 6;

const SCRUB = REACH / 2;

const ART = 112;

const four = (v) => v.toFixed(4);

export function rows(view, said) {
  if (!said) return [];
  const { walk, stairs } = said;
  if (view === "stairs")
    return [
      ["x", String(stairs.x)],
      ["zeros", String(stairs.zeros)],
      ["psi(x)", four(stairs.psi)],
      ["formula", four(stairs.formula)],
      ["gap", four(stairs.gap)],
    ];
  const out = [
    ["t", walk.t.toFixed(2)],
    ["zeros below t", String(walk.count)],
    ["next zero", walk.next === null ? "past the list" : walk.next.toFixed(6)],
    ["Z(t)", four(walk.z)],
    ["re", four(walk.re)],
    ["im", four(walk.im)],
    ["theta", four(walk.theta)],
  ];
  for (const [i, [zero, past]] of walk.list.entries())
    out.push([`zero ${i + 1}`, `${zero.toFixed(6)}${past ? " passed" : ""}`]);
  return out;
}

export function scrub(grab, dx) {
  const share = dx / Math.max(1, grab.w);
  return grab.view === "stairs" ? { zeros: Math.round(grab.zeros + share * ZEROS) } : { t: grab.t + share * SCRUB };
}

export function Widget({ value, onChange, onReady, onExport, onStatus, unit: given }) {
  const unit = useUnit("num", given);
  const live = useRef(null);
  const now = useRef(value);
  now.current = value;
  const hooks = useRef({});
  hooks.current = { onChange, onStatus };
  const grab = useRef(null);
  const tick = useTheme();
  useReroll(value.seed, (next) => hooks.current.onChange?.(roll(next)));
  const tell = useCallback((said) => {
    if (said.t !== now.current.t) hooks.current.onChange?.({ t: said.t });
    hooks.current.onStatus?.(rows(now.current.view, said));
  }, []);
  const scene = useCallback(
    (canvas, view, opts) => make(canvas, view, { ...opts, num: unit, onTell: tell }),
    [unit, tell],
  );
  const ready = (handle) => {
    live.current = handle;
    if (!handle) hooks.current.onStatus?.([]);
    onReady?.(handle);
  };
  const keep = () => {
    const picture = live.current?.svg?.();
    if (picture) text(picture, PAGE.name(now.current), "svg");
  };
  const sheet = () => {
    const list = live.current?.csv?.();
    if (list) text(list, "zeta-zeros", "csv");
  };
  const have = Boolean(unit);
  useEffect(
    () =>
      onExport?.(
        have
          ? [
              ["SVG", keep],
              ["CSV", sheet],
            ]
          : [],
      ),
    [have],
  );
  const down = (e) => {
    if (!live.current) return;
    const box = e.currentTarget.getBoundingClientRect();
    grab.current = {
      x: e.clientX,
      w: box.width,
      view: now.current.view,
      t: now.current.t,
      zeros: now.current.zeros,
      moved: false,
    };
    e.currentTarget.setPointerCapture?.(e.pointerId);
  };
  const move = (e) => {
    const one = grab.current;
    if (!one) return;
    const dx = e.clientX - one.x;
    if (!one.moved && Math.abs(dx) < SWIPE) return;
    one.moved = true;
    onChange?.(scrub(one, dx));
  };
  const up = () => {
    const one = grab.current;
    grab.current = null;
    if (one && !one.moved) live.current?.go?.();
  };
  const cancel = () => {
    grab.current = null;
  };
  return (
    <>
      {unit && (
        <Library>
          {shelves(study(unit).zeros, value).map(([name, list]) => (
            <Shelf key={name} name={name} grid>
              {list.map(({ id, label, sub, set, on }) => (
                <Item
                  key={id}
                  label={label}
                  sub={sub}
                  on={on}
                  onPick={() => onChange?.(set)}
                  art={
                    <Art
                      px={ART}
                      paint={(ctx, w, h, dpr) =>
                        sketch(unit, set, ctx, w, h, dpr, inked(ctx.canvas, paints(ctx.canvas, value.tint).accent))
                      }
                      deps={[unit, value.tint, tick]}
                    />
                  }
                />
              ))}
            </Shelf>
          ))}
        </Library>
      )}
      {unit && (
        <Scene
          make={scene}
          value={value}
          onReady={ready}
          onPointerDown={down}
          onPointerMove={move}
          onPointerUp={up}
          onPointerCancel={cancel}
          style={{ cursor: "ew-resize" }}
        />
      )}
      {!unit && (
        <Bar side="status">
          <span>loading</span>
        </Bar>
      )}
    </>
  );
}
