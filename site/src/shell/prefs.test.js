import { expect, test } from 'bun:test';
import { look } from './prefs.js';

test('an icon style resolves like Apple: Auto follows the theme, Always keeps the dark look, Default never changes', () => {
  expect(look('', false, true)).toBe('');
  expect(look('', true, false)).toBe('');
  expect(look('dark', false, false)).toBe('');
  expect(look('dark', false, true)).toBe('dark');
  expect(look('dark', true, false)).toBe('dark');
  expect(look('clear', false, false)).toBe('clear-light');
  expect(look('clear', false, true)).toBe('clear-dark');
  expect(look('tinted', true, false)).toBe('tinted-dark');
});
