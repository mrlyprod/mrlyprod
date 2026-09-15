import { useMemo, useRef, useState } from 'react';
import * as THREE from 'three';
import { ready, ink } from '../../lib/mrly.js';
import { faces } from '../../lib/stage.js';
import { board, bars, line, axis, rules, tag } from '../../lib/chart.js';
import { mount, Page, Row, Pick, Slider, Check, Stats, Stat, Note, Group } from '../../lib/app.jsx';
import { Grid, Markup, Sketch } from '../../lib/draw.jsx';
import { Stage } from '../../lib/stage.jsx';
import { useSeeds, seeded, Picker } from '../../lib/select.jsx';
import { useQuery } from '../../lib/query.js';

const m = await ready();
const RDEN = 120;
const RMAX = 108;
const CELLS = 600000;
const FOLDS = 64;
const PAD = 14;
const SIGNS = [];
for (let bits = 0; bits < 8; bits++) SIGNS.push([1 - 2 * (bits & 1), 1 - 2 * ((bits >> 1) & 1), 1 - 2 * ((bits >> 2) & 1)]);
const WALLS = {
  box: [[1, 0, 0], [-1, 0, 0], [0, 1, 0], [0, -1, 0], [0, 0, 1], [0, 0, -1]],
  diamond: SIGNS,
  octahedron: SIGNS,
  tetrahedron: [[1, 1, 1], [-1, -1, 1], [-1, 1, -1], [1, -1, -1]],
  pyramid: [[1, 0, 0], [-1, -2, 0], [-1, 2, 0], [-1, 0, -2], [-1, 0, 2]],
};
const POLICIES = { 2: ['inside', 'touching', 'refined1', 'refined2'], 3: ['inside', 'touching'] };
const ART = { background: 'var(--fg)', borderRadius: '8px', padding: '8px', lineHeight: 0 };
const DIM = +(new URLSearchParams(location.search).get('dim') ?? 2);

function planes(shape, r) {
  return WALLS[shape].map((n) => {
    const size = Math.hypot(...n);
    return new THREE.Plane(new THREE.Vector3(-n[0], -n[1], -n[2]).divideScalar(size), (2 * r) / size);
  });
}

function Num({ label, value, min, max, onChange }) {
  return <label>{label} <input type="number" value={value} min={min} max={max} onChange={(e) => onChange(+e.target.value)} /></label>;
}

function Toggle({ label, checked, disabled, hidden, onChange }) {
  return <label hidden={hidden}><input type="checkbox" checked={checked} disabled={disabled} onChange={(e) => onChange(e.target.checked)} /> {label}</label>;
}

function App() {
  const s = useSeeds();
  const [q, set] = useQuery({
    dim: DIM, code: seeded(s, DIM, 2, DIM === 2 ? '7' : '23'), base: 2, number: 3, level: 3,
    shape: 'ball', radius: 60, mode: 'crop', policy: 'touching',
    centre: 'corner', count: DIM === 2 ? 6 : 4, r: 27, low: -2, high: -1,
  });
  const [crisp, setCrisp] = useState(false);
  const [spin, setSpin] = useState(false);
  const live = useRef(null);
  const shapes = useMemo(() => JSON.parse(m.crop_shapes(q.dim)), [q.dim]);
  const top = m.level_cap(q.number, 1, q.dim === 2 ? 243 : 81);
  const level = Math.min(q.level, top);
  const countTop = m.level_cap(q.number, q.dim, CELLS);
  const count = Math.min(q.count, countTop);

  const circle = useMemo(() => {
    const out = {};
    try {
      const code = q.code.trim();
      const flat = m.crop_circle(code, q.number, count, q.base, q.dim, q.centre);
      const width = flat.length / 3;
      out.seen = flat.subarray(0, width);
      out.inside = flat.subarray(width, 2 * width);
      out.cut = flat.subarray(2 * width, 3 * width);
      out.top = width - 1;
      out.mass = Number(m.fills(code, q.number, q.dim, 1, q.base));
      out.d = m.dimension(code, q.number, q.dim, q.base);
      out.side = m.grid_total(q.number, 1, count);
      out.marks = [];
      for (let r = q.number; r <= out.top; r *= q.number) out.marks.push(r);
    } catch (error) {
      out.error = error;
    }
    return out;
  }, [q.code, q.number, q.base, q.dim, q.centre, count]);

  const at = Math.max(1, Math.min(q.r, circle.top ?? q.r));
  const defect = circle.seen && q.number * at <= circle.top ? circle.seen[q.number * at] - circle.mass * circle.seen[at] : null;

  const fold = useMemo(() => {
    try {
      return JSON.parse(m.crop_collapse(q.code.trim(), q.number, count, q.base, q.dim, q.centre, FOLDS));
    } catch (error) {
      return { error, scales: [], pairs: [] };
    }
  }, [q.code, q.number, q.base, q.dim, q.centre, count]);

  const scales = fold.scales ?? [];
  const held = (want) => (scales.length ? Math.max(0, Math.min(scales.length - 1, want < 0 ? scales.length + want : want)) : 0);
  const [low, high] = [held(q.low), held(q.high)];
  const gap = fold.pairs?.find((row) => row.low === Math.min(low, high) && row.high === Math.max(low, high));

  const sweep = useMemo(() => {
    const out = {};
    try {
      const steps = q.dim === 2 ? 36 : 24;
      out.rows = JSON.parse(m.crop_series(q.code.trim(), q.number, level, q.base, q.dim, q.shape, q.radius, RDEN, q.mode === 'anti', 'radius', steps));
    } catch (error) {
      out.error = error;
    }
    return out;
  }, [q.code, q.base, q.number, level, q.dim, q.shape, q.mode]);

  const data = useMemo(() => {
    const out = {};
    try {
      const code = q.code.trim(), d = q.dim, anti = q.mode === 'anti';
      out.name = m.name_of(code, d, q.base);
      out.side = m.grid_total(q.number, 1, level);
      const census = JSON.parse(m.crop_census(code, q.number, level, q.base, d, q.shape, q.radius, RDEN, anti));
      out.census = census;
      out.solid = d === 3;
      out.cut = !!(out.solid && crisp && !anti && WALLS[q.shape]);
      if (!out.solid) {
        if (crisp) {
          const side = Number(m.grid_total(q.number, 1, level));
          const art = m.crop_svg(code, q.number, level, q.base, q.shape, q.radius, RDEN, anti, Math.max(2, Math.round(512 / side)));
          if (art.length > 4000000) throw new Error('that drawing is larger than this page serves; lower the level.');
          out.art = art;
          out.note = 'touching cells under the exact outline';
        } else {
          out.grid = m.crop_grid(code, q.number, level, q.base, q.shape, q.radius, RDEN, anti, q.policy);
          out.note = q.policy;
        }
      } else {
        const load = Number(out.cut ? census.exposed_before : census.exposed_after);
        if (load > 400000) throw new Error(`${load} faces is more than this page draws; lower the level.`);
        out.mesh = out.cut
          ? m.three_faces(code, q.number, level, q.base)
          : m.crop_faces(code, q.number, level, q.base, q.shape, q.radius, RDEN, anti, q.policy);
        out.note = out.cut ? 'the exact walls clip the full mesh' : q.policy;
      }
      if (sweep.error) throw sweep.error;
      const total = Number(census.filled_in) + Number(census.filled_cut) + Number(census.filled_out);
      out.rows = [{ x: 0, filled_in: anti ? total : 0, filled_cut: 0 }, ...sweep.rows];
      out.frac = q.radius / RDEN;
      out.levels = JSON.parse(m.crop_series(code, q.number, level, q.base, d, q.shape, q.radius, RDEN, anti, 'level', top));
      out.level = level;
    } catch (error) {
      out.error = error;
      out.art = null;
      out.grid = null;
      out.mesh = null;
    }
    return out;
  }, [q.code, q.base, q.number, level, q.dim, q.shape, q.radius, q.mode, q.policy, crisp, sweep]);

  const census = data.census ?? {};

  const turn = (on) => {
    setSpin(on);
    if (live.current) live.current.spin = on ? 0.004 : 0;
  };

  const shift = (v) => {
    const d = +v;
    const list = JSON.parse(m.crop_shapes(d));
    s.drop();
    set({
      dim: d, code: d === 2 ? '7' : '23',
      shape: list.includes(q.shape) ? q.shape : 'ball',
      policy: POLICIES[d].includes(q.policy) ? q.policy : 'touching',
    });
  };

  const alongRadius = (canvas) => {
    if (!data.rows) return;
    const b = board(canvas, 170, { pad: PAD, top: 16, bottom: 20 });
    const peak = Math.max(...data.rows.map((r) => Math.max(r.filled_in, r.filled_cut)), 1);
    line(b, data.rows.map((r) => [r.x, r.filled_in / peak]), ink.yellow);
    line(b, data.rows.map((r) => [r.x, r.filled_cut / peak]), ink.blue);
    axis(b, [[0, '0'], [1, 'radius 1']]);
    rules(b, [data.frac], { color: ink.pink });
    const edge = tag(b, 'in', ink.yellow);
    tag(b, 'cut', ink.blue, 'left', edge + 12);
  };

  const alongLevel = (canvas) => {
    if (!data.levels) return;
    const b = board(canvas, 170, { pad: PAD, top: 16, bottom: 20 });
    const logs = data.levels.map((r) => Math.log10(1 + r.filled_in));
    bars(b, logs, { color: (i) => (i === data.level ? ink.pink : ink.yellow), inset: 2 });
    axis(b, [[0, 'level 0'], [1, String(data.levels.length - 1)]]);
  };

  const alongCircle = (canvas) => {
    const { seen, cut, top, d, mass, marks } = circle;
    if (!seen || !seen[top] || top < 2) return;
    const b = board(canvas, 230, { pad: PAD, top: 20, bottom: 22 });
    const span = Math.log(top);
    let ceiling = Math.max(seen[top], 2);
    for (let r = 1; r <= top; r++) {
      ceiling = Math.max(ceiling, cut[r]);
      if (q.number * r <= top) ceiling = Math.max(ceiling, Math.abs(seen[q.number * r] - mass * seen[r]));
    }
    const roof = Math.log(ceiling);
    const lx = (r) => Math.log(r) / span;
    const ly = (v) => Math.log(v) / roof;
    const trail = (pick) => {
      const points = [];
      for (let r = 1; r <= top; r++) {
        const v = pick(r);
        if (v >= 1) points.push([lx(r), ly(v)]);
      }
      return points;
    };
    const ramp = (slope, v0) => {
      if (!(slope > 0) || !(v0 > 1)) return;
      const start = Math.max(1, top * Math.pow(v0, -1 / slope));
      line(b, [[lx(start), ly(v0 * Math.pow(start / top, slope))], [1, ly(v0)]], ink.dim, { width: 1, dash: [4, 4] });
    };
    rules(b, marks.map(lx), { dash: [2, 4] });
    rules(b, [lx(at)], { color: ink.pink });
    ramp(d, seen[top]);
    ramp(d - 1, cut[top]);
    line(b, trail((r) => seen[r]), ink.yellow);
    line(b, trail((r) => cut[r]), ink.blue);
    line(b, trail((r) => (q.number * r <= top ? Math.abs(seen[q.number * r] - mass * seen[r]) : 0)), ink.orange, { width: 1 });
    for (const r of marks) if (seen[r] >= 1) line(b, [[lx(r), ly(seen[r])]], ink.yellow, { dots: 3 });
    axis(b, [[0, 'r 1'], [1, `${top}`]], { wall: true });
    let edge = tag(b, `N slope ${d.toFixed(4)}`, ink.yellow);
    edge = tag(b, `C slope ${(d - 1).toFixed(4)}`, ink.blue, 'left', edge + 12);
    tag(b, 'defect', ink.orange, 'left', edge + 12);
  };

  const collapse = (canvas) => {
    if (scales.length < 1) return;
    const b = board(canvas, 230, { pad: PAD, top: 20, bottom: 22 });
    const [a, z] = [scales[low], scales[high]];
    const seen = a.main.concat(z.main);
    const floor = Math.min(...seen), roof = Math.max(...seen);
    const room = (roof - floor) * 0.1 || 0.05;
    const fx = (j) => j / FOLDS;
    const fy = (v) => (v - floor + room) / (roof - floor + 2 * room);
    const trail = (scale, color) => line(b, scale.main.map((v, j) => [fx(j), fy(v)]), color, { width: 1.6 });
    const step = Math.log(q.number);
    const turn = Math.log(at) / step;
    rules(b, [turn - Math.floor(turn)], { color: ink.pink });
    trail(a, ink.yellow);
    if (high !== low) trail(z, ink.green);
    axis(b, [[0, '0'], [0.5, `log_${q.number} r mod 1`], [1, '1']], { wall: true });
    let edge = tag(b, `R ${a.start} to ${a.stop}`, ink.yellow);
    if (high !== low) edge = tag(b, `R ${z.start} to ${z.stop}`, ink.green, 'left', edge + 12);
    tag(b, 'N / r^d', ink.dim, 'left', edge + 12);
    if (gap) tag(b, `gap ${gap.sup.toFixed(4)}`, ink.dim, 'right');
  };

  const ridge = (canvas) => {
    if (scales.length < 1) return;
    const b = board(canvas, 230, { pad: PAD, top: 20, bottom: 22 });
    const [a, z] = [scales[low], scales[high]];
    const seen = a.drift.concat(high === low ? [] : z.drift);
    if (!seen.length) {
      tag(b, 'the fold runs past the counted radii', ink.dim);
      axis(b, [[0, '0'], [0.5, `log_${q.number} r mod 1`], [1, '1']], { wall: true });
      return;
    }
    const floor = Math.min(...seen), roof = Math.max(...seen);
    const room = (roof - floor) * 0.1 || 0.05;
    const fx = (j) => j / FOLDS;
    const fy = (v) => (v - floor + room) / (roof - floor + 2 * room);
    const trail = (scale, color) => scale.drift.length && line(b, scale.drift.map((v, j) => [fx(j), fy(v)]), color, { width: 1.6 });
    const turn = Math.log(at) / Math.log(q.number);
    rules(b, [turn - Math.floor(turn)], { color: ink.pink });
    trail(a, ink.orange);
    if (high !== low) trail(z, ink.green);
    axis(b, [[0, '0'], [0.5, `log_${q.number} r mod 1`], [1, '1']], { wall: true });
    let edge = tag(b, `R ${a.start} to ${a.stop}`, a.drift.length ? ink.orange : ink.dim);
    if (high !== low) edge = tag(b, `R ${z.start} to ${z.stop}`, z.drift.length ? ink.green : ink.dim, 'left', edge + 12);
    tag(b, 'delta / r^(d - 1)', ink.dim, 'left', edge + 12);
    if (gap?.rsup != null) tag(b, `gap ${gap.rsup.toFixed(4)}`, ink.dim, 'right');
  };

  const folds = scales.map((scale, k) => [k, `R ${scale.start}`]);

  const controls = (
    <>
      <Group name="Design">
        <Picker dimension={q.dim} bases={[2, 3]} code={q.code} base={q.base} seeds={s} onChange={set} />
        <Num label="number" value={q.number} min={2} max={5} onChange={(v) => set({ number: v })} />
        <Slider label="level" value={level} min={1} max={top} onChange={(v) => set({ level: v })} />
      </Group>
      <Group name="Shape">
        <Pick label="dimension" value={q.dim} options={[2, 3]} onChange={shift} />
        <Pick label="shape" value={q.shape} options={shapes} onChange={(v) => set({ shape: v })} />
        <Slider label="radius" value={q.radius} min={0} max={RMAX} show={`${q.radius}/${RDEN}`} onChange={(v) => set({ radius: v })} />
        <Pick label="mode" value={q.mode} options={['crop', 'anti']} onChange={(v) => set({ mode: v })} />
        <Pick label="policy" value={q.policy} options={POLICIES[q.dim]} onChange={(v) => set({ policy: v })} />
      </Group>
      <Group name="Circle">
        <Pick label="centre" value={q.centre} options={['corner', 'centre']} onChange={(v) => set({ centre: v })} />
        <Slider label="count level" value={count} min={1} max={countTop} onChange={(v) => set({ count: v })} />
        <Slider label="radius r" value={at} min={1} max={Math.max(1, circle.top ?? 1)} show={`${at}/${circle.top ?? 1}`} onChange={(v) => set({ r: v })} />
        <Pick label="scale A" value={low} options={folds} onChange={(v) => set({ low: +v })} />
        <Pick label="scale B" value={high} options={folds} onChange={(v) => set({ high: +v })} />
      </Group>
      <Group name="View">
        <Check label="exact edge" checked={crisp} onChange={setCrisp} />
        <Toggle label="spin" checked={spin} disabled={data.cut} hidden={q.dim === 2} onChange={turn} />
      </Group>
    </>
  );

  return (
    <Page crumb="crop" title="A shape keeps only the cells of a design it reaches" controls={controls}
      sub="A named shape of rational radius sits on the unit square or cube and keeps only the cells of a design it reaches: strictly inside, touching, or rebuilt on a finer lattice at the rim. The census splits every cell into in, cut and out before anything is drawn, and the sweeps show how the kept mass grows with the radius and the level. Drag the radius chart to move the shape. Below it the radius stops being a fraction of the box and runs in whole cells: the count of filled cells the ball holds, the count its sphere crosses, and what the count leaves over when the radius is multiplied by the design's own side. The last panel takes two of the windows between consecutive powers of that side, divides the count by the radius to the dimension and reads both at the same offsets of the log radius, so a stranger can pick any two scales and watch them land on one curve. The panel beside it folds the left-over of that multiplication the same way, one power of the radius lower, so the defect has a ridge of its own to read."
      foot={<>The shape is exact rational geometry in Rust: a ball tested on squared fractions or a polytope of half-plane walls, never a float. A cell is in, cut or out by where its corners land, the census tallies the three regions and the perimeter or surface before and after the touching crop, and the sweeps re-run that census at every radius and level. The exact edge in the plane is the same touching crop clipped by the true circle or polygon in SVG; in the cube it clips the uncropped mesh with the shape's own walls as camera-space planes, so the ball and the anti crop stay on the raster mesh, which is always the source of truth for every count. The circle count is the same geometry in whole cells and one pass over the grid: a cell is seen when its own centre lands in the ball, inside when its far corner does and cut when the sphere separates its near corner from its far one, all on doubled integer coordinates with squared distances compared as integers, and the three columns are prefix sums by radius. Its main term is not a constant times r to the d: it is r to the d times a periodic multiplier of log r, which is why two windows between consecutive powers of the side, read at the same offsets of the log radius, land on one curve; the gap beside them is the largest distance between the two profiles and the share is that gap over the higher of their two mean levels, and both fall as the scales deepen, which is what makes the collapse a law rather than one lucky window. The defect is an exact integer, it rides one power below the count, and the last panel folds it at the same offsets over the radius to that lower power, so the two windows carry two ridges with a gap of their own; that ridge gap need not fall with the scale the way the count's does, and the panel shows it rather than hides it. A window whose multiplied radii run past the counted grid has no ridge and says so. At the grid centre the middle block is empty, so the count stays at zero out to the block's inradius. The exact classification, the census of the three regions, the circle theorem and the one open lane the cut column points at are in <a href="/research/crop/">the crop note</a>.</>}>
      <div className="arena" style={{ gridTemplateColumns: '3fr 2fr' }}>
        <div className="panel">
          <h2>The crop <span>{data.note}</span></h2>
          {data.grid && <Grid grid={data.grid} on={ink.yellow} role="img" aria-label="The crop" />}
          <Markup style={ART} hidden={!data.art} svg={data.art ?? ''} role="img" aria-label="The crop" />
          <Stage hidden={!data.solid} role="img" aria-label="The crop" deps={[data]} onStage={(st) => {
            live.current = st;
            st.renderer.localClippingEnabled = true;
            st.spin = data.cut || !spin ? 0 : 0.004;
            if (data.cut) setSpin(false);
            if (!data.mesh) {
              st.clear();
              return;
            }
            const mesh = faces(data.mesh, ink.blue);
            if (data.cut) mesh.material.clippingPlanes = planes(q.shape, q.radius / RDEN);
            st.show(mesh);
          }} />
        </div>
        <div className="panel">
          <h2>Along the radius <span>filled cells kept and cut</span></h2>
          <Sketch className="bars" role="img" aria-label="Along the radius" draw={alongRadius} deps={[data]} onSeek={(frac) => set({ radius: Math.min(RMAX, Math.max(0, Math.round(frac * RDEN))) })} />
          <h2>Along the level <span>kept fills, log scale</span></h2>
          <Sketch className="bars" role="img" aria-label="Along the level" draw={alongLevel} deps={[data]} />
        </div>
      </div>
      <Stats>
        <Stat label="name">{data.name}</Stat>
        <Stat label="side">{data.side}</Stat>
        <Stat label="filled in">{census.filled_in}</Stat>
        <Stat label="filled cut">{census.filled_cut}</Stat>
        <Stat label="filled out">{census.filled_out}</Stat>
        <Stat label="exposed before">{census.exposed_before}</Stat>
        <Stat label="exposed after">{census.exposed_after}</Stat>
      </Stats>
      <div className="arena">
        <div className="panel">
          <h2>The circle count <span>N, C and the defect on log axes</span></h2>
          <Sketch className="bars" role="img" aria-label="The circle count" draw={alongCircle} deps={[circle, at]} onSeek={(frac) => set({ r: Math.round(Math.exp(Math.max(0, Math.min(1, frac)) * Math.log(circle.top ?? 1))) })} />
        </div>
        <div className="panel">
          <h2>The collapse <span>two scales laid on each other</span></h2>
          <Sketch className="bars" role="img" aria-label="The collapse" draw={collapse} deps={[fold, low, high, at]} />
        </div>
        <div className="panel">
          <h2>The defect ridge <span>the same two scales, one power down</span></h2>
          <Sketch className="bars" role="img" aria-label="The defect ridge" draw={ridge} deps={[fold, low, high, at]} />
        </div>
      </div>
      <Stats>
        <Stat label="count side">{circle.side}</Stat>
        <Stat label="r">{at}</Stat>
        <Stat label="N(r)">{circle.seen?.[at]}</Stat>
        <Stat label="inside(r)">{circle.inside?.[at]}</Stat>
        <Stat label="C(r)">{circle.cut?.[at]}</Stat>
        <Stat label="defect">{defect === null ? 'past the grid' : defect}</Stat>
        <Stat label="d">{circle.d?.toFixed(7)}</Stat>
        <Stat label="fill of one tile">{circle.mass}</Stat>
        <Stat label="gap">{gap ? gap.sup.toFixed(5) : 'one scale'}</Stat>
        <Stat label="gap / level">{gap ? gap.share.toFixed(5) : 'one scale'}</Stat>
        <Stat label="ridge gap">{gap?.rsup != null ? gap.rsup.toFixed(5) : 'past the grid'}</Stat>
        <Stat label="ridge gap / ridge">{gap?.rshare != null ? gap.rshare.toFixed(5) : 'past the grid'}</Stat>
      </Stats>
      <Note error={data.error ?? circle.error ?? fold.error} />
    </Page>
  );
}

mount(<App />);
