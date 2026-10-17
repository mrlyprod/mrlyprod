import { afterAll, expect, test } from 'bun:test';
import { mkdirSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { dead } from './links.ts';

const dist = join(tmpdir(), `links-${process.pid}`);

const strict = join(tmpdir(), `links-strict-${process.pid}`);

const routed = join(tmpdir(), `links-routed-${process.pid}`);

afterAll(() => [dist, strict, routed].forEach((one) => rmSync(one, { recursive: true, force: true })));

test('an internal href, src or srcset that lands on no file, folder index or tree path is dead, an allowed, outside or raw one is not', () => {
  mkdirSync(join(dist, 'a'), { recursive: true });
  mkdirSync(join(dist, 'raw'), { recursive: true });
  writeFileSync(join(dist, 'raw/source.html'), '<a href="gone.html">x</a>');
  writeFileSync(join(dist, 'a/pic.png'), '');
  writeFileSync(join(dist, 'a/index.html'), '');
  const page = [
    '<a href="/a/">x</a><a href="/a">x</a><a href="pic.png#y">x</a><a href="https://mrly.net/a/pic.png">x</a>',
    '<img src="gone.png" srcset="pic.png 1x, /lost.png 2x"><a href="/git/src/">x</a><a href="/git/nope.rs">x</a>',
    '<a href="/stats/stats.json">x</a><a href="https://elsewhere.org/x">x</a><a href="mailto:a@b.c">x</a><a href="#top">x</a>',
    '<script>const s = \'<img src="/inline.png">\';</script>',
  ].join('');
  writeFileSync(join(dist, 'a/page.html'), page);
  const tree = { base: '/git/', name: 'r', slug: '', branch: 'main', c: [{ n: 'src', k: 'd' as const, c: [] }] };
  expect(dead(dist, 'https://mrly.net', ['stats/stats.json'], tree)).toEqual({
    links: 10,
    pages: 2,
    dead: [
      { page: 'a/page.html', url: 'gone.png' },
      { page: 'a/page.html', url: '/lost.png' },
      { page: 'a/page.html', url: '/git/nope.rs' },
    ],
  });
});

test('a link is held by its exact case, and a folder with a dot in its name only through a trailing slash', () => {
  mkdirSync(join(strict, 'About'), { recursive: true });
  mkdirSync(join(strict, 'v.d'), { recursive: true });
  writeFileSync(join(strict, 'About/index.html'), '');
  writeFileSync(join(strict, 'v.d/index.html'), '');
  writeFileSync(join(strict, 'index.html'), ['/About/', '/About', '/about/', '/ABOUT/INDEX.HTML', '/v.d/', '/v.d/index.html', '/v.d'].map((url) => `<a href="${url}">x</a>`).join(''));
  expect(dead(strict, 'https://mrly.net', [], null).dead.map((one) => one.url)).toEqual(['/about/', '/ABOUT/INDEX.HTML', '/v.d']);
});

test('a route the build lists is held though no file backs it', () => {
  mkdirSync(routed, { recursive: true });
  writeFileSync(join(routed, 'index.html'), '<a href="/menu/">x</a><a href="/cart/">x</a>');
  expect(dead(routed, 'https://mrly.net', [], null, new Set(['/menu/'])).dead.map((one) => one.url)).toEqual(['/cart/']);
});
