const DEFAULTS = {
  title: '',
  since: new Date().getFullYear(),
  tint: null,
  menu: '/menu/',
  cart: '/cart/',
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

const rule = (hue, at) => `:root${at} { --accent: var(--${hue}); }`;

export function tintCss(tint = current.tint) {
  const lines = HUES.includes(tint) ? [rule(tint, '')] : [];
  for (const hue of HUES) lines.push(rule(hue, `[data-tint="${hue}"]`));
  return `${lines.join('\n')}\n`;
}
