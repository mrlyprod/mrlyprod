import { useEffect, useMemo, useRef, useState } from 'react';
import { Btn } from '../../lib/knobs.jsx';
import { HUES } from '../../lib/scene.js';
import { gallery } from './engine.js';
import { marks } from '../../lib/svg.js';

const ALL = 'all';

const GRID = { display: 'grid', gridTemplateColumns: 'repeat(auto-fill, minmax(6.5rem, 1fr))', gap: 'var(--s2)', padding: '0 var(--s2)' };

const FILTER = { padding: '0 var(--s2)' };

const CARD = { padding: 'var(--s2)', textAlign: 'center' };

const lifts = (n) => (n ? ` · ${n} lift${n > 1 ? 's' : ''}` : '');

const still = () => typeof matchMedia === 'function' && matchMedia('(prefers-reduced-motion: reduce)').matches;

export function Mark({ glyph, tint = '', upto = null }) {
  const fill = `var(--${HUES.includes(tint) ? tint : 'accent'})`;
  const cells = upto === null ? glyph.cells : glyph.order.slice(0, upto);
  return (
    <svg viewBox={`-1 -1 ${glyph.cols + 2} ${glyph.rows + 2}`} shapeRendering="crispEdges" style={{ fill }} aria-hidden="true">
      {marks(cells, glyph.cols).map(([x, y]) => <rect key={`${x},${y}`} x={x} y={y} width="1" height="1" />)}
    </svg>
  );
}

function Card({ glyph, tint, fps, onPick }) {
  const [upto, setUpto] = useState(null);
  const timer = useRef(0);
  const rest = () => {
    clearInterval(timer.current);
    timer.current = 0;
  };
  const write = () => {
    if (!glyph.order.length || still()) return;
    rest();
    let k = 0;
    setUpto(0);
    timer.current = setInterval(() => {
      k++;
      setUpto(k);
      if (k >= glyph.order.length) rest();
    }, 1000 / fps);
  };
  const done = () => {
    rest();
    setUpto(null);
  };
  const pick = () => {
    write();
    onPick(glyph.char);
  };
  useEffect(() => rest, []);
  return (
    <button type="button" className="card" style={CARD} title={glyph.name} aria-label={`${glyph.name}, ${glyph.strokes} strokes of a floor of ${glyph.floor}${lifts(glyph.lifts)}: add it to the text`} onClick={pick} onPointerEnter={write} onPointerLeave={done} onFocus={write} onBlur={done}>
      <Mark glyph={glyph} tint={tint} upto={upto} />
      <div className="code">{glyph.label}</div>
      <div className="name">{glyph.strokes} of {glyph.floor}{lifts(glyph.lifts)}</div>
    </button>
  );
}

export function Gallery({ font, tint, onPick }) {
  const kinds = useMemo(() => gallery(font), [font]);
  const fps = useMemo(() => font.FPS(), [font]);
  const [kind, setKind] = useState(ALL);
  const shown = kinds.filter((one) => kind === ALL || one.id === kind).flatMap((one) => one.glyphs);
  return (
    <section aria-label="Glyphs">
      <h3>Glyphs</h3>
      <div className="filter" style={FILTER}>
        <Btn on={kind === ALL} onClick={() => setKind(ALL)}>All<span>{kinds.reduce((n, one) => n + one.glyphs.length, 0)}</span></Btn>
        {kinds.map((one) => (
          <Btn key={one.id} on={kind === one.id} onClick={() => setKind(one.id)}>{one.name}<span>{one.glyphs.length}</span></Btn>
        ))}
      </div>
      <div style={GRID}>
        {shown.map((glyph) => <Card key={glyph.char} glyph={glyph} tint={tint} fps={fps} onPick={onPick} />)}
      </div>
    </section>
  );
}
