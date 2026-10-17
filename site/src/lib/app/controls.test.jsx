import { expect, test } from 'bun:test';
import { renderToStaticMarkup } from 'react-dom/server';
import { text } from '../../../test/dom.js';
import { Btn, Export, Fold, Icon, Menu, Pick, Toggle } from './controls.jsx';

test('a Toggle writes 1 when switched on and 0 when switched off', () => {
  const wrote = [];
  const input = (value) => Toggle({ label: 'wrap', value, onChange: (v) => wrote.push(v) }).props.children[1];
  input(0).props.onChange({ target: { checked: true } });
  input(1).props.onChange({ target: { checked: false } });
  expect(wrote).toEqual([1, 0]);
});

test('a Btn or a fold header a pointer clicks lets go of focus before its action, one a key presses keeps it', () => {
  const seen = [];
  const click = Btn({ onClick: () => seen.push('act'), children: 'Reroll' }).props.onClick;
  const head = Fold({ name: 'Jump', open: false }).props.children[0].props.onClick;
  const at = (fn, detail) => fn({ detail, currentTarget: { blur: () => seen.push(`blur ${detail}`) } });
  at(click, 1);
  at(click, 0);
  at(head, 1);
  at(head, 0);
  expect(seen).toEqual(['blur 1', 'act', 'act', 'blur 1']);
});

test('a fold summary ends on the expand_more chevron, after its live values', () => {
  const fold = renderToStaticMarkup(<Fold name="Jump" open={false} live="20">x</Fold>);
  expect(fold).toContain(`<span class="live">20</span>${renderToStaticMarkup(<Icon name="expand_more" />)}</summary>`);
});

test('a pick draws the expand_more chevron in a span after its select', () => {
  const pick = renderToStaticMarkup(<Pick label="Shape" value="a" options={['a', 'b']} onChange={() => {}} />);
  expect(pick).toContain(`</select><span class="drop">${renderToStaticMarkup(<Icon name="expand_more" />)}</span></label>`);
});

test('Export is a dialog: PNG and WebP, the sizes 1x, 2x and 4x only for a scene that can fix its frame, a Save button, then the rows of more in order', () => {
  const html = (fix) => renderToStaticMarkup(<Export canvas={null} name="x" fix={fix} more={[['Video 9:16', () => {}], ['Video 16:9', () => {}]]} />);
  const radios = (text) => [...text.matchAll(/type="radio" name="[^"]*"[^>]*? value="([^"]+)"/g)].map((found) => found[1]);
  expect(html(() => {})).toContain('<dialog class="modal export" aria-label="Export">');
  expect([radios(html(() => {})), radios(html())]).toEqual([['png', 'webp', '1', '2', '4'], ['png', 'webp']]);
  expect([...html().matchAll(/<button type="button"( class="primary")?>([^<]+)<\/button>/g)].map((found) => found[2])).toEqual(['Save PNG', 'Video 9:16', 'Video 16:9']);
});

test('a Menu is a button and a popover listing its rows in order, a row with an icon drawing it before its text', () => {
  const html = renderToStaticMarkup(<Menu className="icon more" aria-label="More" rows={[['Copy link', () => {}, 'link'], ['Zen', () => {}]]}>x</Menu>);
  expect(html).toMatch(/<button type="button" class="icon more" popoverTarget="(menu[^"]+)"[^>]*aria-label="More">x<\/button><div id="\1" popover="auto" class="pop"/);
  expect([...html.matchAll(/<button type="button">(<svg[^]*?<\/svg>)?([^<]+)<\/button>/g)].map((found) => [found[2], Boolean(found[1])])).toEqual([['Copy link', true], ['Zen', false]]);
});
