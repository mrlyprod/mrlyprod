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

const boot = (prefix) => `(()=>{const d=document.documentElement;d.classList.add('js');try{for(const k of['theme','font','tint','saver']){const v=localStorage.getItem('${prefix}'+k);if(v)d.dataset[k]=v}}catch{}})()`;

const claims = `(()=>{const f=document.querySelector("form.filter"),b=[...f.querySelectorAll("button")],s=f.querySelector("select"),d=f.querySelector("input"),o=f.querySelector("output"),secs=[...document.querySelectorAll("section.claims")];let tag="";const run=()=>{let n=0;for(const sec of secs){let k=0;for(const li of sec.querySelectorAll("li[data-tag]")){const on=(!tag||li.dataset.tag===tag)&&(!s.value||sec.dataset.slug===s.value)&&(!d.value||li.dataset.date>=d.value);li.hidden=!on;if(on)k++}sec.hidden=!k;n+=k}o.textContent=n+" claims"};for(const x of b)x.addEventListener("click",()=>{tag=x.dataset.tag;for(const y of b)y.classList.toggle("on",y===x);run()});s.addEventListener("change",run);d.addEventListener("input",run)})()`;

export function headScript(prefix = current.prefix || 'mrly-') {
  return `<script data-boot>${boot(prefix)}</script>`;
}

export const claimsScript = () => `<script>${claims}</script>`;

export const inlineScripts = (prefix = current.prefix || 'mrly-') => [boot(prefix), claims];

/* TINT */

export const HUES = ['red', 'orange', 'yellow', 'green', 'mint', 'teal', 'cyan', 'blue', 'indigo', 'purple', 'pink', 'brown'];

const rule = (hue, at) => `:root${at} { --accent: var(--${hue}); }`;

export function tintCss(tint = current.tint) {
  const lines = HUES.includes(tint) ? [rule(tint, '')] : [];
  for (const hue of HUES) lines.push(rule(hue, `[data-tint="${hue}"]`));
  return `${lines.join('\n')}\n`;
}
