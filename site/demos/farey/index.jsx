import { useMemo, useState } from 'react';
import { mount, Page, Row, Slider, Check, Btn } from '../../lib/app.jsx';
import { Sketch } from '../../lib/draw.jsx';
import { useSeeds, roll } from '../../lib/select.jsx';
import { bars, view } from './widget.jsx';

const shuffle = (seed) => roll(seed, [[2, 80]])[0];

function App() {
  const s = useSeeds();
  const [q, setQ] = useState(() => (s.get() ? shuffle(s.get()) : 24));
  const [marks, setMarks] = useState(true);
  const seen = useMemo(() => view(q), [q]);

  const controls = (
    <Row>
      <Slider label="Q" value={q} min={2} max={80} onChange={setQ} />
      <Check label="mark the primes" checked={marks} onChange={setMarks} />
      <Btn onClick={() => setQ(shuffle(s.next()))}>Randomize</Btn>
    </Row>
  );

  return (
    <Page crumb="farey" title="The stack lights the Farey fractions"
      sub="Scale n draws a line at every k/n. A reduced fraction a/b is drawn by every scale divisible by b, so its brightness is the floor of Q over b. Scale n lights phi(n) nodes never seen before, and phi(n) = n - 1 exactly when n is prime."
      controls={controls}
      foot={<>The nodes come from the Stern-Brocot walk of the Farey sequence and the totients from a sieve, both in Rust; the page only stacks bars. The primes are read off the totients as the scales of maximal novelty. What the lit nodes are, and why how evenly they spread is equivalent to the Riemann hypothesis, is in <a href="/research/farey/">the Farey note</a>; what a Farey sequence is to begin with is <a href="/wiki/farey-sequence/">the wiki page</a>.</>}>
      <Sketch draw={bars(seen, q, marks)} deps={[q, marks]} className="bars" role="img" aria-label="The Farey stack, one bar per fraction, taller where more scales draw it" />
      <pre>{`scales 1..${q}   lit nodes ${seen.stack.lit}   1 + sum phi(n) = ${seen.stack.novel}   match ${seen.stack.match ? 'yes' : 'no'}\nprimes found as maximal-novelty scales: ${seen.stack.primes.join(' ')}`}</pre>
    </Page>
  );
}

mount(<App />);
