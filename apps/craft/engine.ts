import { mix, xorshift } from "../../net/sdk/rand.ts";

/* TYPES */

export type Work = "earn" | "free";

export type Over = "done" | null;

export type State = {
  bots: number;
  work: Work;
  teams: number;
  length: number;
  tick: number;
  need: number;
  love: number[];
  drive: number[];
  skill: number[];
  xp: number[];
  at: number[];
  team: number[];
  ate: number[];
  made: number[];
  progress: number[];
  spark: number[];
  shipped: number[];
  ships: number[];
  days: number;
  fed: number;
  loved: number;
  worked: number;
  idle: number;
  busy: number;
  covered: number;
  needed: number;
  effort: number;
  dropped: number;
  sparks: number;
  over: Over;
  seed: number;
};

export type Config = Pick<State, "bots" | "work" | "teams" | "length" | "seed">;

export type Table = { seed?: number; round?: number };

export type Act = { verb: "tick"; args: Record<string, never> };

export type Report = {
  fed: number;
  loved: number;
  idle: number;
  busy: number;
  basics: number;
  shipped: number;
  best: number;
  dropped: number;
  working: number;
  resting: number;
  office: number;
  hungry: number;
};

export const WORKS: Work[] = ["earn", "free"];
export const CRAFTS = 6;
export const BASICS = 3;
export const OFFICE = 6;
export const REST = 7;
export const TEAMS = 8;
export const TOP = 9;
export const SPARK = 10;
export const DAYS = 60;
export const SIZE = 160;
export const MILLE = 1000;
export const NONE = -1;
export const tick: Act = { verb: "tick", args: {} };

const HUNDRED = 100;
const NEED = 110;
const MARKET = 16;
const DESKS = 3;
const CALL = 35;
const PRACTICE = 8;
const LOVE = 2;
const BASE = 5;
const DRIVE = 4;

/* REPORT */

export const ideas = (craft: number) => craft >= BASICS && craft < CRAFTS;

export const skill = (state: Pick<State, "skill">, bot: number, craft: number) => state.skill[bot * CRAFTS + craft];

const share = (n: number, of: number) => (of ? Math.floor((n * MILLE) / of) : 0);

export function report(state: State): Report {
  const today = (place: number) => state.at.filter((one) => one === place).length;
  const shipped = state.shipped.reduce((a, b) => a + b, 0);
  return {
    fed: share(state.fed, state.days),
    loved: share(state.loved, state.worked),
    idle: share(state.idle, state.days),
    busy: share(state.busy, state.days),
    basics: share(state.covered, state.needed),
    shipped,
    best: share(state.sparks, shipped * (SPARK - 1)),
    dropped: share(state.dropped, state.effort),
    working: state.at.filter((one) => one < CRAFTS).length,
    resting: today(REST),
    office: today(OFFICE),
    hungry: state.tick ? state.ate.filter((one) => !one).length : 0,
  };
}

/* RULES */

const roll = (work: State, n = HUNDRED) => {
  work.seed = xorshift(work.seed);
  return work.seed % n;
};

function best(work: State, craft: number, ok: (bot: number) => boolean): number {
  let pick = NONE;
  let score = -1;
  for (let bot = 0; bot < work.bots; bot++) {
    if (!ok(bot) || skill(work, bot, craft) <= score) continue;
    pick = bot;
    score = skill(work, bot, craft);
  }
  return pick;
}

function earn(work: State) {
  const free = (bot: number) => work.at[bot] === REST;
  const hire = (bot: number, craft: number) => {
    work.at[bot] = craft;
    work.made[craft] += skill(work, bot, craft);
  };
  let open = true;
  while (open) {
    open = false;
    for (let craft = 0; craft < BASICS; craft++) {
      if (work.made[craft] >= work.need) continue;
      const bot = best(work, craft, free);
      if (bot === NONE) continue;
      hire(bot, craft);
      open = true;
    }
  }
  const posts = Math.ceil(work.bots / MARKET);
  for (let craft = BASICS; craft < CRAFTS; craft++)
    for (let k = 0; k < posts; k++) {
      const bot = best(work, craft, free);
      if (bot !== NONE) work.at[bot] = craft;
    }
  const start = roll(work, work.bots);
  let desks = Math.ceil(work.bots / DESKS);
  for (let k = 0; k < work.bots && desks > 0; k++) {
    const bot = (start + k) % work.bots;
    if (!free(bot)) continue;
    work.at[bot] = OFFICE;
    desks--;
  }
}

function free(work: State) {
  for (let bot = 0; bot < work.bots; bot++) {
    if (roll(work) >= work.drive[bot] * SPARK) continue;
    work.at[bot] = work.love[bot];
    if (!ideas(work.love[bot])) work.made[work.love[bot]] += skill(work, bot, work.love[bot]);
  }
  const asked = Array(work.bots).fill(false);
  const open = (bot: number) => !asked[bot] && (work.at[bot] === REST || ideas(work.at[bot]));
  let calling = true;
  while (calling) {
    calling = false;
    for (let craft = 0; craft < BASICS; craft++) {
      if (work.made[craft] >= work.need) continue;
      const bot = best(work, craft, open);
      if (bot === NONE) continue;
      asked[bot] = true;
      calling = true;
      if (roll(work) >= CALL) continue;
      work.at[bot] = craft;
      work.made[craft] += skill(work, bot, craft);
    }
  }
}

function race(work: State) {
  for (let craft = BASICS; craft < CRAFTS; craft++) {
    const crew = work.at.flatMap((one, bot) => (one === craft ? [bot] : []));
    const gain = Array(work.teams).fill(0);
    crew.forEach((bot, i) => {
      const k = i % work.teams;
      work.team[bot] = k;
      gain[k] += skill(work, bot, craft) + work.spark[craft * work.teams + k];
    });
    let lead = NONE;
    for (let k = 0; k < work.teams; k++) {
      const i = craft * work.teams + k;
      work.progress[i] += gain[k];
      work.made[craft] += gain[k];
      work.effort += gain[k];
      if (work.progress[i] >= SIZE && (lead === NONE || work.progress[i] > work.progress[craft * work.teams + lead]))
        lead = k;
    }
    if (lead === NONE) continue;
    const won = work.spark[craft * work.teams + lead];
    work.ships.push(craft, lead, won);
    work.shipped[craft]++;
    work.sparks += won;
    for (let k = 0; k < work.teams; k++) {
      const i = craft * work.teams + k;
      if (k !== lead) work.dropped += work.progress[i];
      work.progress[i] = 0;
      work.spark[i] = roll(work, SPARK);
    }
  }
}

function eat(work: State) {
  const food = Math.min(work.made[0], work.need);
  let portions = Math.floor((work.bots * food) / work.need);
  const start = roll(work, work.bots);
  for (let k = 0; k < work.bots; k++) {
    const bot = (start + k) % work.bots;
    const earned = work.work === "free" || work.at[bot] !== REST;
    work.ate[bot] = earned && portions > 0 ? 1 : 0;
    if (work.ate[bot]) portions--;
  }
}

function tally(work: State) {
  for (let bot = 0; bot < work.bots; bot++) {
    const at = work.at[bot];
    work.days++;
    work.fed += work.ate[bot];
    if (at === REST) work.idle++;
    if (at === OFFICE) work.busy++;
    if (at !== REST) work.worked++;
    if (at === work.love[bot]) work.loved++;
    if (at >= CRAFTS) continue;
    const i = bot * CRAFTS + at;
    work.xp[i]++;
    if (work.xp[i] < PRACTICE || work.skill[i] >= TOP) continue;
    work.skill[i]++;
    work.xp[i] = 0;
  }
  for (let craft = 0; craft < BASICS; craft++) {
    work.covered += Math.min(work.made[craft], work.need);
    work.needed += work.need;
  }
}

/* REDUCER */

function copy(state: State): State {
  return {
    ...state,
    skill: state.skill.slice(),
    xp: state.xp.slice(),
    at: Array(state.bots).fill(REST),
    team: Array(state.bots).fill(NONE),
    ate: Array(state.bots).fill(0),
    made: Array(CRAFTS).fill(0),
    progress: state.progress.slice(),
    spark: state.spark.slice(),
    shipped: state.shipped.slice(),
    ships: [],
  };
}

export function boot({ bots, work, teams, length, seed }: Config): State {
  if (!WORKS.includes(work)) throw new Error(`craft: no work ${work}`);
  if (bots < 1) throw new Error("craft: a world needs a bot");
  if (teams < 1 || teams > TEAMS) throw new Error(`craft: teams run from 1 to ${TEAMS}`);
  const state: State = {
    bots,
    work,
    teams,
    length,
    tick: 0,
    need: Math.ceil((bots * NEED) / HUNDRED),
    love: [],
    drive: [],
    skill: [],
    xp: Array(bots * CRAFTS).fill(0),
    at: Array(bots).fill(REST),
    team: Array(bots).fill(NONE),
    ate: Array(bots).fill(1),
    made: Array(CRAFTS).fill(0),
    progress: Array(CRAFTS * teams).fill(0),
    spark: [],
    shipped: Array(CRAFTS).fill(0),
    ships: [],
    days: 0,
    fed: 0,
    loved: 0,
    worked: 0,
    idle: 0,
    busy: 0,
    covered: 0,
    needed: 0,
    effort: 0,
    dropped: 0,
    sparks: 0,
    over: null,
    seed: mix(seed),
  };
  for (let bot = 0; bot < bots; bot++) {
    state.love.push(roll(state, CRAFTS));
    state.drive.push(DRIVE + roll(state, SPARK - DRIVE));
    for (let craft = 0; craft < CRAFTS; craft++)
      state.skill.push(Math.min(TOP, 1 + roll(state, BASE) + (craft === state.love[bot] ? LOVE : 0)));
  }
  for (let i = 0; i < CRAFTS * teams; i++) state.spark.push(roll(state, SPARK));
  return state;
}

export function step(state: State, act: Act): State {
  if (state.over || act.verb !== "tick") return state;
  const work = copy(state);
  work.tick = state.tick + 1;
  if (work.work === "earn") earn(work);
  else free(work);
  race(work);
  eat(work);
  tally(work);
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
  { bots, work, teams }: Pick<Config, "bots" | "work" | "teams">,
  { seed, round = 0 }: Table,
): State {
  return boot({ bots, work, teams, length: DAYS, seed: dealt(seed, round) });
}
