import { useMemo, useRef, useState } from 'react';
import * as THREE from 'three';
import { ready, ink } from '../../lib/mrly.js';
import { faces } from '../../lib/stage.js';
import { board, bars, line, axis, tag } from '../../lib/chart.js';
import { mount, Page, Row, Pick, Slider, Check, Stats, Stat, Note, Group } from '../../lib/app.jsx';
import { Grid, Markup, Sketch } from '../../lib/draw.jsx';
import { Stage } from '../../lib/stage.jsx';
import { useSeeds, seeded, Picker } from '../../lib/select.jsx';
import { useQuery } from '../../lib/query.js';

const m = await ready();
const RDEN = 120;
const RMAX = 108;
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
  });
  const [crisp, setCrisp] = useState(false);
  const [spin, setSpin] = useState(false);
  const live = useRef(null);
  const shapes = useMemo(() => JSON.parse(m.crop_shapes(q.dim)), [q.dim]);
  const top = m.level_cap(q.number, 1, q.dim === 2 ? 243 : 81);
  const level = Math.min(q.level, top);

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
    line(b, data.rows.map((r) => [r.x, r.filled_in / peak]), ink.gold);
    line(b, data.rows.map((r) => [r.x, r.filled_cut / peak]), ink.blue);
    axis(b, [[0, '0'], [1, 'radius 1']]);
    b.ctx.strokeStyle = ink.pink;
    b.ctx.beginPath();
    b.ctx.moveTo(b.x(data.frac), b.roof);
    b.ctx.lineTo(b.x(data.frac), b.floor);
    b.ctx.stroke();
    const edge = tag(b, 'in', ink.gold);
    tag(b, 'cut', ink.blue, 'left', edge + 12);
  };

  const alongLevel = (canvas) => {
    if (!data.levels) return;
    const b = board(canvas, 170, { pad: PAD, top: 16, bottom: 20 });
    const logs = data.levels.map((r) => Math.log10(1 + r.filled_in));
    bars(b, logs, { color: (i) => (i === data.level ? ink.pink : ink.gold), inset: 2 });
    axis(b, [[0, 'level 0'], [1, String(data.levels.length - 1)]]);
  };

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
      <Group name="View">
        <Check label="exact edge" checked={crisp} onChange={setCrisp} />
        <Toggle label="spin" checked={spin} disabled={data.cut} hidden={q.dim === 2} onChange={turn} />
      </Group>
    </>
  );

  return (
    <Page crumb="crop" title="A shape keeps only the cells of a design it reaches" controls={controls}
      sub="A named shape of rational radius sits on the unit square or cube and keeps only the cells of a design it reaches: strictly inside, touching, or rebuilt on a finer lattice at the rim. The census splits every cell into in, cut and out before anything is drawn, and the sweeps show how the kept mass grows with the radius and the level. Drag the radius chart to move the shape."
      foot={<>The shape is exact rational geometry in Rust: a ball tested on squared fractions or a polytope of half-plane walls, never a float. A cell is in, cut or out by where its corners land, the census tallies the three regions and the perimeter or surface before and after the touching crop, and the sweeps re-run that census at every radius and level. The exact edge in the plane is the same touching crop clipped by the true circle or polygon in SVG; in the cube it clips the uncropped mesh with the shape's own walls as camera-space planes, so the ball and the anti crop stay on the raster mesh, which is always the source of truth for every count. The exact classification, the census of the three regions and the one open lane the cut column points at are in <a href="/research/crop/">the crop note</a>.</>}>
      <div className="arena" style={{ gridTemplateColumns: '3fr 2fr' }}>
        <div className="panel">
          <h2>The crop <span>{data.note}</span></h2>
          {data.grid && <Grid grid={data.grid} on={ink.gold} role="img" aria-label="The crop" />}
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
      <Note error={data.error} />
    </Page>
  );
}

mount(<App />);
