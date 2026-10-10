import { rng } from '../scene.js';
import { add, cross, dot, len, mix, mul, norm, rot, sub } from './vec.js';

const DIRS = [[1, 0, 0], [-1, 0, 0], [0, 1, 0], [0, -1, 0], [0, 0, 1], [0, 0, -1]];

export const NAMES = [[23, 'carpet'], [232, 'net'], [22, 'star'], [129, 'void'], [17, 'xtree'], [5, 'ytree'], [3, 'ztree']];

const HALF = [0.5, 0.5, 0.5];
const ZERO = [0, 0, 0];
const LOW = 0.4;
const HIGH = 0.9;
const WALLS = 2.2;
export const SPEED = 1.15;
const STRAIGHT = 0.45;
const FRESH = 0.3;
const TRAIL = 0.1;
const TWIST = 0.15;
const JITTER = 0.4;
const AHEAD = 2;
const PRELUDE = 2;
const KEEP = 4;
const DEPTH = 2;
const LEAD = 3;
const SAMPLES = 16;
const BACK = 2;
const TAPS = 8;
const STEP = 0.05;
const SWAY = 0.3;
const RAIL = 32;
const REACH = 6;
const SHARP = 0.05;
const LONG = 3;
const HOLD = 4;
const REST = 6;
const TURN = Math.PI * 2;
const REGION = 5;
const BEYOND = 1e6;
const SPAN = 3;
const CORRIDOR = 2.2;
const TRENCH = 1.5;
const FLOOR = 0.15;
const STEER = 0.3;
const HOVER = 0.3;
const RUNWAY = 2;
const HALVES = 60;
const RISE = 0.35;
const FADE = 0.6;
const DIP = 0.7;
const CLOSE = 0.8;
const FOG = 0.24;
const HUE = 0.17;
const DRIFT = 0.45;
const SALT = 0x2f6b8d31;
const GOLD = 0x9e3779b1;
const CHOICE = 0x7a3c19e5;

export const PILOTS = {
  steady: { window: 0.6, look: 1.5, gap: 0.3, lead: 0, bank: 0.15, tilt: 0.35, roll: 0.05, ease: 0.2, bend: 2, barrel: 0 },
  fighter: { window: 0.3, look: 1, gap: 0.5, lead: 0.5, bank: 0.4, tilt: 0.9, roll: 0.03, ease: 0.1, bend: 1, barrel: 2.5 },
  coaster: { window: 0.1, look: 1.2, gap: 0.8, lead: 0.8, bank: 0.6, tilt: 1.2, roll: 0, ease: 0, bend: 2, barrel: 0 },
};

/* RULE */

const kept = (code, i, j, k) => (code >> ((i & 1) * 4 + (j & 1) * 2 + (k & 1))) & 1;

export function cells(code, n) {
  const out = new Uint8Array(n * n * n);
  for (let z = 0; z < n; z++) for (let y = 0; y < n; y++) for (let x = 0; x < n; x++) out[(z * n + y) * n + x] = kept(code, x, y, z);
  return out;
}

const index = (n, c) => (c[2] * n + c[1]) * n + c[0];

const unindex = (n, k) => [k % n, Math.floor(k / n) % n, Math.floor(k / (n * n))];

const read = (all, n, c) => (c.every((v) => v >= 0 && v < n) ? all[index(n, c)] : -1);

const axis = (g) => g >> 1;

const flip = (g) => g ^ 1;

const entry = (g, n) => DIRS[g].map((d) => (d > 0 ? 0 : d < 0 ? n - 1 : (n - 1) / 2));

/* DOORS */

export function doors(all, n) {
  const open = DIRS.map((_, g) => read(all, n, entry(g, n)) === 0);
  const table = DIRS.map((_, g) => {
    if (!open[g]) return null;
    const start = entry(g, n);
    const from = new Map([[index(n, start), null]]);
    const queue = [start];
    const exits = [];
    const trail = (c) => {
      const list = [];
      for (let k = index(n, c); k !== null; k = from.get(k)) list.push(unindex(n, k));
      return list.reverse();
    };
    for (let q = 0; q < queue.length; q++) {
      const c = queue[q];
      DIRS.forEach((d, dir) => {
        const next = add(c, d);
        const v = read(all, n, next);
        if (v === 1 && open[dir]) exits.push({ cell: c, dir, trail: trail(c) });
        if (v === 0 && !from.has(index(n, next))) {
          from.set(index(n, next), index(n, c));
          queue.push(next);
        }
      });
    }
    return exits.length ? { start, exits } : null;
  });
  for (let changed = true; changed; ) {
    changed = false;
    table.forEach((one, g) => {
      if (!one) return;
      const exits = one.exits.filter((e) => table[e.dir]);
      if (exits.length === one.exits.length) return;
      changed = true;
      table[g] = exits.length ? { ...one, exits } : null;
    });
  }
  return table;
}

/* GATE */

function whole(all, n, full) {
  const first = all.indexOf(1);
  if (first < 0) return false;
  const seen = new Set([first]);
  const queue = [first];
  for (let q = 0; q < queue.length; q++) {
    const c = unindex(n, queue[q]);
    for (const d of DIRS) {
      const next = add(c, d);
      if (read(all, n, next) !== 1 || seen.has(index(n, next))) continue;
      seen.add(index(n, next));
      queue.push(index(n, next));
    }
  }
  return seen.size === full;
}

export function gate(code, n) {
  const all = cells(code, n);
  let full = 0;
  let empty = 0;
  let walls = 0;
  for (let k = 0; k < all.length; k++) {
    if (all[k]) {
      full++;
      continue;
    }
    empty++;
    const c = unindex(n, k);
    for (const d of DIRS) walls += Math.max(0, read(all, n, add(c, d)));
  }
  const share = full / all.length;
  if (share < LOW || share > HIGH || !empty || walls / empty < WALLS) return false;
  return whole(all, n, full) && doors(all, n).some(Boolean);
}

/* CLASSES */

const SYMMETRIES = [[0, 1, 2], [0, 2, 1], [1, 0, 2], [1, 2, 0], [2, 0, 1], [2, 1, 0]].flatMap((p) => Array.from({ length: 8 }, (_, f) => [p, f]));

const act = (code, [p, f]) => {
  let out = 0;
  for (let b = 0; b < 8; b++) {
    if (!((code >> b) & 1)) continue;
    const q = [0, 0, 0];
    [(b >> 2) & 1, (b >> 1) & 1, b & 1].forEach((bit, i) => {
      q[p[i]] = bit ^ ((f >> i) & 1);
    });
    out |= 1 << (q[0] * 4 + q[1] * 2 + q[2]);
  }
  return out;
};

const CANON = Array.from({ length: 256 }, (_, code) => Math.min(...SYMMETRIES.map((one) => act(code, one))));

export const canon = (code) => CANON[code];

export const classes = () => [...new Set(CANON.slice(1))].sort((a, b) => a - b);

export function name(code) {
  const hit = NAMES.find(([one]) => one === code) ?? NAMES.find(([one]) => CANON[one] === CANON[code]);
  return hit ? hit[1] : `class ${CANON[code]}`;
}

export function teach(code, n) {
  const count = cells(code, n).reduce((a, b) => a + b, 0);
  return { bits: code.toString(2).padStart(8, '0'), kept: count, fill: count / n ** 3, dimension: count ? Math.log(count) / Math.log(n) : 0, class: CANON[code] };
}

const ALL = Array.from({ length: 255 }, (_, i) => i + 1);

const gated = new Map();

const flyable = new Map();

export function codes(n, pick) {
  if (pick === 'all') return ALL;
  if (pick === 'named') return classes();
  if (pick === 'family') return NAMES.map(([code]) => code);
  if (pick === 'flyable') {
    if (!flyable.has(n)) flyable.set(n, ALL.filter((code) => net(code, n).doors.some((list) => list.length)));
    return flyable.get(n);
  }
  if (!gated.has(n)) gated.set(n, Array.from({ length: 256 }, (_, code) => code).filter((code) => gate(code, n)));
  return gated.get(n);
}

export function pickCode(value, seed) {
  if (value.code >= 0) return value.code;
  const pool = codes(value.n, value.pick);
  return pool.length ? pool[Math.floor(rng((seed ^ CHOICE) >>> 0)() * pool.length)] : 23;
}

/* GRAPH */

const nets = new Map();

function face(n, g) {
  const a = axis(g);
  const [b, c] = [0, 1, 2].filter((i) => i !== a);
  const out = [];
  for (let v = 0; v < n; v++) {
    for (let u = 0; u < n; u++) {
      const cell = [0, 0, 0];
      cell[a] = DIRS[g][a] > 0 ? 0 : n - 1;
      cell[b] = u;
      cell[c] = v;
      out.push(cell);
    }
  }
  return out;
}

const lateral = (all, n, c, d) => DIRS.reduce((sum, e, g) => sum + (axis(g) !== axis(d) && read(all, n, add(c, e)) === 1 ? 1 : 0), 0);

function net(code, n) {
  const key = `${code}:${n}`;
  if (nets.has(key)) return nets.get(key);
  const all = cells(code, n);
  const comp = new Int32Array(all.length).fill(-1);
  const leave = [];
  for (let k = 0; k < all.length; k++) {
    if (all[k] || comp[k] >= 0) continue;
    const id = leave.length;
    const dirs = new Set();
    const queue = [k];
    comp[k] = id;
    for (let q = 0; q < queue.length; q++) {
      const c = unindex(n, queue[q]);
      DIRS.forEach((d, dir) => {
        const next = add(c, d);
        const v = read(all, n, next);
        if (v === 1) dirs.add(dir);
        if (v !== 0 || comp[index(n, next)] >= 0) return;
        comp[index(n, next)] = id;
        queue.push(index(n, next));
      });
    }
    leave.push([...dirs]);
  }
  const faces = DIRS.map((_, g) => face(n, g).filter((c) => !all[index(n, c)]));
  const alive = leave.map(() => true);
  const open = (dir) => faces[dir].some((c) => alive[comp[index(n, c)]]);
  for (let changed = true; changed; ) {
    changed = false;
    leave.forEach((dirs, id) => {
      if (!alive[id] || dirs.some(open)) return;
      alive[id] = false;
      changed = true;
    });
  }
  const one = { all, doors: faces.map((list) => list.filter((c) => alive[comp[index(n, c)]])), walks: new Map(), ends: new Map() };
  nets.set(key, one);
  return one;
}

function walk(web, n, g, start) {
  const key = `${g}:${index(n, start)}`;
  if (web.walks.has(key)) return web.walks.get(key);
  const { all } = web;
  const dist = new Int32Array(all.length).fill(-1);
  const score = new Float64Array(all.length * 6).fill(-1);
  const turns = new Int32Array(all.length * 6);
  const from = new Int32Array(all.length * 6).fill(-1);
  const first = index(n, start);
  dist[first] = 0;
  score[first * 6 + g] = lateral(all, n, start, g);
  const order = [first];
  for (let q = 0; q < order.length; q++) {
    const k = order[q];
    const c = unindex(n, k);
    DIRS.forEach((e, dir) => {
      const next = add(c, e);
      if (read(all, n, next) !== 0) return;
      const m = index(n, next);
      if (dist[m] < 0) {
        dist[m] = dist[k] + 1;
        order.push(m);
      }
      if (dist[m] !== dist[k] + 1) return;
      const gain = lateral(all, n, next, dir);
      for (let d = 0; d < 6; d++) {
        const at = k * 6 + d;
        const to = m * 6 + dir;
        if (score[at] < 0) continue;
        const sum = score[at] + gain;
        const bent = turns[at] + (d !== dir ? 1 : 0);
        if (score[to] >= 0 && (bent > turns[to] || (bent === turns[to] && sum <= score[to]))) continue;
        score[to] = sum;
        turns[to] = bent;
        from[to] = at;
      }
    });
  }
  const best = (k, dir) => {
    let top = -1;
    for (let d = 0; d < 6; d++) {
      const at = k * 6 + d;
      if (score[at] < 0) continue;
      const bent = turns[at] + (d !== dir ? 1 : 0);
      const was = top < 0 ? 0 : turns[top] + (top % 6 !== dir ? 1 : 0);
      if (top < 0 || bent < was || (bent === was && score[at] > score[top])) top = at;
    }
    return top;
  };
  const exits = [];
  for (const k of order) {
    const c = unindex(n, k);
    DIRS.forEach((e, dir) => {
      if (read(all, n, add(c, e)) !== 1 || !web.doors[dir].length) return;
      const steps = [];
      for (let at = best(k, dir); at >= 0; at = from[at]) steps.unshift(at);
      const moves = steps.map((at) => at % 6);
      exits.push({ cell: c, dir, trail: steps.map((at) => unindex(n, Math.floor(at / 6))), moves, turns: [...moves, dir].filter((d, i, all) => i > 0 && d !== all[i - 1]).length });
    });
  }
  const ahead = exits.filter((e) => e.dir !== flip(g));
  const one = { exits: ahead.length ? ahead : exits };
  web.walks.set(key, one);
  return one;
}

function landing(web, n, dir, din) {
  const key = `${dir}:${din}`;
  if (!web.ends.has(key)) {
    const off = (c) => sub(add(c, HALF), [n / 2, n / 2, n / 2]).map((v, i) => (i === axis(dir) ? 0 : v));
    const rank = (c) => dot(off(c), off(c)) * n * n - dot(off(c), DIRS[din]);
    web.ends.set(key, web.doors[dir].reduce((top, c) => (rank(c) < rank(top) - 1e-9 ? c : top)));
  }
  return web.ends.get(key);
}

/* ROUTE */

const smooth = (x) => {
  const p = Math.min(Math.max(x, 0), 1);
  return p * p * (3 - 2 * p);
};

const flatten = (u, t) => {
  const v = sub(u, mul(t, dot(u, t)));
  return len(v) > 1e-6 ? norm(v) : norm(cross(t, Math.abs(t[0]) < 0.9 ? [1, 0, 0] : [0, 1, 0]));
};

const basis = (u) => [(1 - u) ** 3 / 6, (3 * u ** 3 - 6 * u * u + 4) / 6, (-3 * u ** 3 + 3 * u * u + 3 * u + 1) / 6, u ** 3 / 6];

const slope = (u) => [-((1 - u) ** 2) / 2, (3 * u * u - 4 * u) / 2, (-3 * u * u + 2 * u + 1) / 2, (u * u) / 2];

const weigh = (w, list) => list.reduce((acc, v, i) => add(acc, mul(v, w[i])), ZERO);

const total = (w, list) => list.reduce((acc, v, i) => acc + v * w[i], 0);

export function route(design, seed = 0, { pilot: style = 'steady' } = {}) {
  const { n } = design;
  const pilot = PILOTS[style] ?? PILOTS.steady;
  const behind = pilot.window + 2 * pilot.gap - pilot.lead;
  const web = net(design.code, n);
  const { all } = web;
  if (!all.some(Boolean)) throw new Error(`bang: code ${design.code} is empty`);
  const dive = !web.doors.some((list) => list.length);
  const mid = [n / 2, n / 2, n / 2];
  let cycles = [];
  let keys = [];
  let base = 0;
  let measured = 1;
  let timed = 1;
  let origin = 0;
  let start = -Infinity;
  let sky = null;
  let lost = 0;
  let first = 0;
  let memo = null;
  let around = new Map();
  let barrels = [];
  let straight = 0;
  let rested = 0;
  let spun = 0;
  const key = (i) => keys[i - base];
  const last = () => base + keys.length - 1;
  const cycle = (j) => cycles.find((one) => one.j === j);
  const pending = () => cycles[cycles.length - 1];
  const place = (q, frame) => {
    let p = q.p;
    for (let k = q.j; k > frame; k--) {
      const up = cycle(k - 1);
      if (!up?.child) return null;
      p = add(up.child, mul(p, 1 / n));
    }
    for (let k = q.j; k < frame; k++) {
      const up = cycle(k);
      if (!up?.child) return null;
      p = mul(sub(p, up.child), n);
    }
    return p;
  };
  const solid = (j, c, depth = 0) => {
    if (c.every((v) => v >= 0 && v < n)) return all[index(n, c)];
    if (j <= 0) return 0;
    if (depth >= DEPTH) return 1;
    const q = c.map((v) => Math.floor(v / n));
    return solid(j - 1, add(cycle(j - 1).child, q), depth + 1) ? all[index(n, sub(c, mul(q, n)))] : 0;
  };
  const orient = (m) => {
    const one = key(m);
    const t = norm(sub(place(key(m + 1), one.j), place(key(Math.max(m - 1, base)), one.j)));
    const prev = m > base ? key(m - 1) : null;
    let up;
    if (sky) up = sky.up;
    else if (!prev?.up) up = Math.abs(t[1]) < 0.9 ? [0, 1, 0] : [0, 0, 1];
    else {
      const pivot = cross(prev.dir, t);
      const s = len(pivot);
      up = s > 1e-9 ? rot(prev.up, pivot, Math.atan2(s, dot(prev.dir, t))) : prev.up;
    }
    up = flatten(up, t);
    if (len(one.sink) > 0) {
      const want = flatten(mul(one.sink, -1), t);
      up = flatten(rot(up, t, Math.min(Math.max(Math.atan2(dot(cross(up, want), t), dot(up, want)), -STEER), STEER)), t);
    }
    one.up = up;
    one.dir = t;
  };
  const push = (j, p, lev, extra = {}) => {
    keys.push({ j, p, lev, sink: keys.length ? keys[keys.length - 1].sink : ZERO, ...extra });
    if (last() - 1 >= base) orient(last() - 1);
  };
  const stay = (j, c, d) => {
    let walls = 0;
    let floor = ZERO;
    DIRS.forEach((e, g) => {
      if (axis(g) === axis(d) || !solid(j, add(c, e))) return;
      walls++;
      floor = add(floor, e);
    });
    const seen = [walls];
    for (let i = last(); i >= base && seen.length < SPAN; i--) if (key(i).cell) seen.push(key(i).walls);
    const mean = seen.reduce((a, b) => a + b, 0) / seen.length;
    const sink = mean >= TRENCH && mean < CORRIDOR && len(floor) > 0 ? norm(floor) : ZERO;
    push(j, add(c, HALF), j, { cell: true, walls, sink });
  };
  const measure = () => {
    for (; measured + 2 <= last(); measured++) {
      const k = measured;
      const F = key(k).j;
      const P = [k - 1, k, k + 1, k + 2].map((i) => place(key(i), F));
      const L = [k - 1, k, k + 1, k + 2].map((i) => key(i).lev);
      if (k === 1) key(1).s = 0;
      const s = new Float64Array(SAMPLES + 1);
      s[0] = key(k).s;
      for (let i = 0; i < SAMPLES; i++) {
        const u = (i + 0.5) / SAMPLES;
        s[i + 1] = s[i] + (len(weigh(slope(u), P)) * n ** (total(basis(u), L) - F)) / SAMPLES;
      }
      key(k).seg = { F, P, L, s, t: null };
      key(k + 1).s = s[SAMPLES];
      if (key(k + 1).door !== undefined) cycle(key(k + 1).door).s = s[SAMPLES];
      if (pilot.barrel) roll(k + 1);
    }
  };
  const bend = (m) => {
    const one = key(m);
    if (one.w === undefined) {
      const a = sub(one.p, place(key(m - 1), one.j));
      const b = sub(place(key(m + 1), one.j), one.p);
      const c = dot(a, b) / (len(a) * len(b));
      one.w = len(a) < 1e-9 || len(b) < 1e-9 ? 0 : Math.min(1, Math.acos(Math.min(Math.max(c, -1), 1)) / (Math.PI / 2));
    }
    return one.w;
  };
  const roll = (m) => {
    const at = key(m).s;
    const bent = bend(m) > SHARP;
    const from = Math.max(straight, rested);
    const reach = at - from;
    if ((bent && reach >= LONG) || reach >= HOLD) {
      const mid = from + Math.min(reach, HOLD) / 2;
      const size = pilot.barrel * SPEED;
      barrels.push({ from: mid - size / 2, to: mid + size / 2, sign: spun++ % 2 ? -1 : 1 });
      rested = mid + size / 2 + REST;
    }
    if (bent) straight = at;
  };
  const spin = (s) => {
    const one = barrels.find((b) => s > b.from && s < b.to);
    return one ? one.sign * TURN * smooth((s - one.from) / (one.to - one.from)) : 0;
  };
  const pace = (s, k) => {
    const { bend: reach, ease } = pilot;
    let bump = 0;
    for (let m = k; m > base && key(m).s > s - reach; m--) bump = Math.max(bump, bend(m) * (1 - smooth(Math.abs(s - key(m).s) / reach)));
    for (let m = k + 1; m < last() && key(m).s < s + reach; m++) bump = Math.max(bump, bend(m) * (1 - smooth(Math.abs(s - key(m).s) / reach)));
    return SPEED * (1 - ease * bump);
  };
  const clock = () => {
    const edge = key(last() - 1).s;
    for (; timed < measured && key(timed + 1).s + pilot.bend <= edge; timed++) {
      const k = timed;
      const seg = key(k).seg;
      if (k === 1) key(1).t = 0;
      const t = new Float64Array(SAMPLES + 1);
      t[0] = key(k).t;
      for (let i = 0; i < SAMPLES; i++) t[i + 1] = t[i] + (seg.s[i + 1] - seg.s[i]) / pace((seg.s[i] + seg.s[i + 1]) / 2, k);
      seg.t = t;
      key(k + 1).t = t[SAMPLES];
      if (key(k + 1).door !== undefined) cycle(key(k + 1).door).at = t[SAMPLES];
    }
  };
  const grow = () => {
    measure();
    clock();
  };
  const horizon = () => key(timed).t;
  const find = (x, end, of) => {
    let lo = Math.max(base + 1, 1);
    let hi = end - 1;
    while (lo < hi) {
      const m = (lo + hi + 1) >> 1;
      if (key(m)[of] <= x) lo = m;
      else hi = m - 1;
    }
    const seg = key(lo).seg;
    const table = seg[of];
    let i = 0;
    while (i < SAMPLES - 1 && table[i + 1] <= x) i++;
    const f = (x - table[i]) / (table[i + 1] - table[i]);
    const u = (i + f) / SAMPLES;
    const lev = total(basis(u), seg.L);
    const sink = mix(key(lo).sink, key(lo + 1).sink, u);
    return { j: seg.F, p: add(weigh(basis(u), seg.P), mul(sink, FLOOR * n ** (seg.F - lev))), lev, k: lo, u, s: seg.s[i] + f * (seg.s[i + 1] - seg.s[i]) };
  };
  const point = (t) => find(t, timed, 't');
  const along = (s) => find(s, measured, 's');
  const recent = (k) => {
    const out = [];
    for (let i = k; i >= base && out.length < SPAN; i--) if (key(i).cell) out.push(i);
    for (let i = k + 1; i <= last() && !out.length; i++) if (key(i).cell) out.push(i);
    return out;
  };
  const back = (j, g, e) => {
    const runs = [];
    const before = cycle(j - 1)?.moves ?? [];
    const old = before.filter((d, i) => d !== before[i - 1]).length;
    [...before.map((d) => [d, n]), [g, 1], ...e.moves.slice(1).map((d) => [d, 1]), [e.dir, 1]].forEach(([d, size]) => {
      if (runs.length && runs[runs.length - 1][0] === d) runs[runs.length - 1][1] += size;
      else runs.push([d, size]);
    });
    return runs.some(([c], k) => {
      if (k < Math.max(old, 2)) return false;
      const [[a], [b, size]] = [runs[k - 2], runs[k - 1]];
      return size < BACK && (c === flip(a) || (axis(c) !== axis(a) && axis(c) !== axis(b)));
    });
  };
  const bounds = (j) => {
    const up = cycle(j - 1);
    if (!up) return [ZERO, ZERO];
    return [up.edge[0].map((v, i) => Math.min(v * n + up.child[i], BEYOND)), up.edge[1].map((v, i) => Math.min(v * n + n - 1 - up.child[i], BEYOND))];
  };
  const open = (j, g, start) => {
    const rand = rng((seed ^ SALT ^ Math.imul(j + 1, GOLD)) >>> 0);
    const door = last();
    key(door).door = j;
    stay(j, start, g);
    const { exits } = walk(web, n, g, start);
    const fair = exits.filter((e) => !back(j, g, e));
    const used = [cycle(j - 1)?.exit?.dir, cycle(j - 2)?.exit?.dir];
    const scored = (fair.length ? fair : exits).map((e) => ({ ...e, score: (1 + STRAIGHT * (e.dir === g) + FRESH * !used.includes(e.dir) + TRAIL * e.trail.length - TWIST * e.turns) * (1 - JITTER / 2 + JITTER * rand()) }));
    const pick = scored.reduce((top, e) => (e.score > top.score ? e : top));
    cycles.push({ j, g, pick, exit: null, child: null, door, edge: bounds(j) });
  };
  const top = (ia, ib) => {
    const c = [0, 0, 0];
    c[axis(sky.along)] = DIRS[sky.along][axis(sky.along)] > 0 ? ia : n - 1 - ia;
    c[axis(sky.face)] = DIRS[sky.face][axis(sky.face)] > 0 ? n - 1 : 0;
    c[sky.side] = ib;
    return c;
  };
  const spot = (x, h, y) => {
    const p = [0, 0, 0];
    p[axis(sky.along)] = DIRS[sky.along][axis(sky.along)] > 0 ? x : n - x;
    p[axis(sky.face)] = DIRS[sky.face][axis(sky.face)] > 0 ? n + h : -h;
    p[sky.side] = y;
    return p;
  };
  const land = (j) => {
    const rand = rng((seed ^ SALT ^ Math.imul(j + 1, GOLD)) >>> 0);
    const door = last();
    key(door).door = j;
    let best = null;
    for (let ia = 0; ia < n; ia++) {
      for (let ib = 0; ib < n; ib++) {
        if (!all[index(n, top(ia, ib))]) continue;
        const score = (1 + STRAIGHT * (ib === (n - 1) / 2) + FRESH * (ia > 0)) * (1 - JITTER / 2 + JITTER * rand());
        if (!best || score > best.score) best = { ia, ib, score };
      }
    }
    cycles.push({ j, ia: best.ia, ib: best.ib, child: top(best.ia, best.ib), door, edge: bounds(j) });
  };
  const lock = (one) => {
    one.until = horizon();
    if (dive) {
      for (let x = -0.5; x < one.ia; x++) push(one.j, spot(x, HOVER, one.ib + 0.5), one.j);
      land(one.j + 1);
      grow();
      return;
    }
    const exit = one.pick;
    one.exit = exit;
    one.moves = exit.moves;
    exit.trail.slice(1).forEach((c, i) => stay(one.j, c, exit.moves[i + 1]));
    one.child = add(exit.cell, DIRS[exit.dir]);
    const next = landing(web, n, exit.dir, exit.moves[exit.moves.length - 1]);
    push(one.j + 1, add(sub(next, DIRS[exit.dir]), HALF), one.j + 1);
    push(one.j + 1, add(add(next, HALF), mul(DIRS[exit.dir], -0.5)), one.j + 1);
    open(one.j + 1, exit.dir, next);
    grow();
  };
  const camera = (t, frame) => {
    let sum = ZERO;
    for (let i = 0; i < TAPS; i++) sum = add(sum, place(point(t - pilot.window + (pilot.window * i) / (TAPS - 1)), frame));
    return mul(sum, 1 / TAPS);
  };
  const build = () => {
    cycles = [];
    keys = [];
    base = 0;
    lost = 0;
    measured = 1;
    timed = 1;
    around = new Map();
    barrels = [];
    straight = 0;
    rested = 0;
    spun = 0;
    const rand = rng((seed ^ SALT) >>> 0);
    let g;
    if (dive) {
      const pairs = [];
      for (let up = 0; up < 6; up++) for (let along = 0; along < 6; along++) if (axis(up) !== axis(along)) pairs.push({ face: up, along, side: 3 - axis(up) - axis(along), up: DIRS[up] });
      const layer = (one) => face(n, flip(one.face)).map((c) => all[index(n, c)]);
      const holed = pairs.filter((one) => layer(one).includes(0) && layer(one).includes(1));
      const pool = holed.length ? holed : pairs.filter((one) => layer(one).includes(1));
      sky = pool[Math.floor(rand() * pool.length)];
      g = sky.along;
      for (let i = 0; i <= LEAD; i++) push(0, spot(i - LEAD - 1.5, HOVER, n / 2), 0);
      land(0);
    } else {
      const ways = web.doors.flatMap((list, way) => list.map((cell) => ({ way, cell })));
      const centre = ways.filter((one) => one.cell.every((v, i) => i === axis(one.way) || v === (n - 1) / 2));
      const pool = centre.length ? centre : ways;
      const pick = pool[Math.floor(rand() * pool.length)];
      g = pick.way;
      const mouth = add(add(pick.cell, HALF), mul(DIRS[g], -0.5));
      for (let i = 0; i <= LEAD; i++) push(0, add(mouth, mul(DIRS[g], i - LEAD)), 0);
      open(0, g, pick.cell);
    }
    while (cycle(PRELUDE)?.at === undefined || horizon() < cycle(0).at + 1) lock(pending());
    if (start === -Infinity) {
      const plane = key(cycle(0).door).p;
      const side = (t) => dot(sub(camera(t, 0), plane), DIRS[g]);
      let lo = 2 * pilot.window;
      let hi = cycle(0).at + 1;
      for (let i = 0; i < HALVES; i++) {
        const m = (lo + hi) / 2;
        if (side(m) < 0) lo = m;
        else hi = m;
      }
      start = hi;
    }
    origin = cycle(PRELUDE).at;
    while (horizon() - origin < RUNWAY) lock(pending());
    first = pending().j;
    for (const one of cycles) if (one !== pending()) one.until = -Infinity;
  };
  const near = (frame) => {
    if (around.has(frame)) return around.get(frame);
    const words = [0, 0, 0, 0];
    const half = (REGION - 1) / 2;
    for (let i = 0; i < REGION ** 3; i++) {
      let carry = [Math.floor(i / REGION ** 2) - half, (Math.floor(i / REGION) % REGION) - half, (i % REGION) - half];
      let ok = true;
      for (let k = frame - 1; ok && carry.some(Boolean); k--) {
        if (k < 0 || (sky && Math.sign(carry[axis(sky.face)]) === DIRS[sky.face][axis(sky.face)])) {
          ok = false;
          break;
        }
        if (k < frame - DEPTH) break;
        const cell = add(cycle(k).child, carry);
        carry = cell.map((v) => Math.floor(v / n));
        ok = read(all, n, cell.map((v) => ((v % n) + n) % n)) === 1;
      }
      if (ok) words[i >> 5] |= 1 << (i & 31);
    }
    for (const j of around.keys()) if (j < frame - KEEP) around.delete(j);
    around.set(frame, words);
    return words;
  };
  const waiting = (t) => cycles.find((one) => !(one.until <= t)) ?? pending();
  const hatch = (one) => add(add(one.exit.cell, HALF), mul(DIRS[one.exit.dir], 0.5));
  const view = (t, tau) => {
    while (horizon() <= t + REACH) lock(pending());
    const here = point(t);
    const seen = dive ? [] : recent(here.k);
    if (base > 0 && (key(base + 1).t > t - behind || (lost > 0 && seen.length < SPAN))) {
      build();
      return view(t, tau);
    }
    const cur = cycles.findLast((one) => one.at <= t);
    const wait = waiting(t);
    const frame = Math.max(Math.min(cur.j, wait.j - 2) - AHEAD, 0);
    const local = (q) => sub(place(q, frame), mid);
    const eye = [0, 1, 2].map((i) => camera(t + pilot.lead - pilot.gap * i, frame));
    const pos = sub(pilot.lead ? camera(t, frame) : eye[0], mid);
    const scale = n ** (frame - here.lev);
    let fwd = sub(local(along(here.s + pilot.look * SPEED)), pos);
    if (len(fwd) < scale * 1e-6) fwd = sub(local(along(here.s + STEP)), local(along(here.s - STEP)));
    fwd = norm(fwd);
    const up = flatten(weigh(basis(here.u), [-1, 0, 1, 2].map((i) => key(here.k + i).up)), fwd);
    const speed = (i) => mul(sub(eye[i], eye[i + 1]), 1 / (pilot.gap * n ** (frame - point(t + pilot.lead - pilot.gap * (i + 0.5)).lev)));
    const accel = mul(sub(speed(0), speed(1)), 1 / pilot.gap);
    const bank = Math.min(Math.max(pilot.bank * dot(accel, norm(cross(fwd, up))), -pilot.tilt), pilot.tilt);
    const walls = seen.reduce((a, i) => a + key(i).walls, 0) / seen.length;
    const mode = dive ? 'skim' : walls >= CORRIDOR ? 'corridor' : walls >= TRENCH ? 'trench' : 'pillars';
    const doors = [];
    if (!dive) {
      const next = cycle(cur.j + 1);
      const into = (here.s - cur.s) / (next.s - cur.s);
      doors.push({ pos: local({ j: cur.j, p: hatch(cur) }), scale: n ** (frame - cur.j), glow: smooth(into / RISE) * (0.6 + 0.4 * into) * (1 - DIP * smooth((into - CLOSE) / (1 - CLOSE))) });
      const before = cycle(cur.j - 1);
      if (before?.exit) doors.push({ pos: local({ j: before.j, p: hatch(before) }), scale: n ** (frame - cur.j + 1), glow: (1 - DIP) * (1 - smooth((t - cur.at) / FADE)) });
    }
    const keep = Math.min(point(t - behind).k, ...seen) - 1;
    while (base < keep) {
      if (keys.shift().cell) lost++;
      base++;
    }
    while (barrels.length && barrels[0].to < key(base + 1).s) barrels.shift();
    while (cycles[0].j < cur.j - KEEP - 1) cycles.shift();
    return {
      pos,
      fwd,
      up,
      roll: bank + pilot.roll * Math.sin(SWAY * tau) + spin(here.s),
      frame,
      level: wait.j - first,
      scale,
      fog: FOG,
      near: near(frame),
      edge: cycle(frame).edge,
      hue: HUE + HUE * Math.sin(DRIFT * here.lev),
      doors,
      mode,
      outside: dive,
    };
  };
  const at = (tau) => {
    if (memo?.tau === tau) return memo.state;
    const t = tau + origin;
    memo = { tau, state: t >= start ? view(t, tau) : { ...view(start, start - origin), pos: [NaN, NaN, NaN] } };
    return memo.state;
  };
  const path = (tau, span, drop = 0) => {
    const state = at(tau);
    if (!state.pos.every(Number.isFinite)) return [];
    const t = tau + origin;
    while (horizon() <= t + span) lock(pending());
    return Array.from({ length: RAIL }, (_, i) => {
      const x = t + (span * i) / (RAIL - 1);
      const p = sub(camera(x, state.frame), mid);
      if (!drop) return p;
      const here = point(x);
      const up = norm(weigh(basis(here.u), [-1, 0, 1, 2].map((m) => key(here.k + m).up)));
      return sub(p, mul(up, drop * n ** (state.frame - here.lev)));
    });
  };
  const toFrame = (q, frame) => sub(place({ j: q.j, p: add(q.p, mid) }, frame) ?? [NaN, NaN, NaN], mid);
  build();
  return { at, path, toFrame };
}
