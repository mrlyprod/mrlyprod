import { useEffect, useMemo, useRef, useState } from 'react';
import { ready, ink, blit, paint } from '../lib/mrly.js';
import { mount, Page, Row, Slider, Btn, Stats, Stat, Note } from '../lib/app.jsx';
import { Pixels, Sketch } from '../lib/draw.jsx';
import { useQuery } from '../lib/query.js';
import { useSeeds, roll, Ramp, Sources, readSource, seedSource, SOURCE_FIRST } from '../lib/select.jsx';
import { board, line, axis, tag } from '../lib/chart.js';

const m = await ready();
const STEPS = 512;
const SIZE = 512;
const NEEDLES = [['33', 33], ['45', 45], ['78', 78], ['899', 899], ['900', 900]];
const FIRST = { ...SOURCE_FIRST, rpm: 33 };

const drawn = (seed) => ({ rpm: roll(seed, [[0, 1800]])[0] });

const offscreen = () => {
  const canvas = document.createElement('canvas');
  canvas.width = SIZE;
  canvas.height = SIZE;
  return canvas;
};

function App() {
  const s = useSeeds();
  const [pick, set] = useQuery({ ...seedSource(s, FIRST), ...(s.get() ? drawn(s.get()) : null) });
  const [look, setLook] = useState({ ramp: 'fire', levels: 64, invert: false });
  const [glow, setGlow] = useState(1);
  const [playing, setPlaying] = useState(true);
  const strobe = useRef(null);
  const table = useRef(null);
  const disc = useRef(null);
  const raw = useRef(null);
  const angle = useRef(0);
  const last = useRef(0);
  const fps = useRef(60);
  disc.current ??= offscreen();
  raw.current ??= document.createElement('canvas');

  const key = Object.keys(SOURCE_FIRST).map((name) => pick[name]).join(':');

  const made = useMemo(() => {
    try {
      const src = readSource(pick);
      const profile = m.profile(src.field, src.size, STEPS);
      return {
        view: {
          grid: src.grid, name: src.name, fills: src.fills, side: src.size, profile,
          raw: src.grid ? null : m.sheet(src.field, src.size, look.ramp, look.levels, look.invert),
          wheel: m.wheel(profile, SIZE, look.ramp, look.levels, look.invert),
          stats: JSON.parse(m.spin_stats(profile, src.size)),
        },
      };
    } catch (error) {
      return { error };
    }
  }, [key, look]);

  const shown = useRef(null);
  if (made.view) shown.current = made.view;
  const view = shown.current;

  useEffect(() => {
    if (!view) return;
    if (view.grid) paint(raw.current, view.grid, ink.blue, ink.deep);
    else blit(raw.current, view.raw);
    const dctx = disc.current.getContext('2d');
    dctx.imageSmoothingEnabled = false;
    dctx.drawImage(raw.current, 0, 0, SIZE, SIZE);
    const tctx = table.current.getContext('2d');
    tctx.globalAlpha = 1;
    tctx.fillStyle = ink.deep;
    tctx.fillRect(0, 0, SIZE, SIZE);
  }, [view]);

  useEffect(() => {
    let id = 0;
    const frame = (now) => {
      id = requestAnimationFrame(frame);
      const dt = last.current ? Math.min(0.05, (now - last.current) / 1000) : 0;
      last.current = now;
      if (dt > 0) fps.current = fps.current * 0.95 + 0.05 / dt;
      if (playing) angle.current = (angle.current + pick.rpm * 6 * dt) % 360;
      if (strobe.current) strobe.current.textContent = `${fps.current.toFixed(0)} fps, ${m.frame_step(pick.rpm, fps.current).toFixed(1)}° per frame`;
      const tctx = table.current.getContext('2d');
      tctx.globalAlpha = 1 / glow;
      tctx.fillStyle = ink.deep;
      tctx.fillRect(0, 0, SIZE, SIZE);
      tctx.save();
      tctx.translate(SIZE / 2, SIZE / 2);
      tctx.rotate(angle.current * Math.PI / 180);
      const side = SIZE / Math.SQRT2;
      tctx.drawImage(disc.current, -side / 2, -side / 2, side, side);
      tctx.restore();
    };
    id = requestAnimationFrame(frame);
    return () => cancelAnimationFrame(id);
  }, [pick.rpm, glow, playing]);

  const chart = (canvas) => {
    if (!shown.current) return;
    const { profile, stats } = shown.current;
    const b = board(canvas, 200);
    const peak = Math.max(stats.peak, 1e-9);
    const low = Math.min(0, ...profile);
    const end = profile.length - 1;
    line(b, Array.from(profile, (v, k) => [k / end, (v - low) / (peak - low)]), ink.blue, { fill: 0.25 });
    const mark = (r, color, label, dx) => {
      const at = b.x(r / stats.reach);
      b.ctx.strokeStyle = color;
      b.ctx.lineWidth = 1;
      b.ctx.setLineDash([3, 3]);
      b.ctx.beginPath();
      b.ctx.moveTo(at, b.roof - 4);
      b.ctx.lineTo(at, b.floor);
      b.ctx.stroke();
      b.ctx.setLineDash([]);
      tag(b, label, color, 'left', at + dx);
    };
    if (stats.disc > 0) mark(stats.disc, ink.pink, `dark disc ${stats.disc.toFixed(2)}`, 4);
    mark(stats.inner, ink.gold, `edge ${stats.inner.toFixed(1)}`, -60);
    axis(b, [[0, '0'], [1, `radius in cells, corner ${stats.reach.toFixed(1)}`]]);
    tag(b, `circle mean, peak ${stats.peak.toFixed(3)}`, ink.blue);
  };

  return (
    <Page crumb="spin" title="Spin the carpet like a record"
      sub="Put a design on a turntable and speed it up. The screen strobes at its own frame rate, so a fast carpet freezes, drifts backwards, or smears; the eye keeps an afterglow and blends the frames. Spun infinitely fast, every pixel becomes the mean of the picture on its own circle, and that picture is not guessed: the wheel on the right is the exact circle mean at every radius, a bullseye whose rings are the design."
      foot={<>The turntable is an ordinary rotation drawn once per screen frame; afterglow blends each new frame into the old ones. The wheel is computed in Rust: every circle about the centre is cut at the grid lines it crosses and each arc is read from the one cell it lies in, so the mean over the circle is exact, and the rings integrate back to the mass of the source. The theory behind the bullseye is one identity: a plane wave averaged over a turn is the Bessel function <code>J0(|k| r)</code>, so the wheel is the Hankel transform of the radially averaged spectrum. On the square lattice the ring frequencies are the sums of two squares; on the hexagonal lattice they are the Loeschian numbers. A 60 Hz screen freezes a carpet at 900 rpm, a quarter turn per frame.</>}>
      <Row>
        <Sources value={pick} onChange={set} seeds={s} onSeed={(seed) => set(drawn(seed))} />
      </Row>
      <Row>
        <Slider label="rpm" value={pick.rpm} min={0} max={1800} onChange={(v) => set({ rpm: v })} />
        <span className="tabs">
          {NEEDLES.map(([word, rpm]) => <Btn key={word} onClick={() => set({ rpm })}>{word}</Btn>)}
        </span>
        <Slider label="afterglow" value={glow} min={1} max={120} onChange={setGlow} />
        <Btn onClick={() => setPlaying(!playing)}>{playing ? 'Stop' : 'Spin'}</Btn>
      </Row>
      <Row>
        <Ramp value={look} onChange={(patch) => setLook({ ...look, ...patch })} />
      </Row>
      <div className="arena">
        <div className="panel">
          <h2>The turntable <span ref={strobe} /></h2>
          <canvas ref={table} className="sheet" width={SIZE} height={SIZE} />
        </div>
        <div className="panel">
          <h2>Infinite speed <span>the exact circle means</span></h2>
          {view && <Pixels data={view.wheel} />}
        </div>
      </div>
      <Sketch draw={chart} deps={[view]} className="bars" />
      <Stats>
        <Stat label="name">{view?.name}</Stat>
        <Stat label="side">{view?.side}</Stat>
        <Stat label="filled">{view?.fills}</Stat>
        <Stat label="mass of the rings">{view?.stats.mass.toFixed(1)}</Stat>
        <Stat label="dark disc to">{view?.stats.disc.toFixed(2)}</Stat>
        <Stat label="brightest ring">{view?.stats.peak.toFixed(3)}</Stat>
        <Stat label="last ring at">{view?.stats.reach.toFixed(1)}</Stat>
      </Stats>
      <Note error={made.error} />
    </Page>
  );
}

mount(<App />);
