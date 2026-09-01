import { useEffect, useMemo, useRef, useState } from 'react';
import { ready, ink, blit } from '../lib/mrly.js';
import { mount, Page, Row, Pick, Slider, Check, Btn, Stats, Stat, Note } from '../lib/app.jsx';
import { Sketch } from '../lib/draw.jsx';
import { useQuery, stamp } from '../lib/query.js';
import { useSeeds, roll } from '../lib/select.jsx';

const m = await ready();
const SIZE = 768;
const LABELS = 1000;
const LATTICES = ['square', 'hex'];
const MARKS = [['prime', 'primes'], ['twin', 'twin primes'], ['squarefree', 'squarefree'], ['mobius', 'Mobius sign']];
const FIRST = { lattice: 'square', side: 201, mark: 'prime', a: 4, b: -2, c: 41, faint: true };
const CLICK = new URLSearchParams(location.search).get('click');

const shuffle = (seed) => {
  const [lattice, side, a, b, c] = roll(seed, [[0, LATTICES.length - 1], [21, 401], [1, 4], [-20, 20], [2, 97]]);
  return { lattice: LATTICES[lattice], side: side | 1, a, b, c: m.prime_from(c) };
};

const first = (seeds) => (seeds.get() ? { ...FIRST, ...shuffle(seeds.get()) } : FIRST);

function Num({ label, value, min, max, onChange }) {
  const [text, setText] = useState(String(value));
  useEffect(() => {
    if (+text !== value) setText(String(value));
  }, [value]);
  return (
    <label>{label} <input type="number" min={min} max={max} value={text} onChange={(e) => {
      setText(e.target.value);
      if (e.target.value !== '' && e.target.value !== '-') onChange(+e.target.value);
    }} /></label>
  );
}

function App() {
  const s = useSeeds();
  const [look, save] = useQuery(first(s));
  const [picked, setPicked] = useState(null);
  const [error, setError] = useState(null);
  const kept = useRef({ pixels: null, poly: null, centres: null });

  const view = useMemo(() => {
    try {
      const pixels = m.spiral_pixels(look.lattice, look.side, look.a, look.b, look.c, look.mark, look.faint, SIZE);
      const poly = JSON.parse(m.spiral_polynomial(look.lattice, look.side, look.a, look.b, look.c));
      const centres = poly.top <= LABELS ? m.spiral_centers(look.lattice, look.side, SIZE) : null;
      kept.current = { pixels, poly, centres };
      return { ...kept.current, error: null };
    } catch (error) {
      return { ...kept.current, error };
    }
  }, [look.lattice, look.side, look.mark, look.a, look.b, look.c, look.faint]);

  const set = (patch) => {
    save(patch);
    if ('faint' in patch) stamp({ faint: patch.faint ? null : 0 });
    setPicked(null);
    setError(null);
  };

  const draw = (canvas) => {
    const { pixels, centres } = view;
    if (!pixels) return;
    blit(canvas, pixels);
    const ctx = canvas.getContext('2d');
    if (centres) {
      const mono = getComputedStyle(document.body).getPropertyValue('--mono');
      const px = Math.min(14, SIZE / look.side * 0.42);
      ctx.font = `${px}px ${mono}`;
      ctx.textAlign = 'center';
      ctx.textBaseline = 'middle';
      ctx.fillStyle = ink.bg;
      for (let n = 1; 2 * n <= centres.length; n++) {
        const [x, y] = [centres[2 * n - 2], centres[2 * n - 1]];
        const light = pixels.rgba[(Math.floor(y) * SIZE + Math.floor(x)) * 4 + 1] > 100;
        ctx.fillStyle = light ? ink.bg : ink.dim;
        ctx.fillText(n, x, y + 1);
      }
    }
    if (picked) {
      ctx.strokeStyle = ink.fg;
      ctx.lineWidth = 2;
      ctx.beginPath();
      ctx.arc(picked.px, picked.py, picked.span / 2 + 3, 0, Math.PI * 2);
      ctx.stroke();
    }
  };

  const hit = (x, y) => {
    if (!view.pixels) return;
    setError(null);
    try {
      setPicked(JSON.parse(m.spiral_at(look.lattice, look.side, x, y, SIZE)));
    } catch (error) {
      setError(error);
    }
  };

  useEffect(() => {
    if (CLICK) hit(...CLICK.split(',').map(Number));
  }, []);

  const poly = view.poly;
  const shown = poly ? poly.values.slice(0, 8).join(' ') + (poly.values.length > 8 ? ' …' : '') : '';
  const legend = poly ? `${look.a} k² ${look.b < 0 ? '-' : '+'} ${Math.abs(look.b)} k ${look.c < 0 ? '-' : '+'} ${Math.abs(look.c)}: ${shown}` : '';

  return (
    <Page crumb="ulam" title="The Ulam spiral"
      sub="Write 1 in the middle, then 2, 3, 4 and so on in a spiral, and paint the primes gold. Nobody ordered them into lines, yet diagonals appear: every straight line through the spiral reads a quadratic a k² + b k + c, and some quadratics are rich in primes. Click a cell for its number and its factors, pick a quadratic to light it up, and wind the same numbers on hexagons to watch the lines bend into six directions."
      foot={<>On the square lattice ring <code>k</code> holds <code>8k</code> cells and ends at the odd square <code>(2k + 1)²</code> on the diagonal below right, so a straight line through the spiral picks one number per ring and reads a quadratic in <code>k</code>: a diagonal has <code>a = 4</code>, a line through the centre <code>a = 1</code> or <code>2</code>. Euler's <code>m² - m + 41</code> is prime for <code>m</code> from 0 to 40; at <code>m = 2k</code> it is <code>4k² - 2k + 41</code>, the line lit at the start, prime for its first 21 values and then at 1763 = 41 · 43 it breaks. On the hexagonal lattice ring <code>r</code> holds <code>6r</code> cells and ends at the centered hexagonal number <code>3r² + 3r + 1</code>, so its straight lines read quadratics with <code>a = 3</code>. Every cell is sieved and painted in Rust: gold for the mark, orange where the quadratic lands on a prime, blue where it lands on a composite, pink for a Mobius value of minus one. The same primes are sieved, counted and found by the carpet stack on the <a href="../primes">primes</a> page.</>}>
      <Row>
        <Pick label="lattice" value={look.lattice} options={LATTICES} onChange={(v) => set({ lattice: v })} />
        <Slider label="side" value={look.side} min={21} max={401} step={2} onChange={(v) => set({ side: v })} />
        <Pick label="mark" value={look.mark} options={MARKS} onChange={(v) => set({ mark: v })} />
        <Check label="faint composites" checked={look.faint} onChange={(v) => set({ faint: v })} />
      </Row>
      <Row>
        <Num label="a" value={look.a} min={1} max={4} onChange={(v) => set({ a: v })} />
        <Num label="b" value={look.b} min={-20} max={20} onChange={(v) => set({ b: v })} />
        <Num label="c" value={look.c} min={2} max={97} onChange={(v) => set({ c: v })} />
        <Btn onClick={() => set(shuffle(s.next()))}>Randomize</Btn>
      </Row>
      <div className="arena">
        <div className="panel">
          <h2>The sheet <span>{legend}</span></h2>
          <Sketch draw={draw} deps={[view, picked]} onClick={(event) => {
            const box = event.currentTarget.getBoundingClientRect();
            hit((event.clientX - box.left) * SIZE / box.width, (event.clientY - box.top) * SIZE / box.height);
          }} />
        </div>
      </div>
      <Stats>
        <Stat label="numbers">{poly?.top}</Stat>
        <Stat label="primes">{poly?.primes}</Stat>
        <Stat label="density">{poly && `${(poly.density * 100).toFixed(2)}%`}</Stat>
        <Stat label="hits on the quadratic">{poly && `${poly.hits} of ${poly.count}`}</Stat>
        <Stat label="share">{poly && `${(poly.share * 100).toFixed(1)}%`}</Stat>
        <Stat label="opening streak">{poly && (poly.count === 0 ? 'off the sheet' : poly.streak === poly.count ? `all ${poly.count} prime` : `${poly.streak} primes, then ${poly.values[poly.streak]}`)}</Stat>
        <Stat label="clicked">{picked && `${picked.n} on ring ${picked.ring} at ${picked.x}, ${picked.y}`}</Stat>
        <Stat label="verdict">{picked && (picked.n === 1 ? 'one, neither prime nor composite' : picked.prime ? 'prime' : 'composite')}</Stat>
        <Stat label="factors">{picked && (picked.factors.length ? picked.factors.map(([p, e]) => (e > 1 ? `${p}^${e}` : p)).join(' · ') : 'none')}</Stat>
      </Stats>
      <Note error={error ?? view.error} />
    </Page>
  );
}

mount(<App />);
