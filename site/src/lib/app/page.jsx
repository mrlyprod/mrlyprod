import { createContext, useCallback, useContext, useEffect, useMemo, useRef, useState } from "react";
import { Chip } from "../designs/chip.jsx";
import { choose, current, fits, keys as owned, place, read } from "../designs/store.js";
import { source } from "../glass/feed.js";
import { Accordion, Btn, Export, Fold, Icon, pair } from "./controls.jsx";
import { Bar, Frame, SIDES, usePanes } from "./frame.jsx";
import { patch as diff, fresh, redo, record as remember, undo } from "./history.js";
import { Rolls, rolls, useUnits } from "./hooks.js";
import { island } from "./island.jsx";
import { flip, Hints, Info, Keys, useKeys } from "./keys.jsx";
import { Knob, Knobs, useAccordion } from "./knobs.jsx";
import { menus, publish } from "./menus.js";
import { share, useQuery } from "./query.js";
import { record as capture } from "./record.js";
import { run } from "./run.js";
import { asked, defaults as knobs, optionsOf, tidy, viewing } from "./spec.js";

const Stage = createContext(null);

export function Scene({ make, value, onReady, ...rest }) {
  const stage = useContext(Stage);
  const at = useRef(null);
  const live = useRef(null);
  const made = useRef("");
  const ready = useRef(onReady);
  ready.current = onReady;
  const [n, bump] = useState(0);
  const key = JSON.stringify(value);
  useEffect(() => {
    const canvas = at.current;
    const frame = typeof canvas.closest === "function" ? canvas.closest(".frame") : null;
    const drop = frame ? source(canvas, () => frame.getBoundingClientRect()) : null;
    const stop = run(canvas, make, value, { still: stage?.still });
    live.current = stop;
    made.current = key;
    ready.current?.({ ...stop.scene, canvas, pause: stop.pause, play: stop.play, fix: stop.fix });
    return () => {
      ready.current?.(null);
      live.current = null;
      stop();
      drop?.();
    };
  }, [make, n]);
  useEffect(() => {
    if (made.current === key) return;
    if (live.current?.set(value)) made.current = key;
    else bump((was) => was + 1);
  }, [key]);
  return <canvas key={n} ref={at} {...rest} />;
}

const PAUSE = { key: " ", label: "Pause", act: "pause" };

const PLAY = { ...PAUSE, label: (_value, handle) => (handle?.playing?.() === false ? "Play" : "Pause") };

const WATCH = 250;

const SHOWN = 3;

const RECORD = { key: "v", label: "Record", act: "record" };

const RATIOS = ["9:16", "16:9"];

const clock = (secs) => `${Math.floor(secs / 60)}:${String(Math.floor(secs % 60)).padStart(2, "0")}`;

const BASE = [
  { key: "r", label: "Reroll", act: "reroll" },
  { key: "z", label: "Zen", act: "zen" },
  { key: "?", label: "Keys", act: "keys" },
];

const MORE = [
  { key: "e", label: "Export", act: "export" },
  { key: "i", label: "Info", act: "info" },
];

export const tail = (told) => MORE.filter((row) => told || row.act !== "info");

const COPIED = 1500;

const same = (a, b) => JSON.stringify(a) === JSON.stringify(b);

export function quiet(on) {
  if (typeof document === "undefined") return;
  if (on) document.documentElement.dataset.zen = "";
  else delete document.documentElement.dataset.zen;
}

const roll = () => Math.floor(Math.random() * 4294967296);

const SEED = { key: "seed", label: "Seed", kind: "number", def: 0, min: 0, max: 4294967295, step: 1 };

const changed = (next, was) => Object.fromEntries(Object.entries(next).filter(([key, v]) => v !== was[key]));

export function values(defaults, spec) {
  return spec ? { ...defaults, ...knobs(spec) } : defaults;
}

export function carried(params, slot, one) {
  if (!slot || slot.blank || String(params.get(slot.code) ?? "").trim() || !fits(slot, one)) return params;
  for (const [key, v] of Object.entries(place(slot, one))) params.set(key, v);
  return params;
}

function seeded(keys, slot) {
  const params = carried(new URLSearchParams(location.search), slot, slot ? current() : null);
  for (const key of [...params.keys()]) if (!keys.includes(key)) params.delete(key);
  if (!params.has("seed")) params.set("seed", roll());
  history.replaceState(history.state, "", `${location.pathname}?${params}${location.hash}`);
}

const keyed = (rows, key) => rows.some((row) => [].concat(row.key).includes(key));

export function toggle(handle, paused, taking, cut) {
  if (taking) {
    cut();
    return paused;
  }
  if (paused) handle.play();
  else handle.pause();
  return !paused;
}

export function resume(handle, paused) {
  if (paused) handle.play();
  return false;
}

export const keymap = (keys, more = [], { still = false, own = false } = {}) => [
  ...(still ? [] : [own ? PLAY : PAUSE]),
  ...keys,
  ...[BASE[0], ...more, ...BASE.slice(1)].filter((row) => !keyed(keys, row.key)),
];

export const labels = (map, value, handle) =>
  map.map((row) => (typeof row.label === "function" ? { ...row, label: row.label(value, handle) } : row));

const owns = (handle) => typeof handle?.toggle === "function" && typeof handle?.playing === "function";

export const called = (name, id, value, units) =>
  typeof name === "function" ? name(value, units) : `${id}-${value.seed}`;

const resting = (handle, own, paused) => (own ? handle?.playing?.() === false : paused);

export const live = (map, value) => map.filter((row) => !row.when || row.when(value));

export const menu = (record, begin, more = []) => [
  ...(record ? RATIOS.map((at) => [`Video ${at}`, () => begin(at)]) : []),
  ...more,
];

const TEXT =
  "input[type=text], input[type=number], input[type=search], input:not([type]), textarea, [contenteditable]:not([contenteditable=false])";

const typing = (e) => typeof e.target?.closest === "function" && Boolean(e.target.closest(TEXT));

export function undoing(e) {
  if (!(e.metaKey || e.ctrlKey) || e.altKey || typing(e)) return 0;
  const key = e.key.toLowerCase();
  if (key === "z") return e.shiftKey ? 1 : -1;
  return key === "y" && !e.shiftKey ? 1 : 0;
}

export const knobbed = (spec, kept = []) =>
  Object.fromEntries((spec ?? []).filter((row) => !kept.includes(row.key)).map((row) => [row.key, row.def]));

function App({
  Widget,
  defaults,
  spec,
  units,
  keys,
  actions,
  gestures,
  record,
  about = [],
  id,
  title,
  calm = false,
  name,
  design,
}) {
  const [raw, set] = useQuery(defaults);
  const value = spec ? { ...raw, ...tidy(spec, raw, units) } : raw;
  const url = spec ? { ...value, ...asked(spec, raw, units) } : value;
  const scene = useRef(null);
  const [paused, setPaused] = useState(false);
  const take = useRef(null);
  const ratio = useRef(RATIOS[0]);
  const [taking, setTaking] = useState("");
  const [rec, setRec] = useState("");
  const [zen, setZen] = useState(false);
  const [more, setMore] = useState([]);
  const [rows, setRows] = useState([]);
  const [copied, setCopied] = useState(false);
  const hush = useRef(0);
  const [fixed, setFixed] = useState(false);
  const [mode, setMode] = useState({ still: calm, own: false });
  const [, setSign] = useState("");
  const stack = useRef(fresh());
  const [, setDepth] = useState(0);
  const touched = useRef(false);
  const told = useMemo(rolls, []);
  const panes = usePanes();
  const folds = useAccordion(id, spec);
  const stage = useMemo(() => ({ still: calm }), [calm]);
  const status = useCallback((next) => setRows((was) => (same(was, next ?? []) ? was : (next ?? []))), []);
  useEffect(() => {
    const on = () => (touched.current = true);
    addEventListener("pointerdown", on, true);
    addEventListener("keydown", on, true);
    return () => {
      removeEventListener("pointerdown", on, true);
      removeEventListener("keydown", on, true);
    };
  }, []);
  useEffect(() => {
    quiet(zen);
    if (!zen) return undefined;
    const leave = (e) => {
      if (e.key !== "Escape" || document.querySelector("dialog[open]")) return;
      e.preventDefault();
      setZen(false);
    };
    addEventListener("keydown", leave);
    return () => {
      removeEventListener("keydown", leave);
      quiet(false);
    };
  }, [zen]);
  const write = (next) => {
    const patch = changed(next, raw);
    if (!Object.keys(patch).length) return;
    if (touched.current) {
      stack.current = remember(stack.current, raw, Date.now());
      setDepth((n) => n + 1);
    }
    set(patch);
  };
  const onChange = (patch) => {
    const next = { ...value, ...patch };
    write(spec ? { ...next, ...tidy(spec, next, units), ...asked(spec, { ...raw, ...patch }, units) } : next);
  };
  const travel = (by) => {
    const moved = (by < 0 ? undo : redo)(stack.current, raw);
    if (!moved) return;
    stack.current = moved.stack;
    setDepth((n) => n + 1);
    set(diff(raw, moved.to));
  };
  useEffect(() => write(url), []);
  const slotted = design ? read(design, url) : null;
  const picked = slotted ? JSON.stringify(slotted) : "";
  const met = useRef(picked);
  useEffect(() => {
    if (met.current === picked) return;
    met.current = picked;
    if (slotted && touched.current && fits(design, slotted)) choose(slotted);
  }, [picked]);
  const ready = useCallback((handle) => {
    take.current?.stop();
    scene.current = handle;
    setPaused(false);
    setFixed(Boolean(handle?.fix));
    setMode({ still: calm || Boolean(handle?.still), own: owns(handle) });
  }, []);
  const all = useMemo(() => (record ? [...keys, RECORD] : keys), [keys, record]);
  const on = live(all, value);
  const known = about.length > 0 || rows.length > 0;
  const map = useMemo(
    () => keymap(on, tail(known), mode),
    [all, on.map((row) => all.indexOf(row)).join(), known, mode],
  );
  const shown = labels(map, value, scene.current);
  const signed = useRef(null);
  signed.current = () =>
    [mode.own ? scene.current?.playing?.() : "", ...labels(map, value, scene.current).map((row) => row.label)].join(
      "|",
    );
  const sync = () => setSign(signed.current());
  const watching = mode.own || map.some((row) => typeof row.label === "function");
  useEffect(() => {
    if (!watching) return undefined;
    const timer = setInterval(sync, WATCH);
    return () => clearInterval(timer);
  }, [watching]);
  const file = called(name, id, value, units);
  const pause = () => {
    const handle = scene.current;
    if (!handle) return;
    if (mode.own && !take.current) handle.toggle();
    else setPaused(toggle(handle, paused, Boolean(take.current), cut));
  };
  const begin = (at) => {
    const handle = scene.current;
    if (!record || !handle?.fix || take.current) return;
    ratio.current = at;
    setPaused(resume(handle, paused));
    try {
      const one = capture(handle, { ratio: at, name: file, audio: handle.sound?.() });
      take.current = one;
      setTaking(at);
      one.done.then(() => {
        if (take.current !== one) return;
        take.current = null;
        setTaking("");
      });
    } catch (error) {
      console.error(error);
    }
  };
  const cut = () => take.current?.stop();
  useEffect(() => cut, []);
  useEffect(() => () => clearTimeout(hush.current), []);
  useEffect(() => {
    if (!taking) return undefined;
    const tick = () => setRec(`rec ${clock(take.current?.secs() ?? 0)} ${taking} late ${take.current?.late() ?? 0}`);
    tick();
    const timer = setInterval(tick, 250);
    return () => clearInterval(timer);
  }, [taking]);
  const reroll = () => {
    const seed = roll();
    onChange({ seed });
    told.tell(seed);
  };
  const reset = () => onChange(knobbed(spec, design ? owned(design) : []));
  const base = {
    pause,
    reroll,
    zen: () => setZen((was) => !was),
    record: () => (take.current ? cut() : begin(ratio.current)),
    ...Object.fromEntries(
      Object.entries(actions).map(([act, fn]) => [act, (e) => scene.current && fn(scene.current, e)]),
    ),
  };
  const acts = Object.fromEntries(
    Object.entries(base).map(([act, fn]) => [
      act,
      (e) => {
        fn(e);
        sync();
      },
    ]),
  );
  useKeys(map, acts);
  const primary = shown.find((row) => row.button && acts[row.act]);
  const copy = () => {
    navigator.clipboard?.writeText(`${location.origin}${location.pathname}${share(url)}`).then(
      () => {
        clearTimeout(hush.current);
        setCopied(true);
        hush.current = setTimeout(() => setCopied(false), COPIED);
      },
      () => {},
    );
  };
  const view = spec ? live(spec.filter(viewing), value)[0] : undefined;
  const files = menu(record, begin, more);
  const sides = Object.fromEntries(
    Object.keys(SIDES)
      .filter(panes.has)
      .map((side) => [side, panes.open[side]]),
  );
  const model = menus({
    exports: files.map(([text]) => text),
    undo: stack.current.past.length > 0,
    redo: stack.current.future.length > 0,
    reset: Boolean(spec),
    view: view ? { value: value[view.key], options: optionsOf(view, value, units).map(pair) } : null,
    panes: sides,
    keys: labels(on, value, scene.current),
    told: known,
  });
  const run = useRef(null);
  run.current = (act) => {
    if (act.startsWith("export:")) return files[Number(act.slice(7))]?.[1]();
    if (act.startsWith("pane:")) return panes.flip(act.slice(5));
    if (act.startsWith("view:"))
      return onChange({
        [view.key]: optionsOf(view, value, units)
          .map(pair)
          .map(([one]) => one)
          .find((one) => String(one) === act.slice(5)),
      });
    const [name, key] = act.split("@");
    const fn = {
      export: () => flip("dialog.export"),
      copy,
      undo: () => travel(-1),
      redo: () => travel(1),
      reset,
      keys: () => flip("dialog.keys"),
      info: () => flip("dialog.info"),
      ...acts,
    }[name];
    const row = map.find((one) => one.act === name);
    fn?.({ key: key ?? (row ? [].concat(row.key)[0] : ""), preventDefault: () => {} });
  };
  const sign = JSON.stringify(model);
  useEffect(() => publish({ title, menus: model, run: (act) => run.current(act) }), [sign]);
  useEffect(() => () => publish(null), []);
  useEffect(() => {
    const press = (e) => {
      const by = undoing(e);
      if (!by || document.querySelector("dialog[open]")) return;
      e.preventDefault();
      run.current(by < 0 ? "undo" : "redo");
    };
    addEventListener("keydown", press);
    return () => removeEventListener("keydown", press);
  }, []);
  return (
    <Rolls.Provider value={told}>
      <Accordion.Provider value={folds}>
        <Stage.Provider value={stage}>
          <Frame label={title} gestures={gestures} panes={panes}>
            <Widget
              value={value}
              onChange={onChange}
              onReady={ready}
              onExport={setMore}
              onStatus={status}
              units={units}
            />
            <Btn className="icon unzen" aria-label="Leave zen" onClick={acts.zen}>
              <Icon name="fullscreen_exit" />
            </Btn>
            {spec && (
              <Bar side="right">
                <Knobs spec={spec} value={value} units={units} onChange={onChange} />
              </Bar>
            )}
            {spec && (
              <Bar side="tail">
                <Fold
                  name="Seed"
                  live={String(value.seed)}
                  open={folds.has("Seed")}
                  onToggle={(on) => folds.flip("Seed", on)}
                >
                  <Knob row={{ ...SEED, def: value.seed }} value={value.seed} onChange={onChange} />
                </Fold>
              </Bar>
            )}
            <Bar side="status">
              {known && (
                <Btn className="icon info" aria-label="Info" onClick={() => flip("dialog.info")}>
                  <Icon name="info" />
                </Btn>
              )}
              <span>seed {value.seed}</span>
              {!mode.still && resting(scene.current, mode.own, paused) && <span>paused</span>}
              {taking && <span>{rec}</span>}
              {copied && <span>link copied</span>}
              {rows.slice(0, SHOWN).map(([label, v]) => (
                <span key={label} className="out">
                  {label} {v}
                </span>
              ))}
            </Bar>
            <Bar side="hints">
              <Hints map={shown} />
              <Btn className="icon keys" aria-label="Keys" onClick={() => flip("dialog.keys")}>
                <Icon name="keyboard" />
              </Btn>
            </Bar>
            <Bar side="actions">
              {design && (!design.when || design.when(value)) && (
                <Chip slot={design} value={url} onChange={onChange} math={units?.math} />
              )}
              {view && (
                <div className="views">
                  <Knob row={view} value={value[view.key]} values={value} units={units} onChange={onChange} />
                </div>
              )}
              {taking && (
                <Btn primary onClick={cut}>
                  Stop
                </Btn>
              )}
              {primary && (
                <Btn primary onClick={acts[primary.act]}>
                  {primary.label}
                </Btn>
              )}
              <Btn className="reroll" onClick={acts.reroll}>
                <Icon name="casino" />
                Reroll
              </Btn>
              <Btn className="export" onClick={() => flip("dialog.export")}>
                <Icon name="download" />
                Export
              </Btn>
              <Btn className="share" onClick={copy}>
                <Icon name="ios_share" />
                Share
              </Btn>
            </Bar>
            <Keys map={shown} />
            {known && <Info about={about} rows={rows} />}
            <Export
              canvas={() => scene.current?.canvas ?? document.querySelector(".frame canvas")}
              name={file}
              draw={() => scene.current?.draw?.()}
              fix={fixed && !taking ? (next) => scene.current?.fix?.(next) : undefined}
              more={files}
            />
          </Frame>
        </Stage.Provider>
      </Accordion.Provider>
    </Rolls.Provider>
  );
}

export function Held({ units, still, ...rest }) {
  const held = useUnits(units);
  const on = !units || Boolean(held);
  useEffect(() => {
    if (on) still?.remove();
  }, [on]);
  return on ? <App {...rest} units={held ?? undefined} /> : null;
}

export function page(
  Widget,
  defaults,
  {
    spec,
    units,
    keys = [],
    actions = {},
    gestures = false,
    record = false,
    about = [],
    still: calm = false,
    name,
    design,
  } = {},
) {
  const start = values(defaults, spec);
  return island((host) => {
    const still = host.nextElementSibling;
    const title = still?.querySelector("h1")?.textContent ?? "";
    const id = host.dataset.app;
    seeded(Object.keys(start), design);
    return {
      node: (
        <Held
          units={units}
          still={still}
          Widget={Widget}
          defaults={start}
          spec={spec}
          keys={keys}
          actions={actions}
          gestures={gestures}
          record={record}
          about={about}
          id={id}
          title={title}
          calm={calm}
          name={name}
          design={design}
        />
      ),
      close: () => still && host.after(still),
    };
  });
}
