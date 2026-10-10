import { useCallback, useEffect, useMemo, useRef, useState } from 'react';
import { Bar, Frame } from './frame.jsx';
import { island } from './island.jsx';
import { Hints, Keys, useKeys } from './keys.jsx';
import { defaults as knobs, tidy } from './knobs.js';
import { Btn, Export, Group, Icon, Knob, Knobs } from './knobs.jsx';
import { share, useQuery } from './query.js';
import { run } from './scene.js';
import { record as capture } from './space/record.js';

export function Scene({ make, value, onReady, ...rest }) {
  const at = useRef(null);
  const ready = useRef(onReady);
  ready.current = onReady;
  const key = JSON.stringify(value);
  useEffect(() => {
    const canvas = at.current;
    const stop = run(canvas, make, value);
    ready.current?.({ ...stop.scene, canvas, pause: stop.pause, play: stop.play, fix: stop.fix });
    return () => {
      ready.current?.(null);
      stop();
    };
  }, [make, key]);
  return <canvas key={key} ref={at} {...rest} />;
}

const PAUSE = { key: ' ', label: 'Pause', act: 'pause' };

const RECORD = { key: 'v', label: 'Record', act: 'record' };

const RATIOS = ['9:16', '16:9'];

const clock = (secs) => `${Math.floor(secs / 60)}:${String(Math.floor(secs % 60)).padStart(2, '0')}`;

const BASE = [
  { key: 'r', label: 'Reroll', act: 'reroll' },
  { key: 'z', label: 'Zen', act: 'zen' },
  { key: '?', label: 'Keys', act: 'keys' },
];

export function quiet(on) {
  if (typeof document === 'undefined') return;
  if (on) document.documentElement.dataset.zen = '';
  else delete document.documentElement.dataset.zen;
}

const roll = () => Math.floor(Math.random() * 4294967296);

const SEED = { key: 'seed', label: 'Seed', kind: 'number', def: 0, min: 0, max: 4294967295, step: 1 };

const changed = (next, was) => Object.fromEntries(Object.entries(next).filter(([key, v]) => v !== was[key]));

export function values(defaults, spec) {
  return spec ? { ...defaults, ...knobs(spec) } : defaults;
}

function seeded(keys) {
  const params = new URLSearchParams(location.search);
  for (const key of [...params.keys()]) if (!keys.includes(key)) params.delete(key);
  if (!params.has('seed')) params.set('seed', roll());
  history.replaceState(history.state, '', `${location.pathname}?${params}${location.hash}`);
}

const keyed = (rows, key) => rows.some((row) => [].concat(row.key).includes(key));

export function toggle(handle, paused, taking, cut) {
  if (taking) {
    cut();
    return paused;
  }
  if (paused) handle.play();
  else handle.pause();
  return !paused;
}

export function resume(handle, paused) {
  if (paused) handle.play();
  return false;
}

export const keymap = (keys) => [PAUSE, ...keys, ...BASE.filter((row) => !keyed(keys, row.key))];

export const live = (map, value) => map.filter((row) => !row.when || row.when(value));

export const menu = (record, begin, more = []) => [...(record ? RATIOS.map((at) => [`Video ${at}`, () => begin(at)]) : []), ...more];

function App({ Widget, defaults, spec, keys, actions, gestures, record, id, title }) {
  const [raw, set] = useQuery(defaults);
  const value = spec ? { ...raw, ...tidy(spec, raw) } : raw;
  const scene = useRef(null);
  const [kept, keep] = useState('');
  const [paused, setPaused] = useState(false);
  const take = useRef(null);
  const ratio = useRef(RATIOS[0]);
  const [taking, setTaking] = useState('');
  const [rec, setRec] = useState('');
  const [zen, setZen] = useState(false);
  const [more, setMore] = useState([]);
  useEffect(() => {
    quiet(zen);
    if (!zen) return undefined;
    const leave = (e) => {
      if (e.key !== 'Escape' || document.querySelector('dialog[open]')) return;
      e.preventDefault();
      setZen(false);
    };
    addEventListener('keydown', leave);
    return () => {
      removeEventListener('keydown', leave);
      quiet(false);
    };
  }, [zen]);
  const write = (next) => {
    const patch = changed(next, raw);
    if (Object.keys(patch).length) set(patch);
  };
  const onChange = (patch) => {
    const next = { ...value, ...patch };
    write(spec ? { ...next, ...tidy(spec, next) } : next);
  };
  useEffect(() => write(value), []);
  const ready = useCallback((handle) => {
    take.current?.stop();
    scene.current = handle;
    setPaused(false);
  }, []);
  const all = useMemo(() => (record ? [...keys, RECORD] : keys), [keys, record]);
  const on = live(all, value);
  const map = useMemo(() => keymap(on), [all, on.map((row) => all.indexOf(row)).join()]);
  const pause = () => {
    const handle = scene.current;
    if (handle) setPaused(toggle(handle, paused, Boolean(take.current), cut));
  };
  const begin = (at) => {
    const handle = scene.current;
    if (!record || !handle?.fix || take.current) return;
    ratio.current = at;
    setPaused(resume(handle, paused));
    try {
      const one = capture(handle, { ratio: at, name: `${id}-${value.seed}`, audio: handle.sound?.() });
      take.current = one;
      setTaking(at);
      one.done.then(() => {
        if (take.current !== one) return;
        take.current = null;
        setTaking('');
      });
    } catch (error) {
      console.error(error);
    }
  };
  const cut = () => take.current?.stop();
  useEffect(() => cut, []);
  useEffect(() => {
    if (!taking) return undefined;
    const tick = () => setRec(`rec ${clock(take.current?.secs() ?? 0)} ${taking} late ${take.current?.late() ?? 0}`);
    tick();
    const timer = setInterval(tick, 250);
    return () => clearInterval(timer);
  }, [taking]);
  const acts = { pause, reroll: () => onChange({ seed: roll() }), zen: () => setZen((was) => !was), record: () => (take.current ? cut() : begin(ratio.current)), ...Object.fromEntries(Object.entries(actions).map(([act, fn]) => [act, (e) => scene.current && fn(scene.current, e)])) };
  useKeys(map, acts);
  const primary = map.find((row) => row.button && acts[row.act]);
  const pick = `${id}${share(value)}`;
  const saver = () => {
    window.dispatchEvent(new CustomEvent('saver', { detail: pick }));
    keep(pick);
  };
  return (
    <Frame label={title} gestures={gestures}>
      <Widget value={value} onChange={onChange} onReady={ready} onExport={setMore} />
      <Btn className="icon unzen" aria-label="Leave zen" onClick={acts.zen}>
        <Icon name="unzen" />
      </Btn>
      <Bar side="top">
        <Btn className="icon" aria-label="Zen" onClick={acts.zen}>
          <Icon name="zen" />
        </Btn>
      </Bar>
      {spec && (
        <Bar side="right">
          <Knobs spec={spec} value={value} onChange={onChange} />
          <Group name="Seed">
            <Knob row={{ ...SEED, def: value.seed }} value={value.seed} onChange={onChange} />
            <Btn onClick={acts.reroll}>
              <Icon name="reroll" />
              Reroll
            </Btn>
          </Group>
        </Bar>
      )}
      <Bar side="status">
        <span>seed {value.seed}</span>
        {paused && <span>paused</span>}
        {taking && <span>{rec}</span>}
      </Bar>
      <Bar side="hints">
        <Hints map={map} />
      </Bar>
      <Bar side="actions">
        {taking && (
          <Btn primary onClick={cut}>
            Stop
          </Btn>
        )}
        {primary && (
          <Btn primary onClick={acts[primary.act]}>
            {primary.label}
          </Btn>
        )}
        <Export className="export" canvas={() => scene.current?.canvas ?? document.querySelector('.frame canvas')} name={`${id}-${value.seed}`} draw={() => scene.current?.draw?.()} more={menu(record, begin, more)} />
        <Btn className="keep" onClick={saver}>{kept === pick ? 'Screensaver set' : 'Set as screensaver'}</Btn>
      </Bar>
      <Keys map={map} />
    </Frame>
  );
}

export function page(Widget, defaults, { spec, keys = [], actions = {}, gestures = false, record = false } = {}) {
  const start = values(defaults, spec);
  return island((host) => {
    const still = host.nextElementSibling;
    const title = still?.querySelector('h1')?.textContent ?? '';
    still?.remove();
    seeded(Object.keys(start));
    return { node: <App Widget={Widget} defaults={start} spec={spec} keys={keys} actions={actions} gestures={gestures} record={record} id={host.dataset.app} title={title} />, close: () => still && host.after(still) };
  });
}
