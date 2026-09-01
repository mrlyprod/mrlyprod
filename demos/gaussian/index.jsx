import { useEffect, useMemo, useRef, useState } from 'react';
import { ready, ink, blit } from '../lib/mrly.js';
import { mount, Page, Row, Pick, Slider, Check, Btn, Stats, Stat, Note } from '../lib/app.jsx';
import { Sketch } from '../lib/draw.jsx';
import { useQuery, stamp } from '../lib/query.js';
import { useSeeds, roll } from '../lib/select.jsx';
import { board, bars, axis, tag } from '../lib/chart.js';

const m = await ready();
const SIZE = 768;
const NORMS = 60;
const RINGS = [['gaussian', 'gaussian, a + bi'], ['eisenstein', 'eisenstein, a + bω']];
const COLOURS = [['class', 'by class'], ['norm', 'by norm'], ['plain', 'plain']];
const FIRST = { ring: 'gaussian', radius: 40, colour: 'class', units: true, composites: true };
const CLICK = new URLSearchParams(location.search).get('click');

const shuffle = (seed) => {
  const [ring, radius, colour] = roll(seed, [[0, RINGS.length - 1], [20, 120], [0, COLOURS.length - 1]]);
  return { ring: RINGS[ring][0], radius, colour: COLOURS[colour][0] };
};

const first = (seeds) => (seeds.get() ? { ...FIRST, ...shuffle(seeds.get()) } : FIRST);

function App() {
  const s = useSeeds();
  const [look, save] = useQuery(first(s));
  const [picked, setPicked] = useState(null);
  const [error, setError] = useState(null);
  const kept = useRef({ pixels: null, census: null, weights: null, fates: null, peak: null });

  const view = useMemo(() => {
    try {
      const pixels = m.ring_pixels(look.ring, look.radius, look.colour, look.composites, SIZE);
      const census = JSON.parse(m.ring_census(look.ring, look.radius));
      const weights = m.ring_weights(look.ring, NORMS);
      const fates = m.ring_fates(look.ring, NORMS);
      const peak = m.ring_peak(look.ring, NORMS);
      kept.current = { pixels, census, weights, fates, peak };
      return { ...kept.current, error: null };
    } catch (error) {
      return { ...kept.current, error };
    }
  }, [look.ring, look.radius, look.colour, look.composites]);

  const set = (patch) => {
    save(patch);
    if ('units' in patch) stamp({ units: patch.units ? null : 0 });
    if ('composites' in patch) stamp({ composites: patch.composites ? null : 0 });
    setPicked(null);
    setError(null);
  };

  const name = (a, b) => {
    const unit = look.ring === 'gaussian' ? 'i' : 'ω';
    const size = Math.abs(b) === 1 ? '' : Math.abs(b);
    if (b === 0) return `${a}`;
    if (a === 0) return `${b < 0 ? '-' : ''}${size}${unit}`;
    return `${a} ${b < 0 ? '-' : '+'} ${size}${unit}`;
  };

  const verdict = (p) => {
    const norm = `norm ${p.norm}`;
    const shown = p.factors.map(([q, e]) => (e > 1 ? `${q}^${e}` : q)).join(' · ');
    const [ca, cb] = p.conjugate;
    if (p.class === 'split') return `prime: ${p.norm} splits as (${name(p.a, p.b)})(${name(ca, cb)})`;
    if (p.class === 'inert') return `prime: ${p.factors[0][0]} stays prime in the plane`;
    if (p.class === 'ramified') return `prime: ${p.norm} ramifies, a unit times a square`;
    if (p.class === 'unit') return 'a unit, norm 1';
    if (p.class === 'zero') return 'the origin';
    return `composite, ${norm} = ${shown}`;
  };

  const sheet = (canvas) => {
    if (!view.pixels) return;
    blit(canvas, view.pixels);
    if (!picked) return;
    const ctx = canvas.getContext('2d');
    const ring = (x, y, color, width, dash = []) => {
      ctx.strokeStyle = color;
      ctx.lineWidth = width;
      ctx.setLineDash(dash);
      ctx.beginPath();
      ctx.arc(x, y, picked.span / 2 + 3, 0, Math.PI * 2);
      ctx.stroke();
      ctx.setLineDash([]);
    };
    if (look.units && picked.norm > 1) {
      for (const [, , x, y] of picked.associates.slice(1)) ring(x, y, ink.fg, 1.5);
      const [, , x, y] = picked.conjugate;
      ring(x, y, ink.pink, 1.5, [4, 3]);
    }
    ring(picked.px, picked.py, ink.fg, 3);
  };

  const chart = (canvas) => {
    if (!view.weights) return;
    const b = board(canvas, 220);
    const values = Array.from(view.weights).slice(1);
    const colour = (k) => [ink.dim, ink.blue, ink.orange, ink.pink][view.fates[k + 1]];
    bars(b, values, { color: colour });
    values.forEach((v, k) => {
      if (v || view.fates[k + 1] !== 2) return;
      b.ctx.fillStyle = ink.orange;
      b.ctx.fillRect(b.x(k / values.length) + 1, b.floor - 3, Math.max(1, b.wide / values.length - 2), 3);
    });
    axis(b, values.map((_, k) => [(k + 0.5) / values.length, k + 1]).filter(([, n]) => n % 10 === 0));
    tag(b, `peak r(${view.peak[0]}) = ${view.peak[1]}`, ink.fg);
    tag(b, 'blue split · orange inert · pink ramified', ink.dim, 'right');
  };

  const hit = (x, y) => {
    if (!view.pixels) return;
    setError(null);
    try {
      setPicked(JSON.parse(m.ring_at(look.ring, look.radius, x, y, SIZE)));
    } catch (error) {
      setError(error);
    }
  };

  useEffect(() => {
    if (CLICK) hit(...CLICK.split(',').map(Number));
  }, []);

  const census = view.census;

  return (
    <Page crumb="gaussian" title="Primes in the plane"
      sub="Give the whole numbers a square root of minus one and the points a + bi have primes of their own; paint them and a four-armed snowflake appears. On the hexagonal numbers a + bω it grows six arms. The colour says what became of an ordinary prime when it entered the plane: split into a point and its mirror image, stayed prime on an axis, or ramified into a square. Click a point for its norm, its class and its unit rotations."
      foot={<>The norm of <code>a + bi</code> is <code>a² + b²</code>, the norm of <code>a + bω</code> is <code>a² - ab + b²</code>, and a norm multiplies like a length squared. A point is prime when its norm is an ordinary prime, or when it is a unit times an ordinary prime that stays prime in the plane: <code>3 mod 4</code> on the square lattice, <code>2 mod 3</code> on the hexagonal one. An ordinary prime that is a norm has split into a point and its conjugate, except the one prime that ramifies, 2 or 3, whose point is a unit times a square. The units, 4 or 6 of them, turn every prime into its associates, and with the mirror give the picture its symmetry. The bars count the points of each norm: on the square lattice <code>r(n) = 4 (d₁ - d₃)</code>, silent exactly where an inert prime divides <code>n</code> to an odd power, summing to <code>4 ζ(s) L(s, χ₋₄)</code>; on the hexagonal lattice the weights sum to <code>6 ζ(s) L(s, χ₋₃)</code>. These are the zeta functions of the two rings, whose values at 2 are the coprime densities of the two lattices and whose weights ring the profiles of the <a href="../spin">spin</a> page. Every point is classified and painted in Rust, the norms sieved once per window.</>}>
      <Row>
        <Pick label="ring" value={look.ring} options={RINGS} onChange={(v) => set({ ring: v })} />
        <Slider label="radius" value={look.radius} min={5} max={200} onChange={(v) => set({ radius: v })} />
        <Pick label="colour" value={look.colour} options={COLOURS} onChange={(v) => set({ colour: v })} />
        <Check label="units of a click" checked={look.units} onChange={(v) => set({ units: v })} />
        <Check label="faint composites" checked={look.composites} onChange={(v) => set({ composites: v })} />
        <Btn onClick={() => set(shuffle(s.next()))}>Randomize</Btn>
      </Row>
      <div className="arena">
        <div className="panel">
          <h2>The window <span>{census && `${look.ring}, reach ${look.radius}, norms to ${census.top}, ${census.units} units`}</span></h2>
          <Sketch draw={sheet} deps={[view, picked, look.units]} onClick={(event) => {
            const box = event.currentTarget.getBoundingClientRect();
            hit((event.clientX - box.left) * SIZE / box.width, (event.clientY - box.top) * SIZE / box.height);
          }} />
        </div>
        <div className="panel">
          <h2>The ring weights <span>{census && `norms 1 to ${NORMS}`}</span></h2>
          <Sketch draw={chart} deps={[view]} className="bars" />
          <Stats>
            <Stat label="points">{census?.points}</Stat>
            <Stat label="primes">{census?.primes}</Stat>
            <Stat label="split">{census?.split}</Stat>
            <Stat label="inert">{census?.inert}</Stat>
            <Stat label="ramified">{census?.ramified}</Stat>
            <Stat label="density">{census && `${(census.density * 100).toFixed(2)}%`}</Stat>
            <Stat label="symmetry">{census && `${census.symmetry}-fold`}</Stat>
          </Stats>
          <Stats>
            <Stat label="clicked">{picked && `${name(picked.a, picked.b)} at ${picked.a}, ${picked.b}`}</Stat>
            <Stat label="norm">{picked?.norm}</Stat>
            <Stat label="class">{picked?.class}</Stat>
            <Stat label="verdict">{picked && verdict(picked)}</Stat>
          </Stats>
        </div>
      </div>
      <Note error={error ?? view.error} />
    </Page>
  );
}

mount(<App />);
