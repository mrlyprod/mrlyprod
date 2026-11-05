import type { Ctx, Json, Knobs, Obj, Schema, Who } from "@mrlyprod/os/contract/index.ts";
import { app, sliced, walk } from "@mrlyprod/os/contract/index.ts";
import { notice, reach, someone } from "../shared/people.ts";
import { allowed, pay } from "../wallet/app.ts";

export const HOUSE = "jobs";

export type Status = "open" | "hired" | "done" | "paid" | "cancelled";

export type Applicant = { by: string; line: string; at: number };

export type Job = {
  id: number;
  title: string;
  about: string;
  pay: number;
  by: string;
  org: string | null;
  at: number;
  status: Status;
  worker: string | null;
  applicants: Applicant[];
  closed: number | null;
};

export type Jobs = { people: string[] | null; next: number; jobs: Record<string, Job> };

const KNOBS = {
  live: {
    line: "Jobs one person may have posted that are not yet paid or cancelled; 0 means nobody posts.",
    schema: { type: "integer", minimum: 0, maximum: 1000 },
    reality: 5,
  },
  working: {
    line: "Jobs one person may work at once.",
    schema: { type: "integer", minimum: 1, maximum: 1000 },
    reality: 3,
  },
  title: { line: "Characters in a job's title.", schema: { type: "integer", minimum: 8, maximum: 200 }, reality: 60 },
  about: {
    line: "Characters in what a job asks for.",
    schema: { type: "integer", minimum: 16, maximum: 5000 },
    reality: 500,
  },
  line: {
    line: "Characters in an application.",
    schema: { type: "integer", minimum: 8, maximum: 2000 },
    reality: 200,
  },
  most: {
    line: "Most coins one job pays.",
    schema: { type: "integer", minimum: 1, maximum: 1_000_000_000 },
    reality: 100_000,
  },
  applicants: {
    line: "Applications one job keeps; later ones are refused.",
    schema: { type: "integer", minimum: 1, maximum: 1000 },
    reality: 20,
  },
  orgs: {
    line: "Whether a member may post a job in the name of an organisation they belong to.",
    schema: { type: "boolean" },
    reality: true,
  },
  listed: {
    line: "Newest open jobs one overview lists.",
    schema: { type: "integer", minimum: 1, maximum: 500 },
    reality: 20,
  },
} satisfies Knobs;

type Group = { n: number; name: string; dissolved: number | null; members: Record<string, string> };

const PERSON = 40;

const LIVE: Status[] = ["open", "hired", "done"];

const jobOf = (state: Jobs, id: unknown, ctx: Ctx): Job => {
  const key = String(id);
  if (Object.hasOwn(state.jobs, key)) return state.jobs[key]!;
  return ctx.refuse(`no job ${key}; read jobs for the open ones`);
};

const live = (state: Jobs, id: string) =>
  Object.values(state.jobs).filter((one) => one.by === id && LIVE.includes(one.status));

const working = (state: Jobs, id: string) =>
  Object.values(state.jobs).filter((one) => one.worker === id && (one.status === "hired" || one.status === "done"));

const applied = (state: Jobs, id: string) =>
  Object.values(state.jobs).filter((one) => one.status === "open" && one.applicants.some((each) => each.by === id));

const opened = (state: Jobs) =>
  Object.values(state.jobs)
    .filter((one) => one.status === "open")
    .sort((a, b) => b.at - a.at || b.id - a.id);

const card = (one: Job) => ({
  id: one.id,
  title: one.title,
  pay: one.pay,
  by: one.by,
  org: one.org,
  status: one.status,
  worker: one.worker,
  applicants: one.applicants.length,
  at: one.at,
});

const detail = (one: Job, who: Who | null): Obj => ({
  ...card(one),
  about: one.about,
  ...(who && who.id === one.by ? { applications: one.applicants } : {}),
  ...(who && one.applicants.some((each) => each.by === who.id)
    ? { applied: one.applicants.find((each) => each.by === who.id)!.line }
    : {}),
});

const see =
  "jobs/open lists open jobs, jobs/job/<id> shows one, jobs.apply to apply, jobs.post to hire someone yourself.";

const mine = (state: Jobs, id: string, _who: Who | null) => ({
  posted: Object.values(state.jobs)
    .filter((one) => one.by === id)
    .sort((a, b) => b.id - a.id)
    .map((one) => ({ ...card(one), applications: one.applicants })),
  working: working(state, id).map(card),
  applied: applied(state, id).map(card),
  open: opened(state)
    .filter((one) => one.by !== id)
    .slice(0, 6)
    .map(card),
  see,
});

const whole = (state: Jobs, who: Who | null): Json =>
  who ? mine(state, who.id, who) : { open: opened(state).slice(0, 6).map(card), see };

const view = sliced<Jobs>(mine, whole);

function read(state: Jobs, path: string[], who: Who | null, at = 0): Json | undefined {
  const [head, one, ...rest] = path;
  if (head === "open") return walk(opened(state).map(card), path.slice(1));
  if (head === "job") {
    if (!one) return Object.fromEntries(Object.values(state.jobs).map((each) => [each.id, card(each)]));
    return Object.hasOwn(state.jobs, one) ? walk(detail(state.jobs[one]!, who), rest) : undefined;
  }
  return view(state, path, who, at);
}

function groupOf(ctx: Ctx, org: string): Group {
  const me = ctx.who.id;
  const groups = Object.values((ctx.read("people/orgs") ?? {}) as Record<string, Group>);
  const named = groups.filter((one) => one.dissolved === null && (one.name === org || String(one.n) === org));
  return (
    named.find((one) => Object.hasOwn(one.members, me)) ??
    ctx.refuse(`you do not belong to an organisation called ${org}; people/orgs lists them`)
  );
}

const shaped = (properties: Record<string, Schema>, required: string[]): Schema => ({
  type: "object",
  properties,
  required,
  additionalProperties: false,
});

const naming = (more: Record<string, Schema> = {}): Schema =>
  shaped({ job: { type: "integer", minimum: 1 }, ...more }, ["job", ...Object.keys(more)]);

const person: Schema = { type: "string", minLength: 1, maxLength: PERSON };

const put = (state: Jobs, one: Job): Jobs => ({ ...state, jobs: { ...state.jobs, [one.id]: one } });

const tell = (ctx: Ctx, people: string[], text: string) => {
  for (const who of new Set(people)) if (who !== ctx.who.id) ctx.wish(notice(who, text));
};

const own = (one: Job, ctx: Ctx, what: string) => {
  if (one.by !== ctx.who.id) ctx.refuse(`job ${one.id} is ${one.by}'s; only they ${what}`);
};

const stage = (one: Job, ctx: Ctx, want: Status, next: string) => {
  if (one.status !== want) ctx.refuse(`job ${one.id} is ${one.status}, not ${want}; ${next}`);
};

export default app<Jobs>({
  id: "jobs",
  title: "Jobs",
  line: "Paid work between people: post a job with its pay held in escrow, others apply, you hire one, they finish, you pay. Read jobs/open for work to take.",
  knobs: KNOBS,
  boot: ({ actors }) => ({ people: actors, next: 1, jobs: {} }),
  verbs: {
    post: {
      line: (knob) =>
        `Post a job and hold its pay: a title of 1 to ${knob("title")} characters, what you need in up to ${knob("about")}, and a whole pay of 1 to ${knob("most")} that leaves your wallet now and waits for the worker. jobs.cancel brings it back until you hire someone. You hold at most ${knob("live")} unpaid jobs.${knob<boolean>("orgs") ? " Add org, an organisation you belong to, to post in its name; the pay is still yours." : ""}`,
      args: (knob) =>
        shaped(
          {
            title: { type: "string", minLength: 1, maxLength: knob("title") },
            about: { type: "string", maxLength: knob("about") },
            pay: { type: "integer", minimum: 1, maximum: knob("most") },
            org: { type: "string", minLength: 1, maxLength: 80 },
          },
          ["title", "pay"],
        ),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const title = String(args.title).trim();
        const most = ctx.knob("live");
        if (!title) return ctx.refuse("the title is only spaces; name the job");
        if (!allowed(ctx.knob, "payments"))
          return ctx.refuse("payments between people are off in this world, so nobody could be paid for a job");
        if (live(state, me).length >= most)
          return ctx.refuse(
            most
              ? `${most} unpaid jobs is the most you may hold; jobs.pay or jobs.cancel one first`
              : "nobody posts jobs in this world",
          );
        let org: string | null = null;
        if (args.org !== undefined) {
          if (!ctx.knob("orgs")) return ctx.refuse("organisations do not post jobs in this world; post as yourself");
          org = groupOf(ctx, String(args.org).trim()).name;
        }
        const id = state.next;
        const amount = Number(args.pay);
        pay(ctx, HOUSE, amount);
        const one: Job = {
          id,
          title,
          about: String(args.about ?? "").trim(),
          pay: amount,
          by: me,
          org,
          at: ctx.at,
          status: "open",
          worker: null,
          applicants: [],
          closed: null,
        };
        ctx.say({
          note: `Posted job ${id}: ${title}${org ? ` for ${org}` : ""}; ${amount} is held until you jobs.pay or jobs.cancel`,
          id,
          pay: amount,
        });
        return { ...put(state, one), next: id + 1 };
      },
    },
    apply: {
      line: (knob) =>
        `Apply for an open job with a line of 1 to ${knob("line")} characters on why you; applying again replaces it. The poster hears.`,
      args: (knob) => naming({ line: { type: "string", minLength: 1, maxLength: knob("line") } }),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const one = jobOf(state, args.job, ctx);
        const line = String(args.line).trim();
        if (!line) return ctx.refuse("the line is only spaces; say why you");
        if (one.by === me) return ctx.refuse(`job ${one.id} is yours; jobs.hire someone from its applications`);
        stage(one, ctx, "open", "read jobs/open for jobs still looking");
        reach(ctx, one.by);
        const had = one.applicants.some((each) => each.by === me);
        if (!had && one.applicants.length >= ctx.knob("applicants"))
          return ctx.refuse(`job ${one.id} has ${ctx.knob("applicants")} applications, its most; try another job`);
        const applicants = [...one.applicants.filter((each) => each.by !== me), { by: me, line, at: ctx.at }];
        tell(ctx, [one.by], `${me} applied for ${one.title} (job ${one.id}): ${line}`);
        ctx.say({ note: `${had ? "Replaced your application for" : "Applied for"} ${one.title}`, id: one.id });
        return put(state, { ...one, applicants });
      },
    },
    hire: {
      line: "Hire one of the people who applied for your open job. They hear, the others hear it is filled, and the pay stays held until you jobs.pay.",
      args: naming({ person }),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const one = jobOf(state, args.job, ctx);
        own(one, ctx, "hire");
        stage(one, ctx, "open", "only an open job takes a hire");
        const who = someone(state.people, String(args.person), ctx);
        if (!one.applicants.some((each) => each.by === who))
          return ctx.refuse(
            `${who} did not apply for job ${one.id}; ${one.applicants.length ? `applicants: ${one.applicants.map((each) => each.by).join(", ")}` : "nobody applied yet"}`,
          );
        const most = ctx.knob("working");
        if (working(state, who).length >= most)
          return ctx.refuse(`${who} works ${most} jobs already, the most at once; hire someone else or wait`);
        tell(
          ctx,
          [who],
          `${me} hired you for ${one.title} (job ${one.id}), ${one.pay} coins: jobs.done when it is finished`,
        );
        tell(
          ctx,
          one.applicants.map((each) => each.by).filter((each) => each !== who),
          `${one.title} (job ${one.id}) went to ${who}`,
        );
        ctx.say({ note: `Hired ${who} for ${one.title}; ${one.pay} stays held`, id: one.id, worker: who });
        return put(state, { ...one, status: "hired", worker: who });
      },
    },
    done: {
      line: "Say the job you were hired for is finished. The poster hears and pays with jobs.pay.",
      args: naming(),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const one = jobOf(state, args.job, ctx);
        if (one.worker !== me)
          return ctx.refuse(`you are not hired for job ${one.id}; jobs/@me lists the jobs you work`);
        stage(one, ctx, "hired", "it is waiting for the poster to pay");
        tell(
          ctx,
          [one.by],
          `${me} finished ${one.title} (job ${one.id}): jobs.pay job ${one.id} to release ${one.pay}`,
        );
        ctx.say({ note: `Marked ${one.title} done; ${one.by} pays you ${one.pay}`, id: one.id });
        return put(state, { ...one, status: "done" });
      },
    },
    quit: {
      line: "Give up a job you were hired for before it is done. It opens again for other applicants and the poster hears.",
      args: naming(),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const one = jobOf(state, args.job, ctx);
        if (one.worker !== me)
          return ctx.refuse(`you are not hired for job ${one.id}; jobs/@me lists the jobs you work`);
        stage(one, ctx, "hired", "a finished job waits for the poster to pay");
        tell(ctx, [one.by], `${me} quit ${one.title} (job ${one.id}); it is open again`);
        ctx.say({ note: `You left ${one.title}; it is open again`, id: one.id });
        return put(state, {
          ...one,
          status: "open",
          worker: null,
          applicants: one.applicants.filter((each) => each.by !== me),
        });
      },
    },
    pay: {
      line: "Pay the worker of your finished job: the held coins go to them now. They hear.",
      args: naming(),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const one = jobOf(state, args.job, ctx);
        own(one, ctx, "pay");
        if (one.status === "hired")
          return ctx.refuse(`${one.worker} has not said job ${one.id} is done; wait for jobs.done`);
        if (one.status === "open")
          return ctx.refuse(`nobody is hired for job ${one.id} yet; jobs.hire an applicant first`);
        stage(one, ctx, "done", "it is closed already");
        pay(ctx, me, one.pay, HOUSE);
        pay(ctx, one.worker!, one.pay);
        tell(ctx, [one.worker!], `${me} paid you ${one.pay} for ${one.title} (job ${one.id})`);
        ctx.say({
          note: `Paid ${one.worker} ${one.pay} for ${one.title}`,
          id: one.id,
          worker: one.worker,
          pay: one.pay,
        });
        return put(state, { ...one, status: "paid", closed: ctx.at });
      },
    },
    cancel: {
      line: "Cancel your job while nobody is hired: the held coins come back to you and the applicants hear.",
      args: naming(),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const one = jobOf(state, args.job, ctx);
        own(one, ctx, "cancel");
        stage(one, ctx, "open", "only a job with nobody hired is cancelled; a hired one ends with jobs.pay");
        pay(ctx, me, one.pay, HOUSE);
        tell(
          ctx,
          one.applicants.map((each) => each.by),
          `${one.title} (job ${one.id}) was cancelled`,
        );
        ctx.say({ note: `Cancelled ${one.title}; ${one.pay} came back to you`, id: one.id, pay: one.pay });
        return put(state, { ...one, status: "cancelled", closed: ctx.at });
      },
    },
  },
  moves: (state) => (state.next > 1 ? ["post", "apply", "hire", "done", "quit", "pay", "cancel"] : ["post"]),
  read,
});
