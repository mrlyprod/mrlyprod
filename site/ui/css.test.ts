import { expect, test } from "bun:test";
import { readdirSync, readFileSync } from "node:fs";
import { dirname, join } from "node:path";

/* SELECTORS */

const selectors = (css: string) => {
  const out: string[] = [];
  let depth = 0;
  let buf = "";
  for (const c of css.replace(/\/\*[\s\S]*?\*\//g, "")) {
    if (c === "{") {
      if (depth === 0) out.push(buf.trim().replace(/\s+/g, " "));
      depth++;
      buf = "";
    } else if (c === "}") {
      depth--;
      if (depth < 0) throw new Error("stray brace");
      buf = "";
    } else if (depth === 0) buf += c;
  }
  if (depth !== 0) throw new Error("unclosed brace");
  return out;
};

/* CSS */

test("the site css never repeats a top-level selector with another rule between", () => {
  const home = import.meta.dir;
  for (const name of readdirSync(home).filter((one) => one.endsWith(".css")).sort()) {
    const file = join(home, name);
    const list = selectors(readFileSync(file, "utf8"));
    const last = new Map<string, number>();
    list.forEach((sel, n) => {
      const was = last.get(sel);
      if (was !== undefined && list.slice(was + 1, n).some((other) => other !== sel)) throw new Error(`${file}: '${sel}' at ${was} and ${n}`);
      last.set(sel, n);
    });
  }
});

/* SHEETS */

const imports = (file: string) => [...readFileSync(file, "utf8").matchAll(/@import "([^"]+)"/g)].map(([, one]) => join(dirname(file), one!));

test("the git and app sheets import nothing the shell sheet already links", () => {
  const page = join(import.meta.dir, "page.css");
  const shell = new Set([page, ...imports(page)]);
  const twice = [join(import.meta.dir, "git.css"), join(import.meta.dir, "..", "lib", "mrly.css")].flatMap((file) => imports(file).filter((one) => shell.has(one)));
  expect(twice).toEqual([]);
});

/* TOKENS */

const SHEETS = ["chrome.css", "base.css", "pages.css", "../lib/mrly.css"];

const bare = (name: string) => readFileSync(join(import.meta.dir, name), "utf8").replace(/\/\*[\s\S]*?\*\//g, "");

const values = (css: string, prop: RegExp) => [...css.matchAll(/(?:^|[{;])\s*([a-z-]+)\s*:\s*([^;{}]+)/g)].filter(([, name]) => prop.test(name)).map(([, , value]) => value.trim());

test("the site css and the demo css set no font size in px", () => {
  for (const name of SHEETS) {
    for (const value of values(bare(name), /^font(-size)?$/)) {
      if (/[\d.]+px\b/.test(value.replace(/var\([^)]*\)/g, ""))) throw new Error(`${name}: font ${value}`);
    }
  }
});

test("the site css and the demo css set no bare numeric z-index", () => {
  for (const name of SHEETS) {
    for (const value of values(bare(name), /^z-index$/)) {
      if (/^-?\d/.test(value)) throw new Error(`${name}: z-index ${value}`);
    }
  }
});

test("tokens.css defines every type and layer token the site css reads", () => {
  const tokens = bare("tokens.css");
  for (const name of [...readdirSync(import.meta.dir).filter((one) => one.endsWith(".css") && one !== "tokens.css"), "../lib/mrly.css"]) {
    for (const [, token] of bare(name).matchAll(/var\(--((?:t|lh)\d+|z-[a-z]+)\b/g)) {
      if (!new RegExp(`--${token}\\s*:`).test(tokens)) throw new Error(`${name}: --${token} undefined`);
    }
  }
});

/* HOME */

const rules = (css: string) => [...css.matchAll(/([^{}]+)\{([^{}]*)\}/g)].map(([, sel, body]) => [sel!.trim().replace(/\s+/g, " "), body!] as const);

test("no rule for a hero, a half hero or a plate on either rounds a corner", () => {
  const round = readdirSync(import.meta.dir)
    .filter((one) => one.endsWith(".css"))
    .flatMap((name) => rules(bare(name)).filter(([sel, body]) => /\.(hero|half|plate)\b/.test(sel) && !/\.paper\b/.test(sel) && /border-radius/.test(body)).map(([sel]) => `${name}: ${sel}`));
  expect(round).toEqual([]);
});
