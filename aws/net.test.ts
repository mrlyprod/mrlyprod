import { expect, test } from "bun:test";
import { mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { modules, onMain, readEvent, SOURCE } from "./net.ts";

const SHA = "0123456789abcdef0123456789abcdef01234567";

process.env.SHELF_REPO = "owner/shelf";

test("the payload parser reads a schedule tick and both push wakes", () => {
  expect(readEvent(JSON.stringify({ source: "schedule" }))).toEqual({ source: "schedule", on: "", sha: "" });
  expect(readEvent(JSON.stringify({ source: "push", repo: SOURCE, sha: SHA })))
    .toEqual({ source: "push", on: "source", sha: SHA });
  expect(readEvent(JSON.stringify({ source: "push", repo: "owner/shelf", sha: SHA })))
    .toEqual({ source: "push", on: "shelf", sha: SHA });
});

test("the payload parser survives junk, an unknown source, a foreign repo and a bad sha", () => {
  expect(readEvent("")).toEqual({ source: "", on: "", sha: "" });
  expect(readEvent("not json")).toEqual({ source: "", on: "", sha: "" });
  expect(readEvent(JSON.stringify({ source: "nonsense", repo: SOURCE }))).toEqual({ source: "", on: "source", sha: "" });
  expect(readEvent(JSON.stringify({ source: "push", repo: "owner/site", sha: SHA }))).toEqual({ source: "push", on: "", sha: SHA });
  expect(readEvent(JSON.stringify({ source: "manual", repo: SOURCE, sha: "nope" })))
    .toEqual({ source: "manual", on: "source", sha: "" });
});

test("a sha with no repo never guesses which repo moved", () => {
  expect(readEvent(JSON.stringify({ body: JSON.stringify({ source: "manual", sha: SHA }) })))
    .toEqual({ source: "manual", on: "", sha: SHA });
});

const says = (status: string) => async () => ({ ok: true, status: 200, json: async () => ({ status }) });

const code = (status: number) => async () => ({ ok: false, status, json: async () => ({}) });

const dead = async () => {
  throw new Error("getaddrinfo ENOTFOUND api.github.com");
};

test("the ancestor check accepts a sha main is ahead of or identical to", async () => {
  expect(await onMain(SOURCE, SHA, says("ahead"))).toBe(true);
  expect(await onMain("owner/shelf", SHA, says("identical"))).toBe(true);
});

test("the ancestor check refuses another status, an unknown sha and a dead fetch", async () => {
  expect(await onMain(SOURCE, SHA, says("behind"))).toBe(false);
  expect(await onMain(SOURCE, SHA, says("diverged"))).toBe(false);
  expect(await onMain(SOURCE, SHA, code(404))).toBe(false);
  expect(await onMain(SOURCE, SHA, dead)).toBe(false);
});

test("the modules step takes the layer only when it holds this lockfile", () => {
  const site = mkdtempSync(join(tmpdir(), "site-"));
  const layer = mkdtempSync(join(tmpdir(), "layer-"));
  writeFileSync(join(site, "bun.lock"), "lock");
  expect(modules(site, layer)).toBe("absent");
  writeFileSync(join(layer, "bun.lock.sha256"), "nope\n");
  expect(modules(site, layer)).toBe("stale");
  writeFileSync(join(layer, "bun.lock.sha256"), `${new Bun.CryptoHasher("sha256").update("lock").digest("hex")}\n`);
  expect(modules(site, layer)).toBe("layer");
});
