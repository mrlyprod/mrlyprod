import { useMemo } from 'react';
import { ready, ink } from '../../../lib/mrly.js';
import { useQuery } from '../../../lib/query.js';
import { demo, Page, Row, Pick, Slider, Btn, Stats, Stat, Note } from '../../../lib/app.jsx';
import { Grid, Sketch } from '../../../lib/draw.jsx';
import { Picker, useSeeds, seeded } from '../../../lib/select.jsx';
import { board, line, axis, tag } from '../../../lib/chart.js';

const m = await ready();
const DIMS = [[1, 'dim 1, the line'], [2, 'dim 2, the square'], [3, 'dim 3, the cube']];
const START = { 1: '1', 2: '7', 3: '23' };
const SIDES = [['odd', 'odd sides 3, 5, 7, ...'], ['even', 'even sides 2, 4, 6, ...']];
const HUES = ['blue', 'orange', 'green', 'pink'];
const LETTERS = 12;
const REACH = 6;
const STOPS = 240;

const attempt = (fn) => {
  try {
    return { value: fn(), error: null };
  } catch (error) {
    return { value: null, error };
  }
};

const ratio = ([p, q]) => (q === 1 ? String(p) : `${p}/${q}`);
const digits = (i, dim) => i.toString(2).padStart(dim, '0');
const ones = (i) => [...i.toString(2)].filter((c) => c === '1').length;
const fixed = (value, places = 12) => (value === null || value === undefined ? 'none' : value.toFixed(places));

function roots(list) {
  const out = [];
  for (const [re, im] of list) {
    if (im > 0) continue;
    out.push(im < 0 ? `${re.toFixed(6)} +- ${(-im).toFixed(6)}i` : re.toFixed(6));
  }
  return out.join(', ');
}

function points(walk, reach, at) {
  const out = [];
  for (let i = 0; i < walk.length; i += 4) out.push([Math.log10(walk[i]) / reach, walk[i + at]]);
  return out;
}

function span(values, extra) {
  let low = Math.min(...values, ...extra);
  let high = Math.max(...values, ...extra);
  if (high - low < 1e-9) [low, high] = [low - 1, high + 1];
  const pad = (high - low) * 0.08;
  return [low - pad, high + pad];
}

function plot(b, series, low, high) {
  for (const [list, color, options] of series) {
    line(b, list.filter(([, y]) => y >= low && y <= high).map(([x, y]) => [x, (y - low) / (high - low)]), color, options);
  }
}

function decades(reach) {
  return Array.from({ length: reach + 1 }, (_, k) => [k / reach, k === 0 ? '1' : `10^${k}`]);
}

function App() {
  const seeds = useSeeds();
  const first = useMemo(() => {
    const asked = +(new URLSearchParams(location.search).get('dim') ?? 2);
    const dim = START[asked] ? asked : 2;
    return { dim, code: seeded(seeds, dim, 2, START[dim]), letters: 3, reach: REACH - 1, sides: 'odd' };
  }, []);
  const [q, set] = useQuery(first);
  const dim = START[q.dim] ? q.dim : 2;
  const even = q.sides === 'even';
  const code = q.code.trim();
  const letters = Math.max(1, Math.min(LETTERS, q.letters));
  const reach = Math.max(1, Math.min(REACH, q.reach));
  const cap = useMemo(() => m.staircase_cap(even), [even]);
  const drawn = Math.min(letters, cap);

  const read = useMemo(() => attempt(() => JSON.parse(m.staircase_read(code, dim, letters, even))), [code, dim, letters, even]);
  const walk = useMemo(() => attempt(() => m.staircase_walk(code, dim, 10 ** reach, STOPS, even)), [code, dim, reach, even]);
  const picture = useMemo(() => attempt(() => m.staircase_grid(code, dim, drawn, even)), [code, dim, drawn, even]);
  const r = read.value;
  const w = walk.value;
  const last = w && w.length ? { level: w[w.length - 4], value: w[w.length - 3], law: w[w.length - 2], scaled: w[w.length - 1] } : null;
  const target = r ? (even ? w?.[1] : r.constant) : null;

  const toggle = (i) => {
    const next = attempt(() => (BigInt(code) ^ (1n << BigInt(i))).toString()).value;
    if (next === null) return;
    seeds.drop();
    set({ code: next });
  };

  const shift = (value) => {
    seeds.drop();
    set({ dim: +value, code: START[+value] });
  };

  const settle = (canvas) => {
    const b = board(canvas, 260, { top: 30, bottom: 24, left: 18, right: 18 });
    if (!r || !w) return;
    const walked = points(w, reach, 1);
    const law = points(w, reach, 2);
    const [low, high] = span(walked.map(([, y]) => y), [target]);
    const level = (target - low) / (high - low);
    line(b, [[0, level], [1, level]], ink.green, { width: 1, dash: [5, 4] });
    plot(b, [[law, ink.orange, { width: 1.5, dash: [2, 3] }], [walked, ink.blue, { width: 2 }]], low, high);
    axis(b, decades(reach));
    tag(b, `${even ? 'the walk, constant ' : 'C = '}${fixed(target)}`, ink.green, 'right', b.x(1), Math.max(b.roof + 10, Math.min(b.floor - 4, b.y(level) - 6)));
    tag(b, even ? 'R_L (2^dim/w)^L on even sides' : 'renormalised fill; dotted, the law C (1 + c_1/L)', ink.dim);
  };

  const correction = (canvas) => {
    const b = board(canvas, 220, { top: 30, bottom: 24, left: 18, right: 18 });
    if (!r || !w) return;
    const scaled = points(w, reach, 3);
    const marks = even ? [] : [r.c1];
    const tail = reach > 1 ? scaled.filter(([x]) => x >= 1 / reach) : scaled;
    const [low, high] = span(tail.map(([, y]) => y), marks);
    if (!even) {
      const level = (r.c1 - low) / (high - low);
      line(b, [[0, level], [1, level]], ink.orange, { width: 1, dash: [5, 4] });
      tag(b, `c_1 = ${ratio(r.correction)} = ${fixed(r.c1, 6)}`, ink.orange, 'right', b.x(1), Math.max(b.roof + 10, Math.min(b.floor - 4, b.y(level) - 6)));
    }
    plot(b, [[scaled, ink.blue, { width: 2 }]], low, high);
    axis(b, decades(reach));
    tag(b, even ? 'L times the gap to the constant: zero at every level' : 'L times the relative gap to C', ink.dim);
  };

  const controls = (
    <Row>
      <Pick label="dim" value={dim} options={DIMS} onChange={shift} />
      <Picker dimension={dim} code={q.code} onChange={(v) => set(v)} seeds={seeds} />
      <Pick label="sides" value={q.sides} options={SIDES} onChange={(v) => set({ sides: v })} />
      <Slider label="letters" value={letters} min={1} max={LETTERS} onChange={(v) => set({ letters: v })} />
      <Slider label="walk to" value={reach} min={1} max={REACH} show={`10^${reach}`} onChange={(v) => set({ reach: v })} />
    </Row>
  );

  const corners = Array.from({ length: 2 ** dim }, (_, i) => i);
  const filled = (i) => attempt(() => (BigInt(code) >> BigInt(i)) & 1n).value === 1n;
  const rows = r ? r.levels : [];
  const word = rows.length ? rows[drawn - 1].word : null;

  return (
    <Page title="Pi on the staircase"
      sub={<>A design is a set of corners of the square, the line or the cube. Draw it at side 3: a cell is filled when the parities of its coordinates name a filled corner. Then put a copy of the side-5 drawing into every filled cell, then side 7, one letter a level. Each letter keeps about <code>w/2^dim</code> of its box, plus or minus a little. Divide out <code>(w/2^dim)^L</code> and the slow power <code>L^drift</code>, and what is left settles on a constant, often pi in disguise: <code>pi/4</code>, <code>cosh(pi/2)/2</code>, <code>3 pi/(4 Gamma(1/3))</code>. Click corners and watch the curve land.</>}
      foot={<>A corner with <code>j</code> odd coordinates fills <code>n^(dim-j) (n-1)^j</code> cells of side <code>2n - 1</code>, since each axis holds <code>n</code> even and <code>n - 1</code> odd positions; so a design with <code>a_j</code> such corners fills <code>P_F(n) = sum_j a_j n^(dim-j) (n-1)^j</code>, a polynomial whose roots <code>r_i</code> decide everything. The row word of sides <code>3, 5, ..., 2L+1</code> has fill ratio exactly <code>(w/2^dim)^L prod_i Gamma(L+2-r_i)/Gamma(2-r_i) / (Gamma(L+3/2)/Gamma(3/2))^dim</code>, hence <code>(w/2^dim)^L L^drift C (1 + c_1/L + O(L^-2))</code> with <code>drift = dim/2 - mean</code>, <code>C = Gamma(3/2)^dim / prod_i Gamma(2 - r_i)</code> and <code>c_1 = dim/8 + drift - var/2</code>, mean and var of the odd count over the corners. The drift is 0 exactly when the corners hold as many odd coordinates as even ones. At even side every letter fills <code>w/2^dim</code> on the nose. The two parity designs fill <code>(N^dim -+ 1)/2</code> at odd side <code>N</code>, so their constants are the Wallis sieve products <code>prod_(N odd &gt;= 3) (1 -+ N^-dim)</code>. Where the roots outside <code>0, 1/2, 1</code> pair as <code>r, 1 - r</code>, <code>Gamma(z) Gamma(1 - z) = pi / sin(pi z)</code> turns <code>C</code> into <code>(sqrt(pi)/2)^(m_0 + m_1) prod_pairs sin(pi r)/(4 r (1-r))</code>, and a design times its mirror always pairs. The proofs are on <a href="/research/notes/pi/">pi</a> and <a href="/research/notes/magic/">magic</a>. The Gamma form, the closed form, the reflection, the Wallis product and every walk are crate calls through wasm; the page only draws.</>}
      controls={controls}>
      <div className="arena">
        <div className="panel">
          <h2>The corners <span>click one to fill or empty it</span></h2>
          <div className="ribbon">
            {corners.map((i) => (
              <span key={i} role="button" tabIndex={0} aria-pressed={filled(i)} className={filled(i) ? undefined : 'dim'}
                style={filled(i) ? { color: `var(--${HUES[ones(i)]})`, borderColor: `var(--${HUES[ones(i)]})` } : undefined}
                onClick={() => toggle(i)} onKeyDown={(event) => { if (event.key === 'Enter') toggle(i); }}>
                <b>{digits(i, dim)}</b><i>{filled(i) ? `in, ${ones(i)} odd` : 'out'}</i>
              </span>
            ))}
          </div>
          {r ? (
            <Stats>
              <Stat label="corners">{`${r.corners} of ${r.box}`}</Stat>
              <Stat label="by odd count a_j">{`(${r.profile.join(', ')})`}</Stat>
              <Stat label="coordinates even, odd">{`${r.coordinates[0]}, ${r.coordinates[1]}`}</Stat>
              <Stat label="drift">{even ? 'none on even sides' : ratio(r.drift)}</Stat>
              <Stat label="roots of P_F">{roots(r.roots)}</Stat>
            </Stats>
          ) : null}
          <Note error={read.error} />
        </div>

        <div className="panel">
          <h2>The word <span>{dim === 3 ? `its floor layer, the plane word of code ${r?.floor ?? ''}` : dim === 1 ? 'one band per prefix, each at the full side' : `the first ${drawn} letters`}</span></h2>
          {picture.value ? <Grid grid={picture.value} on={ink.blue} role="img" aria-label="The first letters of the row word, filled cells inked" /> : null}
          <p className="sub">{word ? `Sides ${rows.slice(0, drawn).map((row) => row.side).join(' x ')} = ${word[0]}: ${word[1]} of ${word[2]} cells filled.${letters > cap ? ` The picture stops at ${cap} letters; the table runs on.` : ''}` : ''}</p>
          <Note error={picture.error} />
        </div>
      </div>

      <div className="arena" style={{ marginTop: 22 }}>
        <div className="panel">
          <h2>The settle <span>{even ? 'even sides: nothing to divide out' : `the renormalised fill walked to L = 10^${reach}`}</span></h2>
          <Sketch draw={settle} deps={[r, w, reach, even]} className="bars" role="img" aria-label="The renormalised fill against the level on a log scale, settling on its constant" />
          {last && r ? (
            <Stats>
              <Stat label={`at L = ${last.level}`}>{fixed(last.value)}</Stat>
              <Stat label={even ? 'the even walk' : 'law C (1 + c_1/L)'}>{fixed(last.law)}</Stat>
              <Stat label="L times the gap">{fixed(last.scaled, 6)}</Stat>
            </Stats>
          ) : null}
          <Note error={walk.error} />
        </div>
        <div className="panel">
          <h2>The correction <span>{even ? 'the even word has none' : 'the gap closes like c_1/L'}</span></h2>
          <Sketch draw={correction} deps={[r, w, reach, even]} className="bars" role="img" aria-label="The level times the relative gap to the constant, settling on the first correction" />
          <p className="sub">{even
            ? `On even sides every letter fills exactly w/2^dim of its box, so the walk stands at ${fixed(w?.[1] ?? null)} from the first letter: no drift and no constant. Switch back to odd sides to bring both back.`
            : r ? `Multiply the gap by L and it flattens onto c_1 = dim/8 + drift - var/2 = ${ratio(r.correction)}: the constant is checked against the walk, not fitted to it.` : ''}</p>
        </div>
      </div>

      {r ? (
        <div className="arena" style={{ marginTop: 22 }}>
          <div className="panel">
            <h2>The constant <span>{`code ${r.code}, dim ${r.dimension}${even ? ', on odd sides' : ''}`}</span></h2>
            <Stats>
              <Stat label="Gamma form at the roots">{fixed(r.constant)}</Stat>
              <Stat label="closed form">{r.closed}</Stat>
              <Stat label="by reflection alone">{r.reflection === null ? 'does not reduce' : fixed(r.reflection)}</Stat>
              {r.parity ? <Stat label={`Wallis sieve, prod_(N odd >= 3) (1 ${r.parity.odd ? '-' : '+'} N^-${r.dimension})`}>{fixed(r.parity.value)}</Stat> : null}
            </Stats>
            <p className="sub">{r.reflection === null
              ? 'A root outside 0, 1/2 and 1 has no partner 1 - r, so a Gamma value stays in the constant.'
              : 'Every root outside 0, 1/2 and 1 pairs with 1 - r, and the reflection formula turns each pair into sin(pi r)/(4 r (1-r)), so no Gamma value is left.'}
              {r.parity ? ` This is a parity design: at odd side N it fills (N^dim ${r.parity.odd ? '-' : '+'} 1)/2, half a Wallis sieve letter, so its constant is a Wallis sieve product.` : ''}</p>
          </div>
          <div className="panel">
            <h2>The mirror <span>flip every coordinate</span></h2>
            <Stats>
              <Stat label="mirror code">{r.mirror.code}</Stat>
              <Stat label="its closed form">{r.mirror.closed}</Stat>
              <Stat label="its constant">{fixed(r.mirror.constant)}</Stat>
              <Stat label="the two constants multiplied">{fixed(r.mirror.both)}</Stat>
              <Stat label="prod sin(pi r)/(4 r (1-r))">{fixed(r.mirror.product)}</Stat>
            </Stats>
            <p className="sub">The mirror's roots are the <code>1 - r_i</code>, so a design times its mirror pairs every root and reduces by reflection, <code>pi/4</code> at a root 0 or 1.</p>
            <Row><Btn onClick={() => { seeds.drop(); set({ code: r.mirror.code }); }}>Load the mirror</Btn></Row>
          </div>
        </div>
      ) : null}

      {r ? (
        <div className="panel" style={{ marginTop: 22 }}>
          <h2>The letters <span>{even ? 'every share is w/2^dim' : 'each share is P_F(n) over side^dim'}</span></h2>
          <div className="scroll">
            <table>
              <thead><tr><th>L</th><th>side</th><th>fill</th><th>of</th><th>share</th><th>R_L</th><th>{even ? 'R_L (2^dim/w)^L' : 'renormalised'}</th><th>{even ? 'the even walk' : 'C (1 + c_1/L)'}</th></tr></thead>
              <tbody>
                {rows.map((row) => (
                  <tr key={row.level}>
                    <td className="num">{row.level}</td>
                    <td className="num">{row.side}</td>
                    <td className="num">{row.fill}</td>
                    <td className="num">{row.cells}</td>
                    <td className="num">{row.share.toFixed(6)}</td>
                    <td className="num">{row.ratio.toExponential(6)}</td>
                    <td className="num">{fixed(row.settle, 9)}</td>
                    <td className="num">{fixed(row.law, 9)}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>
      ) : null}
    </Page>
  );
}

export const { mount, unmount } = demo(<App />);
