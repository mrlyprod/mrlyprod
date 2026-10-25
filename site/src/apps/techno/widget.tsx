import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { Btn, Icon } from "../../lib/app/controls.tsx";
import { download, wav } from "../../lib/app/export.ts";
import { Bar } from "../../lib/app/frame.tsx";
import { Art, Item, Library, Shelf } from "../../lib/app/library.tsx";
import type { Handle, WidgetProps } from "../../lib/app/page.tsx";
import { Scene } from "../../lib/app/page.tsx";
import type { Opts, View } from "../../lib/app/run.ts";
import type { Units } from "../../lib/app/spec.ts";
import type { Engine } from "../../lib/audio/audio.ts";
import { bus, engine, pattern, render } from "../../lib/audio/audio.ts";
import type { Song } from "../../lib/audio/tempo.ts";
import { paints } from "../../lib/color.ts";
import { useTheme } from "../../lib/designs/pick.tsx";
import type { Held, Techno } from "./scene.ts";
import { make, STYLES, score, seconds, song, stamp, styled } from "./scene.ts";

/* TYPES */

type Props = WidgetProps<Units, Held> & { value: Song };

export function Widget({ value, onChange, onReady, onExport }: Props) {
  const line = useMemo(bus, []);
  const box = useRef<Engine | null>(null);
  const heard = useRef(0);
  const live = useRef<Handle<Techno> | null>(null);
  const loud = useRef(0);
  const going = useRef(0);
  const working = useRef(false);
  const now = useRef(value);
  now.current = value;
  const theme = useTheme();
  const [sound, setSound] = useState(0);
  const scene = useCallback(
    (canvas: HTMLCanvasElement, view: View, opts: Opts) => make(canvas, view, { ...opts, audio: line }),
    [line],
  );
  useEffect(
    () => () => {
      line.detach();
      box.current?.stop();
      box.current = null;
    },
    [line],
  );
  const hear = (on: number) => {
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
    if (on) box.current!.start();
    box.current?.set({ sound: on, music: on ? going.current : 0 });
    loud.current = on;
    setSound(on);
  };
  const play = (on: number) => {
    if (on) hear(1);
    going.current = on;
    box.current?.set({ music: on });
    if (on) live.current?.wake?.();
  };
  const go = () => play(going.current ? 0 : 1);
  const mute = () => hear(loud.current ? 0 : 1);
  const ready = (handle: Handle<Techno> | null) => {
    live.current = handle;
    const pause = () => {
      handle!.pause();
      box.current?.set({ music: 0 });
    };
    const resume = () => {
      handle!.play();
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
  useEffect(() => onExport?.([["WAV", save]]), []);
  const style = styled(value);
  const seed = value.seed >>> 0;
  return (
    <>
      <Scene make={scene} value={value} onReady={ready} />
      <Library name="Styles">
        <Shelf grid>
          {STYLES.map((one) => (
            <Item
              key={one.id}
              label={one.name}
              sub={`${one.song.bpm} bpm`}
              on={style === one.id}
              onPick={() => onChange(one.song)}
              art={
                <Art
                  paint={(ctx, w, h) =>
                    stamp(ctx, w, h, pattern(seed, song({ ...value, ...one.song }), 0), paints(ctx.canvas).accent)
                  }
                  deps={[one.id, seed, theme]}
                />
              }
            />
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
