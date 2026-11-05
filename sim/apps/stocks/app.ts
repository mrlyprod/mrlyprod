import { notice } from "@mrlyprod/apps/shared/people.ts";
import { pay } from "@mrlyprod/apps/wallet/app.ts";
import type { Ctx, Json, Knob, Knobs, Obj, Schema, Who } from "@mrlyprod/os/contract/index.ts";
import { app, below, sliced, stream } from "@mrlyprod/os/contract/index.ts";

export type Stock = {
  symbol: string;
  name: string;
  org: number | null;
  base: number;
  seed: number;
  pool: number | null;
  since: number;
};

export type Holdings = Record<string, number>;

export type Dials = { beat: number; swing: number; history: number };

export type Stocks = {
  dials: Dials;
  board: Record<string, Stock>;
  watching: Record<string, string[]>;
  holdings: Record<string, Holdings>;
};

type Org = { n: number; name: string; dissolved: number | null; members: Record<string, string> };

export const EXCHANGE = "exchange";

const BANDS = 6;

const SYMBOL = 8;

const KNOBS = {
  companies: {
    line: "Seeded companies on the board at the start.",
    schema: { type: "integer", minimum: 1, maximum: 48 },
    reality: 12,
  },
  low: {
    line: "The lowest opening price of a share.",
    schema: { type: "integer", minimum: 1, maximum: 1000 },
    reality: 10,
  },
  high: {
    line: "The highest opening price of a share.",
    schema: { type: "integer", minimum: 1, maximum: 100000 },
    reality: 200,
  },
  swing: {
    line: "Thousandths of its opening price a share may move in one beat.",
    schema: { type: "integer", minimum: 0, maximum: 200 },
    reality: 20,
  },
  beat: {
    line: "World milliseconds a price holds before it moves.",
    schema: { type: "integer", minimum: 1000, maximum: 86400000 },
    reality: 3600000,
  },
  history: {
    line: "Past prices a watched share shows, and how far back its change looks.",
    schema: { type: "integer", minimum: 2, maximum: 200 },
    reality: 30,
  },
  watch: { line: "Symbols one watchlist holds.", schema: { type: "integer", minimum: 1, maximum: 500 }, reality: 20 },
  lot: { line: "Shares one trade may move.", schema: { type: "integer", minimum: 1, maximum: 1000000 }, reality: 1000 },
  members: {
    line: "Members an org needs before its owner may list it on the board.",
    schema: { type: "integer", minimum: 1, maximum: 1000 },
    reality: 3,
  },
  founder: {
    line: "Shares the owner who lists an org receives.",
    schema: { type: "integer", minimum: 0, maximum: 1000000 },
    reality: 100,
  },
} satisfies Knobs;

const HEADS = [
  "Nor",
  "Bel",
  "Cal",
  "Dor",
  "Fen",
  "Gal",
  "Hal",
  "Kel",
  "Lor",
  "Mar",
  "Pel",
  "Ros",
  "Sal",
  "Tor",
  "Val",
  "Wen",
];

const TAILS = ["vik", "ant", "mor", "den", "ith", "ora", "ius", "and", "ett", "ion"];

const KINDS = [
  "Mills",
  "Works",
  "Freight",
  "Foods",
  "Tools",
  "Glass",
  "Paper",
  "Timber",
  "Salt",
  "Lamps",
  "Looms",
  "Wells",
];

export function symbolOf(name: string, taken: readonly string[]): string {
  const words = name
    .toUpperCase()
    .replace(/[^A-Z ]/g, " ")
    .split(" ")
    .filter(Boolean);
  const initials = words.map((word) => word[0]!).join("");
  const base = (initials.length >= 2 ? initials : (words[0] ?? "CO").padEnd(3, "X")).slice(0, 4);
  let symbol = base;
  for (let i = 2; taken.includes(symbol); i++) symbol = `${base}${i}`;
  return symbol;
}

const listed = (symbol: string, name: string, org: number | null, base: number, seed: number, since = 0): Stock => ({
  symbol,
  name,
  org,
  base,
  seed,
  pool: org === null ? null : 0,
  since,
});

function opening(draw: () => number, knob: Knob): number {
  const [low, high] = [knob("low"), knob("high")].sort((a, b) => a - b) as [number, number];
  return low + below(draw, high - low + 1);
}

function seeded(seed: number, knob: Knob): Record<string, Stock> {
  const { draw } = stream(seed);
  const board: Record<string, Stock> = {};
  let kinds: string[] = [];
  while (Object.keys(board).length < knob("companies")) {
    if (!kinds.length) kinds = [...KINDS];
    const kind = kinds.splice(below(draw, kinds.length), 1)[0]!;
    const name = `${HEADS[below(draw, HEADS.length)]}${TAILS[below(draw, TAILS.length)]} ${kind}`;
    const symbol = symbolOf(name, Object.keys(board));
    board[symbol] = listed(symbol, name, null, opening(draw, knob), draw());
  }
  return board;
}

function wave(seed: number, band: number, cell: number, reach: number): number {
  const { draw } = stream((seed ^ Math.imul(cell + 1, 0x9e3779b1) ^ Math.imul(band + 1, 0x85ebca6b)) >>> 0);
  draw();
  draw();
  return below(draw, 2 * reach + 1) - reach;
}

function drift(seed: number, step: number, swing: number): number {
  let sum = 0;
  for (let band = 0; band < BANDS; band++) {
    const span = 4 ** band;
    const cell = Math.floor(step / span);
    const into = step - cell * span;
    const reach = swing * (band + 1);
    const from = wave(seed, band, cell, reach);
    sum += into ? from + Math.trunc(((wave(seed, band, cell + 1, reach) - from) * into) / span) : from;
  }
  return sum;
}

const stepOf = (state: Stocks, at: number) => Math.floor(at / state.dials.beat);

const priced = (state: Stocks, one: Stock, step: number) =>
  Math.max(1, one.base + Math.trunc((one.base * drift(one.seed, step, state.dials.swing)) / 1000));

export const priceOf = (state: Stocks, symbol: string, at: number) =>
  priced(state, state.board[symbol]!, stepOf(state, at));

const changeOf = (state: Stocks, one: Stock, step: number) =>
  priced(state, one, step) - priced(state, one, Math.max(one.since ?? 0, step - state.dials.history));

function orgOf(ctx: Ctx, n: number): Org {
  if (ctx.read("people") === null)
    return ctx.refuse("this world has no people, so no orgs to list; the board holds the seeded companies");
  const one = ctx.read(`people/orgs/${n}`) as Org | null;
  if (!one) return ctx.refuse(`no org ${n} in people`);
  if (one.dissolved !== null) return ctx.refuse(`${one.name} was dissolved`);
  if (one.members[ctx.who.id] !== "owner") return ctx.refuse(`only an owner of ${one.name} lists it`);
  return one;
}

function named(state: Stocks, args: Obj, ctx: Ctx): string {
  const symbol = String(args.symbol).trim().toUpperCase();
  if (!Object.hasOwn(state.board, symbol))
    return ctx.refuse(`no symbol ${symbol} on the board; read stocks/market for the symbols`);
  return symbol;
}

const quoted = (ctx: Ctx, symbol: string) => ctx.see(`stocks/market/${symbol}/price`) as number;

const holding = (state: Stocks, id: string, symbol: string) => state.holdings[id]?.[symbol] ?? 0;

function traded(state: Stocks, id: string, symbol: string, qty: number): Stocks {
  const mine = { ...(state.holdings[id] ?? {}) };
  const now = holding(state, id, symbol) + qty;
  if (now > 0) mine[symbol] = now;
  else delete mine[symbol];
  const holdings = { ...state.holdings };
  if (Object.keys(mine).length) holdings[id] = mine;
  else delete holdings[id];
  return { ...state, holdings };
}

function pooled(state: Stocks, symbol: string, coins: number): Stocks {
  const one = state.board[symbol]!;
  if (one.pool === null) return state;
  return { ...state, board: { ...state.board, [symbol]: { ...one, pool: one.pool + coins } } };
}

const market = (state: Stocks, at: number): Obj => {
  const step = stepOf(state, at);
  return Object.fromEntries(
    Object.keys(state.board)
      .sort()
      .map((symbol) => {
        const one = state.board[symbol]!;
        const org: Obj = one.org === null ? {} : { org: one.org, backed: one.pool ?? 0 };
        return [
          symbol,
          { name: one.name, price: priced(state, one, step), change: changeOf(state, one, step), ...org },
        ];
      }),
  );
};

const nextMove = (state: Stocks, at: number) => (stepOf(state, at) + 1) * state.dials.beat;

const trail = (state: Stocks, one: Stock, step: number): Json =>
  Array.from({ length: state.dials.history }, (_, back) => step - back)
    .filter((past) => past >= 0)
    .reverse()
    .map((past) => priced(state, one, past));

const watched = (state: Stocks, id: string, at: number) => {
  const step = stepOf(state, at);
  return (state.watching[id] ?? []).map((symbol) => {
    const one = state.board[symbol]!;
    return { symbol, name: one.name, price: priced(state, one, step), change: changeOf(state, one, step) };
  });
};

const sellable = (one: Stock, qty: number, price: number) =>
  one.pool === null ? qty : Math.min(qty, Math.floor(one.pool / price));

const portfolio = (state: Stocks, id: string, at: number) => {
  const step = stepOf(state, at);
  const holdings = Object.entries(state.holdings[id] ?? {}).map(([symbol, qty]) => {
    const one = state.board[symbol]!;
    const price = priced(state, one, step);
    const sells = sellable(one, qty, price);
    return { symbol, name: one.name, qty, price, sellable: sells, value: price * sells };
  });
  return { holdings, worth: holdings.reduce((sum, one) => sum + one.value, 0) };
};

const summary = (state: Stocks, at: number) => ({ next: nextMove(state, at), market: market(state, at) });

const mine = (state: Stocks, id: string, _who: Who | null, at: number) => ({
  ...summary(state, at),
  watching: watched(state, id, at).map((one) => ({
    ...one,
    history: trail(state, state.board[one.symbol]!, stepOf(state, at)),
  })),
  ...portfolio(state, id, at),
});

const whole = (state: Stocks, who: Who | null, at: number) =>
  who
    ? { ...summary(state, at), watching: watched(state, who.id, at), ...portfolio(state, who.id, at) }
    : summary(state, at);

const naming: Schema = {
  type: "object",
  properties: { symbol: { type: "string", minLength: 1, maxLength: SYMBOL } },
  required: ["symbol"],
  additionalProperties: false,
};

const trading = (knob: Knob): Schema => ({
  type: "object",
  properties: {
    symbol: { type: "string", minLength: 1, maxLength: SYMBOL },
    qty: { type: "integer", minimum: 1, maximum: knob("lot") },
  },
  required: ["symbol", "qty"],
  additionalProperties: false,
});

const listing: Schema = {
  type: "object",
  properties: { org: { type: "integer", minimum: 1 } },
  required: ["org"],
  additionalProperties: false,
};

export default app<Stocks>({
  id: "stocks",
  title: "Stocks",
  line: "A public board of seeded companies and listed orgs, prices moving with world time; a private watchlist and portfolio each, bought from and sold to the exchange through the wallet.",
  boot: ({ seed, knob }) => ({
    dials: { beat: knob("beat"), swing: knob("swing"), history: knob("history") },
    board: seeded(seed, knob),
    watching: {},
    holdings: {},
  }),
  verbs: {
    buy: {
      line: (knob) =>
        `Buy up to ${knob("lot")} shares of a symbol from the exchange at the price stocks/market shows now; your wallet pays in the same act.`,
      args: trading,
      run: (state, args, ctx) => {
        const symbol = named(state, args, ctx);
        const qty = Number(args.qty);
        const price = quoted(ctx, symbol);
        const cash = ctx.read(`wallet/balances/${ctx.who.id}`);
        if (typeof cash === "number" && price * qty > cash)
          return ctx.refuse(
            `${qty} ${symbol} cost ${price * qty} at ${price} each and you hold ${cash}; ${cash < price ? "earn more first" : `buy at most ${Math.floor(cash / price)}`}`,
          );
        const cost = price * qty;
        pay(ctx, EXCHANGE, cost);
        ctx.say({ note: `Bought ${qty} ${symbol} for ${cost}`, id: symbol, qty, cost });
        return pooled(traded(state, ctx.who.id, symbol, qty), symbol, cost);
      },
    },
    sell: {
      line: "Sell shares you hold to the exchange at the price stocks/market shows now; it pays your wallet in the same act. A listed org pays out only what its buyers paid in.",
      args: trading,
      run: (state, args, ctx) => {
        const symbol = named(state, args, ctx);
        const qty = Number(args.qty);
        const had = holding(state, ctx.who.id, symbol);
        if (had < qty) return ctx.refuse(`you hold ${had} ${symbol}, not ${qty}`);
        const got = quoted(ctx, symbol) * qty;
        const { pool } = state.board[symbol]!;
        if (pool !== null && pool < got)
          return ctx.refuse(
            `buyers of ${symbol} have paid in ${pool}, less than the ${got} you ask; sell fewer, or wait for buyers`,
          );
        pay(ctx, ctx.who.id, got, EXCHANGE);
        ctx.say({ note: `Sold ${qty} ${symbol} for ${got}`, id: symbol, qty, got });
        return pooled(traded(state, ctx.who.id, symbol, -qty), symbol, -got);
      },
    },
    watch: {
      line: (knob) => `Add a symbol to your watchlist; it holds at most ${knob("watch")}.`,
      args: naming,
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const symbol = named(state, args, ctx);
        const most = ctx.knob("watch");
        const watching = state.watching[me] ?? [];
        if (watching.includes(symbol)) return ctx.refuse(`you watch ${symbol} already`);
        if (watching.length >= most) return ctx.refuse(`your watchlist is full at ${most}; unwatch one first`);
        ctx.say({ note: `Watching ${symbol}`, id: symbol });
        return { ...state, watching: { ...state.watching, [me]: [...watching, symbol] } };
      },
    },
    unwatch: {
      line: "Take a symbol off your watchlist.",
      args: naming,
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const symbol = named(state, args, ctx);
        const watching = state.watching[me] ?? [];
        if (!watching.includes(symbol)) return ctx.refuse(`you do not watch ${symbol}`);
        ctx.say({ note: `No longer watching ${symbol}`, id: symbol });
        return { ...state, watching: { ...state.watching, [me]: watching.filter((one) => one !== symbol) } };
      },
    },
    list: {
      line: (knob) =>
        `As an owner of an org of people, list it on the board under a symbol drawn from its name; you receive ${knob("founder")} founder shares.`,
      args: listing,
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const org = orgOf(ctx, Number(args.org));
        const people = Object.keys(org.members).length;
        const least = ctx.knob<number>("members");
        if (people < least)
          return ctx.refuse(`${org.name} has ${people} members; an org needs ${least} to list; invite more first`);
        const held = Object.values(state.board).find((one) => one.org === org.n);
        if (held) return ctx.refuse(`${org.name} is listed as ${held.symbol} already`);
        const symbol = symbolOf(org.name, Object.keys(state.board));
        for (const member of Object.keys(org.members))
          if (member !== me) ctx.wish(notice(member, `${me} listed ${org.name} on the board as ${symbol}`));
        const stock = listed(symbol, org.name, org.n, opening(ctx.draw, ctx.knob), ctx.draw(), stepOf(state, ctx.at));
        const next = { ...state, board: { ...state.board, [symbol]: stock } };
        const founder = ctx.knob("founder");
        ctx.say({
          note: `Listed ${org.name} as ${symbol}; you hold ${founder} founder shares, which sell only for what buyers paid in`,
          id: symbol,
        });
        return traded(next, me, symbol, founder);
      },
    },
  },
  read: sliced(mine, whole),
  knobs: KNOBS,
});
