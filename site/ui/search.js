const CAP = 24;
const live = new Map();
const pulls = new Map();

/* RULE */

export const terms = (query) => query.toLowerCase().split(/\s+/).filter(Boolean);

export function meets(words, text) {
  const hay = text.toLowerCase();
  return words.every((word) => hay.includes(word));
}

/* INDEX */

function pull(url) {
  if (!pulls.has(url)) {
    pulls.set(
      url,
      fetch(url)
        .then((reply) => (reply.ok ? reply.json() : Promise.reject(new Error(`search: ${url} did not answer`))))
        .catch(() => {
          pulls.delete(url);
          return [];
        }),
    );
  }
  return pulls.get(url);
}

/* TILES */

function tile(model, [title, route, dark, light]) {
  const a = model.cloneNode(true);
  a.setAttribute('href', route);
  a.querySelector('h2').textContent = title;
  a.querySelector('source').srcset = dark;
  a.querySelector('img').src = light;
  return a;
}

function find(words, doors, model, rows) {
  const seen = new Set();
  const out = [];
  for (const door of doors) {
    const href = door.getAttribute('href');
    if (seen.has(href) || !meets(words, `${door.textContent} ${href}`)) continue;
    seen.add(href);
    out.push(door.cloneNode(true));
  }
  for (const row of rows) {
    if (out.length >= CAP) break;
    if (seen.has(row[1]) || !meets(words, `${row[0]} ${row[1]}`)) continue;
    seen.add(row[1]);
    out.push(tile(model, row));
  }
  return out.slice(0, CAP);
}

/* ISLAND */

export function mount(host) {
  if (live.has(host)) return;
  const doors = [...host.querySelectorAll('a.tile')].filter((a) => !a.getAttribute('href').startsWith('#'));
  const model = doors.find((a) => a.querySelector('picture'));
  const urls = (host.dataset.search ?? '').split(' ').filter(Boolean);
  if (!model || !urls.length) return;
  const form = document.createElement('form');
  const input = document.createElement('input');
  const said = document.createElement('p');
  const grid = document.createElement('div');
  const gate = new AbortController();
  const { signal } = gate;
  let turn = 0;
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
  grid.className = 'gallery grid';
  grid.hidden = true;
  form.append(input, grid, said);
  const run = async () => {
    const mine = ++turn;
    const words = terms(input.value);
    const on = words.length > 0;
    for (const child of host.children) if (child !== form) child.hidden = on;
    grid.hidden = !on;
    said.hidden = !on;
    said.textContent = '';
    if (!on) return grid.replaceChildren();
    const near = find(words, doors, model, []);
    grid.replaceChildren(...near);
    const rows = (await Promise.all(urls.map(pull))).flat();
    if (mine !== turn) return;
    const hits = find(words, doors, model, rows);
    if (hits.length !== near.length) grid.replaceChildren(...hits);
    said.textContent = hits.length ? (hits.length < CAP ? `${hits.length} found` : `The first ${CAP}`) : 'Nothing found.';
  };
  input.addEventListener('focus', () => urls.forEach(pull), { signal });
  input.addEventListener('input', run, { signal });
  input.addEventListener('keydown', (event) => {
    if (event.key !== 'Escape' || !input.value) return;
    event.preventDefault();
    input.value = '';
    run();
  }, { signal });
  form.addEventListener('submit', (event) => {
    event.preventDefault();
    run().then(() => grid.querySelector('a')?.click());
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
