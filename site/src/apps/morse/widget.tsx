import { useCallback, useEffect, useRef } from "react";
import { text } from "../../lib/app/export.ts";
import type { Readout } from "../../lib/app/hooks.ts";
import { useReroll } from "../../lib/app/hooks.ts";
import type { Paint } from "../../lib/app/library.tsx";
import { Art, Item, Library, Shelf } from "../../lib/app/library.tsx";
import type { Handle, WidgetProps } from "../../lib/app/page.tsx";
import { Scene } from "../../lib/app/page.tsx";
import type { Opts, View } from "../../lib/app/run.ts";
import type { Patch, Row, Scalar, Seeded, Value } from "../../lib/app/spec.ts";
import { maxOf, minOf } from "../../lib/app/spec.ts";
import type { Mrly } from "../../lib/designs/engine.ts";
import { next } from "../../lib/designs/engine.ts";
import { useTheme } from "../../lib/designs/pick.tsx";
import type { Rng } from "../../lib/rand.ts";
import { rng } from "../../lib/rand.ts";
import type { Doors, FactsOf, Kind } from "./engine.ts";
import {
  based,
  COLOURS,
  FORMULA,
  fits,
  KINDS,
  LEVELS,
  PICTURES,
  pictureOf,
  place,
  READS,
  ROUNDS,
  roll,
  sideOf,
  study,
  survey,
} from "./engine.ts";
import type { Handled, Hands, Stage } from "./scene.ts";
import { file, inks, make, SPEC, sketch, tidied } from "./scene.ts";

/* TYPES */

export type PicturesProps = { units: Doors; value: Value; tick: number; onChange: (patch: Patch) => void };

type Props = WidgetProps<Doors, Handled> & { value: Seeded };

type Entry = [string, Patch, string?];

const STEP = 700;

const SMALL = { round: 5, level: 4, power: 2, depth: 1 };

const row = (key: string): Row => SPEC.find((one) => one.key === key)!;

export const KNOBS: Record<Kind, string> = { word: "round", plane: "level", power: "power", filter: "depth" };

const within = (n: number, low: number, high: number) => Math.max(low, Math.min(high, n));

export function shift(value: Value, by: number, wrap = false) {
  const key = KNOBS[pictureOf(value)];
  const spec = row(key);
  const low = minOf(spec, value)!;
  const top = maxOf(spec, value)!;
  const to = (value[key] as number) + by;
  return { [key]: wrap ? (to > top ? low : to < low ? top : to) : within(to, low, top) };
}

export const tapped = (value: Value) =>
  value.view === "plane" ? { lift: next(KINDS, value.lift, 1) } : shift(value, 1, true);

export const turned = (value: Value, by: number) => place(next(PICTURES, pictureOf(value), by));

const among = <T,>(list: readonly T[], rand: Rng) => list[Math.floor(rand() * list.length)];

const span = (top: number) => Array.from({ length: top }, (_, i) => i + 1);

export function rerolled(math: Mrly, value: Value, rand: Rng) {
  const kind = pictureOf(value);
  if (kind === "plane")
    return {
      lift: among(
        KINDS.filter((one) => one !== value.lift),
        rand,
      ),
      level: among(span(LEVELS), rand),
    };
  if (kind === "word")
    return {
      round: among(
        span(ROUNDS).filter((one) => one !== value.round),
        rand,
      ),
      colour: among(COLOURS, rand)[0],
    };
  return { code: roll(math, sideOf(value), based(value), rand) };
}

/* ROWS */

const yes = (on: boolean) => (on ? "yes" : "no");

const site = (at: [number, number] | null) => (at ? `row ${at[0]}, col ${at[1]}` : "-");

const sized = (f: { side: number }) => `${f.side} by ${f.side}`;

const design = (f: FactsOf<"power" | "filter">) => (f.name ? `${f.name} ${f.code}` : f.title);

export const ROWS: { [K in Kind]: (f: FactsOf<K>) => Readout[] } = {
  word: (f) => [
    ["letters", f.letters],
    ...(f.colour === "runs"
      ? ([
          ["runs", f.runs],
          ["longest run", f.longest],
          ["runs of one", f.singles],
          ["runs of two", f.doubles],
          ["ones", f.ones],
          ["cube free", f.cube ? "yes, no 000 and no 111" : "no, a cube appears"],
          ["boundaries", f.doubling ? "the period-doubling word" : "not the period-doubling word"],
        ] satisfies Readout[])
      : ([
          ["plus", f.plus],
          ["minus", f.minus],
          ["the two rules", f.agree ? "agree letter for letter" : "differ"],
          ["as a schedule", `${f.zero} at 0, ${f.one} at 1`],
          ["exponent", `${f.exponent.toFixed(15)}, (1/2) log2 6, proved`],
          [`rate at ${f.prefix} letters`, f.rate.toFixed(9)],
          ["periodic control", f.control.toFixed(9)],
        ] satisfies Readout[])),
  ],
  plane: (f) => [
    ["lift", `${f.kind}, ${f.formula}`],
    ["side", sized(f)],
    ["plus", f.plus],
    ["minus", f.minus],
    ["tile", f.tile],
    [
      "kronecker power",
      f.twin ? `yes, the ${f.twin.toLowerCase()} grid` : f.folds ? `yes, ${f.name} read plus-minus` : "no",
    ],
    ...(f.folds
      ? []
      : [["off its fold", `${f.faults} of ${f.cells} sites, first at ${site(f.first)}`] satisfies Readout]),
  ],
  power: (f) => [
    ["design", design(f)],
    ["tile", f.tile],
    ["side", sized(f)],
    ["filled", f.filled],
    ["plus", f.plus],
    ["agree", `${f.agree} of ${f.cells}`],
    ["thue-morse grid", yes(f.exact)],
  ],
  filter: (f) => [
    ["design", design(f)],
    ["tile", f.kin ? `${f.tile}, the Thue-Morse tile` : f.tile],
    ["side", sized(f)],
    ["lit", f.differ],
    ["difference", `${f.closed ? "exactly" : "not"} ${f.form}`],
    ["thue-morse grid", yes(f.exact)],
    [
      "off thue-morse",
      f.morse === null
        ? `nothing to compare: the grid lives at tile side 2, not ${f.number}`
        : `${f.morse} of ${f.cells} sites, a share of ${f.share!.toFixed(4)}${f.half ? ", a coin" : ""}`,
    ],
  ],
};

const tell = <K extends Kind>(view: K, f: FactsOf<K>) => ROWS[view](f);

/* LIBRARY */

const SHELVES: [string, Entry[]][] = [
  ["Word", COLOURS.map(([colour, name]) => [name, { view: "word", colour }])],
  ["Lifts", KINDS.map((lift) => [lift, { view: "plane", lift }, FORMULA[lift]])],
  ["Design", READS.map(([read, name]) => [name, { view: "design", read }])],
];

const shows = (value: Value, patch: Patch) => Object.entries(patch).every(([key, v]) => value[key] === v);

export function Pictures({ units, value, tick, onChange }: PicturesProps) {
  const paint =
    (patch: Patch): Paint =>
    (ctx, w, h, dpr) =>
      sketch(ctx, w, h, dpr, survey(units, tidied({ ...value, ...patch, ...SMALL })), inks(ctx.canvas));
  return SHELVES.map(([name, items]) => (
    <Shelf key={name} name={name}>
      {items.map(([label, patch, sub]) => (
        <Item
          key={label}
          label={label}
          sub={sub}
          on={shows(value, patch)}
          onPick={() => onChange(patch)}
          art={<Art paint={paint(patch)} deps={[tick, value.code, value.base, value.number, value.fold]} />}
        />
      ))}
    </Shelf>
  ));
}

/* WIDGET */

export function Widget({ value, onChange, onReady, onExport, onStatus, units }: Props) {
  const tick = useTheme();
  const live = useRef<Handle<Stage> | null>(null);
  const now = useRef(value);
  now.current = value;
  const hooks = useRef(onChange);
  hooks.current = onChange;
  const timer = useRef<ReturnType<typeof setInterval> | 0>(0);
  const wrote = useRef<Scalar | null>(null);
  const plan = units ? study(units, tidied(value)) : null;
  const told = plan ? tell(plan.view, plan.facts) : [];
  const sheet = Boolean(plan && fits(plan));
  const word = value.view === "word";
  const play = (on: boolean) => {
    clearInterval(timer.current);
    timer.current = 0;
    if (!on) return;
    wrote.current = now.current.round;
    timer.current = setInterval(() => {
      const patch = shift({ ...now.current, view: "word" }, 1, true);
      wrote.current = patch.round;
      hooks.current(patch);
    }, STEP);
  };
  const keep = () => {
    const picture = live.current?.svg?.();
    if (picture) text(picture, file(now.current, units), "svg");
  };
  useEffect(() => onStatus?.(told), [JSON.stringify(told)]);
  useEffect(() => onExport?.(sheet ? [["SVG", keep]] : []), [sheet]);
  useEffect(() => () => play(false), []);
  useEffect(() => {
    if (!word || (timer.current && value.round !== wrote.current)) play(false);
  }, [word, value.round]);
  useReroll(value.seed, (seed) => units && onChange(rerolled(units.math, value, rng(seed))));
  const scene = useCallback(
    (canvas: HTMLCanvasElement, view: View, opts: Opts) => make(canvas, view, { ...opts, ...units }),
    [units],
  );
  const ready = (handle: Handle<Stage> | null) => {
    live.current = handle;
    if (!handle) return onReady?.(null);
    const hands: Hands = {
      deeper: (by) => hooks.current(shift(now.current, by)),
      turn: (by) => hooks.current(turned(now.current, by)),
      toggle: () => play(!timer.current),
      playing: () => Boolean(timer.current),
    };
    onReady?.({ ...handle, ...hands });
  };
  const tap = () => hooks.current(tapped(now.current));
  return (
    <>
      {units && <Scene make={scene} value={value} onReady={ready} onClick={tap} />}
      {units && (
        <Library name="Pictures">
          <Pictures units={units} value={value} tick={tick} onChange={onChange} />
        </Library>
      )}
    </>
  );
}
