import { useMemo } from 'react';
import { ready, ink } from '../../lib/mrly.js';
import { useQuery } from '../../lib/query.js';
import { mount, Page, Row, Slider, Pick, Stats, Stat } from '../../lib/app.jsx';
import { Sketch } from '../../lib/draw.jsx';
import { board, line, axis } from '../../lib/chart.js';

const m = await ready();

const FLOOR = 2;
const REACH = 2000;
const STOPS = 160;
const HEIGHT = 96;
const WIKI = 'https://en.wikipedia.org/wiki/';

const VIEWS = [['value', 'the partial'], ['error', 'the gap'], ['log', 'the gap, log scale']];

const whole = (value, low, high, fallback) =>
  (Number.isFinite(value) ? Math.min(high, Math.max(low, Math.round(value))) : fallback);

const CARDS = [
  {
    key: 'wallis',
    name: 'Wallis product',
    page: 'Wallis_product',
    form: 'W(n) = prod 4k² / (4k² - 1) → π/2',
    hue: ink.blue,
    target: 'π/2',
  },
  {
    key: 'leibniz',
    name: 'Leibniz series',
    page: 'Leibniz_formula_for_%CF%80',
    form: 'L(n) = 1 - 1/3 + 1/5 - 1/7 + ... → π/4',
    hue: ink.blue,
    target: 'π/4',
  },
  {
    key: 'basel',
    name: 'Basel problem',
    page: 'Basel_problem',
    form: 'B(n) = sum 1/k² → π²/6',
    hue: ink.blue,
    target: 'π²/6',
    note: <>The same constant counted out of the lattice is <a href="/research/pi/">the pi note</a>, drawn in <a href="../pi/">the pi demo</a>.</>,
  },
  {
    key: 'gamma',
    name: 'Euler-Mascheroni constant',
    page: 'Euler%27s_constant',
    form: 'H(n) - ln n → γ',
    hue: ink.blue,
    target: 'γ',
  },
  {
    key: 'e',
    name: "Euler's number",
    page: 'E_(mathematical_constant)',
    form: '(1 + 1/n)ⁿ → e',
    hue: ink.blue,
    target: 'e',
  },
  {
    key: 'primes',
    name: 'Prime counting function',
    page: 'Prime-counting_function',
    form: 'pi(n) / li(n) → 1',
    hue: ink.gold,
    note: <>The staircase against both guesses is <a href="../primes/">the primes demo</a>.</>,
  },
  {
    key: 'goldbach',
    name: "Goldbach's conjecture",
    page: 'Goldbach%27s_conjecture',
    form: 'g(2n) = #{p + q = 2n, p ≤ q prime}',
    hue: ink.gold,
    note: <>Open since 1742: nobody has proved g(2n) is never zero, so the least count below is evidence and not a proof.</>,
  },
  {
    key: 'mertens',
    name: 'Mertens function',
    page: 'Mertens_function',
    form: 'M(n) = sum μ(k), read against √n',
    hue: ink.orange,
    note: <>Where the swings of M(n)/√n meet the zeta zeros is <a href="/research/farey/">the Farey stack note</a>.</>,
  },
];

const READ = {
  wallis: (c) => [['partial', c.value.toFixed(12)], ['π/2', c.limit.toFixed(12)], ['off by', c.error.toExponential(2)]],
  leibniz: (c) => [['partial', c.value.toFixed(12)], ['π/4', c.limit.toFixed(12)], ['off by', c.error.toExponential(2)]],
  basel: (c) => [['partial', c.value.toFixed(12)], ['π²/6', c.limit.toFixed(12)], ['off by', c.error.toExponential(2)]],
  gamma: (c) => [['partial', c.value.toFixed(12)], ['γ', c.limit.toFixed(12)], ['off by', c.error.toExponential(2)]],
  e: (c) => [['partial', c.value.toFixed(12)], ['e', c.limit.toFixed(12)], ['off by', c.error.toExponential(2)]],
  primes: (c) => [['pi(n)', c.value], ['li(n)', c.li.toFixed(6)], ['n / ln n', c.ratio.toFixed(6)], ['pi(n) / li(n)', c.gauge.toFixed(9)]],
  goldbach: (c) => [['2n', c.even], ['g(2n)', c.value], ['least g up to 2n', c.floor], ['1 / g(2n)', c.rel.toFixed(9)]],
  mertens: (c) => [['M(n)', c.value], ['√n', c.root.toFixed(6)], ['|M(n)| / √n', c.rel.toFixed(9)]],
};

const height = (view, gauge, value) =>
  (view === 'value' ? value : view === 'error' ? gauge : Math.log10(Math.max(gauge, 1e-12)));

const sketch = (walk, view, rule, hue) => (canvas) => {
  const b = board(canvas, HEIGHT, { pad: 12, top: 10, bottom: 12 });
  const stops = walk.length / 3;
  const xs = [];
  const ys = [];
  for (let k = 0; k < stops; k += 1) {
    xs.push(walk[3 * k]);
    ys.push(height(view, walk[3 * k + 2], walk[3 * k + 1]));
  }
  const level = view === 'value' ? rule : null;
  let lo = Math.min(...ys);
  let hi = Math.max(...ys);
  if (level !== null) {
    lo = Math.min(lo, level);
    hi = Math.max(hi, level);
  }
  const room = hi - lo || 1;
  const up = (v) => 0.08 + 0.84 * (v - lo) / room;
  const span = xs[stops - 1] - xs[0] || 1;
  const across = (x) => (x - xs[0]) / span;
  axis(b, []);
  if (level !== null) line(b, [[0, up(level)], [1, up(level)]], ink.dim, { width: 1, dash: [4, 4] });
  line(b, xs.map((x, k) => [across(x), up(ys[k])]), hue, { width: 1.4 });
};

function Card({ card, read, view }) {
  const { key, name, form, hue, note } = card;
  const at = read.cards[key];
  const walk = useMemo(() => m.formulas_walk(key, read.n, STOPS), [key, read.n]);
  const rule = at.limit === undefined ? null : at.limit;
  return (
    <div className="panel">
      <h2><a href={WIKI + card.page}>{name}</a> <span>{form}</span></h2>
      <Sketch draw={sketch(walk, view, rule, hue)} deps={[walk, view]} role="img"
        aria-label={`${name} walked from ${FLOOR} to ${read.n}`} />
      <Stats>
        {READ[key](at).map(([label, text]) => <Stat key={label} label={label}>{text}</Stat>)}
      </Stats>
      {note && <p className="sub">{note}</p>}
    </div>
  );
}

function App() {
  const [q, set] = useQuery({ n: 400, view: 'value' });
  const n = whole(q.n, FLOOR, REACH, 400);
  const view = VIEWS.some(([id]) => id === q.view) ? q.view : 'value';
  const read = useMemo(() => JSON.parse(m.formulas_read(n)), [n]);

  const controls = (
    <Row>
      <Slider label="depth n" value={n} min={FLOOR} max={REACH} onChange={(v) => set({ n: v })} />
      <Pick label="every sparkline draws" value={view} onChange={(v) => set({ view: v })} options={VIEWS} />
    </Row>
  );

  return (
    <Page crumb="formulas" title="The formulas"
      sub="Eight elementary systems on one dial. Turn n and watch five products and sums close on pi, e and gamma, the prime count close on the logarithmic integral, Goldbach's partition count refuse to reach zero, and Mertens's sum refuse to settle at all."
      controls={controls}
      foot={<>Every partial, every target and every gap on this page is computed in the crates and handed over as it stands: pi, e and gamma are the crate's own constants, li is the offset logarithmic integral by its Ramanujan series, g(2n) counts unordered prime pairs by sieve, and M(n) sums the Mobius values. Two of the eight are not theorems. Goldbach's is a conjecture, so the card reports the smallest partition count seen up to 2n and claims nothing beyond it. Mertens's M(n) is drawn against the square root of n with no claim either; the bound that would follow is the Riemann hypothesis. The rest are classical, and the sparklines say how fast each one pays.</>}>
      <Stats>
        <Stat label="depth n">{n}</Stat>
        <Stat label="π">{read.constants.pi.toFixed(12)}</Stat>
        <Stat label="e">{read.constants.e.toFixed(12)}</Stat>
        <Stat label="γ">{read.constants.gamma.toFixed(12)}</Stat>
      </Stats>
      <div className="arena">
        {CARDS.map((card) => <Card key={card.key} card={card} read={read} view={view} />)}
      </div>
    </Page>
  );
}

mount(<App />);
