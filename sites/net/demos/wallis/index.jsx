import { useMemo, useRef, useState } from 'react';
import { ready, ink } from '../../lib/mrly.js';
import { faces } from '../../lib/stage.js';
import { mount, Page, Pick, Slider, Check, Stats, Stat, Note, Group } from '../../lib/app.jsx';
import { Grid, Sketch } from '../../lib/draw.jsx';
import { Stage } from '../../lib/stage.jsx';
import { useQuery } from '../../lib/query.js';
import { board, line, axis, tag } from '../../lib/chart.js';

const m = await ready();
const KINDS = [['odd', 'the odd letters 3, 5, 7, ...'], ['flat', 'one letter over and over']];
const LETTERS = [[3, 3], [5, 5], [7, 7], [9, 9]];
const DIMS = [[2, 'the plane'], [3, 'the cube']];
const STOPS = 48;
const SOLID = JSON.parse(m.wallis_read('odd', 3, 2, 3)).limit;
const SIDES = JSON.parse(m.wallis_read('odd', 3, 4, 2)).levels.map((row) => row.side).join(', ');

const gauge = (walk, limit, level) => (canvas) => {
  const b = board(canvas, Math.max(160, Math.min(260, canvas.clientWidth / 3.2)), { top: 22, bottom: 22 });
  const lo = Math.min(limit, ...walk), hi = Math.max(limit, ...walk);
  const pad = (hi - lo) * 0.09 || 0.05;
  const norm = (v) => (v - lo + pad) / (hi - lo + 2 * pad);
  axis(b, [[0, 'level 1'], [1, `level ${walk.length}`]]);
  line(b, [[0, norm(limit)], [1, norm(limit)]], ink.dim, { width: 1, dash: [4, 4] });
  tag(b, `limit ${limit.toFixed(9)}`, ink.dim);
  const at = (i) => [i / (walk.length - 1), norm(walk[i])];
  line(b, walk.map((_, i) => at(i)), ink.blue, { width: 1.6 });
  line(b, [at(level - 1)], ink.yellow, { dots: 3.5 });
  tag(b, walk[level - 1].toFixed(9), ink.yellow, 'right');
};

function App() {
  const [q, set] = useQuery({ kind: 'odd', letter: 3, dim: 2, level: 3 });
  const [spin, setSpin] = useState(true);
  const live = useRef(null);

  const data = useMemo(() => {
    const out = {};
    try {
      out.top = m.wallis_cap(q.kind, q.letter, q.dim);
      out.level = Math.min(Math.max(q.level, 1), out.top);
      out.read = JSON.parse(m.wallis_read(q.kind, q.letter, out.level, q.dim));
      out.walk = Array.from(m.wallis_walk(q.kind, q.letter, q.dim, STOPS));
      if (q.dim === 2) out.grid = m.wallis_grid(q.kind, q.letter, out.level);
      else out.mesh = m.wallis_faces(q.kind, q.letter, out.level);
    } catch (error) {
      out.error = error;
    }
    return out;
  }, [q.kind, q.letter, q.dim, q.level]);

  const read = data.read ?? {};
  const level = data.level ?? 1;
  const word = (read.word ?? []).join(', ');
  const solid = q.dim === 3;
  const measure = solid ? 'volume' : 'area';

  const turn = (on) => {
    setSpin(on);
    if (live.current) live.current.spin = on ? 0.004 : 0;
  };

  const controls = (
    <>
      <Group name="Schedule">
        <Pick label="letters" value={q.kind} options={KINDS} onChange={(v) => set({ kind: v })} />
        {q.kind === 'flat' && <Pick label="letter" value={q.letter} options={LETTERS} onChange={(v) => set({ letter: +v })} />}
        <Pick label="dimension" value={q.dim} options={DIMS} onChange={(v) => set({ dim: +v, level: Math.min(q.level, 3) })} />
        <Slider label="level" value={level} min={1} max={data.top ?? 1} onChange={(v) => set({ level: v })} />
      </Group>
      {solid && (
        <Group name="View">
          <Check label="spin" checked={spin} onChange={turn} />
        </Group>
      )}
    </>
  );

  return (
    <Page crumb="wallis" title="The Wallis sieve buys area by growing its letters" controls={controls}
      foot={<>The curve walks {STOPS} levels of the same schedule, past the level the picture holds; the dashed line is the limit and the yellow dot is the level on screen. Every number comes from one call into the crate, never from the raster: the side is the product of the letters, the surviving cells the product of the letters' fills, and the share of the whole the product of one minus each letter's inverse site count, so the picture is drawn to be looked at and never counted. The plane limit is Wallis, <b>Proved</b>. The cube limit is <b>Proved</b> as well: writing m = 2k + 1 and factoring m^3 - 1 over the three cube roots of one turns the product into a ratio of three rising shifts whose sum is exactly three times the one below, so the gamma ratio in the tail tends to one and the product closes as pi^(3/2) / (8 |Gamma(7/4 - i sqrt(3)/4)|^2) = {SOLID.toFixed(9)}, which the crate pins against the truncated product and splits again as cosh(pi sqrt(3) / 2) / (3 pi) over the even product, the ratio form. A hexagon does not divide into smaller hexagons, so the sieve has no hexagonal analogue here; the hexagon this tree owns is the diagonal slice of a cube meshed into triangles, not a self-similar tile, and no hex sieve is drawn without a crate function behind it. Nearby: <a href="../words">the words</a> folds one design into another by the same Kronecker product, <a href="../formulas">the formulas</a> walks the other Wallis product on its dial, <a href="../tower">the tower</a> lets the letter change with the scale along one axis, and <a href="/research/pi/">pi out of the stack</a> counts pi a second way.</>}
      sub={`Level k cuts every surviving square into (2k + 1)^2 equal squares and drops the centre one, so the sides run ${SIDES} and the surviving area is the product of one minus one over the odd squares, which is Wallis' product for pi over four. Read as a word, letter k is the punctured tile of side 2k + 1, the square with its centre cell removed, and the schedule folds those letters by the Kronecker product, so the word's fill is the product of the letters' fills exactly. Hold the letter still instead and the same machine draws the Sierpinski carpet, whose ratio never changes, whose area falls to nothing, and which buys a dimension of log 8 over log 3 in exchange.`}>
      {q.dim === 2 && data.grid && <Grid grid={data.grid} on={ink.yellow} role="img" aria-label="The plane sieve" />}
      <Stage hidden={!solid} role="img" aria-label="The punctures of the solid sieve" deps={[data]} onStage={(st) => {
        live.current = st;
        st.spin = spin ? 0.004 : 0;
        if (data.mesh) st.show(faces(data.mesh, ink.blue));
        else st.clear();
      }} />
      <Stats>
        <Stat label="word">{word}</Stat>
        <Stat label="side">{read.side}</Stat>
        <Stat label="cells">{read.cells}</Stat>
        <Stat label="punctures">{read.holes}</Stat>
        <Stat label="box exponent">{read.exponent?.toFixed(6)}</Stat>
      </Stats>
      <Stats>
        <Stat label={measure}>{read.ratio?.toFixed(9)}</Stat>
        <Stat label="limit">{read.closed ? read.limit?.toFixed(9) : 'zero'}</Stat>
        <Stat label="gap">{read.gap?.toFixed(9)}</Stat>
        <Stat label="in closed form">{read.closed ? (solid ? 'pi^(3/2) / (8 |Gamma(7/4 - i sqrt(3)/4)|^2)' : 'pi / 4') : 'zero, geometrically'}</Stat>
      </Stats>
      {(read.levels ?? []).map((row) => (
        <Stats key={row.level}>
          <Stat label="level">{row.level}</Stat>
          <Stat label="letter">{row.letter}</Stat>
          <Stat label="side">{row.side}</Stat>
          <Stat label="cells">{row.cells}</Stat>
          <Stat label={measure}>{row.ratio.toFixed(9)}</Stat>
        </Stats>
      ))}
      {data.walk && <Sketch draw={gauge(data.walk, read.closed ? read.limit : 0, level)} deps={[data.walk, level, read.limit]} />}
      <Note error={data.error} />
    </Page>
  );
}

mount(<App />);
