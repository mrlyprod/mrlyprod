import { expect, test } from 'bun:test';
import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import * as font from '../../../pkgs/mrlyjs/font.js';
import { study } from './engine.js';
import { Gallery, Mark } from './gallery.jsx';

font.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/font/mrlyjs_font_bg.wasm', import.meta.url)).arrayBuffer() });

test('the gallery shows every glyph as a card of its marks, its label and its counts, under a filter by kind', () => {
  const html = renderToStaticMarkup(createElement(Gallery, { font, tint: 'orange', onPick: () => {} }));
  expect(html.match(/class="card"/g)).toHaveLength(font.supported().length);
  expect(html.match(/class="filter"[^>]*>(.*?)<\/div>/)[1].match(/<button /g)).toHaveLength(6);
  expect(html).toContain('title="latin capital letter a"');
  expect(html).toContain('<div class="code">A</div><div class="name">2 of 2 · 1 lift</div>');
  expect(html).toContain('<div class="code">space</div><div class="name">0 of 0</div>');
  expect(html.match(/<rect /g)).toHaveLength(font.supported().reduce((n, char) => n + font.path(char).length, 0));
  expect(html).toContain('fill:var(--orange)');
});

test('a mark written up to k shows the first k cells of the pen order', () => {
  const glyph = study(font, font.glyph('A'));
  const shown = (upto) => renderToStaticMarkup(createElement(Mark, { glyph, upto })).match(/<rect /g)?.length ?? 0;
  expect([shown(0), shown(3), shown(null)]).toEqual([0, 3, glyph.cells.length]);
  expect(renderToStaticMarkup(createElement(Mark, { glyph, upto: 1 }))).toContain(`x="${glyph.order[0] % glyph.cols}" y="${Math.floor(glyph.order[0] / glyph.cols)}"`);
});
