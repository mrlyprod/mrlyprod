import { expect, test } from 'bun:test';
import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import * as font from '@mrlyprod/mrly/font';
import { gallery, study } from './engine.js';
import { Gallery, Mark } from './gallery.jsx';

const KINDS = gallery(font);

const PENS = KINDS.flatMap((kind) => kind.glyphs);

test('the gallery shows every pen as a card of its marks, its label and its strokes of its floor, under a filter by kind', () => {
  const html = renderToStaticMarkup(createElement(Gallery, { kinds: KINDS, tint: 'orange', rate: 25, hold: 25, onPick: () => {} }));
  expect(html.match(/class="card( on)?"/g)).toHaveLength(font.supported().length);
  expect(html.match(/class="filter"[^>]*>(.*?)<\/div>/)[1].match(/<button /g)).toHaveLength(6);
  expect(html).toContain('title="latin capital letter a"');
  expect(html).toContain('<div class="code">A</div><div class="name">2 of 2</div></button>');
  expect(html).toContain('<div class="code">space</div><div class="name">0 of 0</div>');
  expect(html.match(/<rect /g)).toHaveLength(font.supported().reduce((n, char) => n + font.path(char).length, 0));
  expect(html).toContain('fill:var(--orange)');
});

test('a flagged pen wears a lit card and a chip saying why', () => {
  const html = renderToStaticMarkup(createElement(Gallery, { kinds: KINDS, rate: 25, hold: 25, onPick: () => {} }));
  const flagged = PENS.filter((one) => one.flag);
  expect(html.match(/class="card on"/g)).toHaveLength(flagged.length);
  expect(html.match(/class="chip refuted"/g)).toHaveLength(flagged.length);
  const D = PENS.find((one) => one.char === 'D');
  expect(html).toContain(`<div class="code">D</div><div class="name">${D.strokes} of ${D.floor}</div><span class="chip refuted">over floor</span>`);
});

test('a mark at frame k shows that frame of the pen, centred on one square slot', () => {
  const glyph = study(font, font.glyph('i'));
  const shown = (k) => renderToStaticMarkup(createElement(Mark, { glyph, k })).match(/<rect /g)?.length ?? 0;
  expect([shown(0), shown(3), shown(null)]).toEqual([0, 3, glyph.frames.at(-1).length]);
  const html = renderToStaticMarkup(createElement(Mark, { glyph, k: 1 }));
  const [c] = glyph.frames[1];
  expect(html).toContain('viewBox="-1 -1 9 9"');
  expect(html).toContain(`x="${(c % glyph.cols) + 2}" y="${Math.floor(c / glyph.cols) + 1}"`);
});
