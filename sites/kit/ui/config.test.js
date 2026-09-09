import { expect, test } from 'bun:test';
import { HUES, headScript, tintCss } from './config.js';

test('a tint name sets the site accent and still leaves every hue overridable', () => {
  const css = tintCss('purple');
  expect(css).toContain(':root { --accent: var(--purple-dark); --on-accent: var(--white); }');
  expect(css).toContain(':root[data-theme="dark"] { --accent: var(--purple-light); --on-accent: var(--black); }');
  for (const hue of HUES) expect(css).toContain(`:root[data-tint="${hue}"] { --accent: var(--${hue}-dark);`);
  expect(css.indexOf(':root[data-tint="purple"]')).toBeGreaterThan(css.indexOf(':root { --accent'));
  expect(tintCss(null)).not.toContain(':root { --accent');
});

test('the head script paints the saved theme, font, tint and saver before the first frame', () => {
  const boot = headScript('cm-');
  expect(boot.startsWith('<script data-boot>')).toBe(true);
  expect(boot).toContain("localStorage.getItem('cm-'+k)");
  expect(boot).toContain("['theme','font','tint','saver']");
  expect(boot).toContain("classList.add('js')");
});
