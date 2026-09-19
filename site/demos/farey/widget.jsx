import { useMemo, useState } from 'react';
import { ready, ink, rgb } from '../../lib/mrly.js';
import { Row, Slider, Check } from '../../lib/app.jsx';
import { Sketch } from '../../lib/draw.jsx';
import { board, axis } from '../../lib/chart.js';
import { embed } from '../../lib/widget.jsx';

const m = await ready();

export const view = (q) => ({ nodes: JSON.parse(m.farey(q)), stack: JSON.parse(m.farey_novelty(q)) });

export const bars = (seen, q, marks) => (canvas) => {
  const b = board(canvas, 220, { pad: 24, top: 12, bottom: 30 });
  const { ctx } = b;
  const pale = rgb(ink.fg).join(', ');
  axis(b, [[0, '0'], [1, '1']]);
  for (const [num, den, bright] of seen.nodes) {
    const x = b.x(num / den);
    ctx.strokeStyle = `rgba(${pale}, ${0.14 + 0.7 * bright / q})`;
    ctx.lineWidth = bright > q / 3 ? 1.5 : 0.7;
    ctx.beginPath();
    ctx.moveTo(x, b.floor);
    ctx.lineTo(x, b.y(bright / q));
    ctx.stroke();
  }
  if (marks) {
    ctx.fillStyle = ink.orange;
    for (const [num, den] of seen.nodes) {
      if (seen.stack.primes.includes(den)) ctx.fillRect(b.x(num / den) - 1, b.floor, 2, 7);
    }
  }
};

export function stack() {
  const [q, setQ] = useState(24);
  const [marks, setMarks] = useState(true);
  const seen = useMemo(() => view(q), [q]);
  return (
    <>
      <Sketch draw={bars(seen, q, marks)} deps={[q, marks]} className="bars" role="img" aria-label="The Farey stack, one bar per fraction, taller where more scales draw it" />
      <Row>
        <Slider label="Q" value={q} min={2} max={80} onChange={setQ} />
        <Check label="mark the primes" checked={marks} onChange={setMarks} />
      </Row>
      <p className="sub">{`scales 1 to ${q} light ${seen.stack.lit} fractions; the primes are ${seen.stack.primes.join(', ')}`}</p>
    </>
  );
}

embed('farey', { stack });
