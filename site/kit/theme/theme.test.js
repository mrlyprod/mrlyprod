import { expect, test } from 'bun:test';
import { palette } from './palette.js';
import { dark, light, tinted } from './theme.js';

test('a blue tint is the identity on the hues and leaves its roles untouched', () => {
  const before = { ...dark };
  expect(tinted(dark, { accent: palette.blue, link: palette.blue, theme: 'dark' })).toEqual({ ...dark, accent: palette.blue });
  expect(tinted(light, { accent: ` ${palette.blue.toUpperCase()} `, link: '#0073d1', theme: 'light' })).toEqual({ ...light, accent: palette.blue });
  expect(dark).toEqual(before);
});

test('a yellow tint swaps yellow into the blue slot and blue into the yellow slot', () => {
  const roles = tinted(dark, { accent: palette.yellow, link: palette.yellow, theme: 'dark' });
  expect([roles.blue, roles.yellow, roles.accent]).toEqual([palette.yellow, palette.blue, palette.yellow]);
});

test('the light theme puts the link shade in the blue slot', () => {
  const roles = tinted(light, { accent: palette.mint, link: '#007f72', theme: 'light' });
  expect([roles.blue, roles.mint, roles.accent]).toEqual(['#007f72', palette.blue, palette.mint]);
});

test('an unknown accent swaps nothing and is still the accent', () => {
  expect(tinted(dark, { accent: '#123456', link: '#123456', theme: 'dark' })).toEqual({ ...dark, accent: '#123456' });
});
