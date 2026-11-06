import { paints } from "@mrlyprod/ui/kit/color.ts";
import { useTheme } from "@mrlyprod/ui/kit/designs/pick.tsx";
import { text } from "@mrlyprod/ui/kit/export.ts";
import type { Readout } from "@mrlyprod/ui/kit/hooks.ts";
import { useCrates, useLatest, useStatus } from "@mrlyprod/ui/kit/hooks.ts";
import { Art, Item, Library, Shelf } from "@mrlyprod/ui/kit/library.tsx";
import type { Handle, WidgetProps } from "@mrlyprod/ui/kit/page.tsx";
import { Scene } from "@mrlyprod/ui/kit/page.tsx";
import type { Opts, View } from "@mrlyprod/ui/kit/run.ts";
import type { Scalar, Seeded } from "@mrlyprod/ui/kit/spec.ts";
import type { PointerEvent } from "react";
import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import type { Facts, Num, Pair, Point, Word } from "./engine.ts";
import { clicked, coords, FATES, hit, LIST, name, parse, product, shelves, study, verdict } from "./engine.ts";
import type { Gaussian } from "./scene.ts";
import { make, named, sketch } from "./scene.ts";

type Props = WidgetProps<Num, Gaussian> & { value: Seeded; unit?: Num | null };

const TAP = 8;

const ART = 112;

const KEY = "blue split · orange inert · pink ramified · green units";

const pct = (x: number) => `${(x * 100).toFixed(2)}%`;

function told(word: Word, p: Point | null): Readout[] {
  if (!p) return [];
  const head: Readout[] = [
    ["point", clicked(word, p)],
    ["verdict", verdict(word, p)],
  ];
  if (!p.norm) return head;
  return [
    ...head,
    ["norm", String(p.norm)],
    ["class", FATES[p.fate]],
    ...(p.factors.length ? [["factors", product(p.factors)] satisfies Readout] : []),
    ["conjugate", name(word, p.conjugate[0], p.conjugate[1])],
    ["associates", p.associates.map(([a, b]) => name(word, a, b)).join(", ")],
    [`points of norm ${p.norm}`, String(p.shell)],
  ];
}

export function rows(facts: Facts | undefined, point: Point | null, missed: boolean, look: Scalar): Readout[] {
  if (!facts) return [];
  const word = facts.ring;
  return [
    ...(missed ? [["tap", "missed the window"] satisfies Readout] : []),
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
    ...(look === "fate" ? [["key", KEY] satisfies Readout] : []),
  ];
}

export function Widget({ value, onChange, onReady, onExport, onStatus, unit: given }: Props) {
  const unit = useCrates(LIST, given);
  const live = useRef<Handle<Gaussian> | null>(null);
  const from = useRef<Pair | null>(null);
  const now = useLatest(value);
  const change = useLatest(onChange);
  const [missed, setMissed] = useState(false);
  const plan = useMemo(
    () => unit && study(unit, value.ring as Word, value.limit as number),
    [unit, value.ring, value.limit],
  );
  const point = useMemo(() => {
    const at = parse(value.click);
    return plan && at ? hit(unit!, plan, at[0], at[1]) : null;
  }, [plan, value.click]);
  const key = JSON.stringify(value);
  useEffect(() => setMissed(false), [key]);
  const tick = useTheme();
  useStatus(onStatus, rows(plan?.facts, point, missed, value.look));
  const scene = useCallback(
    (canvas: HTMLCanvasElement, view: View, opts: Opts) => make(canvas, view, { ...opts, mrly: unit }),
    [unit],
  );
  const pin = (click: string | null) => {
    setMissed(false);
    if (click) change.current({ click });
  };
  const ready = (handle: Handle<Gaussian> | null) => {
    live.current = handle;
    onReady?.(handle);
  };
  const down = (e: PointerEvent<HTMLCanvasElement>) => {
    from.current = [e.clientX, e.clientY];
  };
  const up = (e: PointerEvent<HTMLCanvasElement>) => {
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
                      paint={(ctx, w, h, dpr) =>
                        sketch(unit, set, ctx, w, h, dpr, paints(ctx.canvas, value.tint as string))
                      }
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
