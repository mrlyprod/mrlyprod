export const SIDE = 1024;

export const LOOKS = ['default', 'dark', 'clear-light', 'clear-dark', 'tinted-light', 'tinted-dark'];

const APPLE = 'M39.75 0L60.25 0C71.7 0 77.42 0 82.58 1.7L83.58 1.95C90.31 4.4 95.6 9.69 98.05 16.42C100 22.58 100 28.3 100 39.75L100 60.25C100 71.7 100 77.42 98.3 82.58L98.05 83.58C95.6 90.31 90.31 95.6 83.58 98.05C77.42 100 71.7 100 60.25 100L39.75 100C28.3 100 22.58 100 17.42 98.3L16.42 98.05C9.69 95.6 4.4 90.31 1.95 83.58C0 77.42 0 71.7 0 60.25L0 39.75C0 28.3 0 22.58 1.7 17.42L1.95 16.42C4.4 9.69 9.69 4.4 16.42 1.95C22.58 0 28.3 0 39.75 0Z';

const scale = (k) => APPLE.replace(/\d+(?:\.\d+)?/g, (x) => String(Math.round(Number(x) * k * 1e4) / 1e4));

export const SHAPE = scale(SIDE / 100);

export const BOX = scale(1 / 100);

const DEFS = new Set(['linearGradient', 'radialGradient', 'stop', 'clipPath', 'mask', 'filter', 'feGaussianBlur', 'feOffset', 'feFlood', 'feComposite', 'feMerge', 'feMergeNode', 'feColorMatrix', 'feBlend', 'feMorphology', 'feComponentTransfer', 'feFuncR', 'feFuncG', 'feFuncB', 'feFuncA']);
const SHAPES = new Set(['g', 'path', 'rect', 'circle', 'ellipse', 'line', 'polyline', 'polygon', 'use']);
const WRAPS = new Set(['linearGradient', 'radialGradient', 'clipPath', 'mask', 'filter']);
const ATTR = /^(?:[a-z][a-z0-9-]*|[a-zA-Z]+|xlink:href)$/;
const BANNED = /^(?:on|style$|class$|xmlns|xml:)/;
const MONO = new Set(['hide', 'full', 'dim']);
const LINKS = /^(?:fill|stroke|stop-color|flood-color|clip-path|mask|filter|data-dark|data-dark-stroke|data-key)$/;
const WHITE = /^(?:#fff|#ffffff|white)$/i;

const fail = (id, text) => new Error(`appicon: ${id}/icon.svg ${text}`);

/* PARSE */

export function parse(text, id = '?') {
  const src = text.replace(/<\?xml[\s\S]*?\?>/g, '').replace(/<!--[\s\S]*?-->/g, '');
  const root = { name: '', attrs: [], kids: [] };
  const stack = [root];
  const tag = /<(\/?)([a-zA-Z][\w:-]*)((?:\s+[\w:-]+\s*=\s*(?:"[^"]*"|'[^']*'))*)\s*(\/?)>/g;
  let at = 0;
  for (let m = tag.exec(src); m; m = tag.exec(src)) {
    if (src.slice(at, m.index).trim()) throw fail(id, `holds text before <${m[2]}>; an icon is shapes only`);
    at = tag.lastIndex;
    const [, close, name, rest, self] = m;
    if (close) {
      const open = stack.pop();
      if (open?.name !== name) throw fail(id, `closes <${name}> while <${open?.name}> is open`);
      continue;
    }
    const attrs = [...rest.matchAll(/([\w:-]+)\s*=\s*(?:"([^"]*)"|'([^']*)')/g)].map(([, key, a, b]) => [key, a ?? b]);
    const node = { name, attrs, kids: [] };
    stack.at(-1).kids.push(node);
    if (!self) stack.push(node);
  }
  if (src.slice(at).trim()) throw fail(id, 'ends in text or a broken tag');
  if (stack.length !== 1) throw fail(id, `never closes <${stack.at(-1).name}>`);
  if (root.kids.length !== 1 || root.kids[0].name !== 'svg') throw fail(id, 'is not one <svg>');
  return root.kids[0];
}

const get = (node, key) => node?.attrs.find(([k]) => k === key)?.[1];

const escape = (value) => value.replace(/&(?!(?:[a-z]+|#\d+|#x[\da-f]+);)/gi, '&amp;').replace(/"/g, '&quot;').replace(/</g, '&lt;');

function write(node) {
  const attrs = node.attrs.map(([k, v]) => ` ${k}="${escape(v)}"`).join('');
  return node.kids.length ? `<${node.name}${attrs}>${node.kids.map(write).join('')}</${node.name}>` : `<${node.name}${attrs}/>`;
}

/* FACE */

function check(id, svg) {
  if (get(svg, 'viewBox')?.trim().split(/[\s,]+/).join(' ') !== `0 0 ${SIDE} ${SIDE}`) throw fail(id, `needs viewBox="0 0 ${SIDE} ${SIDE}"`);
  const ids = new Set();
  const walk = (node, inDefs) => {
    for (const kid of node.kids) {
      if (!DEFS.has(kid.name) && !SHAPES.has(kid.name) && kid.name !== 'defs') throw fail(id, `uses <${kid.name}>; an icon draws with ${[...SHAPES].join(', ')} and keeps paint servers in <defs>`);
      if (WRAPS.has(kid.name) && !inDefs) throw fail(id, `puts <${kid.name}> outside <defs>`);
      if (kid.name === 'defs' && node !== svg) throw fail(id, 'nests <defs>; keep one at the top');
      for (const [key, value] of kid.attrs) {
        if (BANNED.test(key) && !(key === 'class' && value === 'back' && node === svg)) throw fail(id, `sets ${key} on <${kid.name}>; CSS owns the looks, so no style, class or handlers`);
        if (!ATTR.test(key)) throw fail(id, `sets the unknown attribute ${key} on <${kid.name}>`);
        if (key === 'id') {
          if (!/^[\w-]+$/.test(value)) throw fail(id, `names an id ${value}; ids are letters, digits, - and _`);
          if (ids.has(value)) throw fail(id, `repeats the id ${value}`);
          ids.add(value);
        }
        if ((key === 'href' || key === 'xlink:href') && !value.startsWith('#')) throw fail(id, `links ${value}; only #ids inside the icon`);
        if (key === 'data-mono' && !MONO.has(value)) throw fail(id, `sets data-mono="${value}"; it is ${[...MONO].join(', ')}`);
        if (key.startsWith('data-') && !['data-mono', 'data-dark', 'data-dark-stroke', 'data-key'].includes(key)) throw fail(id, `sets ${key}; the data attributes are data-dark, data-dark-stroke, data-mono and data-key`);
        if (/url\((?!#)/.test(value)) throw fail(id, `points ${key} outside the icon`);
      }
      walk(kid, inDefs || kid.name === 'defs');
    }
  };
  walk(svg, false);
  return ids;
}

export function appicon(id, text) {
  const svg = parse(text, id);
  const ids = check(id, svg);
  const lost = [...text.matchAll(/url\(#([\w-]+)\)|href="#([\w-]+)"/g)].map((m) => m[1] ?? m[2]).find((name) => !ids.has(name));
  if (lost) throw fail(id, `points at #${lost}, which it never defines`);
  if (svg.kids.slice(1).some((kid) => kid.name === 'defs')) throw fail(id, 'puts <defs> after a shape; it comes first');
  const body = svg.kids.filter((kid) => kid.name !== 'defs');
  if (get(body[0], 'class') !== 'back') throw fail(id, 'starts with no class="back"; the first shape is the plate');
  if (!body[1]) throw fail(id, 'draws nothing on its plate');
  const local = (value) => value.replace(/url\(#([\w-]+)\)/g, (m, name) => `url(#${id}-${name})`);
  const defs = new Map(svg.kids.filter((kid) => kid.name === 'defs').flatMap((kid) => kid.kids).map((kid) => [get(kid, 'id'), kid]));
  const key = get(body[0], 'data-key') ?? keyOf(get(body[0], 'fill'), defs);
  if (!key) throw fail(id, 'has a back whose fill gives no key colour; set data-key on the back');
  const dress = (node) => {
    const attrs = [];
    const style = [];
    const classes = [];
    for (const [k, v] of node.attrs) {
      const value = LINKS.test(k) ? local(v) : v;
      if (k === 'href' || k === 'xlink:href') attrs.push(['href', `#${id}-${v.slice(1)}`]);
      else if (k === 'id') attrs.push(['id', `${id}-${v}`]);
      else if (k === 'class') classes.push(v);
      else if (k === 'data-key') continue;
      else if (k === 'data-dark') {
        style.push(`--d:${value}`);
        classes.push('d');
      } else if (k === 'data-dark-stroke') {
        style.push(`--ds:${value}`);
        classes.push('ds');
      } else attrs.push([k, value]);
    }
    if (!classes.length && WHITE.test(get(node, 'fill') ?? '')) classes.push('w');
    if (classes.length) attrs.unshift(['class', classes.join(' ')]);
    if (style.length) attrs.push(['style', style.join(';')]);
    return { ...node, attrs, kids: node.kids.map(dress) };
  };
  const [back, ...ink] = body.map(dress);
  return {
    defs: [...defs.values()].map(dress).map(write).join(''),
    art: `${write(back)}<g class="ink" mask="url(#appicon-fade)" style="--key:${key}">${ink.map(write).join('')}</g>`,
  };
}

function keyOf(fill, defs) {
  if (!fill) return null;
  const at = /^url\(#([\w-]+)\)$/.exec(fill);
  if (!at) return fill;
  const stops = defs.get(at[1])?.kids.filter((kid) => kid.name === 'stop') ?? [];
  return get(stops.at(-1), 'stop-color') ?? null;
}

/* MARKUP */

export const markup = (art) => `<svg class="appicon" viewBox="0 0 ${SIDE} ${SIDE}" aria-hidden="true" focusable="false"><g clip-path="url(#appicon-clip)">${art}<path class="rim" d="${SHAPE}"/></g></svg>`;

const stops = (list) => list.map(([at, color, alpha = 1]) => `<stop offset="${at}" stop-color="${color}"${alpha === 1 ? '' : ` stop-opacity="${alpha}"`}/>`).join('');

const down = (id, list) => `<linearGradient id="${id}" x1="0" y1="0" x2="0" y2="1">${stops(list)}</linearGradient>`;

const rim = (id, top, low, side) => down(id, [[0, '#fff', top], [0.035, '#fff', 0], [0.09, '#000', side], [0.91, '#000', side], [0.965, '#fff', 0], [1, '#fff', low]]);

const LIFT = '<feGaussianBlur in="A" stdDeviation="22"/><feOffset dy="34" result="b"/><feFlood flood-color="#000" flood-opacity=".09"/><feComposite in2="b" operator="in" result="s"/>';

const mono = (id, flood) => `<filter id="${id}" filterUnits="userSpaceOnUse" x="-64" y="-64" width="${SIDE + 128}" height="${SIDE + 128}" color-interpolation-filters="sRGB"><feColorMatrix in="SourceGraphic" type="matrix" values="0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 .2126 .7152 .0722 0 0" result="l"/><feComponentTransfer in="l" result="g"><feFuncA type="gamma" amplitude="1" exponent=".55" offset="0"/></feComponentTransfer><feComposite in="g" in2="SourceAlpha" operator="in" result="A"/><feFlood ${flood} result="c"/><feComposite in="c" in2="A" operator="in" result="ink"/>${LIFT}<feMerge><feMergeNode in="s"/><feMergeNode in="ink"/></feMerge></filter>`;

export const SPRITE = [
  `<clipPath id="appicon-clip"><path d="${SHAPE}"/></clipPath>`,
  `<clipPath id="appicon-shape" clipPathUnits="objectBoundingBox"><path d="${BOX}"/></clipPath>`,
  `<mask id="appicon-fade" maskUnits="userSpaceOnUse" x="0" y="0" width="${SIDE}" height="${SIDE}" maskContentUnits="objectBoundingBox"><rect x="-.1" y="0" width="1.2" height="1.25" fill="url(#appicon-fall)"/></mask>`,
  down('appicon-fall', [[0, '#fff'], [0.2, '#fff', 0.96], [0.4, '#fff', 0.85], [0.6, '#fff', 0.68], [0.8, '#fff', 0.5], [1, '#fff', 0.5]]),
  rim('appicon-rim', 0.45, 0.25, 0.08),
  rim('appicon-rim-dark', 0.6, 0.35, 0.5),
  rim('appicon-rim-clear-light', 0.56, 0.4, 0.09),
  rim('appicon-rim-clear-dark', 0.27, 0.2, 0.3),
  down('appicon-dark', [[0, '#212121'], [1, '#0d0d0d']]),
  `<linearGradient id="appicon-tint-dark" x1="0" y1="0" x2="0" y2="1"><stop offset="0" style="stop-color:color-mix(in srgb, var(--accent) 15%, #19191a)"/><stop offset="1" stop-color="#19191a"/></linearGradient>`,
  `<linearGradient id="appicon-tint-light" x1="0" y1="0" x2="0" y2="1"><stop offset="0" style="stop-color:color-mix(in oklab, var(--accent) 63%, #666)"/><stop offset="1" style="stop-color:color-mix(in oklab, var(--accent) 63%, #666)"/></linearGradient>`,
  down('appicon-clear-light', [[0, '#fff', 0.14], [1, '#fff', 0.04]]),
  down('appicon-clear-dark', [[0, '#000', 0.5], [1, '#000', 0.66]]),
  `<filter id="appicon-lift" filterUnits="userSpaceOnUse" x="-64" y="-64" width="${SIDE + 128}" height="${SIDE + 128}" color-interpolation-filters="sRGB"><feComposite in="SourceAlpha" in2="SourceAlpha" operator="over" result="A"/>${LIFT}<feMerge><feMergeNode in="s"/><feMergeNode in="SourceGraphic"/></feMerge></filter>`,
  mono('appicon-white', 'flood-color="#fff"'),
  mono('appicon-white-soft', 'flood-color="#f2f2f2" flood-opacity=".95"'),
  mono('appicon-accent', 'style="flood-color:var(--accent)"'),
].join('');
