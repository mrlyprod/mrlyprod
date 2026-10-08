import { useEffect, useRef } from 'react';
import { hit, label } from './keys.js';
import { Icon } from './knobs.jsx';

const card = () => document.querySelector('dialog.keys');

const open = () => [...document.querySelectorAll('dialog[open]')].some((one) => !one.classList.contains('keys'));

const BASE = {
  keys: () => {
    const at = card();
    if (at?.open) at.close();
    else at?.showModal();
  },
};

const caps = (row) => [].concat(row.key).map((key) => <kbd key={key}>{label(key)}</kbd>);

export function useKeys(map, acts) {
  const latest = useRef(acts);
  latest.current = acts;
  useEffect(() => {
    const press = (e) => {
      const row = hit(map, e, open);
      const run = row && (latest.current?.[row.act] ?? BASE[row.act]);
      if (!run) return;
      e.preventDefault();
      run(e);
    };
    addEventListener('keydown', press);
    return () => removeEventListener('keydown', press);
  }, [map]);
}

export function Keys({ map, title = 'Keys' }) {
  return (
    <dialog className="keys" aria-label={title}>
      <form method="dialog">
        <h2>{title}</h2>
        <button type="submit" className="icon" aria-label="Close">
          <Icon name="close" />
        </button>
      </form>
      <dl>
        {map.map((row) => (
          <div key={row.act ?? row.label}>
            <dt>{caps(row)}</dt>
            <dd>{row.label}{row.does && <span>{row.does}</span>}</dd>
          </div>
        ))}
      </dl>
    </dialog>
  );
}

export function Hints({ map, count = 3 }) {
  return map.slice(0, count).map((row) => (
    <span key={row.act ?? row.label}>
      {caps(row).slice(0, 1)}
      {row.label}
    </span>
  ));
}
