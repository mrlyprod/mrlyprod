import { useEffect, useMemo, useRef } from 'react';
import { ready, ink } from '../lib/mrly.js';
import { mount, Page, Row, Pick, Slider, Btn, Stats, Stat, Note } from '../lib/app.jsx';
import { Grid, Markup, Sketch } from '../lib/draw.jsx';
import { useQuery } from '../lib/query.js';
import { useSeeds, seeded, Picker } from '../lib/select.jsx';
import { board, line, axis, tag } from '../lib/chart.js';

const m = await ready();
const SERVE = 1100;
const OBJECTS = [['triangle', 'Sierpinski triangle'], ['code', 'flat code'], ['carpet', 'carpet slice'], ['solid', 'solid slice']];
const FIRST = { object: 'triangle', code: '7', number: 3, side: 5, level: 4, operator: 1, window: 10 };

const first = (seeds) => (seeds.get() ? { ...FIRST, code: seeded(seeds, 2, 2, '7'), object: 'code' } : FIRST);

function plan(look, number) {
  if (look.object === 'solid') return { kind: 'slice', code: '255', number, cap: 1 };
  if (look.object === 'carpet') return { kind: 'slice', code: '23', number: 3, cap: 2 };
  if (look.object === 'triangle') return { kind: 'flat', code: '7', number: 2, cap: m.fill_cap('7', 2, 2, 2, SERVE) };
  const code = look.code.trim();
  return { kind: 'flat', code, number: look.number, cap: m.fill_cap(code, look.number, 2, 2, SERVE) };
}

function Lock({ off, children }) {
  return <fieldset className="set" disabled={off}>{children}</fieldset>;
}

function App() {
  const s = useSeeds();
  const [look, set] = useQuery(first(s));
  const kept = useRef({ spec: null, level: look.level, number: 0, data: null, grid: null, svg: null, what: '' });

  const view = useMemo(() => {
    try {
      const number = JSON.parse(m.slice_series('255', look.side))[look.side - 1].n;
      const spec = plan(look, number);
      const level = Math.min(Math.max(1, look.level), spec.cap);
      const data = JSON.parse(m.spectrum(spec.kind, spec.code, spec.number, level, look.operator === 1, look.window / 100));
      const slice = spec.kind === 'slice';
      const svg = slice ? m.hex_svg(spec.code, spec.number, level, 2, 'cut', Math.max(2, Math.round(300 / spec.number ** level))) : null;
      const grid = slice ? null : m.two_grid(spec.code, spec.number, level, 0, 2);
      const what = `${m.name_of(spec.code, slice ? 3 : 2, 2)}, level ${level}`;
      kept.current = { spec, level, number, data, grid, svg, what };
      return { ...kept.current, error: null };
    } catch (error) {
      return { ...kept.current, error };
    }
  }, [look.object, look.code, look.number, look.side, look.level, look.operator, look.window]);

  useEffect(() => {
    if (view.level !== look.level) set({ level: view.level });
  }, [view.level]);

  const chart = (canvas) => {
    const data = view.data;
    if (!data) return;
    const b = board(canvas, 300, { left: 52, right: 14, top: 14, bottom: 26 });
    const steps = data.stair;
    if (steps.length < 2) return;
    const xs = steps.map((p) => Math.log(p[0]));
    const ys = steps.map((p) => Math.log(p[1]));
    const x0 = xs[0], x1 = xs.at(-1), y0 = ys[0];
    const fx = (x) => (x1 === x0 ? 0.5 : (x - x0) / (x1 - x0));
    const fy = (y) => (y0 === 0 ? 0.5 : (y - y0) / -y0);
    if (data.fitted) {
      b.ctx.fillStyle = '#151b22';
      b.ctx.fillRect(b.x(0), b.roof, b.x(fx(xs[data.fitted - 1])) - b.x(0), b.tall);
    }
    axis(b, [[0, steps[0][0].toExponential(2)], [1, steps.at(-1)[0].toFixed(4)]], { wall: true });
    const stair = [[0, fy(ys[0])]];
    for (let i = 1; i < steps.length; i++) stair.push([fx(xs[i]), fy(ys[i - 1])], [fx(xs[i]), fy(ys[i])]);
    line(b, stair, ink.blue, { width: 1.6 });
    if (data.fit) {
      const [intercept, slope] = data.fit;
      const seg = (a, c) => [[fx(a), fy(intercept + slope * a)], [fx(c), fy(intercept + slope * c)]];
      line(b, seg(x0, x1), ink.gold, { width: 1.2, dash: [3, 4] });
      line(b, seg(x0, xs[data.fitted - 1]), ink.gold, { width: 2.2 });
    }
    tag(b, '1', ink.dim, 'right', b.x(0) - 6, b.y(fy(0)) + 4);
    tag(b, steps[0][1].toFixed(4), ink.dim, 'right', b.x(0) - 6, b.y(fy(y0)) + 4);
  };

  const data = view.data, cap = view.spec ? view.spec.cap : 1, level = view.level;
  const legend = !view.error && data;

  return (
    <Page crumb="spectra" title="A third of the triangle's spectrum sits on a single number"
      sub={<>Build the graph on the filled cells of a design, take its normalised Laplacian <code>I - D^-1/2 A D^-1/2</code>, and diagonalise. On the Sierpinski triangle the spectrum is extraordinarily degenerate: the eigenvalue 1 alone carries a third of it, and a cascade of families sits beneath, the first at <code>1 -/+ sqrt(30)/6</code>. The same operator on the diagonal section of a cube answers a different question - the low end of its integrated density of states has a slope, and twice that slope is the random-walk spectral dimension <code>d_s</code>. One staircase, two readings: the jumps are the degeneracy, the low corner is the dimension.</>}
      foot={<>The graph, the Laplacian, the eigenvalues, the clustering and the fit all come out of the crates: a dense real symmetric eigensolver written from scratch - Householder tridiagonalisation then implicit QL with Wilkinson shifts - returns the spectrum ascending, the clusters split it at consecutive gaps above <code>1e-9</code>, and the low-window least squares in log-log returns an intercept and a slope the page only draws. The slice reading is taken on the giant piece; the whole section's piece count is reported beside it. Anything past the export's node cap is refused rather than served slowly.</>}>
      <Row>
        <Pick label="object" value={look.object} options={OBJECTS} onChange={(v) => set({ object: v })} />
        <Lock off={look.object !== 'code'}>
          <Picker dimension={2} code={look.code} seeds={s} button={false} onChange={(patch) => set({ ...patch, object: 'code' })} />
        </Lock>
        <Btn onClick={() => set({ code: m.random_code(2, 2, s.next()), object: 'code' })}>Randomize</Btn>
        <Lock off={look.object !== 'code'}>
          <Pick label="tile" value={look.number} options={[[2, 2], [3, 3]]} onChange={(v) => set({ number: +v })} />
        </Lock>
        <Lock off={look.object !== 'solid'}>
          <Slider label="side" value={look.side} min={2} max={7} show={`n ${view.number}`} onChange={(v) => set({ side: v })} />
        </Lock>
        <Lock off={look.object === 'solid'}>
          <Slider label="level" value={level} min={1} max={cap} show={`${level} of ${cap}`} onChange={(v) => set({ level: v })} />
        </Lock>
        <Pick label="operator" value={look.operator} options={[[1, 'normalised'], [0, 'combinatorial']]} onChange={(v) => set({ operator: +v })} />
        <Slider label="window" value={look.window} min={1} max={20} show={`${look.window}%`} onChange={(v) => set({ window: v })} />
      </Row>
      <div className="arena">
        <div className="panel">
          <h2>Integrated density of states <span>{`log rank fraction against log eigenvalue, ${look.operator === 1 ? 'normalised' : 'combinatorial'}`}</span></h2>
          <Sketch draw={chart} deps={[view]} className="bars" />
          <Stats>
            {legend && (
              <>
                <span>staircase <b style={{ color: ink.blue }}>{data.distinct} distinct</b></span>
                <span>shaded low window <b>{look.window}%</b></span>
                <span>fitted slope <b style={{ color: ink.gold }}>{data.fit ? data.fit[1].toFixed(4) : 'none'}</b></span>
                <span>d_s = 2 x slope <b>{data.exponent === null ? 'none' : data.exponent.toFixed(3)}</b></span>
              </>
            )}
          </Stats>
        </div>
        <div className="panel">
          <h2>The object <span>{view.what}</span></h2>
          {view.grid ? <Grid grid={view.grid} on={ink.blue} /> : <Markup svg={view.svg ?? ''} />}
          <h2>The eight largest clusters</h2>
          <pre>{legend ? data.top.map(([value, size]) => `${value.toFixed(10).padStart(14)}   x${size}`).join('\n') : ''}</pre>
        </div>
      </div>
      <Stats>
        <Stat label="nodes">{data?.nodes}</Stat>
        <Stat label="edges">{data?.edges}</Stat>
        <Stat label="pieces">{data?.components}</Stat>
        <Stat label="distinct">{data?.distinct}</Stat>
        <Stat label="degenerate classes">{data?.classes}</Stat>
        <Stat label="repeated fraction">{data?.repeated}</Stat>
        <Stat label="mult of 1">{data?.one}</Stat>
        <Stat label="mult of the pair">{data?.pair.join(' and ')}</Stat>
        <Stat label="d_s">{data && (data.exponent === null ? 'none' : data.exponent.toFixed(4))}</Stat>
      </Stats>
      <Note error={view.error} />
    </Page>
  );
}

mount(<App />);
