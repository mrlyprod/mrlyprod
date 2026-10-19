import { useCallback, useRef, useState } from 'react';
import { Bar } from '../../lib/app/frame.jsx';
import { Btn } from '../../lib/app/controls.jsx';
import { Art, Item, Library, Shelf } from '../../lib/app/library.jsx';
import { Scene } from '../../lib/app/page.jsx';
import { paints } from '../../lib/color.js';
import { useTheme } from '../../lib/designs/pick.jsx';
import { DESIGNS, ink, tile } from '../../lib/designs/tiles.js';
import { useStatus } from '../../lib/app/hooks.js';
import { DOWN, LEFT, RIGHT, UP } from './engine.js';
import { PARTS, PIECES, make, named } from './scene.js';

const SWIPE = 24;

const CELLS = 5;

const GAP = 0.12;

export function sample(ctx, design, w, accent) {
  ctx.fillStyle = accent;
  const room = w * (1 - 2 * GAP);
  if (design) {
    ink(ctx, tile(design, CELLS), w * GAP, w * GAP, room);
    return;
  }
  DESIGNS.slice(0, 4).forEach((one, i) => ink(ctx, tile(one, CELLS), w * GAP + (i % 2) * (room / 2), w * GAP + Math.floor(i / 2) * (room / 2), room / 2));
}

export function Widget({ value, onChange, onReady, onStatus }) {
  const live = useRef(null);
  const from = useRef(null);
  const [score, setScore] = useState(0);
  const [over, setOver] = useState(null);
  const tick = useTheme();
  const game = useCallback((canvas, view, opts) => make(canvas, view, { ...opts, player: true, onScore: setScore, onOver: setOver }), []);
  const ready = (handle) => {
    live.current = handle;
    onReady?.(handle);
  };
  const down = (e) => {
    from.current = [e.clientX, e.clientY];
    e.currentTarget.setPointerCapture?.(e.pointerId);
  };
  const move = (e) => {
    if (!from.current) return;
    const dx = e.clientX - from.current[0];
    const dy = e.clientY - from.current[1];
    if (Math.max(Math.abs(dx), Math.abs(dy)) < SWIPE) return;
    live.current?.turn(Math.abs(dx) > Math.abs(dy) ? (dx > 0 ? RIGHT : LEFT) : dy > 0 ? DOWN : UP);
    from.current = [e.clientX, e.clientY];
  };
  const up = () => {
    from.current = null;
  };
  const said = over === 'won' ? 'won' : over ? 'game over' : 'score';
  useStatus(onStatus, [[said, score]]);
  return (
    <>
      <Scene make={game} value={value} onReady={ready} onPointerDown={down} onPointerMove={move} onPointerUp={up} onPointerCancel={up} />
      <Library name="Pieces">
        {PARTS.map((part) => (
          <Shelf key={part} name={named(part)} grid>
            {PIECES.map(([design, label]) => <Item key={design || 'random'} label={label} on={value[part] === design} onPick={() => onChange({ [part]: design })} art={<Art paint={(ctx, w) => sample(ctx, design, w, paints(ctx.canvas, value.tint).accent)} deps={[design, value.tint, tick]} />} />)}
          </Shelf>
        ))}
      </Library>
      {over && value.play === 'me' && (
        <Bar side="actions">
          <Btn primary onClick={() => live.current?.restart()}>Retry</Btn>
        </Bar>
      )}
    </>
  );
}
