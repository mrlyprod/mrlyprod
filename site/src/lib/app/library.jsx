import { useEffect, useRef } from 'react';
import { Bar } from './frame.jsx';
import { Icon } from './controls.jsx';

/* QUEUE */

const jobs = [];
let pending = 0;

function flush() {
  pending = 0;
  jobs.shift()?.();
  if (jobs.length) pending = requestAnimationFrame(flush);
}

function queue(job) {
  jobs.push(job);
  if (!pending) pending = requestAnimationFrame(flush);
  return () => {
    const at = jobs.indexOf(job);
    if (at >= 0) jobs.splice(at, 1);
  };
}

/* ART */

export function Art({ paint, px = 56, deps = [], label }) {
  const at = useRef(null);
  useEffect(() => {
    const canvas = at.current;
    if (!canvas || !paint) return undefined;
    let drop = () => {};
    const draw = () => {
      drop = queue(() => {
        const dpr = Math.min(globalThis.devicePixelRatio || 1, 2);
        const side = Math.round(px * dpr);
        canvas.width = side;
        canvas.height = side;
        paint(canvas.getContext('2d'), side, side, dpr);
      });
    };
    if (typeof IntersectionObserver !== 'function') {
      draw();
      return () => drop();
    }
    const eye = new IntersectionObserver(([one]) => {
      if (!one.isIntersecting) return;
      eye.disconnect();
      draw();
    }, { rootMargin: '200px' });
    eye.observe(canvas);
    return () => {
      eye.disconnect();
      drop();
    };
  }, [px, ...deps]);
  return <canvas ref={at} width={px} height={px} role={label ? 'img' : undefined} aria-label={label} aria-hidden={label ? undefined : 'true'} />;
}

/* LIBRARY */

const ITEMS = '.shelf [data-item]';

export function roam(items, at, key, cols = 1) {
  const by = { ArrowDown: cols, ArrowUp: -cols, ArrowRight: cols > 1 ? 1 : 0, ArrowLeft: cols > 1 ? -1 : 0, Home: -Infinity, End: Infinity }[key];
  if (!by || !items.length) return null;
  if (by === -Infinity) return 0;
  if (by === Infinity) return items.length - 1;
  return Math.max(0, Math.min(items.length - 1, at + by));
}

function move(e) {
  const from = e.target instanceof Element ? e.target.closest('[data-item]') : null;
  if (!from) return;
  const shelf = from.closest('.shelf');
  const items = [...e.currentTarget.querySelectorAll(ITEMS)];
  const grid = shelf?.querySelector('.grid');
  const cols = grid ? Math.max(1, getComputedStyle(grid).gridTemplateColumns.split(' ').length) : 1;
  const next = roam(items, items.indexOf(from), e.key, cols);
  if (next === null) return;
  e.preventDefault();
  items[next]?.focus();
}

export function Library({ name = 'Library', find, onFind, onGo, hint = 'Search', label = `Search ${name}`, at, chips, chip, onChip, children }) {
  const go = (e) => {
    if (e.key === 'Enter' && onGo) {
      e.preventDefault();
      onGo(e.currentTarget.value);
    }
    if (e.key === 'Escape' && e.currentTarget.value === '') e.currentTarget.blur();
  };
  return (
    <Bar side="left">
      <div className="library" onKeyDown={move}>
        <h2>{name}</h2>
        {onFind && (
          <label className="find">
            <Icon name="search" />
            <input ref={at} type="search" value={find ?? ''} placeholder={hint} aria-label={label} onChange={(e) => onFind(e.target.value)} onKeyDown={go} />
          </label>
        )}
        {chips?.length > 0 && (
          <div className="chips" role="radiogroup" aria-label={`${name} filter`}>
            {chips.map(([v, text]) => (
              <button type="button" key={v} role="radio" aria-checked={String(v) === String(chip)} className={String(v) === String(chip) ? 'on' : undefined} onClick={() => onChip?.(v)}>
                {text}
              </button>
            ))}
          </div>
        )}
        {children}
      </div>
    </Bar>
  );
}

export function Shelf({ name, grid = false, children }) {
  return (
    <section className="shelf" aria-label={name}>
      {name && <h3>{name}</h3>}
      <div className={grid ? 'grid' : 'list'} role="list">{children}</div>
    </section>
  );
}

export function Item({ label, sub, on = false, onPick, href, art, title }) {
  const body = (
    <>
      {art}
      <span className="label">{label}</span>
      {sub && <span className="sub">{sub}</span>}
    </>
  );
  const look = { 'data-item': '', 'aria-current': on ? 'true' : undefined, className: on ? 'on' : undefined, title, role: 'listitem' };
  if (href) return <a href={href} {...look}>{body}</a>;
  const pick = (e) => {
    if (e.detail > 0) e.currentTarget.blur();
    onPick?.(e);
  };
  return <button type="button" onClick={pick} {...look}>{body}</button>;
}
