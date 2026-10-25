import { figures } from "site:figures";
import type { Site } from "site:routes";
import type { Row } from "../../../scripts/build.ts";
import { Figure, hide, Lede, show } from "../../shell/parts.tsx";
import type { Layout } from "../../shell/router.ts";

export const layout: Layout = { wide: true, bare: true };

const live = new WeakMap<HTMLElement, () => void>();

export function render(row: Row, _site: Site, host: HTMLElement) {
  const names = Object.keys(figures).sort();
  show(
    host,
    <>
      <Lede id="figures" title={row.title} lead={`${row.lead} ${names.length} figures.`} />
      <div className="gallery grid" data-eager>
        {names.map((name) => (
          <div className="tile" key={name}>
            <Figure name={name} label={name} caption={name} />
          </div>
        ))}
      </div>
    </>,
  );
}

export function mount(host: HTMLElement) {
  const drawn = (event: HTMLElementEventMap["drawn"]) => {
    const caption = event.target.querySelector("figcaption");
    if (caption) caption.textContent = `${event.target.dataset.figure} ${event.detail.toFixed(1)} ms`;
  };
  host.addEventListener("drawn", drawn);
  live.set(host, () => host.removeEventListener("drawn", drawn));
}

export function unmount(host: HTMLElement) {
  live.get(host)?.();
  live.delete(host);
  hide(host);
}
