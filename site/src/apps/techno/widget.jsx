import { useCallback, useEffect, useMemo, useRef, useState } from 'react';
import { download, wav } from '../../lib/app/export.js';
import { Bar } from '../../lib/app/frame.jsx';
import { Btn, Icon } from '../../lib/app/controls.jsx';
import { Art, Item, Library, Shelf } from '../../lib/app/library.jsx';
import { Scene } from '../../lib/app/page.jsx';
import { bus, engine, pattern, render } from '../../lib/audio/audio.js';
import { paints } from '../../lib/color.js';
import { useTheme } from '../../lib/designs/pick.jsx';
import { STYLES, make, score, seconds, song, stamp, styled } from './scene.js';

export function Widget({ value, onChange, onReady, onExport }) {
  const line = useMemo(bus, []);
  const box = useRef(null);
  const heard = useRef(0);
  const live = useRef(null);
  const loud = useRef(0);
  const going = useRef(0);
  const working = useRef(false);
  const now = useRef(value);
  now.current = value;
  const theme = useTheme();
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
    onReady?.(handle && { ...handle, pause, play: resume, go, mute, going: () => going.current === 1 });
  };
  const save = async () => {
    if (working.current) return;
    working.current = true;
    const one = now.current;
    try {
      download(wav(await render(seconds(one), score(one))), `techno-${one.seed}.wav`);
    } catch {}
    working.current = false;
  };
  useEffect(() => onExport?.([['WAV', save]]), []);
  const style = styled(value);
  const seed = value.seed >>> 0;
  return (
    <>
      <Scene make={scene} value={value} onReady={ready} />
      <Library name="Styles">
        <Shelf grid>
          {STYLES.map((one) => (
            <Item key={one.id} label={one.name} sub={`${one.song.bpm} bpm`} on={style === one.id} onPick={() => onChange(one.song)} art={<Art paint={(ctx, w, h) => stamp(ctx, w, h, pattern(seed, song({ ...value, ...one.song }), 0), paints(ctx.canvas).accent)} deps={[one.id, seed, theme]} />} />
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
