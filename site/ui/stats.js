const PATH = '/stats/stats.json';
const EVERY = 60 * 1000;

const cloud = document.querySelector('[data-stats="cloud"]');
const board = document.querySelector('[data-stats="errors"]');

/* FORMAT */

function ago(seconds) {
  if (!seconds) return 'never';
  const gap = Math.max(0, Math.floor(Date.now() / 1000 - seconds));
  if (gap < 90) return `${gap} s ago`;
  if (gap < 5400) return `${Math.round(gap / 60)} min ago`;
  if (gap < 172800) return `${(gap / 3600).toFixed(1)} h ago`;
  return `${Math.round(gap / 86400)} d ago`;
}

const size = (n) => (n == null ? '' : n < 1e6 ? `${(n / 1e3).toFixed(0)} kB` : n < 1e9 ? `${(n / 1e6).toFixed(1)} MB` : `${(n / 1e9).toFixed(2)} GB`);

const head = (label) => label.charAt(0).toUpperCase() + label.slice(1);

/* DRAW */

function table(rows) {
  const wrap = document.createElement('div');
  wrap.className = 'table';
  const out = document.createElement('table');
  const body = document.createElement('tbody');
  for (const [name, value] of rows) {
    const line = document.createElement('tr');
    const key = document.createElement('th');
    key.textContent = name;
    const cell = document.createElement('td');
    cell.textContent = value;
    line.append(key, cell);
    body.append(line);
  }
  out.append(body);
  wrap.append(out);
  return wrap;
}

function note(where, text) {
  const line = document.createElement('p');
  line.className = 'fine';
  line.textContent = text;
  where.replaceChildren(line);
}

function counts(bucket) {
  const rows = [];
  for (const [key, value] of Object.entries(bucket)) {
    if (key.endsWith('_bytes')) continue;
    if (key.endsWith('_objects')) {
      const label = key.slice(0, -8);
      rows.push([head(label), `${value} files (${size(bucket[`${label}_bytes`])})`]);
      continue;
    }
    rows.push([head(key), String(value)]);
  }
  return rows;
}

function rows(data) {
  const cdn = data.cdn ?? {};
  return [
    ['Stats', `${ago(data.at)}, last ${data.hours} h`],
    ['CDN', cdn.requests == null ? 'no distribution' : `${cdn.requests} requests, ${size(cdn.bytes)}, ${cdn.error_4xx}% 4xx, ${cdn.error_5xx}% 5xx`],
    ...counts(data.bucket ?? {}),
  ];
}

const runs = (data) =>
  Object.entries(data.lambdas ?? {}).map(([name, one]) => [name, `${one.invocations} runs, ${one.errors} errors, ${one.throttles} throttles, ${Math.round(one.duration_ms)} ms avg`]);

const lines = (data) =>
  Object.entries(data.errors ?? {}).flatMap(([name, list]) => list.map((one) => `${one.at ?? 'sometime'} ${one.level} ${name} ${one.message}`));

function draw(data) {
  if (!data) {
    note(cloud, 'No data yet.');
    note(board, 'No data yet.');
    return;
  }
  cloud.replaceChildren(table(rows(data)), table(runs(data)));
  const found = lines(data);
  if (!found.length) {
    note(board, 'None in the window.');
    return;
  }
  const block = document.createElement('pre');
  const text = document.createElement('code');
  text.textContent = found.join('\n');
  block.append(text);
  board.replaceChildren(block);
}

/* POLL */

async function grab() {
  try {
    const reply = await fetch(PATH, { cache: 'no-store' });
    return reply.ok ? await reply.json() : null;
  } catch {
    return null;
  }
}

let timer = 0;

const refresh = async () => draw(await grab());

function run() {
  clearInterval(timer);
  if (document.hidden) return;
  void refresh();
  timer = setInterval(refresh, EVERY);
}

if (cloud && board) {
  document.addEventListener('visibilitychange', run);
  run();
}
