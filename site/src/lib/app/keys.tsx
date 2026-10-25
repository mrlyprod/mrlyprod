import type { ReactNode } from "react";
import { useEffect, useRef } from "react";
import { Modal } from "./controls.tsx";
import type { Readout } from "./hooks.ts";
import { label } from "./menus.ts";
import type { Value } from "./spec.ts";

export { label };

/* TYPES */

export type Fired = { key?: string; preventDefault?: () => void };

export type Press = Fired & {
  key: string;
  defaultPrevented?: boolean;
  metaKey?: boolean;
  ctrlKey?: boolean;
  altKey?: boolean;
  shiftKey?: boolean;
  target?: EventTarget | null;
};

export type Label<H> = string | ((value: Value, handle: H | null) => string);

export type Key<H = unknown> = {
  key: string | string[];
  label: Label<H>;
  act: string;
  button?: boolean;
  when?: (value: Value) => boolean;
  does?: string;
  menu?: string;
};

export type Shown = Key<never> & { label: string };

export type Act = (e: Fired) => unknown;

export type Acts = Partial<Record<string, Act>>;

export type Open = (e: Press) => boolean;

export type KeysProps = { map: readonly Shown[]; title?: string };

export type InfoProps = { about?: readonly string[]; rows?: readonly Readout[]; title?: string };

export type HintsProps = { map: readonly Shown[]; count?: number };

type Fused = Shown & { keys: string[][] };

const KEPT =
  ".controls, .library, .chooser, .browser, .pulldown, .trio .menus, textarea, select, [contenteditable]:not([contenteditable=false]), input:not([type=range], [type=checkbox], [type=radio], [type=button], [type=submit], [type=reset])";

const PRESSED = "button, a[href], summary, [role=button], [role=link]";

const tag = (key: string) =>
  key === " " || key === "Spacebar" ? "Space" : key === "Esc" ? "Escape" : key?.length === 1 ? key.toLowerCase() : key;

const within = (e: Press, selector: string) =>
  typeof (e.target as Partial<Element> | null | undefined)?.closest === "function" &&
  !!(e.target as Element).closest(selector);

export function hit<K extends Key<never>>(map: readonly K[], e: Press, open: Open = () => false): K | null {
  if (e.defaultPrevented || e.metaKey || e.ctrlKey || e.altKey) return null;
  const key = tag(e.key);
  const row = map.find((one) => ([] as string[]).concat(one.key).some((k) => tag(k) === key));
  if (!row || within(e, KEPT) || open(e)) return null;
  if ((key === "Space" || key === "Enter") && within(e, PRESSED)) return null;
  return row;
}

const LIGHT = ["keys", "info"];

export function flip(selector: string): void {
  const at = document.querySelector<HTMLDialogElement>(selector);
  if (!at) return;
  if (at.open) at.close();
  else at.showModal();
}

const open = () =>
  [...document.querySelectorAll("dialog[open]")].some((one) => !LIGHT.some((name) => one.classList.contains(name)));

const BASE: Acts = {
  keys: () => flip("dialog.keys"),
  info: () => flip("dialog.info"),
  export: () => flip("dialog.export"),
};

const caps = (keys: string[]) => [...new Set(keys)].map((key) => <kbd key={key}>{label(key)}</kbd>);

const same = <T,>(a: T | undefined, b: T | undefined) => a !== undefined && a === b;

function fused(map: readonly Shown[]): Fused[] {
  const out: Fused[] = [];
  for (const row of map) {
    const keys = ([] as string[]).concat(row.key);
    const twin = out.find((one) => same(one.act, row.act) || same(one.label, row.label));
    if (twin) Object.assign(twin, { keys: [...twin.keys, keys], does: twin.does ?? row.does });
    else out.push({ ...row, keys: [keys] });
  }
  return out;
}

export function useKeys<K extends Key<never>>(map: readonly K[], acts: Acts | undefined): void {
  const latest = useRef(acts);
  latest.current = acts;
  useEffect(() => {
    const press = (e: KeyboardEvent) => {
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

export function Keys({ map, title = "Keys" }: KeysProps) {
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

function linked(text: string): ReactNode[] {
  const out: ReactNode[] = [];
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

export function Info({ about = [], rows = [], title = "Info" }: InfoProps) {
  return (
    <Modal className="info" title={title}>
      {about.map((line) => (
        <p key={line}>{linked(line)}</p>
      ))}
      {rows.length > 0 && (
        <dl>
          {rows.map(([name, value]: Readout) => (
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

export function Hints({ map, count = 3 }: HintsProps) {
  return fused(map)
    .slice(0, count)
    .map((row) => (
      <span key={row.act ?? row.label}>
        {caps(row.keys.map((keys) => keys[0]))}
        {row.label}
      </span>
    ));
}
