import type { Ctx, Knob, Knobs, Obj, Schema, Who } from "@mrlyprod/os/contract/index.ts";
import { app, SLICE, sliced } from "@mrlyprod/os/contract/index.ts";
import { notice, reach, someone } from "../shared/people.ts";

export type Message = { n: number; from: string; text: string; at: number; reply: number | null };

export type Thread = {
  between: string[];
  name: string | null;
  org: number | null;
  by: string;
  n: number;
  messages: Message[];
};

export type Chat = {
  people: string[] | null;
  threads: Record<string, Thread>;
  seen: Record<string, Record<string, number>>;
  groups: number;
};

const KNOBS = {
  keep: {
    line: "Messages a thread keeps.",
    schema: { type: "integer", minimum: 5, maximum: 5000 },
    reality: 50,
  },
  text: {
    line: "Characters in one message.",
    schema: { type: "integer", minimum: 20, maximum: 50_000 },
    reality: 500,
  },
  group: {
    line: "People one group holds.",
    schema: { type: "integer", minimum: 2, maximum: 256 },
    reality: 32,
  },
  groups: {
    line: "Groups one person may start.",
    schema: { type: "integer", minimum: 1, maximum: 200 },
    reality: 10,
  },
  name: {
    line: "Characters in a group's name.",
    schema: { type: "integer", minimum: 8, maximum: 200 },
    reality: 40,
  },
} satisfies Knobs;

const ID = 40;

const KEY = 100;

const SHOWN = 8;

const OPEN = 5;

const TAIL = 3;

const PREVIEW = 80;

export const threadOf = (a: string, b: string) => [a, b].sort().join("+");

const threadAt = (state: Chat, key: string) => (Object.hasOwn(state.threads, key) ? state.threads[key] : undefined);

const keyOf = (state: Chat, given: string, me: string) => {
  const pair = threadOf(me, given);
  return !threadAt(state, given) && threadAt(state, pair) ? pair : given;
};

const behind = (state: Chat, id: string, thread: string) =>
  (state.threads[thread]?.n ?? 0) - (state.seen[id]?.[thread] ?? 0);

const mark = (state: Chat, id: string, thread: string, n: number): Chat => ({
  ...state,
  seen: { ...state.seen, [id]: { ...state.seen[id], [thread]: n } },
});

const put = (state: Chat, key: string, thread: Thread): Chat => ({
  ...state,
  threads: { ...state.threads, [key]: thread },
});

const orgFolk = (ctx: Ctx, org: number) =>
  Object.keys((ctx.read(`people/orgs/${org}/members`) as Obj | null) ?? {}).sort();

function synced(state: Chat, key: string, ctx: Ctx): Chat {
  const thread = threadAt(state, key);
  if (!thread || thread.org === null) return state;
  const between = orgFolk(ctx, thread.org);
  const fresh = between.filter((one) => !thread.between.includes(one));
  return fresh.reduce((next, one) => mark(next, one, key, thread.n), put(state, key, { ...thread, between }));
}

const yours = (state: Chat, key: string, me: string, ctx: Ctx): Thread => {
  const thread = threadAt(state, key);
  if (!thread?.between.includes(me)) return ctx.refuse(`you have no thread ${key}; read messages lists yours`);
  return thread;
};

const groupOf = (state: Chat, key: string, me: string, ctx: Ctx): Thread => {
  const thread = yours(state, key, me, ctx);
  if (thread.name === null) return ctx.refuse(`${key} is between two people; start a group for more`);
  if (thread.org !== null) return ctx.refuse(`${thread.name} follows org ${thread.org}; membership is set in people`);
  return thread;
};

const allowed = (thread: Thread, me: string, ctx: Ctx) => {
  if (thread.name === null) reach(ctx, thread.between.find((one) => one !== me)!);
};

const spoken = (name: string | null, key: string, me: string, reply: number | null, text: string) => {
  const verb = reply === null ? "" : " replied";
  return name === null ? `${me}${verb}: ${text}` : `${me}${verb} in ${name} (${key}): ${text}`;
};

function spoke(state: Chat, key: string, text: string, reply: number | null, ctx: Ctx): Chat {
  const me = ctx.who.id;
  const was = state.threads[key]!;
  const n = was.n + 1;
  const messages = [...was.messages, { n, from: me, text, at: ctx.at, reply }].slice(-ctx.knob("keep"));
  for (const to of was.between) if (to !== me) ctx.wish(notice(to, spoken(was.name, key, me, reply, text)));
  ctx.say({ note: `Sent message ${n} in ${key}`, thread: key, n, last: messages.slice(-TAIL) });
  return mark(put(state, key, { ...was, n, messages }), me, key, n);
}

const textOf = (args: Obj, ctx: Ctx) => {
  const text = String(args.text);
  if (!text.trim()) ctx.refuse("the text is only spaces; say something");
  return text;
};

const tellAdded = (ctx: Ctx, to: string, name: string, key: string) => {
  ctx.wish(notice(to, `${ctx.who.id} added you to ${name} (${key})`));
};

const started = (state: Chat, me: string) =>
  Object.values(state.threads).filter(
    (one) => one.by === me && one.name !== null && one.org === null && one.between.includes(me),
  ).length;

function opened(state: Chat, key: string, name: string, org: number | null, between: string[], ctx: Ctx) {
  const by = ctx.who.id;
  for (const to of between) if (to !== by) tellAdded(ctx, to, name, key);
  ctx.say({ note: `Started ${name} as ${key}`, thread: key, people: between });
  return put(state, key, { between: [...between].sort(), name, org, by, n: 0, messages: [] });
}

function orgGroup(state: Chat, org: number, name: string, ctx: Ctx): Chat {
  const me = ctx.who.id;
  const key = `org-${org}`;
  const folk = orgFolk(ctx, org);
  if (!folk.includes(me)) return ctx.refuse(`you are not a member of org ${org}; join it in people first`);
  if (threadAt(state, key)) {
    ctx.say({ note: `Org ${org} already has its group, ${key}`, thread: key });
    return synced(state, key, ctx);
  }
  return opened(state, key, name, org, folk, ctx);
}

function plainGroup(state: Chat, name: string, people: string[], ctx: Ctx): Chat {
  const me = ctx.who.id;
  const limit = ctx.knob("groups");
  if (started(state, me) >= limit) return ctx.refuse(`you started ${limit} groups; leave one first`);
  if (people.includes(me)) return ctx.refuse("you are in your own group already; name the others");
  for (const one of people) reach(ctx, one);
  const groups = state.groups + 1;
  return opened({ ...state, groups }, `group-${groups}`, name, null, [me, ...people], ctx);
}

const brief = ({ n, from, text, at, reply }: Message) => ({
  n,
  from,
  text: text.length > PREVIEW ? `${text.slice(0, PREVIEW - 3)}...` : text,
  at,
  reply,
});

const head = (state: Chat, id: string, key: string): Obj => {
  const thread = state.threads[key]!;
  const unread = behind(state, id, key);
  if (thread.name === null) return { with: thread.between.find((one) => one !== id)!, unread };
  return { name: thread.name, members: thread.between, unread };
};

const summary = (state: Chat, id: string, key: string) => {
  const last = state.threads[key]!.messages.at(-1);
  return { ...head(state, id, key), last: last ? brief(last) : null };
};

const lastAt = (thread: Thread) => thread.messages.at(-1)?.at ?? 0;

const mineOf = (state: Chat, id: string) =>
  Object.keys(state.threads)
    .filter((key) => state.threads[key]!.between.includes(id))
    .sort((a, b) => lastAt(state.threads[b]!) - lastAt(state.threads[a]!) || a.localeCompare(b));

const inFull = (state: Chat, id: string, key: string) => ({
  ...head(state, id, key),
  messages: state.threads[key]!.messages,
});

const view = (state: Chat, id: string, depth: number): Obj => {
  const keys = mineOf(state, id);
  const unread = keys.reduce((sum, key) => sum + behind(state, id, key), 0);
  const brieflies = (some: string[]) => Object.fromEntries(some.map((key) => [key, summary(state, id, key)]));
  if (depth > 1) return { unread, threads: Object.fromEntries(keys.map((key) => [key, inFull(state, id, key)])) };
  if (depth === 1) return { unread, threads: brieflies(keys) };
  return {
    unread,
    threads: brieflies(keys.slice(0, SHOWN)),
    more: "messages/threads, messages/threads/<thread>",
  };
};

const totals = (state: Chat) => {
  const threads = Object.values(state.threads);
  return {
    threads: threads.length,
    groups: threads.filter((one) => one.name !== null).length,
    messages: threads.reduce((sum, one) => sum + one.n, 0),
  };
};

const depthOf = (path: string[]) => path.length - (path[0]?.startsWith(SLICE) ? 1 : 0);

const person: Schema = { type: "string", minLength: 1, maxLength: ID };

const threadKey: Schema = { type: "string", minLength: 1, maxLength: KEY };

const words = (knob: Knob): Schema => ({ type: "string", minLength: 1, maxLength: knob("text") });

const shape = (properties: Record<string, Schema>, required = Object.keys(properties)): Schema => ({
  type: "object",
  properties,
  required,
  additionalProperties: false,
});

export default app<Chat>({
  id: "messages",
  title: "Messages",
  line: "Threads between two people and named groups: send, reply, read, and an unread count each.",
  boot: ({ actors }) => ({ people: actors, threads: {}, seen: {}, groups: 0 }),
  knobs: KNOBS,
  verbs: {
    send: {
      line: (knob) =>
        `Write to someone; they are told at once: 1 to ${knob("text")} characters; a thread keeps its last ${knob("keep")} messages.`,
      args: (knob) => shape({ to: person, text: words(knob) }),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const to = someone(state.people, String(args.to), ctx);
        const text = textOf(args, ctx);
        if (to === me) return ctx.refuse("a thread is between two people; write to someone else");
        reach(ctx, to);
        const key = threadOf(me, to);
        const base = threadAt(state, key)
          ? state
          : put(state, key, { between: [me, to].sort(), name: null, org: null, by: me, n: 0, messages: [] });
        return spoke(base, key, text, null, ctx);
      },
    },
    post: {
      line: "Write in a thread of yours by its id or the other person's id; the reply shows its last messages.",
      args: (knob) => shape({ thread: threadKey, text: words(knob) }),
      run: (state, args, ctx) => {
        const key = keyOf(state, String(args.thread), ctx.who.id);
        const base = synced(state, key, ctx);
        const thread = yours(base, key, ctx.who.id, ctx);
        const text = textOf(args, ctx);
        allowed(thread, ctx.who.id, ctx);
        return spoke(base, key, text, null, ctx);
      },
    },
    reply: {
      line: "Answer one message of a thread of yours, by the thread's id or the other person's id, and the message's n.",
      args: (knob) => shape({ thread: threadKey, n: { type: "integer", minimum: 1 }, text: words(knob) }),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const key = keyOf(state, String(args.thread), me);
        const n = Number(args.n);
        const base = synced(state, key, ctx);
        const thread = yours(base, key, me, ctx);
        const text = textOf(args, ctx);
        const to = thread.messages.find((one) => one.n === n);
        if (!to) return ctx.refuse(`${key} no longer holds message ${n}; read messages/threads/${key}`);
        allowed(thread, me, ctx);
        if (to.from !== me) reach(ctx, to.from);
        return spoke(base, key, text, n, ctx);
      },
    },
    read: {
      line: `Open a thread of yours, by its id or the other person's id: it is marked read and the reply holds its last ${OPEN} messages, or last as many.`,
      args: (knob) =>
        shape({ thread: threadKey, last: { type: "integer", minimum: 1, maximum: knob("keep") } }, ["thread"]),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const key = keyOf(state, String(args.thread), me);
        const base = synced(state, key, ctx);
        const thread = yours(base, key, me, ctx);
        const messages = thread.messages.slice(-(args.last === undefined ? OPEN : Number(args.last)));
        ctx.say({ note: `${key}: the last ${messages.length} of ${thread.n} messages`, thread: key, messages });
        return mark(base, me, key, thread.n);
      },
    },
    group: {
      line: (knob) =>
        `Start a named group with up to ${knob("group")} people, or with org, the group of an org you belong to, whose members follow the org.`,
      args: (knob) =>
        shape(
          {
            name: { type: "string", minLength: 1, maxLength: knob("name") },
            people: { type: "array", items: person, maxItems: knob("group") - 1 },
            org: { type: "integer", minimum: 1 },
          },
          ["name"],
        ),
      run: (state, args, ctx) => {
        const name = String(args.name).trim();
        if (!name) return ctx.refuse("a name needs more than spaces");
        const people = [
          ...new Set(((args.people as string[] | undefined) ?? []).map((one) => someone(state.people, one, ctx))),
        ];
        if (args.org === undefined) return plainGroup(state, name, people, ctx);
        if (people.length) return ctx.refuse("an org's group takes its members from the org; leave people out");
        return orgGroup(state, Number(args.org), name, ctx);
      },
    },
    add: {
      line: "Add someone to a group of yours.",
      args: shape({ thread: threadKey, person }),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const key = String(args.thread);
        const one = someone(state.people, String(args.person), ctx);
        const thread = groupOf(state, key, me, ctx);
        const limit = ctx.knob("group");
        if (thread.between.includes(one)) return ctx.refuse(`${one} is already in ${thread.name}`);
        if (thread.between.length >= limit) return ctx.refuse(`${thread.name} is full at ${limit} people`);
        reach(ctx, one);
        tellAdded(ctx, one, thread.name!, key);
        ctx.say({ note: `Added ${one} to ${thread.name}`, thread: key });
        const between = [...thread.between, one].sort();
        return mark(put(state, key, { ...thread, between }), one, key, thread.n);
      },
    },
    leave: {
      line: "Leave a group of yours; the last one out closes it.",
      args: shape({ thread: threadKey }),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const key = String(args.thread);
        const thread = groupOf(state, key, me, ctx);
        const between = thread.between.filter((one) => one !== me);
        for (const to of between) ctx.wish(notice(to, `${me} left ${thread.name}`));
        ctx.say({ note: `Left ${thread.name}`, thread: key });
        if (between.length) return put(state, key, { ...thread, between });
        const threads = { ...state.threads };
        delete threads[key];
        return { ...state, threads };
      },
    },
  },
  moves: (state) =>
    Object.keys(state.threads).length ? ["send", "post", "reply", "read", "group", "add", "leave"] : ["send", "group"],
  read: (state, path, who, at) =>
    sliced(
      (one: Chat, id: string) => view(one, id, depthOf(path)),
      (one: Chat, by: Who | null) => (by ? view(one, by.id, depthOf(path)) : totals(one)),
    )(state, path, who, at),
});
