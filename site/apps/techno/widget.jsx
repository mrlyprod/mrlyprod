import { useCallback, useEffect, useMemo, useRef, useState } from 'react';
import { download } from '../../lib/export.js';
import { Bar } from '../../lib/frame.jsx';
import { Btn, Icon } from '../../lib/knobs.jsx';
import { Scene } from '../../lib/scene.jsx';
import { bus, engine, render, wav } from '../../lib/space/audio.js';
import { make, score, seconds } from './scene.js';

export function Widget({ value, onReady }) {
  const line = useMemo(bus, []);
  const box = useRef(null);
  const heard = useRef(0);
  const live = useRef(null);
  const loud = useRef(0);
  const going = useRef(0);
  const now = useRef(value);
  now.current = value;
  const [sound, setSound] = useState(0);
  const [playing, setPlaying] = useState(0);
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
      box.current.set({ music: going.current });
      line.attach(box.current);
    }
    if (on) box.current.start();
    box.current?.set({ sound: on, music: on ? going.current : 0 });
    loud.current = on;
    setSound(on);
  };
  const play = (on) => {
    if (on) hear(1);
    going.current = on;
    box.current?.set({ music: on });
    if (on) live.current?.wake?.();
    setPlaying(on);
  };
  const go = () => play(going.current ? 0 : 1);
  const mute = () => hear(loud.current ? 0 : 1);
  const ready = (handle) => {
    live.current = handle;
    const pause = () => {
      handle.pause();
      box.current?.set({ music: 0 });
    };
    const resume = () => {
      handle.play();
      box.current?.set({ music: loud.current ? going.current : 0 });
    };
    onReady?.(handle && { ...handle, pause, play: resume, go, mute });
  };
  const save = async () => {
    if (busy) return;
    setBusy(true);
    const one = now.current;
    try {
      download(wav(await render(seconds(one), score(one))), `techno-${one.seed}.wav`);
    } catch {}
    setBusy(false);
  };
  return (
    <>
      <Scene make={scene} value={value} onReady={ready} />
      <Bar side="actions">
        <Btn primary onClick={go}>{playing ? 'Stop' : 'Play'}</Btn>
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
