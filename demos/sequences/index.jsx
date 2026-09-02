import { Fragment, useEffect, useMemo, useReducer, useRef, useState } from 'react';
import { ready, ink } from '../lib/mrly.js';
import { stamp, useQuery } from '../lib/query.js';
import { mount, Page, Row, Pick, Btn, Stats, Stat, Note } from '../lib/app.jsx';
import { Grid, Markup, Sketch } from '../lib/draw.jsx';
import { useSeeds, roll } from '../lib/select.jsx';
import { board, bars, axis, tag } from '../lib/chart.js';
import { Pins, Staircase, Ratios, Differences, Digits, Terms } from '../lib/series.jsx';

const m = await ready();
const TERMS = 8;
const STEP = 4;
const DEPTH = 6;
const BUDGET = '500000';
const TASTE = '20000';
const CHUNK = 50;
const DIMS = [1, 2, 3, 4];
const BASES = [2, 3, 4, 5];
const RECORDS = JSON.parse(m.ledger_records());
const MEASURES = [...m.ledger_measures()];
const BUILT = m.ledger_build('closed', TERMS);

function chip(text) {
  return text ? <span className={`chip ${text.toLowerCase()}`}>{text}</span> : '';
}

function badge(row) {
  if (!row.oeis) return '';
  return (
    <>
      <a className="badge" href={`https://oeis.org/${row.oeis}`} target="_blank" rel="noopener">{row.oeis}</a>
      {row.shift ? <> <span className="shift">{row.shift > 0 ? '+' : ''}{row.shift}</span></> : null}
    </>
  );
}

function read(code, d, b, measures, cells) {
  const out = [];
  for (const measure of measures) {
    for (const way of ['level', 'side']) {
      try {
        out.push({ ...JSON.parse(m.ledger_row(code, d, b, measure, way, TERMS, cells)), extra: 'typed' });
      } catch {
        continue;
      }
    }
  }
  return out;
}

function reading(name, cells) {
  const named = name.match(/^(mrly_bang_\w+)\.(\w+)\.(\w+)$/);
  if (!named) return null;
  try {
    const bang = JSON.parse(m.name_parse(named[1]));
    return JSON.parse(m.ledger_row(bang.code, bang.dimension, bang.base, named[2], named[3], TERMS, cells));
  } catch {
    return null;
  }
}

function fits(row, f) {
  return (!f.measure || row.measure === f.measure) && (!f.d || row.d === f.d) && (!f.b || row.q === f.b);
}

function keyed(f) {
  const out = [];
  for (const found of JSON.parse(m.ledger_identify(f.q))) {
    const record = RECORDS.find((r) => r.id === found.id);
    if (!record?.key || JSON.parse(m.ledger_search(record.key, '', 0, 0, 0, 1)).total) continue;
    const row = reading(record.key, BUDGET);
    if (row && fits(row, f)) out.push({ ...row, extra: 'record' });
  }
  return out;
}

function typed(f) {
  const measures = f.measure ? [f.measure] : MEASURES;
  if (/\./.test(f.q)) {
    const row = reading(f.q, TASTE);
    return row ? [{ ...row, extra: 'typed' }] : [];
  }
  if (!/^\d+$/.test(f.q)) return [];
  const b = f.b || BASES[0];
  const dims = f.d ? [f.d] : DIMS;
  for (const d of dims) {
    try {
      m.name_of(f.q, d, b);
    } catch {
      continue;
    }
    return read(f.q, d, b, measures, TASTE);
  }
  return [];
}

function idle() {
  return new Promise((resolve) => (window.requestIdleCallback ? requestIdleCallback(resolve, { timeout: 100 }) : setTimeout(resolve, 0)));
}

function Found({ terms }) {
  const key = terms.slice(0, TERMS).join(', ');
  const found = useMemo(() => JSON.parse(m.ledger_identify(key)), [key]);
  if (!found.length) return 'no curated record holds these terms';
  return found.slice(0, 3).map((r, i) => (
    <Fragment key={r.id}>
      {i ? <br /> : null}
      <a href={`https://oeis.org/${r.id}`} target="_blank" rel="noopener">{r.id}</a> at index {r.shift}: {r.name}
    </Fragment>
  ));
}

function Design({ row }) {
  const made = useMemo(() => {
    try {
      if (row.d === 2) {
        const level = m.level_cap(row.number, 2, 60000);
        return { title: `side ${row.number}, level ${level}`, art: <Grid grid={m.two_grid(row.code, row.number, level, 0, row.q)} on={ink.blue} /> };
      }
      if (row.d === 3) {
        const level = m.level_cap(row.number, 3, 8000);
        return { title: `side ${row.number}, level ${level}, isometric`, art: <Markup svg={m.hex_svg(row.code, row.number, level, row.q, 'iso', 4)} /> };
      }
      if (row.d === 1) {
        const level = m.level_cap(row.number, 1, 729);
        const cells = m.ledger_profile(row.code, 1, row.q, row.number, level);
        return { title: `side ${row.number}, level ${level}, the strip`, art: <Grid grid={{ width: cells.length, height: 1, types: Uint8Array.from(cells) }} on={ink.blue} style={{ height: 48 }} /> };
      }
      const level = m.level_cap(row.number, row.d, 600000);
      const counts = m.ledger_profile(row.code, row.d, row.q, row.number, level).map(Number);
      const draw = (canvas) => {
        const b = board(canvas, 220);
        bars(b, counts, { color: ink.pink, inset: 0 });
        axis(b, [[0, 'first plane'], [1, 'last plane']]);
        tag(b, `cells on every diagonal plane, level ${level}`, ink.dim);
      };
      return { title: `side ${row.number}, level ${level}, the diagonal profile`, art: <Sketch className="bars" draw={draw} deps={[counts]} /> };
    } catch (error) {
      return { title: '', art: null, error };
    }
  }, [row.name]);

  return (
    <div className="panel">
      <h2>The design <span>{made.title}</span></h2>
      <div>{made.art}</div>
      <Stats>
        <Stat label="code">{row.code}</Stat>
        <Stat label="space">{`dimension ${row.d}, base ${row.q}`}</Stat>
        <Stat label="measure">{row.measure}</Stat>
        <Stat label="axis">{row.axis === 'level' ? `level L at side ${row.number}` : 'odd side 2k - 1 at level 1'}</Stat>
      </Stats>
      <Note error={made.error} />
    </div>
  );
}

const VIEWS = [['pins', 'pin plot'], ['steps', 'step function'], ['sums', 'partial sums'], ['ratios', 'ratios'], ['differences', 'difference triangle'], ['digits', 'digit heatmap']];
const BLENDED = ['ratios', 'differences'];

const bases = (q) => [...new Set([2, 3, 10, q])].sort((a, b) => a - b);

const plotted = (row) => `../plot/?code=${row.code}&dimension=${row.d}&base=${row.q}&measure=${row.measure}&axis=${row.axis}&count=${row.terms.length}`;

function View({ row, view, dig, blend }) {
  const label = `${row.axis === 'level' ? 'level L' : 'side k'} from ${row.start}`;
  if (view === 'digits') return <Digits terms={row.terms} start={row.start} base={dig} label={label} />;
  if (view === 'steps') return <Staircase terms={row.terms} start={row.start} label={label} />;
  if (view === 'sums') return <Staircase terms={row.terms} start={row.start} sums label={label} />;
  if (view === 'ratios') return blend ? <Ratios values={blend.ratios} start={row.start} label={`${label}, each term against the one before`} /> : null;
  if (view === 'differences') return blend ? <Differences rows={blend.differences} label={`${label}, each row the differences of the row above`} /> : null;
  return <Pins terms={row.terms} start={row.start} label={label} />;
}

function App() {
  const s = useSeeds();
  const [pick, set] = useQuery({ q: '', measure: '', dimension: '', base: '', rows: 25, page: '', open: '', view: 'pins', dig: 2 });
  const [count, setCount] = useState(BUILT);
  const [tier, setTier] = useState('closed');
  const [built, grew] = useReducer((x) => x + 1, 0);
  const [picked, setPicked] = useState(null);
  const [capped, setCapped] = useState(false);
  const [note, setNote] = useState(null);
  const latest = useRef(null);
  latest.current = picked;

  const view = useMemo(() => {
    const f = { q: pick.q.trim(), measure: pick.measure, d: +pick.dimension, b: +pick.base, rows: +pick.rows };
    try {
      let at = +pick.page;
      let hits = JSON.parse(m.ledger_search(f.q, f.measure, f.d, f.b, at, f.rows));
      if (at && !hits.rows.length) {
        at = 0;
        hits = JSON.parse(m.ledger_search(f.q, f.measure, f.d, f.b, at, f.rows));
      }
      const terms = /^[\d,\s-]+$/.test(f.q) && !/^\d+$/.test(f.q);
      const extra = at ? [] : terms ? keyed(f) : typed(f);
      return { f, at, hits, extra, terms, error: null };
    } catch (error) {
      return { f, at: +pick.page, hits: null, extra: [], terms: false, error };
    }
  }, [pick.q, pick.measure, pick.dimension, pick.base, pick.rows, pick.page, built]);

  const now = useRef(view);
  now.current = view;

  const blend = useMemo(() => {
    if (!picked || !BLENDED.includes(pick.view)) return null;
    try {
      return { row: JSON.parse(m.blend_series(picked.code, picked.d, picked.q, picked.measure, picked.axis, picked.terms.length, BUDGET, DEPTH)), error: null };
    } catch (error) {
      return { row: null, error };
    }
  }, [picked, pick.view]);

  const choose = (row) => {
    setPicked(row);
    latest.current = row;
    setCapped(false);
    setNote(null);
    set({ open: row.name });
  };

  const shuffle = (seed) => {
    const f = now.current.f;
    const total = JSON.parse(m.ledger_search(f.q, f.measure, f.d, f.b, 0, 1)).total;
    if (!total) return;
    const [at] = roll(seed, [[0, total - 1]]);
    const row = JSON.parse(m.ledger_search(f.q, f.measure, f.d, f.b, at, 1)).rows[0];
    set({ page: Math.floor(at / f.rows) || '' });
    choose(row);
  };

  const deeper = () => {
    const row = picked;
    setNote(null);
    try {
      const asked = row.terms.length + STEP;
      const more = m.ledger_terms(row.code, row.d, row.q, row.measure, row.axis, asked, BUDGET);
      if (more.length <= row.terms.length) {
        setPicked({ ...row, capped: true });
        setCapped(true);
      } else {
        setPicked({ ...row, terms: more, capped: more.length < asked });
      }
    } catch (error) {
      setNote(error);
    }
  };

  useEffect(() => {
    const f = now.current.f;
    stamp({ q: f.q, measure: f.measure, dimension: f.d || '', base: f.b || '', rows: f.rows, page: now.current.at || '' });
    if (pick.open && !s.get()) {
      const row = reading(pick.open, BUDGET);
      if (row) choose(row);
      else set({ open: '' });
    }
    let live = true;
    (async () => {
      for (const name of ['convolved', 'side']) {
        let state;
        do {
          await idle();
          if (!live) return;
          state = JSON.parse(m.ledger_grow(name, TERMS, CHUNK));
          setCount(state.rows);
          setTier(`${name} ${state.done} of ${state.total}`);
        } while (state.done < state.total);
        grew();
      }
      setTier('complete');
      if (s.get() && !latest.current) shuffle(s.get());
    })();
    return () => { live = false; };
  }, []);

  useEffect(() => { if (view.at !== +pick.page) set({ page: view.at || '' }); }, [view]);

  const f = view.f;
  const listed = view.error ? [] : [...view.extra, ...view.hits.rows];

  return (
    <Page crumb="sequences" title="Every sequence the designs write"
      sub="A design fills cells, and counted level by level or side by side the counts make an integer sequence: its fills, its voids, its exposed faces, the cells on its deepest diagonal plane, the pieces of its slice. This is the ledger of all of them, read live from the crates. Type a few terms and find which design writes them, or type a name, a record, or any code."
      foot={<>A sequence is one design, one measure and one axis. The level axis grows the fractal level by level at the smallest side the base allows; the side axis holds level one and widens the odd side. The name is the design's name dotted with the measure and the axis, so the sponge's surface by level is <code>mrly_bang_d3_23.surface.level</code>. The closed tier comes first, the fills, voids and exposed faces that close in a formula; the convolved tier, the diagonal profile's peak and its height count, and the side grid tier, the vertices, edges, faces, Euler characteristic and slice census read off a rendered grid, build behind the page while it stays live, and the counter says how far they are. A record badge names the OEIS entry whose terms hold the row's, and the number after it is the record's index of the row's first term less the ledger's; the status is the record's own where the record names this design, and a collision to explain where it does not. Deeper reads more terms within a budget of cells a term, and stops where the budget or the width of a number stops it. The terms of a picked row are drawn as a pin plot, a step function, its partial sums, the ratio of each term to the one before, the difference triangle or a digit heatmap in a chosen base, and the ribbon under them prints every term. The picked row opens whole in <a href="../plot">plot</a>, which reads the same key and adds the smallest linear recurrence its terms satisfy, the characteristic polynomial, the growth and a second sequence to mix in. The table lists the least code of every orbit; a typed code of any spelling, canonical or not, is read the same way and shown first, and the odd-side rows are code specific, so the orbit mates of one design read different polynomials. Every number on this page is computed in Rust; the page only draws. The ledger itself, with its closed forms and status per row, is the <a href="https://github.com/mrlyprod/mrlyprod/blob/main/research/sequences.md">sequences</a> page.</>}>
      <Row>
        <label>search <input type="text" className="wide" value={pick.q} placeholder="6, 42, 306 or carpet or A000567 or 23" onChange={(e) => { s.drop(); set({ q: e.target.value, page: '' }); }} /></label>
        <Pick label="measure" value={pick.measure} options={[['', 'any'], ...MEASURES]} onChange={(v) => set({ measure: v, page: '' })} />
        <Pick label="dimension" value={pick.dimension} options={[['', 'any'], ...DIMS]} onChange={(v) => set({ dimension: v, page: '' })} />
        <Pick label="base" value={pick.base} options={[['', 'any'], ...BASES]} onChange={(v) => set({ base: v, page: '' })} />
        <Pick label="rows" value={pick.rows} options={[10, 25, 50, 100]} onChange={(v) => set({ rows: +v, page: '' })} />
        <Btn onClick={() => shuffle(s.next())}>Randomize</Btn>
      </Row>
      <div className="panel">
        <h2>The ledger <span>{view.terms ? <Found terms={f.q.split(/[\s,]+/).filter(Boolean)} /> : f.q ? `matching "${f.q}"` : 'every row'}</span></h2>
        <Stats>
          <Stat label="rows built">{count}</Stat>
          <Stat label="tier">{tier}</Stat>
          <Stat label="hits">{view.error ? '' : view.hits.total + (view.extra.length ? ` + ${view.extra.length} ${view.extra[0].extra}` : '')}</Stat>
          <Stat label="page">{view.at + 1}</Stat>
          <span>
            <button disabled={view.at === 0} onClick={() => set({ page: (view.at - 1) || '' })}>prev</button>{' '}
            <button disabled={!!view.error || (view.at + 1) * f.rows >= view.hits.total} onClick={() => set({ page: view.at + 1 })}>next</button>
          </span>
        </Stats>
        <div className="scroll">
          <table>
            <thead><tr><th>name</th><th>first terms</th><th>closed form</th><th>record</th><th>status</th></tr></thead>
            <tbody>
              {listed.map((row, i) => {
                const shown = picked && picked.name === row.name ? picked : row;
                return (
                  <tr key={`${i}:${row.name}`} className={picked?.name === row.name ? 'on' : undefined} onClick={() => choose(row)}>
                    <td className="mono">{row.name}{row.extra ? <> {chip(row.extra)}</> : null}</td>
                    <td><Terms terms={shown.terms} capped={shown.capped} tight /></td>
                    <td className="mono">{row.closed}</td>
                    <td>{badge(row)}</td>
                    <td>{chip(row.tag)}</td>
                  </tr>
                );
              })}
            </tbody>
          </table>
        </div>
        <Note error={view.error} />
      </div>
      {picked && (
        <div className="arena">
          <Design row={picked} />
          <div className="panel">
            <h2>The terms <span>{picked.name}</span></h2>
            <Row>
              <Pick label="view" value={pick.view} options={VIEWS} onChange={(v) => set({ view: v })} />
              {pick.view === 'digits' ? <Pick label="digit base" value={pick.dig} options={bases(picked.q)} onChange={(v) => set({ dig: +v })} /> : null}
              <a className="badge" href={plotted(picked)}>draw this row in plot</a>
            </Row>
            <View row={picked} view={pick.view} dig={pick.dig} blend={blend?.row} />
            <Terms terms={picked.terms} start={picked.start} capped={picked.capped} />
            <Stats>
              <Stat label="closed form">{picked.closed || 'none known'}</Stat>
              <Stat label="record">{badge(picked) || 'none'}</Stat>
              <Stat label="status">{chip(picked.tag) || 'unmatched'}</Stat>
              <Stat label="terms">{picked.capped ? `${picked.terms.length}, to the budget` : picked.terms.length}</Stat>
              <span><button disabled={capped} onClick={deeper}>{capped ? 'at the budget' : 'deeper'}</button></span>
            </Stats>
            <p className="foot" style={{ marginTop: 12, paddingTop: 10 }}><Found terms={picked.terms} /></p>
            <Note error={note ?? blend?.error} />
          </div>
        </div>
      )}
    </Page>
  );
}

mount(<App />);
