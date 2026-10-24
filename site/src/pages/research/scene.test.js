import { expect, test } from "bun:test";
import { cards, codeOf, hrefOf, many, shelf, slugOf, sourceOf } from "./scene.js";

test("the grid names no slug, a topic path names its folder, and a path outside research names none", () => {
  expect(["/research/", "/research/farey/", "/research/farey", "/research/a/b/", "/about/"].map(slugOf)).toEqual([
    "",
    "farey",
    "farey",
    "a/b",
    "",
  ]);
});

test("a slug makes its route, its README and its Code link, and a card is the title over that route", () => {
  expect([hrefOf("farey"), sourceOf("farey"), codeOf("farey")]).toEqual([
    "/research/farey/",
    "research/farey/README.md",
    "/git/research/farey/README.md",
  ]);
  expect(cards([{ slug: "farey", title: "Farey", figure: "research-farey" }])).toEqual([
    { name: "Farey", href: "/research/farey/", figure: "research-farey" },
  ]);
});

test("the shelf keeps the topics whose title or lead holds the query, any case, newest first", () => {
  const topics = [
    { slug: "a", title: "Farey", lead: "Fractions in order.", date: "2026-01-02" },
    { slug: "b", title: "Moire", lead: "Two gratings.", date: "2026-03-04" },
    { slug: "c", title: "Tilings", lead: "Farey arcs on a plane.", date: "2026-02-03" },
  ];
  expect(
    [shelf(topics), shelf(topics, " FAREY "), shelf(topics, "none")].map((list) => list.map((one) => one.slug)),
  ).toEqual([["b", "c", "a"], ["c", "a"], []]);
});

test("a count names its word, plural past one", () => {
  expect([many(0, "topic"), many(1, "figure"), many(3, "figure")]).toEqual(["0 topics", "1 figure", "3 figures"]);
});
