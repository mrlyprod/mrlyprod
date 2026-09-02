import { useMemo } from 'react';
import { ready } from '../../lib/mrly.js';
import { useQuery } from '../../lib/query.js';
import { mount, Page, Row, Pick, Slider, Check, Text, Btn, Stats, Stat, Note, Group } from '../../lib/app.jsx';
import { Picker, useSeeds } from '../../lib/select.jsx';
import { Pins, Ratios, Differences, Digits, Terms } from '../../lib/series.jsx';

const m = await ready();
const BUDGET = '500000';
const DEPTH = 6;
const HEADS = 3;
const LABELS = '4000';
const DIMS = [1, 2, 3, 4];
const BASES = [2, 3, 4, 5];
const MEASURES = [...m.ledger_measures()];
const OPS = [...m.blend_ops()];
const PAIRED = ['add', 'sub', 'hadamard', 'cauchy'];
const VIEWS = [['pins', 'pin plot'], ['log', 'log plot'], ['ratios', 'ratios'], ['differences', 'difference triangle'], ['digits', 'digit heatmap']];

function listed(dimension, base) {
  try {
    return [...m.ledger_designs(dimension, base)];
  } catch {
    return null;
  }
}

const SPACES = new Map(DIMS.map((d) => [d, BASES.filter((b) => listed(d, b))]));

function gallery(dimension) {
  try {
    m.universe(dimension);
    return true;
  } catch {
    return false;
  }
}

const GALLERY = DIMS.filter(gallery);

function digits(base) {
  return [...new Set([2, 3, 10, base])].sort((a, b) => a - b);
}

function shown(value) {
  return value === null || value === undefined ? 'none' : Number(value).toFixed(6);
}

function View({ row, view, dig, label, name }) {
  if (view === 'ratios') return <Ratios values={row.ratios} start={row.start} label={`${label}, each term against the one before`} role="img" aria-label={`${name}, each term against the one before`} />;
  if (view === 'differences') return <Differences rows={row.differences} label={`${label}, each row the differences of the row above`} role="img" aria-label={`${name}, the difference triangle`} />;
  if (view === 'digits') return <Digits terms={row.terms} start={row.start} base={dig} label={label} role="img" aria-label={`${name}, a heatmap of the digits of every term`} />;
  return <Pins terms={row.terms} start={row.start} log={view === 'log'} label={label} role="img" aria-label={`${name}, every term standing on its index`} />;
}

function Rule({ row }) {
  return (
    <Stats>
      <Stat label="order">{row.order ?? 'none fits'}</Stat>
      <Stat label="rule">{row.recurrence || 'no linear rule fits these terms'}</Stat>
      <Stat label="characteristic">{row.polynomial || 'none'}</Stat>
      <Stat label="largest positive real root">{shown(row.root)}</Stat>
      <Stat label="growth">{shown(row.growth)}</Stat>
      <Stat label="exponent">{shown(row.exponent)}</Stat>
      <Stat label="read from">{row.growth_from}</Stat>
    </Stats>
  );
}

function App() {
  const s = useSeeds();
  const [pick, set] = useQuery({
    code: '7', dimension: 2, base: 2, measure: 'surface', axis: 'level', count: 12,
    view: 'pins', dig: 2, on: false, op: 'hadamard', arg: 2, scode: '3', smeasure: 'fills',
  });

  const move = (patch) => {
    const dimension = patch.dimension ?? pick.dimension;
    const wanted = patch.base ?? pick.base;
    const bases = SPACES.get(dimension) ?? [];
    const base = bases.includes(wanted) ? wanted : bases[0] ?? wanted;
    const list = listed(dimension, base) ?? [];
    const keep = (code) => {
      try {
        m.name_of(code, dimension, base);
        return code;
      } catch {
        return list.at(-1) ?? code;
      }
    };
    s.drop();
    set({ ...patch, base, code: keep(pick.code.trim()), scode: keep(pick.scode) });
  };

  const read = useMemo(() => {
    try {
      return { row: JSON.parse(m.blend_series(pick.code.trim(), pick.dimension, pick.base, pick.measure, pick.axis, pick.count, BUDGET, DEPTH)), error: null };
    } catch (error) {
      return { row: null, error };
    }
  }, [pick.code, pick.dimension, pick.base, pick.measure, pick.axis, pick.count]);

  const family = useMemo(() => {
    try {
      return JSON.parse(m.blend_family(pick.dimension, pick.base, pick.smeasure, pick.axis, HEADS, LABELS));
    } catch {
      return [];
    }
  }, [pick.dimension, pick.base, pick.smeasure, pick.axis]);

  const blend = useMemo(() => {
    if (!pick.on || !read.row) return { row: null, mate: null, error: null };
    try {
      const mate = JSON.parse(m.blend_series(pick.scode.trim(), pick.dimension, pick.base, pick.smeasure, pick.axis, pick.count, BUDGET, DEPTH));
      return { row: JSON.parse(m.blend_mix(read.row.terms, mate.terms, pick.op, pick.arg, DEPTH)), mate, error: null };
    } catch (error) {
      return { row: null, mate: null, error };
    }
  }, [pick.on, read.row, pick.scode, pick.smeasure, pick.op, pick.arg]);

  const row = read.row;
  const label = row ? `${row.axis === 'level' ? 'level L' : 'side k'} from ${row.start}` : '';
  const mates = family.map((one) => [one.code, `${one.code} · ${one.terms.join(', ') || 'no terms'}`]);
  const options = family.some((one) => one.code === pick.scode) ? mates : [[pick.scode, pick.scode], ...mates];

  const controls = (
    <>
      <Group name="The sequence">
        {GALLERY.includes(pick.dimension) ? <Picker dimension={pick.dimension} bases={[pick.base]} code={pick.code} seeds={s} onChange={set} /> : (
          <span className="set">
            <Text label="code" value={pick.code} onChange={(v) => { s.drop(); set({ code: v }); }} />
            <Btn onClick={() => set({ code: m.random_code(pick.dimension, pick.base, s.next()) })}>Randomize</Btn>
          </span>
        )}
        <Pick label="dimension" value={pick.dimension} options={DIMS} onChange={(v) => move({ dimension: +v })} />
        <Pick label="base" value={pick.base} options={SPACES.get(pick.dimension) ?? [pick.base]} onChange={(v) => move({ base: +v })} />
        <Pick label="measure" value={pick.measure} options={MEASURES} onChange={(v) => set({ measure: v })} />
        <Pick label="axis" value={pick.axis} options={[['level', 'level L'], ['side', 'odd side 2k - 1']]} onChange={(v) => set({ axis: v })} />
        <Slider label="terms" value={pick.count} min={4} max={32} onChange={(v) => set({ count: v })} />
      </Group>
      <Group name="The view">
        <Pick label="view" value={pick.view} options={VIEWS} onChange={(v) => set({ view: v })} />
        {pick.view === 'digits' && <Pick label="digit base" value={pick.dig} options={digits(pick.base)} onChange={(v) => set({ dig: +v })} />}
      </Group>
      <Group name="The mix">
        <Check label="mix a second sequence" checked={pick.on} onChange={(v) => set({ on: v })} />
        {pick.on && (
          <span className="set">
            <Pick label="second design" value={pick.scode} options={options} onChange={(v) => set({ scode: v })} />
            <Pick label="its measure" value={pick.smeasure} options={MEASURES} onChange={(v) => set({ smeasure: v })} />
            <Pick label="operation" value={pick.op} options={OPS} onChange={(v) => set({ op: v })} />
            <Slider label="argument" value={pick.arg} min={-8} max={12} onChange={(v) => set({ arg: v })} />
          </span>
        )}
      </Group>
    </>
  );

  return (
    <Page crumb="plot" title="Every sequence the designs write, drawn"
      sub="The ledger lists these sequences; this page draws one. Pick a design, a measure and an axis and the terms stand up as a pin plot, a log plot, a ratio strip, a difference triangle or a heatmap of their digits, with the linear recurrence they satisfy read out beneath. Then mix in a second sequence and watch the rule the blend inherits."
      foot={<>A sequence is one design, one measure and one axis, the same key the <a href="../sequences">ledger</a> holds, so a row there links straight into this page. The pin plot stands every term on its index and the log plot lifts the tall ones back into view; the ratio strip colours each term against the one before, the difference triangle takes differences until the rows run out, and the heatmap spells every term in a chosen base. Under the drawing is the rule: the smallest linear constant-coefficient recurrence every term satisfies, hunted modulo three primes, rebuilt as exact fractions and verified on every term before it is shown, with its monic characteristic polynomial and the largest real root of that polynomial. The growth is that root where a rule fits and a least-squares slope of the log terms over the tail where none does, a fit and not a rule, and the exponent is the decades a term gains per step, which is the slope the log plot draws. Order and growth are honest only to the terms in hand: a rule needs twice its order plus two terms to be trusted, so a short read finds no rule where a longer one would. The mixer takes a second key from the same space and axis and runs a term operation over the two, adding, subtracting, multiplying term by term or convolving them, or dropping, thinning, differencing, summing and scaling the first alone; the mixed sequence carries its own rule, because sequences satisfying linear recurrences are closed under every one of these. The registry behind both pages is written up in <a href="/research/sequences/">the sequences note</a>, and their formal census, with the fill law and the exposed-face recurrence, is the <a href="/papers/sequence-census/">sequence-census paper</a>. Every number on this page is computed in Rust; the page only draws.</>}
      controls={controls}>
      <div className={pick.on ? 'arena' : undefined}>
        <div className="panel">
          <h2>The sequence <span>{row ? row.name : 'nothing read'}</span></h2>
          {row && <View row={row} view={pick.view} dig={pick.dig} label={label} name={`The sequence ${row.name}`} />}
          {row && <Terms terms={row.terms} start={row.start} capped={row.capped} />}
          {row && (
            <Stats>
              <Stat label="closed form">{row.closed || 'none known'}</Stat>
              <Stat label="record">{row.oeis ? <a href={`https://oeis.org/${row.oeis}`} target="_blank" rel="noopener">{row.oeis}</a> : 'none'}</Stat>
              <Stat label="status">{row.tag || 'unmatched'}</Stat>
              <Stat label="terms">{row.capped ? `${row.terms.length}, to the budget` : row.terms.length}</Stat>
            </Stats>
          )}
          {row && <Rule row={row} />}
          <Note error={read.error} />
        </div>
        {pick.on && (
          <div className="panel">
            <h2>The mix <span>{blend.mate ? `${pick.op} of this and ${blend.mate.name}` : pick.op}</span></h2>
            {blend.row && <View row={blend.row} view={pick.view} dig={pick.dig} label={`${PAIRED.includes(pick.op) ? 'both sequences' : 'the first sequence'}, index from 0`} name={`The mix, the ${pick.op} of the two sequences`} />}
            {blend.row && <Terms terms={blend.row.terms} />}
            {blend.mate && (
              <Stats>
                <Stat label="second">{blend.mate.name}</Stat>
                <Stat label="its closed form">{blend.mate.closed || 'none known'}</Stat>
                <Stat label="its growth">{shown(blend.mate.growth)}</Stat>
                <Stat label="argument">{PAIRED.includes(pick.op) ? 'unused' : pick.arg}</Stat>
              </Stats>
            )}
            {blend.row && <Rule row={blend.row} />}
            <Note error={blend.error} />
          </div>
        )}
      </div>
    </Page>
  );
}

mount(<App />);
