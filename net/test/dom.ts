import { act, type ReactNode } from "react";
import { createRoot } from "react-dom/client";

/* TYPES */

export type Style = {
  [name: string]: unknown;
  setProperty: (name: string, value: string) => void;
  removeProperty: (name: string) => void;
};

export type FakeNode = {
  nodeType: number;
  nodeName: string;
  tagName: string;
  namespaceURI: string;
  childNodes: FakeNode[];
  style: Style;
  attributes: Partial<Record<string, string>>;
  dataset: Partial<Record<string, string>>;
  parentNode: FakeNode | null;
  clientWidth: number;
  clientHeight: number;
  nodeValue?: string;
  textContent?: string;
  readonly firstChild: FakeNode | null;
  readonly lastChild: FakeNode | null;
  readonly nextSibling: FakeNode | null;
  readonly ownerDocument: FakeDoc;
  appendChild: (kid: FakeNode) => FakeNode;
  removeChild: (kid: FakeNode) => FakeNode;
  insertBefore: (kid: FakeNode, before: FakeNode | null) => FakeNode;
  setAttribute: (name: string, value: string) => void;
  removeAttribute: (name: string) => void;
  getAttribute: (name: string) => string | null;
  hasAttribute: (name: string) => boolean;
  contains: (kid: FakeNode | null) => boolean;
  getRootNode: () => FakeDoc;
  addEventListener: (
    type: string,
    listener: EventListenerOrEventListenerObject,
    options?: boolean | AddEventListenerOptions,
  ) => void;
  removeEventListener: (
    type: string,
    listener: EventListenerOrEventListenerObject,
    options?: boolean | EventListenerOptions,
  ) => void;
  dispatchEvent: (event: Event) => boolean;
};

declare module "react-dom/client" {
  function createRoot(container: FakeNode, options?: RootOptions): Root;
}

type DocApi = {
  createElement: (tag: string) => FakeNode;
  createElementNS: (ns: string | null, tag: string) => FakeNode;
  createTextNode: (text: string) => FakeNode;
  createComment: (data: string) => FakeNode;
  activeElement: FakeNode | null;
  hidden: boolean;
  defaultView: typeof globalThis;
  documentElement: FakeNode;
  querySelector: (selector: string) => FakeNode | null;
  querySelectorAll: (selector: string) => FakeNode[];
};

export type FakeDoc = FakeNode & DocApi & { body: FakeNode };

export type Test = (at: FakeNode) => unknown;

export type Draw = (one: ReactNode) => Promise<void>;

const GLOBALS = [
  "document",
  "window",
  "HTMLIFrameElement",
  "IS_REACT_ACT_ENVIRONMENT",
  "requestAnimationFrame",
  "cancelAnimationFrame",
];

export function node(tag: string, nodeType = 1): FakeNode {
  const kids: FakeNode[] = [];
  const self: FakeNode = {
    nodeType,
    nodeName: tag.toUpperCase(),
    tagName: tag.toUpperCase(),
    namespaceURI: "http://www.w3.org/1999/xhtml",
    childNodes: kids,
    style: {
      setProperty(name: string, value: string) {
        this[name] = value;
      },
      removeProperty(name: string) {
        delete this[name];
      },
    },
    attributes: {},
    dataset: {},
    parentNode: null,
    clientWidth: 4,
    clientHeight: 4,
    get firstChild() {
      return kids[0] ?? null;
    },
    get lastChild() {
      return kids.at(-1) ?? null;
    },
    get nextSibling() {
      const row = self.parentNode?.childNodes ?? [];
      return row[row.indexOf(self) + 1] ?? null;
    },
    get ownerDocument() {
      return doc;
    },
    appendChild(kid: FakeNode) {
      return self.insertBefore(kid, null);
    },
    removeChild(kid: FakeNode) {
      kids.splice(kids.indexOf(kid), 1);
      kid.parentNode = null;
      return kid;
    },
    insertBefore(kid: FakeNode, before: FakeNode | null) {
      kid.parentNode?.removeChild(kid);
      kid.parentNode = self;
      const row: readonly (FakeNode | null)[] = kids;
      const at = row.indexOf(before);
      kids.splice(at < 0 ? kids.length : at, 0, kid);
      return kid;
    },
    setAttribute(name: string, value: string) {
      self.attributes[name] = String(value);
    },
    removeAttribute(name: string) {
      delete self.attributes[name];
    },
    getAttribute(name: string) {
      return self.attributes[name] ?? null;
    },
    hasAttribute(name: string) {
      return name in self.attributes;
    },
    contains(kid: FakeNode | null) {
      for (let at = kid; at; at = at.parentNode) if (at === self) return true;
      return false;
    },
    getRootNode: () => doc,
    addEventListener() {},
    removeEventListener() {},
    dispatchEvent: () => true,
  };
  return self;
}

export const doc = Object.assign<FakeNode, DocApi>(node("#document", 9), {
  createElement: (tag: string) => node(tag),
  createElementNS: (_ns: string | null, tag: string) => node(tag),
  createTextNode: (text: string) => Object.assign(node("#text", 3), { nodeValue: text }),
  createComment: () => node("#comment", 8),
  activeElement: null,
  hidden: false,
  defaultView: globalThis,
  documentElement: node("html"),
  querySelector: () => null,
  querySelectorAll: () => [],
}) as FakeDoc;

doc.body = doc.appendChild(node("html")).appendChild(node("body"));

export const find = (at: FakeNode, test: Test): FakeNode | null =>
  test(at) ? at : at.childNodes.reduce<FakeNode | null>((found, kid) => found ?? find(kid, test), null);

export const all = (at: FakeNode, test: Test): FakeNode[] => [
  ...(test(at) ? [at] : []),
  ...at.childNodes.flatMap((kid) => all(kid, test)),
];

export const text = (at: FakeNode): string | undefined =>
  at.nodeType === 3 ? at.nodeValue : at.childNodes.length ? at.childNodes.map(text).join("") : (at.textContent ?? "");

export const classed = (name: string) => (at: FakeNode) => (at.attributes?.class ?? "").split(" ").includes(name);

export async function mount<T>(fn: (draw: Draw, host: FakeNode) => T): Promise<Awaited<T>> {
  Object.assign(globalThis, {
    document: doc,
    window: globalThis,
    HTMLIFrameElement: class {},
    IS_REACT_ACT_ENVIRONMENT: true,
    requestAnimationFrame: () => 0,
    cancelAnimationFrame: () => {},
  });
  const host = doc.body.appendChild(node("div"));
  const root = createRoot(host);
  const draw = (one: ReactNode) => act(async () => root.render(one));
  try {
    return await fn(draw, host);
  } finally {
    await act(async () => root.unmount());
    doc.body.removeChild(host);
    const scope: Record<string, unknown> = globalThis;
    for (const name of GLOBALS) delete scope[name];
  }
}
