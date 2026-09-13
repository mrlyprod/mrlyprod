import { useMemo } from 'react';
import { ready, ink, fit } from '../../lib/mrly.js';
import { mount, Page, Group, Row, Pick, Slider, Btn, Stats, Stat, Note } from '../../lib/app.jsx';
import { Sketch } from '../../lib/draw.jsx';
import { useQuery } from '../../lib/query.js';

const m = await ready();

const MENU = JSON.parse(m.radix_menu());
const KOCH = MENU.presets[0];
const PAD = 16;
const TALL = 520;

const PLACE =
  'A design places its copies by hand today: the base is a whole number `q`, the digits are cells of a `q` by `q` box, and every cell sits where its row and column put it. The place dial hands that job to a ring of the plane. Pick a ring, `Z[i]` on the square lattice or `Z[w]` on the hexagonal, pick a base `b` of norm `q = N(b)` inside it, and pick a digit `d` for some of the `q` residue classes modulo `b`. Each digit is a place map `phi_d(x) = (u_d x + d) / b`, a similarity of ratio `1 / sqrt(q)` turned by a unit `u_d`, and the design is the set those maps hold fixed. A word `d_1 ... d_L` lands on `sum_(j=1..L) (prod_(i<j) u_(d_i)) d_j b^(-j)`, which is what the dots below are.';

const GLUE =
  'Accept and place do not interfere: the words are counted before any of them is drawn, so a design of `card F` digits writes `(card F)^L` words at level `L` whatever the base and whatever the twists. What the twists can do is send two words to one point. That is the third slot, the glue, and it is never chosen: it is what the place maps do to the accepted words. Watch the distinct count fall below the fill and the design fold onto itself.';

const CODE =
  'A digit set is one representative per chosen class, read against the canonical system: the `q` representatives of least norm, ties broken by argument. The code below is one bit a class in that order, so it names which classes are in and never which representatives stand for them. Move a digit by a multiple of `b` and the code does not move while the design does, `phi_(d + b m)(x) = phi_d(x) + m`, so a code alone names a design only when every digit is canonical. The Koch preset and the carpet preset are both off the canonical system, which is why the chips print the digit and not only the class.';

const TWIST =
  'A twist is a unit of the ring, one per digit: four on the square lattice, six on the hexagonal, so every twist is a rotation of order `1, 2, 3, 4` or `6` and nothing else, the crystallographic restriction reading off the trace. Turning a digit keeps the word count and the similarity ratio and moves only where its copy lands, which is the whole difference between a blob and a curve: the Koch preset is the four-digit design at base `3` on the hexagonal lattice with two of its copies turned, one by a sixth of a turn and one back.';

const attempt = (fn) => {
  try {
    return { ...fn(), error: null };
  } catch (error) {
    return { error };
  }
};

const symbol = (ring) => (ring === 'gaussian' ? 'i' : 'w');

const spell = (z, ring) => {
  if (z.c === 0) return `${z.a}`;
  const tail = Math.abs(z.c) === 1 ? symbol(ring) : `${Math.abs(z.c)}${symbol(ring)}`;
  if (z.a === 0) return z.c < 0 ? `-${tail}` : tail;
  return `${z.a} ${z.c < 0 ? '-' : '+'} ${tail}`;
};

const write = (list) => [list.map((d) => `${d.a}:${d.c}`).join('_'), list.map((d) => d.u).join('_')];

const residues = (ring, a, c) => JSON.parse(m.radix_read(ring, a, c, '0:0', '0', 1)).residues;

const full = (ring, a, c) => write(residues(ring, a, c).map((z) => ({ ...z, u: 0 })));

function App() {
  const [q, set] = useQuery({ ring: KOCH.ring, a: KOCH.a, c: KOCH.c, d: KOCH.digits, t: KOCH.twists, level: KOCH.level, line: KOCH.line });

  const cap = useMemo(() => attempt(() => ({ cap: m.radix_cap(q.d) })), [q.d]);
  const level = Math.max(1, Math.min(cap.cap ?? 1, Math.round(q.level) || 1));
  const read = useMemo(() => attempt(() => ({ card: JSON.parse(m.radix_read(q.ring, q.a, q.c, q.d, q.t, level)) })), [q.ring, q.a, q.c, q.d, q.t, level]);
  const drawn = useMemo(() => attempt(() => ({ points: m.radix_points(q.ring, q.a, q.c, q.d, q.t, level) })), [q.ring, q.a, q.c, q.d, q.t, level]);

  const card = read.card;
  const points = drawn.points;
  const ringCard = MENU.rings.find((row) => row.name === q.ring);

  const chosen = card ? card.digits.map((d, i) => ({ a: d.a, c: d.c, u: card.twists[i], cls: d.class })) : [];

  const lay = (list) => {
    const sorted = [...list].sort((one, two) => one.cls - two.cls);
    const [d, t] = write(sorted);
    set({ d, t, level: sorted.length ? Math.min(level, m.radix_cap(d)) : level });
  };

  const toggle = (index) => {
    if (chosen.some((d) => d.cls === index)) lay(chosen.filter((d) => d.cls !== index));
    else lay([...chosen, { ...card.residues[index], u: 0, cls: index }]);
  };

  const turn = (index, unit) => lay(chosen.map((d) => (d.cls === index ? { ...d, u: unit } : d)));

  const rebase = (ring, a, c) => {
    const [d, t] = full(ring, a, c);
    set({ ring, a, c, d, t, level: Math.min(level, m.radix_cap(d)) });
  };

  const preset = (name) => {
    const row = MENU.presets.find((one) => one.name === name);
    if (row) set({ ring: row.ring, a: row.a, c: row.c, d: row.digits, t: row.twists, level: row.level, line: row.line });
  };

  const draw = (canvas) => {
    const [ctx, w, h] = fit(canvas, TALL);
    ctx.clearRect(0, 0, w, h);
    if (!points || points.length < 2) return;
    let [lowX, lowY, highX, highY] = [Infinity, Infinity, -Infinity, -Infinity];
    for (let i = 0; i < points.length; i += 2) {
      lowX = Math.min(lowX, points[i]);
      highX = Math.max(highX, points[i]);
      lowY = Math.min(lowY, points[i + 1]);
      highY = Math.max(highY, points[i + 1]);
    }
    const span = Math.max(highX - lowX, highY - lowY, 1e-9);
    const scale = Math.min(w - 2 * PAD, h - 2 * PAD) / span;
    const X = (x) => w / 2 + (x - (lowX + highX) / 2) * scale;
    const Y = (y) => h / 2 - (y - (lowY + highY) / 2) * scale;
    const count = points.length / 2;
    ctx.strokeStyle = ink.blue;
    ctx.fillStyle = ink.blue;
    if (q.line) {
      ctx.lineWidth = count > 4096 ? 0.6 : 1.2;
      ctx.lineJoin = 'round';
      ctx.beginPath();
      ctx.moveTo(X(points[0]), Y(points[1]));
      for (let i = 2; i < points.length; i += 2) ctx.lineTo(X(points[i]), Y(points[i + 1]));
      ctx.stroke();
      return;
    }
    const dot = Math.max(1, Math.min(5, Math.round(0.6 * scale * span / Math.sqrt(count))));
    for (let i = 0; i < points.length; i += 2) ctx.fillRect(X(points[i]) - dot / 2, Y(points[i + 1]) - dot / 2, dot, dot);
  };

  const controls = (
    <>
      <Group name="The design">
        <Pick label="preset" value="" options={[['', 'pick one'], ...MENU.presets.map((row) => [row.name, row.label])]} onChange={preset} />
        <Pick label="ring" value={q.ring}
          options={MENU.rings.map((row) => [row.name, row.name === 'gaussian' ? 'Z[i], the square lattice' : 'Z[w], the hexagonal lattice'])}
          onChange={(name) => { const row = MENU.rings.find((one) => one.name === name); rebase(name, row.bases[0].a, row.bases[0].c); }} />
        <Pick label="base b" value={`${q.a}:${q.c}`}
          options={(ringCard?.bases ?? []).map((b) => [`${b.a}:${b.c}`, `${spell(b, q.ring)}, norm ${b.norm}`])}
          onChange={(value) => { const [a, c] = value.split(':').map(Number); rebase(q.ring, a, c); }} />
      </Group>
      <Group name="The depth">
        <Slider label={`level L to ${cap.cap ?? 1}`} value={level} min={1} max={cap.cap ?? 1} onChange={(value) => set({ level: value })} />
        <Pick label="drawn as" value={q.line ? 'line' : 'dots'} options={[['dots', 'dots, one a word'], ['line', 'a polyline through the words']]}
          onChange={(value) => set({ line: value === 'line' })} />
      </Group>
      <Group name="The digits">
        <Btn onClick={() => rebase(q.ring, q.a, q.c)}>Every residue</Btn>
        <Btn onClick={() => lay(chosen.map((d) => ({ ...d, u: 0 })))}>Drop the twists</Btn>
      </Group>
    </>
  );

  return (
    <Page crumb="radix" title="The place dial"
      sub="A design writes words of digits and then has to put them somewhere. Today the somewhere is a box of cells at a whole-number scale. Turn the place dial and the scale becomes an element of a ring of the plane, the digits become residues modulo that element, and each one carries a unit twist: the word count never moves, the picture becomes a dragon, a gasket, a snowflake or a tile, and two words can land on one point."
      controls={controls}
      foot={<>Every residue, count, dimension and point on this page comes from the crates through wasm; the page only draws. The level is capped so one drawing stays under 2^16 points. The preset names are the standard ones of the literature: what the crate pins is the arithmetic of each quintuple, never the naming. Nearby: <a href="../memory">the memory dial</a> thins the words the same design accepts, <a href="../gaussian">the plane primes</a> walks the same two rings, <a href="../tile">the tile</a> is what the untwisted whole-number base does.</>}>

      <div className="panel">
        <h2>the design at level {level} <span>{card ? `${card.fill} words, ${card.distinct} distinct points` : 'the place maps at work'}</span></h2>
        <Note error={cap.error ?? read.error ?? drawn.error} />
        <Sketch draw={draw} deps={[points, q.line]} role="img" aria-label={`the level ${level} points of the radix design at base ${q.a}, ${q.c}`} />
        {card ? (
          <Stats>
            <Stat label="base">{`${spell({ a: card.a, c: card.c }, card.ring)}, norm ${card.q}`}</Stat>
            <Stat label="card F">{card.size}</Stat>
            <Stat label="fill">{card.fill}</Stat>
            <Stat label="distinct">{card.distinct}</Stat>
            <Stat label="dimension">{card.dimension.toFixed(6)}</Stat>
            <Stat label="code">{card.code}</Stat>
            <span className="chip proved">the fill is `card F ^ L` at every level, twists and all</span>
            {card.glued ? <span className="chip verified">the place maps glue: fewer points than words</span> : <span className="chip verified">no glue: one point a word</span>}
          </Stats>
        ) : null}
        <p className="sub">{PLACE}</p>
      </div>

      <div className="arena">
        <div className="panel">
          <h2>the digits <span>{card ? `${card.size} of the ${card.q} classes modulo the base` : 'one representative a class'}</span></h2>
          {card ? (
            <div className="ribbon tight">
              {card.residues.map((z, index) => {
                const held = chosen.find((d) => d.cls === index);
                return (
                  <span key={index} role="button" tabIndex={0} className={held ? 'yellow' : undefined}
                    onClick={() => toggle(index)} onKeyDown={(event) => { if (event.key === 'Enter') toggle(index); }}>
                    <b>{spell(held ?? z, card.ring)}</b>
                  </span>
                );
              })}
            </div>
          ) : null}
          {card ? (
            <Row>
              {chosen.map((d) => (
                <Pick key={d.cls} label={`twist on ${spell(d, card.ring)}`} value={String(d.u)}
                  options={card.units.map((u, index) => [String(index), spell(u, card.ring)])}
                  onChange={(value) => turn(d.cls, Number(value))} />
              ))}
            </Row>
          ) : null}
          {card ? (
            <Stats>
              <Stat label="canonical">{card.canonical ? 'every digit' : 'not every digit'}</Stat>
              <span className={`chip ${card.canonical ? 'verified' : 'proved'}`}>
                {card.canonical ? 'every digit is the least-norm representative of its class, so the code names this design' : 'a digit is off the canonical system, so the code names the classes and not this design'}
              </span>
            </Stats>
          ) : null}
          <p className="sub">{CODE}</p>
        </div>

        <div className="panel">
          <h2>the twists <span>{card ? `${card.units.length} units on this lattice` : 'a rotation a digit'}</span></h2>
          <p className="sub">{TWIST}</p>
          <p className="sub">{GLUE}</p>
        </div>
      </div>
    </Page>
  );
}

mount(<App />);
