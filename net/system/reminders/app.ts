import type { Ctx, Schema } from "../../sdk/system.ts";
import { app, notice, sliced } from "../../sdk/system.ts";
import bot from "./bot.ts";

/* TYPES */

export type Item = { id: number; text: string; done: boolean; by: string; at: number };

export type List = { id: number; owner: string; title: string; shared: string[]; items: Item[] };

export type Todo = { people: string[] | null; next: number; lists: Record<string, List> };

export const TITLE = 60;

export const TEXT = 200;

export const LISTS = 20;

export const ITEMS = 100;

const ID = 40;

/* LISTS */

export const can = (list: List, id: string) => list.owner === id || list.shared.includes(id);

const listOf = (state: Todo, id: number, refuse: Ctx["refuse"]): List =>
  state.lists[id] ?? refuse(`no list ${id}; it was deleted`);

const open = (state: Todo, id: number, me: string, refuse: Ctx["refuse"]): List => {
  const list = listOf(state, id, refuse);
  return can(list, me) ? list : refuse(`list ${id} is ${list.owner}'s; ask them to share it`);
};

const owned = (state: Todo, id: number, me: string, refuse: Ctx["refuse"]): List => {
  const list = listOf(state, id, refuse);
  return list.owner === me ? list : refuse(`list ${id} is ${list.owner}'s; only its owner shares or deletes it`);
};

const holding = (state: Todo, item: number, me: string, refuse: Ctx["refuse"]): List => {
  const list = Object.values(state.lists).find((one) => one.items.some((each) => each.id === item));
  if (!list) return refuse(`no item ${item}; it was removed`);
  return can(list, me) ? list : refuse(`item ${item} is on ${list.owner}'s list ${list.id}; ask them to share it`);
};

const put = (state: Todo, list: List | null, id: number): Todo => {
  const lists = { ...state.lists };
  if (list) lists[id] = list;
  else delete lists[id];
  return { ...state, lists };
};

const mine = (state: Todo, id: string) => ({
  lists: Object.values(state.lists)
    .filter((list) => can(list, id))
    .map((list) => ({ ...list, open: list.items.filter((item) => !item.done).length })),
});

/* SCHEMAS */

const person = (state: Todo): Schema =>
  state.people ? { enum: state.people } : { type: "string", minLength: 1, maxLength: ID };

const number = (state: Todo): Schema => ({ type: "integer", minimum: 1, maximum: Math.max(1, state.next - 1) });

const shape = (properties: Record<string, Schema>): Schema => ({
  type: "object",
  properties,
  required: Object.keys(properties),
  additionalProperties: false,
});

/* APP */

export default app<Todo>({
  id: "reminders",
  title: "Reminders",
  line: "To-do lists per user, each item ticked when done; an owner shares a list with others.",
  boot: ({ actors }) => ({ people: actors, next: 1, lists: {} }),
  verbs: {
    make: {
      line: `Make a list: a title of 1 to ${TITLE} characters; you own at most ${LISTS}.`,
      args: shape({ title: { type: "string", minLength: 1, maxLength: TITLE } }),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const title = String(args.title);
        if (!title.trim()) return ctx.refuse("the title is only spaces; name the list");
        if (Object.values(state.lists).filter((list) => list.owner === me).length >= LISTS)
          return ctx.refuse(`you own ${LISTS} lists; delete one first`);
        const list: List = { id: state.next, owner: me, title, shared: [], items: [] };
        return { ...put(state, list, list.id), next: state.next + 1 };
      },
    },
    add: {
      line: `Add an item of 1 to ${TEXT} characters to a list you own or share; a list holds at most ${ITEMS}.`,
      args: (state) => shape({ list: number(state), text: { type: "string", minLength: 1, maxLength: TEXT } }),
      run: (state, args, ctx) => {
        const text = String(args.text);
        const list = open(state, Number(args.list), ctx.who.id, ctx.refuse);
        if (!text.trim()) return ctx.refuse("the item is only spaces; say what to do");
        if (list.items.length >= ITEMS) return ctx.refuse(`list ${list.id} holds ${ITEMS} items; remove one first`);
        const item: Item = { id: state.next, text, done: false, by: ctx.who.id, at: ctx.at };
        return { ...put(state, { ...list, items: [...list.items, item] }, list.id), next: state.next + 1 };
      },
    },
    tick: {
      line: "Mark an item done, or not done, on a list you own or share.",
      args: (state) => shape({ item: number(state), done: { type: "boolean" } }),
      run: (state, args, ctx) => {
        const item = Number(args.item);
        const list = holding(state, item, ctx.who.id, ctx.refuse);
        const items = list.items.map((one) => (one.id === item ? { ...one, done: Boolean(args.done) } : one));
        return put(state, { ...list, items }, list.id);
      },
    },
    remove: {
      line: "Remove an item from a list you own or share.",
      args: (state) => shape({ item: number(state) }),
      run: (state, args, ctx) => {
        const item = Number(args.item);
        const list = holding(state, item, ctx.who.id, ctx.refuse);
        return put(state, { ...list, items: list.items.filter((one) => one.id !== item) }, list.id);
      },
    },
    share: {
      line: "Share a list of yours with someone, who may then add, tick and remove its items.",
      args: (state) => shape({ list: number(state), with: person(state) }),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const to = String(args.with);
        const list = owned(state, Number(args.list), me, ctx.refuse);
        if (to === me) return ctx.refuse("you own the list; share it with someone else");
        if (list.shared.includes(to)) return ctx.refuse(`list ${list.id} is already shared with ${to}`);
        ctx.wish(notice(to, `Shared list: ${list.title}`));
        return put(state, { ...list, shared: [...list.shared, to] }, list.id);
      },
    },
    unshare: {
      line: "Stop sharing a list of yours with someone, or leave a list shared with you.",
      args: (state) => shape({ list: number(state), with: person(state) }),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const to = String(args.with);
        const list = listOf(state, Number(args.list), ctx.refuse);
        if (list.owner !== me && to !== me)
          return ctx.refuse(`list ${list.id} is ${list.owner}'s; you may only leave it yourself`);
        if (!list.shared.includes(to)) return ctx.refuse(`list ${list.id} is not shared with ${to}`);
        return put(state, { ...list, shared: list.shared.filter((one) => one !== to) }, list.id);
      },
    },
    delete: {
      line: "Delete a list of yours with its items.",
      args: (state) => shape({ list: number(state) }),
      run: (state, args, ctx) => {
        const list = owned(state, Number(args.list), ctx.who.id, ctx.refuse);
        return put(state, null, list.id);
      },
    },
  },
  moves: (state) =>
    Object.keys(state.lists).length ? ["make", "add", "tick", "remove", "share", "unshare", "delete"] : ["make"],
  read: sliced(mine),
  bot,
});
