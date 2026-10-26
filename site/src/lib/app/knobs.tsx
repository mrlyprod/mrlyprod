import type { ReactElement } from "react";
import { useCallback, useContext, useEffect, useMemo, useRef, useState } from "react";
import type { Folds, Styled } from "./controls.tsx";
import { Accordion, Fold, Group, Pick, pair, Segment, Slider, said, Text, Toggle } from "./controls.tsx";
import type { Kind, Option, Row, Scalar, Spec, Units, Value } from "./spec.ts";
import { describe, items, labelOf, maxOf, minOf, optionsOf, places, tidy, unitOf } from "./spec.ts";

/* TYPES */

export type Change = (patch: Value) => void;

export type Flip = (name: string, on: boolean) => void;

export type KnobProps<U = Units> = { row: Row<U>; value?: Scalar; values?: Value; units?: U; onChange: Change };

export type KnobsProps<U = Units> = { spec: Spec<U>; value: Value; units?: U; onChange: Change; id?: string };

type Props = { label: string; value: Scalar; onChange: (v: Scalar | readonly string[]) => void };

type Renderer = <U>(row: Row<U>, props: Props, values: Value, units?: U) => ReactElement;

type ListProps<U> = { row: Row<U>; value: Scalar; values: Value; units?: U; set: (v: string[]) => void };

const choice = (option: Option) => String(pair(option)[0]);

const named = <U,>(row: Row<U>, v: Scalar, values: Value, units?: U) =>
  optionsOf(row, values, units)!
    .map(pair)
    .find(([one]) => String(one) === String(v))?.[1] ?? String(v);

const told = (out: unknown): out is Scalar => typeof out === "string" || typeof out === "number";

const showOf = <U,>(row: Row<U>, values: Value) =>
  typeof row.show === "function"
    ? (v: Scalar) => {
        const out = row.show!(v as number, values);
        return told(out) ? out : v;
      }
    : undefined;

export function shown<U>(row: Row<U>, v: Scalar | undefined, values: Value | undefined, units?: U): Scalar | null {
  const value = v ?? row.def;
  const all = values ?? { [row.key]: value };
  if (row.kind === "slider" || row.kind === "number") {
    const unit = unitOf(row, all, units);
    return said(row.zero, value) ?? `${showOf(row, all)?.(value) ?? value}${unit ? ` ${unit}` : ""}`;
  }
  if (row.kind === "toggle") return Number(value) === 1 ? labelOf(row, all, units) : null;
  if (row.kind === "text") return value === "" || value === undefined || value === null ? null : String(value);
  if (row.kind === "list")
    return (
      items(row, value, all, units)
        .map((one) => named(row, one, all, units))
        .join(", ") || null
    );
  return named(row, value, all, units);
}

export const gist = <U,>(rows: readonly Row<U>[], values: Value | undefined, units?: U) =>
  rows
    .map((row) => shown(row, values?.[row.key], values, units))
    .filter((one) => one !== null && one !== "")
    .join(" · ");

function List<U>({ row, value, values, units, set }: ListProps<U>) {
  const options = optionsOf(row, values, units)!;
  const have = items(row, value, values, units);
  const flip = (option: string) => (on: 0 | 1) =>
    set(options.map(choice).filter((one) => (one === option ? on : have.includes(one))));
  return options.map(pair).map(([v, text]) => {
    const on = have.includes(String(v));
    return (
      <Toggle key={v} label={text} value={on ? 1 : 0} disabled={on && have.length === 1} onChange={flip(String(v))} />
    );
  });
}

const KNOB: Record<Kind, Renderer> = {
  slider: (row, props, values, units) => (
    <Slider
      {...props}
      min={minOf(row, values, units)}
      max={maxOf(row, values, units)}
      step={row.step}
      unit={unitOf(row, values, units)}
      zero={row.zero}
      show={showOf(row, values)}
    />
  ),
  number: (row, props, values, units) => (
    <Text
      {...props}
      type="number"
      inputMode="decimal"
      min={minOf(row, values, units)}
      max={maxOf(row, values, units)}
      step={row.step}
      zero={row.zero}
      commit
    />
  ),
  toggle: (_row, props) => <Toggle {...props} />,
  pick: (row, props, values, units) => <Pick {...props} options={optionsOf(row, values, units)!} />,
  segment: (row, props, values, units) => <Segment {...props} options={optionsOf(row, values, units)!} />,
  text: (row, props) => <Text {...props} wide={row.wide} commit />,
  list: (row, props, values, units) => (
    <List row={row} value={props.value} values={values} units={units} set={props.onChange} />
  ),
};

export function Knob<U = Units>({ row, value, values, units, onChange }: KnobProps<U>) {
  const all: Value = values ?? (value === undefined ? {} : { [row.key]: value });
  return KNOB[row.kind](
    row,
    {
      label: labelOf(row, all, units),
      value: value ?? row.def,
      onChange: (v) => onChange(tidy([row], { ...all, [row.key]: v }, units)),
    },
    all,
    units,
  );
}

const kept = (id: string) => `knobs:${id}`;

function stored(id: string | undefined): string[] | null {
  if (!id) return null;
  try {
    const raw = sessionStorage.getItem(kept(id));
    return raw ? JSON.parse(raw) : null;
  } catch {
    return null;
  }
}

function store(id: string | undefined, open: string[]): void {
  if (!id) return;
  try {
    sessionStorage.setItem(kept(id), JSON.stringify(open));
  } catch {}
}

const turn = (list: string[], name: string, on: boolean) =>
  on === list.includes(name) ? list : on ? [...list, name] : list.filter((one) => one !== name);

function useFolds(id: string | undefined, first: string): [string[], Flip] {
  const [open, setOpen] = useState(() => stored(id) ?? (first ? [first] : []));
  useEffect(() => store(id, open), [id, open]);
  const flip = (name: string, on: boolean) => setOpen((was) => turn(was, name, on));
  return [open, flip];
}

const first = <U,>(spec: Spec<U>) => spec.find((row) => row.group)?.group ?? "";

export function useAccordion<U = Units>(id: string | undefined, spec: Spec<U> = []): Folds {
  const pane = useMemo(() => places(spec), [spec]);
  const [open, setOpen] = useState(() => stored(id));
  const [lead, setLead] = useState("");
  const met = useRef(Infinity);
  useEffect(() => {
    if (open) store(id, open);
  }, [id, open]);
  const now = (was: string[] | null) => was ?? [lead || pane.lead || first(spec)].filter(Boolean);
  const flip = (name: string, on: boolean) =>
    setOpen((was) => {
      const list = now(was);
      const next = turn(list, name, on);
      return next === list ? was : next;
    });
  const meet = useCallback((name: string, order: number) => {
    if (order >= met.current) return;
    met.current = order;
    setLead(name);
  }, []);
  return { has: (name) => now(open).includes(name), flip, meet, of: pane.of, at: pane.at };
}

export function Knobs<U = Units>({ spec, value, units, onChange, id }: KnobsProps<U>) {
  const shared = useContext(Accordion);
  const [open, flip] = useFolds(shared ? "" : id, first(spec));
  return describe(spec, value).map(({ name, rows }) => {
    const knobs = rows.map((row) => (
      <Knob key={row.key} row={row} value={value?.[row.key]} values={value} units={units} onChange={onChange} />
    ));
    const style: Styled | undefined = shared ? { order: shared.of(name) } : undefined;
    if (!name)
      return (
        <Group key="" style={style}>
          {knobs}
        </Group>
      );
    const own = shared ? {} : { open: open.includes(name), onToggle: (on: boolean) => flip(name, on) };
    return (
      <Fold key={name} name={name} style={style} live={gist(rows, value, units)} {...own}>
        {knobs}
      </Fold>
    );
  });
}
