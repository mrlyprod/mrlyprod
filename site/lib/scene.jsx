import { useCallback, useEffect, useMemo, useRef, useState } from 'react';
import { Bar, Frame } from './frame.jsx';
import { island } from './island.jsx';
import { Hints, Keys, useKeys } from './keys.jsx';
import { defaults as knobs, tidy } from './knobs.js';
import { Btn, Export, Group, Icon, Knob, Knobs } from './knobs.jsx';
import { share, useQuery } from './query.js';
import { run } from './scene.js';

export function Scene({ make, value, onReady, ...rest }) {
  const at = useRef(null);
  const ready = useRef(onReady);
  ready.current = onReady;
  const key = JSON.stringify(value);
  useEffect(() => {
    const canvas = at.current;
    const stop = run(canvas, make, value);
    ready.current?.({ ...stop.scene, canvas, pause: stop.pause, play: stop.play });
    return () => {
      ready.current?.(null);
      stop();
    };
  }, [make, key]);
  return <canvas key={key} ref={at} {...rest} />;
}

const PAUSE = { key: ' ', label: 'Pause', act: 'pause' };

const BASE = [
  { key: 'r', label: 'Reroll', act: 'reroll' },
  { key: '?', label: 'Keys', act: 'keys' },
];

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

export const keymap = (keys) => [PAUSE, ...keys, ...BASE.filter((row) => !keyed(keys, row.key))];

function App({ Widget, defaults, spec, keys, actions, gestures, id, title }) {
  const [raw, set] = useQuery(defaults);
  const value = spec ? { ...raw, ...tidy(spec, raw) } : raw;
  const scene = useRef(null);
  const [kept, keep] = useState('');
  const [paused, setPaused] = useState(false);
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
    scene.current = handle;
    setPaused(false);
  }, []);
  const map = useMemo(() => keymap(keys), [keys]);
  const pause = () => {
    const handle = scene.current;
    if (!handle) return;
    if (paused) handle.play();
    else handle.pause();
    setPaused(!paused);
  };
  const acts = { pause, reroll: () => onChange({ seed: roll() }), ...Object.fromEntries(Object.entries(actions).map(([act, fn]) => [act, (e) => scene.current && fn(scene.current, e)])) };
  useKeys(map, acts);
  const primary = map.find((row) => row.button && acts[row.act]);
  const pick = `${id}${share(value)}`;
  const saver = () => {
    window.dispatchEvent(new CustomEvent('saver', { detail: pick }));
    keep(pick);
  };
  return (
    <Frame label={title} gestures={gestures}>
      <Widget value={value} onChange={onChange} onReady={ready} />
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
      </Bar>
      <Bar side="hints">
        <Hints map={map} />
      </Bar>
      <Bar side="actions">
        {primary && (
          <Btn primary onClick={acts[primary.act]}>
            {primary.label}
          </Btn>
        )}
        <Export className="export" canvas={() => scene.current?.canvas ?? document.querySelector('.frame canvas')} name={`${id}-${value.seed}`} draw={() => scene.current?.draw?.()} />
        <Btn className="keep" onClick={saver}>{kept === pick ? 'Screensaver set' : 'Set as screensaver'}</Btn>
      </Bar>
      <Keys map={map} />
    </Frame>
  );
}

export function page(Widget, defaults, { spec, keys = [], actions = {}, gestures = false } = {}) {
  const start = values(defaults, spec);
  return island((host) => {
    const still = host.nextElementSibling;
    const title = still?.querySelector('h1')?.textContent ?? '';
    still?.remove();
    seeded(Object.keys(start));
    return { node: <App Widget={Widget} defaults={start} spec={spec} keys={keys} actions={actions} gestures={gestures} id={host.dataset.app} title={title} />, close: () => still && host.after(still) };
  });
}
