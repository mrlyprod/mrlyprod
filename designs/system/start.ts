import type { World } from "@mrlyprod/ui/shell/design.ts";
import { hash } from "./own/cell.ts";

export function start({ root }: World) {
  const box = root.querySelector<HTMLElement>(".window");
  if (!box) return () => {};
  const paint = () => {
    const hue = hash(box.dataset.app || location.pathname);
    box.dataset.hue = hue;
    box.style.setProperty("--hue", `var(--${hue})`);
  };
  paint();
  const eye = new MutationObserver(paint);
  eye.observe(box, { attributeFilter: ["data-app", "aria-label"] });
  return () => eye.disconnect();
}
