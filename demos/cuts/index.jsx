import { useRef, useState } from 'react';
import { ready, ink } from '../lib/mrly.js';
import { mount, Page, Row, Pick, Slider, Check, Stats, Stat, Note } from '../lib/app.jsx';
import { Markup, Sketch } from '../lib/draw.jsx';
import { useQuery } from '../lib/query.js';
import { useSeeds, seeded, Picker } from '../lib/select.jsx';
import { board, bars, axis } from '../lib/chart.js';

const m = await ready();
const PAD = 12;

function App() {
  const s = useSeeds();
  const [pick, set] = useQuery({ code: seeded(s, 3, 2, '126'), level: 5 });
  const [height, setHeight] = useState(null);
  const [both, setBoth] = useState(false);
  const [view, setView] = useState('points');
  const shown = useRef(null);

  const top = m.level_cap(2, 1, view === 'section' ? 64 : 512);
  const level = Math.min(pick.level, top);
  const code = pick.code.trim();

  let error = null, art = null, drawn = '';
  try {
    const cut = JSON.parse(m.diagonal_profile(code, 2, level, 2));
    const [low, high] = cut.support;
    const at = Math.min(high, Math.max(low, height ?? cut.central[0]));
    const heights = both ? cut.central : [at];
    shown.current = {
      cut, low, high, at, heights,
      name: m.name_of(code, 3, 2),
      here: heights.map((h) => m.diagonal_count(code, 2, level, 2, h)).join(' and '),
      digits: heights.map((h) => m.diagonal_digits(code, 2, level, 2, h)).join(' and '),
    };
    const svg = view === 'section'
      ? m.hex_svg(code, 2, level, 2, 'cut', Math.max(1, Math.round(256 / 2 ** level)))
      : m.diagonal_svg(code, 2, level, 2, heights, Math.max(1, Math.round(512 / 2 ** level)));
    if (svg.length > 4000000) throw new Error('that drawing is larger than this page serves; lower the level.');
    art = svg;
    drawn = view === 'section' ? '-' : m.diagonal_total(code, 2, level, 2, heights);
  } catch (fault) {
    error = fault;
  }

  const chart = (canvas) => {
    const v = shown.current;
    if (!v) return;
    const b = board(canvas, 180, { pad: PAD, top: 12, bottom: 20 });
    const counts = v.cut.counts.map(Number);
    bars(b, counts, { color: (i) => (v.heights.includes(v.low + i) ? ink.gold : ink.blue) });
    axis(b, [[0, v.low], [1, v.low + counts.length - 1]]);
  };

  const onSeek = (frac) => {
    const v = shown.current;
    if (both || !v) return;
    setHeight(Math.min(v.high, Math.max(v.low, Math.round(v.low + frac * (v.high - v.low)))));
  };

  const onView = (value) => {
    const cap = m.level_cap(2, 1, value === 'section' ? 64 : 512);
    setView(value);
    if (pick.level > cap) {
      set({ level: cap });
      setHeight(null);
    }
  };

  const v = shown.current;

  return (
    <Page crumb="cuts" title="Every diagonal cut is the same size"
      sub={<>A plane <code>x + y + z = s</code> slides through the level-L solid of <code>mrly_bang_d3_126</code> and meets exactly 3^L cells at every height it meets at all. The binary digits of the height say which corners each scale may use, so every cut is a Sierpinski gasket, and the two central heights together fall into six of them tiling a hexagon. Drag the bar chart to move the plane.</>}
      foot={<>The profile is the coefficient list of the digit polynomial, so a height is counted without building a single cell; the points are enumerated on the plane itself and projected down the <code>(1,1,1)</code> axis in Rust, one circle per cell, coloured by height and by top-scale corner. The section view is the crate's own triangular mesh through the same solid.</>}>
      <Row>
        <Picker dimension={3} code={pick.code} seeds={s} onChange={(patch) => { set(patch); setHeight(null); }} />
        <Slider label="level" value={level} min={1} max={top} onChange={(value) => { set({ level: value }); setHeight(null); }} />
        <label>height <input type="range" min={v ? v.low : 0} max={v ? v.high : 1} value={v ? v.at : 0} disabled={both} onChange={(e) => setHeight(+e.target.value)} /><span className="num">{v?.heights.join(' and ')}</span></label>
        <Check label="both central heights" checked={both} onChange={setBoth} />
        <Pick label="view" value={view} options={['points', 'section']} onChange={onView} />
      </Row>
      <Sketch draw={chart} deps={[v]} onSeek={onSeek} pad={PAD} className="bars" />
      <Stats>
        <Stat label="name">{v?.name}</Stat>
        <Stat label="side">{v?.cut.side}</Stat>
        <Stat label="support">{v && `[${v.low}, ${v.high}]`}</Stat>
        <Stat label="non-empty">{v && `${v.cut.nonempty} of ${v.cut.heights}`}</Stat>
        <Stat label="this slice">{v?.here}</Stat>
        <Stat label="min">{v?.cut.min}</Stat>
        <Stat label="max">{v?.cut.max}</Stat>
        <Stat label="constant">{v && (v.cut.constant ? 'yes' : 'no')}</Stat>
        <Stat label="offset in binary">{v?.digits}</Stat>
        <Stat label="points drawn">{drawn}</Stat>
      </Stats>
      <Markup svg={art ?? ''} />
      <Note error={error} />
    </Page>
  );
}

mount(<App />);
