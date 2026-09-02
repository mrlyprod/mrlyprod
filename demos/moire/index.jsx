import { useEffect, useMemo, useRef, useState } from 'react';
import { ready, ink } from '../lib/mrly.js';
import { mount, Page, Row, Pick, Slider, Btn, Stats, Stat, Note } from '../lib/app.jsx';
import { Pixels, Sketch } from '../lib/draw.jsx';
import { board, bars, axis, tag } from '../lib/chart.js';
import { mix } from '../lib/series.jsx';
import { useQuery, stamp } from '../lib/query.js';
import { useSeeds, roll, Ramp, Cropper, cropOf } from '../lib/select.jsx';

const m = await ready();
const NAMES = [...m.moire_names()];
const SIZES = [128, 256, 384, 512];
const PALE = '#eef2f6';

const drawn = (seed) => {
  const [preset, limit] = roll(seed, [[0, NAMES.length - 1], [1, 41]]);
  return { preset: NAMES[preset], limit: limit | 1 };
};

const heat = (r) => {
  const t = Math.min(1, Math.sqrt(Math.max(0, r)));
  return t < 0.5 ? mix(PALE, ink.blue, t * 2) : mix(ink.blue, ink.deep, t * 2 - 1);
};

const shown = (r) => (r === 0 ? '0' : r.toFixed(9));

function Heat({ scales, grid, clear, row, mate }) {
  const draw = (canvas) => {
    const n = scales.length;
    const gutter = 42;
    const free = Math.max(60, canvas.clientWidth - gutter - 12);
    const size = n ? Math.max(4, Math.min(26, free / n)) : 0;
    const b = board(canvas, Math.round(52 + n * size), { left: gutter, right: 12, top: 44, bottom: 8 });
    if (!n) {
      tag(b, 'no odd scale reaches 3 yet', ink.dim);
      return;
    }
    const every = Math.max(1, Math.ceil(n / Math.max(1, Math.floor(free / 24))));
    scales.forEach((a, i) => {
      const y = b.roof + i * size;
      grid[i].forEach((r, j) => {
        b.ctx.fillStyle = heat(r);
        b.ctx.fillRect(gutter + j * size, y, Math.max(1, size - 1), Math.max(1, size - 1));
      });
      if (i % every) return;
      const hue = a === row ? ink.gold : clear[i] ? ink.green : ink.dim;
      tag(b, String(a), hue, 'right', gutter - 6, y + size / 2 + 4);
      tag(b, String(a), hue, 'center', gutter + i * size + size / 2, b.roof - 10);
    });
    const k = scales.indexOf(row);
    const j = scales.indexOf(mate);
    b.ctx.lineWidth = 1;
    b.ctx.strokeStyle = ink.dim;
    b.ctx.beginPath();
    b.ctx.moveTo(gutter, b.roof);
    b.ctx.lineTo(gutter + n * size, b.roof + n * size);
    b.ctx.stroke();
    if (k >= 0) {
      b.ctx.strokeStyle = ink.gold;
      b.ctx.strokeRect(gutter - 1.5, b.roof + k * size - 1.5, n * size + 2, size + 2);
    }
    if (j >= 0) {
      b.ctx.strokeStyle = ink.pink;
      b.ctx.strokeRect(gutter + j * size - 1.5, b.roof - 1.5, size + 2, n * size + 2);
    }
    tag(b, 'white where the correlation is exactly zero; left of the diagonal a row clear to its end is prime, in green', ink.dim);
  };
  return <Sketch className="bars" draw={draw} deps={[grid, clear, row, mate]} />;
}

function Strip({ witness, mate, onPick }) {
  const n = witness.scales.length;
  const draw = (canvas) => {
    const b = board(canvas, 170);
    if (!n) {
      axis(b);
      tag(b, `scale ${witness.n} has no earlier odd scale`, ink.dim);
      return;
    }
    bars(b, witness.row, {
      peak: Math.max(witness.max, 1e-12),
      color: (i, v) => (witness.scales[i] === mate ? ink.gold : v === 0 ? ink.line : ink.blue),
    });
    const step = Math.max(1, Math.ceil(n / 14));
    axis(b, witness.scales.map((s, i) => [(i + 0.5) / n, String(s)]).filter((_, i) => i % step === 0));
    witness.row.forEach((v, i) => {
      if (v !== 0 && witness.scales[i] !== mate) return;
      b.ctx.fillStyle = witness.scales[i] === mate ? ink.gold : ink.line;
      b.ctx.fillRect(b.x(i / n) + 1, b.floor - 3, Math.max(1, b.wide / n - 2), 3);
    });
    tag(b, `scale ${witness.n} against every earlier odd scale, click to pick one`, ink.dim);
    tag(b, witness.prime ? 'every bar exactly zero' : `largest ${witness.max.toFixed(6)} at scale ${witness.at}`, witness.prime ? ink.green : ink.gold, 'right');
  };
  const seek = (f) => onPick(witness.scales[Math.max(0, Math.min(n - 1, Math.floor(f * n)))]);
  return <Sketch className="bars" draw={draw} deps={[witness, mate]} onSeek={n ? seek : undefined} />;
}

function App() {
  const s = useSeeds();
  const [pick, setPick] = useState({ preset: NAMES[0], limit: 9, size: 256, ...(s.get() ? drawn(s.get()) : null) });
  const [look, setLook] = useState({ ramp: 'fire', levels: 16, invert: false });
  const [crop, setCrop] = useQuery({ crop: '', 'crop-r': 16, 'crop-anti': false });
  const [law, setLaw] = useQuery({ scale: 9, mate: 3 });
  const [playing, setPlaying] = useState(false);
  const shownField = useRef(null);

  let error = null;
  try {
    const c = cropOf(crop);
    let pixels;
    if (c.active) {
      const field = m.field_crop(m.moire_field(pick.preset, pick.limit, pick.size), pick.size, 2, c.shape, c.rnum, c.rden, c.anti);
      let low = Infinity, high = -Infinity;
      for (const v of field) if (!Number.isNaN(v)) { low = Math.min(low, v); high = Math.max(high, v); }
      pixels = m.paint_span(field, pick.size, low, high, look.ramp, look.levels, look.invert);
    } else {
      pixels = m.moire(pick.preset, pick.limit, pick.size, look.ramp, look.levels, look.invert);
    }
    shownField.current = { pixels, scales: Array.from(m.odd_scales(pick.limit)).join(' '), size: pick.size };
  } catch (fault) {
    error = fault;
  }

  const scales = useMemo(() => Array.from(m.odd_scales(pick.limit)).filter((v) => v >= 3), [pick.limit]);
  const grid = useMemo(() => scales.map((a) => scales.map((b) => m.moire_correlation(a, b))), [scales]);
  const clear = useMemo(() => scales.map((a) => JSON.parse(m.carpet_witness(a)).prime), [scales]);
  const row = scales.filter((v) => v <= law.scale).at(-1) ?? scales[0] ?? 0;
  const witness = useMemo(() => (row ? JSON.parse(m.carpet_witness(row)) : null), [row]);
  const mate = witness && witness.scales.includes(law.mate) ? law.mate : (witness?.scales.at(-1) ?? 0);
  const value = witness && mate ? witness.row[witness.scales.indexOf(mate)] : null;

  const step = () => setPick((old) => ({ ...old, limit: old.limit >= 41 ? 1 : old.limit + 2 }));

  useEffect(() => {
    if (!playing) return;
    const timer = setInterval(step, 350);
    return () => clearInterval(timer);
  }, [playing]);

  const view = shownField.current;

  return (
    <Page crumb="moire" title="Moire"
      sub="One design sampled at scale 1, 3, 5, and so on, the layers stacked into one field. Stacking is where the interference comes from: each new scale adds a finer grid on top of the coarse ones."
      foot={<>The heatmap sums the parity of the low corner over the odd scales, the weave folds the same layers to their parity, the hive samples on the hexagonal lattice, and the carpet keeps eight corners of nine in base 3. The field is quantized into levels and painted through a ramp; the pixels arrive already colored. The r-matrix reads a second law off the same stack: two parity carpets at odd scales correlate to exactly zero when the scales are coprime and to a strictly positive number when the scales share a factor, so a scale whose earlier row is all white is prime, the detector the <a href="../primes">primes</a> page stacks, with the closed form and its proof in the <a href="https://github.com/carlomitchener/carlomitchener/tree/main/research/moire-correlation-laws" target="_blank" rel="noopener">moire correlation laws</a> lane. Every number in the panel is computed in Rust; the page only draws.</>}>
      <Row>
        <Pick label="preset" value={pick.preset} options={NAMES} onChange={(v) => setPick({ ...pick, preset: v })} />
        <Slider label="scales up to" value={pick.limit} min={1} max={41} step={2} onChange={(v) => setPick({ ...pick, limit: v })} />
        <Pick label="size" value={pick.size} options={SIZES.map((v) => [v, v])} onChange={(v) => setPick({ ...pick, size: +v })} />
        <Ramp value={look} onChange={(patch) => setLook({ ...look, ...patch })} />
        <Cropper value={crop} onChange={(patch) => { setCrop(patch); if (!({ ...crop, ...patch }).crop) stamp({ crop: null, 'crop-r': null, 'crop-anti': null }); }} />
        <Btn onClick={() => { setPlaying(!playing); if (!playing) step(); }}>{playing ? 'Stop' : 'Play the scales'}</Btn>
        <Btn onClick={() => setPick({ ...pick, ...drawn(s.next()) })}>Randomize</Btn>
      </Row>
      {view && <Pixels data={view.pixels} style={{ maxWidth: 640 }} />}
      <Stats>
        <Stat label="scales">{view?.scales}</Stat>
        <Stat label="pixels">{view && `${view.size} by ${view.size}`}</Stat>
      </Stats>
      <div className="panel" style={{ marginTop: 22 }}>
        <h2>The r-matrix <span>the correlation of the flat carpet layers at every pair of odd scales</span></h2>
        <Row>
          <Slider label="carpet" value={row || 3} min={3} max={Math.max(3, scales.at(-1) ?? 3)} step={2} onChange={(v) => setLaw({ scale: v })} />
        </Row>
        <Heat scales={scales} grid={grid} clear={clear} row={row} mate={mate} />
        {witness && <Strip witness={witness} mate={mate} onPick={(v) => setLaw({ mate: v })} />}
        <Stats>
          <Stat label="scale m">{row || 'none'}</Stat>
          <Stat label="scale n">{mate || 'none earlier'}</Stat>
          <Stat label="r">{value === null ? 'none' : shown(value)}</Stat>
          <Stat label="row of m">{witness ? (witness.prime ? 'clear against every earlier scale' : `largest ${shown(witness.max)} at scale ${witness.at}`) : 'none'}</Stat>
          <Stat label="matrix">{`${scales.length} by ${scales.length}`}</Stat>
        </Stats>
      </div>
      <Note error={error} />
    </Page>
  );
}

mount(<App />);
