import { useEffect, useId, useRef, useState } from "react";
import { flushSync } from "react-dom";
import { Btn, Icon, Modal } from "../app/controls.tsx";
import { useUnit } from "../app/hooks.ts";
import type { Value } from "../app/spec.ts";
import { Browser, called, here, named, valid } from "./browse.tsx";
import type { Mrly } from "./engine.ts";
import { Thumb, useTheme } from "./pick.tsx";
import type { Design, Slot } from "./store.ts";
import { choose, place, read, star, starred, useDesigns } from "./store.ts";

/* TYPES */

export type Rect = Pick<DOMRect, "left" | "top" | "width" | "bottom">;

export type ChipProps = { slot: Slot; value: Value; onChange: (patch: Value) => void; math?: Mrly | null };

const PX = 22;

/* SPOT */

export function spot(rect: Rect, high: number) {
  const up = rect.top > high - rect.bottom;
  return { x: Math.round(rect.left + rect.width / 2), y: Math.round(up ? high - rect.top : rect.bottom), up };
}

const hide = (pop: HTMLElement | null) => pop?.matches?.(":popover-open") && pop.hidePopover();

function aim(pop: HTMLElement, chip: HTMLElement | null) {
  if (!chip) return;
  const { x, y, up } = spot(chip.getBoundingClientRect(), innerHeight);
  pop.style.setProperty("--x", `${x}px`);
  pop.style.setProperty("--y", `${y}px`);
  pop.toggleAttribute("data-up", up);
}

/* CHIP */

export function Chip({ slot, value, onChange, math: given }: ChipProps) {
  const math = useUnit("math", given);
  const id = `designs${useId().replace(/[^\w-]/g, "")}`;
  const chip = useRef<HTMLButtonElement>(null);
  const pop = useRef<HTMLDivElement>(null);
  const box = useRef<HTMLDialogElement>(null);
  const [open, setOpen] = useState(false);
  const [browsing, setBrowsing] = useState(false);
  const tick = useTheme();
  useDesigns();
  const now = here(slot, value, math);
  const blank = Boolean(slot.blank) && !read(slot, value).code;
  const ok = !blank && valid(math, now);
  const on = ok && starred(now);
  const name = called(math, now);
  useEffect(() => {
    const el = pop.current;
    if (!el) return undefined;
    const before = (e: ToggleEvent) => {
      if (e.newState !== "open") return;
      aim(el, chip.current);
      flushSync(() => setOpen(true));
    };
    const after = (e: ToggleEvent) => (e.newState === "open" ? el.querySelector("input")?.focus() : setOpen(false));
    el.addEventListener("beforetoggle", before);
    el.addEventListener("toggle", after);
    return () => {
      el.removeEventListener("beforetoggle", before);
      el.removeEventListener("toggle", after);
    };
  }, []);
  const pick = (one: Design) => {
    onChange(place(slot, one));
    choose(one);
    box.current?.close?.();
    hide(pop.current);
  };
  const browse = () => {
    hide(pop.current);
    box.current?.showModal?.();
    flushSync(() => setBrowsing(true));
    box.current?.querySelector("input")?.focus();
  };
  return (
    <>
      <button type="button" ref={chip} className="design" popoverTarget={id} title="Designs">
        {ok && <Thumb math={math} dim={now.dim} base={now.base} code={now.code} px={PX} tick={tick} />}
        <span>{blank ? slot.blank : named(math, now) || "Design"}</span>
      </button>
      <div id={id} ref={pop} popover="auto" role="dialog" className="chooser" data-lens="menu" aria-label="Designs">
        {open && (
          <>
            <header>
              <b>{name || `code ${now.code}`}</b>
              <small>
                {[name && `code ${now.code}`, `${now.dim}D`, `base ${now.base}`].filter(Boolean).join(", ")}
              </small>
              <button
                type="button"
                className="icon"
                aria-pressed={on}
                aria-label="Favorite"
                disabled={!ok}
                onClick={() => star(now)}
              >
                <Icon name={on ? "star_fill" : "star"} />
              </button>
            </header>
            <Browser compact slot={slot} value={value} math={math} onPick={pick} />
            <Btn className="all" onClick={browse}>
              <Icon name="grid_view" />
              Browse all
            </Btn>
          </>
        )}
      </div>
      <Modal ref={box} className="browse" title="Designs" onClose={() => setBrowsing(false)}>
        {browsing && <Browser slot={slot} value={value} math={math} onPick={pick} />}
      </Modal>
    </>
  );
}
