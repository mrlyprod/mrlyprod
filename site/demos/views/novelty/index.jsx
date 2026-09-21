import { useEffect, useMemo, useState } from 'react';
import { ready, ink } from '../../../lib/mrly.js';
import { useQuery } from '../../../lib/query.js';
import { mount, Page, Row, Slider, Pick, Stats, Stat, Note } from '../../../lib/app.jsx';
import { Sketch } from '../../../lib/draw.jsx';
import { board, line, bars, axis, tag } from '../../../lib/chart.js';
import { meterChart, LOW, HIGH, PER_OCTAVE, ZEROS, CURVE } from './widget.jsx';

const m = await ready();
const FIRST = { k: 1, window: 'smooth', j: HIGH };
const FLOOR = 1e-8;

const plural = (n, word) => `${n} ${word}${n === 1 ? '' : 's'}`;

function caption(pick, meter, miss, count) {
  if (!meter) return `Sieving the totients to ${(2 ** (HIGH + 1)).toLocaleString('en')} and finding the first ${ZEROS} zeros.`;
  const k = pick.k;
  if (pick.window === 'sharp') {
    const tail = k ? `the ${plural(k, 'zero')} you added, drawn to this scale, shrink by the root of y and vanish under it` : 'the waves of the zeros would sit a factor of the root of y below it';
    return `Cut the window sharply and the error divided by y is a cloud of prime jumps that no wave fits: ${tail}.`;
  }
  if (k === 0) return `The smoothed error at ${count} heights, each divided by y^(3/2): a signal of unit size waiting for its waves.`;
  if (k === 1) return `The first zero alone, at height ${meter.gammas()[0].toFixed(2)}, is one cosine in log y and already hugs the dots, missing them by ${miss.toFixed(2)} of their peak.`;
  if (k < ZEROS) return `${k} zeros sum to a wave that misses the dots by ${miss.toExponential(1)} of their peak.`;
  return `${k} zeros fit every dot to ${miss.toExponential(1)} of the peak: the error is the zeros' waves and nothing else.`;
}

function App() {
  const [pick, set] = useQuery(FIRST);
  const [meter, setMeter] = useState(null);
  const [error, setError] = useState(null);
  useEffect(() => {
    try {
      setMeter(new m.Novelty(HIGH, PER_OCTAVE, ZEROS));
    } catch (fault) {
      setError(fault);
    }
  }, []);
  const sharp = pick.window === 'sharp';
  const heights = useMemo(() => (meter ? meter.heights() : new Float64Array()), [meter]);
  const dots = useMemo(() => (meter ? meter.dots(sharp) : new Float64Array()), [meter, sharp]);
  const gammas = useMemo(() => (meter ? meter.gammas() : new Float64Array()), [meter]);
  const amplitudes = useMemo(() => (meter ? meter.amplitudes() : new Float64Array()), [meter]);
  const curve = useMemo(() => Float64Array.from({ length: CURVE + 1 }, (_, i) => LOW + (pick.j - LOW) * i / CURVE), [pick.j]);
  const wave = useMemo(() => (meter ? meter.wave(pick.k, sharp, curve) : new Float64Array()), [meter, pick.k, sharp, curve]);
  const miss = useMemo(() => (meter ? meter.miss(pick.k) : 1), [meter, pick.k]);
  const count = useMemo(() => heights.filter((j) => j <= pick.j).length, [heights, pick.j]);

  const chart = meterChart({ heights, dots, curve, wave, j: pick.j, k: pick.k, sharp, height: 320 });

  const ladder = (canvas) => {
    const b = board(canvas, 150, { top: 24, bottom: 20 });
    if (!meter) return;
    const top = Math.log(amplitudes[0] * 1.5);
    const bottom = Math.log(FLOOR);
    const values = Array.from(amplitudes, (a) => Math.max(0, (Math.log(Math.max(a, FLOOR)) - bottom) / (top - bottom)));
    bars(b, values, { peak: 1, color: (i) => (i < pick.k ? ink.orange : ink.line), inset: 0.5 });
    axis(b, [[0, 'zero 1'], [1, `zero ${ZEROS}, height ${gammas[ZEROS - 1].toFixed(0)}`]]);
    tag(b, `|c| from ${amplitudes[0].toFixed(2)} down to ${amplitudes[ZEROS - 1].toExponential(0)}, log scale`, ink.dim);
  };

  const controls = (
    <Row>
      <Slider label="zeros in the wave" value={pick.k} min={0} max={ZEROS} onChange={(v) => set({ k: v })} />
      <Pick label="window" value={pick.window} options={[['smooth', 'smooth bump'], ['sharp', 'sharp cutoff']]} onChange={(v) => set({ window: v })} />
      <Slider label="y down to 2^-j, j" value={pick.j} min={LOW + 1} max={HIGH} onChange={(v) => set({ j: v })} />
    </Row>
  );

  const last = meter && pick.k ? gammas[pick.k - 1] : null;

  return (
    <Page crumb="novelty" title="The stack hears the zeros"
      sub={<>Stack a grid of <code>n</code> cells on the unit interval for every scale <code>n</code>: scale <code>n</code> lights <code>phi(n)</code> nodes no smaller scale drew. Count that novelty through a smooth window around the scale <code>1/y</code>, take away the main term, and what is left is a sum of waves, one per zero of zeta, each with the zero's height as its frequency in <code>log y</code>. Add the zeros one at a time and watch the wave settle on the dots; cut the window sharply and the primes shout over it.</>}
      foot={<>The window is the bump <code>f(u) = exp(4 - 1/((u - 1)(2 - u)))</code> on <code>[1, 2]</code>, and the meter is <code>E_f(y) = y^2 sum_n phi(n) f(n y) - (6/pi^2) F(2)</code>, <code>F</code> the Mellin transform of <code>f</code>, the totients sieved to <code>2^{HIGH + 1}</code> so every window fits. Under the Riemann hypothesis <code>E_f(y) = sum_rho F(rho) zeta(rho - 1)/zeta'(rho) y^(2 - rho)</code> up to a smaller remainder, so <code>E_f(y)/y^(3/2)</code> is <code>2 Re sum c_rho y^(-i gamma)</code>, a cosine in <code>log y</code> per zero at the zero's height. The curve is that sum over the first <code>K</code> zeros with nothing fitted: the zeros come from the critical line walked on <a href="../zeta/">the zeta page</a>, <code>zeta(rho - 1)</code> and <code>zeta'(rho)</code> from the same Euler-Maclaurin sum off the line, <code>F(rho)</code> from a 4096-node rule. The sharp window is the indicator of <code>[1, 2]</code>; its error divided by <code>y</code> stays of unit size because the totient sum jumps by about <code>0.39 p</code> at every prime <code>p</code>, louder than the waves, which live at <code>y^(3/2)</code>. The exponent of the smooth meter, <code>3/2</code> in <code>y</code> for every smooth window, is equivalent to the Riemann hypothesis; the paper <a href="/papers/novelty-meter/">The stack hears the zeros</a> assembles that equivalence and <a href="/research/stack/">the stack page</a> reads the meter to <code>2^-23.5</code>. Every number here is one crate call; the page only draws.</>}
      controls={controls}>
      <div className="arena">
        <div className="panel">
          <h2>The meter <span>{sharp ? 'the sharp window, E(y) / y' : 'the smooth window, E(y) / y^(3/2)'}</span></h2>
          <Sketch draw={chart} deps={[meter, dots, wave, pick.j, pick.k, sharp]} className="bars" role="img" aria-label="The novelty error at every height as dots, with the wave of the first zeros as a curve" />
          <p className="sub">{caption(pick, meter, miss, count)}</p>
        </div>
      </div>
      <div className="arena">
        <div className="panel">
          <h2>The waves <span>one bar per zero, its amplitude on a log scale, the first {pick.k} in the sum</span></h2>
          <Sketch draw={ladder} deps={[meter, pick.k]} className="bars" role="img" aria-label="The amplitude of every zero's wave, the first so many lit" />
        </div>
      </div>
      <Stats>
        <Stat label="readings">{count}</Stat>
        <Stat label="sieve">{meter ? meter.sieve().toLocaleString('en') : ''}</Stat>
        <Stat label="zeros">{`${pick.k} of ${ZEROS}`}</Stat>
        <Stat label="last height">{last === null ? 'none' : last.toFixed(4)}</Stat>
        <Stat label="its amplitude">{last === null ? 'none' : amplitudes[pick.k - 1].toExponential(3)}</Stat>
        <Stat label="fit">{meter ? miss.toExponential(1) : ''}</Stat>
      </Stats>
      <Note error={error} />
    </Page>
  );
}

mount(<App />);
