import { expect, test } from 'bun:test';
import { renderToStaticMarkup } from 'react-dom/server';
import * as math from '@mrlyprod/mrly/math';
import { Chip, spot } from './chip.jsx';

const FREE = { dim: 'dim', base: 'base', code: 'code' };

const face = (html) => {
  const [, art, name] = html.match(/<button type="button" class="design"[^>]*>(<canvas[^>]*><\/canvas>)?<span>([^<]*)<\/span><\/button>/);
  return [Boolean(art), name];
};

test('the chip shows a small thumb and the name and code of the design, the home design for an empty code, and the code alone while math loads', () => {
  const chip = (value, given) => face(renderToStaticMarkup(<Chip slot={FREE} value={value} onChange={() => {}} math={given} />));
  expect([chip({ dim: 2, base: 2, code: '7' }, math), chip({ dim: 2, base: 2, code: '11' }, math), chip({ dim: 3, base: 2, code: '' }, math), chip({ dim: 2, base: 2, code: '7' }, null)]).toEqual([[true, 'carpet 7'], [true, 'code 11'], [true, 'carpet 23'], [false, '7']]);
});

test('the chip holds a glass popover, empty until it opens, and the full browser in a closed dialog', () => {
  const html = renderToStaticMarkup(<Chip slot={FREE} value={{ dim: 2, base: 2, code: '7' }} onChange={() => {}} math={math} />);
  expect(html).toMatch(/popoverTarget="(designs[^"]+)"[^>]*>[^]*<\/button><div id="\1" popover="auto" role="dialog" class="chooser" data-lens="menu" aria-label="Designs"><\/div><dialog class="modal browse" aria-label="Designs">/);
  expect(html).not.toContain('class="browser');
});

test('the popover opens under the chip, or over it when there is more room above', () => {
  expect([spot({ left: 100, top: 10, width: 120, bottom: 46 }, 800), spot({ left: 100, top: 700, width: 120, bottom: 736 }, 800)]).toEqual([{ x: 160, y: 46, up: false }, { x: 160, y: 100, up: true }]);
});
