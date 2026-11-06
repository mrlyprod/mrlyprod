import type { Mrly } from "@mrlyprod/ui/kit/designs/engine.ts";
import { useTheme } from "@mrlyprod/ui/kit/designs/pick.tsx";
import { text } from "@mrlyprod/ui/kit/export.ts";
import type { Readout } from "@mrlyprod/ui/kit/hooks.ts";
import type { Paint } from "@mrlyprod/ui/kit/library.tsx";
import { Art, Item, Library, Shelf } from "@mrlyprod/ui/kit/library.tsx";
import type { Handle, WidgetProps } from "@mrlyprod/ui/kit/page.tsx";
import { Scene } from "@mrlyprod/ui/kit/page.tsx";
import type { Opts, View } from "@mrlyprod/ui/kit/run.ts";
import type { Patch, Scalar, Seeded, Value } from "@mrlyprod/ui/kit/spec.ts";
import { useCallback, useEffect, useRef } from "react";
import type { FactsOf, Kind } from "./engine.ts";
import { COLOURS, FORMULA, fits, KINDS, READS, study, survey } from "./engine.ts";
import type { Handled, Hands, Stage } from "./scene.ts";
import { file, inks, make, shift, sketch, tapped, tidied } from "./scene.ts";

export type PicturesProps = { units: Mrly; value: Value; tick: number; onChange: (patch: Patch) => void };

type Props = WidgetProps<Mrly, Handled> & { value: Seeded };

type Entry = [string, Patch, string?];

const STEP = 700;

const SMALL = { round: 5, level: 4, power: 2, depth: 1 };

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
  const scene = useCallback(
    (canvas: HTMLCanvasElement, view: View, opts: Opts) => make(canvas, view, { ...opts, mrly: units }),
    [units],
  );
  const ready = (handle: Handle<Stage> | null) => {
    live.current = handle;
    if (!handle) return onReady?.(null);
    const hands: Hands = {
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
