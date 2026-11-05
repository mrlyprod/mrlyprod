#!/usr/bin/env bun
import { appendFileSync, cpSync, existsSync, mkdirSync, readdirSync, readFileSync, writeFileSync } from "node:fs";
import { join, relative } from "node:path";
import { install } from "@mrlyprod/os/hosts/local/apps.ts";
import { boot, configOf, exists, open } from "@mrlyprod/os/hosts/local/store.ts";
import type { Entry, Obj } from "@mrlyprod/os/kernel/index.ts";
import { nearest } from "@mrlyprod/os/kernel/index.ts";
import type { Made } from "../made.ts";
import { madeNow } from "../made.ts";
import { MEASURES, measuresOf } from "../measures.ts";
import { MINDS, minds, report, scanner } from "../report.ts";
import { generate, knobsOf } from "../worlds.ts";

const HERE = import.meta.dir;
const SIM = join(HERE, "..");
const REPO = join(SIM, "..");
const DATA_DIR = join(REPO, "data", relative(REPO, HERE));
const LEDGER = join(DATA_DIR, "ledger.tsv");
const KEPT = join(REPO, "local", "sim", "worlds");
const HEAD = "run\twake\tbot\tpersona\tmodel\teffort\tcost\tturns\tminutes\tacts\trefused\tstop";
const DOOR = Bun.resolveSync("@mrlyprod/os/doors/cli/cli.ts", HERE);
const HOUR = 3_600_000;
const DAY = 24 * HOUR;
const MORNING = 7 * HOUR;
const SPAN = 16 * HOUR;
const SLOWEST = 1000;
const FASTEST = 3_600_000;
const TOOLS = "mcp__mrlynet__";
const GRACE = 2000;
const DEFAULTS = {
  bots: "6",
  wakes: "1",
  acts: "30",
  budget: "5",
  limit: "189",
  cap: "1",
  jobs: "12",
  minutes: "25",
  world: "",
  seed: "",
  turn: "0",
  personas: "",
  models: "claude-sonnet-5-5",
  effort: "high",
  strong: "breaker,impostor",
  strongmodel: "claude-opus-5-5",
  rules: "",
  resume: "0",
  keep: "0",
  dry: "0",
};

type Bot = { id: string; persona: string; model: string; effort: string };

type Job = { bot: Bot; wake: number };

type Row = Job & { cost: number; turns: number; minutes: number; stop: string; summary: string; tools: string[] };

type Event = {
  type?: string;
  subtype?: string;
  apiKeySource?: string;
  tools?: string[];
  total_cost_usd?: number;
  num_turns?: number;
  result?: string;
};

const ask = { ...DEFAULTS };
for (const arg of process.argv.slice(2)) {
  const at = arg.indexOf("=");
  if (at < 0 || !(arg.slice(0, at) in ask)) throw new Error(`unknown setting ${arg}; settings: ${Object.keys(ask)}`);
  (ask as Record<string, string>)[arg.slice(0, at)] = arg.slice(at + 1);
}

const spentAll = () =>
  existsSync(LEDGER)
    ? readFileSync(LEDGER, "utf8")
        .trim()
        .split("\n")
        .slice(1)
        .reduce((sum, row) => sum + Number(row.split("\t")[6] ?? 0), 0)
    : 0;

const pick = <T>(list: T[], i: number): T => list[i % list.length]!;

const commas = (text: string) => text.split(",").filter(Boolean);

const dollars = (n: number) => `$${n.toFixed(2)}`;

function personas(): string[] {
  const all = readdirSync(join(HERE, "personas")).map((file) => file.replace(/\.md$/, ""));
  return ask.personas ? commas(ask.personas) : all.sort();
}

function bots(): Bot[] {
  const names = personas();
  const strong = commas(ask.strong);
  const unknown = strong.filter((one) => !readdirSync(join(HERE, "personas")).includes(`${one}.md`));
  if (unknown.length) throw new Error(`strong names no persona: ${unknown}`);
  return Array.from({ length: Number(ask.bots) }, (_, i) => {
    const persona = pick(names, i);
    const model = ask.strongmodel && strong.includes(persona) ? ask.strongmodel : pick(commas(ask.models), i);
    return { id: `bot-${i + 1}`, persona, model, effort: pick(commas(ask.effort), i) };
  });
}

function pace(count: number): number {
  const each = SPAN / Math.max(1, count * Number(ask.acts));
  return Math.max(SLOWEST, Math.min(FASTEST, Math.round(each)));
}

function fileRules(): Obj {
  if (!ask.rules) return {};
  const raw: unknown = JSON.parse(readFileSync(ask.rules, "utf8"));
  if (!raw || typeof raw !== "object" || Array.isArray(raw))
    throw new Error(`rules ${ask.rules} must be a JSON object`);
  return raw as Obj;
}

function known(rules: Obj, base: string[], names: string[], file: string) {
  const allowed = new Set([...names, ...base, "canon.not"]);
  allowed.delete("world.people");
  for (const key of Object.keys(rules)) {
    if (allowed.has(key)) continue;
    const near = nearest(key, [...allowed]);
    throw new Error(`unknown rule ${key} in ${file}${near ? `; did you mean ${near}?` : ""}`);
  }
}

const idOf = (one: string | Entry) => (typeof one === "string" ? one : one.id);

async function world(dir: string, list: Bot[]) {
  if (exists(dir)) return;
  const site = JSON.parse(readFileSync(join(SIM, "world.json"), "utf8")) as { apps: (string | Entry)[] };
  const apps = await install(site.apps.map(idOf), SIM);
  const seed = ask.seed ? Number(ask.seed) : Math.floor(Math.random() * 2 ** 32);
  const models = Object.fromEntries(list.map((bot) => [bot.id, bot.model]));
  const shape = { people: list.length, turn: Number(ask.turn), pace: pace(list.length) };
  const given = fileRules();
  const plain = generate(seed, apps, { ...shape, rules: { "identity.models": models } });
  known(
    given,
    Object.keys(plain.rules ?? {}),
    knobsOf(apps).map((one) => one.name),
    ask.rules,
  );
  const made = generate(seed, apps, { ...shape, rules: { "identity.models": models, ...given } });
  const entries = new Map(made.apps.map((one) => [idOf(one), one]));
  boot(apps, dir, { ...made, apps: site.apps.map((one) => entries.get(idOf(one)) ?? one) });
}

async function dawn(dir: string, list: Bot[]) {
  const store = open(await install(undefined, SIM), dir);
  const { at } = store.world();
  const today = Math.floor((at - MORNING) / DAY) * DAY + MORNING;
  const who = { id: list[0]!.id, kind: "bot" as const };
  store.call({ verb: "dawn.rise", args: {}, who, at: today + DAY, via: "testers" });
  return Number((store.world().apps.dawn as { days?: number } | undefined)?.days ?? 0);
}

async function remember(dir: string, job: Job, row: Row) {
  if (row.stop !== "success" || !row.summary) return;
  const store = open(await install(undefined, SIM), dir);
  const { apps, rules } = store.world();
  if (!("mind" in apps) || rules["mind.on"] === false) return;
  const diary = row.summary.slice(0, Number(rules["mind.diary"]));
  try {
    store.call({ verb: "mind.sleep", args: { diary }, who: { id: job.bot.id, kind: "bot" }, via: "testers" });
  } catch (thrown) {
    process.stderr.write(`no diary for ${job.bot.id}: ${String(thrown)}\n`);
  }
}

function agent(job: Job, dir: string, home: string) {
  const { bot } = job;
  const fill = (text: string) => text.replaceAll("{me}", bot.id).replaceAll("{acts}", ask.acts);
  const rules = fill(readFileSync(join(HERE, "prompt.md"), "utf8"));
  const persona = readFileSync(join(HERE, "personas", `${bot.persona}.md`), "utf8");
  const door = {
    command: "bun",
    args: [DOOR, "--world", dir, "--as", `${bot.id}:bot`, "mcp"],
    env: { MRLY_ROOT: SIM },
  };
  writeFileSync(join(home, "mcp.json"), JSON.stringify({ mcpServers: { mrlynet: door } }));
  return Bun.spawn(
    [
      "claude",
      "--restricted",
      "-p",
      "--no-session-persistence",
      "--strict-mcp-config",
      "--output-format",
      "stream-json",
      "--verbose",
      "--permission-mode",
      "dontAsk",
      "--model",
      bot.model,
      "--effort",
      bot.effort,
      "--max-budget-usd",
      ask.cap,
      "--tools",
      "",
      "--allowedTools",
      "mcp__mrlynet__*",
      "--mcp-config",
      join(home, "mcp.json"),
      "--system-prompt",
      `${rules}\n${persona}`,
    ],
    {
      cwd: home,
      stdin: new Blob([`Day ${job.wake}. You just woke up. Live your day as ${bot.id}.`]),
      stdout: "pipe",
      stderr: "pipe",
    },
  );
}

let halted: string | null = null;

const running = new Set<ReturnType<typeof agent>>();

function halt(reason: string) {
  halted ??= reason;
  process.stderr.write(`stopping: ${reason}\n`);
  for (const one of running) one.kill();
}

const wrong = (id: string, init: Event) => {
  if (init.apiKeySource !== "ANTHROPIC_API_KEY") return `${id} billed ${init.apiKeySource}, not the API key`;
  const hands = (init.tools ?? []).filter((one) => !one.startsWith(TOOLS));
  return hands.length ? `${id} holds tools it should not: ${hands}` : null;
};

async function stream(out: ReadableStream<Uint8Array>, kept: (text: string) => void, heard: (event: Event) => void) {
  const decoder = new TextDecoder();
  let text = "";
  let done = 0;
  for await (const chunk of out) {
    text += decoder.decode(chunk, { stream: true });
    kept(text);
    for (let end = text.indexOf("\n", done); end >= 0; end = text.indexOf("\n", done)) {
      const line = text.slice(done, end);
      done = end + 1;
      try {
        if (line) heard(JSON.parse(line));
      } catch (thrown) {
        if (!(thrown instanceof SyntaxError)) throw thrown;
      }
    }
  }
}

const failed = (job: Job, thrown: unknown, cost: number): Row => ({
  ...job,
  cost,
  turns: 0,
  minutes: 0,
  stop: `crash: ${String((thrown as Error)?.message ?? thrown).replaceAll(/\s+/g, " ")}`,
  summary: "",
  tools: [],
});

async function live(job: Job, dir: string, run: string): Promise<Row> {
  const home = join(run, job.bot.id, `wake-${job.wake}`);
  mkdirSync(home, { recursive: true });
  const started = Date.now();
  const cap = Number(ask.cap);
  const child = agent(job, dir, home);
  running.add(child);
  const killer = setTimeout(() => child.kill(), Number(ask.minutes) * 60_000);
  let init: Event | null = null;
  let done: Event | null = null;
  let barred = false;
  try {
    let out = "";
    let err = "";
    const reading = Promise.all([
      stream(
        child.stdout,
        (text) => {
          out = text;
        },
        (event) => {
          if (event.type === "result") done = event;
          if (event.type !== "system" || event.subtype !== "init") return;
          init = event;
          const why = wrong(job.bot.id, event);
          if (!why) return;
          barred = true;
          halt(why);
        },
      ),
      new Response(child.stderr).text().then((text) => {
        err = text;
      }),
    ]);
    await child.exited;
    await Promise.race([reading, Bun.sleep(GRACE)]);
    writeFileSync(join(home, "transcript.jsonl"), out);
    if (err) writeFileSync(join(home, "stderr.txt"), err);
    const end = done as Event | null;
    const held = init as Event | null;
    return {
      ...job,
      cost: end?.total_cost_usd ?? (held && !barred ? cap : 0),
      turns: end?.num_turns ?? 0,
      minutes: (Date.now() - started) / 60_000,
      stop: end?.subtype ?? (barred ? "barred at init" : `exit ${child.exitCode ?? child.signalCode}`),
      summary: String(end?.result ?? err.slice(0, 300)).trim(),
      tools: held?.tools ?? [],
    };
  } catch (thrown) {
    return failed(job, thrown, cap);
  } finally {
    clearTimeout(killer);
    child.kill();
    running.delete(child);
  }
}

async function main() {
  if (!process.env.ANTHROPIC_API_KEY && ask.dry !== "1") {
    process.stderr.write('no ANTHROPIC_API_KEY: run with bun --env-file="$MRLY_ENV"\n');
    process.exit(1);
  }
  mkdirSync(DATA_DIR, { recursive: true });
  if (!existsSync(LEDGER)) writeFileSync(LEDGER, `${HEAD}\n`);
  else if (readFileSync(LEDGER, "utf8").split("\n")[0] !== HEAD) throw new Error(`${LEDGER} has an unknown header`);
  const name = ask.world || new Date().toISOString().replace(/[-:T]/g, "").slice(0, 14);
  const run = join(DATA_DIR, name);
  const dir = join(run, "world");
  const resumed = existsSync(run);
  if (resumed && ask.resume !== "1") throw new Error(`world ${name} exists at ${run}; pass resume=1 to add to it`);
  if (resumed && ask.rules) throw new Error("rules are fixed when a world is made; resume=1 cannot change them");
  const list = bots();
  if (resumed && configOf(dir).actors?.length !== list.length)
    throw new Error(`world ${name} has ${configOf(dir).actors?.length} people; bots=${list.length}`);
  await world(dir, list);
  const apps = await install(undefined, SIM);
  keepMade(run, resumed);
  if (ask.dry === "1") {
    process.stdout.write(`dry: world ${name} at ${dir}; pace ${configOf(dir).rules?.["world.pace"]}\n`);
    return;
  }
  const { scan } = scanner(apps, dir);
  const budget = Number(ask.budget);
  const limit = Number(ask.limit);
  const cap = Number(ask.cap);
  const rows: Row[] = [];
  const seen = new Map<string, { acts: number; refused: number }>();
  let spent = 0;
  let reserved = 0;
  const room = () => !halted && spent + reserved + cap <= budget && spentAll() + reserved + cap <= limit;
  const tally = (id: string) => {
    try {
      const now = scan().tallies.get(id) ?? { acts: 0, refused: 0 };
      const was = seen.get(id) ?? { acts: 0, refused: 0 };
      seen.set(id, { acts: now.acts, refused: now.refused });
      return { acts: now.acts - was.acts, refused: now.refused - was.refused };
    } catch (thrown) {
      process.stderr.write(`no stats for ${id}: ${String(thrown)}\n`);
      return { acts: 0, refused: 0 };
    }
  };
  for (const [id, one] of scan().tallies) seen.set(id, { acts: one.acts, refused: one.refused });
  for (let day = 0; day < Number(ask.wakes) && room(); day++) {
    const wake = resumed || day > 0 ? (await dawn(dir, list)) + 1 : 1;
    const queue: Job[] = list.map((bot) => ({ bot, wake }));
    const worker = async () => {
      while (queue.length && room()) {
        const job = queue.shift()!;
        reserved += cap;
        const row = await live(job, dir, run).catch((thrown: unknown) => failed(job, thrown, 0));
        reserved -= cap;
        spent += row.cost;
        rows.push(row);
        await remember(dir, job, row);
        const stats = tally(job.bot.id);
        appendFileSync(
          LEDGER,
          `${[name, wake, job.bot.id, job.bot.persona, job.bot.model, job.bot.effort, row.cost.toFixed(4), row.turns, row.minutes.toFixed(1), stats.acts, stats.refused, row.stop].join("\t")}\n`,
        );
        process.stdout.write(
          `${job.bot.id} wake ${wake} ${job.bot.persona} ${job.bot.model} ${job.bot.effort}: ${dollars(row.cost)}, ${row.turns} turns, ${row.minutes.toFixed(1)} min, ${row.stop}\n`,
        );
      }
    };
    await Promise.all(Array.from({ length: Number(ask.jobs) }, worker));
  }
  writeFileSync(join(run, "report.md"), writeUp(name, dir, rows, spent, apps));
  process.stdout.write(`spent ${dollars(spent)}, all runs ${dollars(spentAll())}\nreport: ${join(run, "report.md")}\n`);
  if (ask.keep === "1" && !halted) {
    const kept = join(KEPT, name);
    cpSync(run, kept, { recursive: true, errorOnExist: true, force: false });
    process.stdout.write(`kept: ${kept}\n`);
  }
  process.exit(halted ? 1 : 0);
}

function keepMade(run: string, resumed: boolean) {
  const file = join(run, "made.json");
  const now = madeNow(SIM);
  if (!resumed || !existsSync(file)) return writeFileSync(file, `${JSON.stringify(now, null, 2)}\n`);
  const was = JSON.parse(readFileSync(file, "utf8")) as Made;
  writeFileSync(file, `${JSON.stringify({ ...was, resumed: [...(was.resumed ?? []), now] }, null, 2)}\n`);
}

const perModel = (rows: Row[]) => {
  const table = new Map<string, { wakes: number; cost: number }>();
  for (const one of rows) {
    const was = table.get(one.bot.model) ?? { wakes: 0, cost: 0 };
    table.set(one.bot.model, { wakes: was.wakes + 1, cost: was.cost + one.cost });
  }
  return [...table.entries()].map(
    ([model, one]) => `- ${model}: ${one.wakes} wakes, ${dollars(one.cost)}, ${dollars(one.cost / one.wakes)} per wake`,
  );
};

function writeUp(name: string, dir: string, rows: Row[], spent: number, apps: Awaited<ReturnType<typeof install>>) {
  const done = report(apps, dir);
  const measured = measuresOf(apps, dir);
  const mind = minds(apps, dir, rows);
  const journal = readFileSync(join(dir, "journal.jsonl"), "utf8").trim().split("\n").filter(Boolean);
  const feedback = journal
    .map((line) => JSON.parse(line))
    .filter((one) => one.verb === "feedback.send")
    .map((one) => {
      const a = one.args;
      return `- [${a.severity}] ${a.app}${a.verb ? `.${a.verb}` : ""} (${one.who.id}): ${a.happened}${a.expected ? ` EXPECTED: ${a.expected}` : ""}`;
    });
  const counts = (table: Record<string, number>) =>
    Object.entries(table)
      .sort((a, b) => b[1] - a[1])
      .map(([key, n]) => `- ${n} ${key}`);
  const held = open(apps, dir).read("wish").state as {
    wishes?: Record<string, { n: number; by: string; text: string; backers: string[]; withdrawn: number | null }>;
  } | null;
  const wishes = Object.values(held?.wishes ?? {})
    .filter((one) => one.withdrawn === null)
    .sort((a, b) => b.backers.length - a.backers.length)
    .map((one) => `- ${one.backers.length} backers, ${one.by}: ${one.text}`);
  return [
    `# ${name}`,
    "",
    `- spent ${dollars(spent)} on ${rows.length} wakes; tick ${done.tick}; replayed ${done.replayed}; faults ${done.faults}`,
    "",
    "## COST BY MODEL",
    "",
    ...perModel(rows),
    "",
    "## MEASURES",
    "",
    ...MEASURES.map((one) => `- ${one.name}: ${measured[one.name] ?? "-"} (${one.line})`),
    ...MINDS.map((one) => `- ${one.name}: ${mind[one.name] ?? "-"} (${one.line})`),
    "",
    "## AGENTS",
    "",
    ...rows.map(
      (one) =>
        `- ${one.bot.id} wake ${one.wake} ${one.bot.persona} ${one.bot.model} ${one.bot.effort}: ${dollars(one.cost)}, ${one.turns} turns, ${one.stop}, tools ${one.tools.length}`,
    ),
    "",
    "## WISHES",
    "",
    ...wishes,
    "",
    "## FEEDBACK",
    "",
    ...feedback,
    "",
    "## FAULTS",
    "",
    ...counts(done.faulty),
    "",
    "## REFUSED",
    "",
    ...counts(done.refusals),
    "",
    "## VERBS",
    "",
    ...counts(done.verbs),
    "",
    "## SUMMARIES",
    "",
    ...rows.map((one) => `### ${one.bot.id} wake ${one.wake} ${one.bot.persona}\n\n${one.summary}\n`),
  ].join("\n");
}

await main();
