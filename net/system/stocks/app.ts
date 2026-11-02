import type { Ctx, Schema } from "../../sdk/system.ts";
import { app, below, sliced, stream } from "../../sdk/system.ts";
import { bot } from "./bot.ts";

/* TYPES */

export type Stock = { symbol: string; name: string; org: number | null; price: number; history: number[] };

export type Holdings = Record<string, number>;

export type Stocks = {
  seed: number;
  tick: number;
  board: Record<string, Stock>;
  watching: Record<string, string[]>;
  holdings: Record<string, Holdings>;
};

type Org = { n: number; name: string; dissolved: number | null; members: Record<string, string> };

export const COMPANIES = 12;

export const HISTORY = 30;

export const WATCH = 20;

export const MOVE = 20;

export const LOT = 1000;

export const EXCHANGE = "exchange";

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

/* BOARD */

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

const listed = (symbol: string, name: string, org: number | null, price: number): Stock => ({
  symbol,
  name,
  org,
  price,
  history: [],
});

const opening = (draw: () => number) => (5 + below(draw, 196)) * 100;

function seeded(seed: number): Record<string, Stock> {
  const { draw } = stream(seed);
  const board: Record<string, Stock> = {};
  const kinds = [...KINDS];
  while (Object.keys(board).length < COMPANIES && kinds.length) {
    const kind = kinds.splice(below(draw, kinds.length), 1)[0]!;
    const name = `${HEADS[below(draw, HEADS.length)]}${TAILS[below(draw, TAILS.length)]} ${kind}`;
    const symbol = symbolOf(name, Object.keys(board));
    board[symbol] = listed(symbol, name, null, opening(draw));
  }
  return board;
}

export function advance(state: Stocks, tick: number): Stocks {
  if (tick <= state.tick) return state;
  const board = Object.fromEntries(Object.entries(state.board).map(([symbol, one]) => [symbol, { ...one }]));
  const symbols = Object.keys(board).sort();
  for (let t = state.tick + 1; t <= tick; t++) {
    const { draw } = stream((state.seed ^ Math.imul(t, 0x9e3779b1)) >>> 0);
    draw();
    for (const symbol of symbols) {
      const one = board[symbol]!;
      const step = below(draw, 2 * MOVE + 1) - MOVE;
      one.history = [...one.history, one.price].slice(-HISTORY);
      one.price = Math.max(1, one.price + Math.trunc((one.price * step) / 1000));
    }
  }
  return { ...state, tick, board };
}

function orgOf(ctx: Ctx, n: number): Org {
  if (ctx.read("people") === null)
    return ctx.refuse("this world has no people, so no orgs to list; the board holds the seeded companies");
  const one = ctx.read(`people/orgs/${n}`) as Org | null;
  if (!one) return ctx.refuse(`no org ${n} in people`);
  if (one.dissolved !== null) return ctx.refuse(`${one.name} was dissolved`);
  if (one.members[ctx.who.id] !== "owner") return ctx.refuse(`only an owner of ${one.name} lists it`);
  return one;
}

/* TRADES */

const quote = (state: Stocks, tick: number, symbol: string, qty: number) =>
  advance(state, tick).board[symbol]!.price * qty;

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

const mine = (state: Stocks, id: string) => ({
  tick: state.tick,
  watching: (state.watching[id] ?? []).map((symbol) => {
    const one = state.board[symbol]!;
    return { symbol, name: one.name, price: one.price, change: one.price - (one.history[0] ?? one.price) };
  }),
  holdings: Object.entries(state.holdings[id] ?? {}).map(([symbol, qty]) => {
    const one = state.board[symbol]!;
    return { symbol, name: one.name, qty, price: one.price, value: one.price * qty };
  }),
});

/* SCHEMAS */

const naming = (state: Stocks): Schema => ({
  type: "object",
  properties: { symbol: { enum: Object.keys(state.board).sort() } },
  required: ["symbol"],
  additionalProperties: false,
});

const trading = (state: Stocks): Schema => ({
  type: "object",
  properties: {
    symbol: { enum: Object.keys(state.board).sort() },
    qty: { type: "integer", minimum: 1, maximum: LOT },
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

/* APP */

export default app<Stocks>({
  id: "stocks",
  title: "Stocks",
  line: `A board of ${COMPANIES} seeded companies and the orgs listed on it, prices in cents moving every tick; a watchlist and a portfolio each, bought from and sold to the exchange through the wallet.`,
  boot: ({ seed }) => ({ seed, tick: 0, board: seeded(seed), watching: {}, holdings: {} }),
  verbs: {
    buy: {
      line: `Buy up to ${LOT} shares of a symbol from the exchange at the price now; your wallet pays in the same act.`,
      args: trading,
      pay: (state, args, ctx) => ({
        to: EXCHANGE,
        amount: quote(state, ctx.tick, String(args.symbol), Number(args.qty)),
      }),
      run: (state, args, ctx) => traded(advance(state, ctx.tick), ctx.who.id, String(args.symbol), Number(args.qty)),
    },
    sell: {
      line: "Sell shares you hold to the exchange at the price now; it pays your wallet in the same act.",
      args: trading,
      pay: (state, args, ctx) => {
        const symbol = String(args.symbol);
        const qty = Number(args.qty);
        const had = holding(state, ctx.who.id, symbol);
        if (had < qty) return ctx.refuse(`you hold ${had} ${symbol}, not ${qty}`);
        return { from: EXCHANGE, to: ctx.who.id, amount: quote(state, ctx.tick, symbol, qty) };
      },
      run: (state, args, ctx) => traded(advance(state, ctx.tick), ctx.who.id, String(args.symbol), -Number(args.qty)),
    },
    watch: {
      line: `Add a symbol to your watchlist; it holds at most ${WATCH}.`,
      args: naming,
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const symbol = String(args.symbol);
        const watching = state.watching[me] ?? [];
        if (watching.includes(symbol)) return ctx.refuse(`you watch ${symbol} already`);
        if (watching.length >= WATCH) return ctx.refuse(`you watch ${WATCH} symbols; unwatch one first`);
        return { ...advance(state, ctx.tick), watching: { ...state.watching, [me]: [...watching, symbol] } };
      },
    },
    unwatch: {
      line: "Take a symbol off your watchlist.",
      args: naming,
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const symbol = String(args.symbol);
        const watching = state.watching[me] ?? [];
        if (!watching.includes(symbol)) return ctx.refuse(`you do not watch ${symbol}`);
        return {
          ...advance(state, ctx.tick),
          watching: { ...state.watching, [me]: watching.filter((one) => one !== symbol) },
        };
      },
    },
    list: {
      line: "As an owner of an org of people, list it on the board under a symbol drawn from its name.",
      args: listing,
      run: (state, args, ctx) => {
        const org = orgOf(ctx, Number(args.org));
        const held = Object.values(state.board).find((one) => one.org === org.n);
        if (held) return ctx.refuse(`${org.name} is listed as ${held.symbol} already`);
        const symbol = symbolOf(org.name, Object.keys(state.board));
        const next = advance(state, ctx.tick);
        return { ...next, board: { ...next.board, [symbol]: listed(symbol, org.name, org.n, opening(ctx.draw)) } };
      },
    },
  },
  read: sliced(mine),
  bot,
});
