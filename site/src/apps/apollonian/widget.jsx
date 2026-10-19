import { useCallback, useEffect, useRef } from 'react';
import { text } from '../../lib/app/export.js';
import { Art, Item, Library, Shelf } from '../../lib/app/library.jsx';
import { Scene } from '../../lib/app/page.jsx';
import { paints } from '../../lib/color.js';
import { useTheme } from '../../lib/designs/pick.jsx';
import { rng } from '../../lib/rand.js';
import { ROOTS, quadruple, roll, said, step } from './engine.js';
import { file, make, portrait } from './scene.js';
import { useReroll } from '../../lib/app/hooks.js';

const ART = 112;

const at = (e) => {
  const box = e.currentTarget.getBoundingClientRect();
  return [(e.clientX - box.left) / box.width, (e.clientY - box.top) / box.height];
};

function stack(s) {
  if (!s) return [];
  return [['stack', `Q = ${s.order}`], ['brightness', `${s.bright} of Q(Q + 1)/2 = ${s.want}`], ['nodes lit', String(s.nodes)], ['touched', String(s.touched)], ['missed', String(s.missed)], [`off-Ford below ${s.reach}`, String(s.offford)]];
}

export function rows(f) {
  if (!f) return [];
  const line = f.line === null ? [['line', 'none in this root, so no Ford circles and no stack']] : [['Ford', `${f.ford} of ${f.line} on the line`]];
  const census = [['drawn', String(f.drawn)], ['quadruples', String(f.quads)], ['broken', String(f.broken)], ['strayed', String(f.strayed)]];
  const off = f.line === null ? [] : [['off-Ford', String(f.line - f.ford)]];
  return [...(f.circle ? [['circle', said(f.circle)]] : []), ['root', f.title], ['N(T)', `${f.circles} to T = ${f.cap}`], ...line, ...stack(f.shadow), ...census, ...off];
}

export function Widget({ value, onChange, onReady, onExport, onStatus, units }) {
  const unit = units?.num;
  const live = useRef(null);
  const now = useRef(value);
  now.current = value;
  const hooks = useRef({});
  hooks.current = { onChange, onStatus };
  const tick = useTheme();
  useReroll(value.seed, (next) => hooks.current.onChange?.(roll(rng(next))));
  const tell = useCallback((facts) => hooks.current.onStatus?.(rows(facts)), []);
  const scene = useCallback((canvas, view, opts) => make(canvas, view, { ...opts, num: unit, onTell: tell }), [unit, tell]);
  const deeper = (by) => {
    const cap = step(now.current.cap, by);
    if (cap !== now.current.cap) hooks.current.onChange?.({ cap });
  };
  const ready = (handle) => {
    live.current = handle;
    if (!handle) hooks.current.onStatus?.([]);
    onReady?.(handle && { ...handle, deeper });
  };
  const down = (e) => {
    if (e.button) return;
    live.current?.tap(...at(e));
  };
  const keep = () => {
    const picture = live.current?.svg?.();
    if (picture) text(picture, file(now.current), 'svg');
  };
  const dump = () => {
    const list = live.current?.json?.();
    if (list) text(list, file(now.current), 'json');
  };
  const have = Boolean(unit);
  useEffect(() => onExport?.(have ? [['SVG', keep], ['JSON', dump]] : []), [have]);
  return (
    <>
      {unit && (
        <Library>
          <Shelf name="Packings" grid>
            {ROOTS.map((root) => (
              <Item
                key={root}
                label={quadruple(root)}
                sub={root === 'strip' ? 'strip' : 'bounded'}
                on={value.root === root}
                onPick={() => onChange?.({ root })}
                art={<Art px={ART} paint={(ctx, w, h, dpr) => portrait(unit, root, ctx, w, h, dpr, paints(ctx.canvas, value.tint))} deps={[unit, value.tint, tick]} />}
              />
            ))}
          </Shelf>
        </Library>
      )}
      {unit && <Scene make={scene} value={value} onReady={ready} onPointerDown={down} />}
    </>
  );
}
