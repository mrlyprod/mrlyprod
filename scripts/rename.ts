import { existsSync, readdirSync, readFileSync, renameSync, statSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";

const REPO = resolve(import.meta.dir, "..");
const SYSTEM = join(REPO, "net/system");
const LISTS = ["net/mrly.net/launchpad.json", "net/mrly.net/world.json", "../com/site/launchpad.json", "../com/site/world.json"];
const README = join(SYSTEM, "README.md");

const [old, next, title] = process.argv.slice(2);
if (!old || !next) {
  console.log("usage: bun scripts/rename.ts <old> <new> [Title]");
  process.exit(1);
}
if (!/^[a-z]{2,16}$/.test(next)) throw new Error(`id ${next} is not one lowercase word`);
if (!existsSync(join(SYSTEM, old))) throw new Error(`no app ${old}`);
if (existsSync(join(SYSTEM, next))) throw new Error(`app ${next} exists`);

const walk = (dir: string): string[] =>
  readdirSync(dir).flatMap((name) => {
    const path = join(dir, name);
    return statSync(path).isDirectory() ? walk(path) : [path];
  });

const swap = (path: string, pairs: [RegExp, string][]) => {
  const was = readFileSync(path, "utf8");
  let text = was;
  for (const [from, to] of pairs) text = text.replace(from, to);
  if (text !== was) writeFileSync(path, text);
  return text !== was;
};

const quoted = (id: string) => new RegExp(`(["'\`])${id}(?=["'\`./])`, "g");
const member = (id: string) => new RegExp(`\\bapps\\.${id}\\b`, "g");

renameSync(join(SYSTEM, old), join(SYSTEM, next));
const folder = join(SYSTEM, next);
for (const path of walk(folder)) {
  if (!/\.(ts|tsx|json|md)$/.test(path)) continue;
  swap(path, [[quoted(old), `$1${next}`], [member(old), `apps.${next}`], [new RegExp(`/${old}/`, "g"), `/${next}/`]]);
}
if (title) {
  const meta = join(folder, "app.json");
  const json = JSON.parse(readFileSync(meta, "utf8")) as Record<string, unknown>;
  json.title = title;
  writeFileSync(meta, `${JSON.stringify(json, null, 2)}\n`);
  const code = join(folder, "app.ts");
  if (existsSync(code)) swap(code, [[/title: "[^"]+"/, `title: "${title}"`]]);
}
for (const list of LISTS) {
  const path = join(REPO, list);
  if (existsSync(path)) swap(path, [[quoted(old), `$1${next}`]]);
}
swap(README, [[new RegExp(`\`${old}\``, "g"), `\`${next}\``], [new RegExp(`\`${old}\\.`, "g"), `\`${next}.`]]);

const left: string[] = [];
for (const root of ["net", "apps", "org", "../com/site", "../org/site"]) {
  const dir = join(REPO, root);
  if (!existsSync(dir)) continue;
  for (const path of walk(dir)) {
    if (!/\.(ts|tsx|json|md)$/.test(path) || path.includes("/dist/") || path.includes("node_modules")) continue;
    if (path.startsWith(folder)) continue;
    const text = readFileSync(path, "utf8");
    const lines = text.split("\n");
    lines.forEach((line, i) => {
      if (quoted(old).test(line) || line.includes(`/${old}/`)) left.push(`${path.slice(REPO.length + 1)}:${i + 1}: ${line.trim().slice(0, 100)}`);
    });
  }
}
console.log(`${old} -> ${next}${title ? ` (${title})` : ""}`);
if (left.length) console.log(["left to check:", ...left].join("\n"));
