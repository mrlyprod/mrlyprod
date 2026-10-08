import { useEffect, useId, useRef, useState } from 'react';
import { save } from './export.js';
import { ICONS, VIEWBOX } from './icons.js';
import { describe, items, tidy } from './knobs.js';

const pair = (option) => (Array.isArray(option) ? option : [option, option]);

const number = (input) => +input.value;

const string = (input) => input.value;

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
  return [at, draft ?? value, type, draft !== null];
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

export function Group({ name, hidden, children, ...rest }) {
  return (
    <section aria-label={name || undefined} hidden={hidden} {...rest}>
      {name && <h3>{name}</h3>}
      <Row>{children}</Row>
    </section>
  );
}

export function Pick({ label, value, onChange, options, ...rest }) {
  return (
    <label>
      <span>{label}</span>
      <select value={value} onChange={onChange && ((e) => onChange(e.target.value))} {...rest}>
        {options.map(pair).map(([v, text]) => <option key={v} value={v}>{text}</option>)}
      </select>
    </label>
  );
}

export function Slider({ label, value, onChange, min, max, step = 1, show, unit, commit, ...rest }) {
  const [at, now, type, dragging] = useDraft(value, onChange, commit, number);
  return (
    <label>
      <span>{label}</span>
      <span className="num">{dragging ? now : (show ?? value)}{unit ? ` ${unit}` : ''}</span>
      <input ref={at} type="range" min={min} max={max} step={step} value={now} onChange={type} {...rest} />
    </label>
  );
}

export function Text({ label, value, onChange, wide, commit, ...rest }) {
  const [at, now, type] = useDraft(value, onChange, commit, string);
  return (
    <label>
      <span>{label}</span>
      <input ref={at} type="text" className={wide ? 'wide' : undefined} value={now} onChange={type} {...rest} />
    </label>
  );
}

export function Check({ label, checked, onChange, ...rest }) {
  return (
    <label>
      <span>{label}</span>
      <input type="checkbox" checked={checked} onChange={(e) => onChange(e.target.checked)} {...rest} />
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
  const look = [primary ? 'primary' : on ? 'on' : '', className].filter(Boolean).join(' ');
  const click = (e) => {
    onClick?.(e);
    if (e.detail > 0) e.currentTarget.blur();
  };
  return <button type="button" className={look || undefined} onClick={click} {...rest}>{children}</button>;
}

const FORMATS = [['png', 'PNG'], ['webp', 'WebP']];

export function Export({ canvas, name, draw, label = 'Export', ...rest }) {
  const id = `export${useId().replace(/[^\w-]/g, '')}`;
  const take = (kind) => {
    document.getElementById(id)?.hidePopover?.();
    const at = typeof canvas === 'function' ? canvas() : canvas;
    if (at) save(at, { name, kind, draw });
  };
  return (
    <>
      <Btn popoverTarget={id} style={{ anchorName: `--${id}` }} {...rest}>
        <Icon name="export" />
        {label}
      </Btn>
      <div id={id} popover="auto" className="formats" style={{ positionAnchor: `--${id}` }}>
        {FORMATS.map(([kind, text]) => <button type="button" key={kind} onClick={() => take(kind)}>{text}</button>)}
      </div>
    </>
  );
}

const choice = (option) => String(pair(option)[0]);

function List({ row, value, set }) {
  const have = items(row, value);
  const flip = (option) => (on) => set(row.options.map(choice).filter((one) => (one === option ? on : have.includes(one))));
  return row.options.map(pair).map(([v, text]) => {
    const on = have.includes(String(v));
    return <Toggle key={v} label={text} value={on ? 1 : 0} disabled={on && have.length === 1} onChange={flip(String(v))} />;
  });
}

const KNOB = {
  slider: (row, props) => <Slider {...props} min={row.min} max={row.max} step={row.step} unit={row.unit} commit />,
  number: (row, props) => <Text {...props} type="number" inputMode="decimal" min={row.min} max={row.max} step={row.step} commit />,
  toggle: (row, props) => <Toggle {...props} />,
  pick: (row, props) => <Pick {...props} options={row.options} />,
  segment: (row, props) => <Segment {...props} options={row.options} />,
  text: (row, props) => <Text {...props} commit />,
  list: (row, props) => <List row={row} value={props.value} set={props.onChange} />,
};

export function Knob({ row, value, onChange }) {
  return KNOB[row.kind](row, { label: row.label ?? row.key, value: value ?? row.def, onChange: (v) => onChange(tidy([row], { [row.key]: v })) });
}

export function Knobs({ spec, value, onChange }) {
  return describe(spec).map(({ name, rows }) => (
    <Group key={name} name={name}>
      {rows.map((row) => <Knob key={row.key} row={row} value={value?.[row.key]} onChange={onChange} />)}
    </Group>
  ));
}
