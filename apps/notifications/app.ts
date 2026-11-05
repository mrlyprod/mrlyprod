import type { Ctx, Knobs, Schema } from "@mrlyprod/os/contract/index.ts";
import { app, sliced, walk } from "@mrlyprod/os/contract/index.ts";
import { blockedBy, mutedBy, NOTICE, reach, someone } from "../shared/people.ts";

export type Notice = {
  id: number;
  app: string;
  from: string;
  text: string;
  at: number;
  read: boolean;
  low?: boolean;
};

export type Inbox = {
  people: string[] | null;
  next: number;
  notices: Record<string, Notice[]>;
  sent: Record<string, number[]>;
};

const KNOBS = {
  keep: {
    line: "Notices one person keeps.",
    schema: { type: "integer", minimum: 5, maximum: 5000 },
    reality: 100,
  },
  rate: {
    line: "Notices one person may send directly in a window; 0 leaves notices to the apps.",
    schema: { type: "integer", minimum: 0, maximum: 1000 },
    reality: 10,
  },
  window: {
    line: "Milliseconds that window lasts.",
    schema: { type: "integer", minimum: 1000, maximum: 86400000 },
    reality: 60000,
  },
  low: {
    line: "What happens to low-priority notices, such as chat on a cell: keep them listed after the rest, or drop them.",
    schema: { enum: ["keep", "drop"] },
    reality: "keep",
  },
} satisfies Knobs;

const ID = 40;

const PERSON = "people";

const listOf = (state: Inbox, id: string) => (Object.hasOwn(state.notices, id) ? state.notices[id]! : []);

function held(state: Inbox, id: number, me: string, refuse: (note: string) => never): Notice[] {
  const list = listOf(state, me);
  if (!list.some((notice) => notice.id === id)) return refuse(`you hold no notice ${id}; read notifications for yours`);
  return list;
}

const put = (state: Inbox, id: string, list: Notice[]): Inbox => ({
  ...state,
  notices: { ...state.notices, [id]: list },
});

const calm = (one: Notice) => one.low === true;

function trimmed(list: Notice[], keep: number): Notice[] {
  const out = [...list];
  while (out.length > keep) {
    const at = out.findIndex(calm);
    out.splice(at === -1 ? 0 : at, 1);
  }
  return out;
}

const ordered = (list: Notice[]) => [...list].sort((a, b) => Number(calm(a)) - Number(calm(b)) || b.id - a.id);

const counts = (list: Notice[]) => {
  const apps: Record<string, { unread: number; low: number; total: number }> = {};
  for (const one of list) {
    const row = apps[one.app] ?? { unread: 0, low: 0, total: 0 };
    row.total += 1;
    if (!one.read) row.unread += 1;
    if (!one.read && calm(one)) row.low += 1;
    apps[one.app] = row;
  }
  return apps;
};

const summary = (list: Notice[]) => {
  const unread = list.filter((one) => !one.read);
  return {
    unread: unread.length,
    waiting: unread.filter(calm).length,
    apps: counts(list),
  };
};

const mine = (state: Inbox, id: string) => {
  const list = listOf(state, id);
  return {
    ...summary(list),
    by: "notifications/@me/by/<app> lists one app's notices; notices marked low can wait",
    notices: ordered(list),
  };
};

const byApp = (state: Inbox, id: string, name: string) => {
  const list = listOf(state, id).filter((one) => one.app === name);
  return { app: name, ...summary(list), notices: ordered(list) };
};

const same = (name: string | undefined) => (one: Notice) => name === undefined || one.app === name;

function direct(state: Inbox, to: string, ctx: Ctx): number[] {
  const me = ctx.who.id;
  const most = ctx.knob("rate");
  const since = ctx.at - ctx.knob("window");
  const recent = (state.sent[me] ?? []).filter((at) => at > since);
  if (!most) ctx.refuse("in this world only apps raise notices; send a message instead");
  if (to === me) ctx.refuse("that is you; use reminders or notes to tell yourself something");
  reach(ctx, to);
  if (recent.length >= most) ctx.refuse(`you are at the limit of ${most} notices for now; wait or send a message`);
  return [...recent, ctx.at];
}

const silent = (ctx: Ctx, to: string, from: string) =>
  to !== from && (blockedBy(ctx, to, from) || blockedBy(ctx, from, to) || mutedBy(ctx, to, from));

const raising: Schema = {
  type: "object",
  properties: {
    to: { type: "string", minLength: 1, maxLength: ID },
    text: { type: "string", minLength: 1, maxLength: NOTICE },
    system: { type: "boolean" },
    low: { type: "boolean" },
  },
  required: ["to", "text"],
  additionalProperties: false,
};

const sorting: Schema = {
  type: "object",
  properties: {
    id: { type: "integer", minimum: 1 },
    app: { type: "string", minLength: 1, maxLength: ID },
    low: { type: "boolean" },
  },
  additionalProperties: false,
};

const one = (required: boolean): Schema => ({
  type: "object",
  properties: { id: { type: "integer", minimum: 1 } },
  required: required ? ["id"] : [],
  additionalProperties: false,
});

export default app<Inbox>({
  id: "notifications",
  title: "Notifications",
  line: "What is new for you: notices from apps and from people, each with the app it came through and who caused it.",
  boot: ({ actors }) => ({ people: actors, next: 1, notices: {}, sent: {} }),
  verbs: {
    notice: {
      line: (knob) =>
        `Send someone a notice from you, labelled ${PERSON} and showing your id; 1 to ${NOTICE} characters. Limit: ${knob("rate")} per ${knob("window") / 1000} seconds. Each person keeps the last ${knob("keep")}.`,
      args: raising,
      run: (state, args, ctx) => {
        const to = someone(state.people, String(args.to), ctx);
        const text = String(args.text);
        const me = ctx.who.id;
        if (!text.trim()) return ctx.refuse("the text is only spaces; say something");
        const sent = ctx.via ? state.sent : { ...state.sent, [me]: direct(state, to, ctx) };
        const system = args.system === true && ctx.via !== null;
        const low = args.low === true && ctx.via !== null;
        if (!ctx.via) ctx.say({ note: `Sent to ${to}` });
        if (!system && silent(ctx, to, me)) return { ...state, sent };
        if (low && ctx.knob<string>("low") === "drop") return { ...state, sent };
        const from = system ? ctx.via! : me;
        const notice: Notice = {
          id: state.next,
          app: ctx.via ?? PERSON,
          from,
          text,
          at: ctx.at,
          read: false,
          ...(low ? { low } : {}),
        };
        const list = trimmed([...listOf(state, to), notice], ctx.knob("keep"));
        return { ...put(state, to, list), next: state.next + 1, sent };
      },
    },
    read: {
      line: "Mark a notice of yours read by id; with app, all of that app's; with low, all low-priority ones; with nothing, all of them.",
      args: sorting,
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        if (args.id === undefined) {
          const name = args.app === undefined ? undefined : String(args.app);
          const hit = (notice: Notice) => same(name)(notice) && (args.low !== true || calm(notice));
          const n = listOf(state, me).filter((notice) => !notice.read && hit(notice)).length;
          ctx.say({
            note: `${n} notices read${name ? ` from ${name}` : ""}${args.low === true ? ", low priority only" : ""}`,
          });
          return put(
            state,
            me,
            listOf(state, me).map((notice) => (hit(notice) ? { ...notice, read: true } : notice)),
          );
        }
        const id = Number(args.id);
        const list = held(state, id, me, ctx.refuse);
        ctx.say({ note: `Notice ${id} is read`, id });
        return put(
          state,
          me,
          list.map((notice) => (notice.id === id ? { ...notice, read: true } : notice)),
        );
      },
    },
    dismiss: {
      line: "Remove a notice of yours.",
      args: one(true),
      run: (state, args, ctx) => {
        const id = Number(args.id);
        const list = held(state, id, ctx.who.id, ctx.refuse);
        ctx.say({ note: `Removed notice ${id}`, id });
        return put(
          state,
          ctx.who.id,
          list.filter((notice) => notice.id !== id),
        );
      },
    },
    clear: {
      line: "Remove every notice of yours already read; with app, only that app's.",
      args: {
        type: "object",
        properties: { app: { type: "string", minLength: 1, maxLength: ID } },
        additionalProperties: false,
      },
      run: (state, args, ctx) => {
        const name = args.app === undefined ? undefined : String(args.app);
        const kept = listOf(state, ctx.who.id).filter((notice) => !notice.read || !same(name)(notice));
        ctx.say({ note: `Removed ${listOf(state, ctx.who.id).length - kept.length} read notices` });
        return put(state, ctx.who.id, kept);
      },
    },
  },
  moves: (state) => (state.next > 1 ? ["notice", "read", "dismiss", "clear"] : ["notice"]),
  read: (state, path, who, at) => {
    const [head = "", by, name, ...rest] = path;
    if (who && by === "by" && name && [`@${who.id}`, "@me"].includes(head))
      return walk(byApp(state, who.id, name), rest);
    return sliced(mine)(state, path, who, at);
  },
  knobs: KNOBS,
});
