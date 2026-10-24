import { useEffect, useRef } from "react";
import { Modal } from "./controls.jsx";
import { label } from "./menus.js";

export { label };

const KEPT =
  ".controls, .library, .chooser, .browser, .pulldown, .trio .menus, textarea, select, [contenteditable]:not([contenteditable=false]), input:not([type=range], [type=checkbox], [type=radio], [type=button], [type=submit], [type=reset])";

const PRESSED = "button, a[href], summary, [role=button], [role=link]";

const tag = (key) =>
  key === " " || key === "Spacebar" ? "Space" : key === "Esc" ? "Escape" : key?.length === 1 ? key.toLowerCase() : key;

const within = (e, selector) => typeof e.target?.closest === "function" && !!e.target.closest(selector);

export function hit(map, e, open = () => false) {
  if (e.defaultPrevented || e.metaKey || e.ctrlKey || e.altKey) return null;
  const key = tag(e.key);
  const row = map.find((one) => [].concat(one.key).some((k) => tag(k) === key));
  if (!row || within(e, KEPT) || open(e)) return null;
  if ((key === "Space" || key === "Enter") && within(e, PRESSED)) return null;
  return row;
}

const LIGHT = ["keys", "info"];

export function flip(selector) {
  const at = document.querySelector(selector);
  if (!at) return;
  if (at.open) at.close();
  else at.showModal();
}

const open = () =>
  [...document.querySelectorAll("dialog[open]")].some((one) => !LIGHT.some((name) => one.classList.contains(name)));

const BASE = {
  keys: () => flip("dialog.keys"),
  info: () => flip("dialog.info"),
  export: () => flip("dialog.export"),
};

const caps = (keys) => [...new Set(keys)].map((key) => <kbd key={key}>{label(key)}</kbd>);

const same = (a, b) => a !== undefined && a === b;

function fused(map) {
  const out = [];
  for (const row of map) {
    const keys = [].concat(row.key);
    const twin = out.find((one) => same(one.act, row.act) || same(one.label, row.label));
    if (twin) Object.assign(twin, { keys: [...twin.keys, keys], does: twin.does ?? row.does });
    else out.push({ ...row, keys: [keys] });
  }
  return out;
}

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
    addEventListener("keydown", press);
    return () => removeEventListener("keydown", press);
  }, [map]);
}

export function Keys({ map, title = "Keys" }) {
  return (
    <Modal className="keys" title={title}>
      <dl>
        {fused(map).map((row) => (
          <div key={row.act ?? row.label}>
            <dt>{caps(row.keys.flat())}</dt>
            <dd>
              {row.label}
              {row.does && <span>{row.does}</span>}
            </dd>
          </div>
        ))}
      </dl>
    </Modal>
  );
}

const LINK = /\[([^\]]+)\]\(([^)\s]+)\)/g;

function linked(text) {
  const out = [];
  let at = 0;
  for (const found of text.matchAll(LINK)) {
    out.push(text.slice(at, found.index));
    out.push(
      <a key={found.index} href={found[2]}>
        {found[1]}
      </a>,
    );
    at = found.index + found[0].length;
  }
  out.push(text.slice(at));
  return out.filter((part) => part !== "");
}

export function Info({ about = [], rows = [], title = "Info" }) {
  return (
    <Modal className="info" title={title}>
      {about.map((line) => (
        <p key={line}>{linked(line)}</p>
      ))}
      {rows.length > 0 && (
        <dl>
          {rows.map(([name, value]) => (
            <div key={name}>
              <dt>{name}</dt>
              <dd>{value}</dd>
            </div>
          ))}
        </dl>
      )}
    </Modal>
  );
}

export function Hints({ map, count = 3 }) {
  return fused(map)
    .slice(0, count)
    .map((row) => (
      <span key={row.act ?? row.label}>
        {caps(row.keys.map((keys) => keys[0]))}
        {row.label}
      </span>
    ));
}
