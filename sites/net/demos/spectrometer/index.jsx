import { useMemo } from 'react';
import { ready, ink, role } from '../../lib/mrly.js';
import { mount, Page, Row, Pick, Check, Btn, Stats, Stat, Note, Group } from '../../lib/app.jsx';
import { Markup, Sketch } from '../../lib/draw.jsx';
import { useQuery } from '../../lib/query.js';
import { useSeeds, seeded, Picker } from '../../lib/select.jsx';
import { board, bars, line, axis, tag } from '../../lib/chart.js';

const m = await ready();
const TOP = 16;
const CORNERS = [0, 1, 2, 3, 4, 5, 6, 7, 8];
const EIGHTHS = [[-4, '-1/2'], [-3, '-3/8'], [-2, '-1/4'], [-1, '-1/8'], [0, '0'], [1, '1/8'], [2, '1/4'], [3, '3/8'], [4, '1/2']];
const RULER = Array.from({ length: 9 }, (_, step) => step / 8);
const LEVELS = ['level 0', 'level 1', 'level 2', 'level 3'];

function App() {
  const s = useSeeds();
  const [pick, set] = useQuery({ code: seeded(s, 3, 2, '23'), k: 8, mystery: false, mseed: 1, corners: 4, top: 0, shown: false });

  const k = Math.min(TOP, Math.max(1, pick.k));
  const sealed = pick.mystery && !pick.shown;
  const code = pick.mystery ? m.random_code(3, 2, pick.mseed) : pick.code.trim();

  const built = useMemo(() => {
    try {
      return {
        read: JSON.parse(m.walsh_spectrum(code, TOP)),
        series: JSON.parse(m.slice_series(code, TOP)),
        name: m.name_of(code, 3, 2),
        error: null,
      };
    } catch (error) {
      return { read: null, series: null, name: '', error };
    }
  }, [code]);

  const read = built.read;
  const row = read?.law[k - 1];
  const counted = built.series?.[k - 1].fills;
  const art = useMemo(() => (row ? m.hex_svg(code, row.n, 1, 2, 'cut', Math.max(1, Math.round(320 / row.n))) : ''), [code, row]);

  const law = (canvas) => {
    if (!read) return;
    const b = board(canvas, 260);
    const rows = read.law, count = rows.length;
    const at = (i) => (i + 0.5) / count;
    for (const step of RULER) line(b, [[0, step], [1, step]], ink.line, { width: 1 });
    if (!sealed) line(b, [[0, read.background], [1, read.background]], ink.dim, { width: 1, dash: [5, 5] });
    line(b, built.series.map((each, i) => [at(i), each.fills / rows[i].triangles]), ink.gold, { width: 1, dash: [1, 5], dots: 3.5 });
    line(b, rows.map((each, i) => [at(i), each.ink]), ink.blue, { width: 1.6, dots: 2 });
    axis(b, rows.map((each, i) => [at(i), each.n]));
    const next = tag(b, 'closed form read off the spectrum', ink.blue);
    tag(b, 'ink counted on the mesh', ink.gold, 'left', next + 16);
  };

  const spectrum = (canvas) => {
    if (!read) return;
    const b = board(canvas, 200, { bottom: 96 });
    const values = read.levels.map((level) => level.sigma);
    bars(b, values, { peak: Math.max(...values.map(Math.abs), 1e-12), color: (i) => role[i], inset: 14 });
    axis(b, values.map((value, i) => [(i + 0.5) / values.length, LEVELS[i]]), { wall: true });
  };

  const controls = (
    <>
      <Group name="Design">
        <span className="set" hidden={pick.mystery}>
          <Picker dimension={3} code={pick.code} seeds={s} onChange={set} />
        </span>
        <label>side <input type="range" min={1} max={TOP} value={k} onChange={(e) => set({ k: +e.target.value })} /><span className="num">{`k ${k}, n ${row?.n ?? ''}`}</span></label>
      </Group>
      <Group name="Mystery">
        <Check label="mystery" checked={pick.mystery} onChange={(value) => set({ mystery: value, shown: false })} />
        <span className="set" hidden={!pick.mystery}>
          <Pick label="filled corners" value={pick.corners} options={CORNERS} onChange={(value) => set({ corners: +value })} />
          <Pick label="top level" value={pick.top} options={EIGHTHS} onChange={(value) => set({ top: +value })} />
          <Btn primary={!pick.shown} onClick={() => set({ shown: !pick.shown })}>{pick.shown ? 'Hide again' : 'Reveal'}</Btn>
          <Btn onClick={() => set({ mseed: pick.mseed + 1, shown: false })}>New mystery</Btn>
        </span>
      </Group>
    </>
  );

  return (
    <Page crumb="spectrometer" title="Point the slice at a sponge and it reads the recipe back" controls={controls}
      sub={<>A cube design is eight yes-or-no answers about corner parities. Cut its cube down the main diagonal and the hexagon comes back part inked, part blank, and that one fraction is an exact closed form in the design's Walsh spectrum, level by level: a steady background, a two-step blink, and two corrections that die as <code>1/n</code> and <code>1/n^2</code>. No fit, no error term. Turn the mystery on and the code is hidden - read the recipe off the curve, then reveal it.</>}
      foot={<>The spectrum is the crate's Walsh-Hadamard transform of the design's eight corners, the four bars are its level sums <code>Sigma_0</code> to <code>Sigma_3</code>, and the blue curve is the ink law evaluated over the integers in Rust and handed here as an exact numerator over <code>96n^2</code>. The gold dots are the fills the crate counts triangle by triangle on the real mesh, over the <code>6n^2</code> triangles of the hexagon. The two never part: the law predicts the count itself, not an approximation to it. The same hexagon, its mesh census, its pieces and its holes are <a href="../slices">the slices</a> page; the research note this grew from is <a href="/research/slices/">slices</a>, and the theorem, its proof and its checks are the shelf paper <a href="/papers/walsh-spectrometer/">the Walsh spectrometer</a>.</>}>
      <div className="arena">
        <div className="panel">
          <h2>The slice <span>{`side ${row?.n ?? ''}, ${row?.triangles ?? ''} triangles`}</span></h2>
          <Markup svg={art} role="img" aria-label="The slice" />
        </div>
        <div className="panel">
          <h2>The spectrum <span>{sealed ? 'sealed' : 'level sums'}</span></h2>
          <Sketch draw={spectrum} deps={[read, sealed]} className="bars" role="img" aria-label="The spectrum" hidden={sealed} />
          <p className="sub" hidden={!sealed}>The readout is sealed, and the curve below still carries it. The level the blink straddles is <code>Sigma_0</code>, one eighth for every filled corner; the gap between the two combs is <code>Sigma_3</code>, the top level, positive when the higher comb sits at the sides 1, 5, 9.</p>
          {!sealed && <Stats>{read?.coefficients.map((part) => <Stat key={part.mask} label={`F ${part.mask.toString(2).padStart(3, '0')}`}>{part.value.toFixed(3)}</Stat>)}</Stats>}
        </div>
      </div>
      <Sketch draw={law} deps={[read, sealed]} className="bars" role="img" aria-label="The ink law against the counted ink" />
      <Stats>
        <Stat label="name">{sealed ? 'sealed' : built.name}</Stat>
        <Stat label="sign s">{row?.s}</Stat>
        <Stat label="closed form">{row?.fills}</Stat>
        <Stat label="counted">{counted}</Stat>
        <Stat label="verdict">{row ? (row.fills === counted ? 'exact' : 'broken') : ''}</Stat>
        <Stat label="ink">{row?.ink.toFixed(6)}</Stat>
        <Stat label="exact ink">{row ? `${row.numerator}/${row.denominator}` : ''}</Stat>
      </Stats>
      {!sealed && read && (
        <Stats>
          {read.levels.map((level) => <Stat key={level.level} label={`Sigma ${level.level}`}>{`${level.eighths}/8`}</Stat>)}
          <Stat label="background">{read.background.toFixed(4)}</Stat>
          <Stat label="blink">{read.blink.toFixed(4)}</Stat>
        </Stats>
      )}
      {pick.mystery && !sealed && read && (
        <Stats>
          <Stat label="corners">{`${read.corners}, you said ${pick.corners}`}</Stat>
          <Stat label="corners read">{pick.corners === read.corners ? 'right' : 'missed'}</Stat>
          <Stat label="top level">{`${read.levels[3].eighths}/8, you said ${pick.top}/8`}</Stat>
          <Stat label="top level read">{pick.top === read.levels[3].eighths ? 'right' : 'missed'}</Stat>
        </Stats>
      )}
      <Note error={built.error} />
    </Page>
  );
}

mount(<App />);
