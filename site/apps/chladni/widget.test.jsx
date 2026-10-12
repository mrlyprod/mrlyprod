import { expect, test } from 'bun:test';
import { status } from './widget.jsx';

test('the readouts go to the status as rows: both peak rings with their wavelengths, and the windows as counts', () => {
  const stat = { gen: 64, live: 100, share: 0.3051, peak: 7, wave: 36.571, maskPeak: 1, maskWave: 256, span: 27, cells: 512, birth: '145 to 192', survive: '145 to 247', error: '' };
  expect(status(stat)).toEqual([['gen', 64], ['live', 100], ['share', '0.305'], ['still', 'ring 7, 36.57 cells'], ['mask', 'ring 1, 256.00 cells'], ['span', 27], ['cells', 512], ['born', '145 to 192'], ['stay', '145 to 247']]);
});

test('a stat with no still reads as its error alone', () => {
  expect(status({ error: 'the mask is empty.' })).toEqual([['error', 'the mask is empty.']]);
});
