import { afterEach, beforeEach, expect, test } from 'bun:test';
import { grown, save, text } from './export.js';

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
  width: 300,
  height: 150,
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

test('a still at 2x fixes the frame to twice the store before the draw and lets it go once the blob is in hand; at 1x the frame is never touched', async () => {
  const fix = (next) => seen.push(['fix', next?.frame ?? null, next?.dpr ?? null]);
  await save(canvas(), { name: 'a', scale: 2, fix, draw: () => seen.push(['draw']) });
  expect(seen.slice(0, 5)).toEqual([['fix', [600, 300], 2], ['draw'], ['blob', 'image/png'], ['fix', null, null], ['url', 'image/png']]);
  seen = [];
  await save(canvas(), { name: 'b', scale: 1, fix });
  expect(seen.some(([kind]) => kind === 'fix')).toBe(false);
});

test('a grown frame keeps the aspect and stops at 2^24 pixels, and a scale that grows nothing gives no frame', () => {
  expect([grown(300, 150, 4), grown(3200, 2000, 2), grown(2732, 1800, 2), grown(3200, 2000, 1), grown(8000, 4000, 2)]).toEqual([[1200, 600], [5181, 3238], [5046, 3324], null, null]);
});

test('a blob of another type or none falls back to png', async () => {
  expect([await save(canvas('image/png'), { name: 'a', kind: 'webp' }), await save(canvas(''), { name: 'b', kind: 'jpeg' })]).toEqual(['a.png', 'b.png']);
});

test('text downloads a string as a file of its kind with the matching mime, plain text for an unknown kind', () => {
  expect([text('<svg/>', 'two-7', 'svg'), text('{}', 'font-hi', 'json'), text('a,b', 'seq-1', 'csv'), text('v 0 0 0', 'three-2', 'obj'), text('hi', 'note', 'md')]).toEqual(['two-7.svg', 'font-hi.json', 'seq-1.csv', 'three-2.obj', 'note.md']);
  expect(seen.filter(([kind]) => kind === 'url').map(([, type]) => type.split(';')[0])).toEqual(['image/svg+xml', 'application/json', 'text/csv', 'model/obj', 'text/plain']);
  expect(seen).toContainEqual(['click', 'two-7.svg', 'blob:x']);
});
