import { classes } from "@mrlyprod/ui/kit/room.ts";
import type { Props } from "@mrlyprod/ui/shell/design.ts";

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
