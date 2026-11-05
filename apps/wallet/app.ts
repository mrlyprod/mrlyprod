import type { Ctx, Json, Knob, Knobs, Obj, Schema, Who } from "@mrlyprod/os/contract/index.ts";
import { app, sliced } from "@mrlyprod/os/contract/index.ts";
import { blockedBy, notice, reach, someone } from "../shared/people.ts";

export type Kind = "grant" | "claim" | "pay" | "gift";

export type Move = { n: number; at: number; kind: Kind; from: string | null; to: string; amount: number; note: string };

export type Transfer = { kind: Kind; from: string; to: string; amount: number; at: number; note: string };

export type Status = "open" | "paid" | "declined" | "cancelled";

export type Bill = {
  n: number;
  by: string;
  to: string;
  amount: number;
  note: string;
  at: number;
  status: Status;
  done: number | null;
};

export type Dials = { keep: number; ledger: number; note: number; baseline: number; floor: number };

export type Wallet = {
  people: string[] | null;
  causes: Record<string, string>;
  balances: Record<string, number>;
  raised: Record<string, number>;
  history: Record<string, Move[]>;
  gifts: Move[];
  supply: number;
  n: number;
  dials: Dials;
  bills: Record<string, Bill>;
  billed: number;
  claims: Record<string, number> | null;
};

export const MOST = 1_000_000_000;

export const DAY = 24 * 60 * 60 * 1000;

const SHOWN = 5;

const ID = 40;

const KNOBS = {
  keep: {
    line: "Moves one person's history keeps.",
    schema: { type: "integer", minimum: 5, maximum: 5000 },
    reality: 50,
  },
  ledger: {
    line: "Gifts the public ledger keeps.",
    schema: { type: "integer", minimum: 5, maximum: 5000 },
    reality: 100,
  },
  note: {
    line: "Characters a note on a payment, gift or request holds.",
    schema: { type: "integer", minimum: 10, maximum: 2000 },
    reality: 140,
  },
  asks: {
    line: "Requests one person may have open at once.",
    schema: { type: "integer", minimum: 1, maximum: 1000 },
    reality: 20,
  },
  baseline: {
    line: "Coins any person may claim each day, for nothing.",
    schema: { type: "integer", minimum: 0, maximum: 100000 },
    reality: 0,
  },
  work: {
    line: "If nobody has to work to eat, what is a job? earn: coins come only from others. free: anyone may claim back up to the grant each day.",
    schema: { enum: ["earn", "free"] },
    reality: "earn",
  },
  giving: {
    line: "Does giving beat selling? both: the payments and gifts knobs decide. gift: no payments or requests between people. price: no gifts between people.",
    schema: { enum: ["both", "gift", "price"] },
    reality: "both",
  },
  payments: {
    line: "Whether people may pay each other, in the wallet and in every app that sells.",
    schema: { enum: ["on", "off"] },
    reality: "off",
  },
  gifts: {
    line: "Whether people may give each other coins and things for nothing.",
    schema: { enum: ["on", "off"] },
    reality: "off",
  },
  grant: {
    line: "Coins each person holds at boot.",
    schema: { type: "integer", minimum: 0, maximum: 1_000_000_000 },
    reality: 0,
  },
} satisfies Knobs;

const CAUSE = /^[a-z][a-z0-9-]{0,39}$/;

const RESERVE = /^[a-z][a-z0-9-]{0,39}$/;

const SWITCHES = ["payments", "gifts"] as const;

type Switch = (typeof SWITCHES)[number];

const BARRED: Record<Switch, string> = { payments: "gift", gifts: "price" };

export const allowed = (knob: Knob, rule: Switch) =>
  knob<string>(`wallet.${rule}`) === "on" && knob<string>("wallet.giving") !== BARRED[rule];

export const pay = (ctx: Ctx, to: string, amount: number, from?: string): Json | null =>
  ctx.call("wallet.move", { to, amount, ...(from ? { from } : {}) });

function causesOf(config: Obj, actors: string[] | null): Record<string, string> {
  const causes = config.causes ?? {};
  if (!causes || typeof causes !== "object" || Array.isArray(causes))
    throw new Error("wallet: causes must map an id to a name");
  for (const [id, name] of Object.entries(causes)) {
    if (!CAUSE.test(id)) throw new Error(`wallet: cause ${JSON.stringify(id)} must be lowercase letters, digits, -`);
    if (typeof name !== "string" || !name.trim()) throw new Error(`wallet: cause ${id} needs a name`);
    if (actors?.includes(id)) throw new Error(`wallet: cause ${id} shares its id with a person`);
  }
  return causes as Record<string, string>;
}

function reservesOf(config: Obj, actors: string[] | null, causes: Record<string, string>): Record<string, number> {
  const reserves = config.reserves ?? {};
  if (!reserves || typeof reserves !== "object" || Array.isArray(reserves))
    throw new Error("wallet: reserves must map an id to a whole amount");
  for (const [id, amount] of Object.entries(reserves)) {
    if (!RESERVE.test(id))
      throw new Error(`wallet: reserve ${JSON.stringify(id)} must be lowercase letters, digits, -`);
    if (typeof amount !== "number" || !Number.isInteger(amount) || amount < 0 || amount > MOST)
      throw new Error(`wallet: reserve ${id} holds a whole number from 0 to ${MOST}, not ${JSON.stringify(amount)}`);
    if (actors?.includes(id)) throw new Error(`wallet: reserve ${id} shares its id with a person`);
    if (id in causes) throw new Error(`wallet: reserve ${id} shares its id with a cause`);
  }
  return reserves as Record<string, number>;
}

const granting = (n: number, to: string, amount: number): Move => ({
  n,
  at: 0,
  kind: "grant",
  from: null,
  to,
  amount,
  note: "",
});

const held = (state: Wallet, id: string) => (Object.hasOwn(state.balances, id) ? state.balances[id]! : 0);

const historyOf = (state: Wallet, id: string) => (Object.hasOwn(state.history, id) ? state.history[id]! : []);

const logged = (state: Wallet, id: string, move: Move) => [...historyOf(state, id), move].slice(-state.dials.keep);

const isCause = (state: Wallet, id: string) => Object.hasOwn(state.causes, id);

export function move(state: Wallet, { kind, from, to, amount, at, note }: Transfer): Wallet | string {
  if (!Number.isInteger(amount) || amount < 1 || amount > MOST)
    return `an amount is a whole number from 1 to ${MOST}, not ${amount}`;
  if (from === to) return `${from} cannot pay themselves`;
  if (amount > held(state, from)) return `${from} holds ${held(state, from)}, less than ${amount}`;
  const n = state.n + 1;
  const one: Move = { n, at, kind, from, to, amount, note: note.slice(0, state.dials.note) };
  const cause = isCause(state, to);
  const balances = { ...state.balances, [from]: held(state, from) - amount };
  return {
    ...state,
    n,
    balances: cause ? balances : { ...balances, [to]: held(state, to) + amount },
    raised: cause
      ? { ...state.raised, [to]: (Object.hasOwn(state.raised, to) ? state.raised[to]! : 0) + amount }
      : state.raised,
    history: {
      ...state.history,
      [from]: logged(state, from, one),
      ...(cause ? {} : { [to]: logged(state, to, one) }),
    },
    gifts: kind === "gift" ? [...state.gifts, one].slice(-state.dials.ledger) : state.gifts,
  };
}

const notTaking = (to: string) => `${to} is not taking this from you`;

function moved(state: Wallet, ctx: Ctx, kind: Kind, to: string, amount: number, note: string): Wallet {
  const from = ctx.who.id;
  const cause = isCause(state, to);
  if (!cause && blockedBy(ctx, to, from)) ctx.refuse(notTaking(to));
  if (amount > held(state, from)) ctx.refuse(`you hold ${held(state, from)}, less than ${amount}`);
  if (!cause)
    ctx.wish(notice(to, `${from} ${kind === "gift" ? "gave" : "paid"} you ${amount}${note ? `: ${note}` : ""}`));
  const next = move(state, { kind, from, to, amount, at: ctx.at, note });
  if (typeof next === "string") return ctx.refuse(next);
  ctx.say({
    note: `${kind === "gift" ? "Gave" : "Paid"} ${to} ${amount}; you hold ${held(next, from)}`,
    id: next.n,
    to,
    balance: held(next, from),
  });
  return next;
}

const unblocked = (ctx: Ctx, to: string) => {
  if (blockedBy(ctx, ctx.who.id, to)) ctx.refuse(`you blocked ${to}; unblock first`);
};

const closed = (state: Wallet, rule: Switch) => {
  const causes = Object.keys(state.causes);
  return `${rule} between people are off in this world${causes.length ? `; give to a cause: ${causes.join(", ")}` : ""}`;
};

const asking = (one: Bill) => one.status === "open";

function billOf(state: Wallet, args: Obj, ctx: Ctx): Bill {
  const n = Number(args.request);
  const one = state.bills[n];
  if (!one || ![one.by, one.to].includes(ctx.who.id))
    return ctx.refuse(`no request ${n} of yours; read wallet/requests to see yours`);
  if (!asking(one)) return ctx.refuse(`request ${n} is ${one.status} already`);
  return one;
}

function settled(state: Wallet, one: Bill, status: Status, at: number): Wallet {
  const bills: Record<string, Bill> = { ...state.bills, [one.n]: { ...one, status, done: at } };
  const old = Object.values(bills).filter((bill) => !asking(bill));
  for (const bill of old.slice(0, Math.max(0, old.length - state.dials.keep))) delete bills[bill.n];
  return { ...state, bills };
}

const dayOf = (at: number) => Math.floor(at / DAY);

export const claimable = (state: Wallet, id: string) =>
  Math.max(state.dials.baseline, state.dials.floor - held(state, id));

const transfer = (knob: Knob): Schema => ({
  type: "object",
  properties: {
    to: { type: "string", minLength: 1, maxLength: ID },
    amount: { type: "integer", minimum: 1, maximum: MOST },
    note: { type: "string", maxLength: knob("note") },
  },
  required: ["to", "amount"],
  additionalProperties: false,
});

const toward = (state: Wallet, args: Obj, ctx: Ctx, causes: boolean) =>
  someone(state.people, String(args.to), ctx, causes ? Object.keys(state.causes) : []);

const moving: Schema = {
  type: "object",
  properties: {
    from: { type: "string", minLength: 1, maxLength: ID },
    to: { type: "string", minLength: 1, maxLength: ID },
    amount: { type: "integer", minimum: 1, maximum: MOST },
    note: { type: "string", maxLength: ID },
  },
  required: ["to", "amount"],
  additionalProperties: false,
};

const billing: Schema = {
  type: "object",
  properties: { request: { type: "integer", minimum: 1 } },
  required: ["request"],
  additionalProperties: false,
};

const ledger = (state: Wallet) => ({
  causes: state.causes,
  raised: state.raised,
  gifts: state.gifts,
  supply: state.supply,
});

const requests = (state: Wallet, id: string) => {
  const bills = Object.values(state.bills).filter((one) => one.by === id || one.to === id);
  const live = bills.filter(asking);
  return {
    owed: live.filter((one) => one.to === id),
    asked: live.filter((one) => one.by === id),
    closed: bills.filter((one) => !asking(one)),
  };
};

const mine = (state: Wallet, id: string, _who: Who | null, at: number) => ({
  balance: held(state, id),
  history: historyOf(state, id),
  requests: requests(state, id),
  ...(state.claims ? { claim: { amount: claimable(state, id), ready: state.claims[id] !== dayOf(at) } } : {}),
  ...ledger(state),
});

function whole(state: Wallet, who: Who | null, at: number) {
  if (!who) return ledger(state);
  const full = mine(state, who.id, who, at);
  return {
    ...full,
    history: full.history.slice(-SHOWN),
    requests: { owed: full.requests.owed.slice(0, SHOWN), asked: full.requests.asked.slice(0, SHOWN) },
    gifts: full.gifts.slice(-SHOWN),
    more: "the latest of each; wallet/@me holds all of it",
  };
}

export default app<Wallet>({
  id: "wallet",
  title: "Wallet",
  line: "One wallet each, one currency in whole units: a private balance and history, payments, requests, public gifts and the paid acts of every app.",
  boot: ({ config, actors, knob }) => {
    const grant = knob("grant");
    const people = actors ?? [];
    const granted = grant > 0 ? people : [];
    const causes = causesOf(config, actors);
    const reserves = reservesOf(config, actors, causes);
    const baseline = knob("baseline");
    const floor = knob<string>("work") === "free" ? grant : 0;
    return {
      people: actors,
      causes,
      balances: { ...Object.fromEntries(people.map((id) => [id, grant])), ...reserves },
      raised: {},
      history: Object.fromEntries(granted.map((id, i) => [id, [granting(i + 1, id, grant)]])),
      gifts: [],
      supply: grant * people.length + Object.values(reserves).reduce((sum, one) => sum + one, 0),
      n: granted.length,
      dials: { keep: knob("keep"), ledger: knob("ledger"), note: knob("note"), baseline, floor },
      bills: {},
      billed: 0,
      claims: baseline > 0 || floor > 0 ? {} : null,
    };
  },
  verbs: {
    move: {
      inner: true,
      line: "For other apps only: move coins to or from the one who acts, inside that app's act.",
      args: moving,
      run: (state, args, ctx) => {
        const from = String(args.from ?? ctx.who.id);
        const to = String(args.to);
        const amount = Number(args.amount);
        if (from !== ctx.who.id && to !== ctx.who.id)
          return ctx.refuse(`${ctx.who.id} is on neither side of this payment`);
        const next = move(state, { kind: "pay", from, to, amount, at: ctx.at, note: String(args.note ?? ctx.via) });
        if (typeof next === "string") return ctx.refuse(next);
        ctx.say({ from, to, amount, balance: held(next, ctx.who.id) });
        return next;
      },
    },
    send: {
      line: (knob) =>
        `Pay a person from your wallet, privately: a whole amount you hold, a note of up to ${knob("note")} characters.`,
      args: transfer,
      run: (state, args, ctx) => {
        const to = toward(state, args, ctx, false);
        if (!allowed(ctx.knob, "payments")) return ctx.refuse(closed(state, "payments"));
        if (to === ctx.who.id) return ctx.refuse("you cannot pay yourself");
        if (isCause(state, to)) return ctx.refuse(`${to} is a cause; give to it with wallet.give`);
        unblocked(ctx, to);
        return moved(state, ctx, "pay", to, Number(args.amount), String(args.note ?? ""));
      },
    },
    give: {
      line: "Give to a person or a cause, never to yourself; every gift enters the public ledger.",
      args: transfer,
      run: (state, args, ctx) => {
        const to = toward(state, args, ctx, true);
        if (to === ctx.who.id) return ctx.refuse("a gift goes to someone else; nobody gives to themselves");
        if (!isCause(state, to) && !allowed(ctx.knob, "gifts")) return ctx.refuse(closed(state, "gifts"));
        if (!isCause(state, to)) unblocked(ctx, to);
        return moved(state, ctx, "gift", to, Number(args.amount), String(args.note ?? ""));
      },
    },
    request: {
      line: (knob) =>
        `Ask a person to pay you: an amount, a note; they get a notice and pay or decline. At most ${knob("asks")} open at once.`,
      args: transfer,
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const to = toward(state, args, ctx, false);
        const amount = Number(args.amount);
        const note = String(args.note ?? "");
        const most = ctx.knob("asks");
        if (!allowed(ctx.knob, "payments")) return ctx.refuse(closed(state, "payments"));
        if (to === me) return ctx.refuse("you cannot ask yourself; ask someone else");
        if (isCause(state, to)) return ctx.refuse(`${to} is a cause; causes take gifts and owe nothing`);
        reach(ctx, to);
        if (amount > state.supply) return ctx.refuse(`only ${state.supply} coins exist; ask for less than ${amount}`);
        if (requests(state, me).asked.length >= most)
          return ctx.refuse(`your open requests are at the limit of ${most}; wallet.cancel one first`);
        const n = state.billed + 1;
        ctx.wish(
          notice(
            to,
            `${me} asks you for ${amount}, request ${n}: wallet.pay or wallet.decline${note ? `; ${note}` : ""}`,
          ),
        );
        const bill: Bill = { n, by: me, to, amount, note, at: ctx.at, status: "open", done: null };
        ctx.say({ note: `Asked ${to} for ${amount}; request ${n}`, id: n, to });
        return { ...state, billed: n, bills: { ...state.bills, [n]: bill } };
      },
    },
    pay: {
      line: "Pay a request someone sent you, in full, from your wallet; they are told.",
      args: billing,
      run: (state, args, ctx) => {
        const one = billOf(state, args, ctx);
        if (one.to !== ctx.who.id) return ctx.refuse(`request ${one.n} is yours; ${one.to} pays it, or you cancel it`);
        const note = `request ${one.n}${one.note ? `: ${one.note}` : ""}`;
        const next = moved(state, ctx, "pay", one.by, one.amount, note);
        ctx.say({
          note: `Paid request ${one.n}: ${one.amount} to ${one.by}; you hold ${held(next, ctx.who.id)}`,
          id: one.n,
          balance: held(next, ctx.who.id),
        });
        return settled(next, one, "paid", ctx.at);
      },
    },
    decline: {
      line: "Turn down a request someone sent you; they are told.",
      args: billing,
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const one = billOf(state, args, ctx);
        if (one.to !== me) return ctx.refuse(`request ${one.n} is yours; cancel it instead`);
        ctx.wish(notice(one.by, `${me} declined your request ${one.n} for ${one.amount}`));
        ctx.say({ note: `Declined request ${one.n}`, id: one.n });
        return settled(state, one, "declined", ctx.at);
      },
    },
    cancel: {
      line: "Take back a request you sent; they are told.",
      args: billing,
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const one = billOf(state, args, ctx);
        if (one.by !== me) return ctx.refuse(`request ${one.n} was sent to you; decline it instead`);
        ctx.wish(notice(one.to, `${me} took back request ${one.n} for ${one.amount}`));
        ctx.say({ note: `Took back request ${one.n}`, id: one.n });
        return settled(state, one, "cancelled", ctx.at);
      },
    },
    claim: {
      line: (knob) => {
        const baseline = knob("baseline");
        const free = knob<string>("work") === "free";
        return `Claim your coins for today, once a day${baseline ? `: ${baseline} each` : ""}${free ? "; you are topped back up to your grant" : ""}.`;
      },
      run: (state, _args, ctx) => {
        const me = ctx.who.id;
        const today = dayOf(ctx.at);
        const amount = claimable(state, me);
        if (!state.claims) return ctx.refuse("nobody claims coins in this world; earn them from other people");
        if (state.claims[me] === today)
          return ctx.refuse(`you claimed today; come back at ${new Date((today + 1) * DAY).toISOString()}`);
        if (amount < 1) return ctx.refuse("you hold your grant already; nothing to claim today");
        const n = state.n + 1;
        const one: Move = { n, at: ctx.at, kind: "claim", from: null, to: me, amount, note: "" };
        ctx.say({
          note: `Claimed ${amount}; you hold ${held(state, me) + amount}`,
          id: n,
          balance: held(state, me) + amount,
        });
        return {
          ...state,
          n,
          balances: { ...state.balances, [me]: held(state, me) + amount },
          history: { ...state.history, [me]: logged(state, me, one) },
          supply: state.supply + amount,
          claims: { ...state.claims, [me]: today },
        };
      },
    },
  },
  moves: (state) => ["send", "give", "request", "pay", "decline", "cancel", "move", ...(state.claims ? ["claim"] : [])],
  read: sliced(mine, whole),
  knobs: KNOBS,
});
