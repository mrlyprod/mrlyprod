import { useEffect, useRef } from "react";
import { Fold } from "../app/controls.jsx";
import { Bar } from "../app/frame.jsx";
import { useUnits } from "../app/hooks.js";
import { gist, Knob } from "../app/knobs.jsx";
import { paints, veil } from "../color.js";
import { useTheme } from "../designs/pick.jsx";
import { wasm } from "../units/index.js";
import { runs } from "./scene.js";

const LIFE = wasm("life", "math");

const SPOT = 160;

const DOT = 16;

const STRIP = 12;

const GROUND = 0.1;

const at = (e) => {
  const box = e.currentTarget.getBoundingClientRect();
  return [(e.clientX - box.left) / box.width, (e.clientY - box.top) / box.height];
};

export const useLife = () => useUnits(LIFE);

export function usePaint(handle) {
  const painting = useRef(false);
  const down = (e) => {
    if (e.button) return;
    e.currentTarget.setPointerCapture?.(e.pointerId);
    painting.current = true;
    handle.current?.touch?.(...at(e), true);
  };
  const move = (e) => {
    if (painting.current) handle.current?.touch?.(...at(e), false);
  };
  const up = () => {
    painting.current = false;
  };
  return { onPointerDown: down, onPointerMove: move, onPointerUp: up, onPointerCancel: up };
}

export function Pane({ spec, value, onChange, folds }) {
  return (
    <Bar side="right">
      {folds.filter(Boolean).map(({ name, items = [], live }) => {
        const rows = items.filter((item) => typeof item === "string").map((key) => spec.find((row) => row.key === key));
        return (
          <Fold key={name} name={name} live={live ?? gist(rows, value)}>
            {items.map((item) => {
              if (typeof item !== "string") return item;
              const row = rows.find((one) => one.key === item);
              return row ? <Knob key={item} row={row} value={value[item]} values={value} onChange={onChange} /> : null;
            })}
          </Fold>
        );
      })}
    </Bar>
  );
}

export function MaskView({ mask, dot = DOT, label = "The neighbourhood mask" }) {
  const at = useRef(null);
  const tick = useTheme();
  useEffect(() => {
    const canvas = at.current;
    if (!canvas || !mask) return;
    const [rows, cols] = mask.shape;
    const dpr = Math.min(globalThis.devicePixelRatio || 1, 2);
    const cell = Math.max(1, Math.min(Math.floor((SPOT * dpr) / cols), dot * dpr));
    const high = rows === 1 ? Math.max(cell, STRIP * dpr) : cell;
    canvas.width = cols * cell;
    canvas.height = rows * high;
    canvas.style.width = `${(cols * cell) / dpr}px`;
    canvas.style.height = `${(rows * high) / dpr}px`;
    const ctx = canvas.getContext("2d");
    const { accent } = paints(canvas);
    ctx.fillStyle = veil(accent, GROUND);
    ctx.fillRect(0, 0, canvas.width, canvas.height);
    ctx.fillStyle = accent;
    runs(ctx, mask.data, cols, rows, 0, 0, cell, high);
  }, [mask, dot, tick]);
  return <canvas ref={at} className="mask" role="img" aria-label={label} style={{ display: "block" }} />;
}
