import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { text } from "../../lib/app/export.ts";
import { Bar } from "../../lib/app/frame.tsx";
import type { Readout } from "../../lib/app/hooks.ts";
import { useUnit } from "../../lib/app/hooks.ts";
import { Art, Item, Library, Shelf } from "../../lib/app/library.tsx";
import type { Handle, WidgetProps } from "../../lib/app/page.tsx";
import { Scene } from "../../lib/app/page.tsx";
import type { Opts, View } from "../../lib/app/run.ts";
import type { Scalar, Units } from "../../lib/app/spec.ts";
import { turn } from "../../lib/app/turn.ts";
import { paints } from "../../lib/color.ts";
import type { Mrly } from "../../lib/designs/engine.ts";
import { total } from "../../lib/designs/engine.ts";
import { useTheme } from "../../lib/designs/pick.tsx";
import { flat } from "../../lib/designs/thumb.ts";
import { rng } from "../../lib/rand.ts";
import type { Size } from "./engine.ts";
import { DIM, kin, pair, reads, TURN } from "./engine.ts";
import type { Handled, Knobs, Race, Stat, Told } from "./scene.ts";
import { MATCHUPS, make, named } from "./scene.ts";

/* TYPES */

export type Fated = Pick<Stat, "phase" | "reach" | "over"> & { sides: Pick<Told, "code" | "name">[] };

type Contract = WidgetProps<Units, Handled>;

type Props = Partial<Contract> &
  Pick<Contract, "value" | "onChange"> & { value: Knobs & { seed: number }; unit?: Mrly | null };

type Hooks = Partial<Pick<Props, "onChange" | "onStatus">>;

const ENDS = { stall: "stalled", limit: "time up" };

const LEVEL = 2;

const FAIR = { base: 3, number: 3 };

const roll = () => Math.floor(Math.random() * 4294967296);

export const again = (seed: Scalar) => (Number(seed) + 1) >>> 0;

const call = (side: Pick<Told, "code" | "name">) => side.name || `code ${side.code}`;

export function fate({ phase, reach, over, sides }: Fated) {
  const name = (s: number) => call(sides[s]);
  if (over) {
    const when = `at step ${over.tick}`;
    if (over.how === "goal") return over.winner < 0 ? `dead heat ${when}` : `${name(over.winner)} wins ${when}`;
    return `${ENDS[over.how]} ${when}, ${over.winner < 0 ? "level" : `${name(over.winner)} ahead`}`;
  }
  if (phase === "ready") return "at the start";
  const lead = reach[0] > reach[1] ? 0 : reach[1] > reach[0] ? 1 : -1;
  return `${lead < 0 ? "level" : `${name(lead)} leads`}${phase === "held" ? ", paused" : ""}`;
}

const both = (sides: Told[], show: (side: Told) => string) => {
  const [a, b] = sides.map(show);
  return a === b ? a : `${a} · ${b}`;
};

export function rows(stat: Stat | null): Readout[] {
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

function sketch(
  ctx: CanvasRenderingContext2D,
  math: Mrly,
  value: Pick<Size, "base" | "number">,
  sides: [string, number][],
  w: number,
) {
  const { accent } = paints(ctx.canvas);
  const hues = [accent, turn(accent, TURN)];
  const edge = w / sides.length;
  for (const [i, [code, hue]] of sides.entries())
    flat(ctx, math.two.create(code, value.number, LEVEL, 0, value.base), i * edge, (w - edge) / 2, edge, hues[hue]);
}

export function Widget({ value, onChange, onReady, onExport, onStatus, unit: given }: Props) {
  const unit = useUnit("math", given);
  const live = useRef<Handle<Race> | null>(null);
  const now = useRef(value);
  now.current = value;
  const hooks = useRef<Hooks>({});
  hooks.current = { onChange, onStatus };
  const [stat, setStat] = useState<Stat | null>(null);
  const [find, setFind] = useState("");
  const tick = useTheme();
  const tell = useCallback((next: Stat) => {
    setStat(next);
    hooks.current.onStatus?.(rows(next));
  }, []);
  const scene = useCallback(
    (canvas: HTMLCanvasElement, view: View, opts: Opts) => make(canvas, view, { ...opts, math: unit, onTell: tell }),
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
  const ready = (handle: Handle<Race> | null) => {
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
  const word = (code: string, base = value.base) => unit?.name.of(code, DIM, base) ?? "";
  const want = find.trim().toLowerCase();
  const match = (code: string, name: string) => !want || code.startsWith(want) || name.toLowerCase().includes(want);
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
            <Shelf name={`Rivals of ${word(mine!) || mine}`} grid>
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
