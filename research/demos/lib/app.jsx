import { useEffect } from 'react';
import { Contents } from '../ui/contents.jsx';
import { Bar } from './frame.jsx';
import { island } from './island.jsx';
import { follow } from './mrly.js';

export { Btn, Check, Export, Group, Pick, Row, Slider, Text } from './knobs.jsx';

let pages = [];

export function demo(node) {
  return island((host) => {
    const still = host.nextElementSibling;
    pages = [...(still?.querySelectorAll('.reads a') ?? [])].map((a) => ({ name: a.textContent, href: a.getAttribute('href') }));
    still?.remove();
    const quit = follow();
    return {
      node,
      close: () => {
        quit();
        if (still) host.after(still);
      },
    };
  });
}

export function Page({ title, sub, foot, bare, controls, contents = [], children }) {
  useEffect(() => {
    if (!contents.length) return undefined;
    const wire = () => window.dispatchEvent(new Event('wire'));
    wire();
    return wire;
  }, [contents.length]);
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
      <Bar side="right">{controls}<Contents items={contents} /></Bar>
    </>
  );
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
