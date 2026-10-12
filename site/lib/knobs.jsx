import { createContext, useCallback, useContext, useEffect, useId, useLayoutEffect, useMemo, useRef, useState } from 'react';
import { save } from './export.js';
import { ICONS, VIEWBOX } from './icons.js';
import { describe, items, labelOf, maxOf, minOf, optionsOf, places, tidy, unitOf } from './knobs.js';

const pair = (option) => (Array.isArray(option) ? option : [option, option]);

const number = (input) => +input.value;

const string = (input) => input.value;

const said = (zero, v) => (zero && v !== '' && Number(v) === 0 ? zero : null);

const classes = (...names) => names.filter(Boolean).join(' ') || undefined;

const loose = (e) => e.detail > 0 && e.currentTarget.blur();

function useDraft(value, onChange, commit, read) {
  const [draft, setDraft] = useState(null);
  const at = useRef(null);
  const latest = useRef(onChange);
  latest.current = onChange;
  useEffect(() => {
    const input = at.current;
    if (!commit || !input) return undefined;
    const done = () => {
      setDraft(null);
      latest.current(read(input));
    };
    input.addEventListener('change', done);
    return () => input.removeEventListener('change', done);
  }, [commit, read]);
  const type = (e) => (commit ? setDraft(read(e.target)) : onChange(read(e.target)));
  return [at, draft ?? value, type];
}

export function Icon({ name, size = 20 }) {
  return (
    <svg viewBox={VIEWBOX} width={size} height={size} fill="currentColor" aria-hidden="true">
      <path d={ICONS[name]} />
    </svg>
  );
}

export function Row({ children, ...rest }) {
  return <div className="row" {...rest}>{children}</div>;
}

export const Accordion = createContext(null);

const placed = (shared, after, style) => (shared ? { order: shared.at(after), ...style } : style);

export function Group({ name, hidden, after, style, children, ...rest }) {
  const shared = useContext(Accordion);
  return (
    <section aria-label={name || undefined} hidden={hidden} style={placed(shared, after, style)} {...rest}>
      {name && <h3>{name}</h3>}
      <Row>{children}</Row>
    </section>
  );
}

function Joined({ name, after, style, onToggle, ...rest }) {
  const shared = useContext(Accordion);
  const spot = placed(shared, after, style);
  useLayoutEffect(() => {
    shared?.meet(name, spot.order);
  });
  if (!shared) return <Fold name={name} open={null} style={style} onToggle={onToggle} {...rest} />;
  const flip = (on) => {
    shared.flip(name, on);
    onToggle?.(on);
  };
  return <Fold name={name} open={shared.has(name)} style={spot} onToggle={flip} {...rest} />;
}

export function Fold({ name, open, onToggle, live, after, children, ...rest }) {
  if (open === undefined) return <Joined name={name} after={after} live={live} onToggle={onToggle} {...rest}>{children}</Joined>;
  return (
    <details className="group" open={open} onToggle={(e) => onToggle?.(e.currentTarget.open)} {...rest}>
      <summary onClick={loose}>
        <h3>{name}</h3>
        <span className="live">{live ?? ''}</span>
        <Icon name="expand_more" />
      </summary>
      <Row>{children}</Row>
    </details>
  );
}

export function Pick({ label, value, onChange, options, ...rest }) {
  return (
    <label>
      <span>{label}</span>
      <select value={value} onChange={onChange && ((e) => onChange(e.target.value))} {...rest}>
        {options.map(pair).map(([v, text]) => <option key={v} value={v}>{text}</option>)}
      </select>
      <span className="drop">
        <Icon name="expand_more" />
      </span>
    </label>
  );
}

export function Slider({ label, value, onChange, min, max, step = 1, zero, unit, show, commit, ...rest }) {
  const word = said(zero, value);
  return (
    <label>
      <span>{label}</span>
      <span className="num">{word ?? (show ? show(value) : value)}{unit && !word ? ` ${unit}` : ''}</span>
      <input type="range" min={min} max={max} step={step} value={value} onChange={(e) => onChange(number(e.target))} {...rest} />
    </label>
  );
}

export function Text({ label, value, onChange, wide, zero, commit, ...rest }) {
  const [at, now, type] = useDraft(value, onChange, commit, string);
  const word = said(zero, now);
  return (
    <label>
      <span>{label}</span>
      {word && <span className="num">{word}</span>}
      <input ref={at} type="text" className={wide ? 'wide' : undefined} value={now} onChange={type} {...rest} />
    </label>
  );
}

export function Toggle({ label, value, onChange, ...rest }) {
  return (
    <label>
      <span>{label}</span>
      <input type="checkbox" role="switch" checked={Number(value) === 1} onChange={(e) => onChange(e.target.checked ? 1 : 0)} {...rest} />
    </label>
  );
}

export function Segment({ label, value, onChange, options, ...rest }) {
  const name = useId();
  return (
    <div className="segment" role="radiogroup" aria-label={label} {...rest}>
      <span>{label}</span>
      <span className="options">
        {options.map(pair).map(([v, text]) => (
          <label key={v}>
            <input type="radio" name={name} value={v} checked={String(v) === String(value)} onChange={() => onChange(v)} />
            {text}
          </label>
        ))}
      </span>
    </div>
  );
}

export function Btn({ primary, on, className, onClick, children, ...rest }) {
  const look = classes(primary ? 'primary' : on ? 'on' : '', className);
  const click = (e) => {
    loose(e);
    onClick?.(e);
  };
  return <button type="button" className={look} onClick={click} {...rest}>{children}</button>;
}

export function Modal({ title, className, children, ...rest }) {
  return (
    <dialog className={classes('modal', className)} aria-label={title} {...rest}>
      <form method="dialog">
        <h2>{title}</h2>
        <button type="submit" className="icon" aria-label="Close">
          <Icon name="close" />
        </button>
      </form>
      {children}
    </dialog>
  );
}

export function Menu({ rows, children, ...rest }) {
  const id = `menu${useId().replace(/[^\w-]/g, '')}`;
  const hide = () => document.getElementById(id)?.hidePopover?.();
  return (
    <>
      <Btn popoverTarget={id} style={{ anchorName: `--${id}` }} {...rest}>{children}</Btn>
      <div id={id} popover="auto" className="pop" style={{ positionAnchor: `--${id}` }}>
        {rows.map(([text, fn, icon]) => (
          <button type="button" key={text} onClick={() => (hide(), fn())}>
            {icon && <Icon name={icon} />}
            {text}
          </button>
        ))}
      </div>
    </>
  );
}

const FORMATS = [['png', 'PNG'], ['webp', 'WebP']];

const SCALES = [[1, '1x'], [2, '2x'], [4, '4x']];

export function Export({ canvas, name, draw, fix, more = [], title = 'Export' }) {
  const at = useRef(null);
  const [kind, setKind] = useState('png');
  const [scale, setScale] = useState(1);
  const close = () => at.current?.close();
  const take = () => {
    close();
    const from = typeof canvas === 'function' ? canvas() : canvas;
    if (from) save(from, { name, kind, draw, scale: fix ? scale : 1, fix });
  };
  return (
    <Modal ref={at} className="export" title={title}>
      <div className="rows">
        <Segment label="Format" value={kind} options={FORMATS} onChange={setKind} />
        {fix && <Segment label="Size" value={scale} options={SCALES} onChange={(v) => setScale(Number(v))} />}
      </div>
      <Btn primary onClick={take}>Save {FORMATS.find(([one]) => one === kind)[1]}</Btn>
      {more.length > 0 && <div className="also">{more.map(([text, fn]) => <button type="button" key={text} onClick={() => (close(), fn())}>{text}</button>)}</div>}
    </Modal>
  );
}

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
