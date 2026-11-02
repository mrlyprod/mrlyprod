import type { Json, Obj, Schema } from "../../sdk/system.ts";
import { act, app, canon, manuals, sliced } from "../../sdk/system.ts";
import bot from "./bot.ts";

/* TYPES */

export type Call = "list" | "read" | "call";

export type Typed = { line: string; call: Call; result: Json; at: number };

export type Term = { history: Record<string, Typed[]> };

export const LINE = 500;

export const KEEP = 50;

export const RESULT = 2000;

export const USAGE = "type list, read <path> or call <verb> {json}";

/* LINES */

const capped = (value: Json): Json => {
  const text = canon(value);
  return text.length > RESULT ? `${text.slice(0, RESULT - 3)}...` : value;
};

const split = (line: string): [string, string] => {
  const cut = line.search(/\s/);
  return cut < 0 ? [line, ""] : [line.slice(0, cut), line.slice(cut).trim()];
};

/* SCHEMAS */

const typing: Schema = {
  type: "object",
  properties: { line: { type: "string", minLength: 1, maxLength: LINE } },
  required: ["line"],
  additionalProperties: false,
};

/* APP */

export default app<Term>({
  id: "term",
  title: "Terminal",
  line: "The four calls as typed text: list, read <path>, call <verb> {json}; a way for a human to feel like a robot.",
  boot: () => ({ history: {} }),
  verbs: {
    type: {
      line: `Type one line of up to ${LINE} characters: ${USAGE}; your last ${KEEP} lines and their results are kept.`,
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
          result = Object.fromEntries(manuals(Object.keys(world)).map((one) => [one.id, Object.keys(one.verbs)]));
        } else if (word === "read") {
          call = "read";
          result = capped(ctx.read(rest));
        } else if (word === "call") {
          const [verb, json] = split(rest);
          if (!verb.includes(".")) return ctx.refuse(`call takes a verb like messages.send, then its args as JSON`);
          const head = verb.slice(0, verb.lastIndexOf("."));
          const id = head.slice(head.lastIndexOf(".") + 1);
          if (!(id in world)) return ctx.refuse(`no app ${id} in this world; type list to see them`);
          let parsed: Json;
          try {
            parsed = json ? (JSON.parse(json) as Json) : {};
          } catch {
            return ctx.refuse(`args are JSON like {"to":"test-2","text":"hi"}, not ${json}`);
          }
          if (!parsed || typeof parsed !== "object" || Array.isArray(parsed))
            return ctx.refuse("args are a JSON object");
          ctx.wish(act(verb, parsed as Obj));
          call = "call";
          result = `sent ${verb} as ${me}`;
        } else if (word === "boot") return ctx.refuse(`a world boots from outside; ${USAGE}`);
        else return ctx.refuse(`${USAGE}, not ${word}`);
        const mine = [...(state.history[me] ?? []), { line, call, result, at: ctx.at }].slice(-KEEP);
        return { history: { ...state.history, [me]: mine } };
      },
    },
  },
  read: sliced((state: Term, me: string) => state.history[me] ?? []),
  bot,
});
