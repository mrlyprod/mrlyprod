import { classes } from "../../net/sdk/room.ts";
import type { Props } from "../../net/shell/design.ts";
import { BRICKS, hash, paint } from "./brick.ts";

/* TILE */

export function Tile({ id, title, href, art, current, soon, named, bind }: Props["tile"]) {
  return (
    <a
      className={classes("piece launcher", soon ? "soon" : undefined)}
      {...BRICKS}
      href={href}
      draggable="false"
      style={paint(hash(id))}
      aria-label={named ? undefined : title}
      aria-current={current ? "page" : undefined}
      data-name={title}
      {...bind()}
    >
      <span className="brick">
        <span className="lip studs" aria-hidden="true" />
        <span className="face">{art}</span>
      </span>
      {named && <span className="name tile">{title}</span>}
    </a>
  );
}
