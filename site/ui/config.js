import { HUES } from './hues.js';

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

const boot = (prefix) => `(()=>{const d=document.documentElement;d.classList.add('js');try{for(const k of['theme','font','tint']){const v=localStorage.getItem('${prefix}'+k);if(v)d.dataset[k]=v}if(location.pathname==='/'&&CSS.supports('animation-timeline','scroll()'))d.dataset.welcome=localStorage.getItem('${prefix}welcome')||matchMedia('(prefers-reduced-motion: reduce)').matches?'shut':'open'}catch{}})()`;

export function headScript(prefix = current.prefix || 'mrly-') {
  return `<script data-boot>${boot(prefix)}</script>`;
}

export const inlineScripts = (prefix = current.prefix || 'mrly-') => [boot(prefix)];

/* TINT */

const rule = (hue, at) => `:root${at} { --accent: var(--${hue}); --link: var(--${hue}-link); }`;

export function tintCss(tint = current.tint) {
  const lines = HUES.includes(tint) ? [rule(tint, '')] : [];
  for (const hue of HUES) lines.push(rule(hue, `[data-tint="${hue}"]`));
  return `${lines.join('\n')}\n`;
}
