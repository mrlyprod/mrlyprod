import type { Json, Knob, Knobs, Obj, Schema, Who } from "@mrlyprod/os/contract/index.ts";
import { app, appOf, asMe, canon, sliced } from "@mrlyprod/os/contract/index.ts";

export type Call = "list" | "read" | "call";

export type Typed = { line: string; call: Call; result: Json; at: number };

export type Term = { history: Record<string, Typed[]> };

const ID = "term";

export const USAGE = "a line is list, read <path> or call <verb> {json}";

const KNOBS = {
  line: {
    line: "Characters one typed line may hold.",
    schema: { type: "integer", minimum: 20, maximum: 5000 },
    reality: 500,
  },
  keep: {
    line: "Typed lines one person's history keeps.",
    schema: { type: "integer", minimum: 1, maximum: 1000 },
    reality: 50,
  },
  result: {
    line: "Characters of a read's result the terminal keeps.",
    schema: { type: "integer", minimum: 100, maximum: 20000 },
    reality: 1500,
  },
} satisfies Knobs;

const capped = (value: Json, size: number): Json => {
  const text = canon(value);
  return text.length > size ? `${text.slice(0, size - 3)}...` : value;
};

const split = (line: string): [string, string] => {
  const cut = line.search(/\s/);
  return cut < 0 ? [line, ""] : [line.slice(0, cut), line.slice(cut).trim()];
};

const typing = (knob: Knob): Schema => ({
  type: "object",
  properties: { line: { type: "string", minLength: 1, maxLength: knob("line") } },
  required: ["line"],
  additionalProperties: false,
});

const lastOf = (state: Term, id: string) => state.history[id]?.at(-1) ?? null;

const mine = (state: Term, id: string) => ({
  usage: USAGE,
  last: lastOf(state, id),
  history: [...(state.history[id] ?? [])].reverse(),
});

const whole = (state: Term, who: Who | null): Json =>
  who
    ? {
        usage: USAGE,
        last: lastOf(state, who.id),
        lines: state.history[who.id]?.length ?? 0,
        more: "term/@me holds every line you kept, with its result",
      }
    : { usage: USAGE };

export default app<Term>({
  id: ID,
  title: "Terminal",
  line: "The calls as typed text: list, read <path>, call <verb> {json}; reads show only what you may see, and a way for a human to feel like a robot.",
  boot: () => ({ history: {} }),
  verbs: {
    type: {
      line: (knob) =>
        `Type one line of up to ${knob("line")} characters; ${USAGE}. Your last ${knob("keep")} lines and their results are kept, privately.`,
      args: typing,
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const line = String(args.line).trim();
        const [word, rest] = split(line);
        const world = (ctx.read("") ?? {}) as Obj;
        let call: Call;
        let result: Json;
        if (word === "list") {
          call = "list";
          result = { apps: Object.keys(world), verbs: "each app's verbs are the door's tools; read help for more" };
        } else if (word === "read") {
          const seen = ctx.see(rest);
          if (seen === null)
            return ctx.refuse(`nothing you may see at ${rest}; read an app first, then a key inside it`);
          call = "read";
          result = capped(seen, ctx.knob("result"));
        } else if (word === "call") {
          const [verb, json] = split(rest);
          if (!verb.includes(".")) return ctx.refuse("call takes a verb like messages.send, then its args as JSON");
          const id = appOf(verb);
          if (!Object.hasOwn(world, id)) return ctx.refuse(`no app ${id} in this world; list shows them`);
          let parsed: Json;
          try {
            parsed = json ? (JSON.parse(json) as Json) : {};
          } catch {
            return ctx.refuse(`args are JSON like {"to":"test-2","text":"hi"}, not ${json}`);
          }
          if (!parsed || typeof parsed !== "object" || Array.isArray(parsed))
            return ctx.refuse("args are a JSON object");
          ctx.wish(asMe(verb, parsed as Obj));
          call = "call";
          result = `asked ${verb} as ${me}; it may still be refused, so read ${id} to see the outcome`;
        } else if (word === "boot") return ctx.refuse(`a world boots from outside; ${USAGE}`);
        else return ctx.refuse(`${USAGE}, not ${word}`);
        const kept = [...(state.history[me] ?? []), { line, call, result, at: ctx.at }].slice(-ctx.knob("keep"));
        ctx.say({ note: line, result });
        return { history: { ...state.history, [me]: kept } };
      },
    },
  },
  read: sliced(mine, whole),
  knobs: KNOBS,
});
