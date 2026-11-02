import { classes } from "../../net/sdk/room.ts";
import type { Props } from "../../net/shell/design.ts";

/* DIALOG */

export function Dialog({ title, light, tag, bind, children }: Props["dialog"]) {
  return (
    <dialog className={classes("terminal", "dialog", light ? "light" : "frame", tag)} aria-label={title} {...bind()}>
      {!light && (
        <form className="title" method="dialog">
          <button type="submit" className="knob close" aria-label="Close">
            [x]
          </button>
          <span className="name">{title}</span>
        </form>
      )}
      <div className="body">{children}</div>
    </dialog>
  );
}
