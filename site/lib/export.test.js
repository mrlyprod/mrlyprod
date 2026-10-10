import { afterEach, beforeEach, expect, test } from 'bun:test';
import { save, text } from './export.js';

const real = { createObjectURL: URL.createObjectURL, revokeObjectURL: URL.revokeObjectURL, setTimeout: globalThis.setTimeout };
let seen;

beforeEach(() => {
  seen = [];
  URL.createObjectURL = (blob) => {
    seen.push(['url', blob.type]);
    return 'blob:x';
  };
  URL.revokeObjectURL = (url) => seen.push(['revoke', url]);
  globalThis.setTimeout = (fn) => fn();
  globalThis.document = { createElement: () => ({ click() { seen.push(['click', this.download, this.href]); } }) };
});

afterEach(() => {
  Object.assign(URL, { createObjectURL: real.createObjectURL, revokeObjectURL: real.revokeObjectURL });
  globalThis.setTimeout = real.setTimeout;
  delete globalThis.document;
});

const canvas = (type) => ({
  toBlob: (done, asked) => {
    seen.push(['blob', asked]);
    done(new Blob([], { type: type ?? asked }));
  },
});

test('save draws, takes the extension from the blob type, downloads and revokes', async () => {
  const file = await save(canvas(), { name: 'lightspeed-7', kind: 'webp', draw: () => seen.push(['draw']) });
  expect(file).toBe('lightspeed-7.webp');
  expect(seen).toEqual([['draw'], ['blob', 'image/webp'], ['url', 'image/webp'], ['click', 'lightspeed-7.webp', 'blob:x'], ['revoke', 'blob:x']]);
});

test('a blob of another type or none falls back to png', async () => {
  expect([await save(canvas('image/png'), { name: 'a', kind: 'webp' }), await save(canvas(''), { name: 'b', kind: 'jpeg' })]).toEqual(['a.png', 'b.png']);
});

test('text downloads a string as a file of its kind with the matching mime, plain text for an unknown kind', () => {
  expect([text('<svg/>', 'two-7', 'svg'), text('{}', 'font-hi', 'json'), text('a,b', 'seq-1', 'csv'), text('v 0 0 0', 'three-2', 'obj'), text('hi', 'note', 'md')]).toEqual(['two-7.svg', 'font-hi.json', 'seq-1.csv', 'three-2.obj', 'note.md']);
  expect(seen.filter(([kind]) => kind === 'url').map(([, type]) => type.split(';')[0])).toEqual(['image/svg+xml', 'application/json', 'text/csv', 'model/obj', 'text/plain']);
  expect(seen).toContainEqual(['click', 'two-7.svg', 'blob:x']);
});
