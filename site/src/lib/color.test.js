import { expect, test } from 'bun:test';
import { paints } from './color.js';
import { palette } from './theme/palette.gen.js';
import { light } from './theme/theme.js';

test('a scene reads the tint from --accent, or --<hue> when its tint names one of the settings hues, and falls back to the palette', () => {
  const canvas = {};
  const css = { '--accent': ' #00ff00 ', '--red': '#ff0000' };
  globalThis.getComputedStyle = () => ({ getPropertyValue: (name) => css[name] ?? '' });
  expect([paints(canvas).accent, paints(canvas, '').accent, paints(canvas, 'red').accent, paints(canvas, 'gray').accent]).toEqual(['#00ff00', '#00ff00', '#ff0000', '#00ff00']);
  delete globalThis.getComputedStyle;
  expect([paints(canvas).accent, paints(canvas, 'mint').accent]).toEqual([light.accent, palette.mint]);
});

test('look hands the theme foreground as ink beside paper and accent, from --fg or the theme', () => {
  globalThis.getComputedStyle = () => ({ getPropertyValue: (name) => (name === '--fg' ? '#123456' : '') });
  const read = paints({}).ink;
  delete globalThis.getComputedStyle;
  expect([read, paints({}).ink]).toEqual(['#123456', light.fg]);
});
