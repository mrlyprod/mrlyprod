import { useCallback, useRef, useState } from 'react';
import { Bar } from '../../lib/frame.jsx';
import { Btn } from '../../lib/knobs.jsx';
import { Scene } from '../../lib/scene.jsx';
import { DOWN, LEFT, RIGHT, UP } from './engine.js';
import { make } from './scene.js';

const SWIPE = 24;

export function Widget({ value, onReady }) {
  const live = useRef(null);
  const from = useRef(null);
  const [score, setScore] = useState(0);
  const [over, setOver] = useState(null);
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
  return (
    <>
      <Scene make={game} value={value} onReady={ready} onPointerDown={down} onPointerMove={move} onPointerUp={up} onPointerCancel={up} />
      <Bar side="status">
        <span>{said} {score}</span>
      </Bar>
      {over && value.play === 'me' && (
        <Bar side="actions">
          <Btn primary onClick={() => live.current?.restart()}>Retry</Btn>
        </Bar>
      )}
    </>
  );
}
