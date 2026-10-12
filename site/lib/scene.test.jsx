import { expect, test } from 'bun:test';
import { act } from 'react';
import { createRoot } from 'react-dom/client';
import { renderToStaticMarkup } from 'react-dom/server';
import { hit } from './keys.js';
import { Held, Scene, keymap, live, menu, resume, toggle } from './scene.jsx';

const handle = () => {
  const seen = [];
  return { seen, pause: () => seen.push('pause'), play: () => seen.push('play') };
};

test('Space during a take ends the take and leaves the scene alone; otherwise it flips pause', () => {
  const one = handle();
  const cuts = [];
  const cut = () => cuts.push('cut');
  const flips = [toggle(one, false, true, cut), toggle(one, false, false, cut), toggle(one, true, false, cut)];
  expect([flips, one.seen, cuts]).toEqual([[false, true, false], ['pause', 'play'], ['cut']]);
});

test('a take begun while paused plays the scene first and clears the pause', () => {
  const one = handle();
  expect([resume(one, true), resume(one, false), one.seen]).toEqual([false, false, ['play']]);
});

test('the Export menu holds the video rows of a recording page, then the rows a widget hands up', () => {
  const began = [];
  const rows = menu(true, (at) => began.push(at), [['SVG', () => {}], ['OBJ', () => {}]]);
  expect(rows.map(([text]) => text)).toEqual(['Video 9:16', 'Video 16:9', 'SVG', 'OBJ']);
  rows[1][1]();
  expect(began).toEqual(['16:9']);
  expect(menu(false, () => {}, [['CSV', () => {}]]).map(([text]) => text)).toEqual(['CSV']);
  expect(menu(false, () => {})).toEqual([]);
});

test('a keys row whose when is false leaves the map for that value, so it is no primary, no hint and no live key', () => {
  const keys = [
    { key: 'Enter', label: 'Again', act: 'again', button: true, when: (value) => !value.roll },
    { key: ['ArrowUp', 'ArrowDown'], label: 'Reach', act: 'reach', when: (value) => !value.roll },
    { key: 's', label: 'Step', act: 'step' },
  ];
  const map = (roll) => keymap(live(keys, { roll }));
  expect([map(1).map((row) => row.label), map(0).map((row) => row.label)]).toEqual([['Pause', 'Step', 'Reroll', 'Zen', 'Keys'], ['Pause', 'Again', 'Reach', 'Step', 'Reroll', 'Zen', 'Keys']]);
  expect([hit(map(1), { key: 'Enter' }), hit(map(0), { key: 'Enter' })?.act]).toEqual([null, 'again']);
});

test('a hidden app row on r gives the key back to the base row', () => {
  const keys = [{ key: 'r', label: 'Roll', act: 'roll', when: (value) => !value.roll }];
  const act = (roll) => hit(keymap(live(keys, { roll })), { key: 'r' })?.act;
  expect([act(0), act(1)]).toEqual(['roll', 'reroll']);
});

test('a page with units draws nothing, so its still stays up, until the units are ready', () => {
  const still = { gone: false, remove() { this.gone = true; } };
  expect(renderToStaticMarkup(<Held units={() => new Promise(() => {})} still={still} />)).toBe('');
  expect(still.gone).toBe(false);
});

const node = (tag, nodeType = 1) => {
  const kids = [];
  const self = {
    nodeType,
    nodeName: tag.toUpperCase(),
    tagName: tag.toUpperCase(),
    namespaceURI: 'http://www.w3.org/1999/xhtml',
    childNodes: kids,
    style: {},
    attributes: {},
    parentNode: null,
    clientWidth: 4,
    clientHeight: 4,
    get firstChild() {
      return kids[0] ?? null;
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
};

const doc = Object.assign(node('#document', 9), { createElement: (tag) => node(tag), createElementNS: (ns, tag) => node(tag), createTextNode: (text) => Object.assign(node('#text', 3), { nodeValue: text }), createComment: () => node('#comment', 8), activeElement: null, hidden: false, defaultView: globalThis });

doc.body = doc.appendChild(node('html')).appendChild(node('body'));

const mount = (fn) => {
  Object.assign(globalThis, { document: doc, window: globalThis, HTMLIFrameElement: class {}, IS_REACT_ACT_ENVIRONMENT: true, requestAnimationFrame: () => 0, cancelAnimationFrame: () => {} });
  const host = doc.body.appendChild(node('div'));
  const root = createRoot(host);
  const draw = (node) => act(() => root.render(node));
  const out = fn(draw, host);
  act(() => root.unmount());
  doc.body.removeChild(host);
  for (const name of ['document', 'window', 'HTMLIFrameElement', 'IS_REACT_ACT_ENVIRONMENT', 'requestAnimationFrame', 'cancelAnimationFrame']) delete globalThis[name];
  return out;
};

test('a value change on a scene with set keeps the canvas element and hands set the value; one without set replaces it, with one onReady per run', () => {
  const sets = [];
  const handles = [];
  const live = () => ({ draw: () => {}, set: (next) => sets.push(next) });
  const dumb = () => ({ draw: () => {} });
  const trial = (make) => mount((draw, host) => {
    const ready = (handle) => handles.push(handle && handle.canvas);
    draw(<Scene make={make} value={{ seed: 1 }} onReady={ready} />);
    const first = host.firstChild;
    draw(<Scene make={make} value={{ seed: 2 }} onReady={ready} />);
    return [first.tagName, first === host.firstChild];
  });
  const kept = trial(live);
  const seen = handles.length;
  const swapped = trial(dumb);
  expect([kept, swapped, sets, seen, handles.length]).toEqual([['CANVAS', true], ['CANVAS', false], [{ seed: 2 }], 2, 6]);
});
