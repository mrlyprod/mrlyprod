import { classes } from "@mrlyprod/ui/kit/room.ts";
import type { Props } from "@mrlyprod/ui/shell/design.ts";

export function Sheet({ title, tag, bind, children }: Props["sheet"]) {
  return (
    <dialog className={classes("drawer", tag)} aria-label={title} closedby="any" {...bind.root()}>
      <header {...bind.pull()}>
        <span className="grabber" aria-hidden="true" />
        <h2>{title}</h2>
        <form method="dialog">
          <button type="submit" className="done">
            Done
          </button>
        </form>
      </header>
      <div className="body">{children}</div>
    </dialog>
  );
}
