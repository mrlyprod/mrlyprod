const CAP = 24;
const live = new Map();

/* RULE */

export const terms = (query) => query.toLowerCase().split(/\s+/).filter(Boolean);

export function meets(words, text) {
  const hay = text.toLowerCase();
  return words.every((word) => hay.includes(word));
}

export function find(words, list) {
  const seen = new Set();
  const out = [];
  for (const one of list) {
    if (out.length >= CAP) break;
    if (seen.has(one.route) || !meets(words, `${one.title} ${one.route} ${one.lead ?? ''}`)) continue;
    seen.add(one.route);
    out.push(one);
  }
  return out;
}

/* RESULTS */

function result(one) {
  const item = document.createElement('li');
  const a = document.createElement('a');
  const title = document.createElement('b');
  const path = document.createElement('span');
  a.href = one.route;
  title.textContent = one.title;
  path.textContent = one.route;
  a.append(title, path);
  item.append(a);
  if (one.lead) {
    const lead = document.createElement('p');
    lead.textContent = one.lead;
    item.append(lead);
  }
  return item;
}

/* MOUNT */

export function mount(host, rows) {
  if (live.has(host)) return;
  const extra = [...host.querySelectorAll('a.tile[href]')].map((a) => ({ title: a.querySelector('h2')?.textContent ?? '', route: a.getAttribute('href'), lead: '' }));
  const list = [...rows.filter((row) => !row.hidden), ...extra];
  const form = document.createElement('form');
  const input = document.createElement('input');
  const said = document.createElement('p');
  const hits = document.createElement('ol');
  const gate = new AbortController();
  const { signal } = gate;
  form.setAttribute('role', 'search');
  input.type = 'search';
  input.placeholder = 'Search';
  input.autocomplete = 'off';
  input.spellcheck = false;
  input.enterKeyHint = 'go';
  input.setAttribute('aria-label', 'Search');
  said.className = 'fine';
  said.setAttribute('role', 'status');
  said.hidden = true;
  hits.className = 'results';
  hits.hidden = true;
  form.append(input, hits, said);
  const run = () => {
    const words = terms(input.value);
    const on = words.length > 0;
    for (const child of host.children) if (child !== form) child.hidden = on;
    hits.hidden = !on;
    said.hidden = !on;
    if (!on) return hits.replaceChildren();
    const found = find(words, list);
    hits.replaceChildren(...found.map(result));
    said.textContent = found.length ? (found.length < CAP ? `${found.length} found` : `The first ${CAP}`) : 'Nothing found.';
  };
  input.addEventListener('input', run, { signal });
  input.addEventListener('keydown', (event) => {
    if (event.key !== 'Escape' || !input.value) return;
    event.preventDefault();
    input.value = '';
    run();
  }, { signal });
  form.addEventListener('submit', (event) => {
    event.preventDefault();
    run();
    hits.querySelector('a')?.click();
  }, { signal });
  host.prepend(form);
  live.set(host, { gate, form });
  const at = document.activeElement;
  if (matchMedia('(hover: hover)').matches && (!at || at.matches('body, #main'))) input.focus({ preventScroll: true });
}

export function unmount(host) {
  const held = live.get(host);
  if (!held) return;
  held.gate.abort();
  held.form.remove();
  for (const child of host.children) child.hidden = false;
  live.delete(host);
}
