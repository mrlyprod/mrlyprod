import { useEffect, useRef, useState } from 'react';
import { PREFIX, stored, subscribe, write } from '../../shell/prefs.js';
import { Action, Item, Note, Pane as Frame, Section } from './ui.jsx';

export const FIND = ['privacy', 'data', 'storage', 'export', 'import', 'reset', 'erase', 'cookies', 'analytics', 'tracking', 'kept in this browser', 'what the site sends'];

/* NAMES */

const NAMES = {
  theme: 'Appearance',
  font: 'Font',
  tint: 'Accent colour',
  glass: 'Glass',
  icons: 'Icon style',
  'icons-always': 'Icons always dark',
  magnify: 'Dock magnification',
  text: 'Text size',
  motion: 'Reduce motion',
  clear: 'Reduce transparency',
  contrast: 'Increase contrast',
  targets: 'Larger targets',
  saver: 'Desktop picture',
  'savers-off': 'Screen saver picks',
  idle: 'Screen saver delay',
  clock: 'Screen saver clock',
  cart: 'Cart',
  welcome: 'Lock screen seen',
  page: 'Page window width',
  wide: 'Wide window width',
  launchpad: 'Launchpad layout',
  recent: 'Recent apps',
  dock: 'Dock',
  widgets: 'Widgets',
  design: 'Current design',
  'designs-recent': 'Recent designs',
  'designs-saved': 'Saved designs',
  panes: 'Open side panels',
  library: 'Library width',
  knobs: 'Knobs width',
};

const name = (key) => {
  const id = key.slice(PREFIX.length);
  return NAMES[id] ?? id;
};

/* SIZE */

const bytes = (text) => new TextEncoder().encode(text).length;

const size = (n) => (n < 1024 ? `${n} B` : `${(n / 1024).toFixed(1)} KB`);

const count = (n) => `${n} ${n === 1 ? 'item' : 'items'}`;

/* FILE */

function save() {
  const text = JSON.stringify(Object.fromEntries(stored()), null, 2);
  const url = URL.createObjectURL(new Blob([text], { type: 'application/json' }));
  const link = document.createElement('a');
  link.href = url;
  link.download = 'mrly-data.json';
  document.body.append(link);
  link.click();
  link.remove();
  setTimeout(() => URL.revokeObjectURL(url), 1000);
}

function parse(text) {
  try {
    const data = JSON.parse(text);
    if (!data || typeof data !== 'object' || Array.isArray(data)) return null;
    const rows = Object.entries(data);
    return rows.length && rows.every(([key, value]) => key.startsWith(PREFIX) && typeof value === 'string') ? rows : null;
  } catch {
    return null;
  }
}

function wipe() {
  for (const area of ['localStorage', 'sessionStorage']) {
    try {
      const store = window[area];
      for (const key of Object.keys(store)) if (key.startsWith(PREFIX) || key.startsWith('knobs:')) store.removeItem(key);
    } catch {}
  }
  location.replace('/settings/privacy/');
}

/* LIVE */

function useKept() {
  const [list, setList] = useState(stored);
  useEffect(() => {
    setList(stored());
    return subscribe(() => setList(stored()));
  }, []);
  return list;
}

/* SURE */

function Sure({ label, hint, yes, hot, go, no }) {
  return (
    <Item label={label} hint={hint}>
      <span className="priv-pair">
        <button type="button" className="priv-btn" onClick={no} autoFocus>Cancel</button>
        <button type="button" className={hot ? 'priv-btn hot' : 'priv-btn'} onClick={go}>{yes}</button>
      </span>
    </Item>
  );
}

/* PANE */

export default function Pane() {
  const list = useKept();
  const input = useRef(null);
  const [plan, setPlan] = useState(null);
  const [fault, setFault] = useState('');
  const [sure, setSure] = useState(false);
  const total = list.reduce((sum, [, value]) => sum + bytes(value), 0);

  async function choose(event) {
    const file = event.target.files?.[0];
    event.target.value = '';
    if (!file) return;
    const rows = parse(await file.text().catch(() => ''));
    setPlan(rows);
    setFault(rows ? '' : 'That file is not a Mrly data export.');
  }

  function load() {
    for (const [key, value] of plan) write(key, value);
    location.reload();
  }

  return (
    <Frame title="Privacy & Data">
      <Section title="Kept in this browser" foot={list.length ? `${count(list.length)}, ${size(total)}` : undefined}>
        <Note>Settings and layouts stay in this browser. There is no account.</Note>
        {list.length === 0 && <Note>Nothing is kept yet.</Note>}
        {list.map(([key, value]) => (
          <Item key={key} label={name(key)}>
            <span className="num dim">{size(bytes(value))}</span>
          </Item>
        ))}
        <Note>Scroll positions and open Knobs sections last only for this tab.</Note>
      </Section>
      <Section title="Export and import">
        <Action label="Export all data" hint="Saves everything above as mrly-data.json." onClick={save}>Export</Action>
        {plan ? (
          <Sure label={`Import ${count(plan.length)}?`} hint="They replace matching items here. The page reloads." yes="Import" go={load} no={() => setPlan(null)} />
        ) : (
          <Action label="Import data" hint="Choose a mrly-data.json file." onClick={() => input.current?.click()}>Import…</Action>
        )}
        {fault && <Note>{fault}</Note>}
      </Section>
      <Section title="Reset">
        {sure ? (
          <Sure label="Erase all data?" hint="This cannot be undone." yes="Erase" hot go={wipe} no={() => setSure(false)} />
        ) : (
          <Action danger label="Erase all data" hint="Removes everything kept in this browser. The lock screen plays again on your next visit home." onClick={() => setSure(true)}>Erase…</Action>
        )}
      </Section>
      <Section title="What the site sends">
        <Item label="Cookies"><span className="dim">None</span></Item>
        <Item label="Analytics and trackers"><span className="dim">None</span></Item>
        <Item label="Other sites" hint="Pages, fonts, code and data all load from mrly.net."><span className="dim">None</span></Item>
        <Item label="Your settings" hint="Except inside a settings link you copy and share."><span className="dim">Stay here</span></Item>
        <Note>Like any web host, the servers behind mrly.net see your IP address and the pages you request, in order to deliver them. The Stats page shows totals per hour, not visitors.</Note>
        <Note>Links to other sites, such as GitHub, donations and social profiles, open on those sites, which have their own rules.</Note>
      </Section>
      <input ref={input} type="file" accept="application/json,.json" hidden onChange={choose} />
    </Frame>
  );
}
