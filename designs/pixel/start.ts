import type { World } from "@mrlyprod/ui/shell/design.ts";
import { dark, ink } from "./kit.ts";
import { arcade } from "./sound.ts";
import type { Level, Spray } from "./sparks.ts";
import { sparks } from "./sparks.ts";
import { ARROW, cursor, SPRITES } from "./sprite.tsx";

const INKS = ["--px-a", "--px-b", "--px-c", "--px-d"];

export function hex(text: string): number[] | null {
  const m = /^#([0-9a-f]{6})$/i.exec(text.trim());
  if (!m) return null;
  const n = Number.parseInt(m[1]!, 16);
  return [(n >> 16) / 255, ((n >> 8) & 255) / 255, (n & 255) / 255];
}

const LIVE =
  "a[href], button, input, select, textarea, summary, label, [role=button], [role=menuitem], [role=option], [role=switch], [tabindex]";

const TOP = ":popover-open:not(.pixel.sparks), dialog:modal";

const UP = -Math.PI / 2;

export function press(x: number, y: number, live: boolean): Spray {
  return {
    x,
    y,
    n: live ? 56 : 22,
    angle: UP,
    spread: live ? Math.PI * 1.5 : Math.PI,
    speed: live ? [220, 760] : [120, 420],
    life: [0.35, 0.95],
    size: [3, live ? 9 : 6],
  };
}

export function swipe(x: number, y: number, dx: number, dy: number): Spray {
  return {
    x,
    y,
    n: 7,
    angle: Math.atan2(-dy, -dx),
    spread: Math.PI * 0.9,
    speed: [160, 520],
    life: [0.3, 0.8],
    size: [3, 6],
    push: [dx * 4, dy * 4],
  };
}

export function stroke(x: number, y: number, key: string): Spray {
  if (key === "Backspace" || key === "Delete")
    return {
      x,
      y,
      n: 12,
      angle: Math.PI / 2,
      spread: Math.PI * 0.7,
      speed: [20, 160],
      life: [0.4, 0.9],
      size: [3, 6],
      hot: 1,
    };
  if (key === "Enter")
    return { x, y, n: 70, angle: UP, spread: Math.PI * 2, speed: [200, 900], life: [0.5, 1.2], size: [3, 9] };
  return { x, y, n: 16, angle: UP, spread: Math.PI * 0.6, speed: [180, 520], life: [0.3, 0.7], size: [3, 6] };
}

function caret(fallback: [number, number]): [number, number] {
  const el = document.activeElement;
  if (el instanceof HTMLInputElement || el instanceof HTMLTextAreaElement) {
    const box = el.getBoundingClientRect();
    const style = getComputedStyle(el);
    const em = Number.parseFloat(style.fontSize) || 15;
    const pad = (Number.parseFloat(style.paddingLeft) || 0) + (Number.parseFloat(style.borderLeftWidth) || 0);
    const at = el.selectionStart ?? el.value.length;
    const x = Math.min(box.right - 6, box.left + pad + at * em * 0.6 - el.scrollLeft);
    return [x, box.top + box.height / 2];
  }
  if (el instanceof HTMLElement && el !== document.body && el.id !== "main") {
    const box = el.getBoundingClientRect();
    if (box.width && box.height) return [box.left + box.width / 2, box.top + box.height / 2];
  }
  return fallback;
}

export function start(world: World) {
  const html = document.documentElement;
  const canvas = document.createElement("canvas");
  canvas.className = "pixel sparks";
  canvas.setAttribute("aria-hidden", "true");
  canvas.setAttribute("popover", "manual");
  document.body.append(canvas);
  html.style.setProperty("--px-arrow", cursor(ARROW, "#000000", 2, [0, 0], "default"));
  html.style.setProperty("--px-hand", cursor(SPRITES.pac, "#ffd400", 3, [12, 12], "pointer"));
  let fx: ReturnType<typeof sparks> = null;
  try {
    fx = sparks(canvas);
  } catch (error) {
    console.error(error);
  }
  const sound = arcade(() => html.dataset.sound !== "off");
  const level = (): Level => (world.calm() ? "off" : (html.dataset.sparks as Level) || "high");
  const dress = () => {
    const inks = INKS.map((name) => hex(ink(name, ""))).filter((one): one is number[] => one !== null);
    fx?.theme(inks, !dark());
    fx?.level(level());
  };
  dress();
  let last: [number, number] = [innerWidth / 2, innerHeight * 0.66];
  let held: [number, number] | null = null;
  const down = (e: PointerEvent) => {
    sound.wake(e);
    last = [e.clientX, e.clientY];
    held = last;
    const live = e.target instanceof Element && Boolean(e.target.closest(LIVE));
    fx?.spray(press(e.clientX, e.clientY, live));
    sound.press(e.clientX, live);
  };
  const move = (e: PointerEvent) => {
    if (!held || !e.buttons) return;
    const dx = e.clientX - held[0];
    const dy = e.clientY - held[1];
    if (dx * dx + dy * dy < 100) return;
    held = [e.clientX, e.clientY];
    last = held;
    fx?.spray(swipe(e.clientX, e.clientY, dx, dy));
    sound.swipe();
  };
  const up = () => {
    held = null;
  };
  const key = (e: KeyboardEvent) => {
    if (["Shift", "Meta", "Control", "Alt", "CapsLock"].includes(e.key)) return;
    sound.wake(e);
    const [x, y] = caret(last);
    fx?.spray(stroke(x, y, e.key));
    sound.key(e.key);
  };
  const raise = (e?: Event) => {
    if (e?.target === canvas) return;
    if (canvas.matches(":popover-open")) canvas.hidePopover();
    if (document.querySelector(TOP)) canvas.showPopover();
  };
  const seen = new MutationObserver(dress);
  seen.observe(html, {
    attributes: true,
    attributeFilter: ["data-theme", "data-palette", "data-sparks", "data-motion"],
  });
  const opened = new MutationObserver((list) => {
    if (list.some((one) => one.target instanceof HTMLDialogElement)) raise();
  });
  opened.observe(document.body, { attributes: true, attributeFilter: ["open"], subtree: true });
  const night = matchMedia("(prefers-color-scheme: dark)");
  const calm = matchMedia("(prefers-reduced-motion: reduce)");
  const hidden = () => fx?.pause(document.hidden);
  const fit = () => fx?.fit();
  const ears = new AbortController();
  const how = { capture: true, passive: true, signal: ears.signal };
  addEventListener("pointerdown", down, how);
  addEventListener("pointermove", move, how);
  addEventListener("pointerup", up, how);
  addEventListener("pointercancel", up, how);
  addEventListener("keydown", key, how);
  addEventListener("resize", fit, { signal: ears.signal });
  document.addEventListener("toggle", raise, { capture: true, signal: ears.signal });
  document.addEventListener("visibilitychange", hidden, { signal: ears.signal });
  night.addEventListener("change", dress, { signal: ears.signal });
  calm.addEventListener("change", dress, { signal: ears.signal });
  return () => {
    ears.abort();
    seen.disconnect();
    opened.disconnect();
    fx?.stop();
    sound.stop();
    canvas.remove();
    html.style.removeProperty("--px-arrow");
    html.style.removeProperty("--px-hand");
  };
}
