import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { paints, veil } from "../../net/sdk/color.ts";
import { Btn, Icon } from "../../net/sdk/controls.tsx";
import { Bar } from "../../net/sdk/frame.tsx";
import { Art, Item, Library, Shelf } from "../../net/sdk/library.tsx";
import type { Handle, Mathy, WidgetProps } from "../../net/sdk/page.tsx";
import { Scene } from "../../net/sdk/page.tsx";
import type { Opts, View } from "../../net/sdk/run.ts";
import type { Engine } from "../shared/audio/audio.ts";
import { engine } from "../shared/audio/audio.ts";
import { useTheme } from "../shared/designs/pick.tsx";
import { globe } from "../shared/space/globe.ts";
import type { Held, Knobs, Voyage } from "./scene.ts";
import { make, PLACES, quiet, stand } from "./scene.ts";
import type { Ears } from "./score.ts";
import { levels, mixer } from "./score.ts";

/* TYPES */

type Props = WidgetProps<Mathy, Held> & { value: Knobs & { seed: number } };

const ENDS: ["from" | "to", string][] = [
  ["from", "Leave from"],
  ["to", "Go to"],
];

const WHO = 0.62;

const query = (ctx: CanvasRenderingContext2D, w: number, h: number) => {
  const { ink } = paints(ctx.canvas);
  ctx.setTransform(1, 0, 0, 1, 0, 0);
  ctx.clearRect(0, 0, w, h);
  ctx.strokeStyle = veil(ink, 0.3);
  ctx.lineWidth = Math.max(1, w / 28);
  ctx.beginPath();
  ctx.arc(w / 2, h / 2, w * 0.42, 0, Math.PI * 2);
  ctx.stroke();
  ctx.fillStyle = ink;
  ctx.font = `600 ${Math.round(w * WHO)}px sans-serif`;
  ctx.textAlign = "center";
  ctx.textBaseline = "middle";
  ctx.fillText("?", w / 2, h / 2 + w * 0.04);
};

export function Widget({ value, onChange, onReady, units }: Props) {
  const box = useRef<Engine | null>(null);
  const heard = useRef(0);
  const now = useRef(value);
  const tell = useRef(onChange);
  now.current = value;
  tell.current = onChange;
  const line = useMemo(() => mixer(() => now.current), []);
  const theme = useTheme();
  const places = useMemo(
    () => PLACES.map(([id, text]) => ({ id, text, params: stand(id, value.seed >>> 0) })),
    [value.seed],
  );
  const [broken, setBroken] = useState(false);
  const scene = useCallback(
    (canvas: HTMLCanvasElement, view: View, opts: Opts) => make(canvas, view, { ...opts, ...units, audio: line }),
    [units, line],
  );
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
  const flip = (key: keyof Ears) => {
    now.current = { ...now.current, [key]: now.current[key] ? 0 : 1 };
    hear();
    box.current?.set(levels(now.current));
    tell.current?.({ [key]: now.current[key] });
  };
  const ready = (handle: Handle<Voyage> | null) => {
    onReady?.(
      handle && {
        ...handle,
        again: () => {
          hear();
          return handle.again();
        },
        mute: () => flip("sound"),
        sound: () => {
          hear();
          return box.current?.stream() ?? null;
        },
      },
    );
  };
  if (!units?.math) return null;
  return (
    <>
      <Scene make={scene} value={quiet(value)} onReady={ready} onPointerDown={hear} />
      <Library name="Worlds">
        {ENDS.map(([end, name]) => (
          <Shelf key={end} name={name} grid>
            {places.map(({ id, text, params }) => (
              <Item
                key={id}
                label={text}
                on={value[end] === id}
                onPick={() => onChange({ [end]: id })}
                art={
                  <Art
                    paint={params ? (ctx, w, h) => globe(ctx, w, h, params) : query}
                    deps={[id, params ? (params.name === "exo" ? value.seed : 0) : theme]}
                  />
                }
              />
            ))}
          </Shelf>
        ))}
      </Library>
      {broken && (
        <Bar side="status">
          <span>no sound</span>
        </Bar>
      )}
      <Bar side="actions">
        <Btn className="short" on={value.sound === 1} aria-pressed={value.sound === 1} onClick={() => flip("sound")}>
          <Icon name="volume_up" />
          Sound
        </Btn>
        <Btn className="short" on={value.music === 1} aria-pressed={value.music === 1} onClick={() => flip("music")}>
          <Icon name="music_note" />
          Music
        </Btn>
      </Bar>
    </>
  );
}
