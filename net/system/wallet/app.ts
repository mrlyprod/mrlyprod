import type { Ctx, Obj, Schema } from "../../sdk/system.ts";
import { app, notice, sliced } from "../../sdk/system.ts";
import { bot } from "./bot.ts";

/* TYPES */

export type Kind = "grant" | "pay" | "gift";

export type Move = { n: number; at: number; kind: Kind; from: string | null; to: string; amount: number; note: string };

export type Transfer = { kind: Kind; from: string; to: string; amount: number; at: number; note: string };

export type Wallet = {
  people: string[] | null;
  causes: Record<string, string>;
  balances: Record<string, number>;
  raised: Record<string, number>;
  history: Record<string, Move[]>;
  gifts: Move[];
  supply: number;
  n: number;
};

export const MOST = 1_000_000_000;

export const NOTE = 140;

export const KEEP = 50;

export const LEDGER = 100;

const ID = 40;

const CAUSE = /^[a-z][a-z0-9-]{0,39}$/;

const RESERVE = /^[a-z][a-z0-9-]{0,39}$/;

/* RULES */

const SWITCHES = ["payments", "gifts"] as const;

const on = (rules: Obj, rule: (typeof SWITCHES)[number]) => rules[rule] === "on";

function checked(rules: Obj): void {
  for (const rule of SWITCHES) {
    const value = rules[rule] ?? "off";
    if (value !== "on" && value !== "off")
      throw new Error(`wallet: the ${rule} rule is "on" or "off", not ${JSON.stringify(value)}`);
  }
}

function grantOf(rules: Obj): number {
  const grant = rules.grant ?? 0;
  if (typeof grant !== "number" || !Number.isInteger(grant) || grant < 0 || grant > MOST)
    throw new Error(`wallet: the grant rule is a whole number from 0 to ${MOST}, not ${JSON.stringify(grant)}`);
  return grant;
}

/* CONFIG */

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

/* MOVES */

const granting = (n: number, to: string, amount: number): Move => ({
  n,
  at: 0,
  kind: "grant",
  from: null,
  to,
  amount,
  note: "",
});

const held = (state: Wallet, id: string) => state.balances[id] ?? 0;

const logged = (state: Wallet, id: string, move: Move) => [...(state.history[id] ?? []), move].slice(-KEEP);

export function move(state: Wallet, { kind, from, to, amount, at, note }: Transfer): Wallet | string {
  if (!Number.isInteger(amount) || amount < 1 || amount > MOST)
    return `an amount is a whole number from 1 to ${MOST}, not ${amount}`;
  if (from === to) return `${from} cannot pay themselves`;
  if (amount > held(state, from)) return `${from} holds ${held(state, from)}, less than ${amount}`;
  const n = state.n + 1;
  const one: Move = { n, at, kind, from, to, amount, note: note.slice(0, NOTE) };
  const cause = to in state.causes;
  const balances = { ...state.balances, [from]: held(state, from) - amount };
  return {
    ...state,
    n,
    balances: cause ? balances : { ...balances, [to]: held(state, to) + amount },
    raised: cause ? { ...state.raised, [to]: (state.raised[to] ?? 0) + amount } : state.raised,
    history: {
      ...state.history,
      [from]: logged(state, from, one),
      ...(cause ? {} : { [to]: logged(state, to, one) }),
    },
    gifts: kind === "gift" ? [...state.gifts, one].slice(-LEDGER) : state.gifts,
  };
}

function moved(state: Wallet, ctx: Ctx, kind: Kind, to: string, amount: number, note: string): Wallet {
  const from = ctx.who.id;
  if (amount > held(state, from)) ctx.refuse(`you hold ${held(state, from)}, less than ${amount}`);
  if (!(to in state.causes))
    ctx.wish(notice(to, `${from} ${kind === "gift" ? "gave" : "paid"} you ${amount}${note ? `: ${note}` : ""}`));
  const next = move(state, { kind, from, to, amount, at: ctx.at, note });
  return typeof next === "string" ? ctx.refuse(next) : next;
}

const closed = (state: Wallet, rule: (typeof SWITCHES)[number]) => {
  const causes = Object.keys(state.causes);
  return `${rule} between people are off in this world${causes.length ? `; give to a cause: ${causes.join(", ")}` : ""}`;
};

/* SCHEMAS */

const transfer = (state: Wallet, causes: boolean): Schema => ({
  type: "object",
  properties: {
    to: state.people
      ? { enum: [...state.people, ...(causes ? Object.keys(state.causes) : [])] }
      : { type: "string", minLength: 1, maxLength: ID },
    amount: { type: "integer", minimum: 1, maximum: MOST },
    note: { type: "string", maxLength: NOTE },
  },
  required: ["to", "amount"],
  additionalProperties: false,
});

/* APP */

export default app<Wallet>({
  id: "wallet",
  title: "Wallet",
  line: "One wallet each, one currency in whole units: a balance, a history, payments, gifts and the paid acts of every app.",
  boot: ({ config, actors, rules }) => {
    checked(rules);
    const grant = grantOf(rules);
    const people = actors ?? [];
    const granted = grant > 0 ? people : [];
    const causes = causesOf(config, actors);
    const reserves = reservesOf(config, actors, causes);
    return {
      people: actors,
      causes,
      balances: { ...Object.fromEntries(people.map((id) => [id, grant])), ...reserves },
      raised: {},
      history: Object.fromEntries(granted.map((id, i) => [id, [granting(i + 1, id, grant)]])),
      gifts: [],
      supply: grant * people.length + Object.values(reserves).reduce((sum, one) => sum + one, 0),
      n: granted.length,
    };
  },
  verbs: {
    send: {
      line: `Pay a person from your wallet: a whole amount you hold, a note of up to ${NOTE} characters.`,
      args: (state) => transfer(state, false),
      run: (state, args, ctx) => {
        const to = String(args.to);
        if (!on(ctx.rules, "payments")) return ctx.refuse(closed(state, "payments"));
        if (to === ctx.who.id) return ctx.refuse("you cannot pay yourself");
        if (to in state.causes) return ctx.refuse(`${to} is a cause; give to it with wallet.give`);
        return moved(state, ctx, "pay", to, Number(args.amount), String(args.note ?? ""));
      },
    },
    give: {
      line: "Give to a person or a cause, never to yourself; every gift enters the public ledger.",
      args: (state) => transfer(state, true),
      run: (state, args, ctx) => {
        const to = String(args.to);
        if (to === ctx.who.id) return ctx.refuse("a gift goes to someone else; nobody gives to themselves");
        if (!(to in state.causes) && !on(ctx.rules, "gifts")) return ctx.refuse(closed(state, "gifts"));
        return moved(state, ctx, "gift", to, Number(args.amount), String(args.note ?? ""));
      },
    },
  },
  read: sliced((state, id) => ({ balance: held(state, id), history: state.history[id] ?? [] })),
  money: {
    move: (state, { from, to, amount, at, verb }) => move(state, { kind: "pay", from, to, amount, at, note: verb }),
  },
  bot,
});
