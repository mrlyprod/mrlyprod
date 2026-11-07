import type { FocusEvent, KeyboardEvent as Pressed } from "react";
import { useEffect, useMemo, useRef, useState } from "react";
import { flushSync } from "react-dom";
import type { MenuRow } from "../kit/controls.tsx";
import { useMenu } from "../kit/controls.tsx";
import type { Draft } from "../kit/drag.ts";
import { placed, sortable } from "../kit/drag.ts";
import { text } from "../kit/export.ts";
import { px } from "../kit/px.ts";
import type { Shelf as Section } from "../kit/rows.ts";
import { parts } from "./active.ts";
import { AppIcon } from "./appicon.tsx";
import { app, away, dock, KNOWN, PAD, recent, redock, shelves as saved, shelve, take, unshelve } from "./catalog.ts";
import type { Tile } from "./design.ts";
import { exported, RECENT, rank, renamed, spare } from "./launch.ts";
import type { Spot } from "./lens.ts";
import { PANES } from "./parts.tsx";
import { WORD } from "./welcome.ts";

export type Hit = { id?: string; title: string; href: string; more?: string };

export type Group = [string, Hit[]];

type Door = Hit & { id: string };

type Groups = [] | [["Apps", Door[]], ...Group[]];

export type Action = [string, () => void];

type Placed = Pick<Draft, "id" | "index">;

type Hooks = {
  preview: (slot: string, next: Placed | null) => void;
  commit: (slot: string, next: Placed | null) => void;
  order: (next: Placed) => void;
  move: (from: string, id: string, to: string) => void;
  menu: (slot: string, id: string, el: HTMLElement) => void;
  chip: (name: string) => void;
  name: (name: string) => void;
  rename: (from: string, to: string) => string;
  drop: (name: string) => void;
  add: () => void;
};

type Said = { words: string; undo?: () => void };

type Asked = { slot: string; id: string; at: Spot };

type Shown = { slot: string; name: string; kind: string; ids: string[] };

const CAP = { apps: 27, list: 6 };
const SAID = 5000;
const EDGE = 8;
const SEEN = " recent";

const section = (id: string) => PAD.sections.find((one) => one.apps.includes(id))?.name ?? "";

const APPS = [...KNOWN].map((id) => ({
  id,
  title: app(id)!.title,
  more: `${app(id)!.lead} ${section(id)}`,
  aliases: app(id)!.meta.aliases ?? [],
  href: app(id)!.route,
}));

let pull: (() => void) | null = null;

export function opened(host: ParentNode) {
  pull?.();
  const field = host.querySelector<HTMLInputElement>("input[data-find]");
  if (!field || (!field.value && !matchMedia("(hover: hover)").matches)) return;
  field.focus({ preventScroll: true });
  field.setSelectionRange(field.value.length, field.value.length);
}

const without = (map: Record<string, Placed | null>, key: string) =>
  Object.fromEntries(Object.entries(map).filter(([one]) => one !== key));

const MENU = (e: Pressed<Element>) => e.key === "ContextMenu" || (e.key === "F10" && e.shiftKey);

const tile = (id: string): Tile[] => {
  const one = app(id);
  return one ? [{ id, title: one.title, href: one.route, art: <AppIcon id={id} /> }] : [];
};

const head = (slot: string) => `shelf-${slot || "recent"}`.replace(/\W+/g, "-");

export function Launchpad() {
  const [shelves, setShelves] = useState(saved);
  const [drafts, setDrafts] = useState<Record<string, Placed | null>>({});
  const [order, setOrder] = useState<Draft | null>(null);
  const [chip, setChip] = useState("");
  const [query, setQuery] = useState(() => take() || new URLSearchParams(location.search).get("q") || "");
  const [hit, setHit] = useState(0);
  const [naming, setNaming] = useState("");
  const [said, setSaid] = useState<Said | null>(null);
  const [seen, setSeen] = useState(recent);
  const [menu, setMenu] = useState<Asked | null>(null);
  const live = useRef(shelves);
  const clock = useRef<ReturnType<typeof setTimeout> | 0>(0);
  const toast = useRef<HTMLParagraphElement>(null);
  const root = useRef<HTMLDivElement>(null);
  const body = useRef<HTMLDivElement>(null);
  const field = useRef<HTMLInputElement>(null);
  const more = useRef<HTMLButtonElement>(null);
  const names = useRef<HTMLDivElement>(null);
  const pop = useRef<HTMLDivElement>(null);
  const lists = useRef(new Map<string, HTMLElement>());
  const back = useRef("");
  const hooks = useRef<Hooks | null>(null);
  const searching = query.trim() !== "";

  const keep = (next: Section[]) => {
    shelve(next);
    live.current = next;
    setShelves(next);
  };
  const tell = (words: string, undo?: () => void) => {
    const fade = () => {
      if (toast.current?.contains(document.activeElement)) clock.current = setTimeout(fade, SAID);
      else setSaid(null);
    };
    setSaid({ words, undo });
    clearTimeout(clock.current);
    clock.current = setTimeout(fade, SAID);
  };
  const named = (id: string) => app(id)?.title ?? id;

  hooks.current = {
    preview: (slot, next) => flushSync(() => setDrafts((was) => ({ ...was, [slot]: next }))),
    commit: (slot, next) =>
      flushSync(() => {
        setDrafts((was) => without(was, slot));
        if (next && slot !== SEEN)
          keep(live.current.map((one) => (one.name === slot ? { ...one, apps: placed(one.apps, next) } : one)));
      }),
    order: (next) => {
      const by = new Map(live.current.map((one) => [one.name, one] as const));
      keep(
        placed(
          live.current.map((one) => one.name),
          next,
        ).map((name) => by.get(name)!),
      );
    },
    move: (from, id, to) => {
      const end = live.current.find((one) => one.name === to)?.apps.length ?? 0;
      hooks.current!.commit(from, { id, index: null });
      hooks.current!.commit(to, { id, index: end });
      if (chip) flushSync(() => setChip(to));
      [...(lists.current.get(to)?.children ?? [])]
        .find((one) => one.getAttribute("data-id") === id)
        ?.querySelector<HTMLElement>("a")
        ?.focus({ preventScroll: true });
      tell(`Moved ${named(id)} to ${to}.`);
    },
    menu: (slot, id, el) => {
      const box = el.getBoundingClientRect();
      setMenu({ slot, id, at: { x: box.left, y: box.bottom } });
    },
    chip: setChip,
    name: setNaming,
    rename: (from, to) => {
      const next = renamed(live.current, from, to);
      setNaming("");
      if (next === live.current) return from;
      keep(next);
      if (chip === from) setChip(to.trim());
      return to.trim();
    },
    drop: (name) => {
      keep(live.current.filter((one) => one.name !== name));
      if (chip === name) setChip("");
    },
    add: () => {
      const name = spare(live.current);
      keep([...live.current, { name, apps: [] }]);
      setQuery("");
      setChip("");
      setNaming(name);
    },
  };

  const save = () => {
    const json = exported(live.current, dock());
    text(json, "launchpad", "json");
    navigator.clipboard?.writeText(json).catch(() => {});
    tell("Saved launchpad.json and copied it.");
  };
  const reset = () => {
    const was = { shelves: live.current, dock: dock() };
    unshelve();
    setChip("");
    tell("Layout reset.", () => {
      clearTimeout(clock.current);
      keep(was.shelves);
      redock(was.dock);
      setSaid(null);
    });
  };
  const actions = (slot: string, id: string): Action[] => {
    const groups = dock();
    const docked = groups.some((ids) => ids.includes(id));
    const add = () => {
      redock(groups.length ? groups.map((ids, g) => (g === groups.length - 1 ? [...ids, id] : ids)) : [[id]]);
      tell(`Added ${named(id)} to the Dock.`);
    };
    const remove = () => {
      redock(groups.map((ids) => ids.filter((one) => one !== id)));
      tell(`Removed ${named(id)} from the Dock.`);
    };
    const moves =
      slot === SEEN
        ? []
        : live.current
            .filter((one) => one.name !== slot)
            .map((one): Action => [`Move to ${one.name}`, () => hooks.current!.move(slot, id, one.name)]);
    return [docked ? ["Remove from Dock", remove] : ["Add to Dock", add], ...moves];
  };

  const groups = useMemo(
    (): Groups =>
      searching
        ? [
            ["Apps", rank(query, APPS, CAP.apps)],
            ["Settings", rank(query, PANES, CAP.list)],
          ]
        : [],
    [query, searching],
  );
  const hits = groups.flatMap(([, list]) => list);
  const at = hits.length ? Math.min(hit, hits.length - 1) : -1;
  const ids = [...new Set(seen.map((one) => one.id))].slice(0, RECENT);
  const shown: Shown[] = searching
    ? []
    : [
        ...(chip === "" && ids.length > 0 ? [{ slot: SEEN, name: "Recent", kind: "recent", ids }] : []),
        ...shelves
          .filter((one) => !chip || one.name === chip)
          .map((one) => ({ slot: one.name, name: one.name, kind: "launchpad", ids: one.apps })),
      ];
  const lay = [searching, ...shown.map((one) => `${one.kind}:${one.slot}`)].join("\n");

  useEffect(() => {
    const fresh = () => setSeen(recent());
    const reload = () => {
      live.current = saved();
      setShelves(live.current);
    };
    const find = () => {
      field.current?.focus();
      field.current?.select();
    };
    const key = (e: KeyboardEvent) => {
      if (e.key !== "Escape" || e.defaultPrevented || document.querySelector(":popover-open, dialog[open]")) return;
      e.preventDefault();
      away();
    };
    addEventListener("recent", fresh);
    addEventListener("shelves", reload);
    addEventListener("find", find);
    document.addEventListener("keydown", key);
    pull = () => {
      const more = take();
      if (more) flushSync(() => setQuery((was) => was + more));
    };
    return () => {
      pull = null;
      removeEventListener("recent", fresh);
      removeEventListener("shelves", reload);
      removeEventListener("find", find);
      document.removeEventListener("keydown", key);
      clearTimeout(clock.current);
    };
  }, []);

  useEffect(() => {
    const offs = shown.flatMap(({ slot, kind }) => {
      const el = lists.current.get(slot);
      return el
        ? [
            sortable(el, {
              kind,
              takes: (from) => (kind === "launchpad" && from === "launchpad" ? "move" : ""),
              scroller: body.current,
              name: (id) => app(id)?.title ?? id,
              preview: (next) => hooks.current!.preview(slot, next),
              commit: (next) => hooks.current!.commit(slot, next),
            }),
          ]
        : [];
    });
    if (names.current)
      offs.push(
        sortable(names.current, {
          kind: "sections",
          axis: "grid",
          takes: (from) => (from === "sections" ? "move" : ""),
          preview: (next) => flushSync(() => setOrder(next)),
          commit: (next) =>
            flushSync(() => {
              setOrder(null);
              if (next) hooks.current!.order(next);
            }),
        }),
      );
    return () => {
      for (const off of offs) off();
    };
  }, [lay]);

  useEffect(() => {
    if (!back.current || naming || !names.current) return;
    [...(names.current.children as HTMLCollectionOf<HTMLElement>)]
      .find((one) => one.dataset.id === back.current)
      ?.focus({ preventScroll: true });
    back.current = "";
  });

  useEffect(() => {
    const box = pop.current;
    if (!menu || !box) return undefined;
    box.showPopover?.();
    if (box.offsetWidth) {
      box.style.setProperty("--x", px(Math.max(EDGE, Math.min(menu.at.x, innerWidth - box.offsetWidth - EDGE))));
      box.style.setProperty("--y", px(Math.max(EDGE, Math.min(menu.at.y, innerHeight - box.offsetHeight - EDGE))));
    }
    box.querySelector("button")?.focus();
    const gone = (e: ToggleEvent) => e.newState === "closed" && setMenu(null);
    box.addEventListener("toggle", gone);
    return () => box.removeEventListener("toggle", gone);
  }, [menu?.at]);

  useEffect(() => {
    dispatchEvent(new CustomEvent("shift", { detail: 0 }));
  });

  useEffect(() => {
    if (at >= 0) root.current?.querySelector(`#hit-${at}`)?.scrollIntoView({ block: "nearest" });
  }, [at]);

  const press = (e: Pressed<HTMLInputElement>) => {
    if (e.key === "Escape") {
      e.preventDefault();
      e.stopPropagation();
      if (query) setQuery("");
      else away();
      return;
    }
    if (!searching || !hits.length) return;
    const by = e.key === "Tab" ? (e.shiftKey ? -1 : 1) : e.key === "ArrowDown" ? 1 : e.key === "ArrowUp" ? -1 : 0;
    if (by) {
      e.preventDefault();
      setHit((at + by + hits.length) % hits.length);
      return;
    }
    if (e.key === "Enter") {
      e.preventDefault();
      root.current?.querySelector<HTMLElement>(`#hit-${at} a`)?.click();
    }
  };

  const rows: MenuRow[] = [
    ["New Section", () => hooks.current!.add()],
    ...(chip && !searching ? ([["Rename Section", () => hooks.current!.name(chip)]] satisfies MenuRow[]) : []),
    ["Export JSON", save, "download"],
    ["Reset Layout", reset],
  ];
  const menus = useMenu(rows);
  const asked = menu ? actions(menu.slot, menu.id) : [];
  const renaming = useMemo(() => {
    let done = false;
    const end = (value: string, keyed: boolean) => {
      if (done) return;
      done = true;
      const now = hooks.current!.rename(naming, value);
      if (keyed) back.current = now;
    };
    return {
      ref: (el: HTMLInputElement | null) => el?.focus(),
      defaultValue: naming,
      "aria-label": `Rename ${naming}`,
      onFocus: (e: FocusEvent<HTMLInputElement>) => e.target.select(),
      onBlur: (e: FocusEvent<HTMLInputElement>) => end(e.target.value, false),
      onKeyDown: (e: Pressed<HTMLInputElement>) => {
        if (e.key === "Enter") {
          e.preventDefault();
          end(e.currentTarget.value, true);
        }
        if (e.key === "Escape") {
          e.preventDefault();
          e.stopPropagation();
          end(naming, true);
        }
      },
    };
  }, [naming]);
  const listed = (slot: string) => (el: HTMLElement | null) => {
    if (el) lists.current.set(slot, el);
    else lists.current.delete(slot);
  };

  return (
    <parts.launcher
      word={WORD}
      query={query}
      onQuery={(next) => {
        setQuery(next);
        setHit(0);
      }}
      chips={placed(
        shelves.map((one) => one.name),
        order,
      )}
      chip={chip}
      onChip={(name) => hooks.current!.chip(chip === name ? "" : name)}
      naming={naming || null}
      sections={shown.map(({ slot, name, kind, ids: list }) => {
        const draft = drafts[slot];
        const held = draft && draft.index !== null ? draft.id : null;
        return {
          name,
          tiles: placed(list, draft).flatMap(tile),
          bind: {
            shelf: () => ({ "aria-labelledby": head(slot), "data-slot": slot }),
            head: () => ({
              id: head(slot),
              onDoubleClick: kind === "launchpad" ? () => hooks.current!.name(name) : undefined,
            }),
            list: () => ({ ref: listed(slot) }),
            tile: (id) => () => ({
              "data-id": id,
              "data-held": id === held ? "" : undefined,
              onKeyDown: (e: Pressed<HTMLElement>) => {
                if (!MENU(e)) return;
                e.preventDefault();
                hooks.current!.menu(slot, id, e.currentTarget);
              },
            }),
            ...(kind === "launchpad" && { drop: () => ({ onClick: () => hooks.current!.drop(name) }) }),
          },
        };
      })}
      results={
        searching
          ? groups.map(([name, list]) => ({
              name,
              tiles: name === "Apps" ? list.flatMap((one) => tile(one.id!)) : [],
              hits: name === "Apps" ? [] : list.map((one) => ({ label: one.title, href: one.href })),
            }))
          : null
      }
      at={at}
      said={said?.words ?? null}
      more={menus.items}
      menu={
        menu && {
          label: named(menu.id),
          items: asked.map(([label]) => ({ label })),
          bind: {
            list: () => ({
              ref: pop,
              popover: "auto",
              style: { "--x": px(menu.at.x), "--y": px(menu.at.y) },
              onContextMenu: (e: Event) => e.preventDefault(),
            }),
            item: (i) => () => ({
              onClick: () => {
                pop.current?.hidePopover?.();
                asked[i]![1]();
              },
            }),
          },
        }
      }
      bind={{
        root: () => ({ ref: root }),
        veil: () => ({ onClick: away }),
        find: () => ({
          ref: field,
          "data-find": "",
          placeholder: "Search apps, settings",
          "aria-label": "Search",
          role: "combobox",
          "aria-expanded": searching,
          "aria-controls": "hits",
          "aria-activedescendant": at >= 0 ? `hit-${at}` : undefined,
          autoComplete: "off",
          spellCheck: false,
          enterKeyHint: "go",
          onKeyDown: press,
        }),
        body: () => ({ ref: body }),
        results: () => ({ id: "hits" }),
        hit: (i) => () => ({ id: `hit-${i}` }),
        chips: () => ({ ref: names }),
        chip: (name) => () => ({
          "data-id": name,
          "data-held": order?.id === name && order.index !== null ? "" : undefined,
          onDoubleClick: () => hooks.current!.name(name),
          onKeyDown: (e: Pressed<HTMLElement>) => {
            if (e.key !== "F2") return;
            e.preventDefault();
            hooks.current!.name(name);
          },
        }),
        rename: () => renaming,
        add: () => ({ onClick: () => hooks.current!.add() }),
        more: {
          button: () => ({ ...menus.button(), ref: more, "aria-label": "More" }),
          list: menus.list,
          item: menus.item,
        },
        said: () => ({ ref: toast }),
        undo: said?.undo
          ? () => ({
              onClick: () => {
                if (toast.current?.contains(document.activeElement)) more.current?.focus({ preventScroll: true });
                said.undo!();
              },
            })
          : null,
      }}
    />
  );
}
