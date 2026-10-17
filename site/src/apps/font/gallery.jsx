import { useEffect, useRef, useState } from 'react';
import { Btn } from '../../lib/app/controls.jsx';
import { HUES } from '../../lib/color.js';
import { marks } from '../../lib/app/svg.js';
import { SLOT, pen } from './engine.js';

const ALL = 'all';

const GRID = { display: 'grid', gridTemplateColumns: 'repeat(auto-fill, minmax(5.5rem, 1fr))', gap: 'var(--s2)', padding: '0 var(--s2)' };

const FILTER = { padding: '0 var(--s2)' };

const CARD = { padding: 'var(--s2)', textAlign: 'center' };

const still = () => typeof matchMedia === 'function' && matchMedia('(prefers-reduced-motion: reduce)').matches;

const mouse = (e) => e.pointerType === 'mouse';

export function Mark({ glyph, tint = '', k = null }) {
  const fill = `var(--${HUES.includes(tint) ? tint : 'accent'})`;
  const cells = glyph.frames[k ?? glyph.frames.length - 1];
  const dx = Math.floor((SLOT - glyph.cols) / 2);
  const dy = Math.floor((SLOT - glyph.rows) / 2);
  return (
    <svg viewBox={`-1 -1 ${SLOT + 2} ${SLOT + 2}`} shapeRendering="crispEdges" style={{ fill }} aria-hidden="true">
      {marks(cells, glyph.cols).map(([x, y]) => <rect key={`${x},${y}`} x={x + dx} y={y + dy} width="1" height="1" />)}
    </svg>
  );
}

function Card({ glyph, tint, rate, hold, onPick }) {
  const [k, setK] = useState(null);
  const timer = useRef(0);
  const rest = () => {
    clearInterval(timer.current);
    timer.current = 0;
  };
  const write = (once) => {
    const n = glyph.frames.length;
    if (n < 2 || still()) return;
    rest();
    let tick = 0;
    setK(0);
    timer.current = setInterval(() => {
      tick++;
      if (once && tick >= n - 1) {
        rest();
        setK(null);
      } else setK(pen(tick, n, hold));
    }, 1000 / rate);
  };
  const done = () => {
    rest();
    setK(null);
  };
  const pick = () => {
    if (!timer.current) write(true);
    onPick(glyph.char);
  };
  useEffect(() => rest, []);
  return (
    <button
      type="button"
      className={glyph.flag ? 'card on' : 'card'}
      style={CARD}
      title={glyph.name}
      aria-label={`${glyph.name}, ${glyph.strokes} strokes of a floor of ${glyph.floor}${glyph.flag ? `, ${glyph.flag}` : ''}: add it to the text`}
      onClick={pick}
      onPointerEnter={(e) => mouse(e) && write(false)}
      onPointerLeave={(e) => mouse(e) && done()}
      onFocus={(e) => e.currentTarget.matches?.(':focus-visible') && write(false)}
      onBlur={done}
    >
      <Mark glyph={glyph} tint={tint} k={k} />
      <div className="code">{glyph.label}</div>
      <div className="name">{glyph.strokes} of {glyph.floor}</div>
      {glyph.flag && <span className="chip refuted">{glyph.flag}</span>}
    </button>
  );
}

export function Gallery({ kinds, tint, rate, hold, onPick }) {
  const [kind, setKind] = useState(ALL);
  const shown = kinds.filter((one) => kind === ALL || one.id === kind).flatMap((one) => one.glyphs);
  return (
    <section aria-label="Pens">
      <h3>Pens</h3>
      <div className="filter" style={FILTER}>
        <Btn on={kind === ALL} onClick={() => setKind(ALL)}>All<span>{kinds.reduce((n, one) => n + one.glyphs.length, 0)}</span></Btn>
        {kinds.map((one) => (
          <Btn key={one.id} on={kind === one.id} onClick={() => setKind(one.id)}>{one.name}<span>{one.glyphs.length}</span></Btn>
        ))}
      </div>
      <div style={GRID}>
        {shown.map((glyph) => <Card key={glyph.char} glyph={glyph} tint={tint} rate={rate} hold={hold} onPick={onPick} />)}
      </div>
    </section>
  );
}
