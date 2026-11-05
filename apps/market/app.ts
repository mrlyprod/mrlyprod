import type { Ctx, Json, Knob, Knobs, Obj, Schema, Who } from "@mrlyprod/os/contract/index.ts";
import { app, below, sliced, stream, walk } from "@mrlyprod/os/contract/index.ts";
import { blockedBy, notice, reach, someone } from "../shared/people.ts";
import { allowed, pay } from "../wallet/app.ts";
import type { Good, Held, Stocked, Traits } from "./goods.ts";
import { alike, NAME, PEOPLE, placed, slugOf } from "./goods.ts";

export const HOUSE = "market";

const ID = "market";

const DAY = 86_400_000;

const HOUR = 3_600_000;

const ABOUT = 200;

const TEXT = 200;

const ITEM = 120;

const PERSON = 40;

const PRICE = 1_000_000;

const NEWEST = 6;

const KNOBS = {
  prices: {
    line: "The market's prices, as a percent of usual.",
    schema: { type: "integer", minimum: 1, maximum: 10000 },
    reality: 100,
  },
  spread: {
    line: "How far a seeded price strays from usual, as a percent.",
    schema: { type: "integer", minimum: 0, maximum: 90 },
    reality: 25,
  },
  shelf: {
    line: "Typical stock of a good on a full shelf; each good draws between half and one and a half of this.",
    schema: { type: "integer", minimum: 1, maximum: 10000 },
    reality: 30,
  },
  restock: {
    line: "Percent of a full shelf that refills per day of world time.",
    schema: { type: "integer", minimum: 0, maximum: 10000 },
    reality: 100,
  },
  sellers: {
    line: "Open listings one person may hold.",
    schema: { type: "integer", minimum: 0, maximum: 1000 },
    reality: 10,
  },
  lot: {
    line: "Most units one act moves.",
    schema: { type: "integer", minimum: 1, maximum: 1000 },
    reality: 20,
  },
  reviews: {
    line: "Reviews a listing keeps.",
    schema: { type: "integer", minimum: 0, maximum: 1000 },
    reality: 20,
  },
  verified: {
    line: "Whether only people who bought a listing may review it.",
    schema: { type: "boolean" },
    reality: true,
  },
  offers: {
    line: "Open offers one person may have made.",
    schema: { type: "integer", minimum: 0, maximum: 1000 },
    reality: 10,
  },
} satisfies Knobs;

export type Dials = { [K in keyof typeof KNOBS]: (typeof KNOBS)[K]["reality"] };

export type Review = { by: string; stars: number; text: string; at: number };

export type Listing = Good & {
  id: string;
  by: string;
  shelf: string;
  about: string;
  price: number;
  stock: number;
  real: number;
  cap: number;
  rate: number;
  at: number;
  sold: number;
  open: boolean;
  reviews: Review[];
  buyers: string[];
  rated: Record<string, number>;
};

export type Offer = Held & { id: number; by: string; to: string; price: number; at: number };

export type Market = {
  dials: Dials;
  front: string | null;
  people: string[] | null;
  listings: Record<string, Listing>;
  own: Record<string, Record<string, Held>>;
  offers: Record<string, Offer>;
  offered: number;
};

const dialed = (knob: Knob) => Object.fromEntries(Object.keys(KNOBS).map((name) => [name, knob(name)])) as Dials;

const gained = (one: Listing, at: number) => Math.floor((Math.max(0, at - one.at) * one.rate) / DAY);

export const stockOf = (one: Listing, at: number) => Math.min(one.cap, one.stock + gained(one, at));

function settled(one: Listing, at: number): Listing {
  const stock = stockOf(one, at);
  if (stock >= one.cap || !one.rate) return { ...one, stock, at };
  return { ...one, stock, at: one.at + Math.ceil((gained(one, at) * DAY) / one.rate) };
}

function shelved(dials: Dials, seed: number, catalog: readonly Stocked[]): Record<string, Listing> {
  const { draw } = stream(seed);
  const listings: Record<string, Listing> = {};
  for (const { shelf, name, price, traits } of catalog) {
    const id = slugOf(name);
    const swing = 100 - dials.spread + below(draw, 2 * dials.spread + 1);
    const cap = Math.max(1, Math.floor(dials.shelf / 2) + below(draw, dials.shelf));
    listings[id] = {
      id,
      by: HOUSE,
      shelf,
      name,
      about: "",
      traits,
      price: Math.max(1, Math.round((price * dials.prices * swing) / 10000)),
      stock: cap,
      real: cap,
      cap,
      rate: Math.ceil((cap * dials.restock) / 100),
      at: 0,
      sold: 0,
      open: true,
      reviews: [],
      buyers: [],
      rated: {},
    };
  }
  return listings;
}

const ownOf = (own: Market["own"], id: string): Record<string, Held> => (Object.hasOwn(own, id) ? own[id]! : {});

const heldOf = (state: Market, id: string, key: string): Held | null => {
  const mine = ownOf(state.own, id);
  return Object.hasOwn(mine, key) ? mine[key]! : null;
};

const listingOf = (state: Market, id: string): Listing | null =>
  Object.hasOwn(state.listings, id) ? state.listings[id]! : null;

function added(own: Market["own"], id: string, good: Good, n: number): Market["own"] {
  const mine = ownOf(own, id);
  const { key, name } = placed(mine, good);
  const had = Object.hasOwn(mine, key) ? mine[key]!.n : 0;
  return { ...own, [id]: { ...mine, [key]: { name, traits: good.traits, n: had + n } } };
}

const landing = (state: Market, id: string, good: Good) => placed(ownOf(state.own, id), good).key;

function taken(own: Market["own"], id: string, key: string, n: number): Market["own"] {
  const mine = { ...ownOf(own, id) };
  const had = mine[key]!;
  if (had.n > n) mine[key] = { ...had, n: had.n - n };
  else delete mine[key];
  return { ...own, [id]: mine };
}

function holding(state: Market, ctx: Ctx, item: Json | undefined, n: number): Held {
  const one = heldOf(state, ctx.who.id, slugOf(String(item)));
  if (!one) return ctx.refuse(`you own no ${String(item)}; market/@me lists what you own`);
  if (one.n < n) return ctx.refuse(`you own ${one.n} ${one.name}, not ${n}`);
  return one;
}

function found(state: Market, name: Json | undefined, ctx: Ctx): Listing {
  const slug = slugOf(String(name));
  if (!slug) return ctx.refuse("name a listing; read market for what is for sale");
  const hit = listingOf(state, slug);
  if (hit) return hit;
  const near = Object.keys(state.listings)
    .filter((id) => id.includes(slug) || slug.includes(id))
    .slice(0, 3);
  return ctx.refuse(
    `no listing ${slug}; ${near.length ? `did you mean ${near.join(", ")}?` : "read market for what is for sale"}`,
  );
}

function soldOut(one: Listing, at: number): string {
  if (!one.rate) return `${one.name} sold out for good; read market for what is left`;
  const wait = Math.max(1, Math.ceil((DAY / one.rate - Math.max(0, at - one.at)) / HOUR));
  return `${one.name} sold out; more in about ${wait} ${wait === 1 ? "hour" : "hours"}, or read market for something else`;
}

function wanted(state: Market, args: Obj, ctx: Ctx): { one: Listing; n: number } {
  const me = ctx.who.id;
  const one = found(state, args.item, ctx);
  const n = Number(args.n ?? 1);
  if (!one.open) return ctx.refuse(`${one.id} is no longer for sale; read market for what is`);
  if (one.by === me) return ctx.refuse(`${one.id} is your own listing; unlist it to take it back`);
  if (one.by !== HOUSE && blockedBy(ctx, one.by, me)) return ctx.refuse(`${one.by} is not taking this from you`);
  const left = stockOf(one, ctx.at);
  if (!left) return ctx.refuse(soldOut(one, ctx.at));
  if (n > left) return ctx.refuse(`only ${left} ${one.name} left; buy ${left} or fewer`);
  return { one, n };
}

const openBy = (state: Market, id: string) => Object.values(state.listings).filter((one) => one.open && one.by === id);

const worth = (one: Good) =>
  Object.entries(one.traits).flatMap(([key, value]) =>
    value === true ? [key] : typeof value === "number" && value ? [`${key} ${value}`] : [],
  );

function sellable(state: Market, ctx: Ctx, name: string, n: number, live: Listing | null): Good {
  const key = slugOf(name);
  if (live && !n) return live;
  const owned = heldOf(state, ctx.who.id, key);
  if (owned && owned.n < n) return ctx.refuse(`you own ${owned.n} ${owned.name}, not ${n}`);
  if (owned && live && !alike(owned.traits, live.traits))
    return ctx.refuse(`your ${owned.name} differs from what ${live.id} sells; unlist ${live.id} first`);
  if (owned) return owned;
  const house = listingOf(state, key);
  if (house) return ctx.refuse(`you own no ${house.name}; buy some first, or name your thing otherwise`);
  if (live && worth(live).length)
    return ctx.refuse(`you own no more ${live.name}; make or buy more before topping up ${live.id}`);
  return live ?? { name, traits: {} };
}

function relisted(
  had: Listing | null,
  id: string,
  me: string,
  good: Good,
  args: Obj,
  n: number,
  real: number,
  at: number,
) {
  const live = had?.open ? had : null;
  const stock = (live?.stock ?? 0) + n;
  const one: Listing = {
    id,
    by: me,
    shelf: PEOPLE,
    name: good.name,
    about: String(args.about ?? had?.about ?? "").trim(),
    traits: good.traits,
    price: Number(args.price),
    stock,
    real: (live?.real ?? 0) + real,
    cap: stock,
    rate: 0,
    at,
    sold: had?.sold ?? 0,
    open: true,
    reviews: had?.reviews ?? [],
    buyers: had?.buyers ?? [],
    rated: had?.rated ?? {},
  };
  return one;
}

const totalOf = (one: Offer) => one.price * one.n;

const offersOf = (state: Market, id: string) => {
  const open = Object.values(state.offers);
  return { toYou: open.filter((one) => one.to === id), fromYou: open.filter((one) => one.by === id) };
};

const ratingOf = (one: Listing) => {
  const stars = Object.values(one.rated);
  return stars.length ? Math.round((10 * stars.reduce((sum, each) => sum + each, 0)) / stars.length) / 10 : null;
};

const backing = (one: Listing): string[] => {
  if (one.by === HOUSE) return [];
  if (!one.real) return ["offer only, no goods behind it"];
  return one.real < one.stock ? [`only ${one.real} of ${one.stock} are goods held`] : [];
};

const rowOf = (one: Listing, at: number) => {
  const rating = ratingOf(one);
  return `${one.id}: ${[
    `${one.price} each`,
    ...worth(one),
    ...backing(one),
    `${stockOf(one, at)} left`,
    ...(one.by === HOUSE ? [] : [`by ${one.by}`]),
    ...(rating === null ? [] : [`${rating} stars`]),
  ].join(", ")}`;
};

const onShelf = (state: Market, shelf: string) =>
  Object.values(state.listings).filter((one) => one.open && one.shelf === shelf);

const shelves = (state: Market) => {
  const names = Object.values(state.listings)
    .filter((one) => one.open)
    .map((one) => one.shelf);
  return Object.fromEntries([...new Set([...names, PEOPLE])].sort().map((name) => [name, onShelf(state, name).length]));
};

const sale = (state: Market, at: number) => ({
  ...(state.front ? { [state.front]: onShelf(state, state.front).map((one) => rowOf(one, at)) } : {}),
  people: onShelf(state, PEOPLE)
    .sort((a, b) => b.at - a.at)
    .slice(0, NEWEST)
    .map((one) => rowOf(one, at)),
  shelves: shelves(state),
  see: "market/shelf/<name> lists a shelf; market/listing/<id> shows one listing with its reviews.",
});

const mine = (state: Market, id: string, _who: Who | null, at: number) => ({
  own: Object.entries(ownOf(state.own, id)).map(([key, one]) => [`${key} x${one.n}`, ...worth(one)].join(", ")),
  selling: openBy(state, id).map((one) => rowOf(one, at)),
  offers: offersOf(state, id),
  ...sale(state, at),
});

const whole = (state: Market, who: Who | null, at: number): Json =>
  who ? mine(state, who.id, who, at) : sale(state, at);

const view = sliced(mine, whole);

const detail = (one: Listing, at: number): Obj => ({
  id: one.id,
  name: one.name,
  by: one.by,
  shelf: one.shelf,
  about: one.about,
  price: one.price,
  left: stockOf(one, at),
  traits: one.traits,
  sold: one.sold,
  open: one.open,
  rating: ratingOf(one),
  reviews: one.reviews,
});

const allOpen = (state: Market, at: number) =>
  Object.fromEntries(
    Object.values(state.listings)
      .filter((one) => one.open)
      .map((one) => [one.id, rowOf(one, at)]),
  );

function read(state: Market, path: string[], who: Who | null, at: number): Json | undefined {
  const [head, one, ...rest] = path;
  if (head === "listing") {
    if (!one) return allOpen(state, at);
    const hit = listingOf(state, slugOf(one));
    return hit ? walk(detail(hit, at), rest) : undefined;
  }
  if (head === "shelf") {
    if (!one) return shelves(state);
    if (!Object.hasOwn(shelves(state), one)) return undefined;
    return walk(
      onShelf(state, one).map((each) => rowOf(each, at)),
      rest,
    );
  }
  if (head === "offers") {
    if (!who) return undefined;
    const open = offersOf(state, who.id);
    if (!one) return open;
    const hit = [...open.toYou, ...open.fromYou].find((each) => String(each.id) === one);
    return hit ? walk(hit, rest) : undefined;
  }
  return view(state, path, who, at);
}

const shaped = (properties: Record<string, Schema>, required: string[]): Schema => ({
  type: "object",
  properties,
  required,
  additionalProperties: false,
});

const itemName: Schema = { type: "string", minLength: 1, maxLength: ITEM };

const units = (knob: Knob): Schema => ({ type: "integer", minimum: 1, maximum: knob("lot") });

const person: Schema = { type: "string", minLength: 1, maxLength: PERSON };

const naming: Schema = shaped({ offer: { type: "integer", minimum: 1 } }, ["offer"]);

const notTaking = (to: string) => `${to} is not taking this from you`;

function offerOf(state: Market, args: Obj, ctx: Ctx): Offer {
  const key = String(args.offer);
  const one = Object.hasOwn(state.offers, key) ? state.offers[key]! : null;
  if (!one || ![one.by, one.to].includes(ctx.who.id))
    return ctx.refuse(`no open offer ${key} of yours; market/offers lists yours`);
  return one;
}

function acceptable(state: Market, args: Obj, ctx: Ctx): Offer {
  const me = ctx.who.id;
  const one = offerOf(state, args, ctx);
  if (one.by === me) return ctx.refuse(`offer ${one.id} is yours; market.decline takes it back`);
  if (blockedBy(ctx, one.by, me)) return ctx.refuse(notTaking(one.by));
  if (blockedBy(ctx, me, one.by)) return ctx.refuse(`you blocked ${one.by}; unblock first, or market.decline`);
  return one;
}

const closing = (state: Market, one: Offer) => {
  const { [String(one.id)]: _, ...offers } = state.offers;
  return offers;
};

const keptReviews = (list: Review[], keep: number) => list.slice(Math.max(0, list.length - keep));

export default app<Market>({
  id: ID,
  title: "Market",
  line: "A ledger of things: goods a world stocks, which run out and restock, listings anyone can post, reviews, and what each person owns.",
  knobs: KNOBS,
  boot: ({ seed, config, knob, actors }) => {
    const dials = dialed(knob);
    const catalog = (config.catalog ?? []) as Stocked[];
    return {
      dials,
      front: catalog[0]?.shelf ?? null,
      people: actors,
      listings: shelved(dials, seed, catalog),
      own: {},
      offers: {},
      offered: 0,
    };
  },
  verbs: {
    buy: {
      line: (knob) =>
        `Buy from a listing, 1 to ${knob("lot")} at a time; your wallet pays in the same act. item is a listing id from read market.`,
      args: (knob) => shaped({ item: itemName, n: units(knob) }, ["item"]),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const { one, n } = wanted(state, args, ctx);
        pay(ctx, one.by, one.price * n);
        const now = settled(one, ctx.at);
        const stock = now.stock - n;
        const person = one.by !== HOUSE;
        if (person)
          ctx.wish(
            notice(one.by, `${me} bought ${n} ${one.name} for ${one.price * n}${stock ? "" : "; that was the last"}`),
          );
        const listing: Listing = {
          ...now,
          stock,
          sold: now.sold + n,
          open: !person || stock > 0,
          buyers: now.buyers.includes(me) ? now.buyers : [...now.buyers, me],
        };
        ctx.say({
          note: `Bought ${n} ${one.name} for ${one.price * n}; ${stock} left`,
          id: landing(state, me, one),
          listing: one.id,
        });
        return {
          ...state,
          listings: { ...state.listings, [one.id]: listing },
          own: added(state.own, me, one, n),
        };
      },
    },
    sell: {
      line: (knob) =>
        `List something for sale at a whole price each: what you own moves onto the shelf with its traits, any other name is something you offer, and goes nowhere if you unlist it. Listing a name again sets its price; with n it also tops it up. Up to ${knob("sellers")} open listings each.`,
      args: (knob) =>
        shaped(
          {
            name: { type: "string", minLength: 1, maxLength: NAME },
            price: { type: "integer", minimum: 1, maximum: PRICE },
            n: units(knob),
            about: { type: "string", maxLength: ABOUT },
          },
          ["name", "price"],
        ),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const asked = String(args.name).trim();
        const named = listingOf(state, slugOf(asked));
        const mineAlready = named && named.by === me ? named : null;
        const name = mineAlready?.name ?? asked;
        const key = slugOf(name);
        if (!key) return ctx.refuse("a name needs letters or digits");
        if (!allowed(ctx.knob, "payments"))
          return ctx.refuse("payments between people are off in this world, so nobody could pay you; use market.give");
        const id = mineAlready?.id ?? slugOf(`${key}-${me}`);
        const had = listingOf(state, id);
        if (had && had.by !== me) return ctx.refuse(`${id} is ${had.by}'s listing; pick another name`);
        const live = had?.open ? had : null;
        const most = state.dials.sellers;
        if (!live && openBy(state, me).length >= most)
          return ctx.refuse(
            most ? `${most} open listings is the most you may hold; unlist one first` : "nobody sells in this world",
          );
        const n = args.n === undefined && live ? 0 : Number(args.n ?? 1);
        const good = sellable(state, ctx, name, n, live);
        const real = n && heldOf(state, me, key) ? n : 0;
        const one = relisted(had, id, me, good, args, n, real, ctx.at);
        ctx.say({
          note: `${
            n
              ? `Listed ${n} ${good.name} at ${one.price} each; ${one.stock} on the shelf`
              : `${id} now costs ${one.price}; ${one.stock} on the shelf`
          }${backing(one).length ? `; buyers see it marked: ${backing(one).join(", ")}` : ""}`,
          id,
        });
        return {
          ...state,
          listings: { ...state.listings, [id]: one },
          own: real ? taken(state.own, me, key, n) : state.own,
        };
      },
    },
    unlist: {
      line: "Close a listing of yours; what is unsold comes back to you.",
      args: shaped({ listing: itemName }, ["listing"]),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const one = found(state, args.listing, ctx);
        if (one.by !== me) return ctx.refuse(`only ${one.by} unlists ${one.id}`);
        if (!one.open) return ctx.refuse(`${one.id} is closed already`);
        const back = Math.min(one.real, one.stock);
        ctx.say({ note: `Closed ${one.id}; ${back} ${one.name} came back to you`, id: one.id, back });
        return {
          ...state,
          listings: { ...state.listings, [one.id]: { ...one, stock: 0, real: 0, cap: 0, open: false } },
          own: back ? added(state.own, me, one, back) : state.own,
        };
      },
    },
    review: {
      line: (knob) =>
        `Review a listing, 1 to 5 stars and up to ${TEXT} characters; a new review replaces your last.${knob<boolean>("verified") ? " Only buyers review." : ""}`,
      args: shaped(
        {
          listing: itemName,
          stars: { type: "integer", minimum: 1, maximum: 5 },
          text: { type: "string", minLength: 1, maxLength: TEXT },
        },
        ["listing", "stars", "text"],
      ),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const one = found(state, args.listing, ctx);
        const stars = Number(args.stars);
        const text = String(args.text).trim();
        if (!text) return ctx.refuse("the review is only spaces; say what you thought");
        if (one.by === me) return ctx.refuse("you cannot review your own listing");
        const person = one.by !== HOUSE;
        if (person && blockedBy(ctx, one.by, me)) return ctx.refuse(`${one.by} is not taking this from you`);
        if (state.dials.verified && !one.buyers.includes(me))
          return ctx.refuse(`you have not bought ${one.id}; buy it first, then review it`);
        if (person) ctx.wish(notice(one.by, `${me} gave ${one.name} ${stars} stars: ${text}`));
        const review: Review = { by: me, stars, text, at: ctx.at };
        const reviews = keptReviews([...one.reviews.filter((each) => each.by !== me), review], state.dials.reviews);
        const listing: Listing = { ...one, reviews, rated: { ...one.rated, [me]: stars } };
        ctx.say({ note: `You gave ${one.name} ${stars} stars`, id: one.id });
        return { ...state, listings: { ...state.listings, [one.id]: listing } };
      },
    },
    give: {
      line: "Hand something you own to someone, for nothing.",
      args: (knob) => shaped({ to: person, item: itemName, n: units(knob) }, ["to", "item"]),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const to = someone(state.people, String(args.to), ctx);
        const n = Number(args.n ?? 1);
        if (to === me) return ctx.refuse("you hold it already; name someone else");
        if (!allowed(ctx.knob, "gifts"))
          return ctx.refuse("gifts between people are off in this world; list it with market.sell");
        reach(ctx, to);
        const one = holding(state, ctx, args.item, n);
        ctx.wish(notice(to, `${me} gave you ${n} ${one.name}`));
        ctx.say({ note: `Gave ${n} ${one.name} to ${to}`, id: slugOf(one.name), to });
        return { ...state, own: added(taken(state.own, me, slugOf(one.name), n), to, one, n) };
      },
    },
    offer: {
      line: (knob) =>
        `Offer someone something you own at a whole price each. It leaves your hands until they market.accept, which swaps goods and coins at once, or market.decline. Up to ${knob("offers")} open offers each.`,
      args: (knob) =>
        shaped({ to: person, item: itemName, n: units(knob), price: { type: "integer", minimum: 1, maximum: PRICE } }, [
          "to",
          "item",
          "price",
        ]),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const to = someone(state.people, String(args.to), ctx);
        const n = Number(args.n ?? 1);
        const price = Number(args.price);
        const most = state.dials.offers;
        if (to === me) return ctx.refuse("you hold it already; name someone else");
        if (!allowed(ctx.knob, "payments"))
          return ctx.refuse("payments between people are off in this world, so nobody could pay you; use market.give");
        reach(ctx, to);
        if (offersOf(state, me).fromYou.length >= most)
          return ctx.refuse(
            most
              ? `${most} open offers is the most you may hold; market.decline one first`
              : "nobody offers in this world",
          );
        const had = holding(state, ctx, args.item, n);
        const id = state.offered + 1;
        const one: Offer = { id, by: me, to, name: had.name, traits: had.traits, n, price, at: ctx.at };
        ctx.wish(
          notice(to, `${me} offers ${n} ${one.name} for ${totalOf(one)}: market.accept or market.decline offer ${id}`),
        );
        ctx.say({ note: `Offered ${n} ${one.name} to ${to} for ${totalOf(one)}`, id, to });
        return {
          ...state,
          offered: id,
          offers: { ...state.offers, [id]: one },
          own: taken(state.own, me, slugOf(had.name), n),
        };
      },
    },
    accept: {
      line: "Accept an offer made to you: your wallet pays and the goods reach you in the same act, or neither happens.",
      args: naming,
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const one = acceptable(state, args, ctx);
        pay(ctx, one.by, totalOf(one));
        ctx.wish(notice(one.by, `${me} accepted offer ${one.id}: ${totalOf(one)} for ${one.n} ${one.name}`));
        ctx.say({
          note: `Bought ${one.n} ${one.name} from ${one.by} for ${totalOf(one)}`,
          id: landing(state, me, one),
          offer: one.id,
        });
        return { ...state, offers: closing(state, one), own: added(state.own, me, one, one.n) };
      },
    },
    decline: {
      line: "Turn down an offer made to you, or take back one you made; the goods go back to whoever offered them.",
      args: naming,
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const one = offerOf(state, args, ctx);
        const mine = one.by === me;
        const other = mine ? one.to : one.by;
        ctx.wish(notice(other, `${me} ${mine ? "took back" : "declined"} offer ${one.id}: ${one.n} ${one.name}`));
        ctx.say({ note: `${mine ? "Took back" : "Declined"} offer ${one.id}`, id: one.id });
        return { ...state, offers: closing(state, one), own: added(state.own, one.by, one, one.n) };
      },
    },
    use: {
      inner: true,
      line: "For other apps only: use up something you own.",
      args: shaped({ item: itemName, n: { type: "integer", minimum: 1 } }, ["item"]),
      run: (state, args, ctx) => {
        const n = Number(args.n ?? 1);
        const one = holding(state, ctx, args.item, n);
        return { ...state, own: taken(state.own, ctx.who.id, slugOf(one.name), n) };
      },
    },
    make: {
      inner: true,
      line: "For other apps only: put a made thing in your hands, with whatever traits that app gives it.",
      args: shaped(
        {
          name: { type: "string", minLength: 1, maxLength: NAME },
          traits: { type: "object" },
          n: { type: "integer", minimum: 1 },
        },
        ["name", "traits"],
      ),
      run: (state, args, ctx) => {
        const name = String(args.name).trim();
        if (!slugOf(name)) return ctx.refuse("a name needs letters or digits");
        const good: Good = { name, traits: args.traits as Traits };
        return { ...state, own: added(state.own, ctx.who.id, good, Number(args.n ?? 1)) };
      },
    },
  },
  read,
});
