import { useMemo, useState } from 'react';
import { ready, ink } from '../../lib/mrly.js';
import { mount, Page, Row, Slider, Btn, Stats, Stat } from '../../lib/app.jsx';
import { Grid, Markup } from '../../lib/draw.jsx';
import { useSeeds } from '../../lib/select.jsx';
import { useQuery, share } from '../../lib/query.js';

const m = await ready();
const COLORS = [ink.blue, ink.orange, ink.gold, ink.green, ink.pink];
const COUNTS = m.counting_sequence(4).join(', ');
const BASE3 = m.baseq_sequence(3, 2).join(', ');
const WORLD = { 2: JSON.parse(m.universe(2)), 3: JSON.parse(m.universe(3)) };
const FIRST = { 2: '7', 3: '23' };
const BUDGET = 60000;

const tint = (design) => COLORS[design.degree % COLORS.length];

const drawn = (seed, designs) => designs[m.random_between(seed, [0], [designs.length - 1])[0]].code;

function thumb(design, dimension) {
  const label = `design ${design.code}, ${design.anf}`;
  if (dimension === 2) return <Grid grid={m.two_grid(design.code, 3, 2, 0, 2)} on={tint(design)} className="" role="img" aria-label={label} />;
  try {
    return <Markup svg={m.hex_svg(design.code, 3, 1, 2, 'iso', 6)} role="img" aria-label={label} />;
  } catch {
    return <Markup svg='<svg viewBox="0 0 1 1"></svg>' role="img" aria-label={label} />;
  }
}

function grown(design, dimension, level) {
  const label = `${design.name} grown to level ${level}`;
  if (dimension === 2) return <Grid grid={m.two_grid(design.code, 3, level, 0, 2)} on={tint(design)} style={{ maxWidth: 486 }} role="img" aria-label={label} />;
  try {
    return <Markup svg={m.hex_svg(design.code, 3, level, 2, 'iso', level === 3 ? 2 : 8)} role="img" aria-label={label} />;
  } catch (error) {
    return <div>{String(error.message ?? error)}</div>;
  }
}

function App() {
  const s = useSeeds();
  const [pick, set] = useQuery({ d: 2 });
  const dimension = pick.d === 3 ? 3 : 2;
  const world = WORLD[dimension];
  const [level, setLevel] = useState(3);
  const [code, setCode] = useState(() => (s.get() ? drawn(s.get(), world.designs) : FIRST[dimension]));
  const design = world.designs.find((one) => one.code === code) ?? world.designs[0];
  const top = m.level_cap(3, dimension, BUDGET);
  const grow = Math.min(level, top);
  const cards = useMemo(() => world.designs.map((one) => [one, thumb(one, dimension)]), [dimension]);

  const swap = (d) => {
    set({ d });
    setCode(FIRST[d]);
    setLevel(Math.min(level, m.level_cap(3, d, BUDGET)));
  };

  const controls = (
    <Row>
      <div className="tabs" role="group" aria-label="Dimension">
        <Btn on={dimension === 2} onClick={() => swap(2)}>plane</Btn>
        <Btn on={dimension === 3} onClick={() => swap(3)}>cube</Btn>
      </div>
      <Btn onClick={() => setCode(drawn(s.next(), world.designs))}>Randomize</Btn>
      <Slider label="level" value={grow} min={1} max={top} onChange={setLevel} />
      {dimension === 3 && <a href={`../sponge${share({ code: design.code, number: 3, base: 2 })}`}>open in the sponge</a>}
    </Row>
  );

  return (
    <Page crumb="universe" title="The universe of designs is a finite gallery" controls={controls}
      sub={<>A code is a bitmask over the corners of a hypercube. Rotations and reflections fold the codes into orbits, and one design per orbit is all there is: <span className="num">{COUNTS}</span> distinct designs in dimensions 1 to 4. Click one to grow it.</>}
      foot={<>The distinct counts are Burnside averages over the hyperoctahedral group; the gallery enumerates the orbits outright and the two agree. In base 3 the same count runs <span className="num">{BASE3}</span> for dimensions 1 and 2. The two moves this gallery enumerates, a rule on the corners and that rule substituted into itself, are written up in <a href="/research/core/">the core</a>.</>}>
      <p className="num dim">{world.distinct} designs from {world.total} codes</p>
      <div className="cards" role="group" aria-label="Designs">
        {cards.map(([one, art]) => (
          <div key={one.code} role="button" tabIndex={0} aria-pressed={one.code === design.code}
            className={one.code === design.code ? 'card on' : 'card'}
            onClick={() => setCode(one.code)}
            onKeyDown={(e) => { if (e.key === 'Enter' || e.key === ' ') { e.preventDefault(); setCode(one.code); } }}>
            {art}
            <div className="code">{one.code}</div>
            <div className="name">{one.anf}</div>
          </div>
        ))}
      </div>
      <div className="panel" style={{ marginTop: 22 }}>
        <h2><span className="num">{design.name}</span></h2>
        {grown(design, dimension, grow)}
        <Stats>
          <Stat label="orbit">{design.orbit}</Stat>
          <Stat label="degree">{design.degree}</Stat>
          <Stat label="normal form">{design.anf}</Stat>
          <Stat label="filled">{m.fills(design.code, 3, dimension, grow, 2)} of {m.grid_total(3, dimension, grow)}</Stat>
          <Stat label="fill ratio">{m.ratio(design.code, 3, dimension, grow, 2).toFixed(4)}</Stat>
          <Stat label="dimension">{m.dimension(design.code, 3, dimension, 2).toFixed(4)}</Stat>
        </Stats>
      </div>
    </Page>
  );
}

mount(<App />);
