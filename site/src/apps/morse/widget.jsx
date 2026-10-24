import { useCallback, useEffect, useRef } from "react";
import { text } from "../../lib/app/export.js";
import { useReroll } from "../../lib/app/hooks.js";
import { Art, Item, Library, Shelf } from "../../lib/app/library.jsx";
import { Scene } from "../../lib/app/page.jsx";
import { maxOf } from "../../lib/app/spec.js";
import { next } from "../../lib/designs/engine.js";
import { useTheme } from "../../lib/designs/pick.jsx";
import { rng } from "../../lib/rand.js";
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
} from "./engine.js";
import { file, inks, make, SPEC, sketch, tidied } from "./scene.js";

const STEP = 700;

const SMALL = { round: 5, level: 4, power: 2, depth: 1 };

const row = (key) => SPEC.find((one) => one.key === key);

export const KNOBS = { word: "round", plane: "level", power: "power", filter: "depth" };

const within = (n, low, high) => Math.max(low, Math.min(high, n));

export function shift(value, by, wrap = false) {
  const key = KNOBS[pictureOf(value)];
  const spec = row(key);
  const top = maxOf(spec, value);
  const to = value[key] + by;
  return { [key]: wrap ? (to > top ? spec.min : to < spec.min ? top : to) : within(to, spec.min, top) };
}

export const tapped = (value) =>
  value.view === "plane" ? { lift: next(KINDS, value.lift, 1) } : shift(value, 1, true);

export const turned = (value, by) => place(next(PICTURES, pictureOf(value), by));

const among = (list, rand) => list[Math.floor(rand() * list.length)];

const span = (top) => Array.from({ length: top }, (_, i) => i + 1);

export function rerolled(math, value, rand) {
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

const yes = (on) => (on ? "yes" : "no");

const site = (at) => (at ? `row ${at[0]}, col ${at[1]}` : "-");

const sized = (f) => `${f.side} by ${f.side}`;

const design = (f) => (f.name ? `${f.name} ${f.code}` : f.title);

export const ROWS = {
  word: (f) => [
    ["letters", f.letters],
    ...(f.colour === "runs"
      ? [
          ["runs", f.runs],
          ["longest run", f.longest],
          ["runs of one", f.singles],
          ["runs of two", f.doubles],
          ["ones", f.ones],
          ["cube free", f.cube ? "yes, no 000 and no 111" : "no, a cube appears"],
          ["boundaries", f.doubling ? "the period-doubling word" : "not the period-doubling word"],
        ]
      : [
          ["plus", f.plus],
          ["minus", f.minus],
          ["the two rules", f.agree ? "agree letter for letter" : "differ"],
          ["as a schedule", `${f.zero} at 0, ${f.one} at 1`],
          ["exponent", `${f.exponent.toFixed(15)}, (1/2) log2 6, proved`],
          [`rate at ${f.prefix} letters`, f.rate.toFixed(9)],
          ["periodic control", f.control.toFixed(9)],
        ]),
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
    ...(f.folds ? [] : [["off its fold", `${f.faults} of ${f.cells} sites, first at ${site(f.first)}`]]),
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
        : `${f.morse} of ${f.cells} sites, a share of ${f.share.toFixed(4)}${f.half ? ", a coin" : ""}`,
    ],
  ],
};

/* LIBRARY */

const SHELVES = [
  ["Word", COLOURS.map(([colour, name]) => [name, { view: "word", colour }])],
  ["Lifts", KINDS.map((lift) => [lift, { view: "plane", lift }, FORMULA[lift]])],
  ["Design", READS.map(([read, name]) => [name, { view: "design", read }])],
];

const shows = (value, patch) => Object.entries(patch).every(([key, v]) => value[key] === v);

export function Pictures({ units, value, tick, onChange }) {
  const paint = (patch) => (ctx, w, h, dpr) =>
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

export function Widget({ value, onChange, onReady, onExport, onStatus, units }) {
  const tick = useTheme();
  const live = useRef(null);
  const now = useRef(value);
  now.current = value;
  const hooks = useRef(onChange);
  hooks.current = onChange;
  const timer = useRef(0);
  const wrote = useRef(null);
  const plan = units ? study(units, tidied(value)) : null;
  const f = plan?.facts;
  const told = plan ? ROWS[plan.view](f) : [];
  const sheet = Boolean(plan && fits(plan));
  const word = value.view === "word";
  const play = (on) => {
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
  const scene = useCallback((canvas, view, opts) => make(canvas, view, { ...opts, ...units }), [units]);
  const ready = (handle) => {
    live.current = handle;
    if (!handle) return onReady?.(null);
    const hands = {
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
