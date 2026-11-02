import { mix, xorshift } from "../../net/sdk/rand.ts";

/* TYPES */

export type Own = "private" | "shared" | "none";

export type Borders = "open" | "closed";

export type Over = "done" | null;

export type State = {
  bots: number;
  own: Own;
  city: number;
  borders: Borders;
  length: number;
  tick: number;
  ground: number[];
  region: number[];
  hearts: number[];
  use: number[];
  level: number[];
  owner: number[];
  of: number[];
  towns: number[];
  dock: number;
  shore: number;
  home: number[];
  member: number[];
  fresh: number[];
  landed: number[];
  over: Over;
  seed: number;
};

export type Config = Pick<State, "bots" | "own" | "city" | "borders" | "length" | "seed">;

export type Table = { seed?: number; round?: number };

export type Act = { verb: "tick"; args: Record<string, never> };

export type Report = {
  ashore: number;
  housed: number;
  fed: number;
  green: number;
  reach: number;
  tents: number;
  towns: number;
  fenced: number;
};

export const OWNS: Own[] = ["private", "shared", "none"];
export const BORDERS: Borders[] = ["open", "closed"];
export const W = 36;
export const H = 24;
export const SEA = 0;
export const LAND = 1;
export const WOOD = 2;
export const EMPTY = 0;
export const HOME = 1;
export const FARM = 2;
export const SHOP = 3;
export const SCHOOL = 4;
export const PARK = 5;
export const TENT = 6;
export const REGIONS = 3;
export const TOP = 4;
export const DAYS = 100;
export const ARRIVAL = 60;
export const REACH = 4;
export const MILLE = 1000;
export const NONE = -1;
export const tick: Act = { verb: "tick", args: {} };

const HUNDRED = 100;
const TOWER = 12;
const BUSY = 12;
const FEEDS = 2;
const SHOPS = 6;
const SCHOOLS = 10;
const PARKS = 6;
const CLAIM = 6;
const PLOT = 8;
const RADIUS = 6;
const ROOM = 4;
const ROOMY = 6;
const BLOBS = 6;
const FORESTS = 4;
const WILD = 70;
const NEEDS = [FARM, SCHOOL, SHOP, PARK];

/* TILES */

export const at = (x: number, y: number) => y * W + x;

export const xy = (i: number): [number, number] => [i % W, Math.floor(i / W)];

export function far(a: number, b: number): number {
  const [ax, ay] = xy(a);
  const [bx, by] = xy(b);
  return (ax - bx) ** 2 + (ay - by) ** 2;
}

const all = Array.from({ length: W * H }, (_, i) => i);

function around(i: number): number[] {
  const [x, y] = xy(i);
  const out: number[] = [];
  for (const [dx, dy] of [
    [1, 0],
    [-1, 0],
    [0, 1],
    [0, -1],
  ]) {
    const nx = x + dx;
    const ny = y + dy;
    if (nx >= 0 && ny >= 0 && nx < W && ny < H) out.push(at(nx, ny));
  }
  return out;
}

function nearest(from: number, ok: (i: number) => boolean, least = 0): number {
  let best = NONE;
  let score = Infinity;
  for (const i of all) {
    if (!ok(i)) continue;
    const d = far(from, i);
    if (d < least || d >= score) continue;
    best = i;
    score = d;
  }
  return best;
}

/* REPORT */

export const passport = (state: Pick<State, "borders">, bot: number) =>
  state.borders === "closed" ? bot % REGIONS : NONE;

export const arrives = (state: Pick<State, "bots">, bot: number) => 1 + Math.floor((bot * ARRIVAL) / state.bots);

export function members(state: State, town: number): number[] {
  const out: number[] = [];
  for (let bot = 0; bot < state.bots; bot++) if (state.member[bot] === town) out.push(bot);
  return out;
}

const count = (state: State, town: number, use: number) =>
  state.use.filter((one, i) => one === use && state.of[i] === town).length;

export function fed(state: State): number {
  let out = 0;
  for (let town = 0; town < state.towns.length; town++)
    out += Math.min(members(state, town).length, count(state, town, FARM) * FEEDS);
  return out;
}

export function reached(state: State, tile: number): boolean {
  const near = (use: number) => state.use.some((one, i) => one === use && far(i, tile) <= REACH * REACH);
  return near(SCHOOL) && near(PARK);
}

export const housed = (state: State, bot: number) => state.home[bot] !== NONE && state.use[state.home[bot]] === HOME;

export function report(state: State): Report {
  const comes = Array.from({ length: state.bots }, (_, bot) => bot).filter((bot) => arrives(state, bot) <= state.tick);
  const homes = comes.filter((bot) => housed(state, bot));
  const land = state.ground.filter((one) => one !== SEA).length;
  const green = state.ground.filter(
    (one, i) => one !== SEA && ((state.use[i] === EMPTY && state.owner[i] === NONE) || state.use[i] === PARK),
  ).length;
  const share = (n: number, of: number) => (of ? Math.floor((n * MILLE) / of) : 0);
  return {
    ashore: comes.length,
    housed: share(homes.length, comes.length),
    fed: share(fed(state), comes.length),
    green: share(green, land),
    reach: share(homes.filter((bot) => reached(state, state.home[bot])).length, comes.length),
    tents: comes.filter((bot) => state.home[bot] !== NONE && state.use[state.home[bot]] === TENT).length,
    towns: state.towns.length,
    fenced: state.owner.filter((one, i) => one !== NONE && state.use[i] === EMPTY).length,
  };
}

/* RULES */

const roll = (work: State) => {
  work.seed = xorshift(work.seed);
  return work.seed % HUNDRED;
};

const draw = (work: State, n: number) => {
  work.seed = xorshift(work.seed);
  return work.seed % n;
};

const allowed = (work: State, bot: number, i: number) =>
  work.ground[i] !== SEA && (passport(work, bot) === NONE || work.region[i] === passport(work, bot));

const free = (work: State, bot: number, i: number) =>
  allowed(work, bot, i) && work.use[i] === EMPTY && work.owner[i] === NONE;

function clear(work: State, from: number, ok: (i: number) => boolean, least = 0): number {
  const bare = nearest(from, (i) => ok(i) && work.ground[i] !== WOOD, least);
  return bare === NONE ? nearest(from, ok, least) : bare;
}

function scatter(work: State, from: number, ok: (i: number) => boolean): number {
  const near = all.filter((i) => ok(i) && far(i, from) <= RADIUS * RADIUS);
  return near.length ? near[draw(work, near.length)] : nearest(from, ok);
}

function found(work: State, bot: number): number {
  const own = passport(work, bot);
  const mine = work.towns.filter((tile) => own === NONE || work.region[tile] === own);
  const roomy = (i: number) =>
    free(work, bot, i) &&
    work.ground[i] !== WOOD &&
    all.filter((j) => far(i, j) <= ROOM && free(work, bot, j)).length >= ROOMY;
  if (!mine.length) return nearest(own === NONE ? work.shore : work.hearts[own], roomy);
  const start = draw(work, all.length);
  let best = NONE;
  let score = -1;
  for (let k = 0; k < all.length; k++) {
    const i = all[(start + k) % all.length];
    if (!roomy(i)) continue;
    const d = Math.min(...work.towns.map((tile) => far(tile, i)));
    if (d > score) {
      best = i;
      score = d;
    }
  }
  return best;
}

function town(work: State, bot: number): number {
  const own = passport(work, bot);
  for (let t = 0; t < work.towns.length; t++) {
    if (own !== NONE && work.region[work.towns[t]] !== own) continue;
    if (members(work, t).length < work.city) return t;
  }
  const tile = found(work, bot);
  if (tile === NONE) return NONE;
  work.towns.push(tile);
  return work.towns.length - 1;
}

function stack(work: State, t: number, cap: number): number {
  return nearest(work.towns[t], (i) => work.use[i] === HOME && work.of[i] === t && work.level[i] < cap);
}

function lot(work: State, bot: number, t: number): number {
  const center = work.towns[t];
  const ok = (i: number) => free(work, bot, i);
  if (work.own === "shared") {
    const tower = stack(work, t, Math.min(TOP, 1 + Math.floor((members(work, t).length + 1) / TOWER)));
    return tower === NONE ? clear(work, center, ok) : tower;
  }
  if (work.own === "none") return scatter(work, center, ok);
  const home = clear(work, center, ok);
  if (home === NONE) return stack(work, t, TOP);
  work.owner[home] = bot;
  for (let k = 1; k < CLAIM; k++) {
    const next = nearest(home, (i) => ok(i) && far(i, home) <= PLOT);
    if (next === NONE) break;
    work.owner[next] = bot;
  }
  return home;
}

function settle(work: State, bot: number) {
  const t = town(work, bot);
  const tile = t === NONE ? NONE : lot(work, bot, t);
  if (tile !== NONE) {
    if (work.use[tile] === HOME) work.level[tile]++;
    else {
      work.use[tile] = HOME;
      work.level[tile] = 1;
      work.of[tile] = t;
    }
    work.home[bot] = tile;
    work.member[bot] = t;
    work.fresh.push(tile);
    return;
  }
  const tent = nearest(
    work.shore,
    (i) => allowed(work, bot, i) && work.use[i] === EMPTY && around(i).some((j) => work.ground[j] === SEA),
  );
  if (tent === NONE) return;
  work.use[tent] = TENT;
  work.home[bot] = tent;
  work.fresh.push(tent);
}

function want(work: State, t: number): number {
  const n = members(work, t).length;
  const goal: Record<number, number> = {
    [FARM]: Math.ceil(n / FEEDS),
    [SCHOOL]: Math.ceil(n / SCHOOLS),
    [SHOP]: Math.floor(n / SHOPS),
    [PARK]: Math.ceil(n / PARKS),
  };
  let best = NONE;
  let gap = 0;
  for (const use of NEEDS) {
    const short = goal[use] - count(work, t, use);
    if (short > gap) {
      best = use;
      gap = short;
    }
  }
  return best;
}

function spot(work: State, bot: number, t: number, use: number): number {
  const center = work.towns[t];
  const home = work.home[bot];
  const ok = (i: number) => free(work, bot, i);
  if (work.own === "none") return scatter(work, home, ok);
  if (work.own === "private") {
    const mine = (i: number) => allowed(work, bot, i) && work.use[i] === EMPTY && work.owner[i] === bot;
    const plot = use === FARM || use === SHOP ? nearest(home, mine) : NONE;
    return plot === NONE ? clear(work, center, ok) : plot;
  }
  if (use !== FARM) return clear(work, center, ok);
  const core = Math.max(1, ...all.filter((i) => work.of[i] === t && work.use[i] === HOME).map((i) => far(i, center)));
  return clear(work, center, ok, Math.ceil(Math.sqrt(core) + 1) ** 2);
}

function building(work: State) {
  for (let t = 0; t < work.towns.length; t++) {
    const crew = members(work, t);
    if (!crew.length) continue;
    const times = 1 + Math.floor(crew.length / BUSY);
    for (let k = 0; k < times; k++) {
      const use = want(work, t);
      if (use === NONE) break;
      const bot = crew[(work.tick + k) % crew.length];
      const tile = spot(work, bot, t, use);
      if (tile === NONE) break;
      work.use[tile] = use;
      work.of[tile] = t;
      work.fresh.push(tile);
    }
  }
}

/* LAND */

function smooth(ground: number[], from: number, to: number) {
  const next = ground.slice();
  for (const i of all) {
    const [x, y] = xy(i);
    let same = 0;
    for (let dy = -1; dy <= 1; dy++)
      for (let dx = -1; dx <= 1; dx++) {
        const nx = x + dx;
        const ny = y + dy;
        if (nx >= 0 && ny >= 0 && nx < W && ny < H && ground[at(nx, ny)] === to) same++;
      }
    if (ground[i] === from && same >= 5) next[i] = to;
    if (ground[i] === to && same <= 3) next[i] = from;
  }
  return next;
}

function shape(work: State) {
  let ground = Array(W * H).fill(SEA);
  const blobs = BLOBS + draw(work, 3);
  for (let b = 0; b < blobs; b++) {
    const c = b ? at(8 + draw(work, W - 16), 6 + draw(work, H - 12)) : at(W / 2, H / 2);
    const r = (b ? 3 : 6) + draw(work, 4);
    for (const i of all) if (far(i, c) < r * r) ground[i] = LAND;
  }
  for (const i of all) if (roll(work) < 12) ground[i] = ground[i] === SEA ? LAND : SEA;
  for (let k = 0; k < 2; k++) ground = smooth(ground, SEA, LAND);
  for (const i of all) {
    const [x, y] = xy(i);
    if (x < 2 || y < 2 || x > W - 3 || y > H - 3) ground[i] = SEA;
  }
  const land = all.filter((i) => ground[i] === LAND);
  for (let f = 0; f < FORESTS; f++) {
    const c = land[draw(work, land.length)];
    const r = 2 + draw(work, 3);
    for (const i of land) if (far(i, c) <= r * r && roll(work) < WILD) ground[i] = WOOD;
  }
  ground = smooth(ground, LAND, WOOD);
  for (const i of all) if (ground[i] === WOOD && around(i).some((j) => ground[j] === SEA)) ground[i] = LAND;
  work.ground = ground;
  const shores = all.filter((i) => ground[i] === SEA && around(i).some((j) => ground[j] !== SEA));
  work.dock = nearest(at(W / 2, H), (i) => shores.includes(i));
  work.shore = nearest(work.dock, (i) => ground[i] !== SEA);
  const hearts = [land[draw(work, land.length)]];
  while (hearts.length < REGIONS) {
    let best = hearts[0];
    let score = -1;
    for (const i of land) {
      const d = Math.min(...hearts.map((h) => far(h, i)));
      if (d > score) {
        best = i;
        score = d;
      }
    }
    hearts.push(best);
  }
  work.hearts = hearts;
  work.region = all.map((i) => {
    if (ground[i] === SEA) return NONE;
    let best = 0;
    for (let r = 1; r < REGIONS; r++) if (far(hearts[r], i) < far(hearts[best], i)) best = r;
    return best;
  });
}

/* REDUCER */

function copy(state: State): State {
  return {
    ...state,
    use: state.use.slice(),
    level: state.level.slice(),
    owner: state.owner.slice(),
    of: state.of.slice(),
    towns: state.towns.slice(),
    home: state.home.slice(),
    member: state.member.slice(),
    fresh: [],
    landed: [],
  };
}

export function boot({ bots, own, city, borders, length, seed }: Config): State {
  if (!OWNS.includes(own)) throw new Error(`settle: no own ${own}`);
  if (!BORDERS.includes(borders)) throw new Error(`settle: no borders ${borders}`);
  if (bots < 1 || city < 1) throw new Error("settle: a world needs a bot and a town of one");
  const work: State = {
    bots,
    own,
    city,
    borders,
    length,
    tick: 0,
    ground: [],
    region: [],
    hearts: [],
    use: Array(W * H).fill(EMPTY),
    level: Array(W * H).fill(0),
    owner: Array(W * H).fill(NONE),
    of: Array(W * H).fill(NONE),
    towns: [],
    dock: NONE,
    shore: NONE,
    home: Array(bots).fill(NONE),
    member: Array(bots).fill(NONE),
    fresh: [],
    landed: [],
    over: null,
    seed: mix(seed),
  };
  shape(work);
  return work;
}

export function step(state: State, act: Act): State {
  if (state.over || act.verb !== "tick") return state;
  const work = copy(state);
  work.tick = state.tick + 1;
  for (let bot = 0; bot < work.bots; bot++) {
    if (arrives(work, bot) !== work.tick) continue;
    work.landed.push(bot);
    settle(work, bot);
  }
  building(work);
  work.over = work.tick >= state.length ? "done" : null;
  return work;
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

export const run = (state: State, ticks: number) => {
  let here = state;
  for (let i = 0; i < ticks && !here.over; i++) here = step(here, tick);
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
  { bots, own, city, borders }: Pick<Config, "bots" | "own" | "city" | "borders">,
  { seed, round = 0 }: Table,
): State {
  return boot({ bots, own, city, borders, length: DAYS, seed: dealt(seed, round) });
}
