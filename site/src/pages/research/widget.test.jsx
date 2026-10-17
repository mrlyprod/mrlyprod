import { afterAll, beforeAll, expect, test } from 'bun:test';
import { renderToStaticMarkup } from 'react-dom/server';
import { Gallery, Widget } from './widget.jsx';

const rows = [{ route: '/research/', kind: 'app', title: 'Research', lead: 'The notes.', meta: {} }];

const saved = Object.getOwnPropertyDescriptor(globalThis, 'location');

beforeAll(() => {
  globalThis.location = { pathname: '/research/' };
});

afterAll(() => {
  if (saved) Object.defineProperty(globalThis, 'location', saved);
  else delete globalThis.location;
});

test('before the topics load the page holds its title and lead and no dialog', () => {
  const html = renderToStaticMarkup(<Widget rows={rows} site={{}} draw={() => Promise.reject()} />);
  expect(html).toContain('<h1 id="research">Research</h1>');
  expect(html).toContain('The notes.');
  expect(html).not.toContain('<dialog');
});

test('the gallery draws every figure of a topic under its name and nothing for a topic without figures', () => {
  const html = renderToStaticMarkup(<Gallery names={['research-farey', 'paper-farey-cover']} />);
  expect(html).toContain('<section class="gallery grid" aria-label="Figures">');
  expect(html).toContain('<figure data-figure="research-farey">');
  expect(html).toContain('<figcaption>paper-farey-cover</figcaption>');
  expect(html.match(/<figure /g)).toHaveLength(2);
  expect(renderToStaticMarkup(<Gallery />)).toBe('');
});
