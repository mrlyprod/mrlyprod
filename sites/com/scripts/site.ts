import { join, relative, resolve } from "node:path";

export const COM = resolve(import.meta.dir, "..");
export const SITE = join(COM, "site");
const REPO = resolve(COM, "..", "..");
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

const VERBS: Record<string, (args: string[]) => Promise<number>> = {
  wasm: () => wasm(),
  content: () => host("content"),
  build: async (args) => (await wasm()) || host("build", args, { MRLY_DIST: DIST }),
  dev: async (args) => (await wasm()) || host("dev", args),
  shots: async (args) => (await wasm()) || host("shots", args, { MRLY_DIST: DIST }),
  push: async (args) => (await wasm()) || host("push", args, { MRLY_DIST: DIST }),
};

if (import.meta.main) {
  const [verb = "build", ...args] = process.argv.slice(2);
  const run = VERBS[verb];
  if (!run) throw new Error(`com: no verb ${verb}; the verbs are ${Object.keys(VERBS).join(", ")}`);
  process.exit(await run(args));
}
