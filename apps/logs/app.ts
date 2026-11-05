import type { Json, Line, Obj, ViewNode, Who } from "@mrlyprod/os/contract/index.ts";
import { app, sliced } from "@mrlyprod/os/contract/index.ts";

type Acted = { tick: number; at: number; verb: string; args: Obj };

type Logs = { keep: number; acts: Record<string, Acted[]> };

const MOST = 10000;

const held = (state: Logs, id: string) => (Object.hasOwn(state.acts, id) ? state.acts[id]! : []);

const own = (state: Logs, id: string, who: Who | null): Json | undefined =>
  who?.id === id ? held(state, id) : undefined;

const logged = (state: Logs, { who, tick, at, verb, args }: Line): Logs => ({
  ...state,
  acts: { ...state.acts, [who.id]: [{ tick, at, verb, args }, ...held(state, who.id)].slice(0, state.keep) },
});

const view: ViewNode[] = [
  {
    kind: "rows",
    from: { world: "logs" },
    fields: [
      { key: "at", as: "time", title: "time" },
      { key: "verb", as: "text" },
      { key: "args", as: "text" },
    ],
  },
];

export default app<Logs>({
  id: "logs",
  view,
  title: "Logs",
  line: "Your own acts, newest first.",
  knobs: {
    keep: {
      line: "Acts one person's log keeps.",
      schema: { type: "integer", minimum: 1, maximum: MOST },
      reality: 200,
    },
  },
  boot: ({ knob }) => ({ keep: knob("keep"), acts: {} }),
  verbs: {
    clear: {
      line: "Empty your own log.",
      guest: true,
      run: (state, _args, ctx) => {
        const { [ctx.who.id]: _gone, ...acts } = state.acts;
        return { ...state, acts };
      },
    },
  },
  hear: (state, line) => (line.via || line.verb.startsWith("logs.") ? state : logged(state, line)),
  read: sliced(own, (state, who) => (who ? held(state, who.id) : [])),
});
