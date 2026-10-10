import { expect, test } from 'bun:test';
import * as math from '../../../pkgs/mrlyjs/math.js';
import { DEEP, MEASURES, TABLE, applies, badge, csv, identify, json, lead, ledger, numbers, parse, picks, print, search } from './engine.js';
import RECORDS from './records.json' with { type: 'json' };

math.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/math/mrlyjs_math_bg.wasm', import.meta.url)).arrayBuffer() });

const book = ledger(math);

const key = (dim, base, code, measure, axis) => ({ dim, base, code: String(code), measure, axis });

test('the three spaces name their designs: the named orbits of base 2, bare codes at base 3, the empty design last', () => {
  const words = (dim, base) => book.designs(dim, base).map((one) => one.label);
  expect(words(2, 2)).toEqual(['dust 1', 'tree 3', 'star 6', 'carpet 7', 'solid 15', 'empty 0']);
  expect(words(3, 2).slice(0, 5)).toEqual(['dust 1', 'tree 3', 'code 6', 'code 7', 'code 15']);
  expect(words(3, 2).at(-1)).toBe('empty 0');
  expect(words(2, 3).at(-1)).toBe('code 0');
  expect(words(3, 2)).toContain('carpet 23');
  expect(words(3, 2)).toContain('star 22');
  expect(words(3, 2)).toContain('void 24');
  expect(words(3, 2)).toContain('solid 255');
  expect(words(2, 3)).toHaveLength(26);
  expect(words(2, 3)[8]).toBe('code 29');
  expect(book.typed(2, 2, '9').map((row) => row.label)[0]).toBe('void 9');
  expect(book.typed(2, 2, '14')[0].label).toBe('net 14');
  expect(book.typed(2, 2, '16')).toEqual([]);
  expect(book.typed(2, 3, '511')).toHaveLength(16);
});

test('the closed readings match the ledger: fills, voids and faces by level and by side', () => {
  const read = (k) => book.read(book.row(k), 4, DEEP.cells).terms;
  expect(read(key(2, 2, 7, 'fills', 'side'))).toEqual(['8', '21', '40', '65']);
  expect(read(key(2, 2, 7, 'voids', 'level'))).toEqual(['1', '17', '217', '2465']);
  expect(read(key(2, 2, 7, 'surface', 'level'))).toEqual(['16', '80', '496', '3536']);
  expect(read(key(3, 2, 23, 'surface', 'level'))).toEqual(['72', '1056', '18048', '336384']);
  expect(read(key(3, 2, 23, 'voids', 'side'))).toEqual(['7', '44', '135', '304']);
  expect(read(key(3, 2, 255, 'fills', 'side'))).toEqual(['27', '125', '343', '729']);
  expect(read(key(2, 3, 100, 'fills', 'level'))).toEqual(['3', '9', '27', '81']);
});

test('the grid, profile and cut readings agree with the rendered censuses', () => {
  const read = (k, n = 3) => book.read(book.row(k), n, DEEP.cells).terms;
  expect(read(key(2, 2, 7, 'euler', 'level'))).toEqual(['0', '-8', '-72']);
  expect(read(key(3, 2, 23, 'euler', 'level'), 2)).toEqual(['-4', '-80']);
  expect(read(key(3, 2, 23, 'faces', 'level'), 1)).toEqual(['96']);
  expect(read(key(3, 2, 23, 'vertices', 'level'), 1)).toEqual(['64']);
  expect(read(key(3, 2, 23, 'edges', 'level'), 1)).toEqual(['144']);
  expect(read(key(2, 2, 7, 'vertices', 'side'), 2)).toEqual(['16', '36']);
  expect(read(key(3, 2, 23, 'triangles', 'level'), 2)).toEqual(['42', '306']);
  expect(read(key(3, 2, 23, 'peak', 'level'), 2)).toEqual(['6', '42']);
  expect(read(key(3, 2, 23, 'heights', 'level'), 2)).toEqual(['7', '25']);
  expect(read(key(3, 2, 23, 'pieces', 'side'), 2)).toEqual(['1', '7']);
  expect(read(key(3, 2, 255, 'holes', 'level'), 2)).toEqual(['0', '0']);
});

test('a budget caps a row honestly, the table budget first and the deep budget after it', () => {
  const row = book.row(key(3, 2, 23, 'euler', 'level'));
  expect(book.read(row, 8, 1000)).toEqual({ terms: ['-4', '-80'], capped: true });
  expect(book.read(row, 8, 26)).toEqual({ terms: ['-4', '-80'], capped: true });
  expect(book.read(row, 8, TABLE.cells)).toEqual({ terms: ['-4', '-80', '-1408'], capped: true });
  const deep = book.read(row, 3, DEEP.cells);
  expect(deep.terms).toHaveLength(3);
  expect(deep.capped).toBe(false);
  expect(book.read(row, 8, DEEP.cells).capped).toBe(true);
  expect(book.read(book.row(key(3, 2, 23, 'triangles', 'level')), 8, DEEP.cells)).toEqual({ terms: ['42', '306', '2250', '16578', '122202'], capped: true });
  expect(book.read(book.row(key(3, 2, 23, 'peak', 'level')), 24, DEEP.cells).terms).toHaveLength(11);
  expect(book.read(book.row(key(3, 2, 255, 'vertices', 'side')), 24, DEEP.cells).capped).toBe(false);
});

test('a row carries its design as the side-3 cells of its first level', () => {
  expect(book.row(key(2, 2, 7, 'fills', 'level')).cells).toEqual([1, 1, 1, 1, 0, 1, 1, 1, 1]);
  expect(book.row(key(3, 2, 23, 'fills', 'level')).cells).toHaveLength(27);
  expect(book.row(key(2, 3, 100, 'fills', 'level')).cells.filter(Boolean)).toHaveLength(3);
});

test('a sweep reads a whole space one term at a time and then rests', () => {
  const total = book.keys(2, 2).length;
  expect(total).toBe(6 * MEASURES.filter((one) => applies(one, 2, 2)).length * 2);
  let steps = 0;
  while (book.sweep(2, 2)) steps++;
  expect(steps).toBeGreaterThan(total * 3);
  expect(book.progress(2, 2)).toEqual({ done: total, total });
  expect(book.sweep(2, 2)).toBe(false);
  expect(book.keys(2, 2).every((row) => row.terms.length >= 3)).toBe(true);
});

test('a seed leads with a row that draws a picture: the empty design and flat readings are passed over', () => {
  const list = book.keys(2, 2);
  const at = (id) => list.findIndex((row) => row.key === id);
  expect(lead(book, list, 0)).toBe(list[0]);
  expect(lead(book, list, at('2-2-15-voids-level')).key).toBe('2-2-15-surface-level');
  expect(lead(book, list, at('2-2-0-fills-level')).key).toBe(list[0].key);
  expect(lead(book, list.filter((row) => row.code === '0'), 3).key).toBe('2-2-0-voids-side');
});

test('the keyed records read their design sequences at their shifts', () => {
  const keyed = RECORDS.filter((record) => record.key);
  expect(keyed).toHaveLength(20);
  for (const record of keyed) {
    const named = math.name.Sequence.from_file(record.key).toJSON();
    const row = book.row(key(named.dim, named.base ?? 2, named.code, named.measure, named.axis));
    const { terms } = book.read(row, 8, DEEP.cells);
    const start = named.axis === 'level' ? 1 : 2;
    let compared = 0;
    terms.forEach((term, i) => {
      const at = start + i + record.shift - record.offset;
      if (at >= 0 && at < record.terms.length) {
        expect([record.id, i, term]).toEqual([record.id, i, record.terms[at]]);
        compared++;
      }
    });
    expect([record.id, compared >= 3]).toEqual([record.id, true]);
  }
});

test('identify finds the records holding a window and badge prefers the row the record names', () => {
  const ids = (terms) => identify(RECORDS, terms).map(({ record, at }) => [record.id, at]);
  expect(ids(['8', '21', '40', '65'])).toEqual([['A000567', 2]]);
  expect(ids(['1', '17', '217'])).toEqual([['A016185', 1], ['A229896', 37]]);
  expect(ids(['5', '7', '11', '14'])).toEqual([]);
  const carpet = book.row(key(2, 2, 7, 'voids', 'level'));
  book.read(carpet, 8, DEEP.cells);
  expect(badge(RECORDS, carpet, carpet.terms)).toMatchObject({ id: 'A016185', at: 1, status: 'Proved' });
  const mate = book.row(key(2, 2, 7, 'voids', 'side'));
  book.read(mate, 8, DEEP.cells);
  expect(mate.terms.slice(0, 4)).toEqual(['1', '4', '9', '16']);
  expect(badge(RECORDS, mate, mate.terms)).toMatchObject({ id: 'A000290', at: 1, status: 'Collision' });
  expect(badge(RECORDS, mate, mate.terms.slice(0, 3))).toBe(null);
  const own = book.typed(2, 2, '9').find((row) => row.measure === 'fills' && row.axis === 'level');
  book.read(own, 8, DEEP.cells);
  expect(badge(RECORDS, own, own.terms)).toMatchObject({ id: 'A000351', at: 1, status: 'Proved' });
});

test('search takes a window of terms, a name, a code, a measure or a record id', () => {
  const list = book.keys(2, 2);
  while (book.sweep(2, 2));
  const found = (row) => badge(RECORDS, row, row.terms);
  const names = (q) => search(list, q, found, (code) => book.typed(2, 2, code)).map((row) => row.key);
  expect(names('8, 21, 40')).toEqual(['2-2-7-fills-side']);
  expect(names('64 512')).toContain('2-2-7-fills-level');
  expect(names('carpet').every((k) => k.startsWith('2-2-7-'))).toBe(true);
  expect(names('A000567')).toEqual(['2-2-7-fills-side']);
  expect(names('surface').every((k) => k.includes('-surface-'))).toBe(true);
  expect(names('')).toHaveLength(list.length);
  expect(names('9, 8, 7, 6, 5')).toEqual([]);
  expect(names('9').slice(0, 2)).toEqual(['2-2-9-fills-level', '2-2-9-fills-side']);
  expect(names('7').filter((k) => k.startsWith('2-2-7-'))).toHaveLength(16);
  expect(numbers('a, 2')).toBe(null);
});

test('a pick is a dot list of valid keys, six at most, printed back as it was parsed', () => {
  expect(parse('3-2-23-surface-level')).toEqual(key(3, 2, 23, 'surface', 'level'));
  expect(print(parse('2-3-27-fills-side'))).toBe('2-3-27-fills-side');
  expect([parse('4-2-7-fills-level'), parse('2-2-x-fills-level'), parse('2-2-7-area-level'), parse('2-3-7-triangles-level'), parse('3-3-7-fills-level')]).toEqual([null, null, null, null, null]);
  expect(picks('2-2-7-fills-side.bad.2-2-7-fills-side.3-2-23-faces-level')).toEqual(['2-2-7-fills-side', '3-2-23-faces-level']);
  expect(picks(Array.from({ length: 8 }, (_, i) => `2-3-${i}-fills-side`).join('.'))).toHaveLength(6);
});

test('the sheets carry the formal name, the record and the terms', () => {
  const read = book.row(key(3, 2, 23, 'surface', 'level'));
  const row = { ...read, terms: book.read(read, 4, DEEP.cells).terms };
  row.hit = badge(RECORDS, row, row.terms);
  expect(csv([row]).split('\n')).toEqual(['name,design,code,dim,base,measure,axis,start,step,record,status,terms', 'sequence_dim=3_code=23_measure=surface_axis=level,carpet,23,3,2,surface,level,1,1,A332705,Proved,"72, 1056, 18048, 336384"']);
  expect(JSON.parse(json([row]))[0]).toMatchObject({ name: 'sequence_dim=3_code=23_measure=surface_axis=level', record: 'A332705', at: 1, terms: ['72', '1056', '18048', '336384'] });
});
