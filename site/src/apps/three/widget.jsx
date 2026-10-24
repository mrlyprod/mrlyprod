import { useCallback, useEffect, useRef, useState } from "react";
import { text } from "../../lib/app/export.js";
import { useReroll, useThree, useUnit } from "../../lib/app/hooks.js";
import { Art, Item, Library, Shelf } from "../../lib/app/library.jsx";
import { Scene } from "../../lib/app/page.jsx";
import { paints } from "../../lib/color.js";
import { roll } from "../../lib/designs/engine.js";
import { read, useTheme } from "../../lib/designs/pick.jsx";
import { rng } from "../../lib/rand.js";
import { DIM } from "../../lib/three/engine.js";
import { BLOCK, block, glance, POSES } from "../../lib/three/thumbs.js";
import { CROPS, make, name } from "./scene.js";

const WHEEL = 0.0015;
const FRESH = 80;
const LOOKS = [
  ["corner", "Corner", "1"],
  ["edge", "Edge", "2"],
  ["face", "Face", "3"],
];
const COMBINE = ["sum", "xor"];

const warn = {
  position: "absolute",
  top: 16,
  left: "50%",
  transform: "translateX(-50%)",
  maxWidth: "80%",
  textAlign: "center",
  fontSize: 13,
  opacity: 0.85,
  pointerEvents: "none",
};

export const hands = (at, onChange) => ({
  deeper: (by) => onChange({ level: at().level + by }),
  widen: (by) => onChange({ radius: at().radius + by }),
  toggle: () => onChange({ spin: Number(at().spin) === 1 ? 0 : 1 }),
  playing: () => Number(at().spin) === 1,
});

export function Widget({ value, onChange, onReady, onExport, onStatus, unit, three: given }) {
  const math = useUnit("math", unit);
  const three = useThree(true, given);
  const live = useRef(null);
  const now = useRef(value);
  const touch = useRef({ points: new Map(), mode: "turn", last: [0, 0], at: 0 });
  const [info, setInfo] = useState(null);
  const tick = useTheme();
  now.current = value;
  const tell = useCallback((next) => setInfo(next), []);
  useEffect(() => onStatus?.(info?.rows ?? []), [info]);
  const rolled = (seed) => {
    if (!math) return;
    const base = read(now.current).base;
    const rand = rng(seed);
    const patch = { code: roll(math, DIM, base, rand) };
    if (now.current.source === "field") patch.combine = COMBINE[Math.floor(rand() * COMBINE.length)];
    onChange(patch);
  };
  useReroll(value.seed, rolled);
  const mode = useRef(`${value.source} ${Boolean(value.crop)}`);
  useEffect(() => {
    const next = `${value.source} ${Boolean(value.crop)}`;
    if (mode.current === next) return;
    const calm = value.source === "field" || Boolean(value.crop);
    const was = mode.current;
    mode.current = next;
    if (calm && was === "design false") onChange({ spin: 0 });
  }, [value.source, value.crop]);
  const wheel = useRef(null);
  const ready = (handle) => {
    live.current = handle;
    wheel.current?.();
    wheel.current = null;
    const canvas = handle?.canvas;
    if (canvas?.addEventListener) {
      const wheeled = (e) => {
        e.preventDefault();
        live.current?.zoom?.(Math.exp(-e.deltaY * WHEEL));
      };
      const menu = (e) => e.preventDefault();
      canvas.addEventListener("wheel", wheeled, { passive: false });
      canvas.addEventListener("contextmenu", menu);
      wheel.current = () => {
        canvas.removeEventListener("wheel", wheeled);
        canvas.removeEventListener("contextmenu", menu);
      };
    }
    onReady?.(handle && { ...handle, ...hands(() => now.current, onChange) });
  };
  useEffect(() => () => wheel.current?.(), []);
  const down = (e) => {
    const state = touch.current;
    state.points.set(e.pointerId, [e.clientX, e.clientY]);
    state.mode = e.button === 2 || e.shiftKey ? "pan" : "turn";
    state.last = [0, 0];
    e.currentTarget.setPointerCapture?.(e.pointerId);
  };
  const move = (e) => {
    const state = touch.current;
    const from = state.points.get(e.pointerId);
    if (!from) return;
    const rect = e.currentTarget.getBoundingClientRect();
    const scene = live.current;
    if (state.points.size === 2) {
      const gap = () => {
        const [a, b] = [...state.points.values()];
        return Math.hypot(a[0] - b[0], a[1] - b[1]);
      };
      const before = gap();
      state.points.set(e.pointerId, [e.clientX, e.clientY]);
      if (before > 0) scene?.zoom?.(gap() / before);
      scene?.pan?.((e.clientX - from[0]) / rect.width / 2, (e.clientY - from[1]) / rect.height / 2);
      return;
    }
    const dx = (e.clientX - from[0]) / rect.width;
    const dy = (e.clientY - from[1]) / rect.height;
    state.points.set(e.pointerId, [e.clientX, e.clientY]);
    if (state.mode === "pan") scene?.pan?.(dx, dy);
    else scene?.turn?.(dx, dy);
    state.last = [dx, dy];
    state.at = e.timeStamp;
  };
  const up = (e) => {
    const state = touch.current;
    const had = state.points.size;
    state.points.delete(e.pointerId);
    if (had === 1 && state.mode === "turn" && e.timeStamp - state.at < FRESH) live.current?.fling?.(...state.last);
  };
  const keep = (body, kind) => body && text(body, name(now.current), kind);
  const obj = () => keep(live.current?.obj?.(), "obj");
  useEffect(() => onExport?.(math ? [["OBJ", obj]] : []), [Boolean(math)]);
  const scene = useCallback((canvas, view, opts) => make(canvas, view, { ...opts, math, three, tell }), [math, three]);
  const tone = (canvas) => paints(canvas, value.tint);
  return (
    <>
      {math && three && (
        <Scene
          make={scene}
          value={value}
          onReady={ready}
          onPointerDown={down}
          onPointerMove={move}
          onPointerUp={up}
          onPointerCancel={up}
        />
      )}
      {info?.error && <p style={warn}>{info.error}</p>}
      {math && (
        <Library>
          <Shelf name="Look from" grid>
            {LOOKS.map(([key, label, hint]) => (
              <Item
                key={key}
                label={label}
                sub={hint}
                onPick={() => live.current?.look?.(key)}
                art={
                  <Art
                    deps={[tick, value.tint]}
                    paint={(ctx, side) => glance(ctx, BLOCK, POSES[key], side, tone(ctx.canvas))}
                  />
                }
              />
            ))}
          </Shelf>
          <Shelf name="Crop" grid>
            {CROPS.map(([key, label]) => (
              <Item
                key={key || "none"}
                label={label}
                on={(value.crop || "") === key}
                onPick={() => onChange({ crop: key })}
                art={
                  <Art
                    deps={[tick, value.tint, key]}
                    paint={(ctx, side) => glance(ctx, block(math, key), POSES.corner, side, tone(ctx.canvas))}
                  />
                }
              />
            ))}
          </Shelf>
        </Library>
      )}
    </>
  );
}
