import type { Site } from "site:routes";
import type { CSSProperties } from "react";
import { useCallback, useEffect, useReducer, useRef, useState, useSyncExternalStore } from "react";
import type { Saver as Listed, Row } from "../../../scripts/build.ts";
import { Icon } from "../../lib/app/controls.tsx";
import { AppIcon } from "../../shell/appicon.tsx";
import { desk } from "../../shell/desk.ts";
import { start as preview } from "../../shell/idle.ts";
import type { Lock } from "../../shell/lock.ts";
import { lock } from "../../shell/lock.ts";
import { hide, Lede, show } from "../../shell/parts.tsx";
import { calm, KEY, offs, read, savers as save, screen, subscribe } from "../../shell/prefs.ts";
import { named } from "../../shell/route.ts";
import type { Context, Layout } from "../../shell/router.ts";
import { state } from "../../shell/router.ts";
import { all, find } from "../../shell/savers.ts";
import { shooter, shots } from "./shots.ts";

/* TYPES */

export type Spot = { kind: "all" } | { kind: "one"; entry: Listed } | { kind: "lost"; id: string };

export type Base = { row: Row; rows: Row[]; site: Site; from: string };

export type PageProps = Base & { live: boolean };

export type LiveProps = { id: string };

export type CardProps = {
  one: Listed;
  off: boolean;
  hot: boolean;
  point: (id: string, on: boolean) => void;
  icon: boolean;
};

export type GalleryProps = { row: Row; rows: Row[]; list: Listed[]; off: string[]; live: boolean };

export type StageProps = { pick: string; label: string; live: boolean; roll: (as: string) => void; fail: () => void };

export type SaverProps = {
  row: Row;
  rows: Row[];
  site: Site;
  entry: Listed;
  list: Listed[];
  off: string[];
  want: string;
  live: boolean;
  go: (to: string) => void;
};

export type MissingProps = { id: string };

const ROOT = "/screensavers/";
const WAIT = 250;
const SHOWN = 1500;
const POSTER: CSSProperties = {
  position: "absolute",
  top: "50%",
  left: "50%",
  width: "36%",
  aspectRatio: "1",
  translate: "-50% -50%",
};

/* PATHS */

const link = (id: string) => `${ROOT}${id}/`;

const routed = (rows: Row[], app: string) => rows.some((row) => row.route === `/${app}/`);

function spot(path: string): Spot {
  const rest = path.slice(ROOT.length).replace(/\/$/, "");
  if (!rest) return { kind: "all" };
  const entry = find(rest);
  return entry?.id === rest ? { kind: "one", entry } : { kind: "lost", id: rest };
}

const number = (text: string) => (text.trim() !== "" && Number.isFinite(Number(text)) ? Number(text) : text);

const usePref = (key: string) => useSyncExternalStore(subscribe, () => read(key));

const away = () => document.documentElement.hasAttribute("data-saver");

/* GALLERY */

function Live({ id }: LiveProps) {
  const at = useRef<HTMLCanvasElement>(null);
  const [on, setOn] = useState(false);
  useEffect(() => {
    let stop: Lock | null = null;
    let gone = false;
    let frame = 0;
    const timer = setTimeout(async () => {
      try {
        const made = await lock(at.current!, id);
        if (gone) return made();
        stop = made;
        frame = requestAnimationFrame(() => (frame = requestAnimationFrame(() => setOn(true))));
      } catch (error) {
        console.error(error);
      }
    }, WAIT);
    return () => {
      gone = true;
      clearTimeout(timer);
      cancelAnimationFrame(frame);
      stop?.();
    };
  }, [id]);
  return <canvas ref={at} className={on ? "on" : undefined} aria-hidden="true" />;
}

function Card({ one, off, hot, point, icon }: CardProps) {
  const shot = shots.get(one.id);
  const enter = () => point(one.id, true);
  const leave = () => point(one.id, false);
  return (
    <li>
      <a
        className="card"
        href={link(one.id)}
        data-id={one.id}
        onPointerEnter={(e) => e.pointerType !== "touch" && enter()}
        onPointerLeave={leave}
        onFocus={enter}
        onBlur={leave}
      >
        <span className="thumb">
          {shot?.url ? (
            <img src={shot.url} alt="" />
          ) : (
            <span style={POSTER}>
              <AppIcon id={one.app} />
            </span>
          )}
          {hot && <Live id={one.id} />}
        </span>
        <span className="who">
          {icon && <AppIcon id={one.app} />}
          <strong>{one.title === one.name ? one.title : `${one.name} · ${one.title}`}</strong>
        </span>
        <span className="name">{one.id}</span>
        {off && <span className="badge">Off</span>}
      </a>
    </li>
  );
}

function Gallery({ row, rows, list, off, live }: GalleryProps) {
  const [hot, setHot] = useState<string | null>(null);
  const [, bump] = useReducer((n: number) => n + 1, 0);
  const grid = useRef<HTMLDivElement>(null);
  useEffect(() => {
    if (!live) return;
    const worker = shooter(
      list.map((one) => one.id),
      bump,
    );
    for (const node of grid.current!.querySelectorAll<HTMLElement>(".card[data-id]")) worker.watch(node);
    return worker.stop;
  }, [live]);
  const point = (id: string, on: boolean) => live && !calm() && setHot((now) => (on ? id : now === id ? null : now));
  return (
    <div ref={grid}>
      <Lede id="screensavers" title={row.title} lead={row.lead} />
      {!list.length && <p className="fine">No app declares a screen saver yet.</p>}
      <ul className="cards">
        {list.map((one) => (
          <Card
            key={one.id}
            one={one}
            off={off.includes(one.id)}
            hot={hot === one.id}
            point={point}
            icon={routed(rows, one.app)}
          />
        ))}
      </ul>
    </div>
  );
}

/* SAVER */

function Stage({ pick, label, live, roll, fail }: StageProps) {
  const at = useRef<HTMLCanvasElement>(null);
  useEffect(() => {
    if (!live) return;
    let stop: Lock | null = null;
    let gone = false;
    const rest = () => stop && (away() ? stop.pause?.() : stop.play?.());
    const eye = new MutationObserver(rest);
    eye.observe(document.documentElement, { attributes: true, attributeFilter: ["data-saver"] });
    lock(at.current!, pick).then(
      (made) => {
        if (gone) return made();
        stop = made;
        roll(made.as);
        if (away()) made.pause?.();
      },
      (error) => {
        console.error(error);
        if (!gone) fail();
      },
    );
    return () => {
      gone = true;
      eye.disconnect();
      stop?.();
    };
  }, [live]);
  return <canvas ref={at} className="stage" role="img" aria-label={label} />;
}

function Saver({ row, rows, site, entry, list, off, want, live, go }: SaverProps) {
  const [turn, setTurn] = useState(0);
  const [as, setAs] = useState("");
  const [failed, setFailed] = useState(false);
  const [copied, setCopied] = useState(false);
  const at = list.findIndex((one) => one.id === entry.id);
  const prev = list[(at - 1 + list.length) % list.length];
  const next = list[(at + 1) % list.length];
  const many = list.length > 1;
  const query = as.includes("?") ? as.slice(as.indexOf("?") + 1) : "";
  const value = as
    ? Object.fromEntries(
        [...new URLSearchParams(query)].map(([key, text]): [string, string | number] => [key, number(text)]),
      )
    : null;
  const current = find(want.split("?")[0] || site.desk)?.id === entry.id;
  const app = routed(rows, entry.app);
  const pick = (!turn && shots.get(entry.id)?.as) || entry.id;
  useEffect(() => {
    if (!live || !many) return;
    const key = (e: KeyboardEvent) => {
      if (e.defaultPrevented || e.metaKey || e.ctrlKey || e.altKey || e.shiftKey || away()) return;
      const to = e.key === "ArrowLeft" ? prev : e.key === "ArrowRight" ? next : null;
      if (
        !to ||
        (e.target instanceof Element && e.target.closest("input, select, textarea, [contenteditable]")) ||
        document.querySelector("dialog[open], :popover-open")
      )
        return;
      e.preventDefault();
      go(link(to.id));
    };
    document.addEventListener("keydown", key);
    return () => document.removeEventListener("keydown", key);
  }, [live, many, prev, next]);
  useEffect(() => {
    if (!copied) return;
    const timer = setTimeout(() => setCopied(false), SHOWN);
    return () => clearTimeout(timer);
  }, [copied]);
  const rolled = (made: string) => {
    setAs(made);
    setCopied(false);
  };
  const reroll = () => {
    setAs("");
    setFailed(false);
    setTurn((n) => n + 1);
  };
  const copy = () =>
    navigator.clipboard?.writeText(as).then(
      () => setCopied(true),
      () => {},
    );
  const use = () => {
    screen(entry.id);
    desk();
  };
  const kept = !off.includes(entry.id);
  const flip = (on: boolean) => save(on ? off.filter((id) => id !== entry.id) : [...off, entry.id]);
  return (
    <div className="one">
      <p className="back">
        <a href={ROOT}>
          <Icon name="chevron_right" />
          {row.title}
        </a>
      </p>
      <div className="lede">
        <h1 id="saver">{entry.title}</h1>
        <p className="lead">
          {app ? <a href={`/${entry.app}/`}>{entry.name}</a> : entry.name} <code>{entry.id}</code>
        </p>
      </div>
      <Stage key={turn} pick={pick} label={entry.title} live={live} roll={rolled} fail={() => setFailed(true)} />
      {failed && <p className="fine">This screen saver did not load.</p>}
      <div className="steps">
        {many && (
          <a className="button prev" href={link(prev.id)} title={prev.title} aria-keyshortcuts="ArrowLeft">
            <Icon name="chevron_right" />
            Previous
          </a>
        )}
        <button type="button" onClick={reroll}>
          <Icon name="casino" />
          Reroll
        </button>
        {many && (
          <a className="button next" href={link(next.id)} title={next.title} aria-keyshortcuts="ArrowRight">
            Next
            <Icon name="chevron_right" />
          </a>
        )}
      </div>
      <section className="value">
        <h2>Value</h2>
        <div className="as">
          <code>{as || entry.id}</code>
          <button type="button" onClick={copy} disabled={!as}>
            {copied ? "Copied" : "Copy"}
          </button>
        </div>
        {value && <pre>{JSON.stringify(value, null, 2)}</pre>}
      </section>
      <div className="use">
        {app && (
          <a className="button" href={`/${entry.app}/${query ? `?${query}` : ""}`}>
            Open in {entry.name}
          </a>
        )}
        {current ? (
          <span className="tag">
            <Icon name="check" />
            Desktop picture
          </span>
        ) : (
          <button type="button" onClick={use}>
            Use as desktop picture
          </button>
        )}
        <button type="button" onClick={() => preview({ only: entry.id })}>
          Run full screen
        </button>
      </div>
      <label className="shuffle">
        <span>
          In shuffle
          <small>When off, the screen saver skips it.</small>
        </span>
        <input
          type="checkbox"
          role="switch"
          checked={kept}
          aria-checked={kept}
          onChange={(e) => flip(e.target.checked)}
        />
      </label>
    </div>
  );
}

function Missing({ id }: MissingProps) {
  return (
    <div className="lede">
      <h1 id="lost">No such screen saver</h1>
      <p className="lead">
        No app declares <code>{id}</code>. <a href={ROOT}>See every screen saver</a>.
      </p>
    </div>
  );
}

/* PAGE */

function Screensavers({ row, rows, site, from, live }: PageProps) {
  const [path, setPath] = useState(from);
  usePref(KEY.off);
  const want = usePref(KEY.saver);
  const off = offs();
  const list = all();
  const at = spot(path);
  const go = useCallback((to: string) => {
    if (to !== location.pathname) {
      const was = state();
      history.pushState({ ...was, depth: (was.depth ?? 0) + 1 }, "", to);
    }
    setPath(to);
    document.getElementById("main")?.scrollTo({ top: 0, behavior: "instant" });
  }, []);
  useEffect(() => {
    if (!live) return;
    const click = (e: MouseEvent) => {
      if (e.defaultPrevented || e.button !== 0 || e.metaKey || e.ctrlKey || e.shiftKey || e.altKey) return;
      const a = e.target instanceof Element ? e.target.closest<HTMLAnchorElement>(`a[href^="${ROOT}"]`) : null;
      if (!a || a.target || a.hasAttribute("download")) return;
      e.preventDefault();
      go(a.pathname);
    };
    const pop = () => location.pathname.startsWith(ROOT) && setPath(location.pathname);
    document.addEventListener("click", click);
    addEventListener("popstate", pop);
    return () => {
      document.removeEventListener("click", click);
      removeEventListener("popstate", pop);
    };
  }, [live]);
  useEffect(() => {
    if (!live) return;
    const name = at.kind === "one" ? at.entry.title : at.kind === "lost" ? "Not found" : "";
    document.title = name ? `${name} · ${row.title} · ${site.title}` : named(row, site);
    document.head
      .querySelector('link[rel="canonical"]')
      ?.setAttribute("href", site.root + (at.kind === "one" ? link(at.entry.id) : path));
    if (at.kind !== "lost") return;
    const robots = Object.assign(document.createElement("meta"), { name: "robots", content: "noindex" });
    document.head.append(robots);
    return () => robots.remove();
  }, [live, path]);
  return (
    <div className="screensavers">
      {at.kind === "all" && <Gallery row={row} rows={rows} list={list} off={off} live={live} />}
      {at.kind === "one" && (
        <Saver
          key={at.entry.id}
          row={row}
          rows={rows}
          site={site}
          entry={at.entry}
          list={list}
          off={off}
          want={want}
          live={live}
          go={go}
        />
      )}
      {at.kind === "lost" && <Missing id={at.id} />}
    </div>
  );
}

export const layout: Layout = { wide: true, bare: true, window: "wide" };

const given = new WeakMap<HTMLElement, Base>();

export function render(row: Row, site: Site, host: HTMLElement, { rows, url }: Context) {
  const base = { row, rows, site, from: url.pathname };
  given.set(host, base);
  show(host, <Screensavers {...base} live={false} />);
}

export function mount(host: HTMLElement) {
  const base = given.get(host);
  if (base) show(host, <Screensavers {...base} live />);
}

export function unmount(host: HTMLElement) {
  hide(host);
  given.delete(host);
}
