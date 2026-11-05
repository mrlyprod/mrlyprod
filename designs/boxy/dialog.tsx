import { classes } from "@mrlyprod/ui/kit/room.ts";
import type { Props } from "@mrlyprod/ui/shell/design.ts";
import { BOXY } from "./cell.ts";

export function Dialog({ title, light, tag, bind, children }: Props["dialog"]) {
  return (
    <dialog className={classes("modal", tag)} {...BOXY} data-light={light} aria-label={title} {...bind()}>
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
