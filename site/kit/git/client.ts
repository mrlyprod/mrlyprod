import { decode, draw, type Code, type Paint, type Tools, type View, type Wood } from "./view.ts";

/* TYPES */

export type Options = {
  tree: string;
  mount: string;
  md?: Tools["md"];
  paint?: () => Promise<Paint>;
  after?: (view: View) => void;
};

/* STATE */

let turn = 0;

let shown: string | null = null;

/* LOAD */

async function load(url: string): Promise<Uint8Array | null> {
  try {
    const reply = await fetch(url);
    return reply.ok ? new Uint8Array(await reply.arrayBuffer()) : null;
  } catch {
    return null;
  }
}

async function wood(url: string): Promise<Wood | null> {
  try {
    const reply = await fetch(url);
    return reply.ok ? ((await reply.json()) as Wood) : null;
  } catch {
    return null;
  }
}

/* PAGE */

const tail = () => (document.title.includes(" · ") ? document.title.slice(document.title.lastIndexOf(" · ")) : "");

function settle() {
  const id = decode(location.hash.slice(1));
  const spot = id ? document.getElementById(id) : null;
  const kept = (history.state as { y?: number } | null)?.y;
  if (spot) spot.scrollIntoView();
  else if (kept !== undefined) scrollTo(0, kept);
}

function mark(base: string, path: string) {
  const here = `${base}${path}`;
  for (const a of document.querySelectorAll<HTMLAnchorElement>(".tree a[aria-current]")) a.removeAttribute("aria-current");
  const parts = path.split("/").filter(Boolean);
  let at = "";
  let tries = 0;
  const step = () => {
    while (parts.length > 1 || (parts.length === 1 && path.endsWith("/"))) {
      const next = at ? `${at}/${parts[0]}` : parts[0]!;
      const fold = [...document.querySelectorAll<HTMLDetailsElement>(".tree details[data-lazy]")].find((one) => one.dataset.lazy === next);
      if (!fold) {
        if (tries++ < 40) setTimeout(step, 50);
        return;
      }
      fold.open = true;
      at = next;
      parts.shift();
    }
    const hit = [...document.querySelectorAll<HTMLAnchorElement>(".tree a[href]")].find((a) => decode(a.pathname) === here);
    const pane = hit?.closest<HTMLElement>(".pane");
    if (hit && pane) {
      hit.setAttribute("aria-current", "page");
      const top = hit.getBoundingClientRect().top - pane.getBoundingClientRect().top;
      if (top < 0 || top > pane.clientHeight) pane.scrollTop += top - pane.clientHeight / 2;
    } else if (hit) hit.setAttribute("aria-current", "page");
    else if (tries++ < 40) setTimeout(step, 50);
  };
  step();
}

async function tint(mount: Element, code: Code, options: Options, mine: number) {
  if (!options.paint) return;
  const paint = await options.paint().catch(() => null);
  const lines = paint ? await paint(code.text, code.lang) : null;
  if (!lines || mine !== turn) return;
  mount.querySelectorAll(".code .line .t").forEach((slot, i) => {
    if (lines[i] !== undefined) slot.innerHTML = lines[i]!;
  });
}

async function show(data: Wood, options: Options, brand: string) {
  const mine = ++turn;
  const mount = document.querySelector(options.mount);
  if (!mount) return;
  const at = decode(location.pathname).slice(data.base.length);
  const view = draw(data, at);
  shown = location.pathname;
  mount.setAttribute("aria-busy", "true");
  mount.innerHTML = view.html;
  document.title = `${view.name}${brand}`;
  document.querySelector('link[rel="canonical"]')?.setAttribute("href", location.origin + location.pathname);
  mark(data.base, view.found ? view.path : at);
  options.after?.(view);
  const full = view.more ? await view.more({ load, md: options.md }).catch(() => null) : null;
  if (mine !== turn) return;
  if (full) {
    mount.innerHTML = full.html;
    options.after?.(view);
  }
  settle();
  if (full?.code) await tint(mount, full.code, options, mine);
  if (mine === turn) mount.removeAttribute("aria-busy");
}

/* START */

export async function start(options: Options) {
  const brand = tail();
  const mount = document.querySelector(options.mount);
  mount?.setAttribute("aria-busy", "true");
  const data = await wood(options.tree);
  if (!data) {
    if (mount) mount.innerHTML = '<p class="lead">The code tree did not load. Reload to try again.</p>';
    mount?.removeAttribute("aria-busy");
    return;
  }
  const go = () => show(data, options, brand);
  document.addEventListener("click", (event) => {
    if (event.defaultPrevented || event.button !== 0 || event.metaKey || event.ctrlKey || event.shiftKey || event.altKey) return;
    const a = event.target instanceof Element ? event.target.closest<HTMLAnchorElement>("a[href]") : null;
    if (!a || a.target || a.hasAttribute("download") || a.origin !== location.origin || !a.pathname.startsWith(data.base)) return;
    if (a.pathname === location.pathname && a.hash) return;
    event.preventDefault();
    if (a.pathname === location.pathname) return scrollTo(0, 0);
    history.replaceState({ y: scrollY }, "");
    history.pushState(null, "", a.href);
    scrollTo(0, 0);
    go();
  });
  addEventListener("popstate", () => {
    if (location.pathname !== shown) go();
  });
  await go();
}
