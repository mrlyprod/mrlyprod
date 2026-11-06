import { existsSync, readdirSync, readFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";

const TOP = resolve(import.meta.dir, "..", "..");
const INSTANCES = ["net", "com", "org", "tv", "store", "sim"];
const HOMES: Record<string, string> = {
  os: join(TOP, "os"),
  apps: join(TOP, "apps"),
  ui: join(TOP, "ui"),
  designs: join(TOP, "designs"),
  net: join(TOP, "sites", "net"),
  com: join(TOP, "sites", "com"),
  org: join(TOP, "sites", "org"),
  tv: join(TOP, "sites", "tv"),
  store: join(TOP, "sites", "store"),
};
for (const arg of process.argv.slice(2)) {
  const [name, dir] = arg.split("=");
  if (name && dir) HOMES[name] = resolve(dir);
}
const LAYERS = ["os", "apps", "ui", "designs"];
const LEAVES = ["draw", "loader", "units", ...readdirSync(join(import.meta.dir, "..", "..", "packages", "rs"))];
const SKIP = new Set(["node_modules", ".git", "dist", "data", "target", "pkg", "wasm", ".venv", "__pycache__"]);
const CODE = /\.(ts|tsx|js|jsx)$/;
const WORDS = /\.(ts|tsx|js|jsx|json|css|py|sh)$/;
const SCOPE = /^@mrlyprod\/([a-z]+)(?:\/(.*))?$/;
const VENDOR = /(?<![-\w])(stripe|aws|s3)\b/i;
const SIM = /\b(sim|mrlysim|goose|scripted|players?)\b|simulat|bots\.ts/i;
const VIRTUAL = /^(site|apps?):/;
const DEPS = ["dependencies", "peerDependencies", "optionalDependencies"];

const seen: string[] = [];
const say = (file: string, line: string) => seen.push(`${relative(TOP, file)}: ${line}`);

const walk = (dir: string): string[] =>
  !existsSync(dir)
    ? []
    : readdirSync(dir, { withFileTypes: true }).flatMap((one) => {
        if (SKIP.has(one.name)) return [];
        const at = join(dir, one.name);
        return one.isDirectory() ? walk(at) : one.isFile() ? [at] : [];
      });

const repoOf = (file: string) =>
  Object.keys(HOMES).find((name) => !relative(HOMES[name]!, file).startsWith("..")) ??
  relative(TOP, file).split("/")[0]!;
const here = (repo: string) => HOMES[repo] ?? join(TOP, repo);

const transpilers = {
  ts: new Bun.Transpiler({ loader: "ts" }),
  tsx: new Bun.Transpiler({ loader: "tsx" }),
  js: new Bun.Transpiler({ loader: "js" }),
  jsx: new Bun.Transpiler({ loader: "jsx" }),
};

const imports = (file: string, text: string) => {
  const kind = file.split(".").at(-1) as keyof typeof transpilers;
  const named = file.endsWith(".d.ts") ? [] : transpilers[kind].scan(text).imports.map((one) => one.path);
  const later = [...text.matchAll(/\bimport\(\s*["']([^"'$`]+)["']\s*\)/g)].map((one) => one[1]!);
  return [...named, ...later];
};

const allowedScopes: Record<string, (rest: string, file: string) => boolean> = {
  os: (rest) => rest.startsWith("os/"),
  apps: (rest) => rest.startsWith("os/contract/") || rest.startsWith("apps/"),
  ui: (rest) => rest.startsWith("os/") || rest.startsWith("ui/"),
  designs: (rest) => rest.startsWith("os/") || rest.startsWith("ui/"),
};

const npmAllowed: Record<string, boolean> = { os: false, apps: false, ui: true, designs: true };

function reach(repo: string, file: string, spec: string) {
  const clean = spec.split("?")[0]!;
  if (LAYERS.includes(repo) && /^three(\/|$)/.test(clean))
    return say(file, `${repo} may not import three; only a site may`);
  if (clean.startsWith("node:") || clean === "bun" || VIRTUAL.test(clean)) return;
  if (clean.startsWith(".")) {
    const to = resolve(dirname(file), clean);
    const target = repoOf(to);
    if (target !== repo) say(file, `reaches into ${target} by the path ${spec}`);
    if (repo === "designs") {
      const own = relative(join(TOP, "designs"), file).split("/")[0];
      const hit = relative(join(TOP, "designs"), to).split("/")[0];
      if (own !== hit) say(file, `design ${own} imports design ${hit}`);
    }
    return;
  }
  const scoped = SCOPE.exec(clean);
  if (scoped) {
    const name = scoped[1]!;
    if (INSTANCES.includes(name)) return say(file, `imports the instance ${name}`);
    if (LEAVES.includes(name)) {
      if (repo === "os") say(file, `os imports the library ${name}`);
      return;
    }
    const rest = clean.slice("@mrlyprod/".length);
    const rule = allowedScopes[repo];
    if (rule && !rule(rest, file)) say(file, `${repo} may not import ${spec}`);
    if (LAYERS.indexOf(name) === -1) say(file, `imports the unknown package ${spec}`);
    return;
  }
  if (repo in npmAllowed && !npmAllowed[repo]) say(file, `${repo} may not import the package ${spec}`);
}

for (const repo of [...LAYERS, ...INSTANCES])
  for (const file of walk(here(repo)).filter((one) => CODE.test(one)))
    for (const spec of imports(file, readFileSync(file, "utf8"))) reach(repo, file, spec);

const noRuntime = (repo: string) => {
  const file = join(here(repo), "package.json");
  if (!existsSync(file)) return say(file, "is missing");
  const held = JSON.parse(readFileSync(file, "utf8")) as Record<string, Record<string, string> | undefined>;
  for (const key of DEPS)
    if (Object.keys(held[key] ?? {}).length) say(file, `${repo} has no runtime dependencies, and it lists ${key}`);
};

noRuntime("os");
noRuntime("apps");

for (const repo of [...LAYERS, ...INSTANCES]) {
  const file = join(here(repo), "package.json");
  if (existsSync(file) && /\.\.\/[a-z]+\/node_modules/.test(readFileSync(file, "utf8")))
    say(file, "reaches into another repo's node_modules");
}

const ownLine = (file: string) => {
  const name = relative(TOP, file);
  return name.startsWith("os/scripts/") || name.endsWith("bun.lock");
};

const vendorFree = (file: string) => {
  const name = relative(TOP, file);
  if (name.startsWith("os/hosts/cloud/")) return false;
  return !INSTANCES.some((one) => file.startsWith(`${here(one)}/seams/`));
};

const offending = (file: string, rule: RegExp, what: string) =>
  readFileSync(file, "utf8")
    .split("\n")
    .forEach((line, i) => {
      if (rule.test(line)) say(file, `line ${i + 1} names ${what}: ${line.trim().slice(0, 80)}`);
    });

for (const repo of [...LAYERS, ...INSTANCES])
  for (const file of walk(here(repo)).filter((one) => WORDS.test(one) && !ownLine(one))) {
    if (vendorFree(file)) offending(file, VENDOR, "a vendor");
    if (LAYERS.includes(repo)) offending(file, SIM, "the simulation");
  }

if (seen.length) {
  console.error([...seen, `rules: ${seen.length} broken`].join("\n"));
  process.exit(1);
}
console.log("rules: every repo keeps to its layer");
