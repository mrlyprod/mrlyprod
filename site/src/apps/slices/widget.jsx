import { useCallback, useEffect, useRef, useState } from 'react';
import { text } from '../../lib/app/export.js';
import { Art, Item, Library, Shelf } from '../../lib/app/library.jsx';
import { Scene } from '../../lib/app/page.jsx';
import { paints } from '../../lib/color.js';
import { rng } from '../../lib/rand.js';
import { roll } from '../../lib/designs/engine.js';
import { read, useTheme } from '../../lib/designs/pick.jsx';
import { DIM } from '../../lib/three/engine.js';

import { TOP, codesOf, glance, make, name, thumb } from './scene.js';
import { useReroll, useUnit } from '../../lib/app/hooks.js';

const SIDES = Array.from({ length: TOP }, (_, i) => i + 1);

export function Widget({ value, onChange, onReady, onExport, onStatus, unit }) {
  const math = useUnit('math', unit);
  const live = useRef(null);
  const now = useRef(value);
  const held = useRef(false);
  const [rows, setRows] = useState([]);
  const tick = useTheme();
  now.current = value;
  const tell = useCallback((next) => setRows(next.rows), []);
  useEffect(() => onStatus?.(rows), [rows]);
  useReroll(value.seed, (seed) => math && onChange({ code: roll(math, DIM, read(now.current).base, rng(seed)) }));
  const ready = (handle) => {
    live.current = handle;
    const hands = {
      step: (by) => onChange({ k: Math.min(TOP, Math.max(1, Number(now.current.tile) ? now.current.k : now.current.k + by)), tile: 0 }),
      deeper: (by) => onChange({ level: now.current.level + by }),
    };
    onReady?.(handle && { ...handle, ...hands });
  };
  const at = (e) => {
    const canvas = e.currentTarget;
    const rect = canvas.getBoundingClientRect();
    return live.current?.seek?.(((e.clientX - rect.left) / rect.width) * canvas.width, ((e.clientY - rect.top) / rect.height) * canvas.height) ?? null;
  };
  const go = (k) => k !== null && (k !== now.current.k || Number(now.current.tile) !== 0) && onChange({ k, tile: 0 });
  const down = (e) => {
    const k = at(e);
    if (k === null) return;
    held.current = true;
    e.currentTarget.setPointerCapture?.(e.pointerId);
    go(k);
  };
  const move = (e) => held.current && go(at(e));
  const up = () => {
    held.current = false;
  };
  const svg = () => live.current?.svg?.().then((body) => text(body, name(now.current), 'svg'));
  useEffect(() => onExport?.(math ? [['SVG', svg]] : []), [Boolean(math)]);
  const scene = useCallback((canvas, view, opts) => make(canvas, view, { ...opts, math, tell }), [math]);
  const tiled = Number(value.tile) !== 0;
  return (
    <>
      {math && <Scene make={scene} value={value} onReady={ready} onPointerDown={down} onPointerMove={move} onPointerUp={up} onPointerCancel={up} />}
      {math && (
        <Library>
          <Shelf name="Sides" grid>
            {SIDES.map((k) => (
              <Item key={k} label={`k ${k}`} sub={`side ${2 * k - 1}`} on={!tiled && value.k === k} onPick={() => go(k)} art={<Art deps={[tick, value.base, value.code, k]} paint={(ctx, side) => thumb(ctx, side, glance(math, value, k), codesOf(math), paints(ctx.canvas).accent)} />} />
            ))}
          </Shelf>
        </Library>
      )}
    </>
  );
}
