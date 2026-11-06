import { existsSync, mkdirSync, readdirSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { parseArgs } from "node:util";

export const DESIGNS = ["modern", "classic", "system"] as const;

export type Design = (typeof DESIGNS)[number];

export type Options = { title: string; domain: string; design: Design };

export const DEFAULTS: Options = { title: "Example", domain: "example.com", design: "modern" };

const json = (value: unknown) => `${JSON.stringify(value, null, 2)}\n`;

const site = ({ title, domain, design }: Options) => ({
  title,
  wordmark: title.toUpperCase(),
  root: `https://${domain}`,
  since: new Date().getFullYear(),
  prefix: `${
    domain
      .split(".")[0]!
      .replace(/[^a-z0-9]/gi, "")
      .toLowerCase() || "site"
  }-`,
  mark: "/mark.svg",
  tint: "blue",
  menu: "/menu/",
  desk: "tiles",
  tiles: { id: "tiles", title: "Tiles" },
  designs: [design, ...DESIGNS.filter((one) => one !== design)].map((one) => `../../designs/${one}`),
  roots: ["pages"],
  screen: [1440, 900, 2],
  min: [1280, 720],
  company: title,
  tree: [
    {
      name: "Root",
      nodes: [
        { name: "robots.txt", href: "/robots.txt" },
        { name: "sitemap.xml", href: "/sitemap.xml" },
        { name: "llms.txt", href: "/llms.txt" },
        { name: "404.html", href: "/404.html" },
      ],
    },
  ],
  socials: [],
  contact: `hello@${domain}`,
  shots: { routes: ["/", "/about/", "/menu/", "/nowhere/"] },
  push: {
    prefix: "",
    guard: ["build/"],
    bucket: "SITE_BUCKET",
    wasm: "SITE_BUCKET",
    store: { bucket: "SITE_BUCKET", prefix: "build/site" },
  },
  git: { root: ".", slug: `owner/${domain}`, branch: "main" },
});

const MARK = `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64"><rect width="64" height="64" rx="14" fill="#2f6fde"/><circle cx="32" cy="32" r="14" fill="#fff"/></svg>\n`;

const svg = (
  top: string,
  bottom: string,
  glyph: string,
) => `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1024 1024">
  <defs>
    <linearGradient id="back" x1="0" y1="0" x2="0" y2="1"><stop offset="0" stop-color="${top}"/><stop offset="1" stop-color="${bottom}"/></linearGradient>
  </defs>
  <rect class="back" width="1024" height="1024" fill="url(#back)" data-key="${top}"/>
  ${glyph}
</svg>
`;

const LINES =
  '<g fill="#fff"><rect x="272" y="320" width="480" height="64" rx="32"/><rect x="272" y="480" width="480" height="64" rx="32"/><rect x="272" y="640" width="320" height="64" rx="32"/></g>';

const figure = (hue: string, art: string[]) => `import type { Pen } from "@mrlyprod/draw";
import { Grid } from "@mrlyprod/draw";
import { plate } from "@mrlyprod/draw/plate.ts";

export const units = {};

const ART = ${JSON.stringify(art)};

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "${hue}", 0.2);
  const cells = new Grid(box, ART[0]!.length, ART.length, 0.2);
  ART.forEach((line, row) => {
    [...line].forEach((c, col) => {
      if (c === "0") return;
      const [x, y, w, h] = cells.cell(col, row);
      pen.round_rect(x, y, w, h, Math.min(w, h) * 0.3, paint.white);
    });
  });
}
`;

const about = ({ domain }: Options) => `---
opener: true
---

- ${domain}
`;

export function files(options: Options): Record<string, string> {
  const { title } = options;
  return {
    "site.json": json(site(options)),
    "launchpad.json": json({ sections: [{ name: "Pages", apps: ["about"] }], dock: [["about"]] }),
    "redirects.json": "{}\n",
    ".gitignore": "dist/\nwasm/\nnode_modules\n",
    "public/mark.svg": MARK,
    "pages/about/app.json": json({ title: "About", line: `What ${title} is and how its pages are made.` }),
    "pages/about/index.md": about(options),
    "pages/about/icon.ts": figure("green", ["111", "000", "110"]),
    "pages/about/icon.svg": svg("#5ed97c", "#2fa84f", LINES),
  };
}

export function init(dir: string, options: Options): string[] {
  const at = resolve(dir);
  if (existsSync(at) && readdirSync(at).length) throw new Error(`init: ${at} is not empty; pick a new or empty folder`);
  const all = { ...files(options), "tsconfig.json": json({ extends: "../../ui/tsconfig.json" }) };
  for (const [path, text] of Object.entries(all)) {
    const file = join(at, path);
    mkdirSync(dirname(file), { recursive: true });
    writeFileSync(file, text);
  }
  return Object.keys(all);
}

export function options(argv: string[]): { dir: string; options: Options } {
  const { values, positionals } = parseArgs({
    args: argv,
    allowPositionals: true,
    options: { title: { type: "string" }, domain: { type: "string" }, design: { type: "string" } },
  });
  const [dir, extra] = positionals;
  if (!dir || extra)
    throw new Error("init: bun run init <dir> [--title Name] [--domain example.com] [--design modern]");
  const design = (values.design ?? DEFAULTS.design) as Design;
  if (!DESIGNS.includes(design)) throw new Error(`init: the design is one of ${DESIGNS.join(", ")}`);
  const domain = (values.domain ?? DEFAULTS.domain).replace(/^https?:\/\//, "").replace(/\/.*$/, "");
  if (!/^[a-z0-9-]+(\.[a-z0-9-]+)+$/i.test(domain)) throw new Error(`init: ${domain} is not a domain`);
  return { dir, options: { title: values.title ?? DEFAULTS.title, domain, design } };
}

if (import.meta.main) {
  try {
    const { dir, options: given } = options(process.argv.slice(2));
    const wrote = init(dir, given);
    console.log(`init: ${wrote.length} files in ${resolve(dir)}, design ${given.design}, https://${given.domain}`);
  } catch (error) {
    console.error((error as Error).message);
    process.exit(1);
  }
}
