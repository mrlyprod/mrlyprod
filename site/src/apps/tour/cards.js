import { sheet } from '../../lib/sequences/design.js';
import { DEEP, closed, identify, ledgered } from '../../lib/sequences/engine.js';
import { RECORDS } from '../../lib/sequences/facts.js';

export const WINDOW = 6;

export const NAMED = 3;

const FLAT = [['low corner', '1'], ['tree', '3'], ['carpet', '7'], ['void', '9'], ['corner and centre', '11'], ['solid', '15']];

const CAPS = { flat: 5, cube: 3, cut: 6 };

const SPONGE = '23';

const GASKET = '126';

const odds = (k) => Array.from({ length: k }, (_, i) => 2 * i + 1);

const upto = (k) => Array.from({ length: k }, (_, i) => i + 1);

export const least = (n) => Math.max(n, NAMED);

const design = (dim, code, measure, axis) => ({ dim, base: 2, code, measure, axis });

function row(book, dim, code, measure, axis, count) {
  const one = book.row(design(dim, code, measure, axis));
  const { terms, capped } = book.read(one, count, DEEP.cells);
  return { terms, capped, closed: closed(RECORDS, one), ledger: true, row: one };
}

const item = (read, label) => ({ label, ...read, axis: read.row.axis });

const census = (math, side) => math.six.census(math.six.cut_design(SPONGE, side, 1, 2), false);

function profile(math, level) {
  const counts = math.three.profile(GASKET, 2, level, 2).map(BigInt);
  const lit = counts.map((v, i) => [v, i]).filter(([v]) => v > 0n);
  const max = lit.reduce((top, [v]) => (v > top ? v : top), 0n);
  const mid = (counts.length - 1) / 2;
  const central = lit.reduce((best, [, i]) => (Math.abs(i - mid) < Math.abs(best - mid) ? i : best), lit[0]?.[1] ?? 0);
  return { max: String(max), heights: lit.length, constant: lit.every(([v]) => v === lit[0][0]), central };
}

export const CARDS = [
  {
    key: 'sides',
    title: 'The odd-side law',
    label: 'Side up to',
    min: 4,
    max: 8,
    first: 4,
    show: (k) => `side ${2 * k - 1}`,
    say: 'At odd side 2k - 1 an axis splits into k low and k - 1 high positions, so a flat design fills a polynomial in k, and the six designs of the plane read as the polygonal numbers: the low corner the squares, the tree the hexagonal numbers, the carpet the octagonal numbers, before any of them is a fractal.',
    build: ({ math, book }, k) => {
      const sides = odds(k).slice(1);
      return {
        art: { kind: 'rows', rows: FLAT.map(([name, code]) => ({ name, sheets: sides.map((n) => sheet(math, design(2, code, 'fills', 'side'), n, 1)) })) },
        lines: FLAT.map(([name, code]) => item(row(book, 2, code, 'fills', 'side', k - 1), `${name}, code ${code}`)),
      };
    },
  },
  {
    key: 'carpet',
    title: 'The carpet',
    label: 'Level',
    min: 1,
    max: CAPS.flat,
    first: 3,
    say: 'Eight of nine cells survive every level, so the carpet fills 8^level, and its dimension log 8 / log 3 is this count read against the side 3^level.',
    build: ({ math, book }, level) => ({ art: { kind: 'sheet', sheet: sheet(math, design(2, '7', 'fills', 'level'), 3, level), ink: 'fills' }, lines: [item(row(book, 2, '7', 'fills', 'level', least(level)), 'filled cells')] }),
  },
  {
    key: 'voids',
    title: 'What the carpet drops',
    label: 'Level',
    min: 1,
    max: CAPS.flat,
    first: 3,
    say: 'The cells the carpet leaves empty are the grid less the fill, 9^level - 8^level. 9^level grows faster, so the share left empty tends to one.',
    build: ({ math, book }, level) => ({ art: { kind: 'sheet', sheet: sheet(math, design(2, '7', 'voids', 'level'), 3, level), ink: 'voids' }, lines: [item(row(book, 2, '7', 'voids', 'level', least(level)), 'empty cells')] }),
  },
  {
    key: 'perimeter',
    title: 'The perimeter of the carpet',
    label: 'Level',
    min: 1,
    max: CAPS.flat,
    first: 3,
    say: 'Two adjacent blocks bury one edge per spanning position, so the perimeter closes as a sum of the powers 8^level and 3^level, the entry the OEIS lists for the carpet at iteration n.',
    build: ({ math, book }, level) => ({ art: { kind: 'sheet', sheet: sheet(math, design(2, '7', 'surface', 'level'), 3, level), ink: 'edges' }, lines: [item(row(book, 2, '7', 'surface', 'level', least(level)), 'exposed edges')] }),
  },
  {
    key: 'sponge',
    title: 'The sponge',
    label: 'Level',
    min: 1,
    max: CAPS.cube,
    first: 2,
    say: 'Twenty of twenty-seven subcubes survive every level, so the sponge fills 20^level.',
    build: ({ math, book }, level) => ({ art: { kind: 'sheet', sheet: sheet(math, design(3, SPONGE, 'fills', 'level'), 3, level), ink: 'fills' }, lines: [item(row(book, 3, SPONGE, 'fills', 'level', least(level)), 'filled cells')] }),
  },
  {
    key: 'surface',
    title: 'The surface of the sponge',
    label: 'Level',
    min: 1,
    max: CAPS.cube,
    first: 2,
    say: 'The same burial count in three dimensions: the exposed faces are a sum of the powers 20^level and 8^level, the surface area of the stage-n Menger sponge.',
    build: ({ math, book }, level) => ({ art: { kind: 'sheet', sheet: sheet(math, design(3, SPONGE, 'profills', 'level'), 3, level), ink: 'fills' }, lines: [item(row(book, 3, SPONGE, 'surface', 'level', least(level)), 'exposed faces')] }),
  },
  {
    key: 'tile',
    title: 'The odd sponge tile',
    label: 'Side',
    min: 2,
    max: 8,
    first: 3,
    show: (k) => `side ${2 * k - 1}`,
    say: 'Widen the sponge tile to odd side 2k - 1 and its fills and voids are cubics in k: the divisor count of 240^n read as a solid, and the entry this tree contributed.',
    build: ({ math, book }, k) => ({
      art: { kind: 'sheet', sheet: sheet(math, design(3, SPONGE, 'fills', 'side'), 2 * k - 1, 1), ink: 'fills' },
      lines: [item(row(book, 3, SPONGE, 'fills', 'side', least(k - 1)), 'filled cells'), item(row(book, 3, SPONGE, 'voids', 'side', least(k - 1)), 'empty cells')],
    }),
  },
  {
    key: 'designs',
    title: 'How many designs',
    label: 'Base up to',
    min: NAMED + 1,
    max: 6,
    first: 4,
    say: 'A design is a Boolean function on the corners up to the symmetries of the cube: in base 2 the count by dimension is the irreducible Boolean functions of the record, and by base the rows are the bracelets, the toroidal squares and the toroidal cubes.',
    build: ({ math }, top) => {
      const lines = [{ label: 'base 2, by dimension', terms: math.bang.baseq.sequence(2, 4), closed: '' }];
      ['a line', 'a square', 'a cube'].forEach((name, i) => {
        const terms = [];
        for (let q = 2; q <= top; q++) {
          try {
            terms.push(math.bang.baseq.sequence(q, i + 1)[i]);
          } catch {
            break;
          }
        }
        lines.push({ label: `${name}, by base from 2`, terms, closed: '', capped: terms.length < top - 1 });
      });
      return { art: { kind: 'gallery', sheets: math.bang.universe_codes(2).map((code) => sheet(math, design(2, code, 'fills', 'level'), 3, 2)) }, lines };
    },
  },
  {
    key: 'classes',
    title: 'How many fractals',
    label: 'Dimension',
    min: NAMED,
    max: 8,
    first: 4,
    say: 'Two base-2 designs draw the same fractal exactly when they fill the same number of corners of each weight, so the distinct fractals of a dimension number the product over the weights of one more than the corners of that weight.',
    build: ({ math }, dims) => {
      const terms = math.bang.baseq.class_sequence(dims);
      return { art: { kind: 'pins', terms, say: 'by dimension' }, lines: [{ label: 'fill classes, by dimension', terms, closed: '' }] };
    },
  },
  {
    key: 'slices',
    title: 'The middle slice of the sponge',
    label: 'Level',
    min: 1,
    max: CAPS.cube,
    first: 2,
    say: 'Cut the sponge on its middle diagonal plane and count the filled triangles: one index up from the star holes of the record, whose recurrence gives the slice its dimension.',
    build: ({ math, book }, level) => {
      const read = row(book, 3, SPONGE, 'triangles', 'level', least(level - 1) + 1);
      return { art: { kind: 'sheet', sheet: sheet(math, design(3, SPONGE, 'triangles', 'level'), 3, level), ink: 'fills' }, lines: [{ ...read, label: 'filled triangles, from the unit hexagon', terms: [String(census(math, 1).fills), ...read.terms], closed: '' }] };
    },
  },
  {
    key: 'vertices',
    title: 'The vertices of the slice',
    label: 'Side',
    min: 2,
    max: 8,
    first: 4,
    show: (k) => `side ${2 * k - 1}`,
    say: 'The middle slice of the odd cube is a hexagon whose vertex count at side n is the centred hexagonal number 3n(n + 1) + 1, the sequence the corner-and-centre design fills at odd sides; a prime among them is a difference of consecutive cubes, a cuban prime, and the bright terms are the primes.',
    build: ({ math, num, book }, k) => {
      const terms = odds(least(k)).map((n) => String(census(math, n).vertices));
      return {
        art: { kind: 'sheet', sheet: sheet(math, design(3, SPONGE, 'triangles', 'side'), 2 * k - 1, 1), ink: 'fills' },
        lines: [{ label: 'vertices, by odd side', terms, marks: terms.map((t) => num.prime.is_prime(Number(t))), closed: '' }, item(row(book, 2, '11', 'fills', 'side', least(k - 1)), 'the corner-and-centre fills, by odd side')],
      };
    },
  },
  {
    key: 'gasket',
    title: 'The gasket cut',
    label: 'Level',
    min: 1,
    max: CAPS.cut,
    first: 4,
    say: 'Every diagonal plane through the octahedral design at side 2 holds exactly 3^level points at every admissible height, by the uniqueness of the binary expansion of the height, so the cut is a Sierpinski gasket at every depth.',
    build: ({ math }, level) => {
      const profiles = upto(least(level)).map((j) => profile(math, j));
      const last = profiles[level - 1];
      return {
        art: { kind: 'dots', points: [...math.three.diagonal_slice(GASKET, 2, level, 2, last.central)].map((p) => [...p]) },
        sub: `${last.heights} heights, ${last.constant ? 'all alike' : 'not all alike'}`,
        lines: [{ label: 'points on a diagonal plane, by level', terms: profiles.map((p) => p.max), closed: '' }],
      };
    },
  },
  {
    key: 'farey',
    title: 'The Farey nodes',
    label: 'Order',
    min: 1,
    max: 24,
    first: 12,
    say: 'Stack the carpet at every odd scale and the lit nodes are the Farey fractions, each new scale n lighting phi(n) new nodes: the count is one more than the totients summed, and a prime scale p lights the most, p - 1.',
    build: ({ num }, order) => {
      const lit = upto(least(order)).map((j) => num.lattice.farey(j).length);
      const fresh = upto(order).reduce((sum, j) => sum + num.lattice.new_nodes(j), 0n);
      const primes = upto(order).filter((j) => num.prime.is_prime(j));
      return {
        art: { kind: 'farey', nodes: num.lattice.farey(order).map((one) => [one.num, one.den, one.brightness]), order },
        sub: `${lit[order - 1]} nodes lit, ${BigInt(lit[order - 1]) === fresh ? 'as the totients say' : 'against the totients'}, primes ${primes.join(' ') || 'none yet'}`,
        lines: [{ label: 'nodes lit, by order', terms: lit.map(String), closed: '' }],
      };
    },
  },
];

export const clamp = (card, step) => {
  const one = CARDS[card];
  const n = Math.round(Number(step));
  return Number.isFinite(n) ? Math.max(one.min, Math.min(one.max, n)) : one.first;
};

export const pick = (value) => {
  const n = Math.round(Number(value.card));
  const card = Number.isFinite(n) ? Math.max(0, Math.min(CARDS.length - 1, n)) : 0;
  return { card, step: clamp(card, value.step) };
};

export const shown = (card, step) => (CARDS[card].show ? CARDS[card].show(step) : String(step));

export function records(terms) {
  if (terms.length < NAMED) return { found: [], formula: '', short: true };
  const found = identify(RECORDS, terms.slice(0, WINDOW)).slice(0, 3);
  return { found, formula: found[0]?.record.formula ?? '', short: false };
}

export function build(units, card, step) {
  const one = CARDS[card];
  try {
    const made = one.build(units, step);
    return { ...made, lines: made.lines.map((line) => ({ ...line, records: records(line.terms), link: line.ledger ? ledgered(line.terms) : '' })), error: '' };
  } catch (error) {
    return { art: null, lines: [], error: String(error?.message ?? error) };
  }
}
