import { useEffect, useState } from 'react';
import { ready, ink, fit } from '../../lib/mrly.js';
import { mount, Page, Row, Pick, Slider, Check, Btn, Stat, Note } from '../../lib/app.jsx';
import { Grid, Signs, Sketch } from '../../lib/draw.jsx';
import { Picker, useSeeds } from '../../lib/select.jsx';
import { useQuery, share } from '../../lib/query.js';

const m = await ready();
const ROUNDS = 8;
const SIDE = 512;
const LEVELS = 9;
const RATES = 64;

const FIRST = { round: 6, length: 256, level: 6, fold: 'sign', code: '9', side: 2, base: 2, depth: 3 };

const GALLERY =
  'Three of the four lifts fold, and the crate checks each one against the Kronecker power of its own corner tile rather than trusting the picture. The corner tile is the only candidate worth testing: if a grid is a tile folded L times then its corner block is that tile with every bit flipped by (L - 1) t00, and folding the block L times flips the grid again by L (L - 1) t00, which is even. So a grid folds if and only if it folds from its own corner, and no search is needed.';

const FILTER =
  'The filter has a closed form in both folds, and the closed form is not the Thue-Morse grid. Under the and fold the next level is this level blown up and masked by the base tile, so the difference is this level blown up and masked by the tile complement. Under the plus-minus fold the next level is this level blown up and exclusive-ored with the repeated tile, so the difference is the repeated tile alone. Either way the filter keeps the last digit and throws every other one, so its output repeats with period equal to the tile side while the Thue-Morse grid does not, and the two differ at every side past the tile. Sharper on the plus-minus fold at tile side two: the disagreement is exactly half the sites at every side four and beyond, for every one of the sixteen designs, because the low digits fix a residue class and inside each class the high digits carry opposite letters exactly half the time. Half is the score a coin gets, so the filter output is no evidence of Thue-Morse at all. The resemblance is real and it is generic rather than special: both pictures are digit rules, so both are Kronecker powers of one small tile, and the eye reads any plus-minus speckle at the finest scale as Thue-Morse. Proved here, and pinned in the crate at every code and level the page draws.';

const attempt = (fn) => {
  try {
    return { ...fn(), error: null };
  } catch (error) {
    return { error };
  }
};

const hue = (bit) => (bit ? ink.blue : ink.orange);

const ribbon = (rows, length) => (canvas) => {
  const tall = rows.length * 26 + 4;
  const [ctx, w] = fit(canvas, tall);
  ctx.clearRect(0, 0, w, tall);
  const step = (w - 16) / length;
  rows.forEach((row, r) => {
    for (let i = 0; i < row.length; i++) {
      ctx.fillStyle = row[i];
      ctx.fillRect(8 + i * step, 2 + r * 26, Math.max(1, step - 1), 22);
    }
  });
};

function spread(runs) {
  const out = [];
  for (const run of runs) for (let k = 0; k < run; k++) out.push(run);
  return out;
}

function tile(bits) {
  return { width: 2, height: 2, types: Uint8Array.from(bits) };
}

function verdict(row) {
  if (row.twin) return <span className="chip proved">the same grid as {row.twin}</span>;
  if (row.folds) return <span className="chip proved">a Kronecker power</span>;
  return <span className="chip refuted">no Kronecker power</span>;
}

function App() {
  const shared = useSeeds();
  const [pick, set] = useQuery(FIRST);
  const [playing, setPlaying] = useState(false);
  const [parity, setParity] = useState(false);

  const clamp = (value, low, high) => Math.max(low, Math.min(value, high));
  const round = clamp(pick.round, 1, ROUNDS);
  const level = clamp(pick.level, 1, LEVELS);
  const length = clamp(pick.length, 16, 4096);
  const cap = m.level_cap(pick.side, 1, SIDE);
  const depth = clamp(pick.depth, 1, Math.max(1, cap - 1));

  useEffect(() => {
    if (!playing) return;
    const tick = setInterval(() => set({ round: round >= ROUNDS ? 1 : round + 1 }), 700);
    return () => clearInterval(tick);
  }, [playing, round]);

  const word = attempt(() => ({ read: JSON.parse(m.morse_word(length)) }));

  const seed = attempt(() => {
    const stage = Array.from(m.morse_stage(round));
    const digits = JSON.parse(m.morse_word(stage.length)).digits;
    return { stage, digits, agree: stage.every((bit, i) => bit === digits[i]) };
  });

  const gallery = attempt(() => ({
    rows: JSON.parse(m.morse_gallery(level)),
    grids: ['parity', 'and', 'xor', 'sum'].map((kind) => m.morse_lift(kind, level)),
  }));

  const filter = attempt(() => ({
    read: JSON.parse(m.morse_filter(pick.code, pick.side, pick.base, depth, pick.fold)),
    coarse: pick.fold === 'sign' ? m.morse_signs(pick.code, pick.side, pick.base, depth) : m.two_grid(pick.code, pick.side, depth, 0, pick.base),
    fine: pick.fold === 'sign' ? m.morse_signs(pick.code, pick.side, pick.base, depth + 1) : m.two_grid(pick.code, pick.side, depth + 1, 0, pick.base),
    difference: m.morse_difference(pick.code, pick.side, pick.base, depth, pick.fold),
  }));

  const rates = attempt(() => ({ read: JSON.parse(m.magic_rates(['3', '7'], [2, 2], [2, 2], 'thue-morse', RATES)) }));

  const wordLink = share({
    l0code: '3', l0base: 2, l0n: 2, l1code: '7', l1base: 2, l1n: 2,
    view: 'nest', compare: 'swap', chart: 'exponent', schedule: 'thue-morse', length: RATES,
  });

  const letters = () => {
    const { stage, digits, agree } = seed;
    const top = parity ? digits : stage;
    const low = parity ? stage : digits;
    return (
      <>
        <Sketch draw={ribbon([top.map(hue), low.map(hue)], top.length)} deps={[top, low, parity]} role="img" aria-label="The word beside the parity of the binary digit sum" />
        <div className="stats">
          <span><span className="swatch" style={{ background: ink.orange }}></span> letter 0, plus one</span>
          <span><span className="swatch" style={{ background: ink.blue }}></span> letter 1, minus one</span>
          <Stat label="letters">{top.length}</Stat>
          <span>{parity ? 'top by bit parity, below by substitution' : 'top by substitution, below by bit parity'}</span>
          <span className={`chip ${agree ? 'proved' : 'refuted'}`}>{agree ? 'the two rules agree letter for letter' : 'the two rules differ'}</span>
        </div>
      </>
    );
  };

  const runs = () => {
    const { read } = word;
    const lengths = spread(read.runs).slice(0, read.length);
    return (
      <>
        <Sketch draw={ribbon([read.digits.map(hue), lengths.map((run) => (run === 1 ? ink.gold : ink.pink))], read.length)} deps={[read]} role="img" aria-label="The runs" />
        <div className="stats">
          <span><span className="swatch" style={{ background: ink.gold }}></span> a run of one</span>
          <span><span className="swatch" style={{ background: ink.pink }}></span> a run of two</span>
          <Stat label="longest run">{read.longest}</Stat>
          <Stat label="runs of one">{read.singles}</Stat>
          <Stat label="runs of two">{read.doubles}</Stat>
          <Stat label="ones">{read.ones}</Stat>
          <span className={`chip ${read.cube_free ? 'proved' : 'refuted'}`}>{read.cube_free ? 'no 000 and no 111' : 'a cube appears'}</span>
        </div>
        <h2>the run boundaries <span>one wherever a letter differs from the next</span></h2>
        <Sketch draw={ribbon([read.boundary.map(hue), read.doubling.map(hue)], read.boundary.length)} deps={[read]} role="img" aria-label="The run boundaries" />
        <div className="stats">
          <span>top, the boundary word of Thue-Morse</span>
          <span>below, the period-doubling word grown by 1 to 10 and 0 to 11</span>
          <span className={`chip ${read.doubling_agree ? 'proved' : 'refuted'}`}>{read.doubling_agree ? 'the same word' : 'two words'}</span>
        </div>
      </>
    );
  };

  const card = (row, grid) => (
    <div className="panel" key={row.name}>
      <h2>{row.formula} <span>side {row.side}</span></h2>
      <Signs grid={grid} role="img" aria-label={row.formula} />
      <div className="stats">
        {verdict(row)}
        {row.folds ? <span>base tile <Signs grid={tile(row.tile)} className="" style={{ width: 22, height: 22, borderRadius: 4, verticalAlign: 'middle', imageRendering: 'pixelated' }} role="img" aria-label="the base tile" /></span> : null}
        {row.design ? <Stat label="the plus-minus render of">{m.name_of(row.design, 2, 2)}</Stat> : null}
        {row.folds ? null : <span>differs from its corner fold at <b>{row.faults}</b> of <b>{row.side * row.side}</b> sites, first at row <b>{row.first[0]}</b> column <b>{row.first[1]}</b></span>}
      </div>
    </div>
  );

  const pane = (label, grid, on, note) => (
    <div key={label}>
      {pick.fold === 'sign' ? <Signs grid={grid} role="img" aria-label={label} /> : <Grid grid={grid} on={on} role="img" aria-label={label} />}
      <div className="stats"><span>{label}</span><span className="dim">side {grid.width}{note}</span></div>
    </div>
  );

  const box = () => {
    const { read } = filter;
    return (
      <>
        <div className="arena">
          {pane('the level', filter.coarse, ink.gold, ', drawn at the width of the next, which is the blow-up')}
          {pane('the next level', filter.fine, ink.gold, '')}
          {pane('the difference', filter.difference, ink.pink, '')}
        </div>
        <div className="stats">
          <span className={`chip ${read.closed_exact ? 'proved' : 'refuted'}`}>{read.closed_exact ? 'exactly' : 'not'} {read.form}</span>
          <span className={`chip ${read.morse_exact ? 'proved' : 'refuted'}`}>{read.morse_exact ? 'the Thue-Morse grid' : 'not the Thue-Morse grid'}</span>
          {read.morse_faults === null ? <span className="dim">the Thue-Morse grid lives at side two, so a side-{read.number} tile has nothing to compare against</span>
            : <span>differs from Thue-Morse at <b>{read.morse_faults}</b> of <b>{read.cells}</b> sites, a share of <b>{(read.morse_faults / read.cells).toFixed(4)}</b></span>}
          {read.morse_faults !== null && read.morse_faults * 2 === read.cells ? <span className="chip refuted">exactly half the sites, the score a coin gets</span> : null}
          <Stat label="lit">{read.lit}</Stat>
          {read.morse_tile ? <span className="chip verified">this design reads plus-minus as the Thue-Morse tile</span> : null}
        </div>
      </>
    );
  };

  const controls = (
    <>
      <section>
        <h3>The word</h3>
        <Row>
          <Slider label="substitution rounds" value={round} min={1} max={ROUNDS} show={`${round}, ${1 << round} letters`} onChange={(v) => { setPlaying(false); set({ round: v }); }} />
          <Btn on={playing} onClick={() => setPlaying(!playing)}>{playing ? 'Stop' : 'Play'}</Btn>
          <Check label="bit parity on top" checked={parity} onChange={setParity} />
        </Row>
      </section>
      <section>
        <h3>The lifts</h3>
        <Row>
          <Slider label="level" value={level} min={1} max={LEVELS} show={`${level}, side ${1 << level}`} onChange={(v) => set({ level: v })} />
        </Row>
      </section>
      <section>
        <h3>The runs</h3>
        <Row>
          <Pick label="letters" value={length} options={[[64, 64], [128, 128], [256, 256], [512, 512], [1024, 1024]]} onChange={(v) => set({ length: +v })} />
        </Row>
      </section>
      <section>
        <h3>The difference filter</h3>
        <Row>
          <span className="set">
            <Picker dimension={2} bases={[2, 3]} code={pick.code} base={pick.base} seeds={shared}
              onChange={(values) => set({ ...values, ...(values.base && pick.side < values.base ? { side: values.base } : {}) })} />
          </span>
          <Pick label="tile side" value={pick.side} options={[[2, 2], [3, 3], [5, 5]]} onChange={(v) => set({ side: Math.max(+v, pick.base) })} />
          <Slider label="level" value={depth} min={1} max={Math.max(1, cap - 1)} onChange={(v) => set({ depth: v })} />
          <Pick label="fold" value={pick.fold} options={[['sign', 'plus-minus, the exclusive or'], ['design', 'the design, the and']]} onChange={(v) => set({ fold: v })} />
        </Row>
      </section>
    </>
  );

  return (
    <Page crumb="morse" title="The Thue-Morse word is one digit rule built twice"
      sub="The most famous aperiodic sequence is a mrly object. Its letter is a digit rule, the same move every design makes; its famous plane pattern is the Kronecker power of one plus-minus tile; and it is the schedule along which the tree computed a component exponent exactly."
      controls={controls}
      foot={<>Every letter, grid, run and verdict below comes out of the crates through wasm; the page only draws. The plus-minus render is the first on the site: a warm cell is plus one, a cool cell is minus one, and a dark cell is empty. Links: <a href={`../words${wordLink}`}>the words</a> drives a word by this schedule, <a href="../moire">moire</a> stacks one design over its scales, <a href="../sequences">the sequences</a> holds the ledger the designs write. A rule that changes with the scale is a word, and the grammar of one is in <a href="/research/magic/">the magic words note</a>.</>}>

      <div className="panel">
        <h2>the word <span>0 to 01, 1 to 10, beside the parity of the binary digit sum</span></h2>
        <Note error={seed.error} />
        {seed.error ? null : letters()}
        <p className="sub">The digit rule is `t(n)`, the parity of the count of one-bits of `n`. The substitution grows the same word from a single 0 by doubling: every 0 becomes 01 and every 1 becomes 10. Digits are the recursion, so the two constructions are one construction seen twice, and the chip above is a live comparison rather than a claim.</p>
      </div>

      <div className="arena">
        {gallery.error ? null : gallery.rows.map((row, at) => card(row, gallery.grids[at]))}
      </div>
      <Note error={gallery.error} />
      <p className="sub">{GALLERY}</p>
      <p className="sub">The first lift is the sign grid `(-1)^(popcount(i) + popcount(j))`, the Kronecker power of the two-by-two tile with plus one on its diagonal, and that tile is a mrly design read plus-minus. The second is the Walsh-Hadamard pattern, the gasket read the same way, which is the spectrometer's world. The third is not a third grid at all: `popcount(i xor j)` and `popcount(i) + popcount(j)` agree modulo two, so `t` carries exclusive or to exclusive or and the third lift is the first. The fourth carries, and carrying is not a digit rule, so it does not fold; its grid is constant along every antidiagonal instead, which is a Hankel pattern and never a Kronecker power past side two.</p>

      <div className="panel">
        <h2>the runs <span>cube-freeness made visible</span></h2>
        <Note error={word.error} />
        {word.error ? null : runs()}
        <p className="sub">Because `t(2n) = t(n)` and `t(2n+1) = 1 - t(n)`, the word changes at every even place, so no run reaches three: `000` and `111` never appear. The word that marks where the runs break is the period-doubling word, and the page checks that identity term by term at every length it draws. Verified in the crates, and stated in research/connectivity.md at every length to `2^20`.</p>
      </div>

      <div className="arena">
        <div className="panel">
          <h2>the schedule <span>Thue-Morse as a word over two letters</span></h2>
          <p className="sub">Read the letters as designs rather than as bits and the word becomes a schedule: one design per level, the domino where the letter is 0 and the gasket where it is 1. That is exactly what <a href={`../words${wordLink}`}>the words</a> draws, with the component exponent charted along the schedule and its periodic control beside it.</p>
          <div className="stats">
            <Stat label="letters">the domino and the gasket at side two</Stat>
            <span className="chip proved">order-blind at interior frequency</span>
          </div>
          <p className="sub">Open <a href={`../words${wordLink}`}>the words at this schedule</a>.</p>
        </div>
        <div className="panel">
          <h2>the exponent <span>the value the tree computed exactly</span></h2>
          <Note error={rates.error} />
          {rates.error ? null : (
            <div className="stats">
              <Stat label="interior exponent, log two units">{rates.read.limit.toFixed(15)}</Stat>
              <Stat label="the prefix rate at length 64">{rates.read.rows[rates.read.length - 1][0].toFixed(9)}</Stat>
              <Stat label="the periodic control">{rates.read.control[rates.read.length - 1].toFixed(9)}</Stat>
              <span className="chip proved">(1/2) log 6, Proved</span>
            </div>
          )}
          <p className="sub">Along Thue-Morse over a gasket-and-domino pair the component exponent is exactly `(1/2) log 6`, with a two-sided certificate rather than a fit: the word has no three equal letters in a row, which caps the sandwich suffix, and it is balanced, which pins the letter counts to within one half of `L/2`. The aperiodicity earns nothing extra here, because the exponent depends on the letter frequencies alone, so a periodic word of the same frequencies returns the same number. Proved, research/connectivity.md.</p>
        </div>
      </div>

      <div className="panel">
        <h2>the difference filter <span>a level exclusive-ored with its own next level</span></h2>
        <Note error={filter.error} />
        {filter.error ? null : box()}
        <p className="sub">{FILTER}</p>
      </div>
    </Page>
  );
}

mount(<App />);
