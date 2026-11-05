import { Btn, Icon } from "@mrlyprod/ui/kit/controls.tsx";
import { download, wav } from "@mrlyprod/ui/kit/export.ts";
import { Bar } from "@mrlyprod/ui/kit/frame.tsx";
import { Item, Library, Shelf } from "@mrlyprod/ui/kit/library.tsx";
import type { Handle, WidgetProps } from "@mrlyprod/ui/kit/page.tsx";
import { Scene } from "@mrlyprod/ui/kit/page.tsx";
import type { Opts, View } from "@mrlyprod/ui/kit/run.ts";
import type { Units } from "@mrlyprod/ui/kit/spec.ts";
import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import type { Engine } from "../shared/audio/audio.ts";
import { bus, engine, render } from "../shared/audio/audio.ts";
import type { Held, Sfx, Sound } from "./scene.ts";
import { facts, lasts, make, NAMES, named, PRESETS, preset, score, seconds } from "./scene.ts";

type Props = WidgetProps<Units, Held> & { value: Sound };

export function Widget({ value, onChange, onReady, onExport, onStatus }: Props) {
  const line = useMemo(bus, []);
  const box = useRef<Engine | null>(null);
  const heard = useRef(0);
  const live = useRef<Handle<Sfx> | null>(null);
  const loud = useRef(0);
  const now = useRef(value);
  now.current = value;
  const working = useRef(false);
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
      line.attach(box.current);
    }
    if (on) box.current!.start();
    box.current?.set({ sound: on });
    loud.current = on;
    setSound(on);
  };
  const go = () => {
    hear(1);
    live.current?.cue();
  };
  const mute = () => hear(loud.current ? 0 : 1);
  const ready = (handle: Handle<Sfx> | null) => {
    live.current = handle;
    onReady?.(handle && { ...handle, go, mute });
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
  useEffect(() => onExport?.([["WAV", save]]), []);
  const kept = preset(value);
  useEffect(() => onStatus?.(facts(value)), [value.effect, value.length, kept]);
  return (
    <>
      <Scene make={scene} value={value} onReady={ready} />
      <Library name="Sounds">
        <Shelf name="Effects">
          {NAMES.map((name) => (
            <Item
              key={name}
              label={named(name)}
              sub={lasts(name)}
              on={value.effect === name}
              onPick={() => onChange({ effect: name })}
            />
          ))}
        </Shelf>
        <Shelf name="Presets">
          {PRESETS.map((one) => (
            <Item
              key={one.id}
              label={one.name}
              sub={one.set.effect}
              on={kept === one.id}
              onPick={() => onChange(one.set)}
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
