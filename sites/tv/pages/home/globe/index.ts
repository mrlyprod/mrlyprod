import type { Body } from "./body.ts";
import { drag, type Spin, settle } from "./drag.ts";
import { picker } from "./picker.ts";
import { glow, type Stream, stream } from "./pretend.ts";
import { fog, paint } from "./shade.ts";
import { SOURCES, type Source } from "./sources.ts";
import { stage } from "./stage.ts";
import { readTheme, type Theme, watchTheme } from "./theme.ts";
import { trails } from "./trails.ts";

const BOB = 0.025;
const HEAVE = 0.6;

export function mount(el: HTMLElement): () => void {
  if (getComputedStyle(el).position === "static") el.style.position = "relative";
  let show: ReturnType<typeof stage>;
  try {
    show = stage(el);
  } catch {
    return () => {};
  }
  const canvas = show.canvas;
  Object.assign(canvas.style, { position: "absolute", inset: "0", width: "100%", height: "100%", display: "block" });
  canvas.setAttribute("role", "img");
  el.prepend(canvas);
  let theme: Theme = readTheme(el);
  const paths = trails();
  paths.paint(theme);
  show.root.add(paths.object);
  const spin: Spin = { yaw: 0, pitch: 0, vy: 0, held: false };
  const released = drag(canvas, spin, show.invalidate);
  let source: Source = SOURCES.find((s) => `#${s.id}` === location.hash) ?? SOURCES[0];
  let body: Body | null = null;
  let feed: Stream | null = null;
  let distance = 3;
  const ui = picker(el, SOURCES, source.id, choose);

  function choose(id: string) {
    source = SOURCES.find((s) => s.id === id) ?? SOURCES[0];
    if (body) {
      show.spin.remove(body.object);
      body.dispose();
    }
    body = source.build(theme);
    fog(body.material, distance);
    feed = stream(source.seed, source.rate, source.route(body), source.longest, source.fade);
    show.spin.add(body.object);
    show.root.rotation.z = source.lean;
    spin.pitch = source.pitch;
    spin.vy = source.auto;
    paths.style(source.width, source.xray);
    ui.label(source.label);
    canvas.setAttribute("aria-label", `${source.name}: ${source.label.replace(/<[^>]+>/g, "")}, live activity`);
    show.invalidate();
  }

  show.frame = (dt, t) => {
    if (!body || !feed) return;
    settle(spin, dt, show.still ? 0 : source.auto);
    show.spin.rotation.y = spin.yaw;
    show.root.rotation.x = spin.pitch;
    show.root.position.y = show.still ? 0 : BOB * Math.sin(t * HEAVE);
    const trips = feed.at(t);
    glow(trips, t, body.lit);
    body.flush();
    paths.update(trips, t, body, feed.fade);
  };
  show.sized = (w, h, dpr, far) => {
    distance = far;
    paths.resize(w, h, dpr);
    if (body) fog(body.material, distance);
  };
  const unwatched = watchTheme(el, (next) => {
    theme = next;
    paths.paint(theme);
    if (body) paint(body.material, theme);
    show.invalidate();
  });
  choose(source.id);

  return () => {
    unwatched();
    released();
    ui.dispose();
    if (body) body.dispose();
    paths.dispose();
    show.dispose();
    canvas.remove();
  };
}
