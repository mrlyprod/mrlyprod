import { classes } from "../../net/sdk/room.ts";
import type { Props } from "../../net/shell/design.ts";
import { BOXY, hash, tint } from "./cell.ts";

/* TILE */

export function Tile({ id, title, href, art, current, soon, named, bind }: Props["tile"]) {
  return (
    <a
      className={classes("tile launcher", soon ? "soon" : undefined)}
      {...BOXY}
      href={href}
      draggable="false"
      style={tint(hash(id))}
      aria-label={named ? undefined : title}
      aria-current={current ? "page" : undefined}
      data-name={title}
      {...bind()}
    >
      <span className="art cell">
        {art}
        {soon && <span className="soon">soon</span>}
      </span>
      {named && <span className="name cell">{title.toLowerCase()}</span>}
    </a>
  );
}
