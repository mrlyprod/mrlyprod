import { useEffect } from 'react';
import { createPortal } from 'react-dom';
import { Contents } from '../ui/contents.jsx';
import { island } from './island.jsx';

let pages = [];

export function demo(node) {
  return island((host) => {
    const still = host.nextElementSibling;
    pages = [...(still?.querySelectorAll('.reads a') ?? [])].map((a) => ({ name: a.textContent, href: a.getAttribute('href') }));
    still?.remove();
    return { node, close: () => still && host.after(still) };
  });
}

function Bar({ controls, contents = [] }) {
  const slot = document.querySelector('#right .controls');
  useEffect(() => {
    if (!contents.length) return undefined;
    const wire = () => window.dispatchEvent(new Event('wire'));
    wire();
    return wire;
  }, [contents.length]);
  if (!slot) return null;
  return createPortal(<>{controls}<Contents items={contents} /></>, slot);
}

export function Page({ title, sub, foot, bare, controls, contents, children }) {
  return (
    <>
      {!bare && title && (
        <div className="lede">
          <h1>{title}</h1>
          {sub && <p className="lead">{sub}</p>}
        </div>
      )}
      {children}
      {foot && <p className="foot" hidden={bare}>{foot}</p>}
      {pages.length > 0 && <p className="reads" hidden={bare}>Read: {pages.map((page, i) => <span key={page.href}>{i > 0 && ', '}<a href={page.href}>{page.name}</a></span>)}.</p>}
      <Bar controls={controls} contents={contents} />
    </>
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
