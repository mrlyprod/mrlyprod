import { expect, test } from 'bun:test';
import { renderToStaticMarkup } from 'react-dom/server';
import { Post } from './prose.jsx';

test('a post shows its lead under the title and byline', () => {
  const html = renderToStaticMarkup(<Post row={{ title: 'Launch', date: '2026-09-02', lead: 'One address.', figure: 'blog-launch' }} site={{ title: 'mrly' }} html="" slug="launch" />);
  expect(html).toContain('<p class="by">2026-09-02 · mrly</p><p class="lead">One address.</p></div>');
});
