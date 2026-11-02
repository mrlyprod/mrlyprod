import type { Schema } from "../../sdk/system.ts";
import { app, NOTICE, sliced } from "../../sdk/system.ts";
import bot from "./bot.ts";

/* TYPES */

export type Notice = { id: number; app: string; from: string; text: string; at: number; read: boolean };

export type Inbox = { people: string[] | null; next: number; notices: Record<string, Notice[]> };

export const KEEP = 100;

const ID = 40;

const APP = 20;

/* NOTICES */

const ownerOf = (state: Inbox, id: number) =>
  Object.keys(state.notices).find((one) => state.notices[one]!.some((notice) => notice.id === id));

function held(state: Inbox, id: number, me: string, refuse: (note: string) => never): Notice[] {
  const owner = ownerOf(state, id);
  if (!owner) return refuse(`no notice ${id}; it was dismissed or dropped`);
  if (owner !== me) return refuse(`notice ${id} is ${owner}'s`);
  return state.notices[me]!;
}

const put = (state: Inbox, id: string, list: Notice[]): Inbox => ({
  ...state,
  notices: { ...state.notices, [id]: list },
});

const mine = (state: Inbox, id: string) => {
  const list = state.notices[id] ?? [];
  return { unread: list.filter((one) => !one.read).length, notices: [...list].reverse() };
};

/* SCHEMAS */

const raising = (state: Inbox): Schema => ({
  type: "object",
  properties: {
    to: state.people ? { enum: state.people } : { type: "string", minLength: 1, maxLength: ID },
    app: { type: "string", minLength: 1, maxLength: APP },
    text: { type: "string", minLength: 1, maxLength: NOTICE },
  },
  required: ["to", "app", "text"],
  additionalProperties: false,
});

const one = (state: Inbox, required: boolean): Schema => ({
  type: "object",
  properties: { id: { type: "integer", minimum: 1, maximum: Math.max(1, state.next - 1) } },
  required: required ? ["id"] : [],
  additionalProperties: false,
});

/* APP */

export default app<Inbox>({
  id: "notifications",
  title: "Notifications",
  line: "The notices other apps raise for each user: new mail, invitations, alarms, missed calls.",
  boot: ({ actors }) => ({ people: actors, next: 1, notices: {} }),
  verbs: {
    notice: {
      line: `Raise a notice for someone, naming the app it comes from; 1 to ${NOTICE} characters; each user keeps the last ${KEEP}.`,
      args: raising,
      run: (state, args, ctx) => {
        const to = String(args.to);
        const from = String(args.app);
        const text = String(args.text);
        if (!/^[a-z][a-z0-9]*$/.test(from)) return ctx.refuse("an app id is lowercase letters and digits");
        if (!text.trim()) return ctx.refuse("the text is only spaces; say something");
        const notice: Notice = { id: state.next, app: from, from: ctx.who.id, text, at: ctx.at, read: false };
        return { ...put(state, to, [...(state.notices[to] ?? []), notice].slice(-KEEP)), next: state.next + 1 };
      },
    },
    read: {
      line: "Mark a notice of yours read; with no id, all of them.",
      args: (state) => one(state, false),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        if (args.id === undefined)
          return put(
            state,
            me,
            (state.notices[me] ?? []).map((notice) => ({ ...notice, read: true })),
          );
        const id = Number(args.id);
        const list = held(state, id, me, ctx.refuse);
        return put(
          state,
          me,
          list.map((notice) => (notice.id === id ? { ...notice, read: true } : notice)),
        );
      },
    },
    dismiss: {
      line: "Remove a notice of yours.",
      args: (state) => one(state, true),
      run: (state, args, ctx) => {
        const id = Number(args.id);
        const list = held(state, id, ctx.who.id, ctx.refuse);
        return put(
          state,
          ctx.who.id,
          list.filter((notice) => notice.id !== id),
        );
      },
    },
    clear: {
      line: "Remove every notice of yours already read.",
      run: (state, _args, ctx) =>
        put(
          state,
          ctx.who.id,
          (state.notices[ctx.who.id] ?? []).filter((notice) => !notice.read),
        ),
    },
  },
  moves: (state) => (state.next > 1 ? ["notice", "read", "dismiss", "clear"] : ["notice"]),
  read: sliced(mine),
  bot,
});
