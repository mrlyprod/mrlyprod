import { createContext, useContext, useEffect, useId, useLayoutEffect, useRef, useState } from 'react';
import { flushSync } from 'react-dom';
import { save } from './export.js';
import { ICONS, VIEWBOX } from '../icons.gen.js';

export const pair = (option) => (Array.isArray(option) ? option : [option, option]);

export const number = (input) => +input.value;

export const string = (input) => input.value;

export const said = (zero, v) => (zero && v !== '' && Number(v) === 0 ? zero : null);

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

export function Pick({ label, value, onChange, options, ref, ...rest }) {
  return (
    <label ref={ref}>
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

const natural = (el) => {
  const was = el.style.width;
  el.style.width = 'max-content';
  const need = el.getBoundingClientRect().width;
  el.style.width = was;
  return need;
};

export function useRoom(key, anywhere = false) {
  const at = useRef(null);
  const need = useRef({ key, width: 0 });
  const [narrow, setNarrow] = useState(false);
  useLayoutEffect(() => {
    const el = at.current;
    if (!el || (!anywhere && !el.parentNode?.classList?.contains('row')) || typeof ResizeObserver !== 'function') return undefined;
    const check = () => {
      if (!narrow) need.current = { key, width: natural(el) };
      setNarrow(need.current.key === key && need.current.width > el.getBoundingClientRect().width);
    };
    check();
    const eye = new ResizeObserver(() => flushSync(check));
    eye.observe(el);
    return () => eye.disconnect();
  }, [narrow, key]);
  return [at, narrow];
}

export function Segment({ label, value, onChange, options, ...rest }) {
  const name = useId();
  const pairs = options.map(pair);
  const [at, narrow] = useRoom([label, ...pairs.map(([v, text]) => `${v}:${text}`)].join('\n'));
  if (narrow) return <Pick ref={at} label={label} value={value} options={pairs} onChange={(picked) => onChange(pairs.find(([v]) => String(v) === picked)?.[0] ?? picked)} {...rest} />;
  return (
    <div ref={at} className="segment" role="radiogroup" aria-label={label} {...rest}>
      <span>{label}</span>
      <span className="options">
        {pairs.map(([v, text]) => (
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
