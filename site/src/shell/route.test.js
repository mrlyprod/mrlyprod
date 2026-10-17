import { expect, test } from 'bun:test';
import { match, sort } from './route.js';

const DEEP = ['/git/', '/research/'];

const ROWS = [
  { route: '/', kind: 'app', meta: { id: 'home' } },
  { route: '/about/', kind: 'prose', meta: { id: 'about' } },
  { route: '/research/', kind: 'app', meta: { id: 'research', deep: true } },
  { route: '/git/', kind: 'app', meta: { id: 'git', deep: true } },
  { route: '/404.html', kind: 'app', meta: { id: 'missing' } },
];

const kind = (to, from) => sort(new URL(to, `https://mrly.net${from}`), new URL(`https://mrly.net${from}`), DEEP);

test('a path takes its exact row, else the longest deep row it sits under, else missing, and /index.html is home', () => {
  const rows = [
    ['/about/', '/about/'],
    ['/git/', '/git/'],
    ['/git/site/infra/push.ts', '/git/'],
    ['/research/', '/research/'],
    ['/research/beneath/', '/research/'],
    ['/nothing-here/', '/404.html'],
    ['/index.html', '/'],
    ['/', '/'],
  ];
  expect(rows.map(([path]) => [path, match(ROWS, path).route])).toEqual(rows);
});

test("the router takes a page and leaves another origin, a file, a fragment of the page and a link of the page's own router", () => {
  const rows = [
    ['/git/site/', '/menu/', 'page'],
    ['/about/#who', '/research/beneath/', 'page'],
    ['/research/', '/menu/', 'page'],
    ['/research/beneath/', '/research/', 'inner'],
    ['/research/', '/research/beneath/', 'inner'],
    ['/about/', '/about/', 'page'],
    ['?seed=7', '/sleep/', 'page'],
    ['/git/site/infra/push.ts', '/about/', 'page'],
    ['/git/', '/menu/', 'page'],
    ['/menu/', '/git/site/README.md', 'page'],
    ['/cart/', '/about/', 'page'],
    ['/git/site/', '/git/site/README.md', 'inner'],
    ['#L12', '/git/site/infra/push.ts', 'inner'],
    ['#who', '/about/', 'mark'],
    ['#', '/menu/', 'mark'],
    ['https://oeis.org/A000001/', '/research/beneath/', ''],
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
