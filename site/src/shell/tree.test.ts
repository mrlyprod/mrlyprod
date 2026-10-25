import { expect, test } from "bun:test";
import { ICONS } from "../lib/icons.gen.js";
import { branch } from "./tree.ts";

type Fake = Element & {
  tag: string;
  kids: Fake[];
  attrs: Record<string, string>;
  href?: string;
};

const node = (tag: string): Fake =>
  Object.assign({} as Element, {
    tag,
    kids: [] as Fake[],
    attrs: {} as Record<string, string>,
    dataset: {},
    append(this: Fake, ...more: Fake[]) {
      this.kids.push(...more);
    },
    setAttribute(this: Fake, name: string, value: string) {
      this.attrs[name] = value;
    },
  });

test("a tree folder summary starts with the expand_more chevron, then its link", () => {
  Object.assign(globalThis, {
    document: { createElement: node, createElementNS: (_space: string, tag: string) => node(tag) },
  });
  const made: Element = branch("/git/", { n: "lib", k: "d" }, "site/lib");
  const summary = (made as Fake).kids[0].kids[0];
  Reflect.deleteProperty(globalThis, "document");
  const [svg, a] = summary.kids;
  expect([svg.tag, svg.kids[0].attrs.d, a.tag, a.href]).toEqual(["svg", ICONS.expand_more, "a", "/git/site/lib/"]);
});
