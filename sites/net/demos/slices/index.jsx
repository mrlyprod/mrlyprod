import { useRef, useState } from 'react';
import { ready, ink } from '../../lib/mrly.js';
import { mount, Page, Row, Pick, Stats, Stat, Note, Group } from '../../lib/app.jsx';
import { Markup, Sketch } from '../../lib/draw.jsx';
import { useQuery } from '../../lib/query.js';
import { useSeeds, seeded, Picker } from '../../lib/select.jsx';
import { board, bars, line, axis, tag } from '../../lib/chart.js';

const m = await ready();
const TOP = 16;
const TALLY = [
  ['boundary', 'boundary'], ['edges', 'edges'], ['interior', 'interior'], ['vertices', 'vertices'], ['euler', 'euler'],
  ['fills', 'filled'], ['voids', 'empty'], ['components', 'pieces'], ['holes', 'holes'], ['giant', 'largest piece'],
];

const capOf = (fractal) => (fractal ? m.level_cap(fractal, 1, 100) : m.level_cap(1, 1, 1));

function App() {
  const s = useSeeds();
  const [pick, set] = useQuery({ code: seeded(s, 3, 2, '23'), k: 2, level: 1 });
  const [fractal, setFractal] = useState(0);
  const shown = useRef(null);

  const lid = capOf(fractal);
  const level = Math.min(pick.level, lid);
  const k = Math.min(TOP, Math.max(1, pick.k));
  const code = pick.code.trim();

  let error = null, art = null, split = null;
  try {
    const rows = JSON.parse(m.slice_series(code, TOP));
    const number = fractal || rows[k - 1].n;
    const tally = JSON.parse(m.slice_census(code, number, level, 2));
    shown.current = {
      rows, tally, here: fractal ? 0 : k,
      name: m.name_of(code, 3, 2),
      reading: fractal ? `tile ${number}` : `k ${k}, n ${number}`,
    };
    if (level === 1) split = JSON.parse(m.slice_partition(number));
    const svg = m.hex_svg(code, number, level, 2, 'cut', Math.max(1, Math.round(360 / tally.side)));
    if (svg.length > 4000000) throw new Error('that drawing is larger than this page serves; lower the level.');
    art = svg;
  } catch (fault) {
    error = fault;
  }

  const chart = (canvas) => {
    const view = shown.current;
    if (!view) return;
    const b = board(canvas, 220);
    const rows = view.rows, n = rows.length;
    const peak = rows.reduce((a, r) => Math.max(a, r.fills), 1);
    const crest = rows.reduce((a, r) => Math.max(a, r.components, r.holes), 1);
    bars(b, rows.map((r) => r.fills), { peak, color: (i) => (rows[i].k === view.here ? ink.gold : ink.blue) });
    for (const [key, color] of [['components', ink.green], ['holes', ink.pink]]) {
      line(b, rows.map((r, i) => [(i + 0.5) / n, r[key] / crest]), color, { dots: 2.5 });
    }
    axis(b, rows.map((r, i) => [(i + 0.5) / n, r.k]));
    const next = tag(b, `filled triangles, peak ${peak}`, ink.blue);
    tag(b, `holes, peak ${crest}`, ink.pink, 'left', tag(b, 'pieces', ink.green, 'left', next + 16) + 16);
  };

  const toFractal = (value) => {
    setFractal(value);
    if (pick.level > capOf(value)) set({ level: capOf(value) });
  };

  const onSeek = (frac) => {
    toFractal(0);
    set({ k: Math.min(TOP, Math.max(1, Math.floor(frac * TOP) + 1)) });
  };

  const view = shown.current;

  const controls = (
    <>
      <Group name="Design">
        <Picker dimension={3} code={pick.code} seeds={s} onChange={set} />
      </Group>
      <Group name="Section">
        <label>side <input type="range" min={1} max={TOP} value={k} disabled={fractal !== 0} onChange={(e) => set({ k: +e.target.value })} /><span className="num">{view?.reading}</span></label>
        <Pick label="fractal" value={fractal} options={[[0, 'off, odd side'], [3, 'base 3 tile'], [5, 'base 5 tile']]} onChange={(value) => toFractal(+value)} />
        <label>level <input type="range" min={1} max={lid} value={level} disabled={fractal === 0} onChange={(e) => set({ level: +e.target.value })} /><span className="num">{level}</span></label>
      </Group>
    </>
  );

  return (
    <Page crumb="slices" title="The middle plane of a cube is a hexagon of triangles" controls={controls}
      sub={<>Cut the cube of odd side <code>n = 2k-1</code> through its centre, square to the main diagonal, and the section is a regular hexagon tiled by <code>6n^2</code> unit triangles. A design's parity rule fills some of them: carpet and net split the hexagon between them with nothing left over, and the carpet's own section alternates as <code>k</code> grows, falling into many separate pieces at odd <code>k</code> and closing into one piece pierced by holes at even <code>k</code>. Drag the chart to move the side.</>}
      foot={<>The mesh is the crate's triangular section of the cube it builds from the code, and every count is read off that mesh; the closed form beside the triangle count is the polynomial in <code>k</code>, evaluated in Rust and never fitted here. Pieces come from the adjacency network of filled triangles and holes from the Euler number of the filled sub-mesh, so the alternation in the chart is counted, not drawn. The mesh census, the closed forms beside it and which designs pierce the hexagon with holes are in <a href="/research/slices/">the slices note</a>.</>}>
      <Sketch draw={chart} deps={[view]} onSeek={onSeek} className="bars" role="img" aria-label="Filled triangles, pieces and holes by side, drag to move the side" />
      <Stats>
        <Stat label="name">{view?.name}</Stat>
        <Stat label="side">{view?.tally.side}</Stat>
        <Stat label="triangles">{view?.tally.triangles}</Stat>
        <Stat label="closed form">{view?.tally.closed.triangles}</Stat>
        {TALLY.map(([key, label]) => <Stat key={key} label={label}>{view?.tally[key]}</Stat>)}
      </Stats>
      <Stats>
        {split && <><Stat label="carpet">{split.carpet}</Stat><Stat label="net">{split.net}</Stat><Stat label="together">{split.together}</Stat><Stat label="hexagon">{split.hexagon}</Stat><Stat label="partition">{split.exact ? 'exact' : 'broken'}</Stat></>}
      </Stats>
      <Markup svg={art ?? ''} role="img" aria-label="The middle slice" />
      <Note error={error} />
    </Page>
  );
}

mount(<App />);
