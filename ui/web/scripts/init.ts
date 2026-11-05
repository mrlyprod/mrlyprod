import { existsSync, mkdirSync, readdirSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { parseArgs } from "node:util";

export const DESIGNS = ["modern", "boxy", "classic", "bricks", "terminal"] as const;

export type Design = (typeof DESIGNS)[number];

export type Options = { title: string; domain: string; design: Design };

export const DEFAULTS: Options = { title: "Example", domain: "example.com", design: "modern" };

const FACES = [
  { family: "Noto Sans", file: "sans", axes: "wght@400..700", weight: "400 700", licence: "notosans" },
  { family: "Noto Serif", file: "serif", axes: "wght@400..700", weight: "400 700", licence: "notoserif" },
  { family: "Noto Sans Mono", file: "mono", axes: "wght@400..700", weight: "400 700", licence: "notosansmono" },
  { family: "Noto Sans Math", file: "math", weight: "400", licence: "notosansmath" },
  { family: "Noto Color Emoji", file: "emoji", weight: "400", licence: "notocoloremoji" },
];

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
  designs: [`../../designs/${design}`],
  roots: ["pages"],
  screen: [1440, 900, 2],
  min: [1280, 720],
  company: title,
  tagline: `${title}, a site on MrlyNet.`,
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
  llms: {
    about: `${title} is a site built on MrlyNet: a home, a menu, an about page and a page for missing urls.`,
    legend: "Links are page sources in markdown: /raw/pages/x/index.md is /x/; a link outside /raw/ is the page.",
  },
  fonts: { faces: FACES },
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

const SQUARES =
  '<g fill="#fff"><rect x="280" y="280" width="144" height="144" rx="29"/><rect x="600" y="280" width="144" height="144" rx="29"/><rect x="280" y="600" width="144" height="144" rx="29"/><rect x="600" y="600" width="144" height="144" rx="29"/></g>';

const DISC = '<circle cx="512" cy="512" r="224" fill="#fff"/>';

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

const HOME_PAGE = `import type { Site } from "site:routes";
import type { Row } from "@mrlyprod/ui/kit/rows.ts";
import { hide, show } from "@mrlyprod/ui/shell/parts.tsx";
import type { Layout } from "@mrlyprod/ui/shell/router.ts";

export const layout: Layout = { wide: true, bare: true, window: "none" };

export function render(_row: Row, site: Site, host: HTMLElement) {
  show(host, <h1 className="visually-hidden">{site.title}</h1>);
}

export const unmount = hide;
`;

const MENU_PAGE = `import type { Site } from "site:routes";
import type { Row } from "@mrlyprod/ui/kit/rows.ts";
import { Launchpad, opened } from "@mrlyprod/ui/shell/launchpad.tsx";
import { hide, show } from "@mrlyprod/ui/shell/parts.tsx";
import type { Layout } from "@mrlyprod/ui/shell/router.ts";

export const layout: Layout = { wide: true, bare: true, window: "none" };

export function render(_row: Row, _site: Site, host: HTMLElement) {
  show(host, <Launchpad />);
}

export const mount = opened;

export const unmount = hide;
`;

const MISSING_PAGE = `import type { Site } from "site:routes";
import type { Row } from "@mrlyprod/ui/kit/rows.ts";
import { hide, Lost, show } from "@mrlyprod/ui/shell/parts.tsx";
import type { Context, Layout } from "@mrlyprod/ui/shell/router.ts";

export const layout: Layout = { bare: true };

export function render(row: Row, site: Site, host: HTMLElement, { rows }: Context) {
  show(host, <Lost row={row} site={site} rows={rows} />);
}

export const unmount = hide;
`;

const about = ({ title, domain }: Options) => `---
opener: true
---

# About

${title} runs on MrlyNet, an open platform that draws every page in the browser.

## THIS SITE

- Each folder of \`pages/\` is a route: \`about/\` is \`/about/\`.
- A route holds an \`app.json\`, a page (\`index.md\` or \`index.tsx\`), an \`icon.ts\` and an \`icon.svg\`.
- \`site.json\` holds the name, the domain (${domain}), the design and the tint.
- \`launchpad.json\` places every shown route in a section of the menu.
`;

export function files(options: Options): Record<string, string> {
  const { title, domain } = options;
  return {
    "site.json": json(site(options)),
    "launchpad.json": json({ sections: [{ name: "Pages", apps: ["about"] }], dock: [["about"]] }),
    "redirects.json": "{}\n",
    ".gitignore": "dist/\nwasm/\nnode_modules\n",
    "public/mark.svg": MARK,
    "pages/home/app.json": json({ title: "{title}", line: `The home of ${title}.` }),
    "pages/home/index.tsx": HOME_PAGE,
    "pages/home/icon.ts": figure("blue", ["101", "010", "101"]),
    "pages/home/icon.svg": svg("#5aa0ff", "#2f6fde", DISC),
    "pages/menu/app.json": json({ title: "Menu", line: `Every page of ${domain}.` }),
    "pages/menu/index.tsx": MENU_PAGE,
    "pages/menu/icon.ts": figure("gray", ["101", "000", "101"]),
    "pages/menu/icon.svg": svg("#a8a8ad", "#5c5c61", SQUARES),
    "pages/missing/app.json": json({
      title: "Nothing here",
      line: "That page does not exist.",
      hidden: true,
      route: "/404.html",
    }),
    "pages/missing/index.tsx": MISSING_PAGE,
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
