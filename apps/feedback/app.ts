import type { Json, Knob, Knobs, Schema, Who } from "@mrlyprod/os/contract/index.ts";
import { app, sliced, walk } from "@mrlyprod/os/contract/index.ts";
import { notice } from "../shared/people.ts";

export type Severity = "note" | "bug" | "blocker";

export type Item = {
  id: number;
  by: string;
  app: string;
  verb: string | null;
  happened: string;
  expected: string | null;
  severity: Severity;
  at: number;
  same: string[];
};

export type Feedback = { next: number; items: Record<string, Item> };

const KNOBS = {
  text: {
    line: "Longest report, in characters.",
    schema: { type: "integer", minimum: 100, maximum: 10000 },
    reality: 500,
  },
  keep: {
    line: "Reports one person may send.",
    schema: { type: "integer", minimum: 1, maximum: 10000 },
    reality: 100,
  },
} satisfies Knobs;

export const SEVERITIES: Severity[] = ["note", "bug", "blocker"];

const NAME = 200;

const ANYWHERE = "world";

const TOP = 20;

const OPENING = ["send"];

const all = (state: Feedback) => Object.values(state.items).sort((a, b) => a.id - b.id);

const sent = (state: Feedback, id: string) => all(state).filter((one) => one.by === id);

const about = (state: Feedback, app: string) => all(state).filter((one) => one.app === app);

const mine = (state: Feedback, id: string) => ({
  sent: sent(state, id),
  same: all(state)
    .filter((one) => one.same.includes(id))
    .map((one) => one.id),
});

const counts = (state: Feedback) => {
  const out = new Map<string, number>();
  for (const one of all(state)) out.set(one.app, (out.get(one.app) ?? 0) + 1);
  return Object.fromEntries(out);
};

const board = (state: Feedback) => ({
  count: all(state).length,
  about: counts(state),
  latest: all(state).slice(-TOP).reverse(),
});

const slice = (state: Feedback, id: string, who: Who | null): Json =>
  who && who.id !== id ? mine(state, id) : { ...board(state), ...mine(state, id) };

const whole = (state: Feedback, who: Who | null): Json => (who ? slice(state, who.id, who) : board(state));

const view = sliced(slice, whole, true);

const free = (value: Json | undefined) =>
  String(value ?? "")
    .trim()
    .slice(0, NAME);

const sending = (knob: Knob): Schema => ({
  type: "object",
  properties: {
    app: { type: "string" },
    verb: { type: "string" },
    happened: { type: "string", minLength: 1, maxLength: knob("text") },
    expected: { type: "string", maxLength: knob("text") },
    severity: { enum: SEVERITIES },
  },
  required: ["app", "happened", "severity"],
  additionalProperties: false,
});

const naming: Schema = {
  type: "object",
  properties: { id: { type: "integer", minimum: 1 } },
  required: ["id"],
  additionalProperties: false,
};

export default app<Feedback>({
  id: "feedback",
  title: "Feedback",
  line: "What broke, what you expected and what you wish existed, for the makers; never voted, never capped by others. Read feedback for the latest, feedback/items/3 for one, feedback/about/wish for one app.",
  boot: () => ({ next: 1, items: {} }),
  knobs: KNOBS,
  verbs: {
    send: {
      line: (knob) =>
        `Tell the makers what happened and what you expected: 1 to ${knob("text")} characters, a severity of ${SEVERITIES.join(", ")}; ${knob("keep")} items each. app is anything, even read, door or world. verb is free text: name a verb you wish existed, not only one that does.`,
      args: sending,
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const happened = String(args.happened).trim();
        const most = ctx.knob("keep");
        if (!happened) return ctx.refuse("happened is only spaces; say what happened");
        if (sent(state, me).length >= most)
          return ctx.refuse(`you sent ${most} items of feedback; the makers have them`);
        const expected = args.expected === undefined ? null : String(args.expected).trim() || null;
        const item: Item = {
          id: state.next,
          by: me,
          app: free(args.app) || ANYWHERE,
          verb: free(args.verb) || null,
          happened,
          expected,
          severity: args.severity as Severity,
          at: ctx.at,
          same: [],
        };
        ctx.say({ note: `Sent item ${item.id} about ${item.app}`, id: item.id });
        return { next: state.next + 1, items: { ...state.items, [item.id]: item } };
      },
    },
    same: {
      line: "Say the same happened to you, on someone else's item, once; its sender hears of it.",
      args: naming,
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const one = state.items[String(args.id)]!;
        if (one.by === me) return ctx.refuse(`you sent item ${one.id}; others say the same`);
        if (one.same.includes(me)) return ctx.refuse(`you already said the same of item ${one.id}`);
        ctx.wish(notice(one.by, `${me} says the same of your item ${one.id}`));
        ctx.say({ note: `Said the same of item ${one.id}; ${one.same.length + 1} others said so`, id: one.id });
        return { ...state, items: { ...state.items, [one.id]: { ...one, same: [...one.same, me] } } };
      },
    },
  },
  moves: (state) => (state.next > 1 ? [...OPENING, "same"] : OPENING),
  read: (state, path, who, at) => {
    const [head, name, ...rest] = path;
    if (head === "items") return walk(state.items, path.slice(1));
    if (head === "about" && name) return walk(about(state, name), rest);
    return view(state, path, who, at);
  },
});
