import { tidy } from "../../lib/app/spec.js";
import { HUES, mix, rgb } from "../../lib/color.js";
import { canon } from "../../lib/designs/engine.js";
import { pick } from "../../lib/rand.js";
import { tints } from "../../lib/theme/hues.js";
import { palette } from "../../lib/theme/palette.gen.js";
import { wasm } from "../../lib/units/index.js";
import {
  bounds,
  CAMERAS,
  COOL,
  cube,
  DEEPEST,
  DIMS,
  deal,
  design,
  graphs,
  kindOf,
  LAYOUTS,
  lay,
  NUMBERS,
  PAD,
  pace,
  picture,
  quick,
  solid,
  spaceOf,
  study,
  top,
  VIEWS,
} from "./engine.js";
import { space } from "./space.js";

const NEVER = () => false;
const BASES = [
  [2, "2"],
  [3, "3"],
];

const most = (value, units) => (units?.math ? top(units.math, value) : DEEPEST);

export const SPEC = [
  { key: "dim", label: "Dim", kind: "segment", def: 2, options: DIMS, when: NEVER },
  {
    key: "view",
    label: "View",
    kind: "segment",
    def: "cube",
    options: VIEWS,
    group: "Design",
    when: (value) => Number(value.dim) === 3,
  },
  {
    key: "number",
    label: "Number",
    kind: "pick",
    def: 3,
    options: NUMBERS.map((n) => [n, String(n)]),
    group: "Design",
  },
  { key: "graph", label: "Graph", kind: "pick", def: "core", options: graphs, when: NEVER },
  {
    key: "level",
    label: "Level",
    kind: "slider",
    def: 2,
    min: 1,
    max: most,
    unit: (value, units) => `of ${most(value, units)}`,
    step: 1,
    group: "Design",
  },
  { key: "base", label: "Base", kind: "segment", def: 3, options: BASES, when: NEVER },
  { key: "code", label: "Code", kind: "text", def: "", when: NEVER },
  { key: "layout", label: "Layout", kind: "segment", def: "lattice", options: LAYOUTS, group: "Layout" },
  { key: "dots", label: "Dots", kind: "slider", def: 3, min: 1, max: 12, step: 1, group: "Layout" },
  { key: "camera", label: "Camera", kind: "segment", def: "eye", options: CAMERAS, group: "Camera", when: solid },
  { key: "spin", label: "Spin", kind: "toggle", def: 0, group: "Camera", when: solid },
  { key: "tint", label: "Tint", kind: "pick", def: "", options: tints("Roles"), group: "Look" },
];

export const name = (value, loaded) => {
  const { dim, base, code } = design(value);
  return `graphs-${loaded?.math ? canon(loaded.math, dim, base, code) : code || "carpet"}-${spaceOf(value)}-${kindOf(value)}`;
};

export const units = wasm("math");

export const PAGE = {
  spec: SPEC,
  units,
  design: { dim: "dim", base: "base", code: "code" },
  gestures: true,
  record: true,
  name,
  about: [
    "A design is dots. Join every filled cell to its neighbours and you have a network: tips, junctions and pieces to count, a length to add up, a box dimension to read. Tips are yellow, paths blue, junctions pink and lone nodes dim; a tint turns them into fades of one ink.",
    "The core graph joins neighbouring filled cells at their centres, the edge graph is the corners and sides those cells outline, and the tunnel graph joins the empty cells. 2D is a flat design, 3D a cube to turn, 6D its hex slice, the hexagon a cube leaves when cut across its diagonal. There the core joins filled triangles across shared sides, the dual takes fills and voids together, and the edge graph is their sides.",
    "Every node, branch, role and count comes from @mrlyprod/mrly, and the Euler number is the design's own, read off its cells. The level stops where the closed-form size says a build would stall.",
    "The force layout is Fruchterman and Reingold's: every pair repels, every branch pulls, and a cooling cap on each move settles the lattice, while the seed jitters the start so a symmetric one can fold. Energy is the mean net force per node; heat is the cap on the next move.",
  ],
  keys: [
    {
      key: "Enter",
      label: (_value, handle) => (handle?.playing?.() ? "Pause" : "Relax"),
      act: "relax",
      button: true,
      menu: "Edit",
    },
    { key: ["ArrowUp", "ArrowDown"], label: "Level", act: "level", menu: "View" },
    { key: ["ArrowLeft", "ArrowRight"], label: "Graph", act: "graph", menu: "View" },
  ],
  actions: {
    relax: (scene) => scene.toggle?.(),
    level: (scene, e) => scene.deeper?.(e.key === "ArrowDown" ? -1 : 1),
    graph: (scene, e) => scene.next?.(e.key === "ArrowLeft" ? -1 : 1),
  },
};

/* SAVERS */

const nets = (kinds) => (rand) => {
  const number = pick(rand, [3, 3, 5]);
  return {
    graph: pick(rand, kinds),
    number,
    level: number === 3 ? 3 : 2,
    base: pick(rand, [3, 3, 2]),
    dots: pick(rand, [3, 4, 5]),
  };
};

export const SAVERS = [
  { id: "webs", title: "Cell webs", pick: nets(["core", "core", "edge"]) },
  { id: "tunnels", title: "Tunnels", pick: nets(["tunnel"]) },
];

const TAU = Math.PI * 2;
export const HOLD = 8000;
const ROOM = 600;
const SLOW = 24;
const ROD = 0.35;
const INKS = ["dim", "yellow", "blue", "pink"];
const FADES = [0.6, 0.4, 0.2, 0];

let clock = 0;

const ink = (canvas, name) =>
  (typeof getComputedStyle === "function" && canvas?.nodeType
    ? getComputedStyle(canvas).getPropertyValue(`--${name}`).trim()
    : "") ||
  palette[name] ||
  palette.gray;

export function tones(canvas, view, tint) {
  if (!HUES.includes(tint) && !view.lock) return INKS.map((name) => ink(canvas, name));
  const { accent, paper } = view.look();
  return FADES.map((t) => (t ? mix(rgb(accent), rgb(paper), t) : accent));
}

/* VALUE */

const built = (value) => JSON.stringify([spaceOf(value), design(value), value.number, value.level, kindOf(value)]);

export function shape(math, opts, lock = false) {
  const ask = { code: !("code" in opts), graph: !("graph" in opts), level: !("level" in opts), layout: false };
  const seed = opts.seed >>> 0;
  const first = { ...tidy(SPEC, { ...opts, layout: opts.layout ?? (lock ? "force" : undefined) }, { math }), seed };
  if (!lock || !(ask.code || ask.graph || ask.level)) return first;
  return { ...tidy(SPEC, { ...first, ...deal(math, first, opts.rand, ask, ROOM) }, { math }), seed };
}

/* RELAX */

export function relax(math, plan, { seed, layout }, view) {
  const per = view.lock ? Math.max(pace(plan.count, plan.dim), SLOW) : pace(plan.count, plan.dim);
  let bed = null;
  let on = false;
  let cooled = false;
  let owed = 0;
  let last = null;
  let heat = Infinity;
  let rest = null;
  let jitter = seed >>> 0;
  let positions = plan.positions;
  let version = ++clock;
  const read = () => {
    positions = bed.positions();
    version = ++clock;
  };
  const begin = () => {
    bed?.free();
    bed = math.graph.Layout.from_network(plan.net, jitter);
    heat = bed.temperature();
    cooled = false;
    owed = 0;
    last = null;
    rest = null;
    read();
  };
  const step = (n) => {
    bed.step(n);
    const was = heat;
    heat = bed.temperature();
    read();
    if (heat < was) return;
    cooled = true;
    on = false;
  };
  if (layout === "force") {
    begin();
    if (!view.still) on = true;
    else if (quick(plan.count, plan.dim)) {
      step(COOL);
      cooled = true;
    }
  }
  const at = (t) => {
    if (!bed) return;
    if (on) {
      if (last !== null) owed += Math.max(0, t - last) / per;
      last = t;
      const n = Math.floor(owed);
      if (n < 1) return;
      owed -= n;
      step(n);
    } else if (view.lock && cooled) {
      rest ??= t;
      if (t - rest < HOLD) return;
      jitter = Math.floor(view.rand() * 4294967296);
      begin();
      on = true;
      last = t;
    }
  };
  const go = () => {
    on = true;
    last = null;
    view.wake?.();
  };
  const toggle = () => {
    if (!bed) return false;
    if (on) on = false;
    else {
      if (cooled) begin();
      go();
    }
    return on;
  };
  const reset = () => {
    if (!bed) return;
    begin();
    go();
  };
  const state = () =>
    bed ? { on, cooled, ticks: bed.ticks(), energy: bed.energy(), moved: bed.moved(), heat: bed.temperature() } : null;
  const free = () => {
    bed?.free();
    bed = null;
  };
  return { at, now: () => ({ positions, version }), toggle, reset, state, free };
}

/* STAGES */

function flat(canvas, view, mover) {
  const ctx = canvas.getContext("2d");
  let plan = null;
  let shades = [];
  let look = { dots: 3, tint: "" };
  let drawn = -1;
  let dirty = true;
  const paint = () => {
    shades = tones(canvas, view, look.tint);
  };
  const fit = (positions, w, h, dpr) => {
    const dot = look.dots * dpr;
    return {
      ...lay(bounds(positions, 2, plan.count), w, h, PAD * dpr + dot),
      dot,
      line: Math.max(1, look.dots / 3) * dpr,
    };
  };
  const ends = (positions, box) => {
    const X = (i) => box.ox + positions[2 * i] * box.k;
    const Y = (i) => box.oy + positions[2 * i + 1] * box.k;
    return { X, Y };
  };
  const draw = () => {
    if (!plan) return;
    const { positions, version } = mover().now();
    if (!dirty && version === drawn) return;
    const box = fit(positions, view.w, view.h, view.dpr);
    const { X, Y } = ends(positions, box);
    const { pairs, shade, count } = plan;
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, view.w, view.h);
    ctx.lineCap = "round";
    ctx.lineWidth = box.line;
    shades.forEach((color, s) => {
      ctx.strokeStyle = color;
      ctx.beginPath();
      let open = false;
      for (let j = 0; j < shade.branches.length; j++) {
        if (shade.branches[j] !== s) continue;
        ctx.moveTo(X(pairs[2 * j]), Y(pairs[2 * j]));
        ctx.lineTo(X(pairs[2 * j + 1]), Y(pairs[2 * j + 1]));
        open = true;
      }
      if (open) ctx.stroke();
    });
    shades.forEach((color, s) => {
      ctx.fillStyle = color;
      ctx.beginPath();
      let open = false;
      for (let i = 0; i < count; i++) {
        if (shade.nodes[i] !== s) continue;
        ctx.moveTo(X(i) + box.dot, Y(i));
        ctx.arc(X(i), Y(i), box.dot, 0, TAU);
        open = true;
      }
      if (open) ctx.fill();
    });
    drawn = version;
    dirty = false;
  };
  const load = (next) => {
    plan = next;
    dirty = true;
  };
  const restyle = (value) => {
    look = { dots: value.dots, tint: value.tint };
    paint();
    dirty = true;
  };
  const size = () => {
    dirty = true;
  };
  const theme = () => {
    paint();
    dirty = true;
  };
  const svg = () => {
    const { positions } = mover().now();
    const w = view.w / view.dpr;
    const h = view.h / view.dpr;
    const box = fit(positions, w, h, 1);
    const { X, Y } = ends(positions, box);
    const { pairs, shade, count } = plan;
    const marks = Array.from({ length: count }, (_, i) => [X(i), Y(i), shade.nodes[i]]);
    const lines = Array.from(shade.branches, (s, j) => [
      X(pairs[2 * j]),
      Y(pairs[2 * j]),
      X(pairs[2 * j + 1]),
      Y(pairs[2 * j + 1]),
      s,
    ]);
    return picture(w, h, marks, lines, box.dot, box.line, shades);
  };
  return { draw, load, restyle, size, theme, svg };
}

function solid3(canvas, view, mover, three) {
  let stage = null;
  let gone = false;
  let plan = null;
  let shades = [];
  let look = { dots: 3, tint: "", camera: "eye", spin: 0 };
  let at = new Float64Array(0);
  let placed = -1;
  let dirty = true;
  const paint = () => {
    shades = tones(canvas, view, look.tint);
  };
  const ball = () => (look.dots * view.dpr) / stage.unit(view.h);
  const fit = (positions) => {
    const { mid, scale } = cube(bounds(positions, 3, plan.count));
    for (let i = 0; i < plan.count * 3; i++) at[i] = (positions[i] - mid[i % 3]) * scale;
  };
  const draw = () => {
    if (!stage || !plan) return;
    const { positions, version } = mover().now();
    if (placed !== version) {
      fit(positions);
      stage.place(at, ball(), ball() * ROD);
      placed = version;
      dirty = true;
    }
    stage.draw(dirty);
    dirty = false;
  };
  const dress = () => {
    stage.lens(look.camera);
    stage.spinning(look.spin);
    stage.tone(shades);
  };
  const load = (next) => {
    plan = next;
    at = new Float64Array(plan.count * 3);
    placed = -1;
    dirty = true;
    if (!stage) return;
    stage.load(plan);
    stage.tone(shades);
  };
  const restyle = (value) => {
    if (value.dots !== look.dots) placed = -1;
    const was = shades;
    look = { dots: value.dots, tint: value.tint, camera: value.camera, spin: value.spin };
    paint();
    dirty = true;
    if (!stage) return;
    stage.lens(look.camera);
    stage.spinning(look.spin);
    if (String(was) !== String(shades)) stage.tone(shades);
  };
  const build = (mod, late) => {
    if (gone) return;
    stage = space(mod, canvas, view);
    if (plan) stage.load(plan);
    dress();
    placed = -1;
    dirty = true;
    if (!late) return;
    draw();
    view.drew?.();
  };
  const size = () => {
    stage?.size();
    placed = -1;
    dirty = true;
  };
  const theme = () => {
    paint();
    stage?.tone(shades);
    dirty = true;
  };
  const svg = () => {
    if (!stage) return "";
    const { positions } = mover().now();
    const w = view.w / view.dpr;
    const h = view.h / view.dpr;
    fit(positions);
    const px = stage.unit(h);
    const r = (look.dots * view.dpr) / stage.unit(view.h);
    const seen = stage.project(at, w, h);
    const { pairs, shade, count } = plan;
    const marks = Array.from({ length: count }, (_, i) => [
      seen[3 * i],
      seen[3 * i + 1],
      shade.nodes[i],
      seen[3 * i + 2],
    ]).sort((p, q) => q[3] - p[3]);
    const lines = Array.from(shade.branches, (s, j) => {
      const a = pairs[2 * j] * 3;
      const c = pairs[2 * j + 1] * 3;
      return [seen[a], seen[a + 1], seen[c], seen[c + 1], s, (seen[a + 2] + seen[c + 2]) / 2];
    }).sort((p, q) => q[5] - p[5]);
    return picture(w, h, marks, lines, r * px, 2 * r * ROD * px, shades);
  };
  const stop = () => {
    gone = true;
    stage?.stop();
    stage = null;
  };
  const hand =
    (name) =>
    (...args) =>
      stage?.[name](...args);
  if (three) build(three, false);
  else
    import("three").then(
      (mod) => build(mod, true),
      () => {},
    );
  return {
    draw,
    load,
    restyle,
    size,
    theme,
    svg,
    stop,
    turn: hand("turn"),
    grab: hand("grab"),
    release: hand("release"),
    zoom: hand("zoom"),
    pan: hand("pan"),
    snap: hand("snap"),
  };
}

/* SCENE */

export function make(canvas, view, opts = {}) {
  const { math, three } = opts;
  if (!math) throw new Error("graphs: the scene wants @mrlyprod/mrly/math as opts.math");
  let value = shape(math, { ...opts, rand: view.rand }, view.lock);
  let plan = study(math, value);
  let mover = relax(math, plan, value, view);
  const stage = solid(value) ? solid3(canvas, view, () => mover, three) : flat(canvas, view, () => mover);
  stage.restyle(value);
  stage.load(plan);
  const draw = () => {
    mover.at(view.t);
    stage.draw();
  };
  const set = (next) => {
    if (solid(next) !== solid(value)) return false;
    const was = value;
    value = shape(math, { ...next, rand: view.rand }, view.lock);
    if (built(was) !== built(value)) {
      mover.free();
      plan.net.free();
      plan = study(math, value);
      stage.load(plan);
      mover = relax(math, plan, value, view);
    } else if (was.layout !== value.layout || was.seed !== value.seed) {
      mover.free();
      mover = relax(math, plan, value, view);
      plan.facts = { ...plan.facts, layout: value.layout, top: top(math, { ...value, code: plan.facts.code }) };
    }
    stage.restyle(value);
  };
  const stop = () => {
    stage.stop?.();
    mover.free();
    plan.net.free();
  };
  const json = () => JSON.stringify({ ...plan.facts, network: plan.net.toJSON() });
  const { turn, grab, release, zoom, pan, snap } = stage;
  return {
    draw,
    set,
    stop,
    json,
    size: stage.size,
    theme: stage.theme,
    svg: stage.svg,
    facts: () => plan.facts,
    relaxed: () => mover.state(),
    toggle: () => mover.toggle(),
    playing: () => Boolean(mover.state()?.on),
    reset: () => mover.reset(),
    turn,
    grab,
    release,
    zoom,
    pan,
    snap,
  };
}
