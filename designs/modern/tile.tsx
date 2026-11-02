import type { Props } from "../../net/shell/design.ts";

/* TILE */

export function Tile({ title, href, art, current, named, bind }: Props["tile"]) {
  return (
    <a
      className="launcher"
      href={href}
      draggable="false"
      aria-label={named ? undefined : title}
      aria-current={current ? "page" : undefined}
      data-name={title}
      {...bind()}
    >
      {art}
      {named && <span>{title}</span>}
    </a>
  );
}
