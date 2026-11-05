import { join, relative, resolve } from "node:path";

export const ORG = resolve(import.meta.dir, "..");
export const SITE = join(ORG, "site");
export const PAGES = join(SITE, "pages");
const REPO = resolve(ORG, "..", "..");
export const HOST = join(REPO, "ui");
export const DATA_DIR = join(REPO, "data", relative(REPO, import.meta.dir));
export const DIST = join(DATA_DIR, "dist");

export async function host(verb: string, args: string[] = [], env: Record<string, string> = {}): Promise<number> {
  const proc = Bun.spawn(["bun", "run", verb, ...args], {
    cwd: HOST,
    env: { ...process.env, MRLY_ROOT: SITE, ...env },
    stdout: "inherit",
    stderr: "inherit",
  });
  return proc.exited;
}

export const wasm = () => host("wasm", ["--copy"]);

export const build = async (dist = DIST) => (await wasm()) || host("build", [], { MRLY_DIST: dist });

const VERBS: Record<string, (args: string[]) => Promise<number>> = {
  wasm: () => wasm(),
  content: () => host("content"),
  build: () => build(),
  dev: async (args) => (await wasm()) || host("dev", args),
  shots: async (args) => (await wasm()) || host("shots", args, { MRLY_DIST: DIST }),
  push: async (args) => (await wasm()) || host("push", args, { MRLY_DIST: DIST }),
};

if (import.meta.main) {
  const [verb = "build", ...args] = process.argv.slice(2);
  const run = VERBS[verb];
  if (!run) throw new Error(`org: no verb ${verb}; the verbs are ${Object.keys(VERBS).join(", ")}`);
  process.exit(await run(args));
}
