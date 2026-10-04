import { expect, test } from 'bun:test';
import { sort } from './router.js';

const DEEP = ['/git/'];

const kind = (to, from) => sort(new URL(to, `https://mrly.net${from}`), new URL(`https://mrly.net${from}`), DEEP);

test('the router takes a page and leaves another origin, a file, a fragment of the page and a link of the page\'s own router', () => {
  const rows = [
    ['/research/wiki/', '/', 'page'],
    ['/research/notes/beneath/#the-radix-dial', '/research/wiki/rep-tiles/', 'page'],
    ['/about/', '/about/', 'page'],
    ['?seed=7', '/demos/sponge/', 'page'],
    ['/git/site/kit/ssg/build.ts', '/about/', 'page'],
    ['/git/', '/menu/', 'page'],
    ['/menu/', '/git/site/README.md', 'page'],
    ['/git/site/', '/git/site/README.md', 'inner'],
    ['#L12', '/git/site/kit/ssg/build.ts', 'inner'],
    ['#proved', '/research/claims/automata/', 'mark'],
    ['#', '/menu/', 'mark'],
    ['https://oeis.org/A000001/', '/research/', ''],
    ['mailto:help@mrly.net', '/contact/', ''],
    ['/robots.txt', '/menu/', ''],
    ['/sitemap.xml', '/menu/', ''],
    ['/404.html', '/menu/', ''],
    ['/manifest.webmanifest', '/menu/', ''],
    ['paper.pdf', '/research/papers/walsh-spectrometer/', ''],
    ['/stats/stats.json', '/stats/', ''],
    ['/raw/site/ui/chrome.js', '/git/site/ui/chrome.js', ''],
    ['/raw/site/demos/views/sponge/index.html', '/git/site/demos/views/sponge/index.html', ''],
  ];
  for (const [to, from, want] of rows) expect([to, from, kind(to, from)]).toEqual([to, from, want]);
});
