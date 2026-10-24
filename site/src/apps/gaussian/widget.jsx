import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { text } from "../../lib/app/export.js";
import { useLatest, useReroll, useStatus, useUnit } from "../../lib/app/hooks.js";
import { Art, Item, Library, Shelf } from "../../lib/app/library.jsx";
import { Scene } from "../../lib/app/page.jsx";
import { paints } from "../../lib/color.js";
import { useTheme } from "../../lib/designs/pick.jsx";
import { rng } from "../../lib/rand.js";
import { clicked, coords, FATES, hit, name, parse, product, shelves, shuffle, study, verdict } from "./engine.js";
import { make, named, sketch } from "./scene.js";

const TAP = 8;

const ART = 112;

const KEY = "blue split · orange inert · pink ramified · green units";

const pct = (x) => `${(x * 100).toFixed(2)}%`;

export const rerolled = (seed) => ({ ...shuffle(rng(seed)), click: "" });

function told(word, p) {
  if (!p) return [];
  const head = [
    ["point", clicked(word, p)],
    ["verdict", verdict(word, p)],
  ];
  if (!p.norm) return head;
  return [
    ...head,
    ["norm", String(p.norm)],
    ["class", FATES[p.fate]],
    ...(p.factors.length ? [["factors", product(p.factors)]] : []),
    ["conjugate", name(word, p.conjugate[0], p.conjugate[1])],
    ["associates", p.associates.map(([a, b]) => name(word, a, b)).join(", ")],
    [`points of norm ${p.norm}`, String(p.shell)],
  ];
}

export function rows(facts, point, missed, look) {
  if (!facts) return [];
  const word = facts.ring;
  return [
    ...(missed ? [["tap", "missed the window"]] : []),
    ...told(word, point),
    ["window", `${word}, reach ${facts.reach}, norms to ${facts.top}, ${facts.units} units`],
    ["primes", String(facts.primes)],
    ["density", pct(facts.density)],
    ["split", String(facts.split)],
    ["inert", String(facts.inert)],
    ["ramified", String(facts.ramified)],
    ["points", String(facts.points)],
    ["symmetry", `${facts.symmetry}-fold`],
    ["busiest norm", `${facts.busy}, ${facts.most} points`],
    ...(look === "fate" ? [["key", KEY]] : []),
  ];
}

export function Widget({ value, onChange, onReady, onExport, onStatus, unit: given }) {
  const unit = useUnit("num", given);
  const live = useRef(null);
  const from = useRef(null);
  const now = useLatest(value);
  const change = useLatest(onChange);
  const [missed, setMissed] = useState(false);
  const plan = useMemo(() => unit && study(unit, value.ring, value.limit), [unit, value.ring, value.limit]);
  const point = useMemo(() => {
    const at = parse(value.click);
    return plan && at ? hit(unit, plan, at[0], at[1]) : null;
  }, [plan, value.click]);
  const key = JSON.stringify(value);
  useEffect(() => setMissed(false), [key]);
  const tick = useTheme();
  useReroll(value.seed, (seed) => change.current(rerolled(seed)));
  useStatus(onStatus, rows(plan?.facts, point, missed, value.look));
  const scene = useCallback((canvas, view, opts) => make(canvas, view, { ...opts, num: unit }), [unit]);
  const pin = (click) => {
    setMissed(false);
    if (click) change.current({ click });
  };
  const ready = (handle) => {
    live.current = handle;
    const hands = {
      walk: (dir) => pin(handle.walk(dir)),
      turn: (dir) => pin(handle.turn(dir)),
    };
    onReady?.(handle && { ...handle, ...hands });
  };
  const down = (e) => {
    from.current = [e.clientX, e.clientY];
  };
  const up = (e) => {
    const was = from.current;
    from.current = null;
    if (!was || Math.hypot(e.clientX - was[0], e.clientY - was[1]) > TAP || !live.current?.at) return;
    const box = e.currentTarget.getBoundingClientRect();
    const found = live.current.at((e.clientX - box.left) / box.width, (e.clientY - box.top) / box.height);
    if (found) pin(coords(found.a, found.b));
    else setMissed(true);
  };
  const cancel = () => {
    from.current = null;
  };
  const keep = () => {
    const picture = live.current?.svg?.();
    if (picture) text(picture, named(now.current), "svg");
  };
  const sheet = () => {
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
              ["CSV", sheet],
            ]
          : [],
      ),
    [have],
  );
  if (!unit) return null;
  return (
    <>
      <Library>
        {shelves(value).map(([title, list], n) => (
          <Shelf key={title} name={title} grid={n === 0}>
            {list.map(({ id, label, sub, set, on }) => (
              <Item
                key={id}
                label={label}
                sub={sub}
                on={on}
                onPick={() => change.current(set)}
                art={
                  n === 0 ? (
                    <Art
                      px={ART}
                      paint={(ctx, w, h, dpr) => sketch(unit, set, ctx, w, h, dpr, paints(ctx.canvas, value.tint))}
                      deps={[unit, value.tint, tick]}
                    />
                  ) : undefined
                }
              />
            ))}
          </Shelf>
        ))}
      </Library>
      <Scene
        make={scene}
        value={value}
        onReady={ready}
        onPointerDown={down}
        onPointerUp={up}
        onPointerCancel={cancel}
        aria-label="The window: the primes of the ring around the origin"
      />
    </>
  );
}
