// A whole app: one folder under a root in site.json, holding this app.ts.
// Boot it with no shelf at all, from os/: bun run hello
// Then type: /hello greet name=Ada
import type { Who } from "@mrlyprod/os/contract/index.ts";
import { app, REPLY, sliced } from "@mrlyprod/os/contract/index.ts";

// The state is plain JSON. Replaying the journal rebuilds it, so a reducer never reads a clock or a random source.
type Hello = { said: Record<string, string[]>; fortunes: Record<string, string> };

// What a person sees. `sliced` adds the `hello/@me` paths, and hides other people's slices unless you allow them.
const mine = (state: Hello, id: string, _who: Who | null) => ({
  greeted: state.said[id] ?? [],
  fortune: state.fortunes[id] ?? null,
});

export default app<Hello>({
  // The id must match the folder name. It also names the verbs: hello.greet.
  id: "hello",
  title: "Hello",
  line: "Greet someone, see who you have greeted, and ask the outside world for a fortune.",

  // A knob shapes a world. A config turns it with the rule hello.most; reality fills it when the config is silent.
  knobs: {
    most: { line: "Names one person may greet.", schema: { type: "integer", minimum: 1, maximum: 100 }, reality: 10 },
  },

  // Runs once, when a world boots.
  boot: () => ({ said: {}, fortunes: {} }),

  verbs: {
    greet: {
      // The line is the manual: people, agents and every door read it. It may read knobs, never state.
      line: (knob) => `Greet someone by name; you greet ${knob("most")} at most.`,

      // A JSON schema. The kernel refuses bad args before run is called, with a note that says why.
      args: {
        type: "object",
        required: ["name"],
        additionalProperties: false,
        properties: { name: { type: "string", minLength: 1, maxLength: 40 } },
      },

      // A pure reducer: old state and an act in, new state out.
      // It reaches the world only through ctx: who acts, the time, knobs, the result to say, a refusal.
      run: (state, args, ctx) => {
        const name = String(args.name);
        const had = state.said[ctx.who.id] ?? [];
        if (name === ctx.who.id) return ctx.refuse("greet someone else");
        if (had.length >= ctx.knob("most")) return ctx.refuse(`you greeted ${ctx.knob("most")} already`);
        ctx.say({ note: `Hello, ${name}!` });
        return { ...state, said: { ...state.said, [ctx.who.id]: [...had, name] } };
      },
    },

    fortune: {
      line: "Ask the outside world for your fortune; it arrives later, in hello/@me.",
      // A wish leaves the world. The host keeps it in outbox.jsonl, runs the instance's handler for its kind,
      // and answers with an act on reply. A world with no handler answers { ok: false }.
      run: (state, _args, ctx) => {
        ctx.wish({ kind: "fortune", data: { for: ctx.who.id }, reply: "hello.told" });
        ctx.say({ note: "Asked; read hello/@me soon" });
        return { ...state, fortunes: { ...state.fortunes, [ctx.who.id]: "waiting" } };
      },
    },

    told: {
      // inner: only an app or the host may run it. The host runs it as the one who wished, with via "host".
      inner: true,
      line: "The outside world's answer to hello.fortune.",
      // A reply's args: the wish's id, whether it worked, and what came back.
      args: REPLY,
      run: (state, args, ctx) => {
        const data = args.data as { text?: string } | undefined;
        const text = args.ok === true && data?.text ? data.text : "the stars are silent";
        return { ...state, fortunes: { ...state.fortunes, [ctx.who.id]: text } };
      },
    },
  },

  // read answers `hello`, `hello/@me` and every path under them.
  read: sliced(mine),
});
