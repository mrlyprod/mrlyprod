import type { Ctx, Json, Schema } from "../../sdk/system.ts";
import { app, manuals, SLICE, walk } from "../../sdk/system.ts";
import { bot } from "./bot.ts";

/* TYPES */

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

export const SEVERITIES: Severity[] = ["note", "bug", "blocker"];

export const TEXT = 500;

export const NAME = 40;

export const KEEP = 100;

/* ITEMS */

const all = (state: Feedback) => Object.values(state.items).sort((a, b) => a.id - b.id);

const sent = (state: Feedback, id: string) => all(state).filter((one) => one.by === id);

const about = (state: Feedback, app: string) => all(state).filter((one) => one.app === app);

const mine = (state: Feedback, id: string) => ({
  sent: sent(state, id),
  same: all(state)
    .filter((one) => one.same.includes(id))
    .map((one) => one.id),
});

export const bare = (verb: string) => verb.slice(verb.lastIndexOf(".") + 1);

export const verbsOf = (app: string): string[] | null => {
  const manual = manuals([app])[0];
  return manual ? Object.keys(manual.verbs).map(bare) : null;
};

const appOf = (ctx: Ctx, name: string) => {
  if (ctx.read(name) !== null) return name;
  const apps = Object.keys((ctx.read("") as Record<string, Json>) ?? {}).sort();
  return ctx.refuse(`no app ${name} in this world; one of: ${apps.join(", ")}`);
};

const verbOf = (ctx: Ctx, app: string, verb: string) => {
  const known = verbsOf(app);
  if (!known || known.includes(verb)) return verb;
  if (!known.length) return ctx.refuse(`${app} has no verbs; leave verb out`);
  return ctx.refuse(`${app} has no verb ${verb}; one of: ${known.join(", ")}`);
};

/* SCHEMAS */

const sending: Schema = {
  type: "object",
  properties: {
    app: { type: "string", minLength: 1, maxLength: NAME },
    verb: { type: "string", minLength: 1, maxLength: NAME },
    happened: { type: "string", minLength: 1, maxLength: TEXT },
    expected: { type: "string", maxLength: TEXT },
    severity: { enum: SEVERITIES },
  },
  required: ["app", "happened", "severity"],
  additionalProperties: false,
};

const naming = (state: Feedback): Schema => ({
  type: "object",
  properties: { id: { type: "integer", minimum: 1, maximum: Math.max(1, state.next - 1) } },
  required: ["id"],
  additionalProperties: false,
});

/* APP */

export default app<Feedback>({
  id: "feedback",
  title: "Feedback",
  line: "What broke and what you expected, for the makers; never voted, never capped by others.",
  boot: () => ({ next: 1, items: {} }),
  verbs: {
    send: {
      line: `Tell the makers what happened in an app (and what you expected): 1 to ${TEXT} characters, a severity of ${SEVERITIES.join(", ")}; ${KEEP} items each.`,
      args: sending,
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const happened = String(args.happened).trim();
        if (!happened) return ctx.refuse("happened is only spaces; say what happened");
        if (sent(state, me).length >= KEEP)
          return ctx.refuse(`you sent ${KEEP} items of feedback; the makers have them`);
        const name = appOf(ctx, String(args.app));
        const verb = args.verb === undefined ? null : verbOf(ctx, name, String(args.verb));
        const expected = args.expected === undefined ? null : String(args.expected).trim() || null;
        const item: Item = {
          id: state.next,
          by: me,
          app: name,
          verb,
          happened,
          expected,
          severity: args.severity as Severity,
          at: ctx.at,
          same: [],
        };
        return { next: state.next + 1, items: { ...state.items, [item.id]: item } };
      },
    },
    same: {
      line: "Say the same happened to you, on someone else's item, once.",
      args: naming,
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const one = state.items[String(args.id)]!;
        if (one.by === me) return ctx.refuse(`you sent item ${one.id}; others say the same`);
        if (one.same.includes(me)) return ctx.refuse(`you already said the same of item ${one.id}`);
        return { ...state, items: { ...state.items, [one.id]: { ...one, same: [...one.same, me] } } };
      },
    },
  },
  moves: (state) => (state.next > 1 ? ["send", "same"] : ["send"]),
  read: (state, path) => {
    const [head = "", ...rest] = path;
    if (!head) return { items: all(state) };
    if (head.startsWith(SLICE)) return walk(mine(state, head.slice(SLICE.length)), rest);
    return walk(about(state, head), rest);
  },
  bot,
});
