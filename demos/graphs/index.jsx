import { useEffect, useMemo, useReducer, useRef, useState } from 'react';
import { ready } from '../lib/mrly.js';
import { web } from '../lib/chart.js';
import { web as solid } from '../lib/stage.js';
import { mount, Page, Row, Pick, Slider, Btn, Stats, Stat, Note } from '../lib/app.jsx';
import { Sketch } from '../lib/draw.jsx';
import { Stage } from '../lib/stage.jsx';
import { Picker, useSeeds, roll } from '../lib/select.jsx';
import { useQuery } from '../lib/query.js';

const m = await ready();
const SPACES = [['flat', 'flat'], ['cube', 'cube'], ['hex', 'hex slice']];
const GRAPHS = [['core', 'core, filled cells'], ['edge', 'edge, corners and sides'], ['tunnel', 'tunnel, empty cells'], ['dual', 'dual, fills and voids']];
const KINDS = { flat: ['core', 'edge', 'tunnel'], cube: ['core', 'edge', 'tunnel'], hex: ['core', 'dual', 'edge'] };
const LAYOUTS = ['lattice', 'force'];
const CAMERAS = [['eye', 'perspective'], ['iso', 'isometric']];
const BUDGET = { lattice: 20000, force: 2000 };
const FIRST = { space: 'flat', camera: 'eye', code: '495', base: 3, ccode: '23', cbase: 2, number: 3, level: 2, graph: 'core', layout: 'lattice', dots: 3 };
const FIRST_TOP = 4;

const rename = (patch) => {
  const next = {};
  if (patch.code !== undefined) next.ccode = patch.code;
  if (patch.base !== undefined) next.cbase = patch.base;
  return next;
};

function App() {
  const s = useSeeds();
  const [pick, set] = useQuery(FIRST);
  const [playing, setPlaying] = useState(false);
  const [pulse, beat] = useReducer((x) => x + 1, 0);
  const [gen, again] = useReducer((x) => x + 1, 0);
  const st = useRef(null);
  const body = useRef(null);
  const speed = useRef(2);
  const opened = useRef(false);

  const flat = pick.space === 'flat';
  const dimension = flat ? 2 : 3;
  const code = (flat ? pick.code : pick.ccode).trim();
  const base = flat ? pick.base : pick.cbase;
  const which = KINDS[pick.space].includes(pick.graph) ? pick.graph : 'core';
  const top = useMemo(() => {
    try {
      return m.graph_cap(pick.space, code, pick.number, base, which, BUDGET[pick.layout]);
    } catch {
      return FIRST_TOP;
    }
  }, [pick.space, code, pick.number, base, which, pick.layout]);
  const level = Math.min(pick.level, top);

  const built = useMemo(() => {
    try {
      const name = m.name_of(code, dimension, base);
      const nodes = m.graph_nodes(pick.space, code, pick.number, level, base, which);
      const net = {
        dim: nodes[0],
        nodes,
        branches: m.graph_branches(pick.space, code, pick.number, level, base, which),
        roles: m.graph_roles(pick.space, code, pick.number, level, base, which),
      };
      const tally = JSON.parse(m.graph_census(pick.space, code, pick.number, level, base, which));
      const relax = pick.layout === 'force' ? new m.Layout(net.nodes.subarray(2), net.branches, net.dim, s.get()) : null;
      return { net, tally, name, relax, error: null };
    } catch (error) {
      return { net: null, tally: null, name: '', relax: null, error };
    }
  }, [pick.space, code, pick.number, base, which, level, pick.layout, pick.camera, gen]);

  const nodes = () => (built.relax ? built.relax.positions() : built.net.nodes.subarray(2));

  useEffect(() => {
    setPlaying(!!built.relax);
  }, [built]);

  useEffect(() => {
    if (!playing || !built.relax) return;
    let id = 0;
    const frame = () => {
      const t0 = performance.now();
      built.relax.step(speed.current);
      const dt = performance.now() - t0;
      if (dt < 8 && speed.current < 8) speed.current += 1;
      else if (dt > 14 && speed.current > 1) speed.current -= 1;
      beat();
      id = requestAnimationFrame(frame);
    };
    id = requestAnimationFrame(frame);
    return () => cancelAnimationFrame(id);
  }, [playing, built]);

  useEffect(() => {
    const seed = s.get();
    if (!seed) return;
    const [lv, g, ly] = roll(seed, [[1, FIRST_TOP], [0, GRAPHS.length - 1], [0, 1]]);
    const drawn = m.random_code(dimension, base, seed);
    set({ ...(flat ? { code: drawn } : { ccode: drawn }), level: lv, graph: GRAPHS[g][0], layout: LAYOUTS[ly] });
  }, []);

  const randomize = () => {
    const seed = s.next();
    const [lv, g, ly] = roll(seed, [[1, top], [0, GRAPHS.length - 1], [0, 1]]);
    const drawn = m.random_code(dimension, base, seed);
    set({ ...(flat ? { code: drawn } : { ccode: drawn }), level: lv, graph: GRAPHS[g][0], layout: LAYOUTS[ly] });
  };

  const sheet = (canvas) => {
    if (!built.net || pick.space === 'cube') return;
    web(canvas, Math.min(canvas.clientWidth, 620), nodes(), built.net.branches, built.net.roles, pick.dots);
  };

  const onStage = (live) => {
    st.current = live;
    live.project(pick.camera);
    if (!opened.current) {
      opened.current = true;
      if (new URLSearchParams(location.search).get('camera') === 'iso') live.view(1, 1, 1);
    }
    if (!built.net) {
      live.clear();
      body.current = null;
      return;
    }
    if (pick.space !== 'cube') return;
    if (body.current && body.current.of === built && body.current.dots === pick.dots) {
      body.current.place(nodes());
      return;
    }
    body.current = { of: built, dots: pick.dots, ...solid(nodes(), built.net.branches, built.net.roles, pick.dots / 250) };
    live.clear();
    for (const part of body.current.parts) live.add(part);
  };

  const tally = built.tally;
  const ticks = built.relax ? built.relax.ticks() : 0;

  return (
    <Page crumb="graphs" title="The network of a design"
      sub="A design is dots. Join every filled cell to its neighbours and you have a network: tips, junctions and pieces you can count, a length you can add up, a box dimension you can read. See it flat, orbit it as a cube, or take the hexagon a cube's diagonal cut leaves, then let the dots push apart and watch the lattice relax into a shape."
      foot={<>The core graph joins face-adjacent filled cells at their centres; the edge graph is the corners and unit sides those cells outline; the tunnel graph joins the empty cells instead. On the hexagon the core graph joins filled triangles across shared sides, the dual takes fills and voids together, and the edge graph is their sides, every triangle at its true aspect. Every node, branch, role and count comes out of the crates, and the Euler number is the design's own, read off its cell complex, so the level slider stops where the closed-form size says a build would stall. The force layout is Fruchterman and Reingold's: every pair repels as <code>k²/d</code>, every branch pulls as <code>d²/k</code>, a cooling cap on the move per tick settles the lattice, and the seed jitters the start so a symmetric lattice can fold; energy is the mean net force per node in units of <code>k</code>. The same graphs diagonalised are the <a href="../spectra">spectra</a> page; the pieces and boundary of a design raced against a random set of the same mass are the <a href="https://github.com/mrlyprod/mrlyprod/blob/main/research/connectivity.md">connectivity</a> page.</>}>
      <Row>
        <Pick label="space" value={pick.space} options={SPACES} onChange={(v) => set({ space: v, graph: KINDS[v].includes(pick.graph) ? pick.graph : 'core' })} />
        <span className="set">
          <Picker dimension={dimension} bases={flat ? [3, 2] : [2, 3]} code={flat ? pick.code : pick.ccode} base={base} seeds={s} button={false}
            onChange={(patch) => set(flat ? patch : rename(patch))} />
          <Btn onClick={randomize}>Randomize</Btn>
        </span>
        <Pick label="number" value={pick.number} options={[[3, 3], [5, 5], [7, 7]]} onChange={(v) => set({ number: +v })} />
        <Slider label="level" value={level} min={1} max={top} show={`${level} of ${top}`} onChange={(v) => set({ level: v })} />
        <Pick label="graph" value={which} options={GRAPHS.filter(([value]) => KINDS[pick.space].includes(value))} onChange={(v) => set({ graph: v })} />
      </Row>
      <Row>
        <Pick label="layout" value={pick.layout} options={LAYOUTS} onChange={(v) => set({ layout: v })} />
        <span className="tabs">
          <button disabled={!built.relax} onClick={() => setPlaying(!playing)}>{playing ? 'Pause' : 'Relax'}</button>
          <button disabled={!built.relax} onClick={again}>Reset</button>
        </span>
        <Slider label="dots" value={pick.dots} min={1} max={12} onChange={(v) => set({ dots: v })} />
        <span className="set" hidden={pick.space !== 'cube'}>
          <Pick label="camera" value={pick.camera} options={CAMERAS} onChange={(v) => set({ camera: v })} />
          <span className="tabs">
            <button onClick={() => st.current?.view(1, 1, 1)}>corner</button>
            <button onClick={() => st.current?.view(1, 1, 0)}>edge</button>
            <button onClick={() => st.current?.view(0, 0, 1)}>face</button>
          </span>
        </span>
      </Row>
      <div className="arena" style={{ gridTemplateColumns: '1fr' }}>
        <div className="panel">
          <h2>The network <span>{`${which} graph, level ${level}, ${pick.layout}${pick.space === 'cube' ? `, ${pick.camera === 'iso' ? 'isometric' : 'perspective'}` : ''}`}</span></h2>
          <Sketch draw={sheet} deps={[built, pick.dots, pick.space, pulse]} hidden={pick.space === 'cube'} />
          <Stage onStage={onStage} deps={[built, pick.dots, pick.space, pick.camera, pulse]} hidden={pick.space !== 'cube'} />
          <Stats>
            <span><i className="swatch" style={{ background: 'var(--gold)' }}></i> tip</span>
            <span><i className="swatch" style={{ background: 'var(--blue)' }}></i> path</span>
            <span><i className="swatch" style={{ background: 'var(--pink)' }}></i> junction</span>
            <span><i className="swatch" style={{ background: 'var(--dim)' }}></i> alone</span>
          </Stats>
        </div>
      </div>
      <Stats>
        <Stat label="name">{built.name}</Stat>
        <Stat label="nodes">{tally?.nodes}</Stat>
        <Stat label="branches">{tally?.branches}</Stat>
        <Stat label="tips">{tally?.tips}</Stat>
        <Stat label="junctions">{tally?.junctions}</Stat>
        <Stat label="pieces">{tally?.components}</Stat>
        <Stat label="length">{tally?.length.toFixed(2)}</Stat>
        <Stat label="box dimension">{tally?.box.toFixed(3)}</Stat>
        <Stat label="euler">{tally ? tally.euler ?? 'none' : ''}</Stat>
      </Stats>
      <Stats>
        <Stat label="ticks">{built.relax ? ticks : ''}</Stat>
        <Stat label="energy">{built.relax && ticks ? built.relax.energy().toExponential(2) : ''}</Stat>
        <Stat label="moved">{built.relax && ticks ? built.relax.moved().toExponential(2) : ''}</Stat>
        <Stat label="heat">{built.relax ? built.relax.temperature().toExponential(2) : ''}</Stat>
      </Stats>
      <Note error={built.error} />
    </Page>
  );
}

mount(<App />);
