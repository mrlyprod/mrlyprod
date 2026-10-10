import { expect, test } from 'bun:test';
import { renderToStaticMarkup } from 'react-dom/server';
import * as num from '../../../pkgs/mrlyjs/num.js';
import { Frame } from '../../lib/frame.jsx';
import { Widget } from './widget.jsx';

num.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/num/mrlyjs_num_bg.wasm', import.meta.url)).arrayBuffer() });

const value = { seed: 1, from: 0, span: 60, speed: 2, look: 'line', trail: 10, tint: '' };

test('the widget mounts its stage once the unit is in hand', () => {
  const page = (unit) => renderToStaticMarkup(<Frame label="Zeta"><Widget value={value} onChange={() => {}} unit={unit} /></Frame>);
  expect(page(undefined)).not.toContain('<canvas');
  expect(page(num)).toContain('<canvas');
});
