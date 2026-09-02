import { useMemo, useState } from 'react';
import { ready, ink } from '../../lib/mrly.js';
import { mount, Page, Group, Pick, Slider, Text, Check, Btn, Stats, Stat, Note } from '../../lib/app.jsx';
import { Grid } from '../../lib/draw.jsx';
import { useSeeds, roll } from '../../lib/select.jsx';
import { useQuery } from '../../lib/query.js';

const m = await ready();

const FIRST = { rule: 110, seed: 'cell', density: 0.35, steps: 128, wrap: false };
const RUNS = [[64, 64], [128, 128], [256, 256], [512, 512]];
const SEEDS = [['cell', 'a single live cell'], ['soup', 'a random soup']];
const SIDE = 64;

const IDENTITY =
  'Wolfram 1983 numbers a rule by the byte it writes on the eight neighbourhoods, bit `4l + 2c + r` of `N` being the output on `(l, c, r)`. The tree numbers a three-dimensional design by the byte it writes on the eight corners of the cube, bit `i` set when corner `i` is filled, `i = 4 x0 + 2 x1 + x2`. Put `(x0, x1, x2) = (l, c, r)` and the two bytes are the same byte: rule `N` is the design `mrly_bang_d3_N`, bit for bit, with no translation step. So every invariant of a design is already an invariant of a rule: popcount, the `GF(2)` degree, the genus, the fill fraction that Langton 1990 calls `lambda`. Proved, research/automata.md.';

const CLASSES =
  'Two groups act on the same byte and they are not the same group. Wolfram equivalence is reflection with conjugation, order 4, and it is a symmetry of the line, so it preserves the dynamics. The cube group is the 48 signed axis permutations of the design, and it is not: a permutation that is not the reflection moves the centre cell off the centre. The two meet exactly in the reflection, so the reductions to 88 classes and to 22 classes are two branches and not a chain. Reversibility survives the cube group anyway, constant on all 22 classes; surjectivity does not, mixed on exactly one class, the 24 rules of the orbit of 30. Proved by witness, research/automata.md.';

const ADDITIVE =
  'Where the rule is affine over `GF(2)` the diagram from one seed is a plane design of the same tree, one dimension down. Rule 60 is `x_i + x_(i-1)`, the binomial recurrence mod 2, so read rightward from the seed its rows are Pascal mod 2, which is `mrly_bang_d2_13`; rule 102 is the same rule mirrored and reads leftward as `mrly_bang_d2_14`; rule 90 is `x + x^-1`, and in the sheared frame `j = (t + i)/2` it is `mrly_bang_d2_13` again. Each is the unique match among the fill-3 codes 7, 11, 13, 14. The comparison below is recomputed live, cell for cell, against the crate design renderer rather than against a picture.';

const attempt = (fn) => {
  try {
    return { ...fn(), error: null };
  } catch (error) {
    return { error };
  }
};

const plain = (grid) => ({ width: grid.width, height: grid.height, types: grid.types });

const lit = (grid) => {
  let count = 0;
  for (const bit of grid.types) count += bit;
  return count;
};

const single = (width) => {
  const row = new Uint8Array(width);
  row[(width - 1) / 2] = 1;
  return row;
};

// THE SHEARED FRAME

const DEPTH = 2 * (SIDE - 1);

const READS = { 60: (t, j) => j, 102: (t, j) => j - (SIDE - 1), 90: (t, j) => 2 * j - t };

const FRAMES = { 60: 'read rightward from the seed', 102: 'read leftward from the seed, the left half of the cone laid out left to right', 90: 'read in the sheared frame j = (t + i)/2' };

function frame(rule, grid) {
  const centre = (grid.width - 1) / 2;
  const read = READS[rule];
  const types = new Uint8Array(SIDE * SIDE);
  for (let t = 0; t < SIDE; t++) {
    for (let j = 0; j < SIDE; j++) types[t * SIDE + j] = grid.types[t * grid.width + centre + read(t, j)];
  }
  return { width: SIDE, height: SIDE, types };
}

function App() {
  const seeds = useSeeds();
  const [pick, set] = useQuery(FIRST);
  const [tap, setTap] = useState(0);

  const rule = Math.max(0, Math.min(255, Math.round(pick.rule) || 0));
  const steps = RUNS.some(([value]) => value === pick.steps) ? pick.steps : 128;
  const width = 2 * steps + 1;
  const turn = (by) => set({ rule: (rule + by + 256) % 256 });

  const run = useMemo(() => attempt(() => {
    if (pick.seed === 'soup') {
      const row = m.eca_soup(width, pick.density, seeds.get());
      return { grid: plain(m.eca_history(row, rule, steps, pick.wrap)), how: `a soup of ${width} cells at density ${pick.density}` };
    }
    if (pick.wrap) return { grid: plain(m.eca_history(single(width), rule, steps, true)), how: `one live cell on a ring of ${width}` };
    return { grid: plain(m.eca_seed(rule, steps)), how: `one live cell, padded by ${steps} and cropped back to ${width}` };
  }), [rule, steps, pick.seed, pick.density, pick.wrap, tap]);

  const card = useMemo(() => attempt(() => {
    const read = JSON.parse(m.eca_card(rule));
    const mates = read.b3_orbit.map((mate) => ({ rule: mate, surjective: JSON.parse(m.eca_card(mate)).surjective }));
    return { read, mates, mixed: mates.some((mate) => mate.surjective) && mates.some((mate) => !mate.surjective) };
  }), [rule]);

  const gasket = useMemo(() => attempt(() => {
    const name = JSON.parse(m.eca_card(rule)).gasket;
    if (!name) return { name: null };
    const code = name.split('_').pop();
    const design = plain(m.two_grid(code, 2, 6, 0, 2));
    const window = frame(rule, plain(m.eca_seed(rule, DEPTH)));
    let faults = 0;
    for (let at = 0; at < design.types.length; at++) if (design.types[at] !== window.types[at]) faults += 1;
    return { name, code, design, window, faults };
  }), [rule]);

  const chip = (mate, badge) => (
    <span key={mate} role="button" tabIndex={0} onClick={() => set({ rule: mate })}
      onKeyDown={(event) => { if (event.key === 'Enter') set({ rule: mate }); }}
      style={mate === rule ? { borderColor: 'var(--accent)', color: 'var(--fg)' } : undefined}>
      <b>{mate}</b>{badge ? <i>{badge}</i> : null}
    </span>
  );

  const stamp = (corners) => (
    <Grid grid={{ width: 4, height: 2, types: Uint8Array.from(corners) }} on={ink.gold} className=""
      style={{ width: 128, height: 64, borderRadius: 4, imageRendering: 'pixelated', background: 'var(--art)' }}
      role="img" aria-label="the eight corner bits of the rule" />
  );

  const controls = (
    <>
      <Group name="The rule">
        <Text label="rule 0 to 255" value={String(rule)} onChange={(value) => set({ rule: Math.max(0, Math.min(255, +value.replace(/\D/g, '') || 0)) })} />
        <Btn onClick={() => turn(-1)}>Prev</Btn>
        <Btn onClick={() => turn(1)}>Next</Btn>
        <Btn onClick={() => set({ rule: roll(seeds.next(), [[0, 255]])[0] })}>Randomize</Btn>
      </Group>
      <Group name="The seed">
        <Pick label="seed" value={pick.seed} options={SEEDS} onChange={(value) => set({ seed: value })} />
        <Slider label="soup density" value={pick.density} min={0.05} max={0.95} step={0.01} onChange={(value) => set({ density: value })} />
        <Btn onClick={() => { seeds.next(); setTap(tap + 1); }}>New soup</Btn>
        <Check label="wrap" checked={pick.wrap} onChange={(value) => set({ wrap: value })} />
      </Group>
      <Group name="The run">
        <Pick label="generations" value={steps} options={RUNS} onChange={(value) => set({ steps: +value })} />
      </Group>
    </>
  );

  const read = card.read;

  return (
    <Page crumb="wolfram" title="The 256 rules are the 256 cube designs"
      sub="An elementary cellular automaton writes one byte on the eight neighbourhoods of a cell, and a three-dimensional parity design writes one byte on the eight corners of a cube. They are the same byte. Pick a rule and it arrives with a design's card already filled in, and where the rule is additive its own diagram is a plane design of the same tree."
      controls={controls}
      foot={<>Every bit, count and class on this page is computed in the crates and walked through wasm; the page only draws. The single-cell view runs the stable convention, a line padded by the depth and cropped back to the light cone, so the boundary never reaches the window; turning wrap on runs the same seed on a ring instead. Nearby: <a href="../life">life</a> runs birth and survival rules read from named sequences, <a href="../mrlylife">mrlylife</a> runs the same rules on any mask the tree draws, <a href="../universe">the universe</a> is the design gallery these bytes come from. The class lattice, the surjectivity census and the additive bridge are in <a href="/research/automata/">the automata note</a>.</>}>

      <div className="panel">
        <h2>the space-time diagram <span>time runs down, one row a generation</span></h2>
        <Note error={run.error} />
        {run.error ? null : <Grid grid={run.grid} on={ink.gold} role="img" aria-label={`Rule ${rule} from ${run.how}`} />}
        {run.error ? null : (
          <Stats>
            <Stat label="rule">{rule}</Stat>
            <Stat label="design">{card.error ? '-' : read.name}</Stat>
            <Stat label="rows">{run.grid.height}</Stat>
            <Stat label="cells across">{run.grid.width}</Stat>
            <Stat label="live cells">{lit(run.grid)}</Stat>
            <span className="dim">{run.how}</span>
          </Stats>
        )}
        <p className="sub">{IDENTITY}</p>
      </div>

      <div className="arena">
        <div className="panel">
          <h2>the card <span>the design's invariants, read off the same byte</span></h2>
          <Note error={card.error} />
          {card.error ? null : (
            <>
              {stamp(read.corners)}
              <Stats>
                <span className="dim">corner <b>i = 4 x0 + 2 x1 + x2</b>, so the top row is <b>l = 0</b> and the columns run <b>00, 01, 10, 11</b> in <b>(c, r)</b></span>
                <Stat label="name">{read.name}</Stat>
                <Stat label="popcount">{read.popcount}</Stat>
                <Stat label="lambda">{read.lambda.toFixed(3)}</Stat>
                <Stat label="GF(2) degree">{read.degree}</Stat>
                <Stat label="genus">{read.genus}</Stat>
                <span className={`chip ${read.affine ? 'proved' : ''}`}>{read.affine ? 'affine over GF(2)' : 'not affine'}</span>
                <span className={`chip ${read.surjective ? 'verified' : ''}`}>{read.surjective ? 'surjective' : 'not surjective'}</span>
                <span className={`chip ${read.reversible ? 'verified' : ''}`}>{read.reversible ? 'reversible' : 'not reversible'}</span>
                <Stat label="as a birth and survival rule">
                  {read.outer_totalistic ? `B${read.outer_totalistic.birth.join('')}/S${read.outer_totalistic.survive.join('')}` : 'not a B/S rule'}
                </Stat>
              </Stats>
              <p className="sub">Exactly 30 of the 256 rules are surjective and exactly 6 are reversible, the degree-one single-axis designs 15, 51, 85, 170, 204, 240 and nothing else. Verified, research/automata.md.</p>
            </>
          )}
        </div>

        <div className="panel">
          <h2>the classes <span>the cube orbit beside the Wolfram class</span></h2>
          <Note error={card.error} />
          {card.error ? null : (
            <>
              <h2>the cube orbit <span>{read.b3_orbit.length} rules, representative {read.b3_rep}</span></h2>
              <div className="ribbon tight">{card.mates.map((mate) => chip(mate.rule, mate.surjective ? 'onto' : ''))}</div>
              <h2>the Wolfram class <span>{read.wolfram_class.length} rules, representative {read.wolfram_rep}</span></h2>
              <div className="ribbon tight">{read.wolfram_class.map((mate) => chip(mate, ''))}</div>
              <Stats>
                <Stat label="NPN representative">{read.npn_rep}</Stat>
                <span className="dim">a chip is a link, click one to run it</span>
                {card.mixed
                  ? <span className="chip refuted">surjectivity is not constant on this orbit, so the cube symmetry is not a dynamical one</span>
                  : <span className="chip verified">every rule in this orbit answers surjectivity the same way</span>}
              </Stats>
              <p className="sub">{CLASSES}</p>
            </>
          )}
        </div>
      </div>

      {gasket.error || !gasket.name ? null : (
        <div className="panel">
          <h2>the gasket <span>{FRAMES[rule]}, against the design the card names</span></h2>
          <div className="arena">
            <div>
              <Grid grid={gasket.window} on={ink.gold} role="img" aria-label="the diagram cropped to 64 rows" />
              <div className="stats"><span>the diagram, {SIDE} rows</span></div>
            </div>
            <div>
              <Grid grid={gasket.design} on={ink.blue} role="img" aria-label={`the design ${gasket.name} at level 6`} />
              <div className="stats"><span>{gasket.name}, level 6</span></div>
            </div>
          </div>
          <Stats>
            <Stat label="cells compared">{SIDE * SIDE}</Stat>
            <Stat label="cells that differ">{gasket.faults}</Stat>
            <span className={`chip ${gasket.faults === 0 ? 'proved' : 'refuted'}`}>{gasket.faults === 0 ? 'the same grid, cell for cell' : 'two grids'}</span>
          </Stats>
          <p className="sub">{ADDITIVE}</p>
        </div>
      )}

      <Note error={gasket.error} />
    </Page>
  );
}

mount(<App />);
