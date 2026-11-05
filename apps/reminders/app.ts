import type { Ctx, Json, Knobs, Obj, Schema, Who } from "@mrlyprod/os/contract/index.ts";
import { app, sliced, walk } from "@mrlyprod/os/contract/index.ts";
import { blockedBy, notice, someone } from "../shared/people.ts";

export type Item = {
  id: number;
  text: string;
  done: boolean;
  by: string;
  at: number;
  due: number | null;
  told: boolean;
};

export type List = { id: number; owner: string; title: string; shared: string[]; items: Item[] };

export type Todo = { people: string[] | null; next: number; lists: Record<string, List> };

const KNOBS = {
  title: {
    line: "Characters in a list's title.",
    schema: { type: "integer", minimum: 8, maximum: 500 },
    reality: 60,
  },
  text: {
    line: "Characters in one item.",
    schema: { type: "integer", minimum: 16, maximum: 5000 },
    reality: 200,
  },
  lists: {
    line: "Lists one person owns.",
    schema: { type: "integer", minimum: 1, maximum: 1000 },
    reality: 20,
  },
  items: {
    line: "Items one list holds.",
    schema: { type: "integer", minimum: 1, maximum: 5000 },
    reality: 100,
  },
  shares: {
    line: "People one list is shared with.",
    schema: { type: "integer", minimum: 1, maximum: 4096 },
    reality: 4096,
  },
} satisfies Knobs;

export const LAST = 8_640_000_000_000_000;

const ID = 40;

const PEEK = 3;

const CLIP = 40;

const iso = (at: number) => new Date(Math.min(at, LAST)).toISOString();

export const can = (list: List, id: string) => list.owner === id || list.shared.includes(id);

const open = (state: Todo, id: number, me: string, refuse: Ctx["refuse"]): List => {
  const list = state.lists[id];
  return list && can(list, me)
    ? list
    : refuse(`you have no list ${id}; it was deleted, or its owner has not shared it`);
};

const owned = (state: Todo, id: number, me: string, refuse: Ctx["refuse"]): List => {
  const list = open(state, id, me, refuse);
  return list.owner === me ? list : refuse(`list ${id} is ${list.owner}'s; only its owner shares or deletes it`);
};

const holding = (state: Todo, item: number, me: string, refuse: Ctx["refuse"]): List => {
  const list = Object.values(state.lists).find((one) => one.items.some((each) => each.id === item));
  return list && can(list, me) ? list : refuse(`you have no item ${item}; it was removed, or its list is not shared`);
};

const put = (state: Todo, list: List | null, id: number): Todo => {
  const lists = { ...state.lists };
  if (list) lists[id] = list;
  else delete lists[id];
  return { ...state, lists };
};

const tell = (ctx: Ctx, list: List, text: string) => {
  const me = ctx.who.id;
  for (const one of [list.owner, ...list.shared]) if (one !== me) ctx.wish(notice(one, text));
};

const due = (item: Item, at: number) => item.due !== null && item.due <= at && !item.done && !item.told;

const anyDue = (state: Todo, at: number) =>
  Object.values(state.lists).some((list) => list.items.some((item) => due(item, at)));

const clip = (text: string) => (text.length > CLIP ? `${text.slice(0, CLIP - 3).trimEnd()}...` : text);

const brief = (list: List) => {
  const waiting = list.items.filter((item) => !item.done);
  return {
    id: list.id,
    owner: list.owner,
    title: list.title,
    shared: list.shared,
    open: waiting.length,
    total: list.items.length,
    items: waiting.slice(0, PEEK).map((item) => ({ id: item.id, text: clip(item.text), due: item.due })),
    full: `reminders/lists/${list.id}`,
  };
};

const mine = (state: Todo, id: string, _who: Who | null, at: number) => ({
  at,
  iso: iso(at),
  lists: Object.values(state.lists)
    .filter((list) => can(list, id))
    .map(brief),
});

const whole = (state: Todo, who: Who | null, at: number): Json =>
  who ? mine(state, who.id, who, at) : { at, iso: iso(at), lists: Object.keys(state.lists).length };

const looked = (state: Todo, path: string[], who: Who | null, at: number): Json | undefined => {
  if (!who || path[0] !== "lists" || path[1] === undefined) return sliced(mine, whole)(state, path, who, at);
  const list = state.lists[Number(path[1])];
  return list && can(list, who.id) ? walk(list, path.slice(2)) : undefined;
};

const person: Schema = { type: "string", minLength: 1, maxLength: ID };

const number: Schema = { type: "integer", minimum: 1 };

const shape = (properties: Record<string, Schema>, optional: string[] = []): Schema => ({
  type: "object",
  properties,
  required: Object.keys(properties).filter((name) => !optional.includes(name)),
  additionalProperties: false,
});

const dueBy = (args: Obj, ctx: Ctx) => {
  if (args.due === undefined) return null;
  const at = Number(args.due);
  return at > ctx.at ? at : ctx.refuse(`due is in the past; it is ${iso(ctx.at)}, world time ${ctx.at}`);
};

export default app<Todo>({
  id: "reminders",
  title: "Reminders",
  line: "To-do lists of yours and those shared with you: tick items when done, set a due time for a notice, and others on the list hear of changes. A list's every item: reminders/lists/id.",
  boot: ({ actors }) => ({ people: actors, next: 1, lists: {} }),
  knobs: KNOBS,
  verbs: {
    make: {
      line: (knob) => `Make a list: a title of 1 to ${knob("title")} characters; you own at most ${knob("lists")}.`,
      args: (knob) => shape({ title: { type: "string", minLength: 1, maxLength: knob("title") } }),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const title = String(args.title);
        const most = ctx.knob("lists");
        if (!title.trim()) return ctx.refuse("the title is only spaces; name the list");
        if (Object.values(state.lists).filter((list) => list.owner === me).length >= most)
          return ctx.refuse(`you own ${most} lists; delete one first`);
        const list: List = { id: state.next, owner: me, title, shared: [], items: [] };
        ctx.say({ note: `Made list ${list.id}: ${title}`, id: list.id });
        return { ...put(state, list, list.id), next: state.next + 1 };
      },
    },
    add: {
      line: (knob) =>
        `Add an item of 1 to ${knob("text")} characters to a list you own or share, with a due world time if you want a notice then; a list holds at most ${knob("items")}. Others on the list hear of it.`,
      args: (knob) =>
        shape(
          {
            list: number,
            text: { type: "string", minLength: 1, maxLength: knob("text") },
            due: { type: "integer", minimum: 0, maximum: LAST },
          },
          ["due"],
        ),
      run: (state, args, ctx) => {
        const text = String(args.text);
        const list = open(state, Number(args.list), ctx.who.id, ctx.refuse);
        const most = ctx.knob("items");
        if (!text.trim()) return ctx.refuse("the item is only spaces; say what to do");
        if (list.items.length >= most) return ctx.refuse(`list ${list.id} holds ${most} items; remove one first`);
        const item: Item = {
          id: state.next,
          text,
          done: false,
          by: ctx.who.id,
          at: ctx.at,
          due: dueBy(args, ctx),
          told: false,
        };
        tell(ctx, list, `${ctx.who.id} added to ${list.title}: ${text}`);
        ctx.say({ note: `Added item ${item.id} to list ${list.id}`, id: item.id, list: list.id });
        return { ...put(state, { ...list, items: [...list.items, item] }, list.id), next: state.next + 1 };
      },
    },
    tick: {
      line: "Mark an item done, or with done false not done, on a list you own or share. Others on the list hear of it.",
      args: shape({ item: number, done: { type: "boolean" } }, ["done"]),
      run: (state, args, ctx) => {
        const item = Number(args.item);
        const done = args.done !== false;
        const list = holding(state, item, ctx.who.id, ctx.refuse);
        const one = list.items.find((each) => each.id === item)!;
        if (one.done === done) return ctx.refuse(`item ${item} is ${done ? "done" : "not done"} already`);
        tell(ctx, list, `${ctx.who.id} ${done ? "ticked" : "unticked"} on ${list.title}: ${one.text}`);
        ctx.say({ note: `Item ${item} is ${done ? "done" : "not done"}`, id: item, list: list.id });
        const items = list.items.map((each) => (each.id === item ? { ...each, done } : each));
        return put(state, { ...list, items }, list.id);
      },
    },
    remove: {
      line: "Remove an item from a list you own or share. Others on the list hear of it.",
      args: shape({ item: number }),
      run: (state, args, ctx) => {
        const item = Number(args.item);
        const list = holding(state, item, ctx.who.id, ctx.refuse);
        const one = list.items.find((each) => each.id === item)!;
        tell(ctx, list, `${ctx.who.id} removed from ${list.title}: ${one.text}`);
        ctx.say({ note: `Removed item ${item} from list ${list.id}`, id: item, list: list.id });
        return put(state, { ...list, items: list.items.filter((each) => each.id !== item) }, list.id);
      },
    },
    share: {
      line: (knob) =>
        `Share a list of yours with someone, who may then add, tick and remove its items; a list reaches at most ${knob("shares")}.`,
      args: shape({ list: number, with: person }),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const to = someone(state.people, String(args.with), ctx);
        const list = owned(state, Number(args.list), me, ctx.refuse);
        const most = ctx.knob("shares");
        if (to === me) return ctx.refuse("you own the list; share it with someone else");
        if (blockedBy(ctx, to, me)) return ctx.refuse(`${to} is not taking this from you`);
        if (list.shared.includes(to)) return ctx.refuse(`list ${list.id} is already shared with ${to}`);
        if (list.shared.length >= most) return ctx.refuse(`list ${list.id} reaches ${most} people; unshare one first`);
        ctx.wish(notice(to, `${me} shared list ${list.id} with you: ${list.title}`));
        ctx.say({ note: `Shared list ${list.id} with ${to}`, id: list.id, with: to });
        return put(state, { ...list, shared: [...list.shared, to] }, list.id);
      },
    },
    unshare: {
      line: "Stop sharing a list of yours with someone, or leave a list shared with you.",
      args: shape({ list: number, with: person }),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const to = String(args.with);
        const list = open(state, Number(args.list), me, ctx.refuse);
        if (list.owner !== me && to !== me)
          return ctx.refuse(`list ${list.id} is ${list.owner}'s; you may only leave it yourself`);
        if (!list.shared.includes(to)) return ctx.refuse(`list ${list.id} is not shared with ${to}`);
        const told = to === me ? list.owner : to;
        ctx.wish(notice(told, to === me ? `${me} left your list: ${list.title}` : `${me} unshared: ${list.title}`));
        ctx.say({
          note: to === me ? `Left list ${list.id}` : `Stopped sharing list ${list.id} with ${to}`,
          id: list.id,
        });
        return put(state, { ...list, shared: list.shared.filter((one) => one !== to) }, list.id);
      },
    },
    delete: {
      line: "Delete a list of yours with its items. Those it was shared with hear of it.",
      args: shape({ list: number }),
      run: (state, args, ctx) => {
        const list = owned(state, Number(args.list), ctx.who.id, ctx.refuse);
        tell(ctx, list, `${ctx.who.id} deleted a list: ${list.title}`);
        ctx.say({ note: `Deleted list ${list.id}`, id: list.id });
        return put(state, null, list.id);
      },
    },
    remind: {
      inner: true,
      line: "Send a notice for every item now due, to everyone on its list; it happens by itself whenever anyone acts.",
      run: (state, _args, ctx) => {
        ctx.say({ note: "Sent a notice for every item due" });
        return {
          ...state,
          lists: Object.fromEntries(
            Object.entries(state.lists).map(([id, list]) => [
              id,
              {
                ...list,
                items: list.items.map((item) => {
                  if (!due(item, ctx.at)) return item;
                  for (const one of [list.owner, ...list.shared])
                    ctx.wish(notice(one, `Due on ${list.title}: ${item.text}`, true));
                  return { ...item, told: true };
                }),
              },
            ]),
          ),
        };
      },
    },
  },
  moves: (state) =>
    Object.keys(state.lists).length
      ? ["make", "add", "tick", "remove", "share", "unshare", "delete", "remind"]
      : ["make"],
  read: looked,
  wake: (state, at) => (anyDue(state, at) ? "remind" : null),
});
