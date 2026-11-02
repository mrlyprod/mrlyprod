import type { Schema } from "../../sdk/system.ts";
import { app, sliced } from "../../sdk/system.ts";
import bot from "./bot.ts";

/* TYPES */

export type Message = { n: number; from: string; text: string; at: number };

export type Thread = { between: string[]; n: number; messages: Message[] };

export type Chat = {
  people: string[] | null;
  threads: Record<string, Thread>;
  seen: Record<string, Record<string, number>>;
  unread: Record<string, number>;
};

export const TEXT = 500;

export const KEEP = 50;

const ID = 40;

/* THREADS */

export const threadOf = (a: string, b: string) => [a, b].sort().join("+");

const behind = (state: Chat, id: string, thread: string) =>
  (state.threads[thread]?.n ?? 0) - (state.seen[id]?.[thread] ?? 0);

const caught = (state: Chat, id: string, thread: string): Chat => ({
  ...state,
  seen: { ...state.seen, [id]: { ...state.seen[id], [thread]: state.threads[thread]!.n } },
  unread: { ...state.unread, [id]: (state.unread[id] ?? 0) - behind(state, id, thread) },
});

const mine = (state: Chat, id: string) => ({
  unread: state.unread[id] ?? 0,
  threads: Object.fromEntries(
    Object.entries(state.threads)
      .filter(([, thread]) => thread.between.includes(id))
      .map(([key, thread]) => [
        key,
        {
          with: thread.between.find((one) => one !== id)!,
          unread: behind(state, id, key),
          messages: thread.messages,
        },
      ]),
  ),
});

/* SCHEMAS */

const sending = (state: Chat): Schema => ({
  type: "object",
  properties: {
    to: state.people ? { enum: state.people } : { type: "string", minLength: 1, maxLength: ID },
    text: { type: "string", minLength: 1, maxLength: TEXT },
  },
  required: ["to", "text"],
  additionalProperties: false,
});

const reading = (state: Chat): Schema => ({
  type: "object",
  properties: { thread: { enum: Object.keys(state.threads) } },
  required: ["thread"],
  additionalProperties: false,
});

/* APP */

export default app<Chat>({
  id: "messages",
  title: "Messages",
  line: "Threads between two people: send, read, and an unread count each.",
  boot: ({ actors }) => ({
    people: actors,
    threads: {},
    seen: {},
    unread: Object.fromEntries((actors ?? []).map((id) => [id, 0])),
  }),
  verbs: {
    send: {
      line: `Write to someone: 1 to ${TEXT} characters; a thread keeps its last ${KEEP} messages.`,
      args: sending,
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const to = String(args.to);
        const text = String(args.text);
        if (to === me) return ctx.refuse("a thread is between two people; write to someone else");
        if (!text.trim()) return ctx.refuse("the text is only spaces; say something");
        const key = threadOf(me, to);
        const base = state.threads[key] ? caught(state, me, key) : state;
        const was = base.threads[key] ?? { between: [me, to].sort(), n: 0, messages: [] };
        const n = was.n + 1;
        const thread = { ...was, n, messages: [...was.messages, { n, from: me, text, at: ctx.at }].slice(-KEEP) };
        return {
          ...base,
          threads: { ...base.threads, [key]: thread },
          seen: { ...base.seen, [me]: { ...base.seen[me], [key]: n } },
          unread: { ...base.unread, [to]: (base.unread[to] ?? 0) + 1 },
        };
      },
    },
    read: {
      line: "Mark a thread of yours read.",
      args: reading,
      run: (state, args, ctx) => {
        const key = String(args.thread);
        const { between } = state.threads[key]!;
        if (!between.includes(ctx.who.id)) return ctx.refuse(`only ${between.join(" and ")} read ${key}`);
        return caught(state, ctx.who.id, key);
      },
    },
  },
  moves: (state) => (Object.keys(state.threads).length ? ["send", "read"] : ["send"]),
  read: sliced(mine),
  bot,
});
