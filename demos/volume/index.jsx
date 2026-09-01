import { useMemo, useRef, useState } from 'react';
import { ready, ink } from '../lib/mrly.js';
import { faces, plane } from '../lib/stage.js';
import { mount, Page, Row, Pick, Slider, Check, Btn, Stats, Stat, Note } from '../lib/app.jsx';
import { Stage } from '../lib/stage.jsx';
import { Pixels } from '../lib/draw.jsx';
import { useSeeds, seeded, roll, Picker, Ramp, Cropper, cropOf } from '../lib/select.jsx';
import { useQuery } from '../lib/query.js';

const m = await ready();
const NORMALS = { x: [1, 0, 0], y: [0, 1, 0], z: [0, 0, 1], d: [1, 1, 1] };
const CUTS = [['x', 'x plane'], ['y', 'y plane'], ['z', 'z plane'], ['d', 'diagonal']];
const COMBINE = ['sum', 'xor', 'and'];

function drawCombine(seed) {
  const [k] = roll(seed, [[0, COMBINE.length - 1]]);
  return COMBINE[k] === 'and' ? COMBINE[(k + 1) % COMBINE.length] : COMBINE[k];
}

function Cut({ label, on, at, onToggle, onSlide }) {
  return (
    <label><input type="checkbox" checked={on} onChange={(e) => onToggle(e.target.checked)} /> {label} <input type="range" min={0} max={1} step={0.01} value={at} onChange={(e) => onSlide(+e.target.value)} /></label>
  );
}

function App() {
  const s = useSeeds();
  const [q, set] = useQuery({
    code: seeded(s, 3, 2, '23'), base: 2, limit: 7, combine: s.get() ? drawCombine(s.get()) : 'sum', level: 1, size: 64,
    camera: 'eye', opacity: 1, crop: '', 'crop-r': 16, 'crop-anti': false,
  });
  const [shell, setShell] = useState(true);
  const [threshold, setThreshold] = useState(null);
  const [cuts, setCuts] = useState({ x: false, y: false, z: false, d: true });
  const [spot, setSpot] = useState({ x: 0.5, y: 0.5, z: 0.5, d: 0.5 });
  const [look, setLook] = useState({ ramp: 'fire', levels: 16, invert: false });
  const [spin, setSpin] = useState(false);
  const live = useRef(null);
  const first = useRef(true);

  const solid = useMemo(() => {
    const out = {};
    try {
      const code = q.code.trim();
      const data = m.volume(code, q.base, q.limit, q.combine, q.level, q.size);
      const stats = JSON.parse(m.volume_stats(data, q.size));
      const shape = JSON.parse(m.volume_shape(q.limit, q.size));
      out.name = m.name_of(code, 3, q.base);
      out.data = data;
      out.stats = stats;
      out.layers = shape.layers;
      out.voxels = shape.voxels;
    } catch (error) {
      out.error = error;
    }
    return out;
  }, [q.code, q.base, q.limit, q.combine, q.level, q.size]);

  const cap = solid.stats ? solid.stats.max : 4;
  const mark = threshold === null || threshold > cap ? cap : threshold;

  const view = useMemo(() => {
    const out = { planes: [] };
    if (!solid.data) return out;
    try {
      const c = cropOf(q);
      const shown = c.active ? m.field_crop(solid.data, q.size, 3, c.shape, c.rnum, c.rden, c.anti) : solid.data;
      out.count = m.volume_count(shown, q.size, mark);
      if (shell) {
        const quads = m.volume_surface(shown, q.size, mark);
        if (quads > 400000) throw new Error(`${quads} faces is more than this page draws; raise the threshold or lower the size.`);
        out.faces = quads;
        out.mesh = m.volume_faces(shown, q.size, mark);
      }
      for (const [key] of CUTS) {
        if (!cuts[key]) continue;
        const normal = NORMALS[key], offset = spot[key];
        const frame = JSON.parse(m.plane_frame(normal, offset));
        const wide = key === 'd' ? 384 : 256;
        const pixels = m.paint_span(m.plane_field(shown, q.size, normal, offset, wide), wide, solid.stats.min, solid.stats.max, look.ramp, look.levels, look.invert);
        out.planes.push({ pixels, frame });
        if (key === 'd') {
          out.cut = pixels;
          out.cutNote = `diagonal at ${offset}`;
        }
      }
      out.note = `${q.combine}, ${q.camera === 'iso' ? 'isometric' : 'perspective'}`;
    } catch (error) {
      out.error = error;
      out.mesh = null;
      out.cut = null;
      out.planes = [];
    }
    return out;
  }, [solid, mark, shell, cuts, spot, look, q.size, q.combine, q.camera, q.crop, q['crop-r'], q['crop-anti']]);

  const turn = (on) => {
    setSpin(on);
    if (live.current) live.current.spin = on ? 0.004 : 0;
  };

  const randomize = () => {
    const seed = s.next();
    set({ code: m.random_code(3, q.base, seed), combine: drawCombine(seed) });
  };

  return (
    <Page crumb="volume" title="The moire stack as a solid"
      sub="One cube design sampled at scale 1, 3, 5, and so on, the layers stacked into one solid field. A level set of the field is drawn as a shell, and any plane through it is painted as the moire that plane sees. Look down the diagonal in isometric projection and the cut through the centre is the hexagon the slices page counts. Drag to orbit, scroll to zoom."
      foot={<>Every voxel reads its design residue at each odd scale, and the layers are summed, folded to parity, or met. The shell is the crate's list of outward faces of the voxels at or above the threshold; a plane is sampled voxel by voxel in Rust and arrives as pixels with the outside of the cube transparent, so the diagonal cut shows its hexagon. Isometric projection is the orthographic camera; the corner button puts it on the <code>(1, 1, 1)</code> axis, where the cube's silhouette and its central cut are the same regular hexagon.</>}>
      <Row>
        <Picker dimension={3} bases={[2, 3]} code={q.code} base={q.base} seeds={s} button={false} onChange={set} />
        <Btn onClick={randomize}>Randomize</Btn>
        <Slider label="scales up to" value={q.limit} min={1} max={21} step={2} onChange={(v) => set({ limit: v })} />
        <Pick label="combine" value={q.combine} options={COMBINE} onChange={(v) => set({ combine: v })} />
        <Slider label="level" value={q.level} min={1} max={2} onChange={(v) => set({ level: v })} />
        <Pick label="size" value={q.size} options={[[48, 48], [64, 64], [80, 80]]} onChange={(v) => set({ size: +v })} />
      </Row>
      <Row>
        <Check label="shell at" checked={shell} onChange={setShell} />
        <Slider label="" value={mark} min={0} max={cap} step={1} onChange={(v) => setThreshold(v >= cap ? null : v)} />
        <Slider label="opacity" value={q.opacity} min={0.05} max={1} step={0.05} onChange={(v) => set({ opacity: v })} />
        {CUTS.map(([key, text]) => (
          <Cut key={key} label={text} on={cuts[key]} at={spot[key]}
            onToggle={(v) => setCuts({ ...cuts, [key]: v })} onSlide={(v) => setSpot({ ...spot, [key]: v })} />
        ))}
      </Row>
      <Row>
        <Pick label="camera" value={q.camera} options={[['eye', 'perspective'], ['iso', 'isometric']]} onChange={(v) => set({ camera: v })} />
        <span className="tabs">
          <Btn onClick={() => live.current.view(1, 1, 1)}>corner</Btn>
          <Btn onClick={() => live.current.view(1, 1, 0)}>edge</Btn>
          <Btn onClick={() => live.current.view(0, 0, 1)}>face</Btn>
        </span>
        <Check label="spin" checked={spin} onChange={turn} />
        <Ramp value={look} onChange={(patch) => setLook({ ...look, ...patch })} />
        <Cropper dimension={3} value={q} onChange={set} />
      </Row>
      <div className="arena" style={{ gridTemplateColumns: '2fr 1fr' }}>
        <div className="panel">
          <h2>The solid <span>{view.note}</span></h2>
          <Stage deps={[view, q.opacity, q.camera]} onStage={(st) => {
            live.current = st;
            st.spin = spin ? 0.004 : 0;
            st.project(q.camera);
            if (first.current) {
              first.current = false;
              if (q.camera === 'iso') st.view(1, 1, 1);
            }
            st.clear();
            if (view.mesh) st.add(faces(view.mesh, ink.blue, q.opacity));
            for (const { pixels, frame } of view.planes) st.add(plane(pixels, frame));
          }} />
        </div>
        <div className="panel">
          <h2>The cut <span>{view.cutNote}</span></h2>
          {view.cut ? <Pixels data={view.cut} /> : <canvas className="sheet" />}
        </div>
      </div>
      <Stats>
        <Stat label="name">{solid.name}</Stat>
        <Stat label="layers">{solid.layers}</Stat>
        <Stat label="voxels">{solid.voxels}</Stat>
        <Stat label="at or above">{view.count}</Stat>
        <Stat label="faces">{view.faces}</Stat>
        <Stat label="field">{solid.stats && `${solid.stats.min} to ${solid.stats.max}`}</Stat>
      </Stats>
      <Note error={solid.error ?? view.error} />
    </Page>
  );
}

mount(<App />);
