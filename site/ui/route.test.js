import { expect, test } from 'bun:test';
import { match, sort } from './route.js';

const DEEP = ['/git/'];

const ROWS = [
  { route: '/', kind: 'home', meta: {} },
  { route: '/about/', kind: 'prose', meta: {} },
  { route: '/research/', kind: 'paused', meta: {} },
  { route: '/git/', kind: 'git', meta: { deep: true } },
  { route: '/404.html', kind: 'missing', meta: {} },
];

const kind = (to, from) => sort(new URL(to, `https://mrly.net${from}`), new URL(`https://mrly.net${from}`), DEEP);

test('a path takes its exact row, else the longest deep row it sits under, else missing, and /index.html is home', () => {
  const rows = [
    ['/about/', '/about/'],
    ['/git/', '/git/'],
    ['/git/site/kit/push.ts', '/git/'],
    ['/research/notes/beneath/', '/404.html'],
    ['/nothing-here/', '/404.html'],
    ['/index.html', '/'],
    ['/', '/'],
  ];
  expect(rows.map(([path]) => [path, match(ROWS, path).route])).toEqual(rows);
});

test("the router takes a page and leaves another origin, a file, a fragment of the page and a link of the page's own router", () => {
  const rows = [
    ['/research/', '/', 'page'],
    ['/about/#who', '/research/', 'page'],
    ['/about/', '/about/', 'page'],
    ['?seed=7', '/sleep/', 'page'],
    ['/git/site/kit/push.ts', '/about/', 'page'],
    ['/git/', '/menu/', 'page'],
    ['/menu/', '/git/site/README.md', 'page'],
    ['/cart/', '/about/', 'page'],
    ['/git/site/', '/git/site/README.md', 'inner'],
    ['#L12', '/git/site/kit/push.ts', 'inner'],
    ['#who', '/about/', 'mark'],
    ['#', '/menu/', 'mark'],
    ['https://oeis.org/A000001/', '/research/', ''],
    ['mailto:help@mrly.net', '/contact/', ''],
    ['/robots.txt', '/menu/', ''],
    ['/sitemap.xml', '/menu/', ''],
    ['/404.html', '/menu/', ''],
    ['/index.html', '/menu/', ''],
    ['/stats/stats.json', '/stats/', ''],
    ['/raw/site/ui/chrome.js', '/git/site/ui/chrome.js', ''],
  ];
  for (const [to, from, want] of rows) expect([to, from, kind(to, from)]).toEqual([to, from, want]);
});
