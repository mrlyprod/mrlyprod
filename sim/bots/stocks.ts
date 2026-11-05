import type { Json } from "@mrlyprod/os/contract/index.ts";
import { below, pick } from "@mrlyprod/os/contract/index.ts";
import type { Stocks } from "../apps/stocks/app.ts";
import { priceOf } from "../apps/stocks/app.ts";
import type { Bot, Step } from "./types.ts";

type Org = { n: number; dissolved: number | null; members: Record<string, string> };

const owned = (apps: Record<string, Json>, me: string): Org[] => {
  const people = apps.people;
  const orgs = people && typeof people === "object" && !Array.isArray(people) ? people.orgs : null;
  const list = orgs && typeof orgs === "object" && !Array.isArray(orgs) ? (Object.values(orgs) as Org[]) : [];
  return list.filter((one) => one.dissolved === null && one.members[me] === "owner");
};

const balance = (apps: Record<string, Json>, id: string): number | null => {
  const wallet = apps.wallet;
  const balances = wallet && typeof wallet === "object" && !Array.isArray(wallet) ? wallet.balances : null;
  if (!balances || typeof balances !== "object" || Array.isArray(balances)) return null;
  const held = balances[id];
  return typeof held === "number" ? held : 0;
};

const bot: Bot<Stocks> = ({ state, me, draw, world }): Step => {
  const listed = new Set(Object.values(state.board).map((one) => one.org));
  const unlisted = owned(world.apps, me).filter((one) => !listed.has(one.n));
  if (unlisted.length && below(draw, 2)) return { verb: "list", args: { org: pick(draw, unlisted).n } };
  const cash = balance(world.apps, me);
  const held = Object.entries(state.holdings[me] ?? {});
  if (cash !== null && below(draw, 2)) {
    const symbol = pick(draw, Object.keys(state.board).sort());
    const afford = Math.min(10, Math.floor(cash / priceOf(state, symbol, world.at)));
    if (afford >= 1) return { verb: "buy", args: { symbol, qty: 1 + below(draw, afford) } };
  }
  if (cash !== null && held.length && below(draw, 2)) {
    const [symbol, qty] = pick(draw, held);
    return { verb: "sell", args: { symbol, qty: 1 + below(draw, qty) } };
  }
  const watching = state.watching[me] ?? [];
  const fresh = Object.keys(state.board).filter((symbol) => !watching.includes(symbol));
  if (fresh.length && watching.length < 4 && below(draw, 2))
    return { verb: "watch", args: { symbol: pick(draw, fresh) } };
  if (watching.length && !below(draw, 5)) return { verb: "unwatch", args: { symbol: pick(draw, watching) } };
  return null;
};

export default bot;
