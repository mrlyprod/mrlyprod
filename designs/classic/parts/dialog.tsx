import { classes } from "@mrlyprod/ui/kit/room.ts";
import type { Props } from "@mrlyprod/ui/shell/design.ts";
import { Glyph } from "../own/glyphs.tsx";

export function Dialog({ title, light, tag, bind, children }: Props["dialog"]) {
  return (
    <dialog className={classes("dialog", light ? "light" : "raised", tag)} aria-label={title} {...bind()}>
      {!light && (
        <form className="title" method="dialog">
          <span className="name">{title}</span>
          <span className="actions">
            <button type="submit" className="raised knob" aria-label="Close">
              <Glyph art="close" />
            </button>
          </span>
        </form>
      )}
      <div className="body">{children}</div>
    </dialog>
  );
}
