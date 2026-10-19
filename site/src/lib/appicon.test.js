import { expect, test } from 'bun:test';
import { appicon } from './appicon.js';

const icon = (body, defs = '') => `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1024 1024">${defs ? `<defs>${defs}</defs>` : ''}<rect class="back" width="1024" height="1024" fill="url(#back)"/>${body}</svg>`;
const BACK = '<linearGradient id="back"><stop offset="0" stop-color="#5aaeff"/><stop offset="1" stop-color="#008cff"/></linearGradient>';

test('an icon namespaces its ids and links, so two icons on one page never share a paint', () => {
  const { defs, art } = appicon('snake', icon('<circle r="9" fill="url(#sun)"/><use href="#dot"/><path id="dot" d="M0 0h9" fill="#ff8f2c"/>', `${BACK}<radialGradient id="sun"><stop offset="0" stop-color="#ffd100"/></radialGradient>`));
  expect(defs).toContain('id="snake-back"');
  expect(defs).toContain('id="snake-sun"');
  expect(art).toContain('fill="url(#snake-back)"');
  expect(art).toContain('fill="url(#snake-sun)"');
  expect(art).toContain('href="#snake-dot"');
  expect(art).toContain('fill="#ff8f2c"');
});

test('in Dark white ink takes the key colour, the last stop of the back, and data-dark overrides one shape', () => {
  const { art } = appicon('mail', icon('<path d="M0 0h9" fill="#fff"/><path d="M9 0h9" fill="#fff" data-dark="#32cc58"/>', BACK));
  expect(art).toContain('style="--key:#008cff"');
  expect(art).toContain('<path class="w" d="M0 0h9" fill="#fff"/>');
  expect(art).toContain('<path class="d" d="M9 0h9" fill="#fff" style="--d:#32cc58"/>');
});

test('an icon refuses what the looks cannot restyle or what reaches outside it', () => {
  const bad = ['<path d="M0 0" style="fill:red"/>', '<text>A</text>', '<path d="M0 0" class="x"/>', '<path d="M0 0" fill="url(https://x.y/a)"/>', '<path d="M0 0" fill="url(#nope)"/>', '<path d="M0 0" data-mono="half"/>', '<image href="a.png"/>'];
  for (const body of bad) expect(() => appicon('x', icon(body, BACK))).toThrow(/^appicon: x\/icon\.svg/);
  expect(() => appicon('x', icon('', BACK))).toThrow(/draws nothing/);
  expect(() => appicon('x', '<svg viewBox="0 0 100 100"><rect class="back"/><path d="M0 0"/></svg>')).toThrow(/viewBox/);
});
