import type { Jobs } from "@mrlyprod/apps/jobs/app.ts";
import { allowed } from "@mrlyprod/apps/wallet/app.ts";
import { below, pick } from "@mrlyprod/os/contract/index.ts";
import { reader } from "@mrlyprod/os/kernel/index.ts";
import type { Bot, Step } from "./types.ts";

type Bank = { balances?: Record<string, number> };

const TITLES = [
  "Carry water to the mill",
  "Copy out a letter",
  "Paint the bakery sign",
  "Walk a parcel to the harbour",
  "Mend a fence",
  "Draw a map of the hill",
  "Cook for ten",
];

const ABOUT = ["Needed by the weekend.", "Easy work, fair pay.", "Bring your own tools.", "Ask me for details."];

const LINES = ["I can start today.", "I have done this before.", "Happy to help, fair pay.", "Reliable and quick."];

const LIVE = 2;

const bot: Bot<Jobs> = ({ state, me, world, draw }): Step => {
  const rules = reader(world.rules, "jobs");
  const jobs = Object.values(state.jobs);
  const posted = jobs.filter((one) => one.by === me);
  const cash = (world.apps.wallet as unknown as Bank | undefined)?.balances?.[me] ?? 0;
  const load = (id: string) => jobs.filter((one) => one.worker === id && one.status !== "paid").length;

  const unpaid = posted.find((one) => one.status === "done");
  if (unpaid) return { verb: "pay", args: { job: unpaid.id } };

  const hiring = posted.find((one) => one.status === "open" && one.applicants.length);
  if (hiring && below(draw, 2)) {
    const free = hiring.applicants.filter((each) => load(each.by) < rules<number>("working"));
    if (free.length) return { verb: "hire", args: { job: hiring.id, person: pick(draw, free).by } };
  }

  const hired = jobs.filter((one) => one.worker === me && one.status === "hired");
  if (hired.length && below(draw, 3))
    return { verb: below(draw, 8) ? "done" : "quit", args: { job: pick(draw, hired).id } };

  const idle = posted.filter((one) => one.status === "open" && !one.applicants.length);
  if (idle.length && !below(draw, 6)) return { verb: "cancel", args: { job: pick(draw, idle).id } };

  const wanted = jobs.filter(
    (one) => one.status === "open" && one.by !== me && !one.applicants.some((each) => each.by === me),
  );
  if (wanted.length && below(draw, 2))
    return { verb: "apply", args: { job: pick(draw, wanted).id, line: pick(draw, LINES) } };

  const open = posted.filter((one) => one.status !== "paid" && one.status !== "cancelled").length;
  if (open < Math.min(LIVE, rules<number>("live")) && allowed(reader(world.rules, "wallet"), "payments") && cash > 6)
    return {
      verb: "post",
      args: { title: pick(draw, TITLES), about: pick(draw, ABOUT), pay: 5 + below(draw, Math.min(25, cash - 5)) },
    };
  return null;
};

export default bot;
