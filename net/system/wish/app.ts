import type { Schema } from "../../sdk/system.ts";
import { app, sliced } from "../../sdk/system.ts";
import { bot } from "./bot.ts";

/* TYPES */

export type Wish = { n: number; by: string; text: string; at: number; backers: string[]; withdrawn: number | null };

export type Wishes = { wishes: Record<string, Wish>; n: number };

export const TEXT = 280;

export const OPEN = 5;

/* WISHES */

const live = (one: Wish) => one.withdrawn === null;

const openBy = (state: Wishes, id: string) => Object.values(state.wishes).filter((one) => live(one) && one.by === id);

const mine = (state: Wishes, id: string) => ({
  posted: openBy(state, id).map((one) => one.n),
  backing: Object.values(state.wishes)
    .filter((one) => live(one) && one.backers.includes(id))
    .map((one) => one.n),
});

/* SCHEMAS */

const posting: Schema = {
  type: "object",
  properties: { text: { type: "string", minLength: 1, maxLength: TEXT } },
  required: ["text"],
  additionalProperties: false,
};

const naming = (state: Wishes): Schema => ({
  type: "object",
  properties: { wish: { type: "integer", minimum: 1, maximum: Math.max(1, state.n) } },
  required: ["wish"],
  additionalProperties: false,
});

/* APP */

export default app<Wishes>({
  id: "wish",
  title: "Wish",
  line: "Public requests for this world to change or build something; others back them.",
  boot: () => ({ wishes: {}, n: 0 }),
  verbs: {
    post: {
      line: `Ask for a change or a new thing: 1 to ${TEXT} characters; up to ${OPEN} open wishes each.`,
      args: posting,
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const text = String(args.text).trim();
        if (!text) return ctx.refuse("the wish is only spaces; say what you want");
        if (openBy(state, me).length >= OPEN) return ctx.refuse(`you have ${OPEN} open wishes; withdraw one first`);
        const same = Object.values(state.wishes).find(
          (one) => live(one) && one.text.toLowerCase() === text.toLowerCase(),
        );
        if (same) return ctx.refuse(`wish ${same.n} already asks this; back it instead`);
        const n = state.n + 1;
        return { n, wishes: { ...state.wishes, [n]: { n, by: me, text, at: ctx.at, backers: [], withdrawn: null } } };
      },
    },
    back: {
      line: "Back someone else's open wish, once.",
      args: naming,
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const one = state.wishes[String(args.wish)]!;
        if (!live(one)) return ctx.refuse(`wish ${one.n} was withdrawn`);
        if (one.by === me) return ctx.refuse(`you posted wish ${one.n}; others back it`);
        if (one.backers.includes(me)) return ctx.refuse(`you already back wish ${one.n}`);
        return { ...state, wishes: { ...state.wishes, [one.n]: { ...one, backers: [...one.backers, me] } } };
      },
    },
    withdraw: {
      line: "Withdraw a wish you posted.",
      args: naming,
      run: (state, args, ctx) => {
        const one = state.wishes[String(args.wish)]!;
        if (one.by !== ctx.who.id) return ctx.refuse(`only ${one.by} withdraws wish ${one.n}`);
        if (!live(one)) return ctx.refuse(`wish ${one.n} is already withdrawn`);
        return { ...state, wishes: { ...state.wishes, [one.n]: { ...one, withdrawn: ctx.at } } };
      },
    },
  },
  moves: (state) => (state.n ? ["post", "back", "withdraw"] : ["post"]),
  read: sliced(mine),
  bot,
});
