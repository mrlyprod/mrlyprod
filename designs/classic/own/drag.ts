import type { PointerEvent } from "react";

export function drag(e: PointerEvent<HTMLElement>) {
  const bar = e.currentTarget;
  const box = bar.closest("dialog");
  if (!box || e.button !== 0 || (e.target as Element).closest("button")) return;
  const [x = 0, y = 0] = box.style.translate.split(" ").map((n) => Number.parseFloat(n) || 0);
  const [fromX, fromY] = [e.clientX, e.clientY];
  const top = box.getBoundingClientRect().top - y;
  bar.setPointerCapture(e.pointerId);
  const move = (at: globalThis.PointerEvent) => {
    box.style.translate = `${x + at.clientX - fromX}px ${Math.max(-top, y + at.clientY - fromY)}px`;
  };
  const end = () => bar.removeEventListener("pointermove", move);
  bar.addEventListener("pointermove", move);
  bar.addEventListener("pointerup", end, { once: true });
  bar.addEventListener("pointercancel", end, { once: true });
}
