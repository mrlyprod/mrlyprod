const DEFAULTS = {
  title: '',
  since: new Date().getFullYear(),
  font: true,
  settings: true,
  tint: null,
  menu: '/menu/',
  cart: '/cart/',
  explorer: '/git/tree.json',
  company: '',
};

let current = { ...DEFAULTS };

export function configure(site) {
  current = { ...DEFAULTS, ...site };
  return current;
}

export function conf() {
  return current;
}

/* BOOT */

export function headScript(prefix = current.prefix || 'mrly-') {
  const body = `const d=document.documentElement;d.classList.add('js');try{for(const k of['theme','font','tint','saver']){const v=localStorage.getItem('${prefix}'+k);if(v)d.dataset[k]=v}}catch{}`;
  return `<script data-boot>(()=>{${body}})()</script>`;
}

/* TINT */

export const HUES = ['red', 'orange', 'yellow', 'green', 'mint', 'teal', 'cyan', 'blue', 'indigo', 'purple', 'pink', 'brown'];

const day = (hue) => `--accent: var(--${hue}-dark); --on-accent: var(--white);`;

const night = (hue) => `--accent: var(--${hue}-light); --on-accent: var(--black);`;

const rules = (hue, at) => [
  `:root${at} { ${day(hue)} }`,
  `@media (prefers-color-scheme: dark) { :root${at}:not([data-theme="light"]) { ${night(hue)} } }`,
  `:root[data-theme="dark"]${at} { ${night(hue)} }`,
];

export function tintCss(tint = current.tint) {
  const lines = HUES.includes(tint) ? rules(tint, '') : [];
  for (const hue of HUES) lines.push(...rules(hue, `[data-tint="${hue}"]`));
  return `${lines.join('\n')}\n`;
}
