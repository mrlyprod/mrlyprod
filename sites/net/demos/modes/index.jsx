import { useMemo } from 'react';
import { ready, ink } from '../../lib/mrly.js';
import { mount, Page, Group, Pick, Slider, Stats, Stat, Note } from '../../lib/app.jsx';
import { Grid, Pixels } from '../../lib/draw.jsx';
import { useQuery } from '../../lib/query.js';
import { useSeeds, seeded, Picker, Ramp } from '../../lib/select.jsx';

const m = await ready();
const SIDES = [2, 3, 5];
const DEEPEST = 5;
const BUDGET = 65536;

const cap = (side) => Math.min(DEEPEST, m.level_cap(side, 2, BUDGET));
const wrap = (value, span) => ((value % span) + span) % span;

function App() {
  const s = useSeeds();
  const [pick, set] = useQuery({
    code: seeded(s, 2, 3, '495'), side: 3, base: 3, level: 4,
    t1: 0, t2: 0, bar: 10, ramp: 'heat', levels: 32, invert: false,
  });

  const code = pick.code.trim();
  const level = Math.max(1, Math.min(pick.level, cap(pick.side)));
  const span = pick.side ** level;
  const t1 = wrap(pick.t1, span);
  const t2 = wrap(pick.t2, span);

  const built = useMemo(() => {
    try {
      return {
        grid: m.two_grid(code, pick.side, level, 0, pick.base),
        field: m.modes_field(code, pick.side, level, pick.base),
        digits: m.modes_digits(code, pick.side, pick.base),
        name: m.name_of(code, 2, pick.base),
        error: null,
      };
    } catch (error) {
      return { grid: null, field: null, digits: 0, name: '', error };
    }
  }, [code, pick.side, level, pick.base]);

  const read = useMemo(() => built.field && Float32Array.from(built.field, (v) => v ** (1 / level)), [built.field, level]);
  const heat = useMemo(() => read && m.paint_span(read, span, 0, 1, pick.ramp, pick.levels, pick.invert), [read, pick.ramp, pick.levels, pick.invert]);
  const wave = useMemo(() => {
    if (built.error) return null;
    return m.paint_span(m.modes_pattern(code, pick.side, level, pick.base, t1, t2), span, -1, 1, 'diverge', pick.levels, false);
  }, [built.field, t1, t2, pick.levels]);
  const value = useMemo(() => (built.error ? null : m.modes_value(code, pick.side, level, pick.base, t1, t2)), [built.field, t1, t2]);
  const large = useMemo(() => (built.error ? 0 : m.modes_large(code, pick.side, level, pick.base, pick.bar / 100)), [built.field, pick.bar]);

  const mass = built.digits ** level;
  const seek = (event) => {
    const box = event.currentTarget.getBoundingClientRect();
    const col = Math.floor(((event.clientX - box.left) / box.width) * span);
    const row = Math.floor(((event.clientY - box.top) / box.height) * span);
    set({ t1: Math.max(0, Math.min(span - 1, row)), t2: Math.max(0, Math.min(span - 1, col)) });
  };

  const controls = (
    <>
      <Group name="Design">
        <Picker dimension={2} bases={[3, 2]} code={pick.code} base={pick.base} seeds={s} onChange={set} />
        <Pick label="side" value={pick.side} options={SIDES.map((v) => [v, v])} onChange={(v) => set({ side: +v, level: Math.min(level, cap(+v)) })} />
        <Slider label="level" value={level} min={1} max={cap(pick.side)} show={`${level}, ${span} by ${span}`} onChange={(v) => set({ level: v, t1: wrap(t1, pick.side ** v), t2: wrap(t2, pick.side ** v) })} />
      </Group>
      <Group name="Frequency">
        <Slider label="t1" value={t1} min={0} max={span - 1} onChange={(v) => set({ t1: v })} />
        <Slider label="t2" value={t2} min={0} max={span - 1} onChange={(v) => set({ t2: v })} />
      </Group>
      <Group name="Large values">
        <Slider label="threshold" value={pick.bar} min={0} max={100} show={`${pick.bar}% of k^L, ${large} frequencies`} onChange={(v) => set({ bar: v })} />
      </Group>
      <Group name="Colour">
        <Ramp value={pick} onChange={set} />
      </Group>
    </>
  );

  return (
    <Page crumb="modes" title="The modes of a design mask"
      sub={<>Lay a design's level-<code>L</code> stencil over every point of a <code>q^L</code> by <code>q^L</code> torus and add up what it covers. That operator has one family of modes, the waves <code>e(&lt;t, x&gt; / q^L)</code>, and each one is stretched by a single number. Pick a frequency and watch its wave; the middle panel is the whole field of those numbers at once.</>}
      controls={controls}
      foot={<>The design is the picker's plane code at side <code>q</code> and its residue base; the filled cells of its level-one tile are the digit set <code>F</code>, and the level-<code>L</code> stencil is every sum of <code>L</code> of them scaled by the powers of <code>q</code>. The eigenvalue is a product over the digits, so the field <code>|lambda|</code> is <code>L</code> rescaled copies of one small transform multiplied together, which is why it repeats at every scale like the design itself. The middle panel reads the field at its per-level root, <code>|lambda|^(1/L) / k</code>, the average size of one factor; the printed numbers are the raw ones. The same design stacked over its own scales is <a href="../moire">moire</a>, turned on itself <a href="../radial">radial</a>, and joined into a network whose Laplacian has its own spectrum on <a href="../spectra">spectra</a>; the same stencil run as a neighbourhood is <a href="../mrlylife">mrlylife</a>. Every eigenvalue, every count and every wave comes out of the crates through wasm; the page only draws.</>}>
      <p><span className="chip proved">Proved</span> On the torus <code>(Z/q^L)^2</code> the mask operator <code>(A x)(u) = sum over s in S_L of x(u + s)</code> holds every character <code>e(&lt;t, x&gt; / q^L)</code> fixed in direction, with eigenvalue <code>lambda(t) = prod over j &lt; L of hat F(q^j t / q^L)</code> where <code>hat F(y) = sum over v in F of e(&lt;v, y&gt;)</code>, so <code>lambda(0) = k^L</code>.</p>
      <div className="arena">
        <div className="panel">
          <h2>The mask <span>{`level ${level}, ${mass} cells of ${span * span}`}</span></h2>
          {built.grid && <Grid grid={built.grid} on={ink.gold} role="img" aria-label="The design mask" />}
        </div>
        <div className="panel">
          <h2>The eigenvalue field <span>{`|lambda|^(1/${level}) / k, click to pick t`}</span></h2>
          {heat && <Pixels data={heat} onPointerDown={seek} role="img" aria-label="The eigenvalue field on the frequency torus" />}
        </div>
        <div className="panel">
          <h2>The mode <span>{`t = (${t1}, ${t2})`}</span></h2>
          {wave && <Pixels data={wave} role="img" aria-label="The real mode of the chosen frequency" />}
        </div>
      </div>
      <Stats>
        <Stat label="design">{built.name}</Stat>
        <Stat label="side q">{pick.side}</Stat>
        <Stat label="base">{pick.base}</Stat>
        <Stat label="digits k">{built.digits}</Stat>
        <Stat label="mass k^L">{mass}</Stat>
        <Stat label="torus">{`${span} by ${span}`}</Stat>
      </Stats>
      <Stats>
        <Stat label="lambda re">{value ? value[0].toFixed(6) : ''}</Stat>
        <Stat label="lambda im">{value ? value[1].toFixed(6) : ''}</Stat>
        <Stat label="|lambda| / k^L">{value ? value[2].toFixed(6) : ''}</Stat>
        <Stat label="large values">{`${large} at ${pick.bar}% of k^L`}</Stat>
        <Stat label="share">{`${((large / (span * span)) * 100).toFixed(3)}%`}</Stat>
      </Stats>
      <Note error={built.error} />
    </Page>
  );
}

mount(<App />);
