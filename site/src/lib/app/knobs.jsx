import { useCallback, useContext, useEffect, useMemo, useRef, useState } from 'react';
import { Accordion, Fold, Group, Pick, Segment, Slider, Text, Toggle, number, pair, said, string } from './controls.jsx';
import { describe, items, labelOf, maxOf, minOf, optionsOf, places, tidy, unitOf } from './spec.js';

const choice = (option) => String(pair(option)[0]);

const named = (row, v, values, units) => optionsOf(row, values, units).map(pair).find(([one]) => String(one) === String(v))?.[1] ?? String(v);

const told = (out) => typeof out === 'string' || typeof out === 'number';

const showOf = (row, values) => (typeof row.show === 'function' ? (v) => {
  const out = row.show(v, values);
  return told(out) ? out : v;
} : undefined);

export function shown(row, v, values, units) {
  const value = v ?? row.def;
  const all = values ?? { [row.key]: value };
  if (row.kind === 'slider' || row.kind === 'number') {
    const unit = unitOf(row, all, units);
    return said(row.zero, value) ?? `${showOf(row, all)?.(value) ?? value}${unit ? ` ${unit}` : ''}`;
  }
  if (row.kind === 'toggle') return Number(value) === 1 ? labelOf(row, all, units) : null;
  if (row.kind === 'text') return value === '' || value === undefined || value === null ? null : String(value);
  if (row.kind === 'list') return items(row, value, all, units).map((one) => named(row, one, all, units)).join(', ') || null;
  return named(row, value, all, units);
}

export const gist = (rows, values, units) => rows.map((row) => shown(row, values?.[row.key], values, units)).filter((one) => one !== null && one !== '').join(' · ');

function List({ row, value, values, units, set }) {
  const options = optionsOf(row, values, units);
  const have = items(row, value, values, units);
  const flip = (option) => (on) => set(options.map(choice).filter((one) => (one === option ? on : have.includes(one))));
  return options.map(pair).map(([v, text]) => {
    const on = have.includes(String(v));
    return <Toggle key={v} label={text} value={on ? 1 : 0} disabled={on && have.length === 1} onChange={flip(String(v))} />;
  });
}

const KNOB = {
  slider: (row, props, values, units) => <Slider {...props} min={minOf(row, values, units)} max={maxOf(row, values, units)} step={row.step} unit={unitOf(row, values, units)} zero={row.zero} show={showOf(row, values)} />,
  number: (row, props, values, units) => <Text {...props} type="number" inputMode="decimal" min={minOf(row, values, units)} max={maxOf(row, values, units)} step={row.step} zero={row.zero} commit />,
  toggle: (row, props) => <Toggle {...props} />,
  pick: (row, props, values, units) => <Pick {...props} options={optionsOf(row, values, units)} />,
  segment: (row, props, values, units) => <Segment {...props} options={optionsOf(row, values, units)} />,
  text: (row, props) => <Text {...props} wide={row.wide} commit />,
  list: (row, props, values, units) => <List row={row} value={props.value} values={values} units={units} set={props.onChange} />,
};

export function Knob({ row, value, values, units, onChange }) {
  const all = values ?? { [row.key]: value };
  return KNOB[row.kind](row, { label: labelOf(row, all, units), value: value ?? row.def, onChange: (v) => onChange(tidy([row], { ...all, [row.key]: v }, units)) }, all, units);
}

const kept = (id) => `knobs:${id}`;

function stored(id) {
  if (!id) return null;
  try {
    const raw = sessionStorage.getItem(kept(id));
    return raw ? JSON.parse(raw) : null;
  } catch {
    return null;
  }
}

function store(id, open) {
  if (!id) return;
  try {
    sessionStorage.setItem(kept(id), JSON.stringify(open));
  } catch {}
}

const turn = (list, name, on) => (on === list.includes(name) ? list : on ? [...list, name] : list.filter((one) => one !== name));

function useFolds(id, first) {
  const [open, setOpen] = useState(() => stored(id) ?? (first ? [first] : []));
  useEffect(() => store(id, open), [id, open]);
  const flip = (name, on) => setOpen((was) => turn(was, name, on));
  return [open, flip];
}

const first = (spec) => spec.find((row) => row.group)?.group ?? '';

export function useAccordion(id, spec = []) {
  const pane = useMemo(() => places(spec), [spec]);
  const [open, setOpen] = useState(() => stored(id));
  const [lead, setLead] = useState('');
  const met = useRef(Infinity);
  useEffect(() => {
    if (open) store(id, open);
  }, [id, open]);
  const now = (was) => was ?? [lead || pane.lead || first(spec)].filter(Boolean);
  const flip = (name, on) => setOpen((was) => {
    const list = now(was);
    const next = turn(list, name, on);
    return next === list ? was : next;
  });
  const meet = useCallback((name, order) => {
    if (order >= met.current) return;
    met.current = order;
    setLead(name);
  }, []);
  return { has: (name) => now(open).includes(name), flip, meet, of: pane.of, at: pane.at };
}

export function Knobs({ spec, value, units, onChange, id }) {
  const shared = useContext(Accordion);
  const [open, flip] = useFolds(shared ? '' : id, first(spec));
  return describe(spec, value).map(({ name, rows }) => {
    const knobs = rows.map((row) => <Knob key={row.key} row={row} value={value?.[row.key]} values={value} units={units} onChange={onChange} />);
    const style = shared ? { order: shared.of(name) } : undefined;
    if (!name) return <Group key="" style={style}>{knobs}</Group>;
    const own = shared ? {} : { open: open.includes(name), onToggle: (on) => flip(name, on) };
    return (
      <Fold key={name} name={name} style={style} live={gist(rows, value, units)} {...own}>
        {knobs}
      </Fold>
    );
  });
}
