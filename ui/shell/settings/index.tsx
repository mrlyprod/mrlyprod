import type { Site } from "site:routes";
import type { PaneNode } from "@mrlyprod/os/kernel/index.ts";
import type { ChangeEvent, KeyboardEvent as Pressed } from "react";
import { useEffect, useRef, useState } from "react";
import { flushSync } from "react-dom";
import type { Asked } from "../../kit/controls.tsx";
import { ask, Empty, Icon, Menu, Search, toast } from "../../kit/controls.tsx";
import { text } from "../../kit/export.ts";
import { rank, terms } from "../launch.ts";
import type { Pane } from "../panes.ts";
import { PANES } from "../panes.ts";
import type { Vars } from "../parts.tsx";
import { decode, PREFIX, pack, share, unpack } from "../prefs.ts";
import type { Titled } from "../route.ts";
import { named } from "../route.ts";
import { state } from "../router.ts";
import { Body, labelsOf } from "./pane.tsx";
import { Glyph } from "./ui.tsx";

type How = "push" | "replace";

type Live = { open: (to: string, how?: How) => void; pop: () => void };

type RowProps = { one: Pane; on: boolean; labels: string[] };

type SettingsProps = { row: Titled; site: Site; start: URL; panes: PaneNode[] };

const ROOT = "/settings/";
const FIRST = "appearance";
const SHOWN = 3;

const IDS = new Set(PANES.map((one) => one.id));

function spot(path: string) {
  const id = path.startsWith(ROOT) ? path.slice(ROOT.length).split("/")[0] : "";
  return IDS.has(id) ? `${ROOT}${id}/` : ROOT;
}

const main = () => document.getElementById("main");
const roll = (top: number) => main()?.scrollTo({ top, behavior: "instant" });

function head(row: Titled, site: Site, path: string) {
  const pane = PANES.find((one) => one.href === path);
  const base = named(row, site);
  document.title = pane ? `${pane.title} · ${base}` : base;
  document.querySelector('link[rel="canonical"]')?.setAttribute("href", site.root + path);
}

function refocus() {
  const now = document.activeElement;
  if (!now || now === document.body || !now.isConnected) main()?.focus({ preventScroll: true });
}

const HUES: Partial<Record<string, string>> = {
  appearance: "purple",
  desktop: "indigo",
  screensaver: "pink",
  accessibility: "blue",
  privacy: "green",
  about: "orange",
};

function Square({ art, hue }: Pick<Pane, "art"> & { hue: string }) {
  return (
    <span className="symbol" style={{ "--hue": `var(--${hue})` } as Vars} aria-hidden="true">
      <Glyph art={art} />
    </span>
  );
}

function Row({ one, on, labels }: RowProps) {
  return (
    <li>
      <a href={one.href} aria-current={on ? "page" : undefined}>
        <Square art={one.art} hue={HUES[one.id] ?? ""} />
        <span className="words">
          <span className="name">{one.title}</span>
          {labels.length > 0 && <span className="hint">{labels.join(", ")}</span>}
        </span>
      </a>
    </li>
  );
}

const APPLY: Asked = {
  title: "Apply settings from this link?",
  text: "They replace the settings kept in this browser.",
  actions: [{ label: "Not now" }, { label: "Apply" }],
};

const BROKEN: Asked = { title: "This settings link is broken.", text: "It may have been cut short." };

async function offer(code: string, drop: () => void) {
  const data = decode(code);
  if (data && (await ask(APPLY)) !== "Apply") return drop();
  try {
    unpack(data);
    location.replace(ROOT);
  } catch {
    await ask(BROKEN);
    drop();
  }
}

export function Settings({ row, site, start, panes }: SettingsProps) {
  const [here, setHere] = useState(() => spot(start.pathname) + start.search);
  const [query, setQuery] = useState("");
  const box = useRef<HTMLDivElement>(null);
  const field = useRef<HTMLInputElement>(null);
  const list = useRef<HTMLUListElement>(null);
  const file = useRef<HTMLInputElement>(null);
  const live = useRef<Live | null>(null);

  const cut = here.indexOf("?");
  const path = cut < 0 ? here : here.slice(0, cut);
  const code = cut < 0 ? null : new URLSearchParams(here.slice(cut)).get("apply");
  const id = path.slice(ROOT.length, -1);
  const shown = id || FIRST;
  const pane = panes.find((one) => one.id === shown);
  const labels = (id: string) => labelsOf(panes.find((one) => one.id === id)!).filter(Boolean);
  const items = PANES.filter((one) => panes.some((each) => each.id === one.id)).map((one) => ({
    ...one,
    more: [one.more, ...labels(one.id)].join(" "),
  }));
  const words = terms(query);
  const hits = words.length ? rank(query, items) : items;
  const found = (one: Pane) =>
    words.length
      ? labels(one.id)
          .filter((label) => words.every((word) => label.toLowerCase().includes(word)))
          .slice(0, SHOWN)
      : [];

  const open = (to: string, how: How = "push") => {
    const next = new URL(to, location.href);
    const fresh = spot(next.pathname);
    const href = fresh + next.search;
    if (how === "push") {
      history.replaceState({ ...state(), y: Math.round(main()?.scrollTop ?? 0) }, "");
      const was = state();
      history.pushState({ ...was, y: 0, depth: (was.depth ?? 0) + 1 }, "", href);
    } else history.replaceState(state(), "", href);
    flushSync(() => setHere(href));
    roll(0);
    head(row, site, fresh);
    refocus();
  };

  const pop = () => {
    const fresh = spot(location.pathname);
    flushSync(() => setHere(fresh + location.search));
    roll(state().y ?? 0);
    head(row, site, fresh);
    refocus();
  };

  live.current = { open, pop };

  useEffect(() => {
    const path = spot(location.pathname);
    if (location.pathname !== path) history.replaceState(history.state, "", path + location.search + location.hash);
    head(row, site, path);
    const mine = () => Boolean(box.current?.isConnected);
    const click = (e: MouseEvent) => {
      if (e.defaultPrevented || e.button !== 0 || e.metaKey || e.ctrlKey || e.shiftKey || e.altKey || !mine()) return;
      const a = e.target instanceof Element ? e.target.closest("a[href]") : null;
      if (
        !(a instanceof HTMLAnchorElement) ||
        a.target ||
        a.hasAttribute("download") ||
        a.origin !== location.origin ||
        !a.pathname.startsWith(ROOT)
      )
        return;
      e.preventDefault();
      live.current!.open(a.pathname + a.search, a.href === location.href ? "replace" : "push");
    };
    const pop = () => {
      if (mine() && location.pathname.startsWith(ROOT)) live.current!.pop();
    };
    const key = (e: KeyboardEvent) => {
      if (e.defaultPrevented || e.altKey || typeof e.key !== "string" || !mine() || !field.current) return;
      const typing = e.target instanceof Element && e.target.closest("input, textarea, select, [contenteditable]");
      const slash = e.key === "/" && !e.metaKey && !e.ctrlKey && !typing;
      const find = (e.metaKey || e.ctrlKey) && !e.shiftKey && e.key.toLowerCase() === "f";
      if ((!slash && !find) || document.querySelector("dialog[open]")) return;
      e.preventDefault();
      field.current.focus();
      field.current.select();
    };
    document.addEventListener("click", click);
    document.addEventListener("keydown", key);
    addEventListener("popstate", pop);
    return () => {
      document.removeEventListener("click", click);
      document.removeEventListener("keydown", key);
      removeEventListener("popstate", pop);
    };
  }, []);

  useEffect(() => {
    if (code !== null) void offer(code, drop);
  }, [code]);

  const copy = () => {
    const link = share(location.origin);
    const done = navigator.clipboard ? navigator.clipboard.writeText(link) : Promise.reject(new Error("no clipboard"));
    done.then(
      () => toast("Link copied", { kind: "ok" }),
      () => toast("The link could not be copied", { kind: "warn" }),
    );
  };

  const keep = () => text(JSON.stringify(pack(), null, 2), `${PREFIX}settings`, "json");

  const load = async (e: ChangeEvent<HTMLInputElement>) => {
    const one = e.target.files?.[0];
    e.target.value = "";
    if (!one) return;
    try {
      unpack(JSON.parse(await one.text()));
      location.reload();
    } catch {
      toast("That file holds no settings", { kind: "warn" });
    }
  };

  const drop = () => {
    history.replaceState(state(), "", path);
    setHere(path);
  };

  const press = (e: Pressed<HTMLInputElement>) => {
    if (e.key === "Enter") {
      e.preventDefault();
      if (words.length && hits.length) open(hits[0].href);
    } else if (e.key === "ArrowDown") {
      e.preventDefault();
      list.current?.querySelector("a")?.focus();
    }
  };

  const walk = (e: Pressed<HTMLUListElement>) => {
    const links = [...(list.current?.querySelectorAll("a") ?? [])];
    const at = (links as readonly (Element | null)[]).indexOf(document.activeElement);
    const to =
      e.key === "ArrowDown"
        ? at + 1
        : e.key === "ArrowUp"
          ? at - 1
          : e.key === "Home"
            ? 0
            : e.key === "End"
              ? links.length - 1
              : null;
    if (at < 0 || to === null) return;
    e.preventDefault();
    if (to < 0) field.current?.focus();
    else links[Math.min(to, links.length - 1)].focus();
  };

  return (
    <div className={pane ? "settings two" : "settings"} ref={box}>
      <div className="side">
        <div className="top">
          <Search
            ref={field}
            label="Search settings"
            value={query}
            placeholder="Search"
            enterKeyHint="go"
            onChange={setQuery}
            onKeyDown={press}
          />
          <Menu
            className="icon"
            aria-label="More"
            rows={[
              ["Copy settings link", copy, "link"],
              ["Export settings", keep, "download"],
              ["Import settings…", () => file.current?.click()],
            ]}
          >
            <Icon name="more" />
          </Menu>
          <input ref={file} type="file" accept="application/json,.json" hidden onChange={load} />
        </div>
        <nav aria-label="Settings">
          {hits.length ? (
            <ul className="list" ref={list} onKeyDown={walk}>
              {hits.map((one) => (
                <Row key={one.id} one={one} on={one.id === shown} labels={found(one)} />
              ))}
            </ul>
          ) : (
            <Empty icon="search" title="No Results" text={`Nothing matches ${query.trim()}.`} />
          )}
        </nav>
      </div>
      {pane && (
        <div className="detail">
          <Body key={shown} node={pane} site={site} />
        </div>
      )}
    </div>
  );
}
