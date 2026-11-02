import { Icon } from "../../net/sdk/icon.tsx";
import { classes } from "../../net/sdk/room.ts";
import type { Props } from "../../net/shell/design.ts";

/* DIALOG */

export function Dialog({ title, tag, bind, children }: Props["dialog"]) {
  return (
    <dialog className={classes("modal", tag)} aria-label={title} {...bind()}>
      <form method="dialog">
        <h2>{title}</h2>
        <button type="submit" className="icon" aria-label="Close">
          <Icon name="close" />
        </button>
      </form>
      {children}
    </dialog>
  );
}
