import { expect, test } from 'bun:test';
import { ICONS } from '../lib/icons.gen.js';
import { branch } from './tree.js';

const node = (tag) => ({ tag, kids: [], attrs: {}, dataset: {}, append(...more) { this.kids.push(...more); }, setAttribute(name, value) { this.attrs[name] = value; } });

test('a tree folder summary starts with the expand_more chevron, then its link', () => {
  globalThis.document = { createElement: node, createElementNS: (space, tag) => node(tag) };
  const summary = branch('/git/', { n: 'lib', k: 'd' }, 'site/lib').kids[0].kids[0];
  delete globalThis.document;
  const [svg, a] = summary.kids;
  expect([svg.tag, svg.kids[0].attrs.d, a.tag, a.href]).toEqual(['svg', ICONS.expand_more, 'a', '/git/site/lib/']);
});
