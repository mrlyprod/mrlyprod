import { useCallback, useEffect, useMemo, useRef, useState } from 'react';
import { Bar } from '../../lib/frame.jsx';
import { Btn, Icon } from '../../lib/knobs.jsx';
import { Scene } from '../../lib/scene.jsx';
import { engine } from '../../lib/space/audio.js';
import { make, quiet } from './scene.js';
import { levels, mixer } from './score.js';

export function Widget({ value, onChange, onReady }) {
  const box = useRef(null);
  const heard = useRef(0);
  const now = useRef(value);
  const tell = useRef(onChange);
  now.current = value;
  tell.current = onChange;
  const line = useMemo(() => mixer(() => now.current), []);
  const [broken, setBroken] = useState(false);
  const scene = useCallback((canvas, view, opts) => make(canvas, view, { ...opts, audio: line }), [line]);
  const wake = () => {
    try {
      const seed = now.current.seed >>> 0;
      if (box.current && heard.current !== seed) {
        line.detach();
        box.current.stop();
        box.current = null;
      }
      if (!box.current) {
        heard.current = seed;
        box.current = engine({ seed });
        line.attach(box.current);
      }
      box.current.start();
      box.current.set(levels(now.current));
      setBroken(false);
    } catch (error) {
      console.error(error);
      setBroken(true);
    }
  };
  const hear = () => {
    if (now.current.sound || now.current.music) wake();
  };
  useEffect(() => {
    box.current?.set(levels(value));
  }, [value.sound, value.music]);
  useEffect(
    () => () => {
      line.detach();
      box.current?.stop();
      box.current = null;
    },
    [line],
  );
  const flip = (key) => {
    now.current = { ...now.current, [key]: now.current[key] ? 0 : 1 };
    hear();
    box.current?.set(levels(now.current));
    tell.current?.({ [key]: now.current[key] });
  };
  const ready = (handle) => {
    onReady?.(handle && { ...handle, again: () => (hear(), handle.again()), mute: () => flip('sound'), sound: () => (hear(), box.current?.stream() ?? null) });
  };
  return (
    <>
      <Scene make={scene} value={quiet(value)} onReady={ready} onPointerDown={hear} />
      {broken && (
        <Bar side="status">
          <span>no sound</span>
        </Bar>
      )}
      <Bar side="actions">
        <Btn className="short" on={value.sound === 1} aria-pressed={value.sound === 1} onClick={() => flip('sound')}>
          <Icon name="sound" />
          Sound
        </Btn>
        <Btn className="short" on={value.music === 1} aria-pressed={value.music === 1} onClick={() => flip('music')}>
          <Icon name="music" />
          Music
        </Btn>
      </Bar>
    </>
  );
}
