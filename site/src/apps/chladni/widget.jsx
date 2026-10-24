import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { Art, Item, Library, Shelf } from "../../lib/app/library.jsx";
import { Scene } from "../../lib/app/page.jsx";
import { useTouch } from "../../lib/app/touch.js";
import { paints } from "../../lib/color.js";
import { useTheme } from "../../lib/designs/pick.jsx";
import { flat } from "../../lib/designs/thumb.js";
import { carpets, cell, hits, inks, ladder, rungOf, swatch } from "./engine.js";
import { make } from "./scene.js";

const SIGNS = { 1: "+", "-1": "-" };
const KINDS = [
  ["plate", "Plates"],
  ["carpet", "Carpets"],
];

export function rows(stat) {
  if (!stat) return [];
  if (stat.error) return [["error", stat.error]];
  if (stat.view === "carpet") {
    return [
      ["k", `${stat.k} of ${stat.count}`],
      ["lambda", stat.lambda.toFixed(6)],
      ["domains", `${stat.domains} of ${stat.bound}`],
      ["grains left", String(stat.grains)],
    ];
  }
  return [
    ["figure", `(${[stat.n, stat.m, SIGNS[stat.sign]].filter((one) => one !== undefined).join(", ")})`],
    ["pitch", stat.pitch === null ? "none" : stat.pitch.toFixed(3)],
    ["grains", String(stat.grains)],
  ];
}

export function Shelves({ value, onChange, units }) {
  const [kind, setKind] = useState(value.view);
  const [find, setFind] = useState("");
  const tick = useTheme();
  useEffect(() => setKind(value.view), [value.view]);
  const rungs = useMemo(() => (units?.num ? ladder(units.num) : []), [units]);
  const designs = useMemo(() => (units?.math ? carpets(units.math) : []), [units]);
  const at = value.view === "plate" && units?.num ? rungOf(units.num, value) : -1;
  const dye = (ctx) => inks(paints(ctx.canvas, value.tint));
  return (
    <Library chips={KINDS} chip={kind} onChip={setKind} find={find} onFind={kind === "plate" ? setFind : undefined}>
      {kind === "plate" && (
        <Shelf name="Plates" grid>
          {rungs.map(
            (one, i) =>
              hits(one, find) && (
                <Item
                  key={`${one.n},${one.m},${one.sign}`}
                  label={`${one.n}, ${one.m}`}
                  sub={one.n === one.m ? "" : SIGNS[one.sign]}
                  on={i === at}
                  onPick={() => onChange({ view: "plate", n: one.n, m: one.m, sign: one.sign })}
                  art={
                    <Art
                      px={64}
                      paint={(ctx, w) =>
                        ctx.putImageData(new ImageData(swatch(units.num, one, dye(ctx), w), w, w), 0, 0)
                      }
                      deps={[i, value.tint, tick]}
                    />
                  }
                />
              ),
          )}
        </Shelf>
      )}
      {kind === "carpet" && (
        <Shelf name="Carpets" grid>
          {designs.map((one) => (
            <Item
              key={`${one.number}-${one.code}`}
              label={one.name}
              sub={`no. ${one.number}`}
              on={value.view === "carpet" && value.number === one.number && value.code === one.code}
              onPick={() => onChange({ view: "carpet", number: one.number, code: one.code, level: one.level })}
              art={
                <Art
                  px={64}
                  paint={(ctx, w) => flat(ctx, cell(units.math, one), 0, 0, w, paints(ctx.canvas, value.tint).accent)}
                  deps={[one.number, one.code, value.tint, tick]}
                />
              }
            />
          ))}
        </Shelf>
      )}
    </Library>
  );
}

export function Widget({ value, onChange, onReady, onStatus, units }) {
  const live = useRef(null);
  const change = useRef(onChange);
  const status = useRef(onStatus);
  change.current = onChange;
  status.current = onStatus;
  const tell = useCallback((stat) => status.current?.(rows(stat)), []);
  const push = useCallback((patch) => change.current?.(patch), []);
  const scene = useCallback(
    (canvas, view, opts) => make(canvas, view, { ...opts, ...units, onTell: tell, onWalk: push }),
    [units, tell, push],
  );
  const ready = (handle) => {
    live.current = handle;
    if (!handle) status.current?.([]);
    onReady?.(handle);
  };
  const touch = useTouch(live, change);
  if (!units?.num) return null;
  return (
    <>
      <Scene
        make={scene}
        value={value}
        onReady={ready}
        {...touch}
        role="img"
        aria-label="Sand on a Chladni plate"
        style={{ cursor: "crosshair" }}
      />
      <Shelves value={value} onChange={onChange} units={units} />
    </>
  );
}
