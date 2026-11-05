import { app } from "@mrlyprod/os/contract/index.ts";

export type Dawn = { days: number };

export default app<Dawn>({
  id: "dawn",
  title: "Dawn",
  line: "The day a simulated world is on, marked by its harness.",
  boot: () => ({ days: 0 }),
  verbs: {
    rise: {
      line: "For the harness only: a new day begins.",
      args: { type: "object", additionalProperties: false },
      run: (state, _args, ctx) => {
        if (!ctx.via) return ctx.refuse("only the harness starts a new day; the day comes by itself");
        return { days: state.days + 1 };
      },
    },
  },
});
