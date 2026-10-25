import { AppIcon } from "./appicon.tsx";
import { app } from "./catalog.ts";

/* TYPES */

export type TileProps = { id: string; name?: boolean; href?: string; current?: boolean };

export function Tile({ id, name = true, href, current = false }: TileProps) {
  const one = app(id);
  if (!one) return null;
  return (
    <a
      className="launcher"
      href={href ?? one.route}
      draggable="false"
      aria-label={name ? undefined : one.title}
      aria-current={current ? "page" : undefined}
      data-name={one.title}
    >
      <AppIcon id={id} />
      {name && <span>{one.title}</span>}
    </a>
  );
}
