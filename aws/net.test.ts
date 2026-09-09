import { expect, test } from "bun:test";
import { readEvent, SOURCE } from "./net.ts";

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
  expect(readEvent(JSON.stringify({ source: "push", repo: SOURCE, sha: "nope" })))
    .toEqual({ source: "push", on: "source", sha: "" });
});

test("a sha with no repo never guesses which repo moved", () => {
  expect(readEvent(JSON.stringify({ body: JSON.stringify({ source: "manual", sha: SHA }) })))
    .toEqual({ source: "manual", on: "", sha: SHA });
});
