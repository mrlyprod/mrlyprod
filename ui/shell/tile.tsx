import { parts } from "./active.ts";
import { AppIcon } from "./appicon.tsx";
import { app } from "./catalog.ts";

export type TileProps = { id: string; name?: boolean; href?: string; current?: boolean };

const none = () => ({});

export function Tile({ id, name = true, href, current = false }: TileProps) {
  const one = app(id);
  if (!one) return null;
  return (
    <parts.tile
      id={id}
      title={one.title}
      href={href ?? one.route}
      art={<AppIcon id={id} />}
      current={current}
      named={name}
      bind={none}
    />
  );
}
