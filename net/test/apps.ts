import { mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import { HOME, REPO } from "../hosts/web/scripts/home.ts";

/* APPS */

const site = JSON.parse(readFileSync(join(HOME, "site.json"), "utf8")) as { roots: string[] };

export const APPS = mkdtempSync(join(tmpdir(), "apps-"));

writeFileSync(
  join(APPS, "site.json"),
  JSON.stringify({ ...site, roots: [...site.roots.map((one) => resolve(HOME, one)), join(REPO, "apps")] }),
);

process.on("exit", () => rmSync(APPS, { recursive: true, force: true }));
