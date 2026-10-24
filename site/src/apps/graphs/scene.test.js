import { expect, test } from "bun:test";
import * as math from "@mrlyprod/mrly/math";
import { defaults, tidy, unitOf } from "../../lib/app/spec.js";
import { rng } from "../../lib/rand.js";
import { palette } from "../../lib/theme/palette.gen.js";
import { COOL, PAD, TICK } from "./engine.js";
import { HOLD, make, PAGE, SPEC, tones } from "./scene.js";
import { detail, ORBIT, START, VIEWS } from "./space.js";

const brief = (arg) =>
  ArrayBuffer.isView(arg)
    ? arg.length
    : typeof arg === "function"
      ? "thing"
      : typeof arg === "object" && arg !== null
        ? Object.keys(arg).join(",")
        : arg;

const pen = (log) =>
  new Proxy(
    {},
    {
      get:
        (_, key) =>
        (...args) =>
          log.push([key, ...args]),
      set: (_, key, v) => {
        log.push([key, v]);
        return true;
      },
    },
  );

function target() {}

const fake = (log) => {
  const any = (path) =>
    new Proxy(target, {
      get: (_, key) => (typeof key === "symbol" ? undefined : any(`${path}.${key}`)),
      set: (_, key, value) => {
        log.push([`${path}.${key}=`, brief(value)]);
        return true;
      },
      apply: (_, __, args) => {
        log.push([path, ...args.map(brief)]);
        return any(path);
      },
      construct: (_, args) => {
        log.push([`new ${path}`, ...args.map(brief)]);
        return any(path);
      },
    });
  return any("three");
};

const HOME = { ...defaults(SPEC), seed: 1 };

const open = (value = {}, { still = false, three = false, seed = 1, w = 800, h = 600 } = {}) => {
  const log = [];
  let made = 0;
  const view = {
    rand: rng(seed),
    look: () => ({ paper: "#000000", accent: "#008cff" }),
    still,
    w,
    h,
    dpr: 2,
    t: 0,
    wake: () => (view.still = false),
  };
  const canvas = {
    getContext: () => {
      made++;
      return pen(log);
    },
  };
  const scene = make(canvas, view, { ...HOME, seed, ...value, math, three: three ? fake(log) : undefined });
  const at = (t) => {
    view.t = t;
    log.length = 0;
    scene.draw();
    return log;
  };
  const run = (to, by = 16) => {
    for (let t = view.t + by; t <= to; t += by) at(t);
    return log;
  };
  return {
    view,
    scene,
    log,
    at,
    run,
    made: () => made,
    count: (name) => log.filter(([key]) => key === name).length,
    names: () => log.map(([key]) => key),
  };
};

const arcs = (log) => log.filter(([name]) => name === "arc");

const lines = (log) => log.filter(([name]) => name === "lineTo");

const CUBE = { dim: 3, base: 2, view: "cube", graph: "tunnel" };

const NODES = 64;

const BRANCHES = 88;

test("the page opens on the demo's carpet: the plane design at base 3 with no code, its core on the lattice at level 2", () => {
  expect(tidy(SPEC, {}, { math })).toEqual({
    dim: 2,
    view: "cube",
    base: 3,
    code: "",
    number: 3,
    layout: "lattice",
    graph: "core",
    level: 2,
    dots: 3,
    camera: "eye",
    spin: 0,
    tint: "",
  });
  expect([
    tidy(SPEC, { level: 9 }, { math }).level,
    tidy(SPEC, { level: 9, ...CUBE, graph: "edge" }, { math }).level,
    tidy(SPEC, { dim: 3, view: "hex", graph: "tunnel" }).graph,
  ]).toEqual([4, 2, "core"]);
});

test("the view picks the cube or its hex slice for a 3D design alone, the graph is the library's, and the level reads against its cap", () => {
  const level = SPEC.find((row) => row.key === "level");
  const row = (key) => SPEC.find((one) => one.key === key);
  expect([row("view").when({ dim: 2 }), row("view").when({ dim: 3 }), row("graph").when({}), SPEC[0].key]).toEqual([
    false,
    true,
    false,
    "dim",
  ]);
  expect([unitOf(level, HOME, { math }), unitOf(level, { ...HOME, ...CUBE, graph: "edge" }, { math })]).toEqual([
    "of 4",
    "of 2",
  ]);
  expect(SPEC.filter((one) => one.group === "Design").map((one) => one.key)).toEqual(["view", "number", "level"]);
});

test("an export is named by the design shown, its view and its graph, the empty code the carpet", () => {
  expect([
    PAGE.name(HOME, { math }),
    PAGE.name(HOME),
    PAGE.name({ ...HOME, ...CUBE, view: "hex", graph: "dual" }, { math }),
  ]).toEqual(["graphs-495-flat-core", "graphs-carpet-flat-core", "graphs-23-hex-dual"]);
});

test("the page takes a design of either dimension, and the keys step the level and the graph from the View menu", () => {
  expect([PAGE.design, PAGE.keys.map((row) => row.menu)]).toEqual([
    { dim: "dim", base: "base", code: "code" },
    ["Edit", "View", "View"],
  ]);
});

test("the first draw lays every branch then every node in the role hues, a draw with nothing new is a no-op, and a resize or a theme redraws", () => {
  const live = open();
  expect(live.log).toEqual([]);
  expect(live.scene.every).toBeUndefined();
  live.at(0);
  expect([lines(live.log).length, arcs(live.log).length]).toEqual([BRANCHES, NODES]);
  const names = live.names();
  expect(names.lastIndexOf("lineTo")).toBeLessThan(names.indexOf("arc"));
  expect(live.log).toContainEqual(["fillStyle", palette.pink]);
  expect(live.log).toContainEqual(["strokeStyle", palette.blue]);
  expect(live.at(1000)).toEqual([]);
  live.scene.size();
  expect(arcs(live.at(1000)).length).toBe(NODES);
  live.scene.theme();
  expect(arcs(live.at(1000)).length).toBe(NODES);
  expect(tones({}, live.view, "")).toEqual([palette.gray, palette.yellow, palette.blue, palette.pink]);
});

test("a tint hue turns the role hues into fades of that one ink", () => {
  const view = { look: () => ({ paper: "#000000", accent: "#008cff" }) };
  expect(tones({}, view, "blue")).toEqual(["rgb(0, 56, 102)", "rgb(0, 84, 153)", "rgb(0, 112, 204)", "#008cff"]);
});

test("the dots knob sets the radius in css pixels and the line width with it, the network centred inside the pad", () => {
  const live = open({ dots: 6 });
  live.at(0);
  const xs = arcs(live.log).map(([, x]) => x);
  const ys = arcs(live.log).map(([, , y]) => y);
  expect(Math.min(...xs) + Math.max(...xs)).toBeCloseTo(800, 6);
  expect(Math.min(...ys) + Math.max(...ys)).toBeCloseTo(600, 6);
  expect(arcs(live.log)[0][3]).toBe(12);
  expect(live.log).toContainEqual(["lineWidth", 4]);
  expect(Math.max(...ys) - Math.min(...ys)).toBeCloseTo(600 - 2 * (PAD * 2 + 12), 6);
});

test("a value change keeps the canvas: a new design restudies on the same context, a look change only redraws", () => {
  const live = open();
  live.at(0);
  const first = live.scene.facts();
  live.scene.set({ ...HOME, dots: 5 });
  expect(live.scene.facts()).toBe(first);
  expect(arcs(live.at(10))[0][3]).toBe(10);
  live.scene.set({ ...HOME, ...CUBE, view: "hex", graph: "dual" });
  expect(live.scene.facts()).toMatchObject({ space: "hex", graph: "dual", nodes: 486 });
  expect(arcs(live.at(20)).length).toBe(486);
  live.scene.set({ ...HOME, code: "487", level: 3 });
  expect(live.scene.facts()).toMatchObject({ code: "487", level: 3 });
  expect(live.made()).toBe(1);
});

test("a switch between the flat and the cube stage asks for a new canvas and keeps the old plan", () => {
  const live = open();
  const first = live.scene.facts();
  expect(live.scene.set({ ...HOME, ...CUBE })).toBe(false);
  expect(live.scene.facts()).toBe(first);
  expect(live.scene.set({ ...HOME, ...CUBE, view: "hex" })).not.toBe(false);
});

test("a force layout relaxes from its own ticks as t runs, the same at a t in one jump or by frames, until it cools and rests", () => {
  const stepped = open({ layout: "force" });
  stepped.at(0);
  stepped.run(600, 50);
  const jumped = open({ layout: "force" });
  jumped.at(0);
  jumped.at(600);
  expect(arcs(jumped.log)).toEqual(arcs(stepped.log));
  expect(stepped.scene.relaxed()).toMatchObject({ on: true, cooled: false, ticks: Math.floor(600 / TICK) });
  stepped.run(600 + COOL * TICK * 2, 50);
  const rested = stepped.scene.relaxed();
  expect(rested).toMatchObject({ on: false, cooled: true });
  expect(stepped.at(1e6)).toEqual([]);
  expect(stepped.scene.relaxed()).toEqual(rested);
});

test("the scene owns the pause: toggle pauses and resumes the relax, playing follows it, and a toggle once cooled or a reset starts it again from the lattice", () => {
  const live = open({ layout: "force" });
  live.at(0);
  live.at(300);
  expect(live.scene.playing()).toBe(true);
  expect(live.scene.toggle()).toBe(false);
  expect(live.scene.playing()).toBe(false);
  const held = live.scene.relaxed().ticks;
  live.at(900);
  expect(live.scene.relaxed().ticks).toBe(held);
  expect(live.scene.toggle()).toBe(true);
  live.at(1000);
  live.at(1300);
  expect(live.scene.relaxed().ticks).toBe(held + 100);
  live.scene.reset();
  expect(live.scene.relaxed()).toMatchObject({ on: true, ticks: 0 });
  live.run(1300 + COOL * TICK * 2, 50);
  expect([live.scene.relaxed().cooled, live.scene.playing()]).toEqual([true, false]);
  live.scene.toggle();
  expect(live.scene.relaxed()).toMatchObject({ on: true, cooled: false, ticks: 0 });
  expect([open().scene.relaxed(), open().scene.playing()]).toEqual([null, false]);
});

test("a new seed jitters the relax afresh, a new layout swaps the lattice for the force and back without a restudy", () => {
  const live = open({ layout: "force" });
  live.at(0);
  const first = arcs(live.at(300));
  const facts = live.scene.facts();
  live.scene.set({ ...HOME, layout: "force", seed: 2 });
  live.at(310);
  expect(arcs(live.at(610))).not.toEqual(first);
  live.scene.set({ ...HOME, layout: "lattice", seed: 2 });
  expect([live.scene.relaxed(), live.scene.facts().layout, live.scene.facts().code]).toEqual([
    null,
    "lattice",
    facts.code,
  ]);
});

test("under reduced motion a small force network settles at once, a big one waits on the lattice until relax wakes it", () => {
  const small = open({ layout: "force" }, { still: true });
  expect(small.scene.relaxed()).toMatchObject({ on: false, cooled: true, ticks: COOL });
  const big = open({ layout: "force", level: 3 }, { still: true });
  expect(big.scene.relaxed()).toMatchObject({ on: false, cooled: false, ticks: 0 });
  big.scene.toggle();
  expect([big.view.still, big.scene.relaxed().on]).toEqual([false, true]);
});

test("a lock with no query rolls a worthy flat network at the page base, relaxes it, and folds it afresh once it cools and holds", () => {
  const lock = (seed) => {
    const view = {
      rand: rng(seed),
      look: () => ({ paper: "#000000", accent: "#008cff" }),
      still: false,
      lock: true,
      w: 800,
      h: 600,
      dpr: 2,
      t: 0,
    };
    const scene = make({ getContext: () => pen([]) }, view, { math, seed });
    return { view, scene };
  };
  const seen = Array.from({ length: 12 }, (_, seed) => {
    const { scene } = lock(seed);
    const { space, base, branches, layout } = scene.facts();
    scene.stop();
    return `${space} ${base} ${layout} ${branches > 0}`;
  });
  expect(new Set(seen)).toEqual(new Set(["flat 3 force true"]));
  const { view, scene } = lock(4);
  while (!scene.relaxed().cooled && view.t < 1e6) {
    view.t += 100;
    scene.draw();
  }
  const cooled = scene.relaxed();
  const from = view.t;
  for (; view.t < from + HOLD - 100; view.t += 100) scene.draw();
  expect(scene.relaxed()).toEqual(cooled);
  view.t = from + HOLD + 100;
  scene.draw();
  expect(scene.relaxed()).toMatchObject({ on: true, cooled: false, ticks: 0 });
  scene.stop();
});

test("the svg draws the dots and lines at css size in the shades on screen, and the json carries the facts and the network", () => {
  const live = open();
  live.at(0);
  const text = live.scene.svg();
  expect(text.startsWith('<svg xmlns="http://www.w3.org/2000/svg" width="400" height="300"')).toBe(true);
  expect((text.match(/<circle /g) ?? []).length).toBe(NODES);
  expect((text.match(/L[\d.]+ [\d.]+/g) ?? []).length).toBe(BRANCHES);
  expect(text).toContain(`fill="${palette.pink}"`);
  expect(text).toContain('r="3"');
  const facts = JSON.parse(live.scene.json());
  expect(facts).toMatchObject({ code: "495", nodes: NODES, branches: BRANCHES });
  expect([facts.network.dim, facts.network.nodes.length, facts.network.branches.length]).toEqual([2, NODES, BRANCHES]);
});

test("a cube builds one renderer that keeps its picture, two instanced meshes of the nodes and the branches in the role hues, and renders only what changed", () => {
  const live = open(CUBE, { three: true });
  expect(live.scene.facts()).toMatchObject({ space: "cube", nodes: 329, branches: 600 });
  expect(live.log).toContainEqual(["new three.WebGLRenderer", "canvas,alpha,antialias,preserveDrawingBuffer"]);
  expect(live.log.filter(([key]) => key === "new three.InstancedMesh").map((row) => row[3])).toEqual([329, 600]);
  expect(live.count("three.InstancedMesh.setColorAt")).toBe(929);
  expect(live.log).toContainEqual(["three.Color.set", palette.pink]);
  live.at(0);
  expect([live.count("three.InstancedMesh.setMatrixAt"), live.names().at(-1)]).toEqual([
    929,
    "three.WebGLRenderer.render",
  ]);
  expect(live.at(1000)).toEqual([]);
});

test("a new design on a cube keeps the renderer and the camera and swaps only the meshes", () => {
  const live = open(CUBE, { three: true });
  live.at(0);
  live.log.length = 0;
  live.scene.set({ ...HOME, ...CUBE, graph: "core" });
  live.view.t = 16;
  live.scene.draw();
  expect([
    live.count("new three.WebGLRenderer"),
    live.count("new three.PerspectiveCamera"),
    live.count("three.InstancedMesh.dispose"),
    live.count("new three.InstancedMesh"),
  ]).toEqual([0, 0, 2, 2]);
  expect(live.names().at(-1)).toBe("three.WebGLRenderer.render");
});

test("the cube opens still at the demo's eye, a drag turns it at once, a release glides to a stop, and spin turns it with t", () => {
  const pose = (log) => log.filter(([key]) => key === "three.PerspectiveCamera.position.set").at(-1);
  const live = open(CUBE, { three: true });
  const first = pose(live.at(0));
  expect(live.at(ORBIT / 8)).toEqual([]);
  live.scene.grab();
  live.log.length = 0;
  live.scene.turn(0.05, 0);
  const turned = pose(live.log);
  expect(turned).not.toEqual(first);
  live.at(ORBIT / 8 + 16);
  live.scene.release();
  const glide = [1, 2, 3].map((k) => pose(live.at(ORBIT / 8 + 16 + 16 * k)));
  expect(new Set(glide.map(String)).size).toBe(3);
  live.run(ORBIT / 8 + 4000);
  expect(live.at(ORBIT / 8 + 4100)).toEqual([]);
  const spun = open({ ...CUBE, spin: 1 }, { three: true });
  const at0 = pose(spun.at(0));
  expect(pose(spun.at(ORBIT / 4))).not.toEqual(at0);
  expect(START[0]).toBeCloseTo(Math.atan2(2.8, 2.5), 9);
});

test("the views snap the camera to a corner, an edge or a face, the wheel zooms and a pan moves the target", () => {
  const live = open({ ...CUBE, camera: "iso" }, { three: true });
  live.at(0);
  live.log.length = 0;
  live.scene.zoom(2);
  expect(live.log).toContainEqual(["three.OrthographicCamera.zoom=", 2]);
  expect(live.names().at(-1)).toBe("three.WebGLRenderer.render");
  live.log.length = 0;
  live.scene.pan(0.1, 0);
  expect(
    live.log
      .find(([key]) => key === "three.OrthographicCamera.lookAt")
      .slice(1)
      .some((v) => Math.abs(v) > 0.01),
  ).toBe(true);
  live.log.length = 0;
  live.scene.snap("face");
  expect(live.log).toContainEqual(["three.OrthographicCamera.zoom=", 1]);
  expect(live.log).toContainEqual(["three.OrthographicCamera.lookAt", 0, 0, 0]);
  const [x, y, z] = live.log.find(([key]) => key === "three.OrthographicCamera.position.set").slice(1);
  expect([x, Math.abs(y) < 1e-9, Math.abs(z) < 1e-9]).toEqual([8, true, true]);
  expect(Object.keys(VIEWS)).toEqual(["corner", "edge", "face"]);
});

test("the camera pick renders through the perspective eye or the isometric box", () => {
  const lens = (camera) => {
    const live = open({ ...CUBE, camera }, { three: true });
    return (
      live.at(0).filter(([key]) => key === "three.WebGLRenderer.render").length &&
      live.log.some(([key]) =>
        key.startsWith(camera === "iso" ? "three.OrthographicCamera.position" : "three.PerspectiveCamera.position"),
      )
    );
  };
  expect([lens("eye"), lens("iso")]).toEqual([true, true]);
});

test("the geometry of a cube coarsens as its instances multiply, so the triangles stay near a phone budget", () => {
  expect([detail(329, 600).ball, detail(5000, 15000).ball, detail(19675, 56816).ball]).toEqual([
    [12, 8],
    [6, 4],
    [4, 3],
  ]);
  const live = open(CUBE, { three: true });
  expect(live.log).toContainEqual(["new three.SphereGeometry", 1, 12, 8]);
  expect(live.log).toContainEqual(["new three.CylinderGeometry", 1, 1, 1, 8, 1, false]);
});

test("a cube without three in hand draws nothing until it lands and keeps its facts", () => {
  const live = open({ dim: 3, base: 2, view: "cube" });
  live.at(0);
  expect(live.log).toEqual([]);
  expect(live.scene.facts().nodes).toBe(400);
  expect(live.scene.svg()).toBe("");
  live.scene.stop();
});

test("stop disposes the meshes, the material and the renderer", () => {
  const live = open(CUBE, { three: true });
  live.log.length = 0;
  live.scene.stop();
  expect(live.names()).toEqual([
    "three.Scene.remove",
    "three.SphereGeometry.dispose",
    "three.CylinderGeometry.dispose",
    "three.InstancedMesh.dispose",
    "three.InstancedMesh.dispose",
    "three.MeshLambertMaterial.dispose",
    "three.WebGLRenderer.dispose",
    "three.WebGLRenderer.forceContextLoss",
  ]);
});
