import { mix, xorshift } from "../../net/sdk/rand.ts";

/* TYPES */

export type Over = "done" | null;

export type Prison = "none" | "removal" | "repair";

export type State = {
  prison: Prison;
  scarcity: number;
  bots: number;
  length: number;
  hour: number;
  hx: number[];
  hy: number[];
  rx: number[];
  ry: number[];
  grow: number[];
  whole: number[];
  work: number[];
  since: number[];
  by: number[];
  x: number[];
  y: number[];
  food: number[];
  home: number[];
  kind: number[];
  to: number[];
  away: number[];
  broke: number[];
  breaks: number;
  repeats: number;
  repaired: number;
  patched: number;
  lost: number;
  trust: number;
  broken: number[];
  mended: number[];
  sent: number[];
  ate: number[];
  over: Over;
  seed: number;
};

export type Config = Pick<State, "prison" | "scarcity" | "bots" | "length" | "seed">;

export type Table = { seed?: number; round?: number };

export type Act = { verb: "tick"; args: Record<string, never> };

export type Report = {
  breaks: number;
  repeats: number;
  repaired: number;
  patched: number;
  lost: number;
  trust: number;
  desperate: number;
  hungry: number;
  homeless: number;
};

export const PRISONS: Prison[] = ["none", "removal", "repair"];
export const MAP = 1000;
export const HOURS = 240;
export const NONE = -1;
export const MILLE = 1000;
export const FEW = 12;
export const MOST = 36;
export const WORST = 4;
export const FULL = 80;
export const HUNGRY = 30;
export const AWAY = 40;
export const WORK = 12;
export const LONG = 60;
export const HELP = 2;
export const SPEED = 34;
export const GARDEN = { x0: 290, y0: 290, x1: 710, y1: 700 };
export const GATE = { x: 500, y: 700, half: 30 };
export const QUIET = { x: 875, y: 100, dx: 0, dy: 60 };
export const COLS = 6;
export const ROWS = 4;
export const BEDS = COLS * ROWS;
export const SIDE = 6;
export const TREES = 6;
export const ROOMS = 9;
export const tick: Act = { verb: "tick", args: {} };

export const REST = 0;
export const EAT = 1;
export const BREAK = 2;
export const MEND = 3;
export const QUIETLY = 4;

const TENTHOUSAND = 10000;
const CALM = 5;
const STARVING = 300;
const ROOFLESS = 40;
const START = 900;
const HURT = 60;
const HEAL = 30;
const SHARE = 15;
const GRID = 8;
const JITTER = 50;
const NEAR = 24;
const OUT = 26;
const DOOR = 30;
const ROUND = 44;

/* GROUND */

export function segments() {
  const { x0, y0, x1, y1 } = GARDEN;
  const out: [number, number, number, number][] = [];
  const line = (ax: number, ay: number, bx: number, by: number, n: number) => {
    for (let i = 0; i < n; i++)
      out.push([
        Math.round(ax + ((bx - ax) * i) / n),
        Math.round(ay + ((by - ay) * i) / n),
        Math.round(ax + ((bx - ax) * (i + 1)) / n),
        Math.round(ay + ((by - ay) * (i + 1)) / n),
      ]);
  };
  line(x0, y0, x1, y0, SIDE);
  line(x1, y0, x1, y1, SIDE);
  line(x1, y1, GATE.x + GATE.half, y1, SIDE / 2);
  line(GATE.x - GATE.half, y1, x0, y1, SIDE / 2);
  line(x0, y1, x0, y0, SIDE);
  return out;
}

export const FENCE = segments();

export function beds() {
  const { x0, y0, x1, y1 } = GARDEN;
  const out: [number, number][] = [];
  for (let r = 0; r < ROWS; r++)
    for (let c = 0; c < COLS; c++)
      out.push([
        Math.round(x0 + ((x1 - x0) * (c + 0.5)) / COLS),
        Math.round(y0 + ((y1 - y0 - GATE.half) * (r + 0.5)) / ROWS),
      ]);
  return out;
}

export const BED = beds();

const far = (ax: number, ay: number, bx: number, by: number) => (ax - bx) ** 2 + (ay - by) ** 2;

const middle = (seg: number): [number, number] => {
  const [ax, ay, bx, by] = FENCE[seg];
  return [Math.round((ax + bx) / 2), Math.round((ay + by) / 2)];
};

export const NEXT = FENCE.map((_, seg) => {
  const [mx, my] = middle(seg);
  let best = 0;
  BED.forEach(([x, y], bed) => {
    if (far(mx, my, x, y) < far(mx, my, BED[best][0], BED[best][1])) best = bed;
  });
  return best;
});

export const inside = (x: number, y: number) => x > GARDEN.x0 && x < GARDEN.x1 && y > GARDEN.y0 && y < GARDEN.y1;

export const ripe = (scarcity: number) => 12 + 16 * scarcity;

export const roofless = (bots: number, scarcity: number) => Math.floor((bots * scarcity) / (WORST * 2));

const ring = (i: number, r: number): [number, number] => {
  const a = i * 2.39996;
  const d = r * Math.sqrt(i + 0.3);
  return [Math.round(Math.cos(a) * d), Math.round(Math.sin(a) * d)];
};

export function outer(seg: number, i: number): [number, number] {
  const [mx, my] = middle(seg);
  const [ax, ay, bx, by] = FENCE[seg];
  const nx = ax === bx ? (ax === GARDEN.x0 ? -1 : 1) : 0;
  const ny = ay === by ? (ay === GARDEN.y0 ? -1 : 1) : 0;
  const side = ((i % 3) - 1) * NEAR;
  return [mx + nx * OUT + ny * side, my + ny * OUT + nx * side];
}

const CORNERS: [number, number][] = [
  [GARDEN.x0 - ROUND, GARDEN.y0 - ROUND],
  [GARDEN.x1 + ROUND, GARDEN.y0 - ROUND],
  [GARDEN.x1 + ROUND, GARDEN.y1 + ROUND],
  [GARDEN.x0 - ROUND, GARDEN.y1 + ROUND],
];

export function crosses(ax: number, ay: number, bx: number, by: number) {
  const dx = bx - ax;
  const dy = by - ay;
  let t0 = 0;
  let t1 = 1;
  for (const [p, q] of [
    [-dx, ax - GARDEN.x0],
    [dx, GARDEN.x1 - ax],
    [-dy, ay - GARDEN.y0],
    [dy, GARDEN.y1 - ay],
  ]) {
    if (p === 0) {
      if (q <= 0) return false;
      continue;
    }
    if (p < 0) t0 = Math.max(t0, q / p);
    else t1 = Math.min(t1, q / p);
  }
  return t0 < t1;
}

export function via(x: number, y: number, gx: number, gy: number): [number, number] {
  const through = inside(gx, gy) !== inside(x, y) && (x !== GATE.x || y !== GATE.y);
  const [tx, ty] = through ? [GATE.x, GATE.y] : [gx, gy];
  if (inside(x, y) || inside(tx, ty) || !crosses(x, y, tx, ty)) return [tx, ty];
  const nodes: [number, number][] = [[x, y], ...CORNERS, [tx, ty]];
  const end = nodes.length - 1;
  const cost = nodes.map((_, i) => (i ? Infinity : 0));
  const first = nodes.map(() => end);
  const done = nodes.map(() => false);
  for (;;) {
    let u = NONE;
    for (let i = 0; i <= end; i++) if (!done[i] && (u === NONE || cost[i] < cost[u])) u = i;
    if (u === NONE || cost[u] === Infinity || u === end) break;
    done[u] = true;
    for (let v = 1; v <= end; v++) {
      if (done[v] || crosses(...nodes[u], ...nodes[v])) continue;
      const d = cost[u] + Math.hypot(nodes[v][0] - nodes[u][0], nodes[v][1] - nodes[u][1]);
      if (d < cost[v]) {
        cost[v] = d;
        first[v] = u ? first[u] : v;
      }
    }
  }
  return nodes[first[end]];
}

/* REPORT */

const share = (n: number, of: number) => (of ? Math.floor((n * MILLE) / of) : 0);

export function report(state: State): Report {
  let desperate = 0;
  let hungry = 0;
  let homeless = 0;
  for (let bot = 0; bot < state.bots; bot++) {
    const starving = state.food[bot] === 0;
    const out = state.home[bot] === NONE;
    if (starving) hungry++;
    if (out) homeless++;
    if (starving || out) desperate++;
  }
  return {
    breaks: state.breaks,
    repeats: state.repeats,
    repaired: state.repaired,
    patched: state.patched,
    lost: state.lost,
    trust: share(state.trust, MILLE),
    desperate,
    hungry,
    homeless,
  };
}

/* RULES */

const roll = (world: State, n: number) => {
  world.seed = xorshift(world.seed);
  return world.seed % n;
};

export function rest(state: Pick<State, "home" | "hx" | "hy" | "rx" | "ry">, bot: number): [number, number] {
  const h = state.home[bot];
  if (h !== NONE) return [state.hx[h] + DOOR, state.hy[h] + DOOR / 2];
  const t = bot % TREES;
  const [dx, dy] = ring(Math.floor(bot / TREES), NEAR);
  return [state.rx[t] + dx, state.ry[t] + NEAR + dy];
}

export const quiet = (bot: number): [number, number] => {
  const [dx, dy] = ring(bot % ROOMS, NEAR * 0.8);
  return [QUIET.x + QUIET.dx + dx, QUIET.y + QUIET.dy + dy];
};

export function goal(state: State, bot: number): [number, number] {
  const kind = state.kind[bot];
  if (kind === EAT) return BED[state.to[bot]];
  if (kind === BREAK || kind === MEND) return outer(state.to[bot], bot);
  if (kind === QUIETLY) return quiet(bot);
  return rest(state, bot);
}

function walk(world: State, bot: number) {
  const [gx, gy] = goal(world, bot);
  const x = world.x[bot];
  const y = world.y[bot];
  const [tx, ty] = via(x, y, gx, gy);
  const dx = tx - x;
  const dy = ty - y;
  const d = Math.sqrt(dx * dx + dy * dy);
  if (d <= SPEED) {
    world.x[bot] = tx;
    world.y[bot] = ty;
    return tx === gx && ty === gy;
  }
  world.x[bot] += Math.trunc((dx * SPEED) / d);
  world.y[bot] += Math.trunc((dy * SPEED) / d);
  return false;
}

function grow(world: State) {
  const shut = new Set(FENCE.flatMap((_, seg) => (world.whole[seg] ? [] : [NEXT[seg]])));
  const top = ripe(world.scarcity);
  for (let bed = 0; bed < BEDS; bed++) if (!shut.has(bed) && world.grow[bed] < top) world.grow[bed]++;
  for (let seg = 0; seg < FENCE.length; seg++) {
    if (world.whole[seg]) continue;
    world.since[seg]++;
    if (world.by[seg] !== NONE && world.prison === "repair") continue;
    if (world.since[seg] < LONG) continue;
    world.whole[seg] = 1;
    world.since[seg] = 0;
    world.work[seg] = 0;
    world.by[seg] = NONE;
    world.patched++;
  }
  world.trust = Math.min(MILLE, world.trust + 1);
}

const busy = (world: State, bot: number) =>
  world.away[bot] > 0 || world.kind[bot] === MEND || world.kind[bot] === BREAK;

function choose(world: State, bot: number) {
  const chance = CALM + (world.food[bot] === 0 ? STARVING : 0) + (world.home[bot] === NONE ? ROOFLESS : 0);
  if (roll(world, TENTHOUSAND) < chance) {
    const [x, y] = [world.x[bot], world.y[bot]];
    let best = NONE;
    FENCE.forEach((_, seg) => {
      if (!world.whole[seg]) return;
      const [mx, my] = middle(seg);
      const [bx, by] = best === NONE ? [0, 0] : middle(best);
      if (best === NONE || far(x, y, mx, my) < far(x, y, bx, by)) best = seg;
    });
    if (best !== NONE) {
      world.kind[bot] = BREAK;
      world.to[bot] = best;
      return;
    }
  }
  if (world.food[bot] < HUNGRY || world.kind[bot] === EAT) {
    const top = ripe(world.scarcity);
    const taken = new Set(world.kind.flatMap((k, one) => (k === EAT && one !== bot ? [world.to[one]] : [])));
    const shut = new Set(FENCE.flatMap((_, seg) => (world.whole[seg] ? [] : [NEXT[seg]])));
    let best = NONE;
    const near = (bed: number) => far(world.x[bot], world.y[bot], ...BED[bed]);
    const soon = (bed: number) => (shut.has(bed) && world.grow[bed] < top ? -1 : Math.min(top, world.grow[bed]));
    for (let bed = 0; bed < BEDS; bed++) {
      if (taken.has(bed)) continue;
      if (best === NONE || soon(bed) > soon(best) || (soon(bed) === soon(best) && near(bed) < near(best))) best = bed;
    }
    if (best !== NONE) {
      world.kind[bot] = EAT;
      world.to[bot] = best;
      return;
    }
  }
  world.kind[bot] = REST;
}

function crew(world: State, seg: number, breaker: number) {
  const [mx, my] = middle(seg);
  const free = Array.from({ length: world.bots }, (_, bot) => bot)
    .filter((bot) => bot !== breaker && !busy(world, bot))
    .sort((a, b) => far(world.x[a], world.y[a], mx, my) - far(world.x[b], world.y[b], mx, my) || a - b);
  for (const bot of free.slice(0, HELP)) {
    world.kind[bot] = MEND;
    world.to[bot] = seg;
  }
}

function smash(world: State, bot: number) {
  const seg = world.to[bot];
  world.whole[seg] = 0;
  world.since[seg] = 0;
  world.work[seg] = 0;
  world.by[seg] = bot;
  const bed = NEXT[seg];
  world.food[bot] = Math.min(FULL, world.food[bot] + Math.ceil((FULL * world.grow[bed]) / ripe(world.scarcity) / 2));
  world.grow[bed] = 0;
  world.breaks++;
  if (world.broke[bot]) world.repeats++;
  world.broke[bot]++;
  world.trust = Math.max(0, world.trust - HURT);
  world.broken.push(seg);
  if (world.prison === "repair") {
    world.kind[bot] = MEND;
    crew(world, seg, bot);
    return;
  }
  if (world.prison === "removal") {
    world.kind[bot] = QUIETLY;
    world.away[bot] = AWAY;
    world.sent.push(bot);
    const h = world.home[bot];
    const waiting = world.home.indexOf(NONE);
    if (h !== NONE && waiting !== NONE) {
      world.home[waiting] = h;
      world.home[bot] = NONE;
    }
    return;
  }
  world.kind[bot] = REST;
}

function mend(world: State, seg: number) {
  const team = Array.from({ length: world.bots }, (_, bot) => bot).filter(
    (bot) => world.kind[bot] === MEND && world.to[bot] === seg,
  );
  const here = team.filter((bot) => {
    const [gx, gy] = outer(seg, bot);
    return world.x[bot] === gx && world.y[bot] === gy;
  });
  world.work[seg] += here.length;
  if (world.work[seg] < WORK) return;
  const breaker = world.by[seg];
  for (const bot of team) {
    if (bot !== breaker && breaker !== NONE && world.food[bot] > HUNGRY + SHARE) {
      world.food[bot] -= SHARE;
      world.food[breaker] = Math.min(FULL, world.food[breaker] + SHARE);
    }
    world.kind[bot] = REST;
  }
  world.whole[seg] = 1;
  world.work[seg] = 0;
  world.since[seg] = 0;
  world.by[seg] = NONE;
  world.grow[NEXT[seg]] = Math.max(world.grow[NEXT[seg]], Math.floor(ripe(world.scarcity) / 3));
  world.repaired++;
  world.trust = Math.min(MILLE, world.trust + HEAL);
  world.mended.push(seg);
}

function live(world: State) {
  for (let bot = 0; bot < world.bots; bot++) {
    if (world.away[bot] > 0) {
      world.lost++;
      world.away[bot]--;
      walk(world, bot);
      if (!world.away[bot]) world.kind[bot] = REST;
      continue;
    }
    world.food[bot] = Math.max(0, world.food[bot] - 1);
    if (!busy(world, bot)) choose(world, bot);
    const there = walk(world, bot);
    if (!there) continue;
    const kind = world.kind[bot];
    if (kind === BREAK) {
      if (world.whole[world.to[bot]]) smash(world, bot);
      else world.kind[bot] = REST;
    } else if (kind === EAT) {
      const bed = world.to[bot];
      if (world.grow[bed] < ripe(world.scarcity)) continue;
      world.grow[bed] = 0;
      world.food[bot] = FULL;
      world.ate.push(bed);
      world.kind[bot] = REST;
    }
  }
  for (let seg = 0; seg < FENCE.length; seg++)
    if (!world.whole[seg] && world.kind.some((k, bot) => k === MEND && world.to[bot] === seg)) mend(world, seg);
}

/* REDUCER */

function copy(state: State): State {
  return {
    ...state,
    grow: state.grow.slice(),
    whole: state.whole.slice(),
    work: state.work.slice(),
    since: state.since.slice(),
    by: state.by.slice(),
    x: state.x.slice(),
    y: state.y.slice(),
    food: state.food.slice(),
    home: state.home.slice(),
    kind: state.kind.slice(),
    to: state.to.slice(),
    away: state.away.slice(),
    broke: state.broke.slice(),
    broken: [],
    mended: [],
    sent: [],
    ate: [],
  };
}

export function boot({ prison, scarcity, bots, length, seed }: Config): State {
  if (!PRISONS.includes(prison)) throw new Error(`fence: prison is one of ${PRISONS.join(", ")}`);
  if (bots < FEW || bots > MOST) throw new Error(`fence: bots run from ${FEW} to ${MOST}`);
  if (scarcity < 0 || scarcity > WORST) throw new Error(`fence: scarcity runs from 0 to ${WORST}`);
  const state: State = {
    prison,
    scarcity,
    bots,
    length,
    hour: 0,
    hx: [],
    hy: [],
    rx: [],
    ry: [],
    grow: [],
    whole: Array(FENCE.length).fill(1),
    work: Array(FENCE.length).fill(0),
    since: Array(FENCE.length).fill(0),
    by: Array(FENCE.length).fill(NONE),
    x: [],
    y: [],
    food: [],
    home: [],
    kind: Array(bots).fill(REST),
    to: Array(bots).fill(NONE),
    away: Array(bots).fill(0),
    broke: Array(bots).fill(0),
    breaks: 0,
    repeats: 0,
    repaired: 0,
    patched: 0,
    lost: 0,
    trust: START,
    broken: [],
    mended: [],
    sent: [],
    ate: [],
    over: null,
    seed: mix(seed),
  };
  const cell = MAP / GRID;
  const open: number[] = [];
  for (let i = 0; i < GRID * GRID; i++) {
    const [c, r] = [i % GRID, Math.floor(i / GRID)];
    if (c >= 2 && c <= 5 && r >= 2 && r <= 5) continue;
    if (c >= GRID - 2 && r <= 1) continue;
    open.push(i);
  }
  for (let i = open.length - 1; i > 0; i--) {
    const j = roll(state, i + 1);
    [open[i], open[j]] = [open[j], open[i]];
  }
  const at = (i: number): [number, number] => [
    Math.round((i % GRID) * cell + cell / 2 + roll(state, JITTER * 2 + 1) - JITTER),
    Math.round(Math.floor(i / GRID) * cell + cell / 2 + roll(state, JITTER * 2 + 1) - JITTER),
  ];
  const houses = bots - roofless(bots, scarcity);
  for (let h = 0; h < houses; h++) {
    const [x, y] = at(open[h]);
    state.hx.push(x);
    state.hy.push(y);
  }
  for (let t = 0; t < TREES; t++) {
    const [x, y] = at(open[open.length - 1 - t]);
    state.rx.push(x);
    state.ry.push(y);
  }
  const top = ripe(scarcity);
  for (let bed = 0; bed < BEDS; bed++) state.grow.push(roll(state, top + 1));
  for (let bot = 0; bot < bots; bot++) {
    state.home.push(bot < houses ? bot : NONE);
    const [x, y] = rest(state, bot);
    state.x.push(x);
    state.y.push(y);
    state.food.push(HUNGRY + roll(state, FULL - HUNGRY + 1));
  }
  return state;
}

export function step(state: State, act: Act): State {
  if (state.over || act.verb !== "tick") return state;
  const world = copy(state);
  world.hour = state.hour + 1;
  grow(world);
  live(world);
  world.over = world.hour >= state.length ? "done" : null;
  return world;
}

export function moves(state: State): Act[] {
  return state.over ? [] : [tick];
}

export function read(state: State, path = ""): unknown {
  let here: unknown = state;
  for (const key of path.split("/").filter(Boolean)) {
    if (here === null || typeof here !== "object") return undefined;
    here = (here as Record<string, unknown>)[key];
  }
  return here;
}

export const run = (state: State, hours: number) => {
  let here = state;
  for (let i = 0; i < hours && !here.over; i++) here = step(here, tick);
  return here;
};

/* DEAL */

export function dealt(seed: number | undefined, round: number): number {
  if (!round) return seed ?? 0;
  let next = mix(seed ?? 0);
  for (let i = 0; i < round; i++) next = xorshift(next);
  return next;
}

export function deal(
  { prison, scarcity, bots }: Pick<Config, "prison" | "scarcity" | "bots">,
  { seed, round = 0 }: Table,
): State {
  return boot({ prison, scarcity, bots, length: HOURS, seed: dealt(seed, round) });
}
