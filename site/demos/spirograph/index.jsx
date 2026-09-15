import { useEffect, useMemo, useRef, useState } from 'react';
import { ready, ink, rgb, fit } from '../../lib/mrly.js';
import { mount, Page, Row, Pick, Slider, Btn, Check, Stats, Stat, Note } from '../../lib/app.jsx';
import { Sketch } from '../../lib/draw.jsx';
import { useQuery, stamp } from '../../lib/query.js';
import { useSeeds, seeded, Picker } from '../../lib/select.jsx';

const m = await ready();

const CAPS = JSON.parse(m.spirograph_caps());
const PENS = [['fill', 'the filled cells'], ['void', 'the empty cells'], ['both', 'both'], ['corners', 'the corners of the fills']];
const TRACKS = [['in', 'inside a circle'], ['out', 'outside a circle'], ['line', 'a straight line'], ['polyin', 'inside a polygon'], ['polyout', 'outside a polygon']];
const INKS = [['kind', 'fills blue, voids orange'], ['wheel', 'the six inks'], ['one', 'one ink']];
const SIDES = [3, 4, 5, 6, 8];
const LIVE = 300000;
const DENSITY = 720;
const TURN = 0.5;
const BEAT = 1000;
const PAD = 16;
const RASTER = [512, 256];
const CURVES = 64;
const SHADE = 110;

const place = (value, places) => (Number.isFinite(value) ? value.toFixed(places) : '-');

function App() {
  const s = useSeeds();
  const [q, setQ] = useQuery({
    code: seeded(s, 2, 3, '495'), base: 3, number: 3, level: 1, pens: 'fill', track: 'in',
    ring: 7, wheel: 3, sides: 4, laps: 2, reach: 90, jitter: 0, seed: 1, ink: 'kind', at: 1, fill: false,
  });
  const [at, setAt] = useState(q.at);
  const [playing, setPlaying] = useState(false);
  const live = useRef(at);
  const held = useRef(null);
  live.current = at;

  const cap = m.level_cap(q.number, 2, CAPS.pencils);
  const level = Math.min(q.level, cap);
  const code = q.code.trim();

  let error = null;
  try {
    const clock = performance.now();
    const grid = m.two_grid(code, q.number, level, 0, q.base);
    const args = [grid.types, grid.width, grid.height, q.pens, q.track, q.ring, q.wheel, q.sides, q.laps, q.reach / 100, q.jitter / 100, q.seed];
    const read = JSON.parse(m.spirograph_read(...args));
    const samples = Math.max(2, Math.min(Math.floor(CAPS.points / read.pencils), Math.floor(LIVE / read.pencils), DENSITY * read.orbits + 1));
    const curves = m.spirograph(...args, samples);
    held.current = { grid, read, curves, samples, name: m.name_of(code, 2, q.base), ms: performance.now() - clock };
  } catch (fault) {
    error = fault;
  }

  const view = held.current;
  const read = view?.read;
  const round = q.track === 'in' || q.track === 'out';
  const cover = useMemo(() => {
    if (!q.fill || !view || error) return null;
    if (!round) return { fault: 'the fill needs a circle track: the wall of a line or a polygon roulette need not close' };
    const side = RASTER[read.distinct > CURVES ? 1 : 0];
    try {
      const raw = m.spirograph_cover(view.grid.types, view.grid.width, view.grid.height, q.pens, q.track, q.ring, q.wheel, q.sides, q.laps, q.reach / 100, q.jitter / 100, q.seed, side);
      return { mask: raw.mask, side: raw.side, covered: raw.covered, hole: raw.hole, wall: raw.wall, winding: raw.winding, areas: raw.areas, disc: raw.disc };
    } catch (fault) {
      return { fault: String(fault?.message ?? fault) };
    }
  }, [q.fill, q.code, q.base, q.number, level, q.pens, q.track, q.ring, q.wheel, q.sides, q.laps, q.reach, q.jitter, q.seed]);

  useEffect(() => {
    if (!playing) return;
    let id = 0;
    let last = 0;
    const frame = (now) => {
      id = requestAnimationFrame(frame);
      const step = last ? Math.min(0.25, (now - last) / 1000) : 0;
      last = now;
      if (live.current >= 1) {
        setPlaying(false);
        return;
      }
      const turns = held.current?.read.turns || 1;
      if (step > 0) setAt((old) => Math.min(1, old + TURN * step / turns));
    };
    id = requestAnimationFrame(frame);
    const beat = setInterval(() => stamp({ at: place(live.current, 3) }), BEAT);
    return () => {
      cancelAnimationFrame(id);
      clearInterval(beat);
    };
  }, [playing]);

  const settle = (next) => {
    const value = Math.min(1, Math.max(0, next));
    setPlaying(false);
    setAt(value);
    setQ({ at: Number(value.toFixed(3)) });
  };

  const play = () => {
    if (playing) {
      settle(live.current);
      return;
    }
    if (live.current >= 1) setAt(0);
    setPlaying(true);
  };

  const draw = (canvas) => {
    const view = held.current;
    if (!view) return;
    const { read, curves, samples, grid } = view;
    const [x0, y0, x1, y1] = read.frame;
    const wide = canvas.clientWidth;
    const tall = Math.max(160, Math.min(wide, Math.round((wide - 2 * PAD) * (y1 - y0) / (x1 - x0)) + 2 * PAD));
    const [ctx, w, h] = fit(canvas, tall);
    ctx.clearRect(0, 0, w, h);
    const scale = Math.min((w - 2 * PAD) / (x1 - x0), (h - 2 * PAD) / (y1 - y0));
    const ox = w / 2 - (x0 + x1) / 2 * scale;
    const oy = h / 2 + (y0 + y1) / 2 * scale;
    const X = (x) => ox + x * scale;
    const Y = (y) => oy - y * scale;
    const six = [ink.blue, ink.orange, ink.yellow, ink.green, ink.pink, ink.indigo];
    const colour = (k, kind) => (q.ink === 'one' ? ink.fg : q.ink === 'wheel' ? six[k % 6] : kind === 'fill' ? ink.blue : kind === 'void' ? ink.orange : ink.teal);

    const track = new Path2D();
    read.outline.forEach(([x, y], i) => (i ? track.lineTo(X(x), Y(y)) : track.moveTo(X(x), Y(y))));
    if (read.closed) track.closePath();
    ctx.strokeStyle = ink.line;
    ctx.lineWidth = 1;
    ctx.setLineDash([4, 6]);
    ctx.stroke(track);
    ctx.setLineDash([]);

    if (cover && !cover.fault) {
      const [dx, dy, radius] = cover.disc;
      const n = cover.side;
      const sheet = document.createElement('canvas');
      sheet.width = sheet.height = n;
      const image = new ImageData(n, n);
      const tint = rgb(ink.dim);
      for (let i = 0; i < n * n; i++) {
        if (cover.mask[i] !== 3) continue;
        image.data.set(tint, i * 4);
        image.data[i * 4 + 3] = SHADE;
      }
      sheet.getContext('2d').putImageData(image, 0, 0);
      ctx.imageSmoothingEnabled = false;
      ctx.drawImage(sheet, X(dx - radius), Y(dy + radius), 2 * radius * scale, 2 * radius * scale);
      ctx.strokeStyle = ink.dim;
      ctx.setLineDash([1, 5]);
      ctx.beginPath();
      ctx.arc(X(dx), Y(dy), radius * scale, 0, Math.PI * 2);
      ctx.stroke();
      ctx.setLineDash([]);
    }

    const shown = Math.max(1, Math.round(at * (samples - 1)));
    ctx.lineWidth = 1.2;
    ctx.lineJoin = 'round';
    for (let k = 0; k < read.pencils; k++) {
      const base = k * samples * 2;
      const line = new Path2D();
      line.moveTo(X(curves[base]), Y(curves[base + 1]));
      for (let i = 1; i <= shown; i++) line.lineTo(X(curves[base + 2 * i]), Y(curves[base + 2 * i + 1]));
      ctx.strokeStyle = colour(k, read.seats[k][2]);
      ctx.stroke(line);
    }

    const [cx, cy, phi] = m.spirograph_pose(q.track, q.ring, q.wheel, q.sides, q.laps, at);
    ctx.strokeStyle = ink.dim;
    ctx.beginPath();
    ctx.arc(X(cx), Y(cy), read.wheel * scale, 0, Math.PI * 2);
    ctx.stroke();
    const cell = read.cell * read.wheel * scale;
    ctx.save();
    ctx.translate(X(cx), Y(cy));
    ctx.rotate(-phi);
    ctx.globalAlpha = 0.35;
    ctx.fillStyle = ink.dim;
    for (let i = 0; i < grid.height; i++) {
      for (let j = 0; j < grid.width; j++) {
        if (grid.types[i * grid.width + j]) ctx.fillRect((j - grid.width / 2) * cell + 0.5, (i - grid.height / 2) * cell + 0.5, cell - 1, cell - 1);
      }
    }
    ctx.restore();
    const c = Math.cos(phi);
    const sn = Math.sin(phi);
    read.seats.forEach(([px, py, kind], k) => {
      ctx.fillStyle = colour(k, kind);
      ctx.beginPath();
      ctx.arc(X(cx + read.wheel * (px * c - py * sn)), Y(cy + read.wheel * (px * sn + py * c)), 2.5, 0, Math.PI * 2);
      ctx.fill();
    });
    ctx.fillStyle = ink.fg;
    ctx.beginPath();
    ctx.arc(X(cx), Y(cy), 2, 0, Math.PI * 2);
    ctx.fill();
  };

  const circle = read && read.b > 0;
  const law = read
    ? circle
      ? `R/r = ${read.a}/${read.b}   closes after ${read.orbits} orbit${read.orbits === 1 ? '' : 's'}   ${read.fold}-fold   ${read.distinct} distinct curve${read.distinct === 1 ? '' : 's'} of ${read.pencils}`
      : q.track === 'line'
        ? `${read.distinct} shape${read.distinct === 1 ? '' : 's'} of ${read.pencils} pencils: on a line a seat's angle is a shift along the track, so curves of one radius are translates of one shape`
        : `${read.pencils} curves, ${read.orbits} lap${read.orbits === 1 ? '' : 's'}: no coincidence law on a polygon`
    : '';
  const filled = !cover
    ? ''
    : cover.fault
      ? cover.fault
      : `covered ${place(cover.covered * 100, 2)}% of the disc, of which wall ${place(cover.wall * 100, 2)}%   hole ${place(cover.hole * 100, 2)}%   winding ${place(cover.winding, 4)} of an exact ${place(cover.areas, 4)}   disc ${place(cover.disc[2], 3)} out, ${place(cover.disc[3], 3)} in   raster ${cover.side}`;
  const crossings = read && circle
    ? read.nodes === null
      ? "a seat sits at the wheel's centre, or at or past the threshold min(1, (a - b)/b) inside or 1 outside: the loops open or the seat crosses the centre path, and the node count is not the law's"
      : `nodes N = 2ab C(k,2) + k a(b-1) = ${read.nodes}`
    : '';

  const controls = (
    <>
      <section>
        <h3>The wheel</h3>
        <Row>
          <Picker dimension={2} bases={[3, 2]} code={q.code} base={q.base} seeds={s} onChange={(patch) => setQ(patch)} />
          <Pick label="side" value={q.number} options={[[3, 3], [5, 5], [7, 7]]} onChange={(v) => setQ({ number: +v })} />
          <Slider label="level" value={level} min={1} max={cap} onChange={(v) => setQ({ level: v })} />
          <Pick label="pencils" value={q.pens} options={PENS} onChange={(v) => setQ({ pens: v })} />
          <Slider label="reach" value={q.reach} min={20} max={130} show={`${(q.reach / 100).toFixed(2)} r`} onChange={(v) => setQ({ reach: v })} />
          <Slider label="jitter" value={q.jitter} min={0} max={100} show={`${(q.jitter / 100).toFixed(2)} cells`} onChange={(v) => setQ({ jitter: v })} />
        </Row>
      </section>
      <section>
        <h3>The track</h3>
        <Row>
          <Pick label="track" value={q.track} options={TRACKS} onChange={(v) => setQ({ track: v })} />
          <Slider label="ring radius R" value={q.ring} min={1} max={CAPS.radius} onChange={(v) => setQ({ ring: v })} />
          <Slider label="wheel radius r" value={q.wheel} min={1} max={CAPS.radius} onChange={(v) => setQ({ wheel: v })} />
          <Pick label="sides" value={q.sides} options={SIDES.map((n) => [n, n])} onChange={(v) => setQ({ sides: +v })} />
          <Slider label="laps" value={q.laps} min={1} max={CAPS.laps} onChange={(v) => setQ({ laps: v })} />
        </Row>
      </section>
      <section>
        <h3>The draw</h3>
        <Row>
          <Slider label="drawn" value={Math.round(at * 1000)} min={0} max={1000} show={`${place(at * 100, 0)}%`} onChange={(v) => settle(v / 1000)} />
          <Btn primary on={playing} onClick={play}>{playing ? 'Stop' : 'Play'}</Btn>
          <Pick label="ink" value={q.ink} options={INKS} onChange={(v) => setQ({ ink: v })} />
          <Check label="fill the shape between the walls" checked={q.fill} onChange={(v) => setQ({ fill: v })} />
        </Row>
      </section>
    </>
  );

  return (
    <Page crumb="spirograph" title="The spirograph"
      sub="A design is the wheel and its cells are the holes: a pencil in every one, and the wheel rolls without slipping on a straight line, inside or outside a circle, or around a polygon. The wheel's turn is its centre's path length over its radius, so every pencil draws a trochoid and the design draws them all at once. Two pencils draw the same curve exactly when a rotation of a full turn over b carries one seat onto the other, R/r = a/b in lowest terms, which the square lattice allows only by half turns when b is even and by quarter turns when four divides b. Inside the seat window, where every pencil sits off the wheel's centre and closer to it than min(1, A) wheel radii, A the centre path's radius in those units, (a - b)/b inside and (a + b)/b outside, no curve loops and no seat reaches the centre path: there two distinct curves cross 2ab times and one curve crosses itself a(b - 1) times, so the whole picture has 2ab C(k,2) + k a(b - 1) nodes with k the distinct curves, the design entering the count only through k. Every curve is also a wall no fluid crosses: pour fluid from outside the picture and it stops at the outer wall, stitched from the outermost arcs of the curves; pour it at the centre of the track and it stops at the inner wall. The shape between the two walls, pockets included, is what the fill shades, and covered is the share of the enclosing disc it takes, the chance a point dropped at random on the disc lands in the shape."
      controls={controls}
      foot={<>The seats, the track, the rolling, the trace, the closure, the coincidence law, the node count and the cover are computed in Rust; the page draws the polylines it is handed and the wheel where the crate poses it. The cover is a two-sided flood on a raster of the disc, so it counts the wall inside the shape and its digits carry a boundary error of the order of the curve length times the pixel, which is why the wall's own share is printed beside it. The winding readout checks the raster against Green's theorem and never the floods, which it cannot see: it is the mean signed winding number of the disc's pixel centres, read by scanline off the polylines, against pi b rho (rho -+ d^2/r) summed over the distinct curves and taken over the disc's area. What keeps a flood from leaking is instead the sample spacing, at most half a pixel, which leaves the wall unbroken. The pencil set is the design's own address set, so the picture is a rotation average of the design with an orbit added, the object of <a href="../radial">the radial page</a> and <a href="../spin">the spin page</a>; <a href="../tourbillon">the tourbillon</a> turns the layers of a stack instead of one tile.</>}>
      <Sketch draw={draw} deps={[view, cover, at, q.ink]} role="img" aria-label="The design rolled along its track, every pencil drawing its curve" />
      <Stats>
        <Stat label="wheel">{view?.name}</Stat>
        <Stat label="pencils">{read?.pencils}</Stat>
        <Stat label="curves">{read?.distinct}</Stat>
        <Stat label="nodes">{read && (read.nodes ?? '-')}</Stat>
        <Stat label="wheel turns">{read && place(read.turns, 2)}</Stat>
        <Stat label="covered">{cover && !cover.fault ? `${(cover.covered * 100).toFixed(0)}%` : '-'}</Stat>
        <Stat label="draw">{view && `${view.ms.toFixed(0)} ms`}</Stat>
      </Stats>
      {read && <pre>{[law, crossings, `pencils ${read.pencils}: ${read.fills} on fills, ${read.voids} on voids, ${read.corners} on corners   reach ${(q.reach / 100).toFixed(2)} r   cell ${place(read.cell, 4)} r`, `path length ${place(read.total, 2)}   samples per pencil ${view.samples}   drawn ${place(at * 100, 1)}%`,
        filled].filter(Boolean).join('\n')}</pre>}
      <Note error={error} />
    </Page>
  );
}

mount(<App />);
