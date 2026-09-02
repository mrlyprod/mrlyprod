import { useMemo, useRef, useState } from 'react';
import { ready, ink } from '../lib/mrly.js';
import { faces, cubes } from '../lib/stage.js';
import { mount, Page, Row, Pick, Slider, Check, Stats, Stat, Note } from '../lib/app.jsx';
import { Stage } from '../lib/stage.jsx';
import { useSeeds, seeded, Picker, Cropper, cropOf } from '../lib/select.jsx';
import { useQuery } from '../lib/query.js';

const m = await ready();
const VIEWS = [['shell', 'exposed faces'], ['cubes', 'instanced cubes']];

function Num({ label, value, min, max, onChange }) {
  return <label>{label} <input type="number" value={value} min={min} max={max} onChange={(e) => onChange(+e.target.value)} /></label>;
}

function App() {
  const s = useSeeds();
  const [q, set] = useQuery({ code: seeded(s, 3, 2, '23'), base: 2, number: 3, level: 3, crop: '', 'crop-r': 16, 'crop-anti': false });
  const [view, setView] = useState('shell');
  const [opacity, setOpacity] = useState(1);
  const [spin, setSpin] = useState(true);
  const live = useRef(null);
  const top = m.level_cap(q.number, 1, 128);
  const level = Math.min(q.level, top);

  const data = useMemo(() => {
    const out = {};
    try {
      const code = q.code.trim(), c = cropOf(q);
      out.name = m.name_of(code, 3, q.base);
      out.side = m.grid_total(q.number, 1, level);
      out.fills = m.fills(code, q.number, 3, level, q.base);
      out.voids = m.voids(code, q.number, 3, level, q.base);
      out.surface = m.three_surface(code, q.number, level, q.base);
      out.ratio = m.ratio(code, q.number, 3, level, q.base).toFixed(4);
      out.dimension = m.dimension(code, q.number, 3, q.base).toFixed(4);
      if (Number(m.grid_total(q.number, 3, level)) <= 30000) out.tally = JSON.parse(m.three_census(code, q.number, level, q.base));
      if (view === 'shell') {
        if (Number(out.surface) > 400000) throw new Error(`${out.surface} faces is more than this page draws; lower the level.`);
        const mesh = c.active
          ? m.crop_faces(code, q.number, level, q.base, c.shape, c.rnum, c.rden, c.anti, 'touching')
          : m.three_faces(code, q.number, level, q.base);
        if (c.active && mesh[0] / 36 > 400000) throw new Error(`${mesh[0] / 36} faces is more than this page draws; lower the level.`);
        out.mesh = mesh;
      } else {
        if (Number(out.fills) > 250000) throw new Error(`${out.fills} cubes is more than this page draws; lower the level.`);
        out.cells = c.active
          ? m.crop_cells(code, q.number, level, q.base, c.shape, c.rnum, c.rden, c.anti, 'touching')
          : m.three_cells(code, q.number, level, q.base);
      }
    } catch (error) {
      out.error = error;
    }
    return out;
  }, [q.code, q.base, q.number, level, q.crop, q['crop-r'], q['crop-anti'], view]);

  const turn = (on) => {
    setSpin(on);
    if (live.current) live.current.spin = on ? 0.004 : 0;
  };

  return (
    <Page crumb="sponge" title="The sponge"
      sub="A code picks the filled corners of a cube, the cube grows level by level, and the exposed faces come out of Rust already packed for the screen. Drag to orbit, scroll to zoom."
      foot={<>Filled, empty and exposed counts come from closed formulas, so they answer before any cube is built; the mesh is the crate's own list of outward quads, six floats per vertex, and the Euler number is read off the edge graph while the cube is small enough to walk. Where those formulas come from is <a href="https://github.com/mrlyprod/mrlyprod/blob/main/research/core.md">the core</a>.</>}>
      <Row>
        <Picker dimension={3} bases={[2, 3]} code={q.code} base={q.base} seeds={s} onChange={set} />
        <Num label="number" value={q.number} min={2} max={5} onChange={(v) => set({ number: v })} />
        <Slider label="level" value={level} min={1} max={top} onChange={(v) => set({ level: v })} />
        <Pick label="view" value={view} options={VIEWS} onChange={setView} />
        <Slider label="opacity" value={opacity} min={0.05} max={1} step={0.05} onChange={setOpacity} />
        <Check label="spin" checked={spin} onChange={turn} />
        <Cropper dimension={3} value={q} onChange={set} />
      </Row>
      <Stage deps={[data, opacity]} onStage={(st) => {
        live.current = st;
        st.spin = spin ? 0.004 : 0;
        if (data.mesh) st.show(faces(data.mesh, ink.blue, opacity));
        else if (data.cells) st.show(cubes(data.cells, Number(data.side), ink.orange));
        else st.clear();
      }} />
      <Stats>
        <Stat label="name">{data.name}</Stat>
        <Stat label="side">{data.side}</Stat>
        <Stat label="filled">{data.fills}</Stat>
        <Stat label="empty">{data.voids}</Stat>
        <Stat label="exposed faces">{data.surface}</Stat>
        <Stat label="fill ratio">{data.ratio}</Stat>
        <Stat label="dimension">{data.dimension}</Stat>
        {data.tally && <span>vertices <b>{data.tally.vertices}</b> edges <b>{data.tally.edges}</b> faces <b>{data.tally.faces}</b> euler <b>{data.tally.euler}</b></span>}
      </Stats>
      <Note error={data.error} />
    </Page>
  );
}

mount(<App />);
