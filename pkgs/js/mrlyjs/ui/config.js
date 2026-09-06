const DEFAULTS = {
  title: '',
  root: '',
  since: new Date().getFullYear(),
  prefix: 'mrly-',
  font: true,
  settings: true,
  tint: null,
  tree: [],
  socials: [],
  contact: '',
};

let current = { ...DEFAULTS };

export function configure(site) {
  current = { ...DEFAULTS, ...site };
  return current;
}

export function conf() {
  return current;
}

/* TINT */

const block = (pair) => `--accent: ${pair.accent}; --on-accent: ${pair.on};`;

export function tintCss(tint = current.tint) {
  if (!tint || !tint.light || !tint.dark) return '';
  const light = block(tint.light);
  const dark = block(tint.dark);
  return [
    `:root { ${light} }`,
    `@media (prefers-color-scheme: dark) { :root:not([data-theme="light"]) { ${dark} } }`,
    `:root[data-theme="dark"] { ${dark} }`,
    '',
  ].join('\n');
}
