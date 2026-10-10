import { expect, test } from "bun:test";

const MARK = "/*MOVED*/{}";
const TABLE = JSON.stringify({ "/math/": "/mrlymath/" });

const text = await Bun.file(new URL("./edge.js", import.meta.url)).text();
const edge = new Function(text.replace(MARK, TABLE) + "; return { decide, handler }")() as {
  decide: (uri: string) => { uri?: string; redirect?: string };
  handler: (event: unknown) => any;
};

const pass = (uri: string) => expect(edge.decide(uri)).toEqual({ uri });
const shell = (uri: string) => expect(edge.decide(uri)).toEqual({ uri: "/index.html" });

const ask = (uri: string, query: Record<string, unknown> = {}, host = "mrly.net") =>
  edge.handler({ request: { uri, querystring: query, headers: { host: { value: host } } } });

test("the file holds the mark once", () => {
  expect(text.split(MARK).length).toBe(2);
});

test("the root answers the shell", () => shell("/"));

test("a bare segment 301s to its slash form", () => {
  expect(edge.decide("/x")).toEqual({ redirect: "/x/" });
});

test("a slash form answers the shell", () => shell("/x/"));

test("the 301 keeps the query string", () => {
  const out = ask("/x", { seed: { value: "1" } });
  expect(out.statusCode).toBe(301);
  expect(out.headers.location.value).toBe("/x/?seed=1");
});

test("the 301 keeps a repeated key", () => {
  const out = ask("/x", { a: { value: "1", multiValue: [{ value: "1" }, { value: "2" }] } });
  expect(out.headers.location.value).toBe("/x/?a=1&a=2");
});

test("a rewrite sets the request uri and leaves the query alone", () => {
  const out = ask("/x/", { seed: { value: "1" } });
  expect(out.uri).toBe("/index.html");
  expect(out.querystring).toEqual({ seed: { value: "1" } });
});

test("raw files pass with or without a dot", () => {
  pass("/raw/LICENSE");
  pass("/raw/a.md");
});

test("a git path answers the shell", () => shell("/git/a/b.ts"));

test("the git root answers the shell", () => shell("/git/"));

test("git.json is a file", () => pass("/git.json"));

test("the shell itself passes", () => pass("/index.html"));

test("root text files pass", () => {
  pass("/robots.txt");
  pass("/sitemap-raw.xml");
  pass("/llms.txt");
});

test("a paper pdf passes", () => pass("/a/paper.pdf"));

test("an old page file passes to the S3 miss", () => pass("/x/index.html"));

test("a moved key 301s to its target", () => {
  expect(edge.decide("/math/")).toEqual({ redirect: "/mrlymath/" });
});

test("a moved key without the slash 301s to its target", () => {
  expect(edge.decide("/math")).toEqual({ redirect: "/mrlymath/" });
});

test("www 301s to the apex with the path and query", () => {
  const out = ask("/blog/", { a: { value: "b" } }, "www.mrly.net");
  expect(out.statusCode).toBe(301);
  expect(out.statusDescription).toBe("Moved Permanently");
  expect(out.headers.location.value).toBe("https://mrly.net/blog/?a=b");
});
