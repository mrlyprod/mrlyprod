import { expect, test } from "bun:test";
import { existsSync, readdirSync, readFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";

const REPO = resolve(import.meta.dir, "..", "..");

const TOPS = [
  "net/spec",
  "net/kernel",
  "net/doors",
  "net/hosts/local",
  "net/sim",
  "net/shell",
  "net/sdk",
  "net/system",
  "designs",
  "apps",
  "org",
];

const SPEC = /(?:\bfrom|\bimport)\s*\(?\s*(['"])(\.{1,2}\/[^'"\n]*)\1/g;

const AREAS: [RegExp, (m: RegExpMatchArray) => string][] = [
  [/^net\/spec\//, () => "spec"],
  [/^net\/kernel\//, () => "kernel"],
  [/^net\/doors\//, () => "doors"],
  [/^net\/shell\//, () => "shell"],
  [/^net\/sdk\//, () => "sdk"],
  [/^net\/system\/([^/]+)\//, (m) => `system/${m[1]}`],
  [/^net\/hosts\//, () => "hosts"],
  [/^net\/sim\//, () => "sim"],
  [/^(?:designs\/shared|site\/src\/styles)\//, () => "designs/shared"],
  [/^(?:designs|site\/src\/designs)\/([^/]+)\//, (m) => `designs/${m[1]}`],
  [/^(?:apps\/shared|site\/src\/lib)\/([^/]+)\//, (m) => `family/${m[1]}`],
  [/^(?:apps|site\/src\/apps)\/([^/]+)\//, (m) => `apps/${m[1]}`],
  [/^org\/([^/]+)\//, (m) => `org/${m[1]}`],
];

const MAY: Record<string, string[]> = {
  spec: [],
  kernel: ["spec"],
  doors: ["kernel", "hosts", "sdk", "sim"],
  hosts: ["kernel", "sdk", "sim"],
  sim: ["kernel", "sdk", "spec"],
  shell: ["sdk", "kernel"],
  sdk: ["kernel"],
  system: ["sdk", "shell"],
  org: ["sdk", "shell"],
  apps: ["sdk", "family"],
  family: ["sdk", "family"],
  designs: ["sdk", "designs/shared"],
};

const SEAM = ["net/shell/active.ts", "net/shell/design.ts", "net/shell/measure.ts"];

const CONTRACT = ["net/shell/design.ts"];

const KNOWN = ["designs/boxy -> family/font", "sdk -> family/designs", "sdk -> family/units"];

const area = (path: string) => {
  for (const [look, name] of AREAS) {
    const m = path.match(look);
    if (m) return name(m);
  }
  return "other";
};

const kind = (name: string) => (name === "designs/shared" ? name : name.split("/")[0]!);

const code = TOPS.filter((top) => existsSync(join(REPO, top))).flatMap((top) =>
  readdirSync(join(REPO, top), { encoding: "utf8", recursive: true })
    .filter((path) => /\.(js|jsx|ts|tsx)$/.test(path) && !/\.test\./.test(path) && !path.includes("node_modules"))
    .map((path) => join(REPO, top, path)),
);

const edges = code.flatMap((file) =>
  [...readFileSync(file, "utf8").matchAll(SPEC)]
    .map(([, , spec]) => resolve(dirname(file), spec!))
    .filter((to) => to.startsWith(`${REPO}/`))
    .map((to) => {
      const at = relative(REPO, file);
      const into = relative(REPO, to);
      return { file: at, to: into, from: area(at), into: area(into) };
    })
    .filter(({ from, into }) => from !== into),
);

const allowed = ({ from, into, to }: (typeof edges)[number]) =>
  (MAY[kind(from)]?.some((one) => one === into || one === kind(into)) ?? false) ||
  (kind(from) === "sdk" && SEAM.includes(to)) ||
  (kind(from) === "designs" && CONTRACT.includes(to));

const broken = [...new Set(edges.filter((one) => !allowed(one)).map(({ from, into }) => `${from} -> ${into}`))].sort();

test("every import points down: routes to the sdk and the shell, apps to their families, designs to the contract, nothing up", () => {
  expect(broken.filter((one) => !KNOWN.includes(one))).toEqual([]);
});

test("the known exceptions only shrink: each still happens, or it leaves the list", () => {
  expect(KNOWN.filter((one) => !broken.includes(one))).toEqual([]);
});
