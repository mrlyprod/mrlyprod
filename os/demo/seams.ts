// The instance's handlers for wishes that leave the world, one per kind; site.json names this file.
// A real one would call a service, passing key so a retry never does the work twice.
import type { Handlers } from "@mrlyprod/os/hosts/bus.ts";

const FORTUNES = ["A letter is on its way.", "Someone is thinking of you.", "Today, say yes."];

export default {
  // Return { ok, data } now, or a promise of it. Throw, and the wish stays in the outbox: the next start retries it.
  fortune: (wish, key) => {
    if (process.env.HELLO_OFFLINE) throw new Error("the fortune teller is out");
    return { ok: true, data: { text: FORTUNES[key.length % FORTUNES.length], for: String(wish.who.id) } };
  },
} satisfies Handlers;
