import { num, path, sheet } from '../../lib/svg.js';
import { named, resolve, title, total, word } from '../designs/engine.js';

export const NUMBERS = [3, 5, 7];
export const GRAPHS = ['core', 'edge', 'tunnel'];
export const KINDS = [['', 'Random'], ['core', 'Core, filled cells'], ['edge', 'Edge, corners and sides'], ['tunnel', 'Tunnel, empty cells']];
export const LAYOUTS = [['', 'Random'], ['grid', 'Grid'], ['force', 'Force']];
export const BUDGET = { grid: 20000, force: 1000 };
export const ROLES = ['Alone', 'Tip', 'Through', 'Junction'];
export const FADES = [0.72, 0.48, 0.22, 0];
export const SETTLE = 360;
export const TICK = 10;
export const STILL = 24e6;
export const HOLD = 3000;
export const PAD = 24;
const DEEPEST = 8;
const ROLLS = 32;
const LEAST = 12;
const LOAD = 1e-5;
const DOORS = { core: 'core_graph', edge: 'edge_graph', tunnel: 'tunnel_graph' };

const pick = (rand, list) => list[Math.floor(rand() * list.length)];

export const dimOf = (value) => (Number(value.dim) === 3 ? 3 : 2);

/* ROOM */

export function bound(dim, number, level, graph) {
  return (number ** level + (graph === 'edge' ? 1 : 0)) ** dim;
}

export function cap(value) {
  const budget = BUDGET[value.layout] ?? BUDGET.force;
  const graph = value.graph || 'edge';
  const dim = dimOf(value);
  let level = 1;
  while (level < DEEPEST && bound(dim, value.number, level + 1, graph) <= budget) level++;
  return level;
}

export function choose(value, rand) {
  const graph = GRAPHS.includes(value.graph) ? value.graph : pick(rand, GRAPHS);
  const layout = value.layout === 'grid' || value.layout === 'force' ? value.layout : pick(rand, ['grid', 'force']);
  return { graph, layout };
}

export function steady(layout, count, dim, still) {
  return layout === 'force' && still && SETTLE * count * count * dim > STILL ? 'grid' : layout;
}

export function turn(graph, by) {
  const at = GRAPHS.indexOf(graph);
  return GRAPHS[(Math.max(at, 0) + by + GRAPHS.length) % GRAPHS.length];
}

/* NETWORK */

export function grow(math, dim, base, code, number, level) {
  return dim === 3 ? math.three.create(code, number, level, base) : math.two.create(code, number, level, 0, base);
}

export function extract(math, cell, graph) {
  return math.graph[DOORS[graph]]({ shape: cell.shape, data: cell.types });
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

export function roles(math, net) {
  return Uint8Array.from(math.graph.roles(net), (role) => Math.max(0, ROLES.indexOf(role)));
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

export function tally(role) {
  const out = ROLES.map(() => 0);
  for (const r of role) out[r]++;
  return out;
}

/* REVEAL */

function adjacency(count, pairs) {
  const start = new Uint32Array(count + 1);
  for (const i of pairs) start[i + 1]++;
  for (let i = 0; i < count; i++) start[i + 1] += start[i];
  const fill = start.slice(0, count);
  const next = new Uint32Array(pairs.length);
  for (let j = 0; j < pairs.length; j += 2) {
    const a = pairs[j];
    const b = pairs[j + 1];
    next[fill[a]++] = b;
    next[fill[b]++] = a;
  }
  return { start, next };
}

export function order(count, pairs, positions, dim) {
  const { start, next } = adjacency(count, pairs);
  const mid = new Float64Array(dim);
  for (let i = 0; i < count; i++) for (let a = 0; a < dim; a++) mid[a] += positions[i * dim + a] / count;
  const far = (i) => {
    let d = 0;
    for (let a = 0; a < dim; a++) d += (positions[i * dim + a] - mid[a]) ** 2;
    return d;
  };
  const piece = new Int32Array(count).fill(-1);
  const queue = new Uint32Array(count);
  const pieces = [];
  for (let s = 0; s < count; s++) {
    if (piece[s] >= 0) continue;
    const id = pieces.length;
    let head = 0;
    let tail = 0;
    queue[tail++] = s;
    piece[s] = id;
    let root = s;
    while (head < tail) {
      const u = queue[head++];
      if (far(u) < far(root)) root = u;
      for (let e = start[u]; e < start[u + 1]; e++) {
        const v = next[e];
        if (piece[v] >= 0) continue;
        piece[v] = id;
        queue[tail++] = v;
      }
    }
    pieces.push({ root, size: tail });
  }
  pieces.sort((p, q) => q.size - p.size || far(p.root) - far(q.root) || p.root - q.root);
  const nodes = new Uint32Array(count);
  const rank = new Uint32Array(count);
  const seen = new Uint8Array(count);
  let n = 0;
  for (const { root } of pieces) {
    let head = n;
    nodes[n++] = root;
    seen[root] = 1;
    while (head < n) {
      const u = nodes[head++];
      rank[u] = head - 1;
      for (let e = start[u]; e < start[u + 1]; e++) {
        const v = next[e];
        if (seen[v]) continue;
        seen[v] = 1;
        nodes[n++] = v;
      }
    }
  }
  const links = pairs.length / 2;
  const when = new Uint32Array(links);
  for (let j = 0; j < links; j++) when[j] = Math.max(rank[pairs[2 * j]], rank[pairs[2 * j + 1]]) + 1;
  const branches = Uint32Array.from({ length: links }, (_, j) => j).sort((p, q) => when[p] - when[q] || p - q);
  const shown = new Uint32Array(count + 1);
  for (const j of branches) shown[when[j]]++;
  for (let k = 1; k <= count; k++) shown[k] += shown[k - 1];
  return { nodes, rank, branches, shown, pieces: pieces.length };
}

/* STUDY */

const blank = (code) => {
  const text = String(code ?? '').trim();
  return text === '' || !Number.isFinite(Number(text));
};

const rolls = (value) => blank(value.code) || !GRAPHS.includes(value.graph);

export function study(math, value, rand, still = false) {
  const dim = dimOf(value);
  const { base, number, level } = value;
  const space = total(math, dim, base);
  let best = null;
  for (let go = 0; go < ROLLS; go++) {
    const code = String(resolve(value.code, space, rand));
    const chosen = choose(value, rand);
    const cell = grow(math, dim, base, code, number, level);
    const net = extract(math, cell, chosen.graph);
    const role = roles(math, net);
    const big = giant(math, net);
    const fine = worthy(role, big);
    if (fine || !best || big > best.big) {
      best?.net.free();
      best = { code, chosen, cell, net, role, big };
    } else net.free();
    if (fine || !rolls(value)) break;
  }
  const { code, chosen, cell, net, role, big } = best;
  const census = math.graph.census(net);
  const { count, positions, pairs } = flatten(net);
  const asked = value.layout === 'grid' || value.layout === 'force';
  const layout = steady(asked || joined(big, count) ? chosen.layout : 'grid', count, dim, still);
  const laid = order(count, pairs, positions, dim);
  const shade = {
    nodes: Uint8Array.from(laid.nodes, (n) => role[n]),
    branches: Uint8Array.from(laid.branches, (j) => Math.min(role[pairs[2 * j]], role[pairs[2 * j + 1]])),
  };
  const read = dim === 3 ? math.three.census(cell) : math.two.census(cell);
  const facts = {
    code,
    name: word(named(math, dim, base), code),
    title: title(math, dim, base, code),
    dim,
    base,
    number,
    level,
    side: cell.shape[0],
    graph: chosen.graph,
    layout,
    nodes: census.nodes,
    branches: census.branches,
    tips: census.tips,
    junctions: census.junctions,
    roles: tally(role),
    pieces: census.components,
    length: census.total_length,
    box: census.fractal_dimension,
    euler: Number(read.euler),
  };
  return { dim, count, big, positions, pairs, roles: role, order: laid, shade, net, graph: chosen.graph, layout, facts };
}

/* TIME */

export function share(t, span, still) {
  if (still || span <= 0) return 1;
  const loop = span + HOLD;
  const phase = ((t % loop) + loop) % loop;
  return Math.min(1, phase / span);
}

export const pace = (count, dim) => Math.max(TICK, Math.ceil(count * count * dim * LOAD));

export function ticks(t, span, still, step = TICK) {
  if (still) return SETTLE;
  const loop = span + HOLD;
  const at = span > 0 ? ((t % loop) + loop) % loop : Math.max(0, t);
  return Math.min(SETTLE, Math.floor(at / step));
}

export function reached(part, count, dim) {
  return Math.round(part ** dim * count);
}

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
