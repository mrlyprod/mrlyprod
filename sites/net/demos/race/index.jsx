import { useEffect, useRef, useState } from 'react';
import { ready, ink, rgb } from '../../lib/mrly.js';
import { mount, Page, Row, Btn, Stats, Stat } from '../../lib/app.jsx';
import { useSeeds } from '../../lib/select.jsx';

const m = await ready();
const NUMBER = 3, LEVEL = 4, BASE = 3, WALKERS = 300, TICKS = 12, SCALE = 6;
const SITE = [46, 54, 64], HOLE = rgb(ink.deep);
const DIM = m.dimension('127', NUMBER, 2, BASE).toFixed(3);

const SIDES = [
  { code: '127', color: ink.blue, swatch: 'var(--blue)', tone: 'the fast one' },
  { code: '239', color: ink.orange, swatch: 'var(--orange)', tone: 'the slow one' },
].map((side) => ({
  ...side,
  name: m.name_of(side.code, 2, BASE),
  fill: `${m.fills(side.code, NUMBER, 2, LEVEL, BASE)} of ${m.grid_total(NUMBER, 2, LEVEL)}`,
}));

function runner(code, color) {
  const off = document.createElement('canvas');
  const tint = rgb(color);
  let race = null, side = 0, types = null, image = null;
  return {
    far: 0,
    reset(seed, canvas) {
      race = new m.Race(code, NUMBER, LEVEL, BASE, WALKERS, seed);
      side = race.side();
      types = race.types();
      canvas.width = canvas.height = side * SCALE;
      off.width = off.height = side;
      image = new ImageData(side, side);
      this.far = 0;
    },
    tick() {
      this.far = race.step(1);
    },
    steps() {
      return race.steps();
    },
    draw(canvas) {
      const n = side, px = image.data, trail = race.trail(), [r, g, b] = tint;
      for (let i = 0; i < n * n; i++) {
        const base = types[i] ? SITE : HOLE;
        const heat = Math.min(1, trail[i] / 12) * 0.55;
        px[i * 4] = base[0] + (r - base[0]) * heat;
        px[i * 4 + 1] = base[1] + (g - base[1]) * heat;
        px[i * 4 + 2] = base[2] + (b - base[2]) * heat;
        px[i * 4 + 3] = 255;
      }
      for (const p of race.positions()) {
        px.set([r, g, b, 255], p * 4);
      }
      off.getContext('2d').putImageData(image, 0, 0);
      const ctx = canvas.getContext('2d');
      ctx.imageSmoothingEnabled = false;
      ctx.drawImage(off, 0, 0, n * SCALE, n * SCALE);
      const home = race.home();
      ctx.strokeStyle = ink.fg;
      ctx.lineWidth = 2;
      ctx.strokeRect((home % n) * SCALE - 2, Math.floor(home / n) * SCALE - 2, SCALE + 4, SCALE + 4);
    },
  };
}

function App() {
  const s = useSeeds();
  const [target, setTarget] = useState(20);
  const [finish, setFinish] = useState('20');
  const [running, setRunning] = useState(false);
  const [done, setDone] = useState(false);
  const [label, setLabel] = useState('Start the race');
  const [banner, setBanner] = useState('');
  const [steps, setSteps] = useState('');
  const [far, setFar] = useState([0, 0]);
  const sheets = useRef([]);
  const goal = useRef(target);
  const teams = useRef(null);
  teams.current ??= SIDES.map((side) => runner(side.code, side.color));

  const paint = () => teams.current.forEach((team, i) => team.draw(sheets.current[i]));

  const reset = () => {
    const seed = s.get() || 1;
    teams.current[0].reset(seed, sheets.current[0]);
    teams.current[1].reset(seed + 777, sheets.current[1]);
    paint();
    setFar([0, 0]);
    setSteps('');
    setBanner('');
    setDone(false);
    setRunning(false);
    setLabel('Start the race');
  };

  useEffect(() => {
    reset();
  }, []);

  useEffect(() => {
    if (!running) return;
    let id = 0;
    const frame = () => {
      let finished = false;
      for (let k = 0; k < TICKS && !finished; k++) {
        for (const team of teams.current) team.tick();
        const [a, b] = teams.current;
        if (a.far >= goal.current || b.far >= goal.current) {
          finished = true;
          const [winner, loser] = a.far >= goal.current ? [0, 1] : [1, 0];
          setBanner(`${SIDES[winner].name} wins at step ${teams.current[winner].steps()}; the other team is at ${teams.current[loser].far.toFixed(1)} cells. Same mass. Different music.`);
          setDone(true);
          setRunning(false);
          setLabel('Race again');
        }
      }
      paint();
      setFar(teams.current.map((team) => team.far));
      setSteps(`${teams.current[0].steps()} steps`);
      if (!finished) id = requestAnimationFrame(frame);
    };
    id = requestAnimationFrame(frame);
    return () => cancelAnimationFrame(id);
  }, [running]);

  const start = () => {
    setRunning(true);
    setLabel('Pause');
  };

  const go = () => {
    if (done) {
      s.next();
      reset();
      start();
      return;
    }
    setRunning(!running);
    setLabel(running ? 'Resume' : 'Pause');
  };

  const randomize = () => {
    s.next();
    reset();
    start();
  };

  const aim = (value) => {
    goal.current = value;
    setTarget(value);
    reset();
  };

  const controls = (
    <Row>
      <Btn primary onClick={go}>{label}</Btn>
      <Btn onClick={randomize}>Randomize</Btn>
      <label>finish <input type="number" min={5} max={60} value={finish} onChange={(e) => { setFinish(e.target.value); if (!e.nativeEvent.inputType) aim(+e.target.value); }} onBlur={() => aim(+finish)} onKeyDown={(e) => { if (e.key === 'Enter') aim(+finish); }} /> cells</label>
    </Row>
  );

  return (
    <Page crumb="race" title="The race: same mass, different music"
      sub={<>Two base-3 rules, each keeping 7 of 9 cells: the same material at every scale and the same fractal dimension <span className="num">{DIM}</span>. Drop random walkers on both and watch the shapes carry them at different speeds. First team to wander an average of <span className="num">{target}</span> cells from home wins.</>}
      controls={controls}
      foot={<>Both patterns are grown to level 4 from their own 3 by 3 rule and the walkers start at the filled cell nearest the centre; each tick every walker takes one blind step, and a step into a hole is a lost turn. The grids, the walkers, the distances and the seed all live in Rust, so a race replays exactly from its seed. Distance grows like a power of time whose exponent the shape sets; this page shows the race itself, not that exponent. The exponent, and the census showing that equal mass does not fix it, are in <a href="/research/walks/">the walk dimension note</a>.</>}>
      <div className="arena">
        {SIDES.map((side, i) => (
          <div className="panel" key={side.code}>
            <h2><i className="swatch" style={{ background: side.swatch }} /><span className="num">{side.name}</span><span>{side.tone}</span></h2>
            <canvas className="sheet" ref={(el) => { sheets.current[i] = el; }} role="img" aria-label={`${side.name}, ${side.tone}`} />
            <div className="meter"><div style={{ background: side.swatch, width: `${Math.min(100, far[i] / target * 100)}%` }} /></div>
            <Stats>
              <Stat label="distance from home">{`${far[i].toFixed(1)} cells`}</Stat>
              <Stat label="filled">{side.fill}</Stat>
            </Stats>
          </div>
        ))}
      </div>
      <p className="shift">{steps}</p>
      <div className="banner">{banner}</div>
    </Page>
  );
}

mount(<App />);
