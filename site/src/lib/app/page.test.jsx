import { expect, test } from "bun:test";
import { act, useEffect } from "react";
import { renderToStaticMarkup } from "react-dom/server";
import { all, classed, find, mount, text } from "../../../test/dom.js";
import { Fold, Group } from "./controls.jsx";
import { Bar } from "./frame.jsx";
import { hit } from "./keys.jsx";
import { called, carried, Held, keymap, knobbed, live, menu, resume, Scene, tail, toggle, undoing } from "./page.jsx";

const handle = () => {
  const seen = [];
  return { seen, pause: () => seen.push("pause"), play: () => seen.push("play") };
};

test("Space during a take ends the take and leaves the scene alone; otherwise it flips pause", () => {
  const one = handle();
  const cuts = [];
  const cut = () => cuts.push("cut");
  const flips = [toggle(one, false, true, cut), toggle(one, false, false, cut), toggle(one, true, false, cut)];
  expect([flips, one.seen, cuts]).toEqual([[false, true, false], ["pause", "play"], ["cut"]]);
});

test("a take begun while paused plays the scene first and clears the pause", () => {
  const one = handle();
  expect([resume(one, true), resume(one, false), one.seen]).toEqual([false, false, ["play"]]);
});

test("the Export menu holds the video rows of a recording page, then the rows a widget hands up", () => {
  const began = [];
  const rows = menu(true, (at) => began.push(at), [
    ["SVG", () => {}],
    ["OBJ", () => {}],
  ]);
  expect(rows.map(([text]) => text)).toEqual(["Video 9:16", "Video 16:9", "SVG", "OBJ"]);
  rows[1][1]();
  expect(began).toEqual(["16:9"]);
  expect(menu(false, () => {}, [["CSV", () => {}]]).map(([text]) => text)).toEqual(["CSV"]);
  expect(menu(false, () => {})).toEqual([]);
});

test("a keys row whose when is false leaves the map for that value, so it is no primary, no hint and no live key", () => {
  const keys = [
    { key: "Enter", label: "Again", act: "again", button: true, when: (value) => !value.roll },
    { key: ["ArrowUp", "ArrowDown"], label: "Reach", act: "reach", when: (value) => !value.roll },
    { key: "s", label: "Step", act: "step" },
  ];
  const map = (roll) => keymap(live(keys, { roll }));
  expect([map(1).map((row) => row.label), map(0).map((row) => row.label)]).toEqual([
    ["Pause", "Step", "Reroll", "Zen", "Keys"],
    ["Pause", "Again", "Reach", "Step", "Reroll", "Zen", "Keys"],
  ]);
  expect([hit(map(1), { key: "Enter" }), hit(map(0), { key: "Enter" })?.act]).toEqual([null, "again"]);
});

test("e and i follow Reroll, i only on a page with about or readouts, and an app row on e keeps its key", () => {
  const labels = (keys, more) => keymap(keys, more).map((row) => row.label);
  expect(labels([], tail(true))).toEqual(["Pause", "Reroll", "Export", "Info", "Zen", "Keys"]);
  expect(labels([], tail(false))).toEqual(["Pause", "Reroll", "Export", "Zen", "Keys"]);
  expect(labels([{ key: "e", label: "Erase", act: "erase" }], tail(true))).toEqual([
    "Pause",
    "Erase",
    "Reroll",
    "Info",
    "Zen",
    "Keys",
  ]);
});

test("a hidden app row on r gives the key back to the base row", () => {
  const keys = [{ key: "r", label: "Roll", act: "roll", when: (value) => !value.roll }];
  const act = (roll) => hit(keymap(live(keys, { roll })), { key: "r" })?.act;
  expect([act(0), act(1)]).toEqual(["roll", "reroll"]);
});

test("a page with units draws nothing, so its still stays up, until the units are ready", () => {
  const still = {
    gone: false,
    remove() {
      this.gone = true;
    },
  };
  expect(renderToStaticMarkup(<Held units={() => new Promise(() => {})} still={still} />)).toBe("");
  expect(still.gone).toBe(false);
});

test("a page with about draws the Info card; one with no about and no readouts draws none", () => {
  globalThis.location = { search: "?seed=7", pathname: "/x/", origin: "https://mrly.net" };
  const html = (about) =>
    renderToStaticMarkup(
      <Held Widget={() => null} defaults={{ seed: 0 }} keys={[]} actions={{}} about={about} id="x" />,
    );
  const card = (about) => html(about).includes('<dialog class="modal info" aria-label="Info">');
  expect([card(["A carpet of cells."]), card(undefined)]).toEqual([true, false]);
  delete globalThis.location;
});

test("a value change on a scene with set keeps the canvas element and hands set the value; one without set replaces it, with one onReady per run", async () => {
  const sets = [];
  const handles = [];
  const live = () => ({ draw: () => {}, set: (next) => sets.push(next) });
  const dumb = () => ({ draw: () => {} });
  const trial = (make) =>
    mount(async (draw, host) => {
      const ready = (handle) => handles.push(handle?.canvas);
      await draw(<Scene make={make} value={{ seed: 1 }} onReady={ready} />);
      const first = host.firstChild;
      await draw(<Scene make={make} value={{ seed: 2 }} onReady={ready} />);
      return [first.tagName, first === host.firstChild];
    });
  const kept = await trial(live);
  const seen = handles.length;
  const swapped = await trial(dumb);
  expect([kept, swapped, sets, seen, handles.length]).toEqual([
    ["CANVAS", true],
    ["CANVAS", false],
    [{ seed: 2 }],
    2,
    6,
  ]);
});

const open = async (props, look, search = "?seed=7") => {
  globalThis.location = { search, pathname: "/x/", origin: "https://mrly.net", hash: "" };
  globalThis.sessionStorage = { getItem: () => null, setItem: () => {} };
  try {
    return await mount(async (draw, host) => {
      const one = (
        <Held
          Widget={() => null}
          defaults={{ seed: 0 }}
          keys={[]}
          actions={{}}
          about={[]}
          id="x"
          title="X"
          {...props}
        />
      );
      await draw(one);
      await draw(one);
      return look(host);
    });
  } finally {
    delete globalThis.location;
    delete globalThis.sessionStorage;
  }
};

const press = (key) => act(async () => globalThis.dispatchEvent(Object.assign(new Event("keydown"), { key })));

const within = (host, name) => find(host, classed(name));

const staged =
  (made) =>
  ({ value, onReady }) => <Scene make={made} value={value} onReady={onReady} />;

test("a scene with toggle and playing owns Space: Space flips the scene, and the paused word and the Space row follow playing", async () => {
  let playing = true;
  let flips = 0;
  const Widget = staged(() => ({
    draw: () => {},
    toggle: () => {
      playing = !playing;
      flips++;
    },
    playing: () => playing,
  }));
  const seen = await open({ Widget }, async (host) => {
    const before = text(within(host, "facts"));
    await press(" ");
    return [before, text(within(host, "facts")), flips, text(within(host, "keys")).includes("SpacePlay")];
  });
  expect(seen).toEqual(["seed 7", "seed 7paused", 1, true]);
});

test("a keys row label may be a function of the value and the scene, read again after each act", async () => {
  let on = false;
  const Widget = staged(() => ({ draw: () => {}, flip: () => (on = !on), on: () => on }));
  const keys = [
    { key: "Enter", label: (value, scene) => (scene?.on() ? `Stop ${value.seed}` : "Go"), act: "go", button: true },
  ];
  const seen = await open({ Widget, keys, actions: { go: (scene) => scene.flip() } }, async (host) => {
    const label = () => text(find(within(host, "actions"), classed("primary")));
    const before = label();
    await press("Enter");
    return [before, label()];
  });
  expect(seen).toEqual(["Go", "Stop 7"]);
});

test("a still page, or a scene that returns still, binds no Space and never reads paused", async () => {
  const space = (props) => open(props, (host) => text(within(host, "keys")).includes("Space"));
  const still = staged(() => ({ still: true, draw: () => {} }));
  const moving = staged(() => ({ draw: () => {} }));
  expect([
    await space({ calm: true, Widget: moving }),
    await space({ Widget: still }),
    await space({ Widget: moving }),
  ]).toEqual([false, false, true]);
});

test("the status pill holds the seed and the first three readouts, and Info lists them all", async () => {
  const rows = ["a", "b", "c", "d", "e", "f"].map((one, n) => [one, n]);
  const Widget = ({ onStatus }) => {
    useEffect(() => onStatus(rows), []);
    return null;
  };
  const seen = await open({ Widget }, (host) => [
    all(within(host, "facts"), classed("out")).map(text),
    all(within(host, "info"), (at) => at.tagName === "DT").length,
  ]);
  expect(seen).toEqual([["a 0", "b 1", "c 2"], 6]);
});

test("Cmd or Ctrl with z undoes, with Shift or y redoes, and a text field keeps its own undo", () => {
  const field = { closest: (selector) => (selector.includes("input") ? {} : null) };
  const key = (key, more = {}) =>
    undoing({ key, metaKey: true, shiftKey: false, ...more, target: more.target ?? { closest: () => null } });
  expect([
    key("z"),
    key("Z", { shiftKey: true }),
    key("y"),
    key("z", { metaKey: false, ctrlKey: true }),
    key("z", { target: field }),
    key("z", { metaKey: false }),
    key("k"),
  ]).toEqual([-1, 1, 1, -1, 0, 0, 0]);
});

test("Reset Knobs puts every spec row back to its default but the keys the design slot holds", () => {
  const spec = [
    { key: "code", def: "" },
    { key: "level", def: 3 },
    { key: "tint", def: "site" },
  ];
  expect(knobbed(spec, ["code"])).toEqual({ level: 3, tint: "site" });
});

test("a url with no design takes the current one when it fits the slot, and keeps its own otherwise or when an empty code means something", () => {
  const slot = { dim: 2, base: "base", code: "code" };
  const at = (search, one) => String(carried(new URLSearchParams(search), slot, one));
  expect([
    at("seed=1", { dim: 2, base: 3, code: "42" }),
    at("code=7", { dim: 2, base: 3, code: "42" }),
    at("seed=1", { dim: 3, base: 2, code: "23" }),
    at("seed=1", null),
    String(carried(new URLSearchParams("seed=1"), { ...slot, blank: "Random" }, { dim: 2, base: 3, code: "42" })),
  ]).toEqual(["seed=1&base=3&code=42", "code=7", "seed=1", "seed=1", "seed=1"]);
});

test("the widget is handed the units the page loaded", async () => {
  const seen = [];
  const Widget = ({ units }) => {
    seen.push(units);
    return null;
  };
  await open({ Widget, units: async () => ({ ready: Promise.resolve(), math: "m" }) }, () => {});
  expect(seen.at(-1)).toEqual({ math: "m" });
});

test("a view row also draws as the phone view switch in the actions bar, only while its when holds", async () => {
  const spec = (when) => [
    { key: "view", label: "View", kind: "segment", def: "walk", options: ["walk", "plane"], group: "View", when },
  ];
  const has = (when) =>
    open({ spec: spec(when), defaults: { seed: 0, view: "walk" } }, (host) =>
      Boolean(find(within(host, "actions"), classed("views"))),
    );
  expect([await has(undefined), await has(() => false)]).toEqual([true, false]);
});

test("the pane leads with the view group, then the widget folds in mount order, one placed after a group right behind it, then the other groups", async () => {
  const spec = [
    { key: "size", label: "Size", kind: "slider", def: 1, min: 0, max: 9, step: 1, group: "Sheet" },
    { key: "view", label: "View", kind: "segment", def: "a", options: ["a", "b"], group: "Stack" },
    { key: "tint", label: "Tint", kind: "toggle", def: 0, group: "Look" },
  ];
  const Widget = () => (
    <Bar side="right">
      <Fold name="Palette" after="Look">
        x
      </Fold>
      <Fold name="Design">x</Fold>
      <Group name="Presets" after="Sheet">
        x
      </Group>
      <Fold name="Source">x</Fold>
    </Bar>
  );
  const seen = await open({ Widget, spec, defaults: { seed: 0, size: 1, view: "a", tint: 0 } }, (host) =>
    within(host, "controls")
      .childNodes.map((at) => [Number(at.style.order), text(find(at, (one) => one.tagName === "H3"))])
      .sort(([a], [b]) => a - b)
      .map(([, name]) => name),
  );
  expect(seen).toEqual(["Stack", "Design", "Source", "Sheet", "Presets", "Look", "Palette"]);
});

test("a keep row leaves the asked value in the url while the widget gets the capped one", async () => {
  const real = globalThis.setTimeout;
  const wrote = [];
  const seen = [];
  globalThis.history = { state: null, replaceState: (_state, _title, url) => wrote.push(url) };
  const spec = (keep) => [
    { key: "level", label: "Level", kind: "slider", def: 2, min: 1, max: () => 4, step: 1, keep },
  ];
  const Widget = ({ value }) => {
    seen.push(value.level);
    return null;
  };
  try {
    for (const keep of [true, false]) {
      const queued = [];
      globalThis.setTimeout = (fn) => queued.push(fn);
      await open(
        { Widget, spec: spec(keep), defaults: { seed: 0, level: 2 } },
        () =>
          queued.forEach((fn) => {
            fn();
          }),
        "?seed=7&level=9",
      );
    }
  } finally {
    globalThis.setTimeout = real;
    delete globalThis.history;
  }
  expect([wrote, [...new Set(seen)]]).toEqual([["/x/?seed=7&level=4"], [4]]);
});

test("an export takes the name the page gives for the value, or the id and the seed", () => {
  expect([
    called((value, units) => `six-${value.code}-${units.view}`, "six", { seed: 7, code: "495" }, { view: "iso" }),
    called(undefined, "six", { seed: 7 }),
  ]).toEqual(["six-495-iso", "six-7"]);
});
