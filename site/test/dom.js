import { act } from "react";
import { createRoot } from "react-dom/client";

const GLOBALS = [
  "document",
  "window",
  "HTMLIFrameElement",
  "IS_REACT_ACT_ENVIRONMENT",
  "requestAnimationFrame",
  "cancelAnimationFrame",
];

export function node(tag, nodeType = 1) {
  const kids = [];
  const self = {
    nodeType,
    nodeName: tag.toUpperCase(),
    tagName: tag.toUpperCase(),
    namespaceURI: "http://www.w3.org/1999/xhtml",
    childNodes: kids,
    style: {
      setProperty(name, value) {
        this[name] = value;
      },
      removeProperty(name) {
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
    appendChild(kid) {
      return self.insertBefore(kid, null);
    },
    removeChild(kid) {
      kids.splice(kids.indexOf(kid), 1);
      kid.parentNode = null;
      return kid;
    },
    insertBefore(kid, before) {
      kid.parentNode?.removeChild(kid);
      kid.parentNode = self;
      const at = kids.indexOf(before);
      kids.splice(at < 0 ? kids.length : at, 0, kid);
      return kid;
    },
    setAttribute(name, value) {
      self.attributes[name] = String(value);
    },
    removeAttribute(name) {
      delete self.attributes[name];
    },
    getAttribute(name) {
      return self.attributes[name] ?? null;
    },
    hasAttribute(name) {
      return name in self.attributes;
    },
    contains(kid) {
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

export const doc = Object.assign(node("#document", 9), {
  createElement: (tag) => node(tag),
  createElementNS: (_ns, tag) => node(tag),
  createTextNode: (text) => Object.assign(node("#text", 3), { nodeValue: text }),
  createComment: () => node("#comment", 8),
  activeElement: null,
  hidden: false,
  defaultView: globalThis,
  documentElement: node("html"),
  querySelector: () => null,
  querySelectorAll: () => [],
});

doc.body = doc.appendChild(node("html")).appendChild(node("body"));

export const find = (at, test) =>
  test(at) ? at : at.childNodes.reduce((found, kid) => found ?? find(kid, test), null);

export const all = (at, test) => [...(test(at) ? [at] : []), ...at.childNodes.flatMap((kid) => all(kid, test))];

export const text = (at) =>
  at.nodeType === 3 ? at.nodeValue : at.childNodes.length ? at.childNodes.map(text).join("") : (at.textContent ?? "");

export const classed = (name) => (at) => (at.attributes?.class ?? "").split(" ").includes(name);

export async function mount(fn) {
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
  const draw = (one) => act(async () => root.render(one));
  try {
    return await fn(draw, host);
  } finally {
    await act(async () => root.unmount());
    doc.body.removeChild(host);
    for (const name of GLOBALS) delete globalThis[name];
  }
}
