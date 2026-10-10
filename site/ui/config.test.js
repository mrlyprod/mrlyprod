import { expect, test } from 'bun:test';
import SITE from '../site.json';
import { tintCss } from './config.js';

test("the shell's tint style is the site tint by default and every hue on demand", async () => {
  const shell = await Bun.file(new URL('./index.html', import.meta.url)).text();
  expect(shell).toContain(`<style>\n${tintCss(SITE.tint)}</style>`);
});
