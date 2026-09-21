import { test } from "bun:test";
import { readdirSync, readFileSync } from "node:fs";
import { join } from "node:path";

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
