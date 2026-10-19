import { expect, test } from 'bun:test';
import { renderToStaticMarkup } from 'react-dom/server';
import { Item, Shelf, roam } from './library.jsx';

test('arrows walk a list by one and a grid by its row, clamped at both ends, Home and End jump', () => {
  const items = [0, 1, 2, 3, 4, 5, 6];
  expect([roam(items, 2, 'ArrowDown'), roam(items, 2, 'ArrowDown', 3), roam(items, 5, 'ArrowDown', 3), roam(items, 1, 'ArrowUp', 3), roam(items, 2, 'ArrowRight', 3), roam(items, 2, 'ArrowRight'), roam(items, 4, 'Home'), roam(items, 1, 'End'), roam(items, 1, 'x'), roam([], 0, 'End')]).toEqual([3, 5, 6, 0, 3, null, 0, 6, null, null]);
});

test('a shelf is a named list or grid, and its picked item is the current one', () => {
  const html = renderToStaticMarkup(
    <Shelf name="Places" grid>
      <Item label="Seahorse Valley" sub="-0.75, 0.1" on />
      <Item label="Home" href="/fractals/" />
    </Shelf>,
  );
  expect(html).toBe('<section class="shelf" aria-label="Places"><h3>Places</h3><div class="grid" role="list"><button type="button" data-item="" aria-current="true" class="on" role="listitem"><span class="label">Seahorse Valley</span><span class="sub">-0.75, 0.1</span></button><a href="/fractals/" data-item="" role="listitem"><span class="label">Home</span></a></div></section>');
});
