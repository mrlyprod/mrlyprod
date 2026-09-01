import { createRoot } from 'react-dom/client';

export function mount(node) {
  createRoot(document.getElementById('root')).render(node);
}

export function Page({ crumb, title, sub, foot, bare, children }) {
  return (
    <>
      <header hidden={bare}>
        {crumb && <nav><a href="../">demos</a> / {crumb}</nav>}
        <h1>{title}</h1>
        {sub && <p className="sub">{sub}</p>}
      </header>
      <main>
        {children}
        {foot && <p className="foot" hidden={bare}>{foot}</p>}
      </main>
    </>
  );
}

export function Row({ hidden, children }) {
  return <div className="row" hidden={hidden}>{children}</div>;
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
