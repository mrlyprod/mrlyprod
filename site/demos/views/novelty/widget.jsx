import { useEffect, useMemo, useState } from 'react';
import { ready, ink } from '../../../lib/mrly.js';
import { Row, Slider, Pick } from '../../../lib/app.jsx';
import { Sketch } from '../../../lib/draw.jsx';
import { board, line, axis, tag } from '../../../lib/chart.js';
import { embed } from '../../../lib/widget.jsx';

const m = await ready();

export const LOW = 8;
export const HIGH = 20;
export const PER_OCTAVE = 16;
export const ZEROS = 138;
export const CURVE = 900;
const SHORT = 16;
const FEW = 40;

export const meterChart = ({ heights, dots, curve, wave, j, k, sharp, height }) => (canvas) => {
  const b = board(canvas, height, { top: 26, bottom: 22 });
  if (!dots.length) return;
  const wide = j - LOW;
  let span = 1e-9;
  for (let i = 0; i < heights.length; i++) if (heights[i] <= j) span = Math.max(span, Math.abs(dots[i]));
  for (const v of wave) span = Math.max(span, Math.abs(v));
  span *= 1.1;
  b.ctx.strokeStyle = ink.line;
  b.ctx.lineWidth = 1;
  b.ctx.beginPath();
  b.ctx.moveTo(b.x(0), b.y(0.5));
  b.ctx.lineTo(b.x(1), b.y(0.5));
  b.ctx.stroke();
  if (k > 0) {
    const points = Array.from(wave, (v, i) => [(curve[i] - LOW) / wide, 0.5 + 0.5 * v / span]);
    line(b, points, ink.orange, { width: 1.5 });
  }
  b.ctx.fillStyle = ink.blue;
  for (let i = 0; i < heights.length; i++) {
    if (heights[i] > j) break;
    b.ctx.beginPath();
    b.ctx.arc(b.x((heights[i] - LOW) / wide), b.y(0.5 + 0.5 * dots[i] / span), 2.6, 0, Math.PI * 2);
    b.ctx.fill();
  }
  axis(b, [[0, `y = 2^-${LOW}`], [1, `y = 2^-${j}`]]);
  tag(b, sharp ? 'E(y) / y, the sharp window' : 'E(y) / y^(3/2), the smooth window', ink.blue);
  const whose = k === 1 ? 'the first zero' : `the first ${k} zeros`;
  tag(b, k ? (sharp ? `the wave of ${whose}, times sqrt(y)` : `the wave of ${whose}`) : 'no zeros', ink.orange, 'right');
};

export function meter() {
  const [k, setK] = useState(1);
  const [cut, setCut] = useState('smooth');
  const [reading, setReading] = useState(null);
  useEffect(() => {
    setReading(new m.Novelty(SHORT, PER_OCTAVE, FEW));
  }, []);
  const sharp = cut === 'sharp';
  const heights = useMemo(() => (reading ? reading.heights() : new Float64Array()), [reading]);
  const dots = useMemo(() => (reading ? reading.dots(sharp) : new Float64Array()), [reading, sharp]);
  const curve = useMemo(() => Float64Array.from({ length: CURVE + 1 }, (_, i) => LOW + (SHORT - LOW) * i / CURVE), []);
  const wave = useMemo(() => (reading ? reading.wave(k, sharp, curve) : new Float64Array()), [reading, k, sharp, curve]);
  const miss = useMemo(() => (reading ? reading.miss(k) : 1), [reading, k]);
  return (
    <>
      <Sketch draw={meterChart({ heights, dots, curve, wave, j: SHORT, k, sharp, height: 220 })} deps={[reading, dots, wave, k, sharp]} className="bars" role="img" aria-label="The novelty error as dots with the wave of the first zeros as a curve" />
      <Row>
        <Slider label="zeros" value={k} min={0} max={FEW} onChange={setK} />
        <Pick label="window" value={cut} options={[['smooth', 'smooth bump'], ['sharp', 'sharp cutoff']]} onChange={setCut} />
      </Row>
      <p className="sub">{reading ? (sharp ? `the sharp window's error over y, ${heights.length} readings the waves of ${k} zeros cannot reach` : `${k} of ${FEW} zeros miss the ${heights.length} smoothed readings by ${miss.toExponential(1)} of their peak`) : 'sieving the totients'}</p>
    </>
  );
}

embed('novelty', { meter });
