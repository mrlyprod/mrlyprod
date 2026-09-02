import { Fragment, useEffect, useMemo, useRef } from 'react';
import { ready, ink, fit } from '../lib/mrly.js';
import { stamp, useQuery } from '../lib/query.js';
import { mount, Page, Row, Slider, Btn, Note } from '../lib/app.jsx';
import { Grid, Markup, Sketch } from '../lib/draw.jsx';
import { useSeeds, roll } from '../lib/select.jsx';
import { Pins, Terms } from '../lib/series.jsx';

const m = await ready();
const BUDGET = '500000';
const WINDOW = 6;
const NAMED = 3;
const FLAT = [['low corner', '1'], ['tree', '3'], ['carpet', '7'], ['void', '9'], ['corner and centre', '11'], ['solid', '15']];
const RECORDS = JSON.parse(m.ledger_records());
const FLATCAP = m.level_cap(3, 2, 500000);
const CUBECAP = m.level_cap(3, 3, 60000);
const CUTCAP = m.level_cap(2, 3, 500000);

const row = (code, d, measure, way, count) => JSON.parse(m.ledger_row(code, d, 2, measure, way, count, BUDGET));
const census = (code, n) => JSON.parse(m.two_census(code, n, 1, 0, 2));
const slice = (n) => JSON.parse(m.slice_census('23', n, 1, 2));
const odds = (k) => Array.from({ length: k }, (_, i) => 2 * i + 1);
const upto = (k) => Array.from({ length: k }, (_, i) => i + 1);
const least = (n) => Math.max(n, NAMED);
const item = (r, label) => ({ label, terms: r.terms, closed: r.closed, capped: r.capped, ledger: true });

function hex(number, level, view) {
  const side = number ** level;
  return m.hex_svg('23', number, level, 2, view, Math.max(1, Math.round(65 / side)));
}

function stackOf(canvas, order) {
  const [ctx, w, h] = fit(canvas, 200);
  ctx.strokeStyle = ink.pink;
  for (const [num, den, bright] of JSON.parse(m.farey(order))) {
    const x = 6 + (w - 12) * num / den;
    ctx.globalAlpha = 0.25 + 0.75 * bright / order;
    ctx.beginPath();
    ctx.moveTo(x, h - 6);
    ctx.lineTo(x, h - 6 - (h - 12) * bright / order);
    ctx.stroke();
  }
}

const CARDS = [
  {
    key: 'sides', title: 'The odd-side law', wide: true, slider: ['side up to', 4, 8, 4], show: (k) => 2 * k - 1,
    say: 'At odd side 2k - 1 an axis splits into k low and k - 1 high positions, so a flat design fills a polynomial in k, and the six designs of the plane read as the polygonal numbers: the low corner the squares, the tree the hexagonal numbers, the carpet the octagonal numbers, before any of them is a fractal.',
    build: (k) => ({
      lines: FLAT.map(([name, code]) => ({
        label: `${name}, code ${code}`,
        art: odds(k).slice(1).map((n) => <Grid key={n} grid={m.two_grid(code, n, 1, 0, 2)} on={ink.blue} className="" />),
        terms: odds(k).slice(1).map((n) => String(census(code, n).fills)),
        closed: m.ledger_closed(code, 2, 2, 'fills', 'side'),
        ledger: true,
      })),
    }),
  },
  {
    key: 'carpet', title: 'The carpet', slider: ['level', 1, FLATCAP, 3],
    say: 'Eight of nine cells survive every level, so the carpet fills 8^L, and its dimension log 8 / log 3 is this count read against the side 3^L.',
    build: (L) => ({ art: <Grid grid={m.two_grid('7', 3, L, 0, 2)} on={ink.blue} className="" />, lines: [item(row('7', 2, 'fills', 'level', least(L)), 'filled cells')] }),
  },
  {
    key: 'voids', title: 'What the carpet drops', slider: ['level', 1, FLATCAP, 3],
    say: 'The cells the carpet leaves empty are the grid less the fill, 9^L - 8^L, the powers of nine racing the powers of eight.',
    build: (L) => ({ art: <Grid grid={m.two_grid('7', 3, L, 0, 2)} on={ink.deep} off={ink.orange} className="" />, lines: [item(row('7', 2, 'voids', 'level', least(L)), 'empty cells')] }),
  },
  {
    key: 'perimeter', title: 'The perimeter of the carpet', slider: ['level', 1, FLATCAP, 3],
    say: 'Two adjacent blocks bury one edge per spanning position, so the perimeter closes as a sum of the powers 8^L and 3^L, the entry the OEIS lists for the carpet at iteration n.',
    build: (L) => ({ art: <Grid grid={m.two_grid('7', 3, L, 0, 2)} on={ink.gold} className="" />, lines: [item(row('7', 2, 'surface', 'level', least(L)), 'exposed edges')] }),
  },
  {
    key: 'sponge', title: 'The sponge', slider: ['level', 1, CUBECAP, 2],
    say: 'Twenty of twenty-seven subcubes survive every level, so the sponge fills 20^L.',
    build: (L) => ({ art: <Markup svg={hex(3, L, 'iso')} />, lines: [item(row('23', 3, 'fills', 'level', least(L)), 'filled cells')] }),
  },
  {
    key: 'surface', title: 'The surface of the sponge', slider: ['level', 1, CUBECAP, 2],
    say: 'The same burial count in three dimensions: the exposed faces are a sum of the powers 20^L and 8^L, the surface area of the stage-n Menger sponge.',
    build: (L) => ({ art: <Markup svg={hex(3, L, 'pro')} />, lines: [item(row('23', 3, 'surface', 'level', least(L)), 'exposed faces')] }),
  },
  {
    key: 'tile', title: 'The odd sponge tile', slider: ['side', 2, 8, 3], show: (k) => 2 * k - 1,
    say: 'Widen the sponge tile to odd side 2k - 1 and its fills and voids are cubics in k: the divisor count of 240^n read as a solid, and the entry this tree contributed.',
    build: (k) => ({
      art: <Markup svg={hex(2 * k - 1, 1, 'iso')} />,
      lines: [item(row('23', 3, 'fills', 'side', least(k - 1)), 'filled cells'), item(row('23', 3, 'voids', 'side', least(k - 1)), 'empty cells')],
    }),
  },
  {
    key: 'designs', title: 'How many designs', slider: ['base up to', 2, 6, 4],
    say: 'A design is a Boolean function on the corners up to the symmetries of the cube: in base 2 the count by dimension is the irreducible Boolean functions of the record, and by base the rows are the bracelets, the toroidal squares and the toroidal cubes.',
    build: (Q) => {
      const lines = [{ label: 'base 2, by dimension', terms: m.counting_sequence(4), closed: '' }];
      ['a line', 'a square', 'a cube'].forEach((name, i) => {
        const terms = [];
        for (let q = 2; q <= least(Q - 1) + 1; q++) {
          try {
            terms.push(m.baseq_sequence(q, i + 1)[i]);
          } catch {
            break;
          }
        }
        lines.push({ label: `${name}, by base from 2`, terms, closed: '', capped: terms.length < least(Q - 1) });
      });
      const art = <div className="strip">{JSON.parse(m.universe(2)).designs.map((d) => <Grid key={d.code} grid={m.two_grid(d.code, 3, 2, 0, 2)} on={ink.gold} className="" />)}</div>;
      return { art, lines };
    },
  },
  {
    key: 'classes', title: 'How many fractals', slider: ['dimension', 1, 8, 4],
    say: 'Two base-2 designs draw the same fractal exactly when they fill the same number of corners of each weight, so the distinct fractals of a dimension number the product over the weights of one more than the corners of that weight.',
    build: (D) => {
      const terms = m.classes_sequence(least(D));
      return {
        art: <Pins terms={terms} start={1} height={200} hue={ink.gold} label="by dimension" style={{ width: '100%' }} />,
        lines: [{ label: 'fill classes, by dimension', terms, closed: '' }],
      };
    },
  },
  {
    key: 'slices', title: 'The middle slice of the sponge', slider: ['level', 1, CUBECAP, 2],
    say: 'Cut the sponge on its middle diagonal plane and count the filled triangles: one index up from the star holes of the record, whose recurrence gives the slice its dimension.',
    build: (L) => ({
      art: <Markup svg={hex(3, L, 'cut')} />,
      lines: [{ label: 'filled triangles, from the unit hexagon', terms: [String(slice(1).fills), ...m.ledger_terms('23', 3, 2, 'triangles', 'level', least(L - 1) + 1, BUDGET)], closed: '', ledger: true }],
    }),
  },
  {
    key: 'vertices', title: 'The vertices of the slice', slider: ['side', 2, 8, 4], show: (k) => 2 * k - 1,
    say: 'The middle slice of the odd cube is a hexagon whose vertex count at side n is the centered hexagonal number 3n(n + 1) + 1, the sequence the corner-and-centre design fills at odd sides; a prime among them is a difference of consecutive cubes, a cuban prime, and the gold terms are the primes.',
    build: (k) => {
      const terms = odds(least(k)).map((n) => String(slice(n).vertices));
      return {
        art: <Markup svg={hex(2 * k - 1, 1, 'cut')} />,
        lines: [
          { label: 'vertices, by odd side', terms, marks: terms.map((t) => JSON.parse(m.factor(t)).prime), closed: '' },
          item(row('11', 2, 'fills', 'side', least(k - 1)), 'the corner-and-centre fills, by odd side'),
        ],
      };
    },
  },
  {
    key: 'gasket', title: 'The gasket cut', slider: ['level', 1, CUTCAP, 4],
    say: 'Every diagonal plane through the octahedral design at side 2 holds exactly 3^L points at every admissible height, by the uniqueness of the binary expansion of the height, so the cut is a Sierpinski gasket at every depth.',
    build: (L) => {
      const profiles = upto(least(L)).map((j) => JSON.parse(m.diagonal_profile('126', 2, j, 2)));
      const last = profiles[L - 1];
      return {
        art: <Markup svg={m.diagonal_svg('126', 2, L, 2, last.central, Math.max(2, Math.round(180 / last.side)))} />,
        sub: `${last.heights} heights, ${last.constant ? 'all alike' : 'not all alike'}`,
        lines: [{ label: 'points on a diagonal plane, by level', terms: profiles.map((p) => p.max), closed: '' }],
      };
    },
  },
  {
    key: 'farey', title: 'The Farey nodes', slider: ['order', 1, 24, 12],
    say: 'Stack the carpet at every odd scale and the lit nodes are the Farey fractions, each new scale n lighting phi(n) new nodes: the count is one more than the totients summed, and the primes are the scales of most novelty.',
    build: (order) => {
      const stacks = upto(least(order)).map((j) => JSON.parse(m.farey_novelty(j)));
      const last = stacks[order - 1];
      return {
        art: <Sketch className="" style={{ width: '100%', height: 200 }} draw={(canvas) => stackOf(canvas, order)} deps={[order]} />,
        sub: `${last.lit} nodes lit, ${last.match ? 'as the totients say' : 'against the totients'}, primes ${last.primes.join(' ')}`,
        lines: [{ label: 'nodes lit, by order', terms: stacks.map((f) => String(f.lit)), closed: '' }],
      };
    },
  },
];

const FIRST = Object.fromEntries(CARDS.map((spec) => [spec.key, spec.slider[3]]));

function hits(terms) {
  if (terms.length < NAMED) return { html: <span className="dim">three terms name the record</span>, formula: '' };
  const found = JSON.parse(m.ledger_identify(terms.slice(0, WINDOW).join(', ')));
  const formula = found.length ? RECORDS.find((r) => r.id === found[0].id)?.formula : '';
  const html = found.length
    ? found.slice(0, 3).map((r, i) => (
      <Fragment key={r.id}>
        {i ? ' · ' : null}
        <a className="badge" href={`https://oeis.org/${r.id}`} target="_blank" rel="noopener">{r.id}</a> {r.name}, from index {r.shift}
      </Fragment>
    ))
    : 'no curated record holds these terms';
  return { html, formula: formula && formula !== 'none' ? formula : '' };
}

function Line({ it }) {
  const found = hits(it.terms);
  const forms = [it.closed, found.formula].filter((form, i, all) => form && all.indexOf(form) === i);
  return (
    <div className="line">
      {it.art ? <div className="strip">{it.art}</div> : null}
      <div className="text">
        {it.label ? <><span className="dim">{it.label}</span><br /></> : null}
        <Terms terms={it.terms} marks={it.marks} capped={it.capped} tight />
        {forms.map((form, i) => <Fragment key={form}>{i ? ' · ' : null}<span className="mono">{form}</span></Fragment>)}
        {forms.length ? ' · ' : null}
        {found.html}
        {it.ledger ? <> · <a href={`../sequences/?q=${encodeURIComponent(it.terms.join(', '))}`}>open in the ledger</a></> : null}
      </div>
    </div>
  );
}

function Card({ spec, index, value, onChange, cardRef }) {
  const [label, min, max] = spec.slider;
  const made = useMemo(() => {
    try {
      const built = spec.build(value);
      return { art: built.art, sub: built.sub, lines: built.lines.map((it, i) => <Line key={i} it={it} />), error: null };
    } catch (error) {
      return { lines: [], error };
    }
  }, [value]);

  return (
    <section className={`panel tour${spec.wide ? ' wide' : ''}`} ref={cardRef}>
      <div className="pic">{made.art}</div>
      <div className="body">
        <h2>{index + 1} · {spec.title} <span>{made.sub ?? ''}</span></h2>
        <p className="story">{spec.say}</p>
        <Row><Slider label={label} value={value} min={min} max={max} show={spec.show ? spec.show(value) : value} onChange={onChange} /></Row>
        <div>{made.lines}</div>
        <Note error={made.error} />
      </div>
    </section>
  );
}

function App() {
  const s = useSeeds();
  const [pick, set] = useQuery(FIRST);
  const cards = useRef({});

  const shuffle = (seed) => {
    const [at] = roll(seed, [[0, CARDS.length - 1]]);
    const spec = CARDS[at];
    const [value] = roll(seed, [[spec.slider[1], spec.slider[2]]]);
    set({ [spec.key]: value });
    cards.current[spec.key]?.scrollIntoView({ behavior: 'smooth', block: 'center' });
  };

  useEffect(() => {
    stamp(pick);
    if (s.get()) shuffle(s.get());
  }, []);

  return (
    <Page crumb="tour" title="A dozen sequences the designs write"
      sub="A design is a rule on the corners of a square or a cube. Grown level by level or widened side by side it counts something, and the count is an integer sequence the OEIS already holds or has just learnt. Every card draws the design live, reads its terms from the crates, and names the record; slide, and the picture and the terms grow together."
      foot={<>A card is one design, one measure and one axis, the same row the <a href="../sequences/">ledger</a> lists, and where a line is such a row the link under its terms opens it with the terms prefilled; the counts of designs, of fractals, of slice vertices, of gasket points and of Farey nodes are not rows of the ledger and carry no link. The level axis grows the fractal at the smallest side the base allows, the side axis holds level one and widens the odd side. The record after the terms is found by the crate from the terms alone, as a window of the entry's own first terms, and the number after it is the entry's index of the first term shown. Every number on this page is computed in Rust; the page only draws. The ledger these cards read from is the <a href="https://github.com/mrlyprod/mrlyprod/blob/main/research/sequences.md">sequences</a> page.</>}>
      <Row>
        <Btn onClick={() => shuffle(s.next())}>Randomize</Btn>
        <span className="dim">picks a card and a step for it</span>
      </Row>
      <div>
        {CARDS.map((spec, i) => (
          <Card key={spec.key} spec={spec} index={i} value={pick[spec.key]} cardRef={(el) => { cards.current[spec.key] = el; }} onChange={(v) => { s.drop(); set({ [spec.key]: v }); }} />
        ))}
      </div>
    </Page>
  );
}

mount(<App />);
