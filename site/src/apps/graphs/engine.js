import { num, path, sheet } from '../../lib/app/svg.js';
import { canon, roll, title } from '../../lib/designs/engine.js';

export const NUMBERS = [3, 5, 7];
export const DIMS = [[2, '2D'], [3, '3D']];
export const VIEWS = [['cube', '3D'], ['hex', '6D']];
export const NAMES = { core: 'Core, filled cells', edge: 'Edge, corners and sides', tunnel: 'Tunnel, empty cells', dual: 'Dual, fills and voids' };
export const KINDS = { flat: ['core', 'edge', 'tunnel'], cube: ['core', 'edge', 'tunnel'], hex: ['core', 'dual', 'edge'] };
export const SETS = [
  ['Carpet', { dim: 2, base: 3, code: '', number: 3, graph: 'core', level: 2, layout: 'lattice' }],
  ['Carpet tunnels', { dim: 2, base: 3, code: '', number: 3, graph: 'tunnel', level: 3, layout: 'lattice' }],
  ['Carpet relaxed', { dim: 2, base: 3, code: '', number: 3, graph: 'edge', level: 2, layout: 'force' }],
  ['Sponge', { dim: 3, view: 'cube', base: 3, code: '', number: 3, graph: 'core', level: 2, layout: 'lattice' }],
  ['Cube relaxed', { dim: 3, view: 'cube', base: 2, code: '', number: 3, graph: 'core', level: 2, layout: 'force' }],
  ['Sponge slice', { dim: 3, view: 'hex', base: 3, code: '', number: 3, graph: 'dual', level: 2, layout: 'lattice' }],
];
export const LAYOUTS = [['lattice', 'Lattice'], ['force', 'Force']];
export const CAMERAS = [['eye', 'Perspective'], ['iso', 'Isometric']];
export const BUDGET = { lattice: 20000, force: 2000 };
export const COOL = 360;
export const TICK = 3;
export const DEEPEST = 8;
export const PAD = 16;
const SEEN = 3;
const DEPTH = 2;
const SKEW = 0.5;
const KEPT = 128;
const SAID = ['Alone', 'Tip', 'Through', 'Junction'];
const LOAD = 4e-6;
const SNAP = 200;
const GRID = 2e6;
const ROLLS = 32;
const LEAST = 12;
const LOW = 2;
const ASPECT = [0.5, Math.sqrt(3) / 4];

const pick = (rand, list) => list[Math.floor(rand() * list.length)];

const count = (x) => (x === undefined || x === null ? null : Number(x));

/* SPACE */

export const spaceOf = (value) => (Number(value?.dim) === 3 ? (value.view === 'hex' ? 'hex' : 'cube') : 'flat');

export const solid = (value) => spaceOf(value) === 'cube';

export const design = (value) => ({ dim: Number(value?.dim) === 3 ? 3 : 2, base: value?.base, code: String(value?.code ?? '').trim() });

export const kindOf = (value) => (KINDS[spaceOf(value)].includes(value?.graph) ? value.graph : KINDS[spaceOf(value)][0]);

export const graphs = (value) => KINDS[spaceOf(value)].map((one) => [one, NAMES[one]]);

export function turn(value, by) {
  const list = KINDS[spaceOf(value)];
  return list[(list.indexOf(kindOf(value)) + by + list.length) % list.length];
}

/* ROOM */

export function bound(math, space, base, code, number, level, graph) {
  if (space === 'hex') return Number(graph === 'edge' ? math.counts.six.solid_slice_vertices(number ** level) : math.counts.six.grid_triangles(number, level));
  const dim = space === 'cube' ? 3 : 2;
  if (graph === 'tunnel') return Number(math.counts.void_(code, number, dim, level, base));
  return Number(math.counts.fill(code, number, dim, level, base)) * (graph === 'edge' ? 2 ** dim : 1);
}

export function top(math, value, room) {
  const space = spaceOf(value);
  const { dim, base, code } = design(value);
  const settled = canon(math, dim, base, code);
  const budget = room ?? BUDGET[value.layout] ?? BUDGET.force;
  const fits = (level) => {
    if (value.number ** (level * dim) > GRID) return false;
    try {
      return bound(math, space, base, settled, value.number, level, kindOf(value)) <= budget;
    } catch {
      return false;
    }
  };
  let level = 1;
  while (level < DEEPEST && fits(level + 1)) level++;
  return level;
}

/* NETWORK */

function aspect(math, net) {
  const data = net.toJSON();
  net.free();
  return math.graph.Network.from({ ...data, nodes: data.nodes.map((node) => ({ ...node, position: node.position.map((p, a) => p * ASPECT[a]) })) });
}

export function build(math, space, base, code, number, level, graph) {
  if (space === 'flat') {
    const cell = math.two.create(code, number, level, 0, base);
    return { side: cell.shape[0], net: math.graph[`${graph}_graph`]({ shape: cell.shape, data: cell.types }), euler: count(math.two.census(cell).euler) };
  }
  const cell = math.three.create(code, number, level, base);
  if (space === 'cube') return { side: cell.shape[0], net: math.graph[`${graph}_graph`]({ shape: cell.shape, data: cell.types }), euler: count(math.three.census(cell).euler) };
  const cut = math.six.cut(cell);
  const net = graph === 'edge' ? math.six.slice_edge_graph(cut, math.six.FILL()) : math.six[`slice_${graph}_graph`](cut);
  return { side: cell.shape[0], net: aspect(math, net), euler: count(math.six.fills_only(cut).euler) };
}

export function flatten(net) {
  const dim = net.dim;
  const nodes = net.nodes;
  const positions = new Float64Array(nodes.length * dim);
  nodes.forEach((node, i) => positions.set(node.position, i * dim));
  const branches = net.branches;
  const pairs = new Uint32Array(branches.length * 2);
  branches.forEach((branch, j) => {
    pairs[2 * j] = branch.parent;
    pairs[2 * j + 1] = branch.child;
  });
  return { dim, count: nodes.length, positions, pairs };
}

const sights = new Map();

export function glimpse(math, value, graph) {
  const space = spaceOf(value);
  const { dim, base, code } = design(value);
  const at = [space, base, code, graph].join(':');
  if (!sights.has(at)) {
    if (sights.size >= KEPT) sights.clear();
    const { net } = build(math, space, base, canon(math, dim, base, code), SEEN, DEPTH, graph);
    sights.set(at, flatten(net));
    net.free();
  }
  return sights.get(at);
}

export function plane({ dim, count, positions }) {
  if (dim === 2) return positions;
  const out = new Float64Array(count * 2);
  for (let i = 0; i < count; i++) {
    out[2 * i] = positions[3 * i] + SKEW * positions[3 * i + 2];
    out[2 * i + 1] = positions[3 * i + 1] + SKEW * positions[3 * i + 2];
  }
  return out;
}

export function roles(math, net) {
  return Uint8Array.from(math.graph.roles(net), (role) => Math.max(0, SAID.indexOf(role)));
}

export function body(role) {
  let on = 0;
  for (const r of role) if (r) on++;
  return on;
}

export const joined = (big, count) => big * 10 >= count * 9;

export const worthy = (role, big) => body(role) >= LEAST && body(role) * 2 > role.length && joined(big, role.length);

export function giant(math, net) {
  const part = math.graph.largest_component(net);
  const size = part.nodes.length;
  part.free();
  return size;
}

/* DEAL */

export function deal(math, value, rand, ask = { code: true, graph: true, level: true, layout: true }, room) {
  const space = spaceOf(value);
  const { dim, base, code: kept } = design(value);
  let best = null;
  for (let go = 0; go < ROLLS; go++) {
    const code = ask.code ? roll(math, dim, base, rand) : canon(math, dim, base, kept);
    const graph = ask.graph ? pick(rand, KINDS[space]) : kindOf(value);
    const layout = ask.layout ? pick(rand, ['lattice', 'force']) : value.layout;
    const asked = { ...value, code, graph, layout };
    const most = top(math, asked, room);
    const low = Math.min(LOW, most);
    const level = ask.level ? low + Math.floor(rand() * (most - low + 1)) : Math.min(value.level, most);
    const { net } = build(math, space, base, code, value.number, Math.min(level, top(math, { ...asked, layout: 'force' })), graph);
    const role = roles(math, net);
    const big = giant(math, net);
    net.free();
    const fine = worthy(role, big);
    if (fine || !best || big > best.big) best = { code, graph, layout, level, big, whole: joined(big, role.length) };
    if (fine || (!ask.code && !ask.graph && !ask.level)) break;
  }
  const { code, graph, layout, level, whole } = best;
  return { code, graph, level, layout: ask.layout && !whole ? 'lattice' : layout };
}

/* STUDY */

export function study(math, value) {
  const space = spaceOf(value);
  const { dim, base, code: asked } = design(value);
  const code = canon(math, dim, base, asked);
  const graph = kindOf(value);
  const most = top(math, { ...value, code });
  const level = Math.max(1, Math.min(value.level, most));
  const { side, net, euler } = build(math, space, base, code, value.number, level, graph);
  const census = math.graph.census(net);
  const { dim: drawn, count: nodes, positions, pairs } = flatten(net);
  const role = roles(math, net);
  const links = pairs.length / 2;
  const shade = { nodes: role, branches: Uint8Array.from({ length: links }, (_, j) => Math.min(role[pairs[2 * j]], role[pairs[2 * j + 1]])) };
  const facts = {
    code,
    name: math.name.of(code, dim, base) ?? '',
    title: title(math, dim, base, code),
    space,
    dim,
    base,
    number: value.number,
    level,
    top: most,
    side,
    graph,
    layout: value.layout,
    nodes: census.nodes,
    branches: census.branches,
    tips: census.tips,
    junctions: census.junctions,
    pieces: census.components,
    length: census.total_length,
    box: census.fractal_dimension,
    euler,
  };
  return { space, dim: drawn, count: nodes, positions, pairs, shade, net, facts };
}

/* RELAX */

export const pace = (count, dim) => Math.max(TICK, count * count * dim * LOAD);

export const quick = (count, dim) => COOL * count * count * dim * LOAD <= SNAP;

/* FIT */

export function bounds(positions, dim, count) {
  const box = new Float64Array(2 * dim);
  for (let a = 0; a < dim; a++) {
    box[2 * a] = count ? Infinity : 0;
    box[2 * a + 1] = count ? -Infinity : 0;
  }
  for (let i = 0; i < count; i++) {
    for (let a = 0; a < dim; a++) {
      const p = positions[i * dim + a];
      if (p < box[2 * a]) box[2 * a] = p;
      if (p > box[2 * a + 1]) box[2 * a + 1] = p;
    }
  }
  return box;
}

export function lay(box, w, h, pad) {
  const [x0, x1, y0, y1] = box;
  const k = Math.min((w - 2 * pad) / (x1 - x0 || 1), (h - 2 * pad) / (y1 - y0 || 1));
  return { k, ox: w / 2 - ((x0 + x1) / 2) * k, oy: h / 2 - ((y0 + y1) / 2) * k };
}

export function cube(box) {
  let span = 0;
  const mid = [0, 1, 2].map((a) => (box[2 * a] + box[2 * a + 1]) / 2);
  for (let a = 0; a < 3; a++) span = Math.max(span, box[2 * a + 1] - box[2 * a]);
  return { mid, scale: 2 / (span || 1) };
}

/* SVG */

export function picture(w, h, dots, lines, radius, width, shades) {
  const strokes = shades.map((color, s) => {
    const d = lines.filter((line) => line[4] === s).map(([x1, y1, x2, y2]) => path([x1, y1, x2, y2])).join('');
    return d ? `<path d="${d}" fill="none" stroke="${color}" stroke-width="${num(width)}" stroke-linecap="round"/>` : '';
  });
  const marks = shades.map((color, s) => {
    const body = dots.filter((dot) => dot[2] === s).map(([x, y]) => `<circle cx="${num(x)}" cy="${num(y)}" r="${num(radius)}"/>`).join('');
    return body ? `<g fill="${color}">${body}</g>` : '';
  });
  return sheet(w, h, strokes.join('') + marks.join(''));
}
