import { useEffect, useMemo, useReducer, useRef, useState } from 'react';
import { ready, ink, fit } from '../lib/mrly.js';
import { mount, Page, Row, Pick, Slider, Text, Btn, Stats, Stat, Note } from '../lib/app.jsx';
import { Sketch } from '../lib/draw.jsx';
import { useQuery } from '../lib/query.js';
import { useSeeds, roll } from '../lib/select.jsx';
import { board, bars, line, axis, tag } from '../lib/chart.js';

const m = await ready();
const TOPS = [100, 1000, 10000, 100000, 1000000];
const FIRST = { n: '360', limit: 100, top: 10000, detect: 169 };
const STEPS = +(new URLSearchParams(location.search).get('steps') ?? 0);

const shuffle = (seed) => {
  const [n, limit, top, detect] = roll(seed, [[2, 1000000], [10, 400], [0, TOPS.length - 1], [3, 199]]);
  return { n: String(n), limit, top: TOPS[top], detect: detect | 1 };
};

const first = (seeds) => (seeds.get() ? shuffle(seeds.get()) : FIRST);

function App() {
  const s = useSeeds();
  const [look, set] = useQuery(first(s));
  const [boot] = useState(() => {
    try {
      const made = new m.Sieve(look.limit);
      let at = 0;
      for (let k = 0; k < STEPS; k++) at = made.step();
      return { sieve: made, at, error: null };
    } catch (error) {
      return { sieve: null, at: 0, error };
    }
  });
  const sieve = useRef(boot.sieve);
  const [current, setCurrent] = useState(boot.at);
  const [error, setError] = useState(boot.error);
  const [running, setRunning] = useState(false);
  const [tick, force] = useReducer((x) => x + 1, 0);
  const kept = useRef({ pile: null, data: null, trial: null });

  const view = useMemo(() => {
    try {
      const pile = JSON.parse(m.factor(look.n.trim()));
      const data = JSON.parse(m.prime_chart(look.top, 400));
      const trial = JSON.parse(m.carpet_witness(look.detect));
      kept.current = { pile, data, trial };
      return { ...kept.current, error: null };
    } catch (error) {
      return { ...kept.current, error };
    }
  }, [look.n, look.top, look.detect]);

  const reset = (limit) => {
    setRunning(false);
    setCurrent(0);
    try {
      sieve.current = new m.Sieve(limit);
      setError(null);
    } catch (error) {
      setError(error);
    }
    force();
  };

  const step = () => {
    if (!sieve.current || sieve.current.done()) return;
    setCurrent(sieve.current.step());
    if (sieve.current.done()) setRunning(false);
    force();
  };

  useEffect(() => {
    if (!running) return;
    step();
    const timer = setInterval(step, 600);
    return () => clearInterval(timer);
  }, [running]);

  const random = () => {
    const next = shuffle(s.next());
    set(next);
    reset(next.limit);
  };

  const sheet = (canvas) => {
    if (!sieve.current) return;
    const types = sieve.current.types(), limit = types.length - 1, mark = sieve.current.rank() + 1;
    const cols = limit > 100 ? 20 : 10, rows = Math.ceil(limit / cols);
    const cell = canvas.clientWidth / cols;
    const [ctx, w, h] = fit(canvas, Math.ceil(rows * cell));
    const mono = getComputedStyle(document.body).getPropertyValue('--mono');
    ctx.fillStyle = ink.deep;
    ctx.fillRect(0, 0, w, h);
    ctx.font = `${Math.min(13, cell * 0.42)}px ${mono}`;
    ctx.textAlign = 'center';
    ctx.textBaseline = 'middle';
    for (let n = 1; n <= limit; n++) {
      const t = types[n], x = ((n - 1) % cols) * cell, y = Math.floor((n - 1) / cols) * cell;
      const lit = n === current || t === mark;
      ctx.fillStyle = n === current ? ink.blue : t === mark ? ink.orange : t === 1 ? ink.gold : t ? ink.line : ink.panel;
      ctx.fillRect(x + 1, y + 1, cell - 2, cell - 2);
      if (cell >= 15) {
        ctx.fillStyle = lit || t === 1 ? ink.bg : ink.dim;
        ctx.fillText(n, x + cell / 2, y + cell / 2 + 1);
      }
    }
  };

  const stones = (canvas) => {
    const pile = view.pile;
    if (!pile) return;
    const rects = pile.rectangles, n = pile.n;
    const width = canvas.clientWidth, few = n <= 60;
    const stone = few ? (width - 80) / n : 0;
    const rise = few ? rects.reduce((sum, [a]) => sum + a * stone + 8, 0) : 240;
    const [ctx, w, h] = fit(canvas, Math.max(Math.ceil(rise), 60));
    ctx.fillStyle = ink.deep;
    ctx.fillRect(0, 0, w, h);
    const mono = getComputedStyle(document.body).getPropertyValue('--mono');
    ctx.font = `11px ${mono}`;
    if (few) {
      let y = 4;
      for (const [a, b] of rects) {
        ctx.fillStyle = rects.length === 1 ? ink.gold : ink.blue;
        for (let i = 0; i < a; i++) {
          for (let j = 0; j < b; j++) {
            ctx.beginPath();
            ctx.arc((j + 0.5) * stone, y + (i + 0.5) * stone, stone * 0.36, 0, Math.PI * 2);
            ctx.fill();
          }
        }
        ctx.fillStyle = ink.fg;
        ctx.textAlign = 'right';
        ctx.fillText(`${a} by ${b}`, w - 4, y + a * stone / 2 + 4);
        y += a * stone + 8;
      }
      return;
    }
    const span = Math.log(n);
    const px = (v) => 8 + (w - 16) * Math.log(v) / span;
    const py = (v) => h - 20 - (h - 36) * Math.log(v) / span;
    rects.forEach(([a, b], k) => {
      const x = px(b), y = Math.min(py(a), h - 24);
      ctx.fillStyle = rects.length === 1 ? ink.gold : ink.blue;
      ctx.globalAlpha = 0.18;
      ctx.fillRect(8, y, x - 8, h - 20 - y);
      ctx.globalAlpha = 1;
      ctx.strokeStyle = ctx.fillStyle;
      ctx.strokeRect(8.5, y + 0.5, x - 8, h - 20 - y);
      if (k === rects.length - 1 || k === 0) {
        ctx.fillStyle = ink.fg;
        ctx.textAlign = k ? 'right' : 'left';
        ctx.fillText(`${a} by ${b}`, k ? x - 4 : 12, y - 5);
      }
    });
    ctx.fillStyle = ink.dim;
    ctx.textAlign = 'left';
    ctx.fillText('1', 8, h - 6);
    ctx.textAlign = 'right';
    ctx.fillText(`${n} stones, sides on a log scale`, w - 8, h - 6);
  };

  const chart = (canvas) => {
    const data = view.data;
    if (!data) return;
    const b = board(canvas, 220);
    const top = data.x.at(-1), last = data.x.length - 1;
    const peak = Math.max(data.li.at(-1), data.pi.at(-1), data.ratio.at(-1));
    const trace = (column) => column.map((v, k) => [data.x[k] / top, Math.max(0, v) / peak]);
    line(b, trace(data.ratio), ink.pink, { dash: [4, 4] });
    line(b, trace(data.li), ink.blue);
    line(b, trace(data.pi), ink.gold, { width: 2 });
    axis(b, [[0, '0'], [1, String(top)]]);
    let x = tag(b, `pi(x) ${data.pi[last]}`, ink.gold);
    x = tag(b, `x / ln x ${data.ratio[last].toFixed(1)}`, ink.pink, 'left', x + 14);
    tag(b, `li(x) ${data.li[last].toFixed(1)}`, ink.blue, 'left', x + 14);
  };

  const witness = (canvas) => {
    const trial = view.trial;
    if (!trial) return;
    const b = board(canvas, 220);
    const count = trial.scales.length;
    bars(b, trial.row, { color: ink.gold });
    const every = Math.max(1, Math.round(count / 8));
    axis(b, trial.scales.map((scale, k) => [(k + 0.5) / count, scale]).filter((_, k) => k % every === 0));
    if (trial.prime) tag(b, `${trial.n}: every bar is exactly zero, prime`, ink.green);
    else tag(b, `${trial.n}: largest ${trial.max.toFixed(4)} at scale ${trial.at}`, ink.gold);
  };

  const done = sieve.current ? sieve.current.done() : false;
  const pile = view.pile, data = view.data, trial = view.trial;

  return (
    <Page crumb="primes" title="Numbers that will not split"
      sub="Take a handful of stones and try to lay them out as a rectangle. Twelve stones make three: one by twelve, two by six, three by four. Thirteen stones make one long row and nothing else, so thirteen is prime. Below, the sieve crosses out every number that splits, the stones show the rectangles of any number you type, the chart counts the primes up the number line, and the carpet stack finds the primes with no arithmetic at all."
      foot={<>The sieve is the one Eratosthenes ran: the next untouched number is prime, and its multiples from its square onward are struck. The stones are the divisors of a number paired below and above its square root; a single rectangle means prime. <code>pi(x)</code> counts the primes up to <code>x</code>; <code>x / ln x</code> and <code>li(x)</code> are the two classic guesses, the second summed by the Ramanujan series. The witness is exact: the carpet layer at scale <code>n</code> lights every cell of an <code>n</code> by <code>n</code> grid except those whose row and column are both odd, two layers are correlated on their common grid in whole numbers, and the correlation is exactly zero precisely when the two odd scales share no factor. So the row of an odd <code>n</code> is clear exactly when <code>n</code> is prime, and the smallest composite signal over the odd scales is the square of thirteen. The stack that sums these layers is the carpet preset of <a href="../moire">moire</a>; the same primes peak the novelty of the <a href="../farey">Farey stack</a>. Where pi comes out of that stack as a counted number is in <a href="https://github.com/mrlyprod/mrlyprod/blob/main/research/pi.md">the pi note</a>.</>}>
      <Row>
        <Slider label="sieve up to" value={look.limit} min={10} max={400} onChange={(v) => { set({ limit: v }); reset(v); }} />
        <Btn onClick={() => { setRunning(false); step(); }}>Step</Btn>
        <Btn onClick={() => { if (sieve.current?.done()) reset(look.limit); setRunning(!running); }}>{running ? 'Pause' : 'Play'}</Btn>
        <Btn onClick={() => reset(look.limit)}>Reset</Btn>
      </Row>
      <Row>
        <Text label="stones" value={look.n} onChange={(v) => set({ n: v })} />
        <Pick label="count to" value={look.top} options={TOPS.map((t) => [t, t])} onChange={(v) => set({ top: +v })} />
        <Slider label="scale on trial" value={look.detect} min={3} max={199} step={2} onChange={(v) => set({ detect: v })} />
        <Btn onClick={random}>Randomize</Btn>
      </Row>
      <div className="arena">
        <div className="panel">
          <h2>The sieve <span>{done ? `done, ${sieve.current.count()} primes in gold` : current ? `${current} strikes its multiples in orange` : 'blue is the prime in hand'}</span></h2>
          <Sketch draw={sheet} deps={[tick, current]} />
        </div>
        <div className="panel">
          <h2>The stones <span>{pile && pile.rectangles.slice(0, 8).map(([a, b]) => `${a}×${b}`).join(' ') + (pile.rectangles.length > 8 ? ' …' : '')}</span></h2>
          <Sketch draw={stones} deps={[pile]} />
        </div>
        <div className="panel">
          <h2>Counting primes <span>the staircase and its two guesses</span></h2>
          <Sketch draw={chart} deps={[data]} className="bars" />
        </div>
        <div className="panel">
          <h2>The witness <span>one bar per earlier odd scale</span></h2>
          <Sketch draw={witness} deps={[trial]} className="bars" />
        </div>
      </div>
      <Stats>
        <Stat label="prime in hand">{current || (done ? 'none left' : 'none yet')}</Stat>
        <Stat label="struck">{sieve.current?.struck()}</Stat>
        <Stat label="found">{sieve.current?.count()}</Stat>
        <Stat label="primes up to the top">{data?.pi.at(-1)}</Stat>
        <Stat label="x / ln x">{data?.ratio.at(-1).toFixed(1)}</Stat>
        <Stat label="li(x)">{data?.li.at(-1).toFixed(1)}</Stat>
        <Stat label="factors">{pile && (pile.factors.length ? pile.factors.map(([p, e]) => (e > 1 ? `${p}^${e}` : p)).join(' · ') : 'none')}</Stat>
        <Stat label="verdict">{pile && (pile.prime ? `${pile.n} is prime, one row only` : `${pile.n} makes ${pile.rectangles.length} rectangles`)}</Stat>
        <Stat label="largest correlation">{trial?.max.toFixed(7)}</Stat>
        <Stat label="at scale">{trial && (trial.at || 'nowhere')}</Stat>
      </Stats>
      <Note error={error ?? view.error} />
    </Page>
  );
}

mount(<App />);
