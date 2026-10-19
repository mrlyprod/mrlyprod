import { useCallback, useEffect, useMemo, useRef, useState } from 'react';
import { download, wav } from '../../lib/app/export.js';
import { Bar } from '../../lib/app/frame.jsx';
import { Btn, Icon } from '../../lib/app/controls.jsx';
import { Item, Library, Shelf } from '../../lib/app/library.jsx';
import { Scene } from '../../lib/app/page.jsx';
import { bus, engine, render } from '../../lib/audio/audio.js';
import { NAMES, PRESETS, facts, lasts, make, named, preset, score, seconds } from './scene.js';

export function Widget({ value, onChange, onReady, onExport, onStatus }) {
  const line = useMemo(bus, []);
  const box = useRef(null);
  const heard = useRef(0);
  const live = useRef(null);
  const loud = useRef(0);
  const now = useRef(value);
  const tell = useRef(onChange);
  now.current = value;
  tell.current = onChange;
  const working = useRef(false);
  const [sound, setSound] = useState(0);
  const scene = useCallback((canvas, view, opts) => make(canvas, view, { ...opts, audio: line }), [line]);
  useEffect(
    () => () => {
      line.detach();
      box.current?.stop();
      box.current = null;
    },
    [line],
  );
  const hear = (on) => {
    const seed = now.current.seed >>> 0;
    if (on && box.current && heard.current !== seed) {
      line.detach();
      box.current.stop();
      box.current = null;
    }
    if (on && !box.current) {
      heard.current = seed;
      box.current = engine({ seed });
      line.attach(box.current);
    }
    if (on) box.current.start();
    box.current?.set({ sound: on });
    loud.current = on;
    setSound(on);
  };
  const go = () => {
    hear(1);
    live.current?.cue();
  };
  const mute = () => hear(loud.current ? 0 : 1);
  const step = (by) => {
    const at = NAMES.indexOf(now.current.effect);
    tell.current?.({ effect: NAMES[(at + by + NAMES.length) % NAMES.length] });
  };
  const ready = (handle) => {
    live.current = handle;
    onReady?.(handle && { ...handle, go, mute, step });
  };
  const save = async () => {
    if (working.current) return;
    working.current = true;
    const one = now.current;
    try {
      download(wav(await render(seconds(one), score(one))), `sfx-${one.effect}-${one.seed}.wav`);
    } catch {}
    working.current = false;
  };
  useEffect(() => onExport?.([['WAV', save]]), []);
  const kept = preset(value);
  useEffect(() => onStatus?.(facts(value)), [value.effect, value.length, kept]);
  return (
    <>
      <Scene make={scene} value={value} onReady={ready} />
      <Library name="Sounds">
        <Shelf name="Effects">
          {NAMES.map((name) => (
            <Item key={name} label={named(name)} sub={lasts(name)} on={value.effect === name} onPick={() => onChange({ effect: name })} />
          ))}
        </Shelf>
        <Shelf name="Presets">
          {PRESETS.map((one) => (
            <Item key={one.id} label={one.name} sub={one.set.effect} on={kept === one.id} onPick={() => onChange(one.set)} />
          ))}
        </Shelf>
      </Library>
      <Bar side="actions">
        <Btn className="short" on={sound === 1} aria-pressed={sound === 1} onClick={mute}>
          <Icon name="volume_up" />
          Sound
        </Btn>
      </Bar>
    </>
  );
}
