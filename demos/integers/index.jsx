import { useEffect, useMemo, useRef, useState } from 'react';
import { ready, ink, rgb, fit } from '../lib/mrly.js';
import { useQuery } from '../lib/query.js';
import { mount, Page, Row, Btn, Stats, Stat, Note } from '../lib/app.jsx';
import { Sketch } from '../lib/draw.jsx';
import { useSeeds, roll } from '../lib/select.jsx';
import { board, bars, axis, tag } from '../lib/chart.js';

const m = await ready();
const COLS = 40;
const LINES = 25;
const PAGE = 25;
const BUDGET = 14;
const WIN = JSON.parse(m.census_window());
const CEILING = Number(WIN.ceiling);
const START = JSON.parse(m.census_walk(WIN.tiers[0].keys));

function mix(from, to, at) {
  const [a, b] = [rgb(from), rgb(to)];
  return `rgb(${a.map((c, k) => Math.round(c + (b[k] - c) * at)).join(',')})`;
}

function shade(count, peak) {
  if (count === 0) return ink.orange;
  if (count === 1) return ink.dim;
  return mix(ink.blue, ink.gold, Math.log(count) / peak);
}

function census() {
  return { counts: m.census_counts(), report: JSON.parse(m.census_report()) };
}

function frame() {
  return new Promise((resolve) => requestAnimationFrame(resolve));
}

function App() {
  const s = useSeeds();
  const [pick, set] = useQuery({ value: 16 });
  const value = Math.min(CEILING, Math.max(1, pick.value || 16));
  const [typing, setTyping] = useState(String(value));
  const [page, setPage] = useState(0);
  const [walking, setWalking] = useState(true);
  const [state, setState] = useState(START);
  const [look, setLook] = useState(census);
  const [error, setError] = useState(null);
  const live = useRef(state);
  live.current = state;

  const choose = (v, typed = false) => {
    const next = Math.min(CEILING, Math.max(1, v || 1));
    setPage(0);
    if (!typed || next !== v) setTyping(String(next));
    set({ value: next });
  };

  const pass = async () => {
    setWalking(true);
    setError(null);
    let span = 4;
    let ticks = 0;
    let broken = false;
    let now = live.current;
    do {
      await frame();
      const opened = performance.now();
      while (performance.now() - opened < BUDGET) {
        const clock = performance.now();
        try {
          now = JSON.parse(m.census_walk(span));
        } catch (error) {
          setError(error);
          broken = true;
          break;
        }
        const spent = Math.max(performance.now() - clock, 0.5);
        span = Math.max(1, Math.min(span * 2, 4000, Math.round((span * BUDGET) / spent)));
        if (now.done >= now.total || spent >= BUDGET) break;
      }
      live.current = now;
      setState(now);
      if (++ticks % 8 === 0) setLook(census());
    } while (!broken && now.done < now.total);
    setWalking(false);
    setLook(census());
  };

  useEffect(() => { pass(); }, []);

  const found = useMemo(() => JSON.parse(m.census_writers(value, page, PAGE)), [value, page, look]);
  const champs = useMemo(() => JSON.parse(m.census_champions(20)), [look]);
  const misses = useMemo(() => JSON.parse(m.census_misses(30)), [look]);

  const field = (canvas) => {
    const side = canvas.clientWidth / COLS;
    const [ctx, w, h] = fit(canvas, Math.round(side * LINES));
    ctx.clearRect(0, 0, w, h);
    const peak = Math.log(Math.max(2, ...look.counts));
    for (let i = 0; i < look.counts.length; i++) {
      ctx.fillStyle = shade(look.counts[i], peak);
      ctx.fillRect((i % COLS) * side, Math.floor(i / COLS) * side, Math.max(1, side - 1), Math.max(1, side - 1));
    }
    const at = value - 1;
    ctx.strokeStyle = ink.fg;
    ctx.lineWidth = 2;
    ctx.strokeRect((at % COLS) * side - 1.5, Math.floor(at / COLS) * side - 1.5, side + 2, side + 2);
  };

  const split = (canvas) => {
    const b = board(canvas, 62, { top: 20, bottom: 24 });
    const parts = [
      ['missed', look.report.never, ink.orange],
      ['written once', look.report.once, ink.dim],
      ['written by many', look.report.multiple, ink.blue],
    ];
    let at = 0;
    let label = b.x(0);
    for (const [name, count, color] of parts) {
      b.ctx.fillStyle = color;
      b.ctx.fillRect(b.x(at / CEILING), b.roof, Math.max(1, (b.wide * count) / CEILING - 1), b.floor - b.roof);
      label = tag(b, `${name} ${count}`, color, 'left', label, b.h - 8) + 14;
      at += count;
    }
    tag(b, `1 to ${CEILING} at ${state.depth} rendered terms`, ink.dim);
  };

  const champions = (canvas) => {
    const b = board(canvas, 210);
    bars(b, champs.map((row) => row.rows), { color: (i) => (champs[i].value === value ? ink.gold : ink.blue) });
    axis(b, champs.map((row, i) => [(i + 0.5) / champs.length, row.value]));
    tag(b, 'rows writing the integer, the twenty heaviest', ink.dim);
    tag(b, `leader ${champs[0].value} at ${champs[0].rows} rows`, ink.fg, 'right');
  };

  const onField = (event) => {
    const box = event.currentTarget.getBoundingClientRect();
    const side = box.width / COLS;
    const column = Math.floor((event.clientX - box.left) / side);
    const row = Math.floor((event.clientY - box.top) / side);
    if (column >= 0 && column < COLS && row >= 0 && row < LINES) choose(row * COLS + column + 1);
  };

  const onChampions = (event) => {
    const box = event.currentTarget.getBoundingClientRect();
    const at = Math.floor(((event.clientX - box.left - 14) / (box.width - 28)) * champs.length);
    if (champs[at]) choose(champs[at].value);
  };

  const random = () => {
    const [at] = roll(s.next(), [[1, CEILING]]);
    choose(at);
  };

  const miss = () => {
    const all = JSON.parse(m.census_misses(CEILING));
    if (!all.length) return;
    const [at] = roll(s.next(), [[0, all.length - 1]]);
    choose(all[at]);
  };

  const champion = () => {
    const rows = JSON.parse(m.census_champions(20));
    const [at] = roll(s.next(), [[0, rows.length - 1]]);
    choose(rows[at].value);
  };

  const tierOf = (name) => found.tiers.find((tier) => tier.tier === name)?.rows ?? '';

  return (
    <Page crumb="integers" title="The integers this machine writes, and the ones it misses"
      sub="Every design, every measure, every axis is a row of the ledger, and every row writes a run of integers. Take the union over the whole registry and most small integers are written many times over, a few are written once, and some are written by nothing at all. Type an integer or click the field: you get every row that writes it, the design, the measure and the closed form, or the verdict that it is missed."
      foot={<>The census is only as good as its window, so the window is pinned and printed rather than assumed. A row is one design, one measure and one axis, taken over the four cost tiers of the <a href="../sequences">ledger</a>. A row's rendered window is its first <b>min(48, B)</b> terms, <b>B</b> the leading terms whose footprint fits 100000 cells: one cell for a closed measure, <code>number^dimension + level * span</code> for a convolved one, <code>number^(dimension * level)</code> for a grid. A row whose rendered terms are strictly increasing stops at the first term above the ceiling, which on this page is 1000. A row writes <b>n</b> when <b>n</b> is a term inside that window and <b>n</b> is in range, and multiplicity counts rows and not places, so a row writing the same integer twice counts once. Terms at or below zero - the Euler characteristics, the voids of a solid - are counted apart and never folded in. The page opens at the ledger's own eight-term heads and deepens on request, because more than half of the written set arrives past the head; the last pass is the pinned 48-term window, and the depth table above is the honest measure of how much the window itself decides. That a missed integer is written by no row at any depth is a conjecture and not a result: the rows the cap still cuts have deeper terms nobody has rendered, and the counter says how many rows those are. The same census runs to a ceiling of 100000 in the research tree, where the miss density climbs decade by decade; whatever the ceiling, a fixed registry renders at most 48 terms a row, so the written set is finite and far enough out the census is almost all miss. Every number here is computed in Rust and walked live through wasm; the page only draws.</>}>
      <Row>
        <label>integer <input type="number" min={1} max={CEILING} value={typing} onChange={(e) => { setTyping(e.target.value); if (e.target.value !== '') choose(+e.target.value, true); }} /></label>
        <Btn onClick={random}>Randomize</Btn>
        <Btn onClick={miss}>A miss</Btn>
        <Btn onClick={champion}>A champion</Btn>
      </Row>
      <div className="panel">
        <h2>The pinned window <span>{walking ? `walking ${state.done} of ${state.total} rows at ${state.depth} terms` : state.complete ? `complete at the pinned ${WIN.cap}-term window` : `${state.pending} rows are cut by the ${state.depth}-term cap`}</span></h2>
        <div className="stats">
          <span>registry <b>{WIN.registry}</b> rows</span>
          <span>read <b>{`${state.rows} of ${WIN.registry} rows`}</b></span>
          <span>rendered terms <b>{state.complete ? `${state.depth}, the pinned cap` : state.depth}</b></span>
          <span>ceiling <b>{CEILING}</b></span>
          <span>cells a term <b>{WIN.cells}</b></span>
          <span><button hidden={walking || state.complete} onClick={() => { if (!walking && !state.complete) pass(); }}>{`deepen to ${state.next} terms · ${state.pending} rows`}</button></span>
        </div>
        <div className="meter"><div style={{ width: `${(100 * state.done) / Math.max(1, state.total)}%`, background: walking ? ink.blue : ink.green }} /></div>
        <Note error={error} />
      </div>
      <div className="arena">
        <div className="panel">
          <h2>The field <span>{`1 to ${CEILING}, one cell an integer, click to read one`}</span></h2>
          <Sketch draw={field} deps={[look, value]} onPointerDown={onField} />
          <div className="stats">
            <span><span className="swatch" style={{ background: ink.orange }} /> missed <b>{look.report.never}</b></span>
            <span><span className="swatch" style={{ background: ink.dim }} /> written once <b>{look.report.once}</b></span>
            <span><span className="swatch" style={{ background: ink.blue }} /> written by many <b>{look.report.multiple}</b></span>
            <span>share written <b>{look.report.share.toFixed(4)}</b></span>
          </div>
          <Sketch className="bars" style={{ height: 62 }} draw={split} deps={[look, state.depth]} />
        </div>
        <div className="panel">
          <h2>The verdict <span>{`${state.rows} rows read at ${state.depth} rendered terms`}</span></h2>
          <p className="banner" style={{ color: found.rows === 0 ? ink.orange : found.rows === 1 ? ink.gold : ink.fg }}>
            {found.rows ? `${value} is written by ${found.rows === 1 ? 'exactly one row' : `${found.rows} rows`}` : `${value} is missed: no row of the ${WIN.registry} writes it inside the window`}
          </p>
          <Stats>
            <Stat label="rows writing it">{found.rows}</Stat>
            <Stat label="closed">{tierOf('closed')}</Stat>
            <Stat label="convolved">{tierOf('convolved')}</Stat>
            <Stat label="side grid">{tierOf('side')}</Stat>
            <Stat label="level grid">{tierOf('level')}</Stat>
            <Stat label="page">{found.rows ? `${page + 1} of ${Math.ceil(found.rows / PAGE)}` : '0'}</Stat>
            <span>
              <button disabled={page === 0} onClick={() => setPage(page - 1)}>prev</button>{' '}
              <button disabled={(page + 1) * PAGE >= found.rows} onClick={() => setPage(page + 1)}>next</button>
            </span>
          </Stats>
          <div className="scroll">
            <table>
              <thead><tr><th>row</th><th>measure</th><th>closed form</th><th>writes it at</th><th>first terms</th></tr></thead>
              <tbody>
                {found.shown.length ? found.shown.map((row, i) => (
                  <tr key={`${i}:${row.name}`}>
                    <td className="mono"><a href={`../sequences?q=${row.name}`}>{row.name}</a></td>
                    <td className="mono">{row.measure} · {row.axis}</td>
                    <td className="mono">{row.closed || 'none known'}</td>
                    <td className="num">term {row.index + 1}, {row.axis === 'level' ? `level ${row.term}` : `side ${row.side}`}</td>
                    <td className="num">{row.head.join(', ')}</td>
                  </tr>
                )) : <tr><td className="dim">no row of the registry writes it</td><td colSpan={4}></td></tr>}
              </tbody>
            </table>
          </div>
        </div>
      </div>
      <div className="arena">
        <div className="panel">
          <h2>The champions <span>the integers the most rows write</span></h2>
          <Sketch className="bars" style={{ height: 210 }} draw={champions} deps={[champs, value]} onPointerDown={onChampions} />
          <h2>The first misses</h2>
          <p className="num dim">{misses.join(', ') || 'no integer of the window is missed'}</p>
        </div>
        <div className="panel">
          <h2>The depth of the window <span>{state.complete ? 'the last row is the pinned window' : `${state.pending} rows are still cut by the cap`}</span></h2>
          <div className="scroll">
            <table>
              <thead><tr><th>rendered terms</th><th>written</th><th>missed</th><th>once</th><th>first miss</th><th>rows still cut</th></tr></thead>
              <tbody>
                {look.report.depths.length ? look.report.depths.map((row) => (
                  <tr key={row.depth}>
                    <td className="num">{row.depth}</td>
                    <td className="num">{row.written}</td>
                    <td className="num">{row.never}</td>
                    <td className="num">{row.once}</td>
                    <td className="num">{row.first_miss}</td>
                    <td className="num">{row.deepenable}</td>
                  </tr>
                )) : <tr><td className="dim">{`the ${WIN.depths[0]}-term pass is still walking`}</td><td colSpan={5}></td></tr>}
              </tbody>
            </table>
          </div>
          <h2>The miss density by decade</h2>
          <div className="scroll">
            <table>
              <thead><tr><th>decade</th><th>width</th><th>missed</th><th>miss density</th></tr></thead>
              <tbody>
                {look.report.bands.map((band) => (
                  <tr key={band.first}>
                    <td className="num">{band.first} to {band.last}</td>
                    <td className="num">{band.width}</td>
                    <td className="num">{band.missed}</td>
                    <td className="num">{band.density.toFixed(6)}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
          <h2>The tiers</h2>
          <div className="scroll">
            <table>
              <thead><tr><th>tier</th><th>rows</th><th>written</th><th>written by this tier alone</th></tr></thead>
              <tbody>
                {look.report.tiers.map((tier) => (
                  <tr key={tier.tier}>
                    <td>{tier.tier}</td>
                    <td className="num">{tier.rows}</td>
                    <td className="num">{tier.written}</td>
                    <td className="num">{tier.alone}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>
      </div>
    </Page>
  );
}

mount(<App />);
