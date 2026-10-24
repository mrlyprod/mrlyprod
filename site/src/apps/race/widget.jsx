import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { text } from "../../lib/app/export.js";
import { Bar } from "../../lib/app/frame.jsx";
import { useUnit } from "../../lib/app/hooks.js";
import { Art, Item, Library, Shelf } from "../../lib/app/library.jsx";
import { Scene } from "../../lib/app/page.jsx";
import { turn } from "../../lib/app/turn.js";
import { paints } from "../../lib/color.js";
import { total } from "../../lib/designs/engine.js";
import { useTheme } from "../../lib/designs/pick.jsx";
import { flat } from "../../lib/designs/thumb.js";
import { rng } from "../../lib/rand.js";
import { DIM, kin, pair, reads, TURN } from "./engine.js";
import { MATCHUPS, make, named } from "./scene.js";

const ENDS = { stall: "stalled", limit: "time up" };

const LEVEL = 2;

const FAIR = { base: 3, number: 3 };

const roll = () => Math.floor(Math.random() * 4294967296);

export const again = (seed) => (Number(seed) + 1) >>> 0;

const call = (side) => side.name || `code ${side.code}`;

export function fate({ phase, reach, over, sides }) {
  const name = (s) => call(sides[s]);
  if (over) {
    const when = `at step ${over.tick}`;
    if (over.how === "goal") return over.winner < 0 ? `dead heat ${when}` : `${name(over.winner)} wins ${when}`;
    return `${ENDS[over.how]} ${when}, ${over.winner < 0 ? "level" : `${name(over.winner)} ahead`}`;
  }
  if (phase === "ready") return "at the start";
  const lead = reach[0] > reach[1] ? 0 : reach[1] > reach[0] ? 1 : -1;
  return `${lead < 0 ? "level" : `${name(lead)} leads`}${phase === "held" ? ", paused" : ""}`;
}

const both = (sides, show) => {
  const [a, b] = sides.map(show);
  return a === b ? a : `${a} · ${b}`;
};

export function rows(stat) {
  if (!stat) return [];
  const { sides, reach, goal } = stat;
  const names = sides.map(call);
  const tags = names[0] === names[1] ? names.map((name, i) => `${name} ${"AB"[i]}`) : names;
  return [
    ["race", fate(stat)],
    ["step", String(stat.tick)],
    [tags[0], `${reach[0].toFixed(1)} of ${goal} cells`],
    [tags[1], `${reach[1].toFixed(1)} of ${goal} cells`],
    ["mass", both(sides, (side) => `${side.fills} of ${side.of}`)],
    ["dimension", both(sides, (side) => side.dimension.toFixed(3))],
    ["room", both(sides, (side) => `${side.cells} of ${side.lit}`)],
    ["board", `${stat.side} by ${stat.side}`],
    ["walkers", String(stat.walkers)],
  ];
}

function sketch(ctx, math, value, sides, w) {
  const { accent } = paints(ctx.canvas);
  const hues = [accent, turn(accent, TURN)];
  const edge = w / sides.length;
  for (const [i, [code, hue]] of sides.entries())
    flat(ctx, math.two.create(code, value.number, LEVEL, 0, value.base), i * edge, (w - edge) / 2, edge, hues[hue]);
}

export function Widget({ value, onChange, onReady, onExport, onStatus, unit: given }) {
  const unit = useUnit("math", given);
  const live = useRef(null);
  const now = useRef(value);
  now.current = value;
  const hooks = useRef({});
  hooks.current = { onChange, onStatus };
  const [stat, setStat] = useState(null);
  const [find, setFind] = useState("");
  const tick = useTheme();
  const tell = useCallback((next) => {
    setStat(next);
    hooks.current.onStatus?.(rows(next));
  }, []);
  const scene = useCallback(
    (canvas, view, opts) => make(canvas, view, { ...opts, math: unit, onTell: tell }),
    [unit, tell],
  );
  const go = () => {
    const handle = live.current;
    if (!handle) return;
    if (handle.phase?.() === "done") hooks.current.onChange?.({ seed: again(now.current.seed) });
    else handle.go?.();
  };
  const fresh = () => {
    if (!unit) return;
    const [a, b] = pair(unit, now.current, rng(roll()));
    hooks.current.onChange?.({ a, b });
  };
  const ready = (handle) => {
    live.current = handle;
    if (!handle) hooks.current.onStatus?.([]);
    onReady?.(handle && { ...handle, go, toggle: go, pair: fresh });
  };
  const keep = () => {
    const picture = live.current?.svg?.();
    if (picture) text(picture, named(now.current), "svg");
  };
  const log = () => {
    const list = live.current?.csv?.();
    if (list) text(list, named(now.current), "csv");
  };
  const have = Boolean(unit);
  useEffect(
    () =>
      onExport?.(
        have
          ? [
              ["SVG", keep],
              ["CSV", log],
            ]
          : [],
      ),
    [have],
  );
  const names = stat?.sides.map(call);
  const count = unit ? total(unit, DIM, value.base) : 0;
  const mine = reads(value.a, count);
  const rivals = useMemo(
    () => (unit && mine !== null ? kin(unit, value, mine, count) : []),
    [unit, mine, value.base, value.number],
  );
  const word = (code, base = value.base) => unit?.name.of(code, DIM, base) ?? "";
  const want = find.trim().toLowerCase();
  const match = (code, name) => !want || code.startsWith(want) || name.toLowerCase().includes(want);
  const fair = MATCHUPS.map(([a, b]) => [a, b, word(a, FAIR.base), word(b, FAIR.base)]).filter(
    ([a, b, x, y]) => match(a, x) || match(b, y),
  );
  const rest = rivals.map((code) => [code, word(code)]).filter(([code, name]) => match(code, name));
  return (
    <>
      {unit && (
        <Library find={find} onFind={setFind}>
          {fair.length > 0 && (
            <Shelf name="Matchups">
              {fair.map(([a, b, x, y]) => (
                <Item
                  key={`${a}-${b}`}
                  label={`${x || a} v ${y || b}`}
                  on={value.a === a && value.b === b}
                  onPick={() => onChange({ a, b, ...FAIR })}
                  art={
                    <Art
                      paint={(ctx, w) =>
                        sketch(
                          ctx,
                          unit,
                          FAIR,
                          [
                            [a, 0],
                            [b, 1],
                          ],
                          w,
                        )
                      }
                      deps={[unit, a, b, tick]}
                    />
                  }
                />
              ))}
            </Shelf>
          )}
          {rest.length > 0 && (
            <Shelf name={`Rivals of ${word(mine) || mine}`} grid>
              {rest.map(([code, name]) => (
                <Item
                  key={code}
                  label={name || code}
                  sub={name ? code : undefined}
                  on={reads(value.b, count) === code}
                  onPick={() => onChange({ b: code })}
                  art={
                    <Art
                      paint={(ctx, w) => sketch(ctx, unit, value, [[code, 1]], w)}
                      deps={[unit, code, value.base, value.number, tick]}
                    />
                  }
                />
              ))}
            </Shelf>
          )}
        </Library>
      )}
      {unit && (
        <Scene
          make={scene}
          value={value}
          onReady={ready}
          onClick={go}
          role="img"
          aria-label={names ? `${names[0]} against ${names[1]}` : "Two boards of walkers"}
          style={{ cursor: "pointer" }}
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
