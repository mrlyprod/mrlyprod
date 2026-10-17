import { DEEP, TABLE, axisOf, badge, closed, identify, number } from './engine.js';
import RECORDS from './records.json' with { type: 'json' };

export { RECORDS };

export function hit(row) {
  const n = Math.min(row.terms.length, TABLE.terms);
  if (row.hitAt !== n) {
    row.hitAt = n;
    row.hit = badge(RECORDS, row, row.terms.slice(0, n));
  }
  return row.hit;
}

export function facts(book, row, count) {
  const found = hit(row);
  const n = Math.min(row.terms.length, count);
  const capped = book.capped(row, count, DEEP.cells);
  return {
    record: found,
    status: found ? found.status.toLowerCase() : 'unmatched',
    closed: closed(RECORDS, row) || 'none known',
    terms: row.error ? `${n}, a read error` : capped ? `${n}, to the budget` : String(n),
    space: `dimension ${row.dim}, base ${row.base}`,
    axis: axisOf(row.axis).at(number(row)),
    found: identify(RECORDS, row.terms.slice(0, TABLE.terms)).slice(0, 3),
  };
}

export function found(terms) {
  const one = terms.length >= 4 ? identify(RECORDS, terms.slice(0, TABLE.terms))[0] : null;
  return one ? { id: one.record.id, name: one.record.name, at: one.at, shift: one.at, status: 'Found' } : null;
}
