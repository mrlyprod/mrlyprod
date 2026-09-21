import { useEffect, useState } from 'react';
import { createRoot } from 'react-dom/client';
import { Shell } from '../ui/chrome.jsx';
import { demos, load, sidebar } from './tree.js';
import site from './site.js';

export function mount(node) {
  createRoot(document.getElementById('root')).render(node);
}

export function useShelves() {
  const [shelves, set] = useState(demos);
  useEffect(() => {
    let live = true;
    if (!shelves.length) load().then((list) => live && set(list));
    return () => { live = false; };
  }, [shelves.length]);
  return shelves;
}

const READS = `${site.prefix}reads`;

function reads() {
  const el = typeof document === 'undefined' ? null : document.getElementById(READS);
  return el ? JSON.parse(el.textContent) : [];
}

export function Page({ crumb, title, sub, foot, bare, controls, contents, children }) {
  const shelves = useShelves();
  const nodes = sidebar({ demos: shelves });
  const pages = reads();
  return (
    <Shell route={`/demos/${crumb}/`} title={bare ? undefined : title} lead={sub} tree={nodes} controls={controls} contents={contents} wide>
      {children}
      {foot && <p className="foot" hidden={bare}>{foot}</p>}
      {pages.length > 0 && <p className="reads" hidden={bare}>Read: {pages.map((page, i) => <span key={page.href}>{i > 0 && ', '}<a href={page.href}>{page.name}</a></span>)}.</p>}
    </Shell>
  );
}

export function Row({ hidden, children }) {
  return <div className="row" hidden={hidden}>{children}</div>;
}

export function Group({ name, hidden, children }) {
  return (
    <section aria-label={name} hidden={hidden}>
      <h3>{name}</h3>
      <Row>{children}</Row>
    </section>
  );
}

const pair = (option) => (Array.isArray(option) ? option : [option, option]);

export function Pick({ label, value, onChange, options }) {
  return (
    <label>{label} <select value={value} onChange={(e) => onChange(e.target.value)}>
      {options.map(pair).map(([v, text]) => <option key={v} value={v}>{text}</option>)}
    </select></label>
  );
}

export function Slider({ label, value, onChange, min, max, step = 1, show }) {
  return (
    <label>{label} <input type="range" min={min} max={max} step={step} value={value} onChange={(e) => onChange(+e.target.value)} /><span className="num">{show ?? value}</span></label>
  );
}

export function Text({ label, value, onChange, wide }) {
  return (
    <label>{label} <input type="text" className={wide ? 'wide' : undefined} value={value} onChange={(e) => onChange(e.target.value)} /></label>
  );
}

export function Check({ label, checked, onChange }) {
  return (
    <label><input type="checkbox" checked={checked} onChange={(e) => onChange(e.target.checked)} /> {label}</label>
  );
}

export function Btn({ primary, on, onClick, children }) {
  return <button className={primary ? 'primary' : on ? 'on' : undefined} onClick={onClick}>{children}</button>;
}

export function Stats({ children }) {
  return <div className="stats">{children}</div>;
}

export function Stat({ label, children }) {
  return <span>{label} <b>{children}</b></span>;
}

export function Note({ error, children }) {
  return <div className="note">{error ? String(error.message ?? error) : children}</div>;
}
