import { classes } from "../../net/sdk/room.ts";
import type { Props } from "../../net/shell/design.ts";
import { MRLY } from "./cell.ts";

/* DIALOG */

export function Dialog({ title, light, tag, bind, children }: Props["dialog"]) {
  return (
    <dialog className={classes("modal", tag)} {...MRLY} data-light={light} aria-label={title} {...bind()}>
      <form method="dialog" className="head">
        <h2 className="cell">{title}</h2>
        <button type="submit" className="cell" aria-label="Close">
          x
        </button>
      </form>
      <div className="body">{children}</div>
    </dialog>
  );
}
