import type { Listing, Market } from "@mrlyprod/apps/market/app.ts";
import { stockOf } from "@mrlyprod/apps/market/app.ts";
import { allowed } from "@mrlyprod/apps/wallet/app.ts";
import { below, pick } from "@mrlyprod/os/contract/index.ts";
import { reader } from "@mrlyprod/os/kernel/index.ts";
import type { Body } from "../apps/body/needs.ts";
import { LOW, levelsOf } from "../apps/body/needs.ts";
import type { Bot, Step } from "./types.ts";

type Bank = { balances?: Record<string, number> };

const CRAFTS = ["Knitted scarf", "Walnut loaf", "Clay mug", "Wooden spoon", "Pickled beets", "Hand-drawn map"];

const WORDS = ["Just what I needed.", "Good value.", "Could be fresher.", "Would buy again.", "It did the job."];

const OPEN = 3;

const BASKET = 3;

const buying = (one: Listing, at: number, cash: number, draw: () => number, lot: number): Step => {
  const most = Math.min(BASKET, lot, stockOf(one, at), Math.floor(cash / one.price));
  return { verb: "buy", args: { item: one.id, n: 1 + below(draw, most) } };
};

const selling = (state: Market, me: string, draw: () => number): Step => {
  const mine = Object.values(state.listings).filter((one) => one.open && one.by === me);
  if (mine.length >= Math.min(OPEN, state.dials.sellers)) return null;
  const spare = Object.values(state.own[me] ?? {}).filter((one) => one.n > 1);
  const name = spare.length && below(draw, 2) ? pick(draw, spare).name : pick(draw, CRAFTS);
  return { verb: "sell", args: { name, price: 2 + below(draw, 20), about: "Fresh today." } };
};

const reviewing = (state: Market, me: string, draw: () => number): Step => {
  const open = Object.values(state.listings).filter(
    (one) => one.by !== me && one.buyers.includes(me) && !(me in one.rated),
  );
  return open.length
    ? { verb: "review", args: { listing: pick(draw, open).id, stars: 1 + below(draw, 5), text: pick(draw, WORDS) } }
    : null;
};

const giving = (state: Market, me: string, others: string[], draw: () => number): Step => {
  const held = Object.keys(state.own[me] ?? {});
  return held.length && others.length
    ? { verb: "give", args: { to: pick(draw, others), item: pick(draw, held) } }
    : null;
};

const answering = (state: Market, me: string, cash: number, draw: () => number): Step => {
  const waiting = Object.values(state.offers).filter((one) => one.to === me);
  if (!waiting.length) return null;
  const one = pick(draw, waiting);
  return { verb: one.price * one.n <= cash && below(draw, 3) ? "accept" : "decline", args: { offer: one.id } };
};

const offering = (state: Market, me: string, others: string[], draw: () => number): Step => {
  const stale = Object.values(state.offers).filter((one) => one.by === me);
  if (stale.length >= Math.min(OPEN, state.dials.offers))
    return stale.length ? { verb: "decline", args: { offer: pick(draw, stale).id } } : null;
  const spare = Object.entries(state.own[me] ?? {}).filter(([, one]) => one.n > 1);
  if (!spare.length || !others.length) return null;
  const [item, one] = pick(draw, spare);
  return {
    verb: "offer",
    args: {
      to: pick(draw, others),
      item,
      n: 1 + below(draw, Math.min(one.n, state.dials.lot)),
      price: 1 + below(draw, 15),
    },
  };
};

const bot: Bot<Market> = ({ state, me, actors, at, draw, world }): Step => {
  const food = Object.values(state.own[me] ?? {}).filter((one) => Number(one.traits.food ?? 0) > 0);
  const cash = (world.apps.wallet as unknown as Bank | undefined)?.balances?.[me] ?? 0;
  const shop = Object.values(state.listings).filter(
    (one) => one.open && one.by !== me && one.price <= cash && stockOf(one, at) > 0,
  );
  const answer = answering(state, me, cash, draw);
  if (answer) return answer;
  const groceries = shop.filter((one) => Number(one.traits.food ?? 0) > 0);
  const body = world.apps.body as unknown as Body | undefined;
  const hungry = body ? levelsOf(body, me, at).hunger >= LOW : true;
  const lot = state.dials.lot;
  if (groceries.length && food.length < 2 && (hungry || !food.length))
    return buying(pick(draw, groceries), at, cash, draw, lot);
  const others = actors.filter((id) => id !== me);
  switch (below(draw, 7)) {
    case 0:
      return shop.length ? buying(pick(draw, shop), at, cash, draw, lot) : null;
    case 1:
      return allowed(reader(world.rules, "wallet"), "payments") ? selling(state, me, draw) : null;
    case 2:
      return reviewing(state, me, draw);
    case 3:
      return allowed(reader(world.rules, "wallet"), "gifts") ? giving(state, me, others, draw) : null;
    case 4: {
      const mine = Object.values(state.listings).filter((one) => one.open && one.by === me);
      return mine.length && !below(draw, 3) ? { verb: "unlist", args: { listing: pick(draw, mine).id } } : null;
    }
    case 5:
      return allowed(reader(world.rules, "wallet"), "payments") ? offering(state, me, others, draw) : null;
    default:
      return null;
  }
};

export default bot;
