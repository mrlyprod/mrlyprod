import { useCallback, useEffect, useMemo, useRef, useState } from 'react';
import { download } from '../../lib/export.js';
import { Bar } from '../../lib/frame.jsx';
import { Btn, Icon } from '../../lib/knobs.jsx';
import { Scene } from '../../lib/scene.jsx';
import { bus, engine, render, wav } from '../../lib/space/audio.js';
import { NAMES, make, score, seconds } from './scene.js';

export function Widget({ value, onChange, onReady }) {
  const line = useMemo(bus, []);
  const box = useRef(null);
  const heard = useRef(0);
  const live = useRef(null);
  const loud = useRef(0);
  const now = useRef(value);
  const tell = useRef(onChange);
  now.current = value;
  tell.current = onChange;
  const [sound, setSound] = useState(0);
  const [busy, setBusy] = useState(false);
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
    if (busy) return;
    setBusy(true);
    const one = now.current;
    try {
      download(wav(await render(seconds(one), score(one))), `sfx-${one.effect}-${one.seed}.wav`);
    } catch {}
    setBusy(false);
  };
  return (
    <>
      <Scene make={scene} value={value} onReady={ready} />
      <Bar side="actions">
        <Btn className="short" on={sound === 1} aria-pressed={sound === 1} onClick={mute}>
          <Icon name="sound" />
          Sound
        </Btn>
        <Btn className="short" onClick={save} disabled={busy}>
          <Icon name="wave" />
          WAV
        </Btn>
      </Bar>
    </>
  );
}
