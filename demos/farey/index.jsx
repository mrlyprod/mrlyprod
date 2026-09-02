import { useMemo, useState } from 'react';
import { ready, ink } from '../lib/mrly.js';
import { mount, Page, Row, Slider, Check, Btn } from '../lib/app.jsx';
import { Sketch } from '../lib/draw.jsx';
import { useSeeds, roll } from '../lib/select.jsx';
import { board, axis } from '../lib/chart.js';

const m = await ready();

const shuffle = (seed) => roll(seed, [[2, 80]])[0];

function App() {
  const s = useSeeds();
  const [q, setQ] = useState(() => (s.get() ? shuffle(s.get()) : 24));
  const [marks, setMarks] = useState(true);
  const view = useMemo(() => ({ nodes: JSON.parse(m.farey(q)), stack: JSON.parse(m.farey_novelty(q)) }), [q]);

  const draw = (canvas) => {
    const b = board(canvas, 220, { pad: 24, top: 12, bottom: 30 });
    const { ctx } = b;
    axis(b, [[0, '0'], [1, '1']]);
    for (const [num, den, bright] of view.nodes) {
      const x = b.x(num / den);
      ctx.strokeStyle = `rgba(232, 236, 241, ${0.14 + 0.7 * bright / q})`;
      ctx.lineWidth = bright > q / 3 ? 1.5 : 0.7;
      ctx.beginPath();
      ctx.moveTo(x, b.floor);
      ctx.lineTo(x, b.y(bright / q));
      ctx.stroke();
    }
    if (marks) {
      ctx.fillStyle = ink.orange;
      for (const [num, den] of view.nodes) {
        if (view.stack.primes.includes(den)) ctx.fillRect(b.x(num / den) - 1, b.floor, 2, 7);
      }
    }
  };

  return (
    <Page crumb="farey" title="The stack lights the Farey fractions"
      sub="Scale n draws a line at every k/n. A reduced fraction a/b is drawn by every scale divisible by b, so its brightness is the floor of Q over b. Scale n lights phi(n) nodes never seen before, and phi(n) = n - 1 exactly when n is prime."
      foot={<>The nodes come from the Stern-Brocot walk of the Farey sequence and the totients from a sieve, both in Rust; the page only stacks bars. The primes are read off the totients as the scales of maximal novelty. What the lit nodes are, and why how evenly they spread is equivalent to the Riemann hypothesis, is in <a href="https://github.com/mrlyprod/mrlyprod/blob/main/research/farey.md">the Farey note</a>.</>}>
      <Row>
        <Slider label="Q" value={q} min={2} max={80} onChange={setQ} />
        <Check label="mark the primes" checked={marks} onChange={setMarks} />
        <Btn onClick={() => setQ(shuffle(s.next()))}>Randomize</Btn>
      </Row>
      <Sketch draw={draw} deps={[q, marks]} className="bars" />
      <pre>{`scales 1..${q}   lit nodes ${view.stack.lit}   1 + sum phi(n) = ${view.stack.novel}   match ${view.stack.match ? 'yes' : 'no'}\nprimes found as maximal-novelty scales: ${view.stack.primes.join(' ')}`}</pre>
    </Page>
  );
}

mount(<App />);
