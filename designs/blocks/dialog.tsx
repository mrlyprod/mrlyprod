import { Icon } from "../../net/sdk/icon.tsx";
import { classes } from "../../net/sdk/room.ts";
import type { Props } from "../../net/shell/design.ts";
import { BLOCKS, paint } from "./brick.ts";

/* DIALOG */

export function Dialog({ title, light, tag, bind, children }: Props["dialog"]) {
  return (
    <dialog
      className={classes("modal plate", tag)}
      {...BLOCKS}
      style={paint(light ? "yellow" : "white")}
      data-light={light}
      aria-label={title}
      {...bind()}
    >
      <form method="dialog" className="head plate" style={paint("black")}>
        <h2>{title}</h2>
        <button type="submit" className="close brick" style={paint("red")} aria-label="Close">
          <Icon name="close" size={24} />
        </button>
      </form>
      <div className="body tile">{children}</div>
    </dialog>
  );
}
